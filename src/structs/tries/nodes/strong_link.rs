use std::{cell::RefCell, fmt::Debug, ops::Deref, rc::Rc};

use crate::structs::tries::binary_trie::PathNodeOrLeaf;

use super::{node::Node, weak_link::WeakLinkNode};

#[derive(Debug)]
pub struct StrongLinkNode<T: Clone + PartialEq + Debug>(Option<Rc<RefCell<Node<T>>>>);
impl<T: Clone + PartialEq + Debug> StrongLinkNode<T> {
    pub fn new_leaf(x: T) -> Self {
        Self(Some(Rc::new(RefCell::new(Node::new_leaf(x)))))
    }
    pub fn new(node: Option<Rc<RefCell<Node<T>>>>) -> Self {
        Self(node)
    }
    pub fn new_path_node() -> Self {
        Self(Some(Rc::new(RefCell::new(Node::new_path_node()))))
    }
    pub fn new_none() -> Self {
        Self(None)
    }
    pub fn clone(&self) -> Self {
        Self(self.0.as_ref().map(|node| node.clone()))
    }
    pub fn get_min_child(&self) -> Self {
        let mut node = self.clone();
        while node.left().is_some() {
            node = node.left();
        }
        if node.value().is_some() {
            node
        } else {
            node.jump()
        }
    }
    pub fn get_max_child(&self) -> Self {
        let mut node = self.clone();
        while node.right().is_some() {
            node = node.right();
        }
        if node.value().is_some() {
            node
        } else {
            node.jump()
        }
    }
    pub fn next(&self) -> Self {
        if let Some(next) = self.0.as_ref().map(|node| node.borrow().next.clone()) {
            next
        } else {
            Self(None)
        }
    }
    pub fn prev(&self) -> Self {
        if let Some(prev) = self
            .0
            .as_ref()
            .map(|node| node.borrow().prev.clone().to_node())
        {
            prev
        } else {
            Self(None)
        }
    }
    pub fn to_weak(&self) -> WeakLinkNode<T> {
        WeakLinkNode::new(self.0.as_ref().map(|node| Rc::downgrade(node)))
    }

    pub fn value(&self) -> Option<T> {
        if self.0.is_some() {
            match self.0.as_ref().unwrap().borrow().x.clone() {
                PathNodeOrLeaf::Leaf(x) => Some(x),
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn has_child(&self) -> bool {
        self.right().is_some() || self.left().is_some()
    }
    pub fn left(&self) -> Self {
        self.child(0)
    }
    pub fn right(&self) -> Self {
        self.child(1)
    }
    pub fn child(&self, index: usize) -> Self {
        self.0
            .as_ref()
            .map(|node| node.borrow().children[index].clone())
            .unwrap_or(Self::new_none())
    }
    pub fn parent(&self) -> Self {
        self.0
            .as_ref()
            .map(|node| node.borrow().parent.clone().to_node())
            .unwrap_or(Self(None))
    }
    pub fn jump(&self) -> Self {
        self.0
            .as_ref()
            .map(|node| node.borrow().jump.clone())
            .unwrap_or(Self(None))
    }
    pub fn set_jump(&mut self, leaf: Self) {
        self.0.as_ref().map(|node| node.borrow_mut().jump = leaf);
    }
    pub fn remove_jump(&mut self) {
        self.0
            .as_ref()
            .map(|node| node.borrow_mut().jump = StrongLinkNode::new_none());
    }
    pub fn set_next(&mut self, leaf: Self) {
        leaf.0
            .as_ref()
            .map(|node| node.borrow_mut().prev = self.clone().to_weak());
        self.0.as_ref().map(|node| node.borrow_mut().next = leaf);
    }
    pub fn set_prev(&mut self, leaf: Self) {
        leaf.0
            .as_ref()
            .map(|node| node.borrow_mut().next = self.clone());
        self.0
            .as_ref()
            .map(|node| node.borrow_mut().prev = leaf.to_weak());
    }
    pub fn set_child(&mut self, child: Self, index: usize) {
        if index == 0 {
            self.set_left(child);
            return;
        }
        if index == 1 {
            self.set_right(child);
            return;
        }
    }
    pub fn set_left(&mut self, child: Self) {
        child.clone().set_parent(self.clone());
        self.0
            .as_mut()
            .map(|parent| parent.borrow_mut().children[0] = child);
    }
    pub fn set_right(&mut self, child: Self) {
        child.clone().set_parent(self.clone());
        self.0
            .as_mut()
            .map(|parent| parent.borrow_mut().children[1] = child);
    }
    pub fn set_parent(&mut self, node: Self) {
        self.0
            .as_ref()
            .map(|this| this.borrow_mut().parent = node.to_weak());
    }
}
impl<T: Clone + PartialEq + Debug> Deref for StrongLinkNode<T> {
    type Target = Option<Rc<RefCell<Node<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone + PartialEq + Debug> PartialEq for StrongLinkNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
