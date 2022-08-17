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
#[derive(Debug, PartialEq, Clone)]
struct XFastTrieHashTable<T: Clone + Debug + PartialEq> {
    max_depth: usize,
    table: Vec<HashMap<BinaryLabel, StrongLinkNode<T>>>,
}

impl<T: Clone + Debug + PartialEq + ToUsize> XFastTrieHashTable<T> {
    fn new(max_depth: usize) -> Self {
        let table = vec![HashMap::new(); max_depth - 1];
        Self { max_depth, table }
    }
    fn register_binary_labels(
        &mut self,
        depth: usize,
        usized_data: usize,
        node: &StrongLinkNode<T>,
    ) {
        let key = self.gen_key(depth, usized_data);
        self.table[Self::table_index(depth)].insert(key, node.clone());
    }
    fn remove(&mut self, depth: usize, usized_data: usize) {
        let key = self.gen_key(depth, usized_data);
        self.table[Self::table_index(depth)].remove(&key);
    }
    fn gen_key(&self, depth: usize, usized_data: usize) -> BinaryLabel {
        BinaryLabel::new(self.max_depth, depth, usized_data)
    }
    fn leaf_parent_depth(&self) -> usize {
        self.max_depth - 1
    }
    fn table_index(depth: usize) -> usize {
        depth - 1
    }
    fn containes_all_label_at_data(&self, usized_leaf: usize) -> bool {
        let leaf_parent_depth = self.leaf_parent_depth();
        let leaf_parent_table_index = Self::table_index(leaf_parent_depth);
        self.table[leaf_parent_table_index].contains_key(&BinaryLabel::new(
            self.max_depth,
            leaf_parent_depth,
            usized_leaf,
        ))
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
    #[test]
    fn x_fast_trie_hash_table_register_test() {
        let mut hash_table = XFastTrieHashTable::new(4);
        let mut root = StrongLinkNode::new_path_node();
        let mut root_left = StrongLinkNode::new_path_node();
        let mut root_left_left = StrongLinkNode::new_path_node();
        let mut root_left_left_left = StrongLinkNode::new_path_node();
        let leaf = StrongLinkNode::new_leaf(1);
        root.set_jump(leaf.clone());
        root.set_child(root_left.clone(), 0);

        root_left.set_jump(leaf.clone());
        root_left.set_child(root_left_left.clone(), 0);

        root_left_left.set_jump(leaf.clone());
        root_left_left.set_child(root_left_left_left.clone(), 0);

        root_left_left_left.set_jump(leaf.clone());
        root_left_left_left.set_child(leaf.clone(), 1);

        hash_table.register_binary_labels(1, 1, &root_left);
        hash_table.register_binary_labels(2, 1, &root_left_left);
        hash_table.register_binary_labels(3, 1, &root_left_left_left);

        let mut depth_one: HashMap<BinaryLabel, StrongLinkNode<i32>> = HashMap::new();
        depth_one.insert(BinaryLabel::new(4, 1, 1), root_left.clone());
        let mut depth_two: HashMap<BinaryLabel, StrongLinkNode<i32>> = HashMap::new();
        depth_two.insert(BinaryLabel::new(4, 2, 1), root_left_left.clone());
        let mut depth_three: HashMap<BinaryLabel, StrongLinkNode<i32>> = HashMap::new();
        depth_three.insert(BinaryLabel::new(4, 3, 1), root_left_left_left.clone());
        let tobe = XFastTrieHashTable {
            table: vec![depth_one, depth_two, depth_three],
            max_depth: 4,
        };
        assert_eq!(hash_table, tobe);
    }
    #[test]
    fn x_fast_trie_hash_table_remove_test() {
        let mut root = StrongLinkNode::new_path_node();
        let mut root_left = StrongLinkNode::new_path_node();
        let mut root_left_left = StrongLinkNode::new_path_node();
        let mut root_left_left_left = StrongLinkNode::new_path_node();
        let leaf = StrongLinkNode::new_leaf(1);
        root.set_jump(leaf.clone());
        root.set_child(root_left.clone(), 0);

        root_left.set_jump(leaf.clone());
        root_left.set_child(root_left_left.clone(), 0);

        root_left_left.set_jump(leaf.clone());
        root_left_left.set_child(root_left_left_left.clone(), 0);

        root_left_left_left.set_jump(leaf.clone());
        root_left_left_left.set_child(leaf.clone(), 1);

        let mut hash_table = XFastTrieHashTable::<i32>::new(4);
        hash_table.register_binary_labels(1, 1, &root_left);
        hash_table.register_binary_labels(2, 1, &root_left_left);
        hash_table.register_binary_labels(3, 1, &root_left_left_left);
        hash_table.remove(1, 1);
        hash_table.remove(2, 1);
        hash_table.remove(3, 1);

        assert_eq!(hash_table, XFastTrieHashTable::new(4));
    }
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
