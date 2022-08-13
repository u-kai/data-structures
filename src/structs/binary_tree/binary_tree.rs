use std::{
    cell::RefCell,
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
pub(super) struct BTNode<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> {
    value: T,
    left: Option<WrapNode<T>>,
    right: Option<WrapNode<T>>,
    parent: Option<Weak<RefCell<BTNode<T>>>>,
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> Drop for BTNode<T> {
    fn drop(&mut self) {
        println!("node {:?} is droped", self.value)
    }
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> PartialEq for BTNode<T> {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(parent), Some(other_parent)) = (self.parent.as_ref(), other.parent.as_ref()) {
            let parent_value = &parent.upgrade();
            let parent_value = &parent_value.as_ref().unwrap().borrow().value;
            let other_parent_value = &other_parent.upgrade();
            let other_parent_value = &other_parent_value.as_ref().unwrap().borrow().value;
            return parent_value == other_parent_value
                && self.value == other.value
                && self.left == other.left
                && self.right == other.right;
        }
        self.value == other.value && self.left == other.left && self.right == other.right
    }
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> Eq for BTNode<T> {
    fn assert_receiver_is_total_eq(&self) {}
}

#[derive(Debug, Clone, Eq)]
pub(super) struct WrapNode<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord>(
    Rc<RefCell<BTNode<T>>>,
);
pub(super) trait Tree<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> {
    fn change_root(&mut self, node: WrapNode<T>) -> ();
}
impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> WrapNode<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(BTNode::new(value))))
    }
    pub fn rotation_right(tree: &mut impl Tree<T>, mut rotation_node: WrapNode<T>) {
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
                tree.change_root(child.clone());
            }
            rotation_node.set_left(child.right().map(|right| right.clone()));
            if let Some(mut left) = rotation_node.left() {
                left.set_parent(Some(rotation_node.clone()));
            }
            rotation_node.set_parent(Some(child.clone()));
            child.set_right(Some(rotation_node.clone()));
        }
    }
    pub fn rotation_left(tree: &mut impl Tree<T>, mut rotation_node: WrapNode<T>) {
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
                tree.change_root(child.clone());
            }

            rotation_node.set_right(child.left().map(|left| left.clone()));
            if let Some(mut right) = rotation_node.right() {
                right.set_parent(Some(rotation_node.clone()));
            }
            rotation_node.set_parent(Some(child.clone()));
            child.set_left(Some(rotation_node.clone()));
        }
    }
    pub fn change_value(&self, new_value: T) {
        self.borrow_mut().value = new_value
    }
    pub fn value(&self) -> T {
        self.borrow().value.clone()
    }
    pub fn clone(&self) -> Self {
        WrapNode::from_node(self.to_node().clone())
    }
    pub fn has_child(&self) -> bool {
        self.borrow().left.is_some() || self.borrow().right.is_some()
    }
    pub fn set_parent(&mut self, parent: Option<WrapNode<T>>) {
        self.0.borrow_mut().parent = parent.map(|parent| Rc::downgrade(&parent));
    }
    pub fn set_right(&mut self, right: Option<WrapNode<T>>) {
        self.0.borrow_mut().right = right
    }
    pub fn set_left(&mut self, left: Option<WrapNode<T>>) {
        self.0.borrow_mut().left = left
    }
    pub fn parent(&self) -> Option<Self> {
        if let Some(parent) = &self.borrow().parent {
            let parent = parent.upgrade().as_ref().unwrap().clone();
            Some(WrapNode(parent))
        } else {
            None
        }
    }
    pub fn left(&self) -> Option<Self> {
        if let Some(left) = self.borrow().left.as_ref() {
            let left = left.to_node().clone();
            Some(WrapNode(left))
        } else {
            None
        }
    }
    pub fn right(&self) -> Option<Self> {
        if let Some(right) = self.borrow().right.as_ref() {
            let right = right.to_node().clone();
            Some(WrapNode(right))
        } else {
            None
        }
    }
    pub fn from_node(node: Rc<RefCell<BTNode<T>>>) -> Self {
        Self(node)
    }
    pub fn to_node(&self) -> Rc<RefCell<BTNode<T>>> {
        self.0.clone()
    }
    pub fn add_child(&mut self, child: WrapNode<T>) -> bool {
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
    pub fn size(&self) -> usize {
        let left_len = if let Some(left) = self.left() {
            left.size()
        } else {
            0
        };
        let right_len = if let Some(right) = self.right() {
            right.size()
        } else {
            0
        };
        1 + left_len + right_len
    }
    fn find_node(&self, value: T) -> Option<WrapNode<T>> {
        let mut node = Some(WrapNode::from_node(self.to_node()));
        while node.is_some() {
            if node.as_ref().unwrap().borrow().value > value {
                let new_node = node.as_ref().unwrap().left();
                node = new_node;
            } else if node.as_ref().unwrap().borrow().value < value {
                let new_node = node.as_ref().unwrap().right();
                node = new_node;
            } else if node.as_ref().unwrap().borrow().value == value {
                let node = node.as_ref().unwrap().to_node();
                return Some(WrapNode::from_node(node.clone()));
            }
        }
        None
    }
    pub fn find(&self, value: T) -> bool {
        self.find_node(value).is_some()
    }
    pub fn find_parent(&self, value: T) -> Option<WrapNode<T>> {
        let last = self.find_last(value.clone());
        if let Some(last) = last {
            if last.value() == value.clone() {
                return last.parent();
            }
        }
        None
    }
    pub fn find_last(&self, value: T) -> Option<WrapNode<T>> {
        let mut node = Some(WrapNode::from_node(self.to_node()));
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
    pub fn add(&mut self, value: T) -> bool {
        let mut insert_prev = self.find_last(value.clone()).unwrap();
        let result = insert_prev.add_child(WrapNode::new(value));
        result
    }
    pub fn remove(&mut self, value: T) -> Option<T> {
        let remove_node = self.find_node(value.clone());
        let result = Some(value.clone());
        if remove_node.is_none() {
            return None;
        }
        let remove_node = remove_node.as_ref().unwrap();
        //case remove_node is leaf
        if !remove_node.has_child() {
            let parent = remove_node.parent();
            if let Some(parent) = parent {
                if let Some(left) = parent.left() {
                    if left.borrow().value == value {
                        parent.borrow_mut().left = None;
                        return result;
                    }
                }
                if let Some(right) = parent.right() {
                    if right.borrow().value == value {
                        parent.borrow_mut().right = None;
                        return result;
                    }
                }
            }
        }
        // case remove_node has left child
        if let (Some(new_child), None) = (remove_node.left(), remove_node.right()) {
            let parent = remove_node.parent();
            if let Some(parent) = parent {
                if let Some(parent_left) = parent.left() {
                    if parent_left.borrow().value == value {
                        parent.borrow_mut().left = Some(new_child);
                        return result;
                    }
                }
                if let Some(parent_right) = parent.right() {
                    if parent_right.borrow().value == value {
                        parent.borrow_mut().right = Some(new_child);
                        return result;
                    }
                }
            }
        }
        // case remove_node has right child
        if let (None, Some(new_child)) = (remove_node.left(), remove_node.right()) {
            let parent = remove_node.parent();
            if let Some(parent) = parent {
                if let Some(parent_left) = parent.left() {
                    if parent_left.borrow().value == value {
                        parent.borrow_mut().left = Some(new_child);
                        return result;
                    }
                }
                if let Some(parent_right) = parent.right() {
                    if parent_right.borrow().value == value {
                        parent.borrow_mut().right = Some(new_child);
                        return result;
                    }
                }
            }
        }
        //case remove_node has two child
        if let (Some(remove_node_left), Some(remove_node_right)) =
            (remove_node.left(), remove_node.right())
        {
            let mut new_child = Some(remove_node_right.clone());
            while new_child.is_some() {
                if let Some(left) = new_child.as_ref().unwrap().left() {
                    new_child = Some(left);
                } else {
                    break;
                }
            }
            if let Some(parent) = remove_node.parent() {
                if let Some(parent_left) = parent.left() {
                    if parent_left.borrow().value == value {
                        let new_child_parent = new_child.as_ref().unwrap().parent().unwrap();
                        new_child_parent.borrow_mut().left = None;
                        parent_left.borrow_mut().left = None;
                        parent_left.borrow_mut().right = None;
                        parent.borrow_mut().right = Some(WrapNode::from_node(
                            new_child.as_ref().unwrap().to_node().clone(),
                        ));
                        new_child.as_ref().unwrap().borrow_mut().parent =
                            Some(Rc::downgrade(&parent));
                        remove_node_left.as_ref().borrow_mut().parent =
                            new_child.as_ref().map(|new| Rc::downgrade(new));
                        remove_node_right.as_ref().borrow_mut().parent =
                            new_child.as_ref().map(|new| Rc::downgrade(new));
                        new_child.as_ref().unwrap().borrow_mut().left = Some(remove_node_left);
                        new_child.as_ref().unwrap().borrow_mut().right = Some(remove_node_right);
                        return result;
                    }
                }
                if let Some(parent_right) = parent.right() {
                    if parent_right.borrow().value == value {
                        let new_child_parent = new_child.as_ref().unwrap().parent().unwrap();
                        new_child_parent.borrow_mut().left = None;
                        parent_right.borrow_mut().left = None;
                        parent_right.borrow_mut().right = None;
                        parent.borrow_mut().right = Some(WrapNode::from_node(
                            new_child.as_ref().unwrap().to_node().clone(),
                        ));
                        new_child.as_ref().unwrap().borrow_mut().parent =
                            Some(Rc::downgrade(&parent));
                        remove_node_left.as_ref().borrow_mut().parent =
                            new_child.as_ref().map(|new| Rc::downgrade(new));
                        remove_node_right.as_ref().borrow_mut().parent =
                            new_child.as_ref().map(|new| Rc::downgrade(new));
                        new_child.as_ref().unwrap().borrow_mut().left = Some(remove_node_left);
                        new_child.as_ref().unwrap().borrow_mut().right = Some(remove_node_right);
                        return result;
                    }
                }
            }
        }
        None
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
        self.0 == other.0
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryTree<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> {
    root: Option<WrapNode<T>>,
}

impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> BinaryTree<T> {
    pub fn new(root: T) -> Self {
        Self {
            root: Some(WrapNode::new(root)),
        }
    }
    pub fn remove(&mut self, value: T) -> Option<T> {
        if self.root.is_some() {
            if &self.root.as_ref().unwrap().borrow().value == &value {
                self.root = None;
                return Some(value);
            }
            return self.root.as_mut().unwrap().remove(value);
        }
        None
    }
    pub fn depth(&self, value: T) -> Option<usize> {
        if let Some(d) = self.root.as_ref().map(|root| root.depth(value)) {
            d
        } else {
            None
        }
    }
    pub fn size(&self) -> usize {
        let mut node = Some(self.root.as_ref().unwrap().clone());
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
        if let Some(option_node) = self.root.as_ref().map(|root| root.find_node(value)) {
            option_node
        } else {
            None
        }
    }
    pub fn find(&self, value: T) -> bool {
        self.find_node(value).is_some()
    }
    fn find_last(&self, value: T) -> Option<WrapNode<T>> {
        if let Some(option_node) = self.root.as_ref().map(|root| root.find_last(value)) {
            option_node
        } else {
            None
        }
    }
    pub fn add(&mut self, value: T) -> bool {
        if self.root.is_none() {
            self.root = Some(WrapNode::new(value));
            return true;
        }
        let mut insert_prev = self.find_last(value.clone()).unwrap();
        let result = insert_prev.add_child(WrapNode::new(value));
        result
    }
}

#[cfg(test)]
mod binary_tree_test {

    use super::*;
    #[test]
    fn node_eq_test() {
        let mut node = BinaryTree::new(1);
        node.add(2);
        node.add(3);
        node.add(0);

        let two = WrapNode::new(2);
        let three = WrapNode::new(3);
        two.0.borrow_mut().right = Some(WrapNode::from_node(three.0.clone()));
        three.0.borrow_mut().parent = Some(Rc::downgrade(&two));
        let zero = WrapNode::new(0);

        let tobe = WrapNode::from_node(Rc::new(RefCell::new(BTNode {
            value: 1,
            left: Some(WrapNode::from_node(zero.0.clone())),
            right: Some(WrapNode::from_node(two.0.clone())),
            parent: None,
        })));
        two.0.borrow_mut().parent = Some(Rc::downgrade(&tobe));
        zero.0.borrow_mut().parent = Some(Rc::downgrade(&tobe));
        let tobe = BinaryTree { root: Some(tobe) };
        assert_eq!(node, tobe);
        node.add(4);
        assert_ne!(node, tobe)
    }

    #[test]
    fn find_test() {
        let left_child_node = WrapNode::new(1);
        let right_child_node = WrapNode::new(2);
        let tree = BinaryTree {
            root: Some(WrapNode(Rc::new(RefCell::new(BTNode {
                value: 0,
                parent: None,
                left: Some(left_child_node.clone()),
                right: Some(right_child_node.clone()),
            })))),
        };
        left_child_node.borrow_mut().parent =
            Some(Rc::downgrade(&tree.root.as_ref().unwrap().clone()));
        right_child_node.borrow_mut().parent =
            Some(Rc::downgrade(&tree.root.as_ref().unwrap().clone()));
        assert!(tree.find(2));
        assert!(!tree.find(3));
    }
    #[test]
    fn add_test() {
        let mut tree = BinaryTree { root: None };
        assert!(tree.add(0));
        assert_eq!(
            tree,
            BinaryTree {
                root: Some(WrapNode::new(0))
            }
        );
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
            root: Some(WrapNode(Rc::new(RefCell::new(BTNode {
                value: 0,
                parent: None,
                left: Some(left_child_node),
                right: Some(right_child_node),
            })))),
        };
        assert_eq!(
            tree.root.as_ref().unwrap().borrow().value,
            tobe.root.as_ref().unwrap().borrow().value
        );
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            tobe.root
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .value
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
        let mut tree = BinaryTree::new(7);
        assert_eq!(tree.remove(7), Some(7));
        let mut tree = BinaryTree::new(7);
        tree.add(3);
        tree.add(1);
        tree.add(5);
        tree.add(4);
        tree.add(6);
        tree.add(11);
        tree.add(9);
        tree.add(8);
        tree.add(13);
        tree.add(12);
        tree.add(14);
        assert_eq!(tree.remove(11), Some(11));
        let mut tobe = BinaryTree::new(7);
        tobe.add(3);
        tobe.add(1);
        tobe.add(5);
        tobe.add(4);
        tobe.add(6);
        tobe.add(12);
        tobe.add(9);
        tobe.add(8);
        tobe.add(13);
        tobe.add(14);
        assert_eq!(tree.remove(1), Some(1));
        assert_eq!(tree.remove(9), Some(9));
    }
}
