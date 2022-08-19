use super::{
    block_store::BlockStore,
    helper::{build_B2_minus_1_none, B},
};
use std::{cmp::Ordering, fmt::Debug};
pub(super) type BIndex = usize;

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
            root_index: 0,
            block_store: BlockStore::<Node<T>>::new(),
        }
    }
    pub fn add(&mut self, x: T) -> bool {
        let add_rec_result = self.add_rec(x, self.root_index);
        match add_rec_result {
            AddRecResult::AlreadyExist => false,
            AddRecResult::NotSplite => true,
            AddRecResult::Splited(node_index) => {
                //let new_root = Node::new();
                true
            }
        }
    }
    fn add_rec(&mut self, x: T, node_index: BIndex) -> AddRecResult {
        let node = self.block_store.read_block(node_index);
        match node {
            Some(node) => {
                let node_keys = &node.keys;
                let find_index = Self::find_it(node_keys, &x);
                match find_index {
                    IndexUsedByFindIt::FindJust(_) => return AddRecResult::AlreadyExist,
                    IndexUsedByFindIt::FindGreater(key_index) => {
                        if node.keys.len() - 2 == key_index {
                            //full_case
                        }
                    }
                    IndexUsedByFindIt::LastIndex(key_index) => {
                        //full_case
                    }
                    IndexUsedByFindIt::EmptyIndex(key_index) => {
                        let mut update_node = node.clone();
                        update_node.add(x, key_index);
                        self.block_store.write_block(node_index, update_node);
                    }
                }
            }
            None => self.block_store.write_block(node_index, Node::new(x)),
        }
        //let index = Self::find_it(node_keys, &x);
        //match index {
        //IndexUsedByFindIt::FindJust(_) => return AddRecResult::AlreadyExist,
        //IndexUsedByFindIt::FindGreater(index) => {

        //}
        //IndexUsedByFindIt::EmptyIndex(index) => {
        //node.add(x, index);
        //}

        //}

        AddRecResult::NotSplite
    }

    fn find_it(array: &[Option<T>], x: &T) -> IndexUsedByFindIt {
        let mut start = 0;
        let mut end = array.len();
        while start != end {
            let middle = (end + start) / 2;
            let cmp = if array[middle].is_none() {
                Ordering::Less
            } else {
                x.cmp(&array[middle].as_ref().unwrap())
            };
            match cmp {
                Ordering::Less => {
                    end = middle;
                }
                Ordering::Greater => {
                    start = middle + 1;
                }
                _ => return IndexUsedByFindIt::FindJust(middle),
            }
        }
        if start == array.len() - 1 {
            return IndexUsedByFindIt::LastIndex(start);
        };
        match &array[start] {
            Some(_) => IndexUsedByFindIt::FindGreater(start),
            None => IndexUsedByFindIt::EmptyIndex(start),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) struct Node<T> {
    keys: [Option<T>; 2 * B - 1],
    children: [Option<BIndex>; 2 * B],
}
impl<T> Node<T>
where
    T: Clone + Debug + PartialEq + PartialOrd + Ord + Default,
{
    pub fn new(x: T) -> Self {
        let mut keys = build_B2_minus_1_none::<T>();
        keys[0] = Some(x);
        Self {
            keys,
            children: [None; 2 * B],
        }
    }
    pub fn add(&mut self, x: T, index: BIndex) {
        self.keys[index] = Some(x);
    }
    //fn mode_node(&mut self) -> Node<T> {
    //let keys = self.keys
    //}
}

enum IndexUsedByFindIt {
    FindJust(BIndex),
    FindGreater(BIndex),
    EmptyIndex(BIndex),
    LastIndex(BIndex),
}
enum AddRecResult {
    AlreadyExist,
    NotSplite,
    Splited(BIndex),
}

#[cfg(test)]
mod btree_test {
    use super::*;
    #[test]
    fn add_test() {
        let mut tree = BTree::<i32>::new();
        let root = Node {
            keys: [Some(10), None, None],
            children: [None, None, None, None],
        };

        tree.add(10);
        assert_eq!(
            tree,
            BTree {
                root_index: 0,
                block_store: BlockStore {
                    block_list: vec![root],
                    free_list: vec![]
                },
            }
        );
        let mut left = Node {
            keys: [Some(3), Some(6), None],
            children: [None, None, None, None],
        };
        let mut right = Node {
            keys: [Some(14), Some(17), Some(22)],
            children: [None, None, None, None],
        };
        let left_left = Node {
            keys: [Some(0), Some(1), Some(2)],
            children: [None; 4],
        };
        let left_middle = Node {
            keys: [Some(4), Some(5), None],
            children: [None; 4],
        };
        let left_rigth = Node {
            keys: [Some(7), Some(8), Some(9)],
            children: [None; 4],
        };
        let mut right_left = Node {
            keys: [Some(11), Some(12), Some(14)],
            children: [None, None, None, None],
        };
        let mut right_middle1 = Node {
            keys: [Some(15), Some(16), None],
            children: [None, None, None, None],
        };
        let mut right_middle2 = Node {
            keys: [Some(18), Some(19), Some(20)],
            children: [None, None, None, None],
        };
        let mut right_right = Node {
            keys: [Some(23), Some(24), None],
            children: [None, None, None, None],
        };
        //root.children[0] = Some(1);
        //root.children[1] = Some(2);
        //left.children[0] = Some(3);
        //left.children[1] = Some(4);
        //left.children[2] = Some(5);
        //left.children[] = Some(5);

        //bs.write_block(0, b)
    }
}
