use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    interfaces::{queue::Queue, stack::Stack},
    types::link::StrongLink,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SLList<T: Clone + Debug + Eq + PartialEq> {
    n: usize,
    tail: Option<StrongLink<Node<T>>>,
    head: Option<StrongLink<Node<T>>>,
}
impl<T: Clone + Debug + Eq + PartialEq> SLList<T> {
    #[allow(unused)]
    pub fn new() -> Self {
        SLList {
            n: 0,
            tail: None,
            head: None,
        }
    }
    #[allow(unused)]
    pub fn new_with(xs: Vec<T>) -> Self {
        let mut sl_list = SLList::new();
        for x in xs {
            sl_list.push(x)
        }
        sl_list
    }
    #[allow(unused)]
    pub fn size(&self) -> usize {
        self.n
    }
}
impl<T: Clone + Debug + Eq + PartialEq> Queue<T> for SLList<T> {
    fn add(&mut self, x: T) {
        let node = Rc::new(RefCell::new(Node::new(x)));
        if self.n == 0 {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());
            self.n += 1;
            return;
        }
        let old_tail = self.tail.take().unwrap();
        self.tail = Some(node.clone());
        old_tail.borrow_mut().set_next(node);
        self.n += 1;
    }
    fn remove(&mut self) -> Option<T> {
        self.pop()
    }
}

impl<T: Clone + Debug + Eq + PartialEq> Stack<T> for SLList<T> {
    fn push(&mut self, x: T) {
        let node = Rc::new(RefCell::new(Node::new(x)));
        if let Some(head) = self.head.take() {
            node.borrow_mut().set_next(head);
        }
        self.head = Some(node.clone());
        if self.n == 0 {
            self.tail = Some(node.clone());
        }
        self.n += 1;
    }
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(new_head) = head.borrow_mut().next.take() {
                self.head = Some(new_head);
            } else {
                self.tail = None;
            };
            self.n -= 1;
            Rc::try_unwrap(head).unwrap().into_inner().x
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Node<T: Clone + Debug + Eq + PartialEq> {
    pub x: T,
    pub next: Option<StrongLink<Node<T>>>,
}

impl<T: Clone + Debug + Eq + PartialEq> Node<T> {
    fn new(x: T) -> Self {
        Node { x, next: None }
    }
    fn set_next(&mut self, next: StrongLink<Node<T>>) {
        self.next = Some(next)
    }
}

#[cfg(test)]
mod sl_list_tests {
    use super::*;
    #[test]
    fn add_test() {
        let mut list = SLList::new();
        list.add("hello");
        list.add("world");
        let head = Rc::new(RefCell::new(Node::new("hello")));
        let tail = Rc::new(RefCell::new(Node::new("world")));
        head.borrow_mut().set_next(tail.clone());
        assert_eq!(
            list,
            SLList {
                n: 2,
                head: Some(head),
                tail: Some(tail)
            }
        )
    }
    #[test]
    fn pop_test() {
        let mut list = SLList::new();
        list.push("hello");
        list.push("world");
        assert_eq!(list.pop(), Some("world"));
        let node = Some(Rc::new(RefCell::new(Node::new("hello"))));
        assert_eq!(
            list,
            SLList {
                n: 1,
                head: node.clone(),
                tail: node.clone(),
            }
        )
    }
    #[test]
    fn push_test() {
        let mut list = SLList::new();
        list.push(1);
        list.push(2);
        let head = Rc::new(RefCell::new(Node::new(2)));
        let tail = Rc::new(RefCell::new(Node::new(1)));
        head.borrow_mut().next = Some(tail.clone());
        assert_eq!(
            list,
            SLList {
                n: 2,
                head: Some(head),
                tail: Some(tail),
            }
        );
    }
}
