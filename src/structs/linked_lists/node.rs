use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Node<T: Clone + Debug + PartialEq + Eq> {
    pub x: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone + Debug + PartialEq + Eq> Node<T> {
    pub fn new(x: T) -> Self {
        Node { x, next: None }
    }
    pub fn set_next(&mut self, next: Rc<RefCell<Node<T>>>) {
        self.next = Some(next)
    }
}
