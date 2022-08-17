//use std::{collections::HashMap, fmt::Debug};

//use super::binary_trie::{BinaryTrie, StrongLinkNode, ToUsize};

//pub struct XFastTrie<T: ToUsize + Clone + Debug + PartialEq> {
//root: StrongLinkNode<T>,
//min_prev: StrongLinkNode<T>,
//max_next: StrongLinkNode<T>,
//map: HashMap<usize, StrongLinkNode<T>>,
//w: usize,
//}
//impl<T: ToUsize + Clone + Debug + PartialEq> XFastTrie<T> {
//pub fn new(w: usize) -> Self {
//let tree = BinaryTrie::new(w);
//let mut min_prev = StrongLinkNode::new_node();
//let max_next = StrongLinkNode::new_node();
//min_prev.set_next(max_next.clone());
//let root = StrongLinkNode::new_node();
//Self {
//map: HashMap::new(),
//}
//}
//pub fn add(&mut self, x: T) -> bool {
//true
//}
//pub fn find(&self, x: T) -> bool {
//true
//}
//}

//#[cfg(test)]
//mod x_fast_trie_test {
//use super::*;
//#[test]
//fn find_test() {
//let mut tree = XFastTrie::new(4);
//for i in 0..16 {
//tree.add(i);
//assert!(tree.find(i));
//}
//}
//}
