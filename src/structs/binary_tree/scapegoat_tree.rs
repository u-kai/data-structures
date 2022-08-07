use std::fmt::Debug;

use super::binary_tree::WrapNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScapegoatTree<T: Clone + Debug + PartialEq + Eq + PartialOrd + Ord> {
    root: WrapNode<T>,
    q: usize,
}

impl<T: Clone + Debug + PartialEq + Eq + PartialOrd + Ord> ScapegoatTree<T> {
    pub fn add(&mut self, x: T) -> bool {
        false
    }
    fn rebuild(&mut self, node: WrapNode<T>) {
        let ns = node.size();
        let mut array: Vec<Option<WrapNode<T>>> = vec![None; ns];
        Self::pack_into_array(Some(node.clone()), &mut array, 0);
        //if let Some(parent) = node.parent(){

        //}else{
        //self.root =
        //};
    }
    fn build_balanced(
        mut parent: Option<WrapNode<T>>,
        array: Vec<WrapNode<T>>,
        middle: usize,
    ) -> Option<WrapNode<T>> {
        if array.len() == 0 {
            return None;
        }
        let mut root = array[middle].clone();
        let left_array = array
            .iter()
            .map(|node| node.clone())
            .take(middle)
            .collect::<Vec<_>>();
        let left_middle = left_array.len() / 2;
        let right_array = array
            .iter()
            .map(|node| node.clone())
            .skip(middle + 1)
            .collect::<Vec<_>>();
        let right_middle = right_array.len() / 2;
        let left_node = Self::build_balanced(Some(root.clone()), left_array, left_middle);
        let right_node = Self::build_balanced(Some(root.clone()), right_array, right_middle);
        root.set_left(left_node);
        root.set_right(right_node);
        root.set_parent(parent.as_ref().map(|node| node.clone()));
        parent.as_mut().map(|parent| parent.add_child(root.clone()));
        Some(root)
    }
    fn pack_into_array(
        node: Option<WrapNode<T>>,
        node_array: &mut Vec<Option<WrapNode<T>>>,
        i: usize,
    ) -> usize {
        if let Some(node) = node {
            let i = Self::pack_into_array(node.left(), node_array, i);
            *node_array.get_mut(i).unwrap() = Some(node.clone());

            Self::pack_into_array(node.right(), node_array, i + 1)
        } else {
            i
        }
    }
    fn log3_2(q: usize) -> usize {
        (q as f64).log(3.0 / 2.0).floor() as usize
    }
}

#[cfg(test)]
mod scapegoat_tree_test {
    use super::*;
    fn make_full_node() -> WrapNode<i32> {
        let mut root = WrapNode::new(7);
        root.add(2);
        root.add(6);
        root.add(4);
        root.add(1);
        root.add(0);
        root
    }
    fn make_full_node2() -> WrapNode<i32> {
        let mut root = WrapNode::new(9);
        root.add(8);
        root.add(7);
        root.add(10);
        root.add(11);
        root.add(2);
        root.add(6);
        root.add(4);
        root.add(1);
        root.add(0);
        root
    }
    fn make_full_tree() -> ScapegoatTree<i32> {
        let mut tree = ScapegoatTree {
            root: make_full_node(), //WrapNode::new(9),
            q: 10,
        };
        //tree.root.add(8);
        //tree.root.add(7);
        //tree.root.add(10);
        //tree.root.add(11);
        //tree.root.add(2);
        //tree.root.add(6);
        //tree.root.add(4);
        //tree.root.add(1);
        //tree.root.add(0);
        tree
    }
    #[test]
    fn add_test() {
        let mut tree = make_full_tree();
        tree.add(5);
        let mut tobe = WrapNode::new(9);
        tobe.add(10);
        tobe.add(11);
        tobe.add(8);
        tobe.add(4);
        tobe.add(6);
        tobe.add(7);
        tobe.add(5);
        tobe.add(1);
        tobe.add(2);
        tobe.add(0);
        let tobe = ScapegoatTree { root: tobe, q: 10 };
        //assert_eq!(tree, tobe);
    }
    #[test]
    fn rebuild_test() {
        let mut node = make_full_node();
        node.add(5);
        let mut node_array = vec![None; node.size()];
        ScapegoatTree::pack_into_array(Some(node.clone()), &mut node_array, 0);
        let array = node_array
            .iter()
            .map(|node| node.as_ref().unwrap().clone())
            .collect();
        let node = ScapegoatTree::build_balanced(None, array, node.size() / 2);
        let mut tobe = WrapNode::new(4);
        tobe.add(6);
        tobe.add(7);
        tobe.add(5);
        tobe.add(1);
        tobe.add(2);
        tobe.add(0);
        assert_eq!(node, Some(tobe));
    }
    #[test]
    fn pack_into_array_test() {
        let mut array: Vec<Option<WrapNode<i32>>> = vec![None; 5];
        let mut root = WrapNode::new(7);
        root.add(4);
        root.add(6);
        root.add(5);
        root.add(3);
        ScapegoatTree::pack_into_array(Some(root), &mut array, 0);
        assert_eq!(
            vec![Some(3), Some(4), Some(5), Some(6), Some(7)],
            array
                .iter()
                .map(|node| node.as_ref().map(|node| node.value()))
                .collect::<Vec<_>>()
        );
    }
}
