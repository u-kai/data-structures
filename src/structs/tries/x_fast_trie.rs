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
        let mut prev = self.find_prev(&x);
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

                    if node.has_two_child() && node.has_jump() {
                        node.remove_jump()
                    }
                    if node.child(binary.other().to_num()).is_none() {
                        node.set_jump(leaf.clone())
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
    fn find_prev(&self, x: &T) -> StrongLinkNode<T> {
        let mut start = 0;
        let mut end = self.w + 1;
        let mut depth = (end + start) / 2;

        while (end - start) > 1 {
            depth = (end + start) / 2;
            if self.table.find(depth, x) {
                start = depth;
                continue;
            }
            end = depth;
        }
        if depth == self.w {
            let maybe_x_node = self.table.get(depth, x);
            return match maybe_x_node {
                Some(x_node) => x_node.prev(),
                None => self.min_prev.clone(),
            };
        }
        let parent_node = self.table.get(depth, x);
        let jump_node = parent_node.map(|parent| parent.jump()).unwrap_or({
            match self.root.jump().as_ref() {
                Some(jump_node) => StrongLinkNode::new(Some(jump_node.clone())),
                None => self.min_prev.clone(),
            }
        });
        if jump_node.num() >= Some(x.to_usize()) {
            return jump_node.prev();
        }
        jump_node
    }
    fn find_leaf(&self, x: &T) -> StrongLinkNode<T> {
        let prev = self.find_prev(x);
        if prev.next().num() == Some(x.to_usize()) {
            return prev.next();
        }
        StrongLinkNode::new_none()
    }
    fn in_range(&self, x: &T) -> bool {
        let num_x = x.to_usize();
        num_x < 2_i128.pow(self.w as u32) as usize
    }
    pub fn find(&self, x: &T) -> bool {
        self.find_leaf(x).is_some()
    }
}

#[cfg(test)]
mod x_fast_trie_test {
    use crate::structs::tries::helper::rec_assert;

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
            table: table.clone(),
            w: max_depth,
        };

        let mut tree = XFastTrie::new(max_depth);
        tree.add(3);
        assert_eq!(tree, tobe);
        let mut root_right = StrongLinkNode::new_path_node();
        let mut root_right_left = StrongLinkNode::new_path_node();
        let mut root_right_left_left = StrongLinkNode::new_path_node();
        let leaf_9 = StrongLinkNode::new_leaf(9);
        root.set_jump(StrongLinkNode::new_path_node());
        root_right.set_jump(leaf_9.clone());
        root_right_left.set_jump(leaf_9.clone());
        root_right_left_left.set_jump(leaf_9.clone());
        root_right_left_left.set_right(leaf_9.clone());
        root_right_left.set_left(root_right_left_left.clone());
        root_right.set_left(root_right_left.clone());
        root.set_right(root_right.clone());
        leaf_3.set_next(leaf_9.clone());
        let mut min_prev = StrongLinkNode::new_path_node();
        let mut max_next = StrongLinkNode::new_path_node();
        min_prev.set_next(leaf_3.clone());
        max_next.set_prev(leaf_9.clone());
        table.register_binary_labels(1, &leaf_9.value().clone().unwrap(), root_right.clone());
        table.register_binary_labels(2, &leaf_9.value().clone().unwrap(), root_right_left.clone());
        table.register_binary_labels(
            3,
            &leaf_9.value().clone().unwrap(),
            root_right_left_left.clone(),
        );
        table.register_binary_labels(4, &leaf_9.value().clone().unwrap(), leaf_9.clone());
        let tobe = XFastTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
            table: table.clone(),
        };
        tree.add(9);
        rec_assert("root".to_string(), tree.root.clone(), tobe.root.clone());
        assert_eq!(tree, tobe);
        let mut leaf_1 = StrongLinkNode::new_leaf(1);
        let mut root_left_left_left = StrongLinkNode::new_path_node();
        root_left_left_left.set_right(leaf_1.clone());
        root_left_left_left.set_jump(leaf_1.clone());
        min_prev.set_next(leaf_1.clone());
        leaf_1.set_next(leaf_3.clone());
        root_left_left.set_jump(StrongLinkNode::new_none());
        root_left_left.set_left(root_left_left_left.clone());
        table.register_binary_labels(1, &leaf_1.value().clone().unwrap(), root_left.clone());
        table.register_binary_labels(2, &leaf_1.value().clone().unwrap(), root_left_left.clone());
        table.register_binary_labels(
            3,
            &leaf_1.value().clone().unwrap(),
            root_left_left_left.clone(),
        );
        table.register_binary_labels(4, &leaf_1.value().clone().unwrap(), leaf_1.clone());
        let tobe = XFastTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
            table: table.clone(),
        };
        tree.add(1);
        rec_assert("root".to_string(), tree.root.clone(), tobe.root.clone());
        assert_eq!(tree, tobe);

        let mut leaf_0 = StrongLinkNode::new_leaf(0);
        root_left_left_left.set_left(leaf_0.clone());
        root_left_left_left.set_jump(StrongLinkNode::new_none());
        min_prev.set_next(leaf_0.clone());
        leaf_0.set_next(leaf_1.clone());
        table.register_binary_labels(1, &leaf_0.value().clone().unwrap(), root_left.clone());
        table.register_binary_labels(2, &leaf_0.value().clone().unwrap(), root_left_left.clone());
        table.register_binary_labels(
            3,
            &leaf_0.value().clone().unwrap(),
            root_left_left_left.clone(),
        );
        table.register_binary_labels(4, &leaf_0.value().clone().unwrap(), leaf_0.clone());
        let tobe = XFastTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
            table: table.clone(),
        };

        tree.add(0);
        rec_assert("root".to_string(), tree.root.clone(), tobe.root.clone());
        assert_eq!(tree, tobe);
        let mut leaf_15 = StrongLinkNode::new_leaf(15);
        let mut root_right_right = StrongLinkNode::new_path_node();
        let mut root_right_right_right = StrongLinkNode::new_path_node();
        root_right_right_right.set_jump(leaf_15.clone());
        root_right_right_right.set_right(leaf_15.clone());
        root_right_right.set_jump(leaf_15.clone());
        root_right_right.set_right(root_right_right_right.clone());
        root_right.set_right(root_right_right.clone());
        root_right.set_jump(StrongLinkNode::new_none());
        max_next.set_prev(leaf_15.clone());
        leaf_15.set_prev(leaf_9.clone());
        table.register_binary_labels(1, &leaf_15.value().clone().unwrap(), root_right.clone());
        table.register_binary_labels(
            2,
            &leaf_15.value().clone().unwrap(),
            root_right_right.clone(),
        );
        table.register_binary_labels(
            3,
            &leaf_15.value().clone().unwrap(),
            root_right_right_right.clone(),
        );
        table.register_binary_labels(4, &leaf_15.value().clone().unwrap(), leaf_15.clone());
        let tobe = XFastTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
            table: table.clone(),
        };
        tree.add(15);
        rec_assert("root".to_string(), tree.root.clone(), tobe.root.clone());
        assert_eq!(tree, tobe);
    }
    #[test]
    fn find_test() {
        let mut tree = XFastTrie::new(4);
        for i in 0..16 {
            tree.add(i);
            assert!(tree.find(&i));
            //assert!(!tree.find(i));
        }
        let mut tree = XFastTrie::new(10);
        for i in 0..1000 {
            if i % 2 == 0 {
                tree.add(i);
            }
            if i % 2 == 1 {
                assert!(!tree.find(&i));
            }
            //assert!(!tree.find(i));
        }
    }
}
