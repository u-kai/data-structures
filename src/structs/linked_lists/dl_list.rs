use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::types::link::{StrongLink, WeakLink};
#[derive(Debug)]
pub struct DLList<T: Default + Clone + Debug + Eq + PartialEq> {
    n: isize,
    head: NodeWrapper<T>,
    tail: NodeWrapper<T>,
}

impl<T: Clone + Debug + Eq + PartialEq + Default> DLList<T> {
    pub fn new() -> Self {
        let head = NodeWrapper::new(Default::default());
        let tail = NodeWrapper::new(Default::default());
        head.change_next(&tail);
        tail.change_prev(&head);
        DLList { n: -1, head, tail }
    }
    pub fn add(&mut self, index: usize, x: T) {
        let node = NodeWrapper::new(x);
        if index as isize > (self.n + 1) {
            panic!("do not adding index : {} ", index)
        }
        if index as isize > self.n {
            self.tail.change_prev(&node);
            self.n += 1;
            return;
        }
        if index == 0 {
            self.head.change_next(&node);
            self.n += 1;
            return;
        }
        let old_node = self.get_node(index).unwrap();
        old_node.change_prev(&node)
    }
    pub fn remove(&mut self, index: usize) -> Option<T> {
        let result = self.get_node(index).map(|node| node.value());
        self.get_node(index).map(|node| node.delete());
        self.n = self.n - 1;
        result
    }
    pub fn get(&self, index: usize) -> Option<T> {
        self.get_node(index).map(|node| node.value())
    }
    pub fn set(&self, index: usize, x: T) {
        if let Some(node) = self.get_node(index) {
            node.0.borrow_mut().x = x
        }
    }
    fn get_node(&self, index: usize) -> Option<NodeWrapper<T>> {
        if index as isize > self.n {
            return None;
        }
        let mut node = self.head.next();
        for _ in 0..index {
            if node.is_none() {
                return None;
            }
            node = node.unwrap().next()
        }
        node
    }
}

#[derive(Debug, Clone)]
struct NodeWrapper<T: Default + Clone + Debug + Eq + PartialEq>(Rc<RefCell<Node<T>>>);
impl<T: Default + Clone + Debug + Eq + PartialEq> NodeWrapper<T> {
    fn new(x: T) -> Self {
        NodeWrapper(Rc::new(RefCell::new(Node::new(x))))
    }
    fn value(&self) -> T {
        self.0.borrow().x.clone()
    }
    fn delete(self) {
        let prev = self.prev();
        let next = self.next();
        if let (Some(prev), Some(next)) = (prev, next) {
            next.change_prev(&prev);
        }
        drop(self)
    }
    fn next(&self) -> Option<Self> {
        let n = &self.0.borrow().next; //.map(|node| NodeWrapper(node))
        if let Some(next) = n {
            return Some(NodeWrapper(next.clone()));
        }
        None
    }
    fn prev(&self) -> Option<Self> {
        let n = &self.0.borrow().prev; //.map(|node| NodeWrapper(node))
        if let Some(prev) = n {
            if let Some(prev) = prev.upgrade() {
                return Some(NodeWrapper(prev));
            }
        }
        None
    }
    fn change_prev(&self, node: &NodeWrapper<T>) {
        {
            let old_prev = &self.0.borrow().prev;
            if let Some(old_prev) = old_prev {
                if let Some(old_prev) = old_prev.upgrade() {
                    old_prev.borrow_mut().next = Some(node.0.clone());
                    node.0.borrow_mut().prev = Some(Rc::downgrade(&old_prev));
                }
            }
        }
        self.0.borrow_mut().prev = Some(Rc::downgrade(&node.0));
        node.0.borrow_mut().next = Some(self.0.clone());
    }
    fn change_next(&self, node: &NodeWrapper<T>) {
        {
            let old_next = &self.0.borrow().next;
            if let Some(old_next) = old_next {
                old_next.borrow_mut().prev = Some(Rc::downgrade(&node.0));
                node.0.borrow_mut().next = Some(old_next.clone());
            }
        }
        self.0.borrow_mut().next = Some(node.0.clone());
        node.0.borrow_mut().prev = Some(Rc::downgrade(&self.0));
    }
}
impl<T: Default + Clone + Debug + Eq + PartialEq> Drop for NodeWrapper<T> {
    fn drop(&mut self) {}
}
#[derive(Default, Clone, Debug)]
struct Node<T: Default + Clone + Debug + Eq + PartialEq> {
    x: T,
    prev: Option<WeakLink<Node<T>>>,
    next: Option<StrongLink<Node<T>>>,
}

impl<T: Default + Clone + Debug + Eq + PartialEq> Node<T> {
    fn new(x: T) -> Self {
        Node {
            x,
            prev: None,
            next: None,
        }
    }
}

#[cfg(test)]
mod dl_list_test {

    use super::*;
    #[test]
    fn set_test() {
        let mut list = DLList::new();
        list.add(0, "*****");
        list.add(1, "hello");
        list.add(2, "world");
        list.add(3, "*****");
        list.set(0, "#####");
        assert_eq!(list.get(0).unwrap(), "#####");
        assert_eq!(list.get(1).unwrap(), "hello");
        assert_eq!(list.get(2).unwrap(), "world");
        assert_eq!(list.get(3).unwrap(), "*****");
        assert_eq!(list.get(4), None);
    }
    #[test]
    fn get_test() {
        let mut list = DLList::new();
        list.add(0, "*****");
        list.add(1, "hello");
        list.add(2, "world");
        assert_eq!(list.get(0), Some("*****"));
        assert_eq!(list.get(1), Some("hello"));
        assert_eq!(list.get(2), Some("world"));
        assert_eq!(list.get(4), None);
    }
    #[test]
    fn get_node_test() {
        let mut list = DLList::new();
        list.add(0, "*****");
        list.add(1, "hello");
        list.add(2, "world");
        assert_eq!(list.get_node(0).unwrap().value(), "*****");
        assert_eq!(list.get_node(1).unwrap().value(), "hello");
        assert_eq!(list.get_node(2).unwrap().value(), "world");
    }
    #[test]
    fn remove_test() {
        let mut list = DLList::new();
        list.add(0, "*****");
        list.add(1, "hello");
        list.add(2, "world");
        list.add(3, "*****");
        let remove = list.remove(0);
        assert_eq!(remove.unwrap(), "*****");
        assert_eq!(list.get(0).unwrap(), "hello");
        assert_eq!(list.get(1).unwrap(), "world");
        assert_eq!(list.get(2).unwrap(), "*****");
        assert_eq!(list.get(3), None);
    }
    #[test]
    fn node_wrapper_test() {
        let node_wrapper = NodeWrapper::new("hello");
        let n2 = NodeWrapper::new("world");
        let n3 = NodeWrapper::new("a");
        let n4 = NodeWrapper::new("goodbye");
        node_wrapper.change_prev(&n3);
        node_wrapper.change_next(&n2);
        n2.change_next(&n4);
        assert_eq!(node_wrapper.prev().unwrap().value(), n3.value());
        assert_eq!(node_wrapper.next().unwrap().value(), n2.value());
        assert_eq!(n2.next().unwrap().value(), n4.value());
    }
}
