use std::fmt::Debug;

use crate::structs::tries::binary_trie::PathNodeOrLeaf;

use super::{strong_link::StrongLinkNode, weak_link::WeakLinkNode};

#[derive(Debug)]
pub struct Node<T: Clone + PartialEq + Debug> {
    pub(super) x: PathNodeOrLeaf<T>,
    pub(super) children: [StrongLinkNode<T>; 2],
    pub(super) jump: StrongLinkNode<T>,
    pub(super) parent: WeakLinkNode<T>,
    pub(super) prev: WeakLinkNode<T>,
    pub(super) next: StrongLinkNode<T>,
}
impl<T: Clone + PartialEq + Debug> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.children == other.children
            && self.parent.value() == other.parent.value()
            && self.prev.value() == other.prev.value()
            && self.next.value() == other.next.value()
            && self.jump.value() == other.jump.value()
    }
}

impl<T: Clone + PartialEq + Debug> Node<T> {
    pub fn new_leaf(x: T) -> Self {
        Self {
            x: PathNodeOrLeaf::new_leaf(x),
            children: [StrongLinkNode::new_none(), StrongLinkNode::new_none()],
            jump: StrongLinkNode::new_none(),
            parent: WeakLinkNode::new_none(),
            prev: WeakLinkNode::new_none(),
            next: StrongLinkNode::new_none(),
        }
    }
    pub fn new_path_node() -> Self {
        Self {
            x: PathNodeOrLeaf::new_path(),
            children: [StrongLinkNode::new_none(), StrongLinkNode::new_none()],
            jump: StrongLinkNode::new_none(),
            parent: WeakLinkNode::new_none(),
            prev: WeakLinkNode::new_none(),
            next: StrongLinkNode::new_none(),
        }
    }
    pub fn value(&self) -> Option<&T> {
        self.x.value()
    }
}
