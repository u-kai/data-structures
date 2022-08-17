use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Leaf<T: Clone + PartialEq + Debug> {
    x: Option<T>,
    next: StrongLinkLeaf<T>,
    prev: WeakLinkLeaf<T>,
}
impl<T: Clone + PartialEq + Debug> Leaf<T> {
    pub fn new(x: T) -> Self {
        Self {
            x: Some(x),
            next: StrongLinkLeaf::new_none(),
            prev: WeakLinkLeaf::new_none(),
        }
    }
    pub fn new_dummy() -> Self {
        Self {
            x: None,
            next: StrongLinkLeaf::new_none(),
            prev: WeakLinkLeaf::new_none(),
        }
    }
    pub fn value(&self) -> &Option<T> {
        &self.x
    }
}
impl<T: Clone + PartialEq + Debug> PartialEq for Leaf<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.prev.value() == other.prev.value()
            && self.next.value() == other.next.value()
    }
}

#[derive(Debug)]
pub(super) struct WeakLinkLeaf<T: Clone + PartialEq + Debug>(Option<Weak<RefCell<Leaf<T>>>>);
impl<T: Clone + PartialEq + Debug> WeakLinkLeaf<T> {
    pub fn new_none() -> Self {
        Self(None)
    }
    pub fn new(node: Option<Weak<RefCell<Leaf<T>>>>) -> Self {
        Self(node)
    }
    pub fn value(&self) -> Option<T> {
        if let Some(node) = self.0.as_ref() {
            if let Some(node) = node.upgrade() {
                return node.borrow().value().clone();
            }
        }
        None
    }
}
#[derive(Debug, PartialEq)]
pub(super) struct StrongLinkLeaf<T: Clone + PartialEq + Debug>(Option<Rc<RefCell<Leaf<T>>>>);
impl<T: Clone + PartialEq + Debug> StrongLinkLeaf<T> {
    pub fn new(x: T) -> Self {
        Self(Some(Rc::new(RefCell::new(Leaf::new(x)))))
    }
    pub fn new_dummy() -> Self {
        Self(Some(Rc::new(RefCell::new(Leaf::new_dummy()))))
    }
    pub fn new_none() -> Self {
        Self(None)
    }
    pub fn clone(&self) -> Self {
        Self(self.0.as_ref().map(|node| node.clone()))
    }
    pub fn set_next(&mut self, leaf: Self) {
        leaf.0
            .as_ref()
            .map(|node| node.borrow_mut().prev = self.to_weak());
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
    pub fn to_weak(&self) -> WeakLinkLeaf<T> {
        WeakLinkLeaf::new(self.0.as_ref().map(|node| Rc::downgrade(&node)))
    }
}

impl<T: Clone + PartialEq + Debug> StrongLinkLeaf<T> {
    pub fn value(&self) -> Option<T> {
        self.0
            .as_ref()
            .map(|node| node.borrow().x.clone())
            .unwrap_or(None)
    }
}
