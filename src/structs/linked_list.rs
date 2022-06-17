use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node<T: Clone + Debug + PartialEq + Eq> {
    x: T,
    next: Option<Box<Node<T>>>,
}
fn node_to_node<T: Clone + Debug + PartialEq + Eq>(node: Node<T>) -> Node<T> {
    let x = node.x;
    let next = node.next;
    Node { x, next }
}
