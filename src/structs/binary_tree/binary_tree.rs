use std::{
    cell::RefCell,
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
struct BTNode<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> {
    value: T,
    left: Option<WrapNode<T>>,
    right: Option<WrapNode<T>>,
    parent: Option<Weak<RefCell<BTNode<T>>>>,
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> PartialEq for BTNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
#[derive(Debug, Clone)]
struct WrapNode<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord>(Rc<RefCell<BTNode<T>>>);
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> WrapNode<T> {
    fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(BTNode::new(value))))
    }
    fn from_node(node: Rc<RefCell<BTNode<T>>>) -> Self {
        Self(node)
    }
    fn to_node(&self) -> Rc<RefCell<BTNode<T>>> {
        self.0.clone()
    }
    fn add_child(&mut self, child: WrapNode<T>) -> bool {
        if &self.borrow().value > &child.borrow().value {
            let child = child.to_node();
            self.borrow_mut().left = Some(Self::from_node(child));
        } else if &self.borrow().value < &child.borrow().value {
            let child = child.to_node();
            self.borrow_mut().right = Some(Self::from_node(child))
        } else {
            return false;
        }
        child.borrow_mut().parent = Some(Rc::downgrade(&self));
        true
    }
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> Deref for WrapNode<T> {
    type Target = Rc<RefCell<BTNode<T>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> DerefMut for WrapNode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> BTNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
            parent: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryTree<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> {
    root: WrapNode<T>,
}

impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> BinaryTree<T> {
    pub fn new(root: T) -> Self {
        Self {
            root: WrapNode::new(root),
        }
    }
    pub fn depth(&self, value: T) -> Option<usize> {
        let mut last = self.find_last(value.clone());
        let mut depth = 0;
        if last.is_some() {
            if last.as_ref().unwrap().borrow().value == value {
                while last.is_some() {
                    let parent = if last.as_ref().unwrap().borrow().parent.is_some() {
                        last.as_ref()
                            .unwrap()
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap()
                            .upgrade()
                    } else {
                        return Some(depth);
                    };
                    last = Some(WrapNode(parent.unwrap()));
                    depth += 1;
                }
            }
            return Some(depth);
        }
        None
    }
    pub fn size(&self) -> usize {
        let mut node = Some(self.root.to_node());
        let mut prev = None;
        let mut next = None;
        let mut n = 0;
        while node.is_some() {
            let parent = node
                .as_ref()
                .unwrap()
                .borrow()
                .parent
                .as_ref()
                .map(|parent| parent.upgrade().clone().unwrap());
            let left = node
                .as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .map(|node| node.to_node());
            let right = node
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .map(|node| node.to_node());
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
    pub fn find(&self, value: T) -> bool {
        let mut node = Some(self.root.clone());
        while node.is_some() {
            if node.as_ref().unwrap().borrow().value > value {
                let new_node = node.as_ref().unwrap().borrow().left.clone();
                node = new_node;
            } else if node.as_ref().unwrap().borrow().value < value {
                let new_node = node.as_ref().unwrap().borrow().right.clone();
                node = new_node;
            } else if node.as_ref().unwrap().borrow().value == value {
                return true;
            }
        }
        false
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
    pub fn add(&mut self, value: T) -> bool {
        let mut insert_prev = self.find_last(value.clone()).unwrap();
        let result = insert_prev.add_child(WrapNode::new(value));
        result
    }
}

#[cfg(test)]
mod binary_tree_test {
    use super::*;

    #[test]
    fn find_test() {
        let left_child_node = WrapNode::new(1);
        let right_child_node = WrapNode::new(2);
        let tree = BinaryTree {
            root: WrapNode(Rc::new(RefCell::new(BTNode {
                value: 0,
                parent: None,
                left: Some(left_child_node.clone()),
                right: Some(right_child_node.clone()),
            }))),
        };
        left_child_node.borrow_mut().parent = Some(Rc::downgrade(&tree.root.clone()));
        right_child_node.borrow_mut().parent = Some(Rc::downgrade(&tree.root.clone()));
        assert!(tree.find(2));
        assert!(!tree.find(3));
    }
    #[test]
    fn add_test() {
        let mut tree = BinaryTree::new(0);
        assert!(tree.add(-2));
        assert!(tree.add(-3));
        assert!(tree.add(-1));
        assert!(tree.add(2));
        assert!(tree.add(1));
        assert!(tree.add(3));
        assert!(!tree.add(1));
        let left_right_child_node = WrapNode::new(-1);
        let left_left_child_node = WrapNode::new(-3);
        let left_child_node = WrapNode::new(-2);
        left_left_child_node.borrow_mut().parent = Some(Rc::downgrade(&left_child_node));
        left_right_child_node.borrow_mut().parent = Some(Rc::downgrade(&left_child_node));
        let right_left_child_node = WrapNode::new(1);
        let right_right_child_node = WrapNode::new(3);
        let right_child_node = WrapNode::new(2);
        right_right_child_node.borrow_mut().parent = Some(Rc::downgrade(&right_child_node));
        right_left_child_node.borrow_mut().parent = Some(Rc::downgrade(&right_child_node));
        let tobe = BinaryTree {
            root: WrapNode(Rc::new(RefCell::new(BTNode {
                value: 0,
                parent: None,
                left: Some(left_child_node),
                right: Some(right_child_node),
            }))),
        };
        assert_eq!(tree.root.borrow().value, tobe.root.borrow().value);
        assert_eq!(
            tree.root.borrow().right.as_ref().unwrap().borrow().value,
            tobe.root.borrow().right.as_ref().unwrap().borrow().value
        );
    }
    #[test]
    fn depth_test() {
        let mut tree = BinaryTree::new(0);
        tree.add(-2);
        tree.add(-3);
        tree.add(-1);
        tree.add(2);
        tree.add(1);
        tree.add(3);
        println!("{:#?}", tree);
        assert_eq!(tree.depth(0).unwrap(), 0);
        assert_eq!(tree.depth(-2).unwrap(), 1);
        assert_eq!(tree.depth(-3).unwrap(), 2);
        assert_eq!(tree.depth(1).unwrap(), 2);
        assert_eq!(tree.depth(3).unwrap(), 2);
    }
    #[test]
    fn size_test() {
        let mut tree = BinaryTree::new(0);
        tree.add(-2);
        tree.add(-3);
        tree.add(-1);
        tree.add(2);
        tree.add(1);
        tree.add(3);
        tree.add(3);
        tree.add(3);
        assert_eq!(tree.size(), 7);
    }
}
