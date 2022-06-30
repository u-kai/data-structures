use std::fmt::Debug;

use crate::types::link::StrongLink;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Node<T: Clone + Debug + Eq + PartialEq> {
    pub x: T,
    pub next: Option<StrongLink<Node<T>>>,
}

impl<T: Clone + Debug + Eq + PartialEq> Node<T> {
    pub fn new(x: T) -> Self {
        Node { x, next: None }
    }
    pub fn set_next(&mut self, next: StrongLink<Node<T>>) {
        self.next = Some(next)
    }
}
