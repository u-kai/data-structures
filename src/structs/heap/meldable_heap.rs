use std::{
    cell::RefCell,
    fmt::Debug,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::structs::binary_tree::random_binary_search_tree::RandomGenerator;

#[derive(Debug)]
pub struct MeldableHeap<T: Clone + PartialEq + PartialOrd + Debug, R: RandomGenerator> {
    root: WrapNode<T>,
    rand_gen: R,
}
impl<T: Clone + PartialEq + PartialOrd + Debug, R: RandomGenerator> MeldableHeap<T, R> {
    pub fn new(rand_gen: R) -> Self {
        Self {
            root: WrapNode(None),
            rand_gen,
        }
    }
    pub fn add(&mut self, x: T) -> bool {
        let mut node = WrapNode::new(x);
        let root = node.merge(self.root.clone(), &mut self.rand_gen);
        self.root = root;
        self.root.set_parent(WrapNode(None));
        true
    }
    pub fn remove(&mut self) -> Option<T> {
        let x = self.root.value();
        self.root = self
            .root
            .left()
            .merge(self.root.right(), &mut self.rand_gen);
        if self.root.is_none() {
            self.root.set_parent(WrapNode(None))
        }
        x
    }
}

#[derive(Debug)]
struct WrapNode<T: Clone + PartialEq + PartialOrd + Debug>(
    Option<Rc<RefCell<MeldableHeapNode<T>>>>,
);
impl<T: Clone + PartialEq + PartialOrd + Debug> Deref for WrapNode<T> {
    type Target = Option<Rc<RefCell<MeldableHeapNode<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Clone + PartialEq + PartialOrd + Debug> WrapNode<T> {
    fn new(x: T) -> Self {
        WrapNode(Some(Rc::new(RefCell::new(MeldableHeapNode::new(x)))))
    }
    fn merge(&mut self, mut other: Self, rand: &mut impl RandomGenerator) -> Self {
        if self.is_none() {
            return other;
        }
        if other.is_none() {
            return self.clone();
        }
        if self.value() > other.value() {
            return other.merge(self.clone(), rand);
        }
        if rand.gen_rand() % 2 == 0 {
            let left = self.left().merge(other, rand);
            self.set_left(left);
            if self.left().is_some() {
                self.left().set_parent(self.clone());
            }
        } else {
            let right = self.right().merge(other, rand);
            self.set_right(right);
            if self.right().is_some() {
                self.right().set_parent(self.clone());
            }
        }
        self.clone()
    }
    fn value(&self) -> Option<T> {
        self.as_ref().map(|node| node.borrow().value.clone())
    }
    fn left(&self) -> Self {
        if self.is_some() {
            self.as_ref().unwrap().borrow().left.clone()
        } else {
            WrapNode(None)
        }
    }
    fn right(&self) -> Self {
        if self.is_some() {
            self.as_ref().unwrap().borrow().right.clone()
        } else {
            WrapNode(None)
        }
    }
    fn parent(&self) -> Self {
        if self.is_some() {
            if self.as_ref().unwrap().borrow().parent.is_some() {
                if let Some(parent) = self
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .parent
                    .as_ref()
                    .unwrap()
                    .upgrade()
                {
                    WrapNode(Some(parent))
                } else {
                    WrapNode(None)
                }
            } else {
                WrapNode(None)
            }
        } else {
            WrapNode(None)
        }
    }
    fn set_left(&mut self, left: Self) {
        self.as_ref().map(|node| node.borrow_mut().left = left);
    }
    fn set_right(&mut self, right: Self) {
        self.as_ref().map(|node| node.borrow_mut().right = right);
    }
    fn set_parent(&mut self, parent: Self) {
        if let Some(parent) = parent.0 {
            self.as_ref()
                .map(|node| node.borrow_mut().parent = Some(Rc::downgrade(&parent)));
        } else {
            self.as_ref().map(|node| node.borrow_mut().parent = None);
        }
    }
    fn clone(&self) -> Self {
        match self.as_ref() {
            Some(node) => WrapNode(Some(node.clone())),
            None => WrapNode(None),
        }
    }
}
#[derive(Debug)]
struct MeldableHeapNode<T: Clone + PartialEq + PartialOrd + Debug> {
    value: T,
    left: WrapNode<T>,
    right: WrapNode<T>,
    parent: Option<Weak<RefCell<MeldableHeapNode<T>>>>,
}
impl<T: Clone + PartialEq + PartialOrd + Debug> MeldableHeapNode<T> {
    fn new(x: T) -> Self {
        Self {
            value: x,
            left: WrapNode(None),
            right: WrapNode(None),
            parent: None,
        }
    }
}

#[cfg(test)]
mod meldable_heap_test {
    struct RandomGeneratorMock(Vec<usize>);

    impl RandomGenerator for RandomGeneratorMock {
        fn gen_rand(&mut self) -> usize {
            self.0.remove(0)
        }
    }

    use rand::{prelude::ThreadRng, thread_rng, Rng};

    use super::*;
    #[test]
    fn test() {
        let rand_source = (0..10000).collect::<Vec<_>>();
        let mut tree = MeldableHeap::new(RandomGeneratorMock(rand_source));
        for i in 0..100 {
            tree.add(i);
        }
        for i in 0..100 {
            assert_eq!(tree.remove(), Some(i));
        }
        #[derive(Debug)]
        struct RealRand {
            rand: ThreadRng,
        }
        impl RandomGenerator for RealRand {
            fn gen_rand(&mut self) -> usize {
                self.rand.gen::<usize>()
            }
        }
        let rand_gen = RealRand { rand: thread_rng() };
        let mut tree = MeldableHeap::new(rand_gen);
        for i in 0..100 {
            tree.add(i);
        }
        for i in 0..100 {
            assert_eq!(tree.remove(), Some(i));
        }
    }
}
