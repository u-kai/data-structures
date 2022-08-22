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
        println!("before split {:#?}", self);
        let mut new_node = Node::new_empty();
        for (new_node_index, move_index) in (B..self.children.len()).enumerate() {
            let move_child_index = self.children[move_index].take();
            new_node.children[new_node_index] = move_child_index;
        }
        for (new_node_index, key_index) in (B..self.keys.len()).enumerate() {
            let move_key = self.keys[key_index].take();
            new_node.keys[new_node_index] = move_key;
        }

        println!("after splited {:#?}", self);
        println!("new_node {:#?}", new_node);
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
    fn add_key(&mut self, x: T, key_index: KeyIndex) {
        let last_index = self.keys.len() - 1;
        if last_index != *key_index {
            self.keys[*key_index..=last_index].rotate_right(1);
        }
        self.keys[*key_index] = Some(x);
    }
    fn add_child(&mut self, key_index: KeyIndex, child_index: BIndex) {
        let last_index = self.children.len() - 1;
        let insert_index = *key_index;
        //if last_index != insert_index {
        //self.children[insert_index..=last_index].rotate_right(1);
        //}
        self.children[insert_index..=last_index].rotate_right(1);
        self.children[insert_index] = Some(child_index)
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
                ////let len = self.children.len();
                ////self.children[*key_index..=(len - 1)].rotate_right(1);
                ////self.children[*key_index] = Some(index)
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
        println!("before removed {:#?}", self);
        let removed = self.keys[*key_index].take();
        self.keys.rotate_left(1);
        println!("after removed {:#?}", self);
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
            AddRecResult::Splited(mut splited_node, index, mut node) => {
                let mut new_root = Node::new_empty();
                let x = splited_node.remove(0.into());
                self.block_store.write_block(splited_node);
                new_root.children[0] = self.root_index.into();
                new_root.keys[0] = x;
                new_root.children[1] = index.into();
                let root_index = self.block_store.place_block(new_root);
                self.root_index = root_index;
                true
            }
        }
    }
    fn add_rec(&mut self, x: T, node_index: BIndex) -> AddRecResult<T> {
        let maybe_block = self
            .block_store
            .read_block(node_index)
            .map(|block| block.clone());
        match maybe_block {
            Some(mut block) => match &block.find_it(&x) {
                IndexUsedByFindIt::FindJust(_) => return AddRecResult::AlreadyExist,
                IndexUsedByFindIt::NotFindResult(key_index) => {
                    if block.is_leaf(*key_index) {
                        block.add_key(x, *key_index);
                        self.block_store.write_block(block.clone());
                    } else {
                        let child_index = ChildIndex::from(**key_index);
                        let block_index = block.children[*child_index].unwrap();
                        let rec_result = self.add_rec(x, block_index);
                        if let AddRecResult::Splited(mut splited_node, index, mut new_node) =
                            rec_result
                        {
                            let x = splited_node.remove(KeyIndex::from(0)).unwrap();
                            self.block_store.write_block(splited_node);
                            block.add(x, index);
                            self.block_store.write_block(block.clone());
                        }
                    };
                    if block.is_full() {
                        let new_node = block.split();
                        let new_node_index = self.block_store.place_block(new_node.clone());
                        return AddRecResult::Splited(block, new_node_index, new_node);
                    } else {
                        return AddRecResult::NotSplite;
                    }
                }
            },
            None => self.block_store.add_new_block(Node::new(x)),
        }
        AddRecResult::NotSplite
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
    Splited(Block<Node<T>>, BIndex, Node<T>),
}

#[cfg(test)]
mod btree_test {
    use super::*;
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
                    Block::new(0.into(), left),
                    Block::new(1.into(), right),
                    Block::new(2.into(), root),
                ],
                free_list: vec![],
            },
        };
        println!("tree  {:#?}", tree);
        println!("tobe  {:#?}", tobe);
        assert_eq!(tree, tobe);
        //println!("tree {:#?}", tree);
        tree.add(1);
        //println!("after add 1 {:#?}", tree);
        tree.add(2);
        //println!("after add 2 {:#?}", tree);
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
                    Block::new(0.into(), left),
                    Block::new(1.into(), right),
                    Block::new(2.into(), root),
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
                    Block::new(0.into(), left),
                    Block::new(1.into(), right),
                    Block::new(2.into(), root),
                    Block::new(3.into(), middle),
                ],
                free_list: vec![],
            },
        };
        tree.add(3);
        assert_eq!(tree, tobe);
        let mut tree = BTree::<i32>::new();
        for i in 0..15 {
            tree.add(i);
        }
        println!("{:#?}", tree);
        assert!(false)
    }
}
