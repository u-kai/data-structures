use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct BinarySearchTree<T: Clone + PartialOrd + Ord + PartialEq + Debug> {
    root: Option<Node<T>>,
}
impl<T: Clone + PartialOrd + Ord + PartialEq + Debug> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
    pub fn add(&mut self, x: T) -> bool {
        if self.root.is_none() {
            self.root = Some(Node::new(x));
            return true;
        }
        self.root.as_mut().map(|node| node.add(x)).unwrap()
    }
    pub fn find(&self, x: T) -> bool {
        self.root.as_ref().map(|node| node.find(x)).unwrap_or(false)
    }
}
#[derive(Debug, PartialEq)]
struct Node<T: Clone + PartialOrd + Ord + PartialEq + Debug> {
    data: T,
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
}
impl<T: Clone + PartialOrd + Ord + PartialEq + Debug> Node<T> {
    fn new(x: T) -> Self {
        Self {
            data: x,
            right: None,
            left: None,
        }
    }
    fn find(&self, x: T) -> bool {
        if self.data > x {
            return self.left.as_ref().map(|node| node.find(x)).unwrap_or(false);
        };
        if self.data < x {
            return self
                .right
                .as_ref()
                .map(|node| node.find(x))
                .unwrap_or(false);
        };
        true
    }
    fn add(&mut self, x: T) -> bool {
        if self.data > x {
            return match self.left.as_mut() {
                Some(left) => left.add(x),
                None => {
                    self.left = Some(Box::new(Node::new(x)));
                    true
                }
            };
        }
        if self.data < x {
            return match self.right.as_mut() {
                Some(right) => right.add(x),
                None => {
                    self.right = Some(Box::new(Node::new(x)));
                    true
                }
            };
        }
        false
    }
}
#[test]
fn test() {
    let mut v = vec![7, 3, 1, 5, 4, 6, 11, 9, 8, 13, 12, 14];
    let mut tree = BinarySearchTree::new();
    let mut root = Node::new(7);
    let mut left = Node::new(3);
    let left_left = Node::new(1);
    let mut left_right = Node::new(5);
    let left_right_left = Node::new(4);
    let left_right_right = Node::new(6);
    let mut right = Node::new(11);
    let mut right_left = Node::new(9);
    let right_left_left = Node::new(8);
    let mut right_right = Node::new(13);
    let right_right_left = Node::new(12);
    let right_right_right = Node::new(14);
    right_left.left = Some(Box::new(right_left_left));
    right_right.left = Some(Box::new(right_right_left));
    right_right.right = Some(Box::new(right_right_right));
    right.left = Some(Box::new(right_left));
    right.right = Some(Box::new(right_right));
    left_right.left = Some(Box::new(left_right_left));
    left_right.right = Some(Box::new(left_right_right));
    left.right = Some(Box::new(left_right));
    left.left = Some(Box::new(left_left));
    root.left = Some(Box::new(left));
    root.right = Some(Box::new(right));
    for d in &v {
        println!("add {}", d);
        assert!(tree.add(*d));
        println!("after {:#?}", tree);
    }
    println!("{:#?}", tree);
    for d in v {
        assert!(tree.find(d))
    }
    let tobe = BinarySearchTree { root: Some(root) };
    assert_eq!(tree, tobe);
}
