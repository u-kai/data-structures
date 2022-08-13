use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::binary_tree::{Tree, WrapNode};

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
type RedBlackNode<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> =
    WrapNode<RedBlackNodeValue<T>>;

#[derive(Debug)]
pub struct RedBlackTree<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> {
    root: Option<RedBlackNode<T>>,
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> Tree<RedBlackNodeValue<T>>
    for RedBlackTree<T>
{
    fn change_root(&mut self, node: WrapNode<RedBlackNodeValue<T>>) -> () {
        self.root = Some(node)
    }
}
impl<T> RedBlackTree<T>
where
    T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord,
{
    pub fn new() -> Self {
        Self { root: None }
    }
    pub fn add(&mut self, x: T) -> bool {
        let mut node = RedBlackNode::new(RedBlackNodeValue {
            value: x,
            color: NodeColor::Red,
        });
        if self.root.is_some() {
            let root = self.root.as_ref().unwrap().clone();
            let mut insert_node = root.find_last(node.value()).unwrap();
            let add_result = insert_node.add_child(node.clone());
            if add_result {
                self.add_fixup(node.clone())
            }
            add_result
        } else {
            //node.change_color(NodeColor::R);
            self.root = Some(node);
            true
        }
    }
    fn add_fixup(&mut self, mut node: RedBlackNode<T>) {
        while node.value().color == NodeColor::Red {
            if &node == self.root.as_ref().unwrap() {
                node.change_color(NodeColor::Black);
                return;
            }
            if let Some(mut parent) = node.parent() {
                if parent.left().map(|node| node.value().color) == Some(NodeColor::Black) {
                    parent.flip_left(self);
                    node = parent.clone();
                    parent = node.parent().unwrap();
                }
                if parent.value().color == NodeColor::Black {
                    return;
                }
                if let Some(mut grand_panrent) = parent.parent() {
                    if grand_panrent.right().map(|node| node.value().color)
                        == Some(NodeColor::Black)
                    {
                        grand_panrent.flip_right(self);
                        return;
                    } else {
                        grand_panrent.push_black();
                        node = grand_panrent;
                    }
                } else {
                    parent.change_color(NodeColor::Black);
                    return;
                };
            } else {
                node.change_color(NodeColor::Black);
                return;
            }
        }
    }
}

impl<T> RedBlackNode<T>
where
    T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord,
{
    fn flip_left(&mut self, tree: &mut RedBlackTree<T>) {
        if let Some(mut right) = self.right() {
            self.swap_color(&mut right);
            let node = self.clone();
            WrapNode::rotation_left(tree, node);
        };
    }
    fn flip_right(&mut self, tree: &mut RedBlackTree<T>) {
        if let Some(mut left) = self.left() {
            self.swap_color(&mut left);
            let node = self.clone();
            WrapNode::rotation_right(tree, node);
        };
    }
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
    fn swap_color(&mut self, other: &mut Self) {
        let self_color = self.value().color;
        let other_color = other.value().color;
        self.change_color(other_color);
        other.change_color(self_color);
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
    fn change_color(&mut self, color: NodeColor) {
        match color {
            NodeColor::Red => {
                self.to_red();
            }
            NodeColor::Black => {
                self.to_black();
            }
        }
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

#[cfg(test)]
mod red_black_tree_test {
    use crate::structs::binary_tree::{binary_tree::WrapNode, red_black_tree::*};

    #[test]
    fn add_test() {
        let mut tree = RedBlackTree::new();
        tree.add(2);
        tree.add(1);
        tree.add(5);
        //tree.add(3);
        //tree.add(4);
        //tree.add(6);
        //tree.add(7);
        println!("red_black_tree : {:#?}", tree);
    }
}
