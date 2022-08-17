use std::{cell::RefCell, fmt::Debug, ops::Deref, rc::Weak};

use super::{node::Node, strong_link::StrongLinkNode};

#[derive(Debug)]
pub struct WeakLinkNode<T: Clone + PartialEq + Debug>(Option<Weak<RefCell<Node<T>>>>);
impl<T: Clone + PartialEq + Debug> WeakLinkNode<T> {
    pub fn new_none() -> Self {
        Self(None)
    }
    pub fn new(weak_link: Option<Weak<RefCell<Node<T>>>>) -> Self {
        Self(weak_link)
    }
    fn parent(&self) -> Option<Option<StrongLinkNode<T>>> {
        self.0
            .as_ref()
            .map(|node| node.upgrade().map(|up| up.borrow().jump.clone()))
    }
    pub fn value(&self) -> Option<T> {
        if let Some(node) = self.0.as_ref() {
            if let Some(node) = node.upgrade() {
                return node.borrow().value().cloned();
            }
        }
        None
    }
    pub fn clone(&self) -> Self {
        WeakLinkNode(self.0.as_ref().map(|leaf| leaf.clone()))
    }
    pub fn to_node(self) -> StrongLinkNode<T> {
        self.0
            .as_ref()
            .map(|leaf| {
                let leaf = leaf.upgrade();
                StrongLinkNode::new(leaf)
            })
            .unwrap_or(StrongLinkNode::new_none())
    }
}
impl<T: Clone + PartialEq + Debug> PartialEq for WeakLinkNode<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_node = self.0.as_ref().map(|node| node.upgrade());
        let other_node = other.0.as_ref().map(|node| node.upgrade());
        self_node == other_node
    }
}
impl<T: Clone + PartialEq + Debug> Deref for WeakLinkNode<T> {
    type Target = Option<Weak<RefCell<Node<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Clone + PartialEq + Debug> Drop for WeakLinkNode<T> {
    fn drop(&mut self) {
        //println!("drop node = {:?}", self.value());
        //println!("{:?}", self.parent());
    }
}
