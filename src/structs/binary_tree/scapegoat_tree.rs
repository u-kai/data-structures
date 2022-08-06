use std::fmt::Debug;

use super::binary_tree::BinaryTree;

pub struct ScapegoatTree<T: Clone + Debug + PartialEq + Eq + PartialOrd + Ord> {
    tree: BinaryTree<T>,
    q: usize,
}
