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
    fn change_value(&mut self, value: T) {
        self.value = value
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
    pub fn add(&mut self, value: T) -> bool {
        self.root
            .borrow_mut()
            .left
            .as_mut()
            .map(|node| node.borrow_mut().change_value(value));
        true
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
        assert!(tree.add(1));
        let child_node = BTNode {
            value: 1,
            parent: Some(Weak::new()),
            left: None,
            right: None,
        };
        let tobe = BinaryTree {
            root: Rc::new(RefCell::new(BTNode {
                value: 0,
                parent: None,
                left: Some(Rc::new(RefCell::new(child_node))),
                right: None,
            })),
        };
        //assert_eq!(tree.root.value, tobe.root.value);
        //assert_eq!(
        //tree.root.left.unwrap().borrow().value,
        //tobe.root.left.unwrap().borrow().value
        //);
    }
}
