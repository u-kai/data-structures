use std::{
    cell::RefCell,
    fmt::Debug,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::interfaces::sset::SSet;
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
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord + Default> Drop for TreapNode<T> {
    fn drop(&mut self) {
        println!("node {:?} is droped", self.value)
    }
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord + Default> PartialEq for WrapNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord> WrapNode<T> {
    fn new(value: T, p: usize) -> Self {
        Self(Rc::new(RefCell::new(TreapNode::new(value, p))))
    }
    fn clone(&self) -> Self {
        WrapNode::from_rc_node(self.to_node().clone())
    }
    fn p(&self) -> usize {
        self.borrow().p
    }
    fn add_use_binary_search_algo(&mut self, child: WrapNode<T>) -> bool {
        if &self.borrow().value > &child.borrow().value {
            self.borrow_mut().left = Some(child.clone());
        } else if &self.borrow().value < &child.borrow().value {
            self.borrow_mut().right = Some(child.clone())
        } else {
            return false;
        }
        child.borrow_mut().parent = Some(Rc::downgrade(&self));
        true
    }
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
    fn set_parent(&mut self, parent: Option<WrapNode<T>>) {
        self.0.borrow_mut().parent = parent.map(|parent| Rc::downgrade(&parent));
    }
    fn set_right(&mut self, right: Option<WrapNode<T>>) {
        self.0.borrow_mut().right = right
    }
    fn set_left(&mut self, left: Option<WrapNode<T>>) {
        self.0.borrow_mut().left = left
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

pub trait RandomGenerator {
    fn gen_rand(&mut self) -> usize;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Treap<
    T: Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord,
    R: RandomGenerator + Debug,
> {
    root: WrapNode<T>,
    random_generator: R,
}

impl<
        T: Default + Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord,
        R: RandomGenerator + Debug,
    > Treap<T, R>
{
    fn new(root: T, mut random_generator: R) -> Self {
        let rand = random_generator.gen_rand();
        Self {
            root: WrapNode::new(root, rand),
            random_generator,
        }
    }
    fn find_node(&self, value: T) -> Option<WrapNode<T>> {
        let mut node = Some(self.root.clone());
        while node.is_some() {
            if node.as_ref().unwrap().borrow().value > value {
                let new_node = node.as_ref().unwrap().left();
                node = new_node;
            } else if node.as_ref().unwrap().borrow().value < value {
                let new_node = node.as_ref().unwrap().right();
                node = new_node;
            } else if node.as_ref().unwrap().borrow().value == value {
                let node = node.as_ref().unwrap();
                return Some(node.clone());
            }
        }
        None
    }
    fn find_last(&self, value: T) -> Option<WrapNode<T>> {
        let mut node = Some(self.root.clone());
        let mut prev = None;
        while node.is_some() {
            if node.as_ref().unwrap().borrow().value > value {
                let new_node = node.as_ref().unwrap().borrow().left.clone();
                prev = Some(node.as_ref().unwrap().clone());
                node = new_node;
            } else if node.as_ref().unwrap().borrow().value < value {
                let new_node = node.as_ref().unwrap().borrow().right.clone();
                prev = Some(node.as_ref().unwrap().clone());
                node = new_node
            } else {
                return node;
            }
        }
        prev
    }
    fn rotation_right(&mut self, mut rotation_node: WrapNode<T>) {
        if let Some(mut child) = rotation_node.left() {
            let parent = rotation_node.parent();
            if let Some(mut parent) = parent {
                child.set_parent(Some(parent.clone()));
                if parent.left().is_some() && parent.left().as_ref().unwrap() == &rotation_node {
                    parent.set_left(Some(child.clone()));
                } else {
                    parent.set_right(Some(child.clone()));
                }
            } else {
                child.set_parent(None);
                self.root = child.clone();
            }
            rotation_node.set_left(child.right().map(|right| right.clone()));
            if let Some(mut left) = rotation_node.left() {
                left.set_parent(Some(rotation_node.clone()));
            }
            rotation_node.set_parent(Some(child.clone()));
            child.set_right(Some(rotation_node.clone()));
        }
    }
    fn rotation_left(&mut self, mut rotation_node: WrapNode<T>) {
        let parent = rotation_node.parent();
        if let Some(mut child) = rotation_node.right() {
            child.set_parent(parent.as_ref().map(|parent| parent.clone()));
            if let Some(mut parent) = parent {
                if parent.left().is_some() && parent.left().as_ref().unwrap() == &rotation_node {
                    parent.set_left(Some(child.clone()));
                } else {
                    parent.set_right(Some(child.clone()));
                }
            } else {
                child.set_parent(None);
                self.root = child.clone();
            }

            rotation_node.set_right(child.left().map(|left| left.clone()));
            if let Some(mut right) = rotation_node.right() {
                right.set_parent(Some(rotation_node.clone()));
            }
            rotation_node.set_parent(Some(child.clone()));
            child.set_left(Some(rotation_node.clone()));
        }
    }
}

impl<
        T: Clone + Default + Debug + Eq + PartialEq + PartialOrd + Ord,
        R: RandomGenerator + Debug,
    > SSet<T> for Treap<T, R>
{
    fn add(&mut self, x: T) -> bool {
        let new_node = WrapNode::new(x.clone(), self.random_generator.gen_rand());
        let mut insert_prev = self.find_last(x).unwrap();
        let result = insert_prev.add_use_binary_search_algo(new_node.clone());
        if !result {
            return result;
        }
        while new_node.parent().is_some() && new_node.parent().as_ref().unwrap().p() > new_node.p()
        {
            let new_node_parent = new_node.parent().as_ref().unwrap().clone();
            if new_node.parent().as_ref().unwrap().right() == Some(new_node.clone()) {
                self.rotation_left(new_node_parent);
            } else {
                self.rotation_right(new_node_parent);
            }
        }
        if new_node.parent().is_none() {
            self.root = new_node.clone()
        }
        result
    }
    fn find(&self, x: T) -> bool {
        self.find_node(x).is_some()
    }
    fn remove(&mut self, x: T) -> Option<T> {
        let remove_node = self.find_node(x.clone());
        match remove_node {
            Some(mut remove_node) => {
                while remove_node.right().is_some() || remove_node.left().is_some() {
                    if remove_node.left().is_none() {
                        self.rotation_left(remove_node.clone())
                    } else if remove_node.right().is_none() {
                        self.rotation_right(remove_node.clone())
                    } else if remove_node.left().as_ref().unwrap().p()
                        < remove_node.right().as_ref().unwrap().p()
                    {
                        self.rotation_right(remove_node.clone())
                    } else {
                        self.rotation_left(remove_node.clone())
                    }
                }
                if let Some(mut remove_node_parent) = remove_node.parent() {
                    let parent_left = remove_node_parent.left();
                    let parent_right = remove_node_parent.right();
                    if parent_left.is_some() && parent_left == Some(remove_node.clone()) {
                        remove_node_parent.set_left(None);
                        remove_node.set_parent(None);
                        return Some(x);
                    }
                    if parent_right.is_some() && parent_right == Some(remove_node.clone()) {
                        remove_node_parent.set_right(None);
                        remove_node.set_parent(None);
                        return Some(x);
                    }
                };
                None
            }
            None => None,
        }
    }
    fn size(&self) -> usize {
        let mut node = Some(self.root.clone());
        let mut prev = None;
        let mut next = None;
        let mut n = 0;
        while node.is_some() {
            let parent = node.as_ref().unwrap().parent();
            let left = node.as_ref().unwrap().left();
            let right = node.as_ref().unwrap().right();
            if parent == prev {
                n += 1;
                if node.as_ref().unwrap().borrow().left.is_some() {
                    next = Some(left.unwrap());
                } else if node.as_ref().unwrap().borrow_mut().right.is_some() {
                    next = Some(right.unwrap())
                } else {
                    next = parent;
                }
            } else if prev == left {
                if right.is_some() {
                    next = right;
                } else {
                    next = parent;
                }
            } else {
                next = parent
            }
            prev = node;
            node = next;
        }
        n
    }
}

#[cfg(test)]
mod treap_tree_test {
    use std::rc::Rc;

    use crate::interfaces::sset::SSet;

    use super::{RandomGenerator, Treap, TreapNode, WrapNode};
    #[test]
    fn remove_test() {
        let mut tree = make_test_tree();
        tree.add(10);
        tree.random_generator.0.push(100);
        tree.add(7);
        tree.remove(10);
        tree.remove(7);
        assert_eq!(tree, make_test_tree())
    }
    #[test]
    fn size_test() {
        let tree = make_test_tree();
        assert_eq!(tree.size(), 4);
    }
    #[test]
    fn add_test() {
        let rand = RandomGeneratorMock(vec![9, 99, 6, 4]);
        let mut tree = Treap::new(3, rand);
        assert!(tree.add(2));
        assert!(tree.add(4));
        assert!(tree.add(1));
        assert_eq!(make_test_tree(), tree);
        let rand = RandomGeneratorMock(vec![4, 99, 9, 6]);
        let mut tree = Treap::new(2, rand);
        assert!(tree.add(1));
        assert!(tree.add(4));
        assert!(tree.add(3));
        assert_eq!(make_test_tree(), tree);
        let rand = RandomGeneratorMock(vec![99, 9, 6, 4]);
        let mut tree = Treap::new(3, rand);
        assert!(tree.add(2));
        assert!(tree.add(1));
        assert!(tree.add(4));
        assert_eq!(make_test_tree(), tree);
        let rand = RandomGeneratorMock(vec![9, 6, 4, 99]);
        let mut tree = Treap::new(4, rand);
        assert!(tree.add(3));
        assert!(tree.add(2));
        assert!(tree.add(1));
        assert!(!tree.add(1));
        assert_eq!(make_test_tree(), tree);
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RandomGeneratorMock(Vec<usize>);
    impl RandomGenerator for RandomGeneratorMock {
        fn gen_rand(&mut self) -> usize {
            self.0.pop().unwrap_or_default()
        }
    }
    fn make_test_tree() -> Treap<i32, RandomGeneratorMock> {
        Treap {
            root: make_test_node(),
            random_generator: RandomGeneratorMock(Vec::new()),
        }
    }
    fn make_test_node() -> WrapNode<i32> {
        let tobe_right = WrapNode::from_node(TreapNode {
            parent: None,
            left: None,
            right: None,
            value: 4,
            p: 99,
        });
        let tobe_left_left = WrapNode::from_node(TreapNode {
            parent: None,
            left: None,
            right: None,
            value: 1,
            p: 9,
        });
        let tobe_left = WrapNode::from_node(TreapNode {
            parent: None,
            right: None,
            left: Some(WrapNode::from_rc_node(tobe_left_left.0.clone())),
            value: 2,
            p: 6,
        });
        let tobe = WrapNode::from_node(TreapNode {
            parent: None,
            left: Some(WrapNode::from_rc_node(tobe_left.0.clone())),
            right: Some(WrapNode::from_rc_node(tobe_right.0.clone())),
            value: 3,
            p: 4,
        });

        tobe_left.0.borrow_mut().parent = Some(Rc::downgrade(&tobe));
        tobe_right.0.borrow_mut().parent = Some(Rc::downgrade(&tobe));
        tobe_left_left.0.borrow_mut().parent = Some(Rc::downgrade(&tobe_left));
        tobe
    }
}
