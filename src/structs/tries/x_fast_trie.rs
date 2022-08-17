use std::{collections::HashMap, fmt::Debug};

use super::{
    binary_trie::{Binary, ToUsize},
    nodes::strong_link::StrongLinkNode,
};
#[cfg(test)]
mod x_fast_trie_hashtable_test {
    use super::*;
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
