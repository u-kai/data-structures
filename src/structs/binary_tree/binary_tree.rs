use std::{
    cell::RefCell,
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
    string::ParseError,
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
    fn has_child(&self) -> bool {
        self.borrow().left.is_some() || self.borrow().right.is_some()
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
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> PartialEq for WrapNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().value == other.borrow().value
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
    pub fn remove(&mut self, value: T) -> Option<T> {
        let remove_node = self.find_node(value);
        if remove_node.is_none() {
            return None;
        }
        let remove_node = remove_node.unwrap();
        //case remove_node is leaf
        if !remove_node.has_child() {
            let parent = remove_node
                .borrow()
                .parent
                .as_ref()
                .unwrap()
                .upgrade()
                .as_ref()
                .unwrap();
        }

        None
    }
    fn split(&mut self, node: WrapNode<T>) {
        let left = node.borrow().left.as_ref().map(|left| left.to_node());
        let right = node.borrow().right.as_ref().map(|right| right.to_node());
        let parent = node
            .borrow()
            .parent
            .as_ref()
            .map(|parent| parent.upgrade().unwrap().clone());
        let mut s = None;
        let mut p = None;
        if left.is_none() {
            s = left;
        } else {
            s = right;
        }
        let node_value = node.0.borrow().value.clone();
        let root_value = self.root.0.borrow().value.clone();
        if root_value == node_value {
            *self.root = s.unwrap();
            p = None;
        } else {
            p = parent;
            let p_left = p.unwrap();
            if p_left == node.to_node() {
                //parent.unwrap().borrow_mut().left = s.unwrap();
            }
        }
    }
    pub fn depth(&self, value: T) -> Option<usize> {
        let mut last = self.find_last(value.clone());
        let mut depth = 0;
        if last.is_none() {
            return None;
        };
        if last.as_ref().unwrap().borrow().value == value {
            while last.is_some() {
                if let Some(parent) = last.unwrap().parent() {
                    last = Some(parent);
                    depth += 1;
                } else {
                    return Some(depth);
                };
            }
        }
        return Some(depth);
    }
    pub fn size(&self) -> usize {
        let mut node = Some(WrapNode::from_node(self.root.to_node()));
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
    fn find_node(&self, value: T) -> Option<WrapNode<T>> {
        let mut node = Some(self.root.clone());
        while node.is_some() {
            if node.as_ref().unwrap().borrow().value > value {
                let new_node = node.as_ref().unwrap().borrow().left.clone();
                node = new_node;
            } else if node.as_ref().unwrap().borrow().value < value {
                let new_node = node.as_ref().unwrap().borrow().right.clone();
                node = new_node;
            } else if node.as_ref().unwrap().borrow().value == value {
                let node = node.as_ref().unwrap().to_node();
                return Some(WrapNode::from_node(node.clone()));
            }
        }
        None
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
    #[test]
    fn remove_test() {
        let mut tree = BinaryTree::new(0);
        tree.add(-2);
        tree.add(-3);
        tree.add(-1);
        tree.add(2);
        tree.add(1);
        tree.add(3);
        tree.add(3);
        tree.add(3);
        //assert_eq!(tree.remove(-2), Some(-2));
        //assert_eq!(tree.remove(-3), Some(-3));
        //assert_eq!(tree.remove(0), Some(0));
        assert_eq!(tree.remove(-5), None);
    }
}
