use std::fmt::Debug;

use super::{
    binary_trie::{Binary, ToUsize},
    nodes::strong_link::StrongLinkNode,
    x_fast_trie_parts::hash_table::XFastTrieHashTable,
};

#[derive(Debug, PartialEq)]
pub struct XFastTrie<T: ToUsize + Clone + Debug + PartialEq> {
    root: StrongLinkNode<T>,
    min_prev: StrongLinkNode<T>,
    max_next: StrongLinkNode<T>,
    table: XFastTrieHashTable<StrongLinkNode<T>>,
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
            table: XFastTrieHashTable::new(w),
        }
    }
    fn digit_to_depth(&self, digit: usize) -> usize {
        self.w - digit + 1
    }
    pub fn add(&mut self, x: T) -> bool {
        let num_x = x.to_usize();
        let leaf = StrongLinkNode::new_leaf(x.clone());
        let mut node = self.root.clone();
        let mut prev = self.find_prev(num_x);
        let usized_data = x;

        for digit in (1..=self.w).rev() {
            let binary = Binary::calc_binary(num_x, digit);
            let child = node.child(binary.to_num());
            if child.is_some() {
                if digit == 1 {
                    //x can not add because x is exist
                    return false;
                }
                if node.jump().is_some() {
                    if binary == Binary::Zero && node.jump().num() < Some(num_x)
                        || binary == Binary::One && node.jump().num() > Some(num_x)
                    {
                        node.set_jump(leaf.clone());
                    }
                }
                node = child;
            } else {
                if digit != 1 {
                    let mut new_path_node = StrongLinkNode::new_path_node();
                    new_path_node.set_jump(leaf.clone());
                    self.table.register_binary_labels(
                        self.digit_to_depth(digit),
                        &usized_data,
                        new_path_node.clone(),
                    );
                    node.set_child(new_path_node.clone(), binary.to_num());
                    if node.child(binary.other().to_num()).is_none() {
                        node.set_jump(leaf.clone())
                    } else if node.jump().is_some() {
                        node.remove_jump()
                    }
                    node = new_path_node;
                } else {
                    self.table.register_binary_labels(
                        self.digit_to_depth(digit),
                        &usized_data,
                        leaf.clone(),
                    );
                    node.set_child(leaf.clone(), binary.to_num());
                    if node.child(binary.other().to_num()).is_none() {
                        node.set_jump(leaf.clone());
                    } else if node.jump().is_some() {
                        node.remove_jump()
                    }
                    let mut next = prev.next();
                    prev.set_next(leaf.clone());
                    next.set_prev(leaf.clone());
                }
            }
        }
        true
    }
    fn find_prev(&self, num: usize) -> StrongLinkNode<T> {
        //let mut start = 0;
        //let mut end = self.w + 1;
        //let mut depth = (end+start)/2;
        //while end - start > 1 {
        //if self.table.find(depth, x) {
        //start = depth;
        //}else{
        //end = depth;
        //}
        //}
        //if depth == self.w {

        //}
        let mut node = self.root.clone();
        for i in (1..=self.w).rev() {
            let binary = Binary::calc_binary(num, i);
            let child = node.child(binary.to_num());
            if child.is_some() {
                node = child
            } else {
                node = node.jump();
                if node.is_none() {
                    return self.min_prev.clone();
                }
                if node.num() >= Some(num) {
                    node = node.prev()
                }
                return node;
            }
        }
        if node.num() == Some(num) {
            node.prev()
        } else {
            self.min_prev.clone()
        }
    }
    fn in_range(&self, x: &T) -> bool {
        let num_x = x.to_usize();
        num_x < 2_i128.pow(self.w as u32) as usize
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
        let max_depth = 4;
        let mut root = StrongLinkNode::new_path_node();
        let mut leaf_3 = StrongLinkNode::new_leaf(3);
        let mut min_prev = StrongLinkNode::new_path_node();
        let mut max_next = StrongLinkNode::new_path_node();
        let mut table = XFastTrieHashTable::new(max_depth);
        min_prev.set_next(leaf_3.clone());
        max_next.set_prev(leaf_3.clone());

        let mut root_left = StrongLinkNode::new_path_node();
        let mut root_left_left = StrongLinkNode::new_path_node();
        let mut root_left_left_right = StrongLinkNode::new_path_node();

        root.set_jump(leaf_3.clone());
        root_left.set_jump(leaf_3.clone());
        root_left_left.set_jump(leaf_3.clone());
        root_left_left_right.set_jump(leaf_3.clone());
        root_left_left_right.set_right(leaf_3.clone());
        root_left_left.set_right(root_left_left_right.clone());
        root_left.set_left(root_left_left.clone());
        root.set_left(root_left.clone());

        table.register_binary_labels(1, &leaf_3.value().clone().unwrap(), root_left.clone());
        table.register_binary_labels(2, &leaf_3.value().clone().unwrap(), root_left_left.clone());
        table.register_binary_labels(
            3,
            &leaf_3.value().clone().unwrap(),
            root_left_left_right.clone(),
        );
        table.register_binary_labels(4, &leaf_3.value().clone().unwrap(), leaf_3.clone());

        let tobe = XFastTrie {
            root: root.clone(),
            min_prev,
            max_next,
            table,
            w: max_depth,
        };

        let mut tree = XFastTrie::new(max_depth);
        tree.add(3);
        assert_eq!(tree, tobe);
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
