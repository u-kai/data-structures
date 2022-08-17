use std::{collections::HashMap, fmt::Debug};

use crate::structs::tries::binary_trie::ToUsize;

use super::binary_label::BinaryLabel;

#[derive(Debug, PartialEq, Clone)]
pub struct XFastTrieHashTable<T: Clone + Debug + PartialEq> {
    max_depth: usize,
    table: Vec<HashMap<BinaryLabel, T>>,
}

impl<T: Clone + Debug + PartialEq> XFastTrieHashTable<T> {
    pub fn new(max_depth: usize) -> Self {
        let table = vec![HashMap::new(); max_depth];
        Self { max_depth, table }
    }
    pub fn get(&self, depth: usize, usized_data: &impl ToUsize) -> Option<&T> {
        self.chaeck_in_range(depth);
        let key = self.gen_key(depth, usized_data);
        self.table[Self::table_index(depth)].get(&key)
    }
    pub fn register_binary_labels(&mut self, depth: usize, usized_data: &impl ToUsize, node: T) {
        self.chaeck_in_range(depth);
        let key = self.gen_key(depth, usized_data);
        self.table[Self::table_index(depth)].insert(key, node.clone());
    }
    pub fn remove(&mut self, depth: usize, usized_data: &impl ToUsize) {
        self.chaeck_in_range(depth);
        let key = self.gen_key(depth, usized_data);
        self.table[Self::table_index(depth)].remove(&key);
    }
    pub fn find(&self, depth: usize, usized_data: &impl ToUsize) -> bool {
        self.chaeck_in_range(depth);
        let key = self.gen_key(depth, usized_data);
        self.table[Self::table_index(depth)].contains_key(&key)
    }
    fn chaeck_in_range(&self, depth: usize) {
        if !self.in_range(depth) {
            panic!("depth is out of range")
        }
    }
    fn in_range(&self, depth: usize) -> bool {
        self.max_depth >= depth
    }
    fn gen_key(&self, depth: usize, usized_data: &impl ToUsize) -> BinaryLabel {
        BinaryLabel::new(self.max_depth, depth, usized_data.to_usize())
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
    use crate::structs::tries::nodes::strong_link::StrongLinkNode;

    use super::*;
    #[test]
    fn get_test() {
        let mut hash_table = XFastTrieHashTable::new(4);
        let data = "hello";
        let data2 = "world";
        let data3 = "yeah";
        hash_table.register_binary_labels(1, &1, data);
        hash_table.register_binary_labels(2, &1, data2);
        hash_table.register_binary_labels(3, &1, data3);
        assert_eq!(hash_table.get(1, &1), Some(&data));
        assert_eq!(hash_table.get(2, &1), Some(&data2));
        assert_eq!(hash_table.get(3, &1), Some(&data3));
    }
    #[test]
    #[should_panic]
    fn should_panic() {
        let mut hash_table = XFastTrieHashTable::new(4);
        let data = "hello";
        let data2 = "world";
        let data3 = "yeah";
        hash_table.register_binary_labels(1, &1, data);
        hash_table.register_binary_labels(2, &1, data2);
        hash_table.register_binary_labels(3, &1, data3);
        hash_table.register_binary_labels(4, &1, data3);
        hash_table.find(5, &1);
        hash_table.get(5, &1);
        hash_table.get(5, &1);
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

        hash_table.register_binary_labels(1, &1, root_left.clone());
        hash_table.register_binary_labels(2, &1, root_left_left.clone());
        hash_table.register_binary_labels(3, &1, root_left_left_left.clone());
        hash_table.register_binary_labels(4, &1, leaf.clone());

        let mut depth_one = HashMap::new();
        depth_one.insert(BinaryLabel::new(4, 1, 1), root_left.clone());
        let mut depth_two = HashMap::new();
        depth_two.insert(BinaryLabel::new(4, 2, 1), root_left_left.clone());
        let mut depth_three = HashMap::new();
        depth_three.insert(BinaryLabel::new(4, 3, 1), root_left_left_left.clone());
        let mut depth_four = HashMap::new();
        depth_four.insert(BinaryLabel::new(4, 4, 1), leaf.clone());
        let tobe = XFastTrieHashTable {
            table: vec![depth_one, depth_two, depth_three, depth_four],
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

        let mut hash_table = XFastTrieHashTable::new(4);
        hash_table.register_binary_labels(1, &1, root_left);
        hash_table.register_binary_labels(2, &1, root_left_left);
        hash_table.register_binary_labels(3, &1, root_left_left_left);
        hash_table.remove(1, &1);
        hash_table.remove(2, &1);
        hash_table.remove(3, &1);

        assert_eq!(hash_table, XFastTrieHashTable::new(4));
    }
    #[test]
    fn find_test() {
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

        let mut hash_table = XFastTrieHashTable::new(4);
        hash_table.register_binary_labels(1, &1, root_left);
        hash_table.register_binary_labels(2, &1, root_left_left);
        hash_table.register_binary_labels(3, &1, root_left_left_left);
        assert!(hash_table.find(1, &1),);
        assert!(hash_table.find(2, &1),);
        assert!(hash_table.find(1, &5));
        assert!(!hash_table.find(1, &8));
        assert!(hash_table.find(3, &1));
        assert!(!hash_table.find(3, &2));
    }
}
