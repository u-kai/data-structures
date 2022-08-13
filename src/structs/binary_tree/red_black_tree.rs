use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use super::binary_tree::WrapNode;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum NodeColor {
    Red,
    Black,
}
impl NodeColor {
    fn new() -> Self {
        NodeColor::Black
    }
    fn change_color(&mut self, color: NodeColor) {
        *self = color
    }
    fn to_red(&mut self) {
        *self = NodeColor::Red
    }
    fn to_black(&mut self) {
        *self = NodeColor::Black
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RedBlackNodeValue<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> {
    value: T,
    color: NodeColor,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct RedBlackNode<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord>(
    WrapNode<RedBlackNodeValue<T>>,
);

impl<T> Deref for RedBlackNode<T>
where
    T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord,
{
    type Target = WrapNode<RedBlackNodeValue<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for RedBlackNode<T>
where
    T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> RedBlackNode<T>
where
    T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord,
{
    fn push_black(&mut self) {
        self.to_red();
        self.change_left_color(NodeColor::Black);
        self.change_right_color(NodeColor::Black);
    }
    fn pull_black(&mut self) {
        self.to_black();
        self.change_left_color(NodeColor::Red);
        self.change_right_color(NodeColor::Red);
    }
    fn change_left_color(&mut self, color: NodeColor) {
        self.left().as_ref().map(|node| {
            let mut value = node.value();
            value.color.change_color(color);
            node.change_value(value)
        });
    }
    fn change_right_color(&mut self, color: NodeColor) {
        self.right().as_ref().map(|node| {
            let mut value = node.value();
            value.color.change_color(color);
            node.change_value(value)
        });
    }
    fn to_red(&mut self) {
        let mut value = self.value();
        value.color.to_red();
        self.change_value(value);
    }
    fn to_black(&mut self) {
        let mut value = self.value();
        value.color.to_black();
        self.change_value(value);
    }
}

pub struct RedBlackTree<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> {
    root: RedBlackNode<T>,
}

#[cfg(test)]
mod red_black_tree_test {
    use crate::structs::binary_tree::{binary_tree::WrapNode, red_black_tree::*};

    #[test]
    fn add_test() {
        let mut node = RedBlackNode(WrapNode::new(RedBlackNodeValue {
            value: 1,
            color: NodeColor::new(),
        }));
        node.0.add_child(WrapNode::new(RedBlackNodeValue {
            value: 0,
            color: NodeColor::new(),
        }));
        node.0.add_child(WrapNode::new(RedBlackNodeValue {
            value: 2,
            color: NodeColor::new(),
        }));
        node.push_black();
        println!("{:#?}", node);

        node.pull_black();
        println!("{:#?}", node);
        assert!(false);
    }
}
