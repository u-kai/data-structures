use std::fmt::Debug;

use super::{leaf::StrongLinkLeaf, node::StrongLinkPathNode};

pub trait ToUsize {
    fn to_usize(&self) -> usize;
}
#[derive(Debug, PartialEq)]
pub struct XFastTrie<T: ToUsize + Clone + PartialEq + Debug> {
    root: StrongLinkPathNode<T>,
    min_prev: StrongLinkLeaf<T>,
    max_next: StrongLinkLeaf<T>,
    w: usize,
}
impl<T: ToUsize + Clone + PartialEq + Debug> XFastTrie<T> {
    //fn new(w: usize) -> Self {
    //let root = StrongLinkPathNode::new();
    //let mut min_prev = StrongLinkLeaf::new_dummy();
    //let mut max_prev = StrongLinkLeaf::new_dummy();
    //}
}

#[cfg(test)]
mod x_fast_trie_test {
    use super::*;
    impl ToUsize for u32 {
        fn to_usize(&self) -> usize {
            *self as usize
        }
    }
    //#[test]
    //fn add_test() {
    //let mut root = StrongLinkPathNode::new();
    //let mut leaf_3 = StrongLinkLeaf::new(3);
    //let mut min_prev = StrongLinkLeaf::new_dummy();
    //let mut max_next = StrongLinkLeaf::new_dummy();
    //min_prev.set_next(leaf_3.clone());
    //max_next.set_prev(leaf_3.clone());

    //let mut root_left_child = StrongLinkPathNode::new();
    //let mut root_left_child_left_child = StrongLinkPathNode::new();
    //let mut root_left_child_left_child_right_child = StrongLinkPathNode::new();

    //root.set_jump(leaf_3.clone());
    //root_left_child.set_jump(leaf_3.clone());
    //root_left_child_left_child.set_jump(leaf_3.clone());
    //root_left_child_left_child_right_child.set_jump(leaf_3.clone());
    //root_left_child_left_child.set_right(root_left_child_left_child_right_child.clone());
    //root_left_child.set_left(root_left_child_left_child.clone());
    //root.set_left(root_left_child.clone());

    //let tobe: XFastTrie<u32> = XFastTrie {
    //root: root.clone(),
    //min_prev,
    //max_next,
    //w: 4,
    //};

    //let mut tree = XFastTrie::new(4);
    //tree.add(3);
    //assert_eq!(tree, tobe);
    //}
}
