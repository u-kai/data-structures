use std::fmt::Debug;

use super::nodes::strong_link::StrongLinkNode;

pub trait ToUsize {
    fn to_usize(&self) -> usize;
}
impl<T: ToUsize + Clone + PartialEq + Debug> StrongLinkNode<T> {
    fn num(&self) -> Option<usize> {
        self.value().map(|value| value.to_usize())
    }
    fn update_jump(&mut self, binary: Binary) {
        match binary {
            Binary::Zero => {
                let jump = self.left().get_max_child();
                self.set_jump(jump);
            }
            Binary::One => {
                let jump = self.right().get_min_child();
                self.set_jump(jump);
            }
        }
    }
}
#[derive(Debug, PartialEq, Clone)]

pub(super) enum PathNodeOrLeaf<T: Clone + PartialEq> {
    PathNode,
    Leaf(T),
}
impl<T: Clone + PartialEq> PathNodeOrLeaf<T> {
    pub fn new_leaf(x: T) -> Self {
        PathNodeOrLeaf::Leaf(x)
    }
    pub fn new_path() -> Self {
        PathNodeOrLeaf::PathNode
    }
    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Leaf(x) => Some(&x),
            Self::PathNode => None,
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub(super) enum Binary {
    Zero,
    One,
}
impl Binary {
    pub fn calc_binary(number: usize, digit: usize) -> Self {
        if (number >> (digit - 1) & 1) == 1 {
            Self::One
        } else {
            Self::Zero
        }
    }
    pub fn other(&self) -> Self {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Zero,
        }
    }
    pub fn to_num(&self) -> usize {
        match self {
            Self::Zero => 0,
            Self::One => 1,
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct BinaryTrie<T: ToUsize + Clone + PartialEq + Debug> {
    root: StrongLinkNode<T>,
    min_prev: StrongLinkNode<T>,
    max_next: StrongLinkNode<T>,
    w: usize,
}
impl<T: ToUsize + Clone + PartialEq + Debug> BinaryTrie<T> {
    pub fn new(w: usize) -> Self {
        let root = StrongLinkNode::new_path_node();
        let mut min_prev = StrongLinkNode::new_path_node();
        let max_next = StrongLinkNode::new_path_node();
        min_prev.set_next(max_next.clone());
        Self {
            root,
            min_prev,
            max_next,
            w,
        }
    }
    pub fn add(&mut self, x: T) -> bool {
        if !self.in_range(&x) {
            panic!("x is too big! please use small x or large w at new method")
        }

        let num_x = x.to_usize();
        let leaf = StrongLinkNode::new_leaf(x);
        let mut node = self.root.clone();
        let mut prev = self.find_prev(num_x);

        for digit in (1..=self.w).rev() {
            let binary = Binary::calc_binary(num_x, digit);
            let child = node.child(binary.to_num());
            if child.is_some() {
                if digit == 1 {
                    //x can not add because x is exist
                    return false;
                }
                if node.jump().is_some() {
                    if binary == Binary::Zero && node.jump().num() < Some(num_x)
                        || binary == Binary::One && node.jump().num() > Some(num_x)
                    {
                        node.set_jump(leaf.clone());
                    }
                }
                node = child;
            } else {
                if digit != 1 {
                    let mut new_path_node = StrongLinkNode::new_path_node();
                    new_path_node.set_jump(leaf.clone());
                    node.set_child(new_path_node.clone(), binary.to_num());
                    if node.child(binary.other().to_num()).is_none() {
                        node.set_jump(leaf.clone())
                    } else if node.jump().is_some() {
                        node.remove_jump()
                    }
                    node = new_path_node;
                } else {
                    node.set_child(leaf.clone(), binary.to_num());
                    if node.child(binary.other().to_num()).is_none() {
                        node.set_jump(leaf.clone());
                    } else if node.jump().is_some() {
                        node.remove_jump()
                    }
                    let mut next = prev.next();
                    prev.set_next(leaf.clone());
                    next.set_prev(leaf.clone());
                }
            }
        }
        true
    }
    pub fn remove(&mut self, x: T) -> Option<T> {
        let num_x = x.to_usize();
        let remove_leaf = self.find_leaf(num_x);
        if remove_leaf.is_none() {
            return None;
        }
        let mut prev = remove_leaf.prev();
        let mut next = remove_leaf.next();
        prev.set_next(next.clone());
        next.set_prev(prev.clone());
        let mut parent = remove_leaf.parent();
        for i in 1..=self.w {
            let binary = Binary::calc_binary(num_x, i);
            let child = parent.child(binary.to_num());
            if !child.has_child() {
                parent.set_child(StrongLinkNode::new_none(), binary.to_num());
            }
            if parent.jump() == remove_leaf {
                parent.update_jump(binary);
            }
            if parent.child(binary.to_num()).is_none() && parent.jump().is_none() {
                parent.update_jump(binary.other());
            }
            parent = parent.parent();
        }
        remove_leaf.value()
    }
    pub fn find(&self, x: T) -> bool {
        self.find_leaf(x.to_usize()).is_some()
    }
    fn find_leaf(&self, num: usize) -> StrongLinkNode<T> {
        let prev = self.find_prev(num);
        if prev.next().num() == Some(num) {
            prev.next()
        } else {
            StrongLinkNode::new_none()
        }
    }
    fn find_prev(&self, num: usize) -> StrongLinkNode<T> {
        let mut node = self.root.clone();
        for i in (1..=self.w).rev() {
            let binary = Binary::calc_binary(num, i);
            let child = node.child(binary.to_num());
            if child.is_some() {
                node = child
            } else {
                node = node.jump();
                if node.is_none() {
                    return self.min_prev.clone();
                }
                if node.num() >= Some(num) {
                    node = node.prev()
                }
                return node;
            }
        }
        if node.num() == Some(num) {
            node.prev()
        } else {
            self.min_prev.clone()
        }
    }
    fn in_range(&self, x: &T) -> bool {
        let num_x = x.to_usize();
        num_x < 2_i128.pow(self.w as u32) as usize
    }
}

#[cfg(test)]

mod binary_trie_test {
    use super::*;
    #[test]
    fn remove_test() {
        let mut tree = BinaryTrie::new(4);
        tree.add(3);
        tree.add(9);
        tree.add(1);
        tree.add(0);
        tree.add(15);
        let mut tobe = BinaryTrie::new(4);
        tobe.add(9);
        tobe.add(1);
        tobe.add(0);
        tobe.add(15);
        assert_eq!(tree.remove(3), Some(3));
        assert_eq!(tree, tobe);
        let mut tobe = BinaryTrie::new(4);
        tobe.add(1);
        tobe.add(0);
        tobe.add(15);
        assert_eq!(tree.remove(9), Some(9));
        assert_eq!(tree, tobe);
        assert_eq!(tree.remove(1), Some(1));
        assert_eq!(tree.remove(0), Some(0));
        assert_eq!(tree.remove(15), Some(15));
        assert_eq!(tree.remove(3), None);
        assert_eq!(tree.remove(9), None);
        assert_eq!(tree.remove(1), None);
        assert_eq!(tree.remove(0), None);
        assert_eq!(tree.remove(15), None);
    }
    #[test]
    fn find_prev_test() {
        let mut tree = BinaryTrie::new(4);
        tree.add(3);
        tree.add(9);
        tree.add(1);
        tree.add(0);
        tree.add(15);
        assert_eq!(tree.find_prev(0).num(), None);
        assert_eq!(tree.find_prev(8).num(), Some(3));
        assert_eq!(tree.find_prev(15).num(), Some(9));
        let tree = BinaryTrie::<i32>::new(4);
        assert_eq!(tree.find_prev(0).num(), None);
    }
    #[test]
    fn find_test() {
        let mut tree = BinaryTrie::<i32>::new(4);
        tree.add(0);
        tree.add(1);
        tree.add(3);
        tree.add(9);
        tree.add(15);
        assert!(tree.find(0));
        assert!(tree.find(1));
        assert!(!tree.find(2));
        assert!(tree.find(3));
        assert!(!tree.find(4));
        assert!(!tree.find(5));
        assert!(!tree.find(6));
        assert!(!tree.find(7));
        assert!(!tree.find(8));
        assert!(tree.find(9));
        assert!(!tree.find(10));
        assert!(!tree.find(11));
        assert!(!tree.find(12));
        assert!(!tree.find(13));
        assert!(!tree.find(14));
        assert!(tree.find(15));
    }
    #[test]
    fn add_test() {
        let mut root = StrongLinkNode::new_path_node();
        let mut leaf_3 = StrongLinkNode::new_leaf(3);
        let mut min_prev = StrongLinkNode::new_path_node();
        let mut max_next = StrongLinkNode::new_path_node();
        min_prev.set_next(leaf_3.clone());
        max_next.set_prev(leaf_3.clone());

        let mut root_left_child = StrongLinkNode::new_path_node();
        let mut root_left_child_left_child = StrongLinkNode::new_path_node();
        let mut root_left_child_left_child_right_child = StrongLinkNode::new_path_node();

        root.set_jump(leaf_3.clone());
        root_left_child.set_jump(leaf_3.clone());
        root_left_child_left_child.set_jump(leaf_3.clone());
        root_left_child_left_child_right_child.set_jump(leaf_3.clone());
        root_left_child_left_child_right_child.set_right(leaf_3.clone());
        root_left_child_left_child.set_right(root_left_child_left_child_right_child.clone());
        root_left_child.set_left(root_left_child_left_child.clone());
        root.set_left(root_left_child.clone());

        let tobe: BinaryTrie<i32> = BinaryTrie {
            root: root.clone(),
            min_prev,
            max_next,
            w: 4,
        };

        let mut tree = BinaryTrie::new(4);
        tree.add(3);

        assert_eq!(tree, tobe);

        let mut root_right_child = StrongLinkNode::new_path_node();
        let mut root_right_child_left_child = StrongLinkNode::new_path_node();
        let mut root_right_child_left_child_left_child = StrongLinkNode::new_path_node();
        let leaf_9 = StrongLinkNode::new_leaf(9);
        root.set_jump(StrongLinkNode::new_path_node());
        root_right_child.set_jump(leaf_9.clone());
        root_right_child_left_child.set_jump(leaf_9.clone());
        root_right_child_left_child_left_child.set_jump(leaf_9.clone());
        root_right_child_left_child_left_child.set_right(leaf_9.clone());
        root_right_child_left_child.set_left(root_right_child_left_child_left_child.clone());
        root_right_child.set_left(root_right_child_left_child.clone());
        root.set_right(root_right_child.clone());
        leaf_3.set_next(leaf_9.clone());
        let mut min_prev = StrongLinkNode::new_path_node();
        let mut max_next = StrongLinkNode::new_path_node();
        min_prev.set_next(leaf_3.clone());
        max_next.set_prev(leaf_9.clone());
        tree.add(9);
        let tobe = BinaryTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
        };
        rec_assert("root".to_string(), tree.root.clone(), tobe.root.clone());
        assert_eq!(tree, tobe);
        let mut leaf_1 = StrongLinkNode::new_leaf(1);
        let mut root_left_child_left_child_left_child = StrongLinkNode::new_path_node();
        root_left_child_left_child_left_child.set_right(leaf_1.clone());
        root_left_child_left_child_left_child.set_jump(leaf_1.clone());
        min_prev.set_next(leaf_1.clone());
        leaf_1.set_next(leaf_3.clone());
        root_left_child_left_child.set_jump(StrongLinkNode::new_none());
        root_left_child_left_child.set_left(root_left_child_left_child_left_child.clone());
        tree.add(1);
        let tobe = BinaryTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
        };
        rec_assert("root".to_string(), tree.root.clone(), tobe.root.clone());
        let mut leaf_0 = StrongLinkNode::new_leaf(0);
        root_left_child_left_child_left_child.set_left(leaf_0.clone());
        root_left_child_left_child_left_child.set_jump(StrongLinkNode::new_none());
        min_prev.set_next(leaf_0.clone());
        leaf_0.set_next(leaf_1.clone());
        let tobe = BinaryTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
        };
        tree.add(0);
        assert_eq!(tree, tobe);
        let mut leaf_15 = StrongLinkNode::new_leaf(15);
        let mut root_right_child_right_child = StrongLinkNode::new_path_node();
        let mut root_right_child_right_child_right_child = StrongLinkNode::new_path_node();
        root_right_child_right_child_right_child.set_jump(leaf_15.clone());
        root_right_child_right_child_right_child.set_right(leaf_15.clone());
        root_right_child_right_child.set_jump(leaf_15.clone());
        root_right_child_right_child.set_right(root_right_child_right_child_right_child.clone());
        root_right_child.set_right(root_right_child_right_child.clone());
        root_right_child.set_jump(StrongLinkNode::new_none());
        max_next.set_prev(leaf_15.clone());
        leaf_15.set_prev(leaf_9.clone());
        let tobe = BinaryTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
        };
        tree.add(15);
        assert_eq!(tree, tobe);
        let mut tree_2 = BinaryTrie::new(4);
        tree_2.add(0);
        tree_2.add(1);
        tree_2.add(3);
        tree_2.add(9);
        tree_2.add(15);

