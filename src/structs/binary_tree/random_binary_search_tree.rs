use std::{
    cell::RefCell,
    fmt::Debug,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::interfaces::sset::SSet;

pub trait RandomGenerator {
    fn gen_rand(&mut self) -> usize;
}
#[derive(Debug, Clone)]
pub struct Treap<T: Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord, R: RandomGenerator>
{
    root: WrapNode<T>,
    random_generator: R,
}

impl<
        T: Default + Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord,
        R: RandomGenerator,
    > Treap<T, R>
{
    fn new(root: T, mut random_generator: R) -> Self {
        let rand = random_generator.gen_rand();
        Self {
            root: WrapNode::new(root, rand),
            random_generator,
        }
    }
}
#[derive(Debug, Clone, Eq)]
struct WrapNode<T: Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord>(
    Rc<RefCell<TreapNode<T>>>,
);
impl<T: Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord> Deref for WrapNode<T> {
    type Target = Rc<RefCell<TreapNode<T>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord> WrapNode<T> {
    fn new(value: T, p: usize) -> Self {
        Self(Rc::new(RefCell::new(TreapNode::new(value, p))))
    }
    fn rotation_right(&mut self) {}
    fn rotation_left(&mut self) {}
    fn parent(&self) -> Option<Self> {
        if let Some(parent) = &self.borrow().parent {
            let parent = parent.upgrade().as_ref().unwrap().clone();
            Some(WrapNode(parent))
        } else {
            None
        }
    }
    fn left(&self) -> Option<Self> {
        if let Some(left) = self.borrow().left.as_ref() {
            let left = left.to_node().clone();
            Some(WrapNode(left))
        } else {
            None
        }
    }
    fn right(&self) -> Option<Self> {
        if let Some(right) = self.borrow().right.as_ref() {
            let right = right.to_node().clone();
            Some(WrapNode(right))
        } else {
            None
        }
    }
    fn from_node(node: TreapNode<T>) -> Self {
        Self(Rc::new(RefCell::new(node)))
    }
    fn from_rc_node(node: Rc<RefCell<TreapNode<T>>>) -> Self {
        Self(node)
    }
    fn to_node(&self) -> Rc<RefCell<TreapNode<T>>> {
        self.0.clone()
    }
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord + Default> PartialEq for WrapNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone)]
struct TreapNode<T: Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord> {
    value: T,
    left: Option<WrapNode<T>>,
    right: Option<WrapNode<T>>,
    parent: Option<Weak<RefCell<TreapNode<T>>>>,
    p: usize,
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord + Default> PartialEq for TreapNode<T> {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(parent), Some(other_parent)) = (self.parent.as_ref(), other.parent.as_ref()) {
            let parent_value = &parent.upgrade();
            let parent_value = &parent_value.as_ref().unwrap().borrow().value;
            let other_parent_value = &other_parent.upgrade();
            let other_parent_value = &other_parent_value.as_ref().unwrap().borrow().value;
            return parent_value == other_parent_value
                && self.value == other.value
                && self.left == other.left
                && self.right == other.right
                && self.p == other.p;
        }
        self.value == other.value && self.left == other.left && self.right == other.right
    }
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord + Default> Eq for TreapNode<T> {
    fn assert_receiver_is_total_eq(&self) {}
}
impl<T: Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord> TreapNode<T> {
    fn new(value: T, p: usize) -> Self {
        Self {
            value,
            left: None,
            right: None,
            parent: None,
            p,
        }
    }
}

impl<T: Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord, R: RandomGenerator> SSet<T>
    for Treap<T, R>
{
    fn add(&mut self, x: T) -> bool {
        true
    }
    fn find(&self, x: T) -> bool {
        true
    }
    fn remove(&mut self, x: T) -> Option<T> {
        None
    }
    fn size(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod treap_tree_test {
    use std::rc::{Rc, Weak};

    use crate::interfaces::sset::SSet;

    use super::{RandomGenerator, Treap, TreapNode, WrapNode};

    struct RandomGeneratorMock(Vec<usize>);
    impl RandomGenerator for RandomGeneratorMock {
        fn gen_rand(&mut self) -> usize {
            self.0.pop().unwrap()
        }
    }
    #[test]
    fn rotation_test() {
        // before rotation
        let mut rotation_node = WrapNode::from_node(TreapNode {
            parent: None,
            left: None,
            right: None,
            value: 3,
            p: 4,
        });
        let four_99 = WrapNode::from_node(TreapNode {
            parent: None,
            left: Some(WrapNode::from_rc_node(rotation_node.0.clone())),
            right: None,
            value: 4,
            p: 99,
        });
        rotation_node.0.borrow_mut().parent = Some(Rc::downgrade(&four_99));
        let one_9 = WrapNode::from_node(TreapNode {
            parent: None,
            left: None,
            right: None,
            value: 1,
            p: 9,
        });
        let two_6 = WrapNode::from_node(TreapNode {
            parent: None,
            left: Some(WrapNode::from_rc_node(four_99.0.clone())),
            right: Some(WrapNode::from_rc_node(one_9.0.clone())),
            value: 2,
            p: 6,
        });
        one_9.0.borrow_mut().parent = Some(Rc::downgrade(&two_6));
        four_99.0.borrow_mut().parent = Some(Rc::downgrade(&two_6));
        //
        //after rotation_right
        let tobe_right_right = WrapNode::from_node(TreapNode {
            parent: None,
            left: None,
            right: None,
            value: 4,
            p: 99,
        });
        let tobe_right = WrapNode::from_node(TreapNode {
            parent: None,
            left: None,
            right: Some(WrapNode::from_rc_node(tobe_right_right.0.clone())),
            value: 3,
            p: 4,
        });
        tobe_right_right.0.borrow_mut().parent = Some(Rc::downgrade(&tobe_right));
        let tobe_left = WrapNode::from_node(TreapNode {
            parent: None,
            left: None,
            right: None,
            value: 1,
            p: 9,
        });
        let tobe_root = WrapNode::from_node(TreapNode {
            parent: None,
            right: Some(WrapNode::from_rc_node(tobe_right.0.clone())),
            left: Some(WrapNode::from_rc_node(tobe_left.0.clone())),
            value: 2,
            p: 6,
        });
        rotation_node.rotation_right();
        assert_eq!(tobe_root, rotation_node);
    }
    //#[test]
    //fn add_test() {
    //let mut tree = Treap::new(3, RandomGeneratorMock(vec![1, 6]));
    //assert!(tree.add(1));
    //assert!(!tree.add(1));
    //}
}
