use std::{collections::HashMap, fmt::Debug};

use super::{
    binary_trie::{Binary, ToUsize},
    nodes::strong_link::StrongLinkNode,
};
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct BinaryLabel {
    label: Vec<Binary>,
    max_depth: usize,
}
impl BinaryLabel {
    fn new(max_depth: usize, label_depth: usize, usized_data: usize) -> Self {
        let mut v = Vec::new();
        for depth in 0..label_depth {
            v.push(Binary::calc_binary(usized_data, max_depth - depth))
        }
        Self {
            max_depth,
            label: v,
        }
    }
    fn is_same(&self, label_depth: usize, usized_data: usize) -> bool {
        for depth in 0..label_depth {
            if self.label[depth] != Binary::calc_binary(usized_data, self.max_depth - depth) {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod x_fast_trie_parts_test {
    use super::*;
    #[test]
    fn binary_label_test() {
        let max_depth = 4;
        let label_depth = 2;
        let usized_data = 1;
        let binary_label = BinaryLabel::new(max_depth, label_depth, usized_data);
        let tobe = BinaryLabel {
            label: vec![Binary::Zero, Binary::Zero],
            max_depth,
        };
        assert_eq!(binary_label, tobe);
        assert!(binary_label.is_same(label_depth, usized_data));
        assert!(binary_label.is_same(label_depth, 2));
        assert!(binary_label.is_same(label_depth, 3));
        assert!(!binary_label.is_same(label_depth, 5));

        let max_depth = 4;
        let label_depth = 3;
        let usized_data = 1;
        let binary_label = BinaryLabel::new(max_depth, label_depth, usized_data);
        let tobe = BinaryLabel {
            label: vec![Binary::Zero, Binary::Zero, Binary::Zero],
            max_depth,
        };
        assert_eq!(binary_label, tobe);
        assert!(binary_label.is_same(label_depth, usized_data));

        let max_depth = 4;
        let label_depth = 3;
        let usized_data = 15;
        let binary_label = BinaryLabel::new(max_depth, label_depth, usized_data);
        let tobe = BinaryLabel {
            label: vec![Binary::One, Binary::One, Binary::One],
            max_depth,
        };
        assert_eq!(binary_label, tobe);
        assert!(binary_label.is_same(label_depth, usized_data));
    }
    //fn test() {
    //let mut hash_table = XFastTrieHashTable::new(4);
    //hash_table.add_data(1);
    //let leaf = StrongLinkNode::new_leaf(1);
    //let mut depth_zero: HashMap<BinaryLabel, StrongLinkNode<i32>> = HashMap::new();
    //depth_zero.insert(BinaryLabel { label: () }::Zero, leaf.clone());
    //let mut depth_one: HashMap<Binary, StrongLinkNode<i32>> = HashMap::new();
    //depth_one.insert(Binary::Zero, leaf.clone());
    //let tobe = XFastTrieHashTable { table: vec![] };
    //assert_eq!(hash_table, tobe);
    //}
}

#[derive(Debug, PartialEq)]
pub struct XFastTrie<T: ToUsize + Clone + Debug + PartialEq> {
    root: StrongLinkNode<T>,
    min_prev: StrongLinkNode<T>,
    max_next: StrongLinkNode<T>,
    map: HashMap<Vec<Binary>, StrongLinkNode<T>>,
    w: usize,
}
impl<T: ToUsize + Clone + Debug + PartialEq> XFastTrie<T> {
    pub fn new(w: usize) -> Self {
        let mut min_prev = StrongLinkNode::new_path_node();
        let max_next = StrongLinkNode::new_path_node();
        min_prev.set_next(max_next.clone());
        let root = StrongLinkNode::new_path_node();
        Self {
            root,
            min_prev,
            max_next,
            w,
            map: HashMap::new(),
        }
    }
    pub fn add(&mut self, x: T) -> bool {
        true
    }
    pub fn find(&self, x: T) -> bool {
        true
    }
}

#[cfg(test)]
mod x_fast_trie_test {
    use super::*;

    #[test]
    fn add_test() {
        let mut root = StrongLinkNode::new_path_node();
        let mut leaf_3 = StrongLinkNode::new_leaf(3);
        let mut min_prev = StrongLinkNode::new_path_node();
        let mut max_next = StrongLinkNode::new_path_node();
        //let mut map = HashMap::new();
        min_prev.set_next(leaf_3.clone());
        max_next.set_prev(leaf_3.clone());

        let mut root_left_child = StrongLinkNode::new_path_node();
        let mut root_left_child_left_child = StrongLinkNode::new_path_node();
        let mut root_left_child_left_child_right_child = StrongLinkNode::new_path_node();

        //map.insert(vec![Binary::Zero], root_left_child.clone());
        //map.insert(vec![Binary::Zero], root_left_child.clone());
        root.set_jump(leaf_3.clone());
        root_left_child.set_jump(leaf_3.clone());
        root_left_child_left_child.set_jump(leaf_3.clone());
        root_left_child_left_child_right_child.set_jump(leaf_3.clone());
        root_left_child_left_child_right_child.set_right(leaf_3.clone());
        root_left_child_left_child.set_right(root_left_child_left_child_right_child.clone());
        root_left_child.set_left(root_left_child_left_child.clone());
        root.set_left(root_left_child.clone());

        //let tobe = XFastTrie {
        //root: root.clone(),
        //min_prev,
        //max_next,
        //map,
        //w: 4,
        //};

        let mut tree = XFastTrie::new(4);
        tree.add(3);
        //assert_eq!(tree, tobe);
    }
    //#[test]
    //fn find_test() {
    //let mut tree = XFastTrie::new(4);
    //for i in 0..16 {
    //tree.add(i);
    //assert!(tree.find(i));
    //assert!(!tree.find(i));
    //}
    //}
}