        assert_eq!(tree, tree_2);
        rec_assert("root".to_string(), tree.root.clone(), tobe.root.clone());
    }

    #[test]
    fn calc_binary_test() {
        assert_eq!(Binary::calc_binary(255, 1), Binary::One);
        assert_eq!(Binary::calc_binary(255, 2), Binary::One);
        assert_eq!(Binary::calc_binary(255, 3), Binary::One);
        assert_eq!(Binary::calc_binary(255, 4), Binary::One);
        assert_eq!(Binary::calc_binary(255, 5), Binary::One);
        assert_eq!(Binary::calc_binary(255, 6), Binary::One);
        assert_eq!(Binary::calc_binary(255, 7), Binary::One);
        assert_eq!(Binary::calc_binary(255, 8), Binary::One);
        assert_eq!(Binary::calc_binary(255, 9), Binary::Zero);
        assert_eq!(Binary::calc_binary(0, 1), Binary::Zero);
        assert_eq!(Binary::calc_binary(8, 5), Binary::Zero);
    }
}

#[allow(unused)]
fn check_prev_next<T: ToUsize + Clone + PartialEq + Debug>(tree: BinaryTrie<T>) {
    let mut next = tree.min_prev.clone();
    println!("from prev");
    println!();
    while next.is_some() {
        println!("next = {:?}", next.value());
        next = next.next();
    }
    let mut prev = tree.max_next.clone();
    println!("from next");
    println!();
    while prev.is_some() {
        println!("prev = {:?}", prev.value());
        prev = prev.prev();
    }
}
#[allow(unused)]
fn rec_assert<T: ToUsize + Clone + PartialEq + Debug>(
    name: String,
    node: StrongLinkNode<T>,
    other: StrongLinkNode<T>,
) {
    println!();
    let s = name;
    println!("{}", s);
    println!(
        "value : self = {:?}, other = {:?}",
        node.value(),
        other.value()
    );
    println!(
        "parent : self = {:?}, other = {:?}",
        node.parent().is_some(),
        other.parent().is_some()
    );
    assert_eq!(node.parent().is_some(), other.parent().is_some());

    println!(
        "left : self = {:?}  other = {:?}",
        node.left().is_some(),
        other.left().is_some()
    );
    println!(
        "right : self = {:?}  other = {:?}",
        node.right().is_some(),
        other.right().is_some()
    );
    assert_eq!(node.left().is_some(), other.left().is_some());
    assert_eq!(node.right().is_some(), other.right().is_some());

    println!(
        "prev : self = {:?} other = {:?}",
        node.prev().value(),
        other.prev().value()
    );
    assert_eq!(node.prev().value(), other.prev().value());

    println!(
        "next : self = {:?} other = {:?} ",
        node.next().value(),
        other.next().value()
    );
    assert_eq!(node.next().value(), other.next().value());

    println!(
        "jump self = {:?} other = {:?}",
        node.jump().value(),
        other.jump().value()
    );
    assert_eq!(node.jump().value(), other.jump().value());

    if node.left().is_some() {
        rec_assert(format!("{}-left", s), node.left(), other.left());
    }
    if node.right().is_some() {
        rec_assert(format!("{}-right", s), node.right(), other.right());
    }
}
