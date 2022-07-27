use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
struct BTNode<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> {
    value: T,
    left: Option<Rc<RefCell<BTNode<T>>>>,
    right: Option<Rc<RefCell<BTNode<T>>>>,
    parent: Option<Weak<RefCell<BTNode<T>>>>,
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
    fn add_child(&mut self, child: Rc<RefCell<BTNode<T>>>) -> bool {
        if &self.value > &child.borrow().value {
            self.left = Some(child.clone())
        } else if &self.value < &child.borrow().value {
            self.right = Some(child.clone())
        } else {
            return false;
        }
        child.borrow_mut().parent = Some(Rc::downgrade(&child));
        true
    }
}

#[derive(Debug, Clone)]
pub struct BinaryTree<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> {
    root: Rc<RefCell<BTNode<T>>>,
}

impl<T: Clone + Debug + Eq + PartialEq + PartialOrd + Ord> BinaryTree<T> {
    pub fn new(root: T) -> Self {
        Self {
            root: Rc::new(RefCell::new(BTNode::new(root))),
        }
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
    fn find_last(&self, value: T) -> Option<Rc<RefCell<BTNode<T>>>> {
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
        let insert_prev = self.find_last(value.clone()).unwrap();
        let result = insert_prev
            .borrow_mut()
            .add_child(Rc::new(RefCell::new(BTNode::new(value))));
        result
    }
}

#[cfg(test)]
mod binary_tree_test {
    use super::*;

    #[test]
    fn find_test() {
        let left_child_node = Rc::new(RefCell::new(BTNode {
            value: 1,
            parent: Some(Weak::new()),
            left: None,
            right: None,
        }));
        let right_child_node = Rc::new(RefCell::new(BTNode {
            value: 2,
            parent: Some(Weak::new()),
            left: None,
            right: None,
        }));
        let tree = BinaryTree {
            root: Rc::new(RefCell::new(BTNode {
                value: 0,
                parent: None,
                left: Some(left_child_node.clone()),
                right: Some(right_child_node.clone()),
            })),
        };
        left_child_node.borrow_mut().parent = Some(Rc::downgrade(&tree.root.clone()));
        right_child_node.borrow_mut().parent = Some(Rc::downgrade(&tree.root.clone()));
        println!("{:#?}", tree);
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
        println!("{:#?}", tree);
        let left_right_child_node = Rc::new(RefCell::new(BTNode {
            value: -1,
            parent: Some(Weak::new()),
            left: None,
            right: None,
        }));
        let left_left_child_node = Rc::new(RefCell::new(BTNode {
            value: -3,
            parent: Some(Weak::new()),
            left: None,
            right: None,
        }));
        let left_child_node = Rc::new(RefCell::new(BTNode {
            value: -2,
            parent: Some(Weak::new()),
            left: Some(left_left_child_node.clone()),
            right: Some(left_right_child_node.clone()),
        }));
        left_left_child_node.borrow_mut().parent = Some(Rc::downgrade(&left_child_node));
        left_right_child_node.borrow_mut().parent = Some(Rc::downgrade(&left_child_node));
        let right_left_child_node = Rc::new(RefCell::new(BTNode {
            value: 1,
            parent: Some(Weak::new()),
            left: None,
            right: None,
        }));
        let right_right_child_node = Rc::new(RefCell::new(BTNode {
            value: 3,
            parent: Some(Weak::new()),
            left: None,
            right: None,
        }));
        let right_child_node = Rc::new(RefCell::new(BTNode {
            value: 2,
            parent: Some(Weak::new()),
            left: Some(right_left_child_node.clone()),
            right: Some(right_right_child_node.clone()),
        }));
        right_right_child_node.borrow_mut().parent = Some(Rc::downgrade(&right_child_node));
        right_left_child_node.borrow_mut().parent = Some(Rc::downgrade(&right_child_node));
        let tobe = BinaryTree {
            root: Rc::new(RefCell::new(BTNode {
                value: 0,
                parent: None,
                left: Some(left_child_node),
                right: Some(right_child_node),
            })),
        };
        assert_eq!(tree.root.borrow().value, tobe.root.borrow().value);
        assert_eq!(
            tree.root.borrow().right.as_ref().unwrap().borrow().value,
            tobe.root.borrow().right.as_ref().unwrap().borrow().value
        );
    }
}
