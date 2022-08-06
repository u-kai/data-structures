use std::fmt::Debug;

use super::binary_tree::WrapNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScapegoatTree<T: Clone + Debug + PartialEq + Eq + PartialOrd + Ord> {
    tree: WrapNode<T>,
    q: usize,
}

#[cfg(test)]
mod scapegoat_tree_test {
    use super::*;
    fn make_full_tree() -> ScapegoatTree<i32> {
        let mut tree = ScapegoatTree {
            tree: WrapNode::new(9),
            q: 10,
        };
        tree.tree.add(8);
        tree.tree.add(7);
        tree.tree.add(10);
        tree.tree.add(11);
        tree.tree.add(2);
        tree.tree.add(6);
        tree.tree.add(4);
        tree.tree.add(1);
        tree.tree.add(0);
        tree
    }
    #[test]
    fn add_test() {
        let mut tree = make_full_tree();
        println!("{:#?}", tree);
        //tree.add(5);
        assert!(false);
    }
}
