use std::{
    cell::RefCell,
    fmt::Debug,
    ops::Deref,
    rc::{Rc, Weak},
};

use super::leaf::StrongLinkLeaf;

#[derive(Debug)]
pub(super) struct StrongLinkPathNode<T: Clone + PartialEq + Debug>(
    Option<Rc<RefCell<PathNode<T>>>>,
);
impl<T: Clone + PartialEq + Debug> StrongLinkPathNode<T> {
    pub fn new() -> Self {
        Self(Some(Rc::new(RefCell::new(PathNode::new()))))
    }
    pub fn new_none() -> Self {
        Self(None)
    }
    pub fn clone(&self) -> Self {
        Self(self.0.as_ref().map(|node| node.clone()))
    }
    pub fn to_weak(&self) -> WeakLinkPathNode<T> {
        WeakLinkPathNode::from_strong(self.clone())
    }
    pub fn from_weak(node: &WeakLinkPathNode<T>) -> Self {
        Self(node.0.as_ref().map(|node| node.upgrade()).unwrap_or(None))
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
            .map(|node| Self::from_weak(&node.borrow().parent))
            .unwrap_or(Self(None))
    }
    pub fn jump(&self) -> StrongLinkLeaf<T> {
        self.0
            .as_ref()
            .map(|node| node.borrow().jump.clone())
            .unwrap_or(StrongLinkLeaf::new_none())
    }
    pub fn set_jump(&mut self, leaf: StrongLinkLeaf<T>) {
        self.0.as_ref().map(|node| node.borrow_mut().jump = leaf);
    }
    pub fn remove_jump(&mut self) {
        self.0
            .as_ref()
            .map(|node| node.borrow_mut().jump = StrongLinkLeaf::new_none());
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
    pub fn get_min_child(&self) -> StrongLinkLeaf<T> {
        let mut node = self.clone();
        while node.left().is_some() {
            node = node.left();
        }
        node.jump()
    }
    pub fn get_max_child(&self) -> StrongLinkLeaf<T> {
        let mut node = self.clone();
        while node.right().is_some() {
            node = node.right();
        }
        node.jump()
    }
}

impl<T: Clone + PartialEq + Debug> Deref for StrongLinkPathNode<T> {
    type Target = Option<Rc<RefCell<PathNode<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone + PartialEq + Debug> PartialEq for StrongLinkPathNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug)]
pub(super) struct PathNode<T: Clone + PartialEq + Debug> {
    pub(super) parent: WeakLinkPathNode<T>,
    pub(super) children: [StrongLinkPathNode<T>; 2],
    pub(super) jump: StrongLinkLeaf<T>,
}
impl<T: Clone + PartialEq + Debug> PathNode<T> {
    pub fn new() -> Self {
        Self {
            parent: WeakLinkPathNode::new(),
            children: [
                StrongLinkPathNode::new_none(),
                StrongLinkPathNode::new_none(),
            ],
            jump: StrongLinkLeaf::new_none(),
        }
    }
}
impl<T: Clone + PartialEq + Debug> PartialEq for PathNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
            && self.parent.is_some() == other.parent.is_some()
            && self.jump.value() == other.jump.value()
    }
}

#[derive(Debug)]
pub(super) struct WeakLinkPathNode<T: Clone + PartialEq + Debug>(
    Option<Weak<RefCell<PathNode<T>>>>,
);
impl<T: Clone + PartialEq + Debug> WeakLinkPathNode<T> {
    pub fn new() -> Self {
        Self(None)
    }
    pub fn from_strong(node: StrongLinkPathNode<T>) -> Self {
        Self(node.0.map(|node| Rc::downgrade(&node)))
    }
}
impl<T: Clone + PartialEq + Debug> PartialEq for WeakLinkPathNode<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_node = self.0.as_ref().map(|node| node.upgrade());
        let other_node = other.0.as_ref().map(|node| node.upgrade());
        self_node == other_node
    }
}
impl<T: Clone + PartialEq + Debug> Deref for WeakLinkPathNode<T> {
    type Target = Option<Weak<RefCell<PathNode<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Clone + PartialEq + Debug> Drop for WeakLinkPathNode<T> {
    fn drop(&mut self) {
        println!("drop weak-node");
    }
}
