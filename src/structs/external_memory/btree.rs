use super::{
    block_store::{Block, BlockStore},
    helper::{build_B2_none, B},
    indexs::{BIndex, ChildIndex, KeyIndex},
};
use std::{cmp::Ordering, fmt::Debug};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) struct Node<T> {
    keys: [Option<T>; 2 * B],
    children: [Option<BIndex>; 2 * B + 1],
}
impl<T> Node<T>
where
    T: Clone + Debug + PartialEq + PartialOrd + Ord + Default,
{
    pub fn new(x: T) -> Self {
        let mut keys = build_B2_none::<T>();
        keys[0] = Some(x);
        Self {
            keys,
            children: [None; 2 * B + 1],
        }
    }

    fn split(&mut self) -> Self {
        if !self.is_full() {
            panic!("can not split! because node = {:#?} is not full", self)
        };
        let mut new_node = Node::new_empty();
        for (new_node_index, move_index) in (B..self.children.len()).enumerate() {
            let move_child_index = self.children[move_index].take();
            new_node.children[new_node_index] = move_child_index;
        }
        for (new_node_index, key_index) in (B..self.keys.len()).enumerate() {
            let move_key = self.keys[key_index].take();
            new_node.keys[new_node_index] = move_key;
        }
        new_node
    }
    pub fn is_full(&self) -> bool {
        self.keys.iter().filter(|node| node.is_some()).count() == 2 * B
    }
    pub fn is_leaf(&self, key_index: KeyIndex) -> bool {
        self.children[*key_index].is_none()
    }
    pub fn new_empty() -> Self {
        let keys = build_B2_none::<T>();
        Self {
            keys,
            children: [None; 2 * B + 1],
        }
    }
    fn child_num(&self) -> usize {
        self.children
            .as_ref()
            .iter()
            .filter(|child_index| child_index.is_none())
            .count()
    }
    fn key_num(&self) -> usize {
        self.keys
            .as_ref()
            .iter()
            .filter(|key| key.is_none())
            .count()
    }
    fn last_child(&mut self) -> Option<BIndex> {
        self.children[self.child_num()].take()
    }
    fn last_key(&mut self) -> Option<T> {
        self.keys[self.key_num()].take()
    }
    fn add_key(&mut self, x: T, key_index: KeyIndex) {
        let last_index = self.keys.len() - 1;
        if last_index != *key_index {
            self.keys[*key_index..=last_index].rotate_right(1);
        }
        self.keys[*key_index] = Some(x);
    }
    fn add_child(&mut self, insert_index: KeyIndex, child_index: BIndex) {
        let last_index = self.children.len() - 1;
        self.children[*insert_index..=last_index].rotate_right(1);
        self.children[*insert_index] = Some(child_index)
    }
    pub fn add(&mut self, x: T, index: BIndex) {
        let insert_key_index = self.find_it(&x);
        match insert_key_index {
            IndexUsedByFindIt::FindJust(_) => panic!("find just is not pattern at node.add"),
            IndexUsedByFindIt::NotFindResult(key_index) => {
                self.add_key(x.clone(), key_index);
                if let IndexUsedByFindIt::FindJust(key_index) = self.find_it(&x) {
                    let key_index = (*key_index + 1).into();
                    self.add_child(key_index, index);
                };
            }
        }
    }
    fn find_it(&self, x: &T) -> IndexUsedByFindIt {
        let mut start = 0;
        let mut end = self.keys.len();
        while start != end {
            let middle = (end + start) / 2;
            let cmp = if self.keys[middle].is_none() {
                Ordering::Less
            } else {
                x.cmp(&self.keys[middle].as_ref().unwrap())
            };
            match cmp {
                Ordering::Less => {
                    end = middle;
                }
                Ordering::Greater => {
                    start = middle + 1;
                }
                _ => return IndexUsedByFindIt::FindJust(middle.into()),
            }
        }
        IndexUsedByFindIt::NotFindResult(start.into())
    }
    pub fn remove(&mut self, key_index: KeyIndex) -> Option<T> {
        let removed = self.keys[*key_index].take();
        self.keys.rotate_left(1);
        removed
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct BTree<T>
where
    T: Clone + Debug + PartialEq + PartialOrd + Ord + Default,
{
    root_index: BIndex,
    block_store: BlockStore<Node<T>>,
}
impl<T> BTree<T>
where
    T: Clone + Debug + PartialEq + PartialOrd + Ord + Default,
{
    pub fn new() -> Self {
        Self {
            root_index: 0.into(),
            block_store: BlockStore::<Node<T>>::new(),
        }
    }
    pub fn add(&mut self, x: T) -> bool {
        let add_rec_result = self.add_rec(x, self.root_index);
        match add_rec_result {
            AddRecResult::AlreadyExist => false,
            AddRecResult::NotSplite => true,
            AddRecResult::Splited(mut splited_node, new_node_index) => {
                let mut new_root = Node::new_empty();
                let x = splited_node.remove(0.into());
                self.block_store.write_block(splited_node);
                new_root.children[0] = self.root_index.into();
                new_root.keys[0] = x;
                new_root.children[1] = new_node_index.into();
                let root_index = self.block_store.place_block(new_root);
                self.root_index = root_index;
                true
            }
        }
    }
    fn add_rec(&mut self, x: T, node_index: BIndex) -> AddRecResult<T> {
        let maybe_block = self.block_store.read_block(node_index);
        match maybe_block {
            Some(mut block) => match block.find_it(&x) {
                IndexUsedByFindIt::FindJust(_) => return AddRecResult::AlreadyExist,
                IndexUsedByFindIt::NotFindResult(key_index) => {
                    if block.is_leaf(key_index) {
                        block.add_key(x, key_index);
                        self.block_store.write_block(block.clone());
                    } else {
                        let child_index = ChildIndex::from(*key_index);
                        let block_index = block.children[*child_index].unwrap();
                        let rec_result = self.add_rec(x, block_index);
                        if let AddRecResult::Splited(mut splited_node, index) = rec_result {
                            let x = splited_node.remove(KeyIndex::from(0)).unwrap();
                            self.block_store.write_block(splited_node);
                            block.add(x, index);
                            self.block_store.write_block(block.clone());
                        }
                    };
                    if block.is_full() {
                        let new_node = block.split();
                        let new_node_index = self.block_store.place_block(new_node);
                        return AddRecResult::Splited(block, new_node_index);
                    } else {
                        return AddRecResult::NotSplite;
                    }
                }
            },
            None => self.block_store.add_new_block(Node::new(x)),
        }
        AddRecResult::NotSplite
    }
    pub fn remove(&mut self, x: T) -> bool {
        self.remove_rec(x, self.root_index.into())
    }
    fn remove_rec(&mut self, x: T, node_index: Option<BIndex>) -> bool {
        if node_index.is_none() {
            return false;
        }
        let mut node = self.block_store.read_block(node_index.unwrap()).unwrap();
        println!("before remove_rec {:#?}", node);
        let index = node.find_it(&x);
        match index {
            IndexUsedByFindIt::FindJust(index) => {
                if node.is_leaf(index) {
                    node.remove(index);
                    println!("after remove_node{:#?}", node);
                    self.block_store.write_block(node);
                } else {
                    node.keys[*index] =
                        Some(self.remove_smallest(node.children[*index + 1].unwrap()));
                    self.block_store.write_block(node.clone());
                    self.check_underflow(node, (*index + 1).into());
                }
                true
            }
            IndexUsedByFindIt::NotFindResult(index) => {
                println!("result_index {:?}", index);
                if self.remove_rec(x, node.children[*index]) {
                    self.check_underflow(node, (*index).into());
                    return true;
                }
                false
            }
        }
    }
    fn remove_smallest(&mut self, node_index: BIndex) -> T {
        let mut node = self.block_store.read_block(node_index).unwrap();
        if node.is_leaf(0.into()) {
            let x = node.remove(0.into()).unwrap();
            self.block_store.write_block(node);
            return x;
        }
        let x = self.remove_smallest(node.children[0].unwrap());
        self.check_underflow(node, 0.into());
        x
    }
    fn check_underflow(&mut self, block: Block<Node<T>>, child_index: ChildIndex) {
        if block.children[*child_index].is_none() {
            return;
        }
        if *child_index == 0 {
            self.check_underflow_zero(block, child_index);
            return;
        }
        self.check_underflow_non_zero(block, child_index)
    }
    fn shift_rl(&mut self, mut underflow_block: Block<Node<T>>, mut helper_block: Block<Node<T>>) {
        println!("before underflow_block {:#?}", underflow_block);
        println!("before helper_block {:#?}", helper_block);
        let move_node_index = helper_block.last_child().unwrap();
        let move_key = helper_block.last_key().unwrap();
        let insert_key_index = underflow_block.key_num();
        underflow_block.add_key(move_key, insert_key_index.into());
        underflow_block.add_child((insert_key_index + 1).into(), move_node_index);
        println!("after underflow_block {:#?}", underflow_block);
        println!("after helper_block {:#?}", helper_block);
        self.block_store.write_block(underflow_block);
        self.block_store.write_block(helper_block);
    }
    fn merge(
        &mut self,
        mut parent_block: Block<Node<T>>,
        left_index: ChildIndex,
        mut left_block: Block<Node<T>>,
        mut right_block: Block<Node<T>>,
    ) {
        println!("before paretnt_block {:#?}", parent_block);
        println!("before left_block {:#?}", left_block);
        println!("before right_block {:#?}", right_block);
        let left_len = left_block.key_num();
        for i in 0..right_block.key_num() {
            let x = right_block.remove(i.into()).unwrap();
            left_block.add_key(x, (left_len + i).into());
        }
        let left_len = left_block.child_num();
        for i in 0..right_block.child_num() {
            let index = right_block.children[i].take().unwrap();
            left_block.add_child((left_len + i).into(), index);
        }
        let left_block_index = left_block.index();
        parent_block.children[*left_index] = Some(left_block_index);
        parent_block.children[(*left_index + 1)] = None;
        println!("after paretnt_block {:#?}", parent_block);
        println!("after left_block {:#?}", left_block);
        println!("after right_block {:#?}", right_block);
        self.block_store.write_block(parent_block);
        self.block_store.write_block(left_block);
        self.block_store.free_block(right_block.index());
    }
    fn check_underflow_zero(&mut self, parent_block: Block<Node<T>>, child_index: ChildIndex) {
        let child_index = parent_block.children[*child_index].unwrap();
        let maybe_underflow_block = self.block_store.read_block(child_index).unwrap();
        if maybe_underflow_block.key_num() < B - 1 {
            let helper_block = self
                .block_store
                .read_block((*child_index + 1).into())
                .unwrap();
            let underflow_block = maybe_underflow_block;
            let helper_block = helper_block;
            let child_index = ChildIndex::from(*child_index);
            if helper_block.key_num() > B {
                self.shift_rl(underflow_block, helper_block);
                return;
            }
            self.merge(parent_block, child_index, underflow_block, helper_block);
        }
    }
    fn check_underflow_non_zero(&mut self, parent_block: Block<Node<T>>, child_index: ChildIndex) {
        let child_index = parent_block.children[*child_index].unwrap();
        let maybe_underflow_block = self.block_store.read_block(child_index).unwrap();
        if maybe_underflow_block.key_num() < B - 1 {
            let helper_block = self
                .block_store
                .read_block((*child_index - 1).into())
                .unwrap();
            let right_block = maybe_underflow_block;
            let left_block = helper_block;
            if left_block.key_num() > B {
                self.shift_rl(right_block, left_block);
                return;
            }
            let left_index = *child_index - 1;
            self.merge(parent_block, left_index.into(), left_block, right_block);
        }
    }
    fn to_string_rec(&self, node_index: BIndex) -> String {
        let mut result = String::new();
        if let Some(node) = self.block_store.read_block(node_index) {
            fn string_conect(s1: String, s2: String) -> String {
                let mut result = String::new();
                let s1_lines = s1.lines().collect::<Vec<_>>();
                let s2_lines = s2.lines().collect::<Vec<_>>();
                let max_len = s1_lines.len().max(s2_lines.len());
                for i in 0..max_len {
                    let s1 = s1_lines.get(i).unwrap_or(&"");
                    let s2 = s2_lines.get(i).unwrap_or(&"");
                    result = format!("{}{}{}\n", result, s1, s2,)
                }
                result
            }
            fn indent(mut s1: String, mut s2: String) -> String {
                let s1_len = s1.len();
                let s2_len = s2.len();
                let max_len = s1_len.max(s2_len);
                for i in 0..=max_len {
                    if s1_len < i {
                        s1.push(' ')
                    }
                    if s2_len < i {
                        s2.push(' ')
                    }
                }
                format!("{}\n{}", s1, s2)
            }
            let children_string = node.children.iter().filter(|child| child.is_some()).fold(
                String::new(),
                |acc, child_index| {
                    string_conect(
                        acc,
                        format!("{}", self.to_string_rec(child_index.unwrap().clone())),
                    )
                },
            );
            let mut keys_string = node
                .keys
                .iter()
                .filter(|key| key.is_some())
                .fold(String::from(" "), |acc, key| {
                    format!("{}{:?} ", acc, key.clone().unwrap())
                });
            if children_string == "".to_string() {
                if keys_string.len() % 2 == 0 {
                    keys_string.push('x');
                }
                let index_string = format!(
                    "{}i{}{}",
                    " ".repeat((keys_string.len() / 2) - 1),
                    *node_index,
                    " ".repeat(keys_string.len() / 2)
                );
                let d = indent(index_string, keys_string);

                return d;
            }
            let max_len = children_string
                .lines()
                .collect::<Vec<_>>()
                .get(0)
                .unwrap()
                .len();
            let mut diff = max_len - keys_string.len();
            if diff % 2 == 1 {
                diff += 1
            }
            let keys_string = format!(
                "{}{}{}",
                " ".repeat(diff / 2),
                keys_string,
                " ".repeat(diff / 2)
            );
            let index_string = format!(
                "{}i{}{}",
                " ".repeat((keys_string.len() / 2) - 1),
                *node_index,
                " ".repeat(keys_string.len() / 2)
            );
            let d = indent(index_string, keys_string);

            result = format!("{}\n{}", d, children_string)
        };
        result
    }
    pub fn to_string(&self) -> String {
        self.to_string_rec(self.root_index)
    }
}

#[derive(Debug)]
enum IndexUsedByFindIt {
    FindJust(KeyIndex),
    NotFindResult(KeyIndex),
}
#[derive(Debug)]
enum AddRecResult<T> {
    AlreadyExist,
    NotSplite,
    Splited(Block<Node<T>>, BIndex),
}

#[cfg(test)]
mod btree_test {
    use super::*;

    #[test]
    fn remove_test() {
        let mut tree = BTree::new();
        for i in 0..10 {
            tree.add(i);
        }
        //for i in 0..10 {
        //println!("{}", i);
        //assert!(tree.remove(i));
        //}
    }
    #[test]
    fn node_add_test() {
        let mut node = Node {
            keys: [Some(1), Some(2), None, None],
            children: [Some(0.into()), Some(1.into()), Some(2.into()), None, None],
        };
        node.add(0, 4.into());
        let tobe = Node {
            keys: [Some(0), Some(1), Some(2), None],
            children: [
                Some(0.into()),
                Some(4.into()),
                Some(1.into()),
                Some(2.into()),
                None,
            ],
        };
        assert_eq!(node, tobe);
        let mut node = Node {
            keys: [Some(1), Some(3), None, None],
            children: [Some(0.into()), Some(1.into()), Some(2.into()), None, None],
        };
        node.add(2, 4.into());
        let tobe = Node {
            keys: [Some(1), Some(2), Some(3), None],
            children: [
                Some(0.into()),
                Some(1.into()),
                Some(4.into()),
                Some(2.into()),
                None,
            ],
        };
        assert_eq!(node, tobe);
        let mut node = Node {
            keys: [Some(1), Some(10), Some(11), None],
            children: [
                Some(0.into()),
                Some(1.into()),
                Some(2.into()),
                Some(3.into()),
                None,
            ],
        };
        node.add(2, 4.into());
        let tobe = Node {
            keys: [Some(1), Some(2), Some(10), Some(11)],
            children: [
                Some(0.into()),
                Some(1.into()),
                Some(4.into()),
                Some(2.into()),
                Some(3.into()),
            ],
        };
        assert_eq!(node, tobe);
    }
    #[test]
    fn add_test() {
        let mut tree = BTree::<i32>::new();
        let root = Node {
            keys: [Some(10), None, None, None],
            children: [Some(0.into()), Some(1.into()), None, None, None],
        };
        let left = Node {
            keys: [Some(11), None, None, None],
            children: [None; 5],
        };
        let right = Node {
            keys: [Some(12), Some(13), Some(14), None],
            children: [None; 5],
        };
        tree.add(10);
        tree.add(11);
        tree.add(12);
        tree.add(13);
        tree.add(14);
        let tobe = BTree {
            root_index: 2.into(),
            block_store: BlockStore {
                block_list: vec![
                    Some(Block::new(0.into(), left)),
                    Some(Block::new(1.into(), right)),
                    Some(Block::new(2.into(), root)),
                ],
                free_list: vec![],
            },
        };
        assert_eq!(tree, tobe);
        tree.add(1);
        tree.add(2);
        let root = Node {
            keys: [Some(10), None, None, None],
            children: [Some(0.into()), Some(1.into()), None, None, None],
        };
        let left = Node {
            keys: [Some(1), Some(2), Some(11), None],
            children: [None; 5],
        };
        let right = Node {
            keys: [Some(12), Some(13), Some(14), None],
            children: [None; 5],
        };
        let tobe = BTree {
            root_index: 2.into(),
            block_store: BlockStore {
                block_list: vec![
                    Some(Block::new(0.into(), left)),
                    Some(Block::new(1.into(), right)),
                    Some(Block::new(2.into(), root)),
                ],
                free_list: vec![],
            },
        };
        assert_eq!(tree, tobe);
        let root = Node {
            keys: [Some(1), Some(10), None, None],
            children: [Some(0.into()), Some(3.into()), Some(1.into()), None, None],
        };
        let left = Node {
            keys: [Some(2), None, None, None],
            children: [None; 5],
        };
        let middle = Node {
            keys: [Some(3), Some(11), None, None],
            children: [None; 5],
        };
        let right = Node {
            keys: [Some(12), Some(13), Some(14), None],
            children: [None; 5],
        };
        let tobe = BTree {
            root_index: 2.into(),
            block_store: BlockStore {
                block_list: vec![
                    Some(Block::new(0.into(), left)),
                    Some(Block::new(1.into(), right)),
                    Some(Block::new(2.into(), root)),
                    Some(Block::new(3.into(), middle)),
                ],
                free_list: vec![],
            },
        };
        tree.add(3);
        assert_eq!(tree, tobe);
        let mut tree = BTree::<i32>::new();
    }
}
