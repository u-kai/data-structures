use std::{
    cell::RefCell,
    fmt::Debug,
    ops::Deref,
    rc::{Rc, Weak},
};

#[derive(Debug, PartialEq)]
pub struct BinaryTrie<T: ToUsize + Clone + PartialEq + Debug> {
    root: WrapNode<T>,
    min_prev: WrapNode<T>,
    max_next: WrapNode<T>,
    w: usize,
}
impl<T: ToUsize + Clone + PartialEq + Debug> BinaryTrie<T> {
    pub fn new(w: usize) -> Self {
        let mut min_prev = WrapNode::new_node();
        let mut max_next = WrapNode::new_node();
        min_prev.set_next(max_next.clone().to_leaf());
        max_next.set_prev(min_prev.clone().to_leaf());
        Self {
            root: WrapNode::<T>::new_node(),
            min_prev,
            max_next,
            w,
        }
    }
    pub fn add(&mut self, x: T) -> bool {
        let num_x = x.to_usize();
        if num_x > 2_i32.pow(self.w as u32) as usize {
            panic!("num_x is too big! please use more large w at new method")
        }
        let leaf = WrapNode::new_leaf(x);
        let mut node = self.root.clone();
        for i in (1..=self.w).rev() {
            let binary = Self::calc_binary(num_x, i);
            match binary {
                Binary::One => {
                    let right = node.right();
                    if right.is_some() {
                        if i == 1 {
                            return false;
                        }
                        node = right;
                    } else {
                        if i != 1 {
                            let new_path_node = WrapNode::new_node();
                            node.set_right(new_path_node.clone());
                            node = new_path_node.clone();
                            node.set_jump(leaf.clone().to_leaf());
                        } else {
                            // case leaf parent
                            node.set_right(leaf.clone());
                            if node.left().is_none() {
                                node.set_jump(leaf.clone().to_leaf());
                            } else {
                                node.set_jump(WrapLeaf(None))
                            }
                            let mut prev = self.min_prev.clone();
                            let mut next = prev.next();
                            let mut next_value = next.num();
                            while next_value.is_some() && next_value < Some(num_x) {
                                prev = next.clone();
                                next = next.next();
                                next_value = next.num()
                            }
                            prev.set_next(leaf.clone().to_leaf());
                            next.set_prev(leaf.clone().to_leaf());
                        }
                    }
                }
                Binary::Zero => {
                    let left = node.left();
                    if left.is_some() {
                        if i == 1 {
                            return false;
                        }
                        node = left;
                    } else {
                        if i != 1 {
                            let new_path_node = WrapNode::new_node();
                            node.set_left(new_path_node.clone());
                            node = new_path_node.clone();
                            node.set_jump(leaf.clone().to_leaf());
                        } else {
                            // case leaf parent
                            node.set_left(leaf.clone());
                            if node.right().is_none() {
                                node.set_jump(leaf.clone().to_leaf());
                            } else {
                                node.set_jump(WrapLeaf(None))
                            }
                            let mut prev = self.min_prev.clone();
                            let mut next = prev.next();
                            let mut next_value = next.num();
                            while next_value.is_some() && next_value < Some(num_x) {
                                prev = next.clone();
                                next = next.next();
                                next_value = next.num()
                            }
                            prev.set_next(leaf.clone().to_leaf());
                            next.set_prev(leaf.clone().to_leaf());
                        }
                    }
                }
            }
        }
        true
    }
    fn calc_binary(i: usize, digit_num: usize) -> Binary {
        if (i >> (digit_num - 1) & 1) == 1 {
            Binary::One
        } else {
            Binary::Zero
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
enum BinaryTrieValue<T: ToUsize + Clone + PartialEq> {
    PathNode,
    Leaf(T),
}
#[derive(Debug, PartialEq, Clone)]
enum Binary {
    Zero,
    One,
}
impl<T: ToUsize + Clone + PartialEq> BinaryTrieValue<T> {
    fn new_leaf(x: T) -> Self {
        BinaryTrieValue::Leaf(x)
    }
    fn new_node() -> Self {
        BinaryTrieValue::PathNode
    }
}
#[derive(Debug)]
struct Node<T: ToUsize + Clone + PartialEq> {
    x: BinaryTrieValue<T>,
    jump: WrapLeaf<T>,
    right: WrapNode<T>,
    left: WrapNode<T>,
    parent: ParentNode<T>,
    prev: WrapLeaf<T>,
    next: WrapLeaf<T>,
}
impl<T: ToUsize + Clone + PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.left == other.left
            && self.right == other.right
            && self.parent.value() == other.parent.value()
            && self.prev.value() == other.prev.value()
            && self.next.value() == other.next.value()
    }
}

impl<T: ToUsize + Clone + PartialEq> Node<T> {
    fn new_leaf(x: T) -> Self {
        Self {
            x: BinaryTrieValue::new_leaf(x),
            jump: WrapLeaf::new_none(),
            right: WrapNode::new_none(),
            left: WrapNode::new_none(),
            parent: ParentNode::new_none(),
            prev: WrapLeaf::new_none(),
            next: WrapLeaf::new_none(),
        }
    }
    fn new_node() -> Self {
        Self {
            x: BinaryTrieValue::new_node(),
            jump: WrapLeaf::new_none(),
            right: WrapNode::new_none(),
            left: WrapNode::new_none(),
            parent: ParentNode::new_none(),
            prev: WrapLeaf::new_none(),
            next: WrapLeaf::new_none(),
        }
    }
}
#[derive(Debug)]
struct WrapNode<T: ToUsize + Clone + PartialEq>(Option<Rc<RefCell<Node<T>>>>);
impl<T: ToUsize + Clone + PartialEq> PartialEq for WrapNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ToUsize + Clone + PartialEq> WrapNode<T> {
    fn new_leaf(x: T) -> Self {
        Self(Some(Rc::new(RefCell::new(Node::new_leaf(x)))))
    }
    fn clone(&self) -> Self {
        WrapNode(self.0.as_ref().map(|node| node.clone()))
    }
    fn next(&self) -> WrapNode<T> {
        if let Some(next) = self
            .0
            .as_ref()
            .map(|node| node.borrow().next.clone().to_node())
        {
            next
        } else {
            WrapNode(None)
        }
    }
    fn prev(&self) -> WrapNode<T> {
        if let Some(prev) = self
            .0
            .as_ref()
            .map(|node| node.borrow().prev.clone().to_node())
        {
            prev
        } else {
            WrapNode(None)
        }
    }
    fn to_leaf(&self) -> WrapLeaf<T> {
        WrapLeaf(self.0.as_ref().map(|node| Rc::downgrade(node)))
    }
    fn num(&self) -> Option<usize> {
        let result = self.0.as_ref().map(|node| match &node.borrow().x {
            BinaryTrieValue::Leaf(x) => Some(x.to_usize()),
            _ => None,
        });
        match result {
            Some(non_or_some) => non_or_some,
            None => None,
        }
    }
    fn value(&self) -> Option<T> {
        if self.0.is_some() {
            match self.0.as_ref().unwrap().borrow().x.clone() {
                BinaryTrieValue::Leaf(x) => Some(x),
                _ => None,
            }
        } else {
            None
        }
    }
    fn left(&self) -> WrapNode<T> {
        self.0
            .as_ref()
            .map(|node| node.borrow().left.clone())
            .unwrap_or(WrapNode(None))
    }
    fn right(&self) -> WrapNode<T> {
        self.0
            .as_ref()
            .map(|node| node.borrow().right.clone())
            .unwrap_or(WrapNode(None))
    }
    fn parent(&self) -> WrapNode<T> {
        self.0
            .as_ref()
            .map(|node| node.borrow().parent.clone().to_node())
            .unwrap_or(WrapNode(None))
    }
    fn jump(&self) -> WrapNode<T> {
        self.0
            .as_ref()
            .map(|node| node.borrow().jump.clone().to_node())
            .unwrap_or(WrapNode(None))
    }
    fn set_jump(&mut self, leaf: WrapLeaf<T>) {
        self.0.as_ref().map(|node| node.borrow_mut().jump = leaf);
    }
    fn set_next(&mut self, leaf: WrapLeaf<T>) {
        leaf.clone().0.as_ref().map(|node| {
            if let Some(node) = node.upgrade() {
                node.borrow_mut().prev = self.clone().to_leaf()
            }
        });
        self.0.as_ref().map(|node| node.borrow_mut().next = leaf);
    }
    fn set_prev(&mut self, leaf: WrapLeaf<T>) {
        leaf.clone().0.as_ref().map(|node| {
            if let Some(node) = node.upgrade() {
                node.borrow_mut().next = self.clone().to_leaf()
            }
        });
        self.0.as_ref().map(|node| node.borrow_mut().prev = leaf);
    }
    fn set_left(&mut self, node: WrapNode<T>) {
        node.clone().set_parent(self.clone());
        self.0.as_ref().map(|this| this.borrow_mut().left = node);
    }
    fn set_right(&mut self, node: WrapNode<T>) {
        node.clone().set_parent(self.clone());
        self.0.as_ref().map(|this| this.borrow_mut().right = node);
    }
    fn set_parent(&mut self, node: WrapNode<T>) {
        self.0
            .as_ref()
            .map(|this| this.borrow_mut().parent = node.to_parent());
    }
    fn to_parent(self) -> ParentNode<T> {
        ParentNode(self.0.map(|node| Rc::downgrade(&node)))
    }
    fn new_node() -> Self {
        Self(Some(Rc::new(RefCell::new(Node::new_node()))))
    }
    fn new_none() -> Self {
        Self(None)
    }
}
impl<T: ToUsize + Clone + PartialEq> Deref for WrapNode<T> {
    type Target = Option<Rc<RefCell<Node<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug)]
struct ParentNode<T: ToUsize + Clone + PartialEq>(Option<Weak<RefCell<Node<T>>>>);
impl<T: ToUsize + Clone + PartialEq> ParentNode<T> {
    fn new(node: WrapNode<T>) -> Self {
        let node = node.0.as_ref().map(|node| Rc::downgrade(node));
        Self(node)
    }
    fn value(&self) -> Option<T> {
        if self.0.is_some() {
            let parent = self.0.as_ref().unwrap().upgrade();
            if let Some(parent) = parent {
                match parent.borrow().x.clone() {
                    BinaryTrieValue::Leaf(x) => Some(x),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    fn new_none() -> Self {
        Self(None)
    }
    fn to_node(self) -> WrapNode<T> {
        self.0
            .as_ref()
            .map(|parent| {
                let parent = parent.upgrade();
                if parent.is_some() {
                    WrapNode(parent)
                } else {
                    WrapNode(None)
                }
            })
            .unwrap_or(WrapNode(None))
    }
    fn clone(&self) -> Self {
        ParentNode(self.0.as_ref().map(|parent| parent.clone()))
    }
}
impl<T: ToUsize + Clone + PartialEq> Deref for ParentNode<T> {
    type Target = Option<Weak<RefCell<Node<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: ToUsize + Clone + PartialEq> PartialEq for ParentNode<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_node = self.0.as_ref().map(|node| node.upgrade());
        let other_node = other.0.as_ref().map(|node| node.upgrade());
        self_node == other_node
    }
}
#[derive(Debug)]
struct WrapLeaf<T: ToUsize + Clone + PartialEq>(Option<Weak<RefCell<Node<T>>>>);
impl<T: ToUsize + Clone + PartialEq> WrapLeaf<T> {
    fn new(x: T) -> Self {
        let node = WrapNode::new_leaf(x);
        Self::from_node(node)
    }
    fn from_node(node: WrapNode<T>) -> Self {
        let node = node.0.as_ref().map(|node| Rc::downgrade(node));
        Self(node)
    }
    fn new_none() -> Self {
        Self(None)
    }
    fn value(&self) -> Option<T> {
        if self.0.is_some() {
            let node = self.0.as_ref().unwrap().upgrade();
            if let Some(node) = node {
                match node.borrow().x.clone() {
                    BinaryTrieValue::Leaf(x) => Some(x),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    fn clone(&self) -> Self {
        WrapLeaf(self.0.as_ref().map(|leaf| leaf.clone()))
    }
    fn to_node(self) -> WrapNode<T> {
        self.0
            .as_ref()
            .map(|leaf| {
                let leaf = leaf.upgrade();
                if leaf.is_some() {
                    WrapNode(leaf)
                } else {
                    WrapNode(None)
                }
            })
            .unwrap_or(WrapNode(None))
    }
}
impl<T: ToUsize + Clone + PartialEq> Deref for WrapLeaf<T> {
    type Target = Option<Weak<RefCell<Node<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: ToUsize + Clone + PartialEq> PartialEq for WrapLeaf<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_node = self.0.as_ref().map(|node| node.upgrade());
        let other_node = other.0.as_ref().map(|node| node.upgrade());
        self_node == other_node
    }
}

pub trait ToUsize {
    fn to_usize(&self) -> usize;
}

#[cfg(test)]

mod binary_trie_test {

    use super::*;
    impl ToUsize for i32 {
        fn to_usize(&self) -> usize {
            self.clone() as usize
        }
    }
    #[test]
    fn calc_binary_test() {
        assert_eq!(BinaryTrie::<i32>::calc_binary(255, 1), Binary::One);
        assert_eq!(BinaryTrie::<i32>::calc_binary(255, 2), Binary::One);
        assert_eq!(BinaryTrie::<i32>::calc_binary(255, 3), Binary::One);
        assert_eq!(BinaryTrie::<i32>::calc_binary(255, 4), Binary::One);
        assert_eq!(BinaryTrie::<i32>::calc_binary(255, 5), Binary::One);
        assert_eq!(BinaryTrie::<i32>::calc_binary(255, 6), Binary::One);
        assert_eq!(BinaryTrie::<i32>::calc_binary(255, 7), Binary::One);
        assert_eq!(BinaryTrie::<i32>::calc_binary(255, 8), Binary::One);
        assert_eq!(BinaryTrie::<i32>::calc_binary(255, 9), Binary::Zero);
        assert_eq!(BinaryTrie::<i32>::calc_binary(0, 1), Binary::Zero);
        assert_eq!(BinaryTrie::<i32>::calc_binary(8, 5), Binary::Zero);
    }
    #[test]
    fn add_test() {
        let mut tree = BinaryTrie::new(4);
        let mut root = WrapNode::new_node();

        let mut root_left_child = WrapNode::new_node();
        let mut root_left_child_left_child = WrapNode::new_node();
        let mut root_left_child_left_child_right_child = WrapNode::new_node();
        let mut leaf_3 = WrapNode::new_leaf(3);

        root.set_jump(leaf_3.clone().to_leaf());
        root_left_child.set_jump(leaf_3.clone().to_leaf());
        root_left_child_left_child.set_jump(leaf_3.clone().to_leaf());
        root_left_child_left_child_right_child.set_jump(leaf_3.clone().to_leaf());
        root_left_child_left_child_right_child.set_right(leaf_3.clone());
        root_left_child_left_child.set_right(root_left_child_left_child_right_child.clone());
        root_left_child.set_left(root_left_child_left_child.clone());
        root.set_left(root_left_child.clone());
        tree.add(3);
        let mut min_prev = WrapNode::new_node();
        let mut max_next = WrapNode::new_node();
        min_prev.set_next(leaf_3.clone().to_leaf());
        max_next.set_prev(leaf_3.clone().to_leaf());
        let tobe: BinaryTrie<i32> = BinaryTrie {
            root: root.clone(),
            min_prev,
            max_next,
            w: 4,
        };
        assert_eq!(tree, tobe);

        let mut root_right_child = WrapNode::new_node();
        let mut root_right_child_left_child = WrapNode::new_node();
        let mut root_right_child_left_child_left_child = WrapNode::new_node();
        let leaf_9 = WrapNode::new_leaf(9);
        root.set_jump(WrapLeaf(None));
        root_right_child.set_jump(leaf_9.clone().to_leaf());
        root_right_child_left_child.set_jump(leaf_9.clone().to_leaf());
        root_right_child_left_child_left_child.set_jump(leaf_9.clone().to_leaf());
        root_right_child_left_child_left_child.set_right(leaf_9.clone());
        root_right_child_left_child.set_left(root_right_child_left_child_left_child.clone());
        root_right_child.set_left(root_right_child_left_child.clone());
        root.set_right(root_right_child.clone());
        leaf_3.set_next(leaf_9.clone().to_leaf());
        let mut min_prev = WrapNode::new_node();
        let mut max_next = WrapNode::new_node();
        min_prev.set_next(leaf_3.clone().to_leaf());
        max_next.set_prev(leaf_9.clone().to_leaf());
        tree.add(9);
        let tobe = BinaryTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
        };
        assert_eq!(tree, tobe);
        let mut leaf_1 = WrapNode::new_leaf(1);
        let mut root_left_child_left_child_left_child = WrapNode::new_node();
        root_left_child_left_child_left_child.set_right(leaf_1.clone());
        root_left_child_left_child_left_child.set_jump(leaf_1.clone().to_leaf());
        min_prev.set_next(leaf_1.clone().to_leaf());
        leaf_1.set_next(leaf_3.clone().to_leaf());
        root_left_child_left_child.set_jump(WrapLeaf(None));
        root_left_child_left_child.set_left(root_left_child_left_child_left_child.clone());
        tree.add(1);
        let tobe = BinaryTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
        };
        assert_eq!(tree, tobe);
        let mut leaf_0 = WrapNode::new_leaf(0);
        root_left_child_left_child_left_child.set_left(leaf_0.clone());
        root_left_child_left_child_left_child.set_jump(WrapLeaf(None));
        min_prev.set_next(leaf_0.clone().to_leaf());
        leaf_0.set_next(leaf_1.clone().to_leaf());
        let tobe = BinaryTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
        };
        tree.add(0);
        assert_eq!(tree, tobe);
        let mut leaf_15 = WrapNode::new_leaf(15);
        let mut root_right_child_right_child = WrapNode::new_node();
        let mut root_right_child_right_child_right_child = WrapNode::new_node();
        root_right_child_right_child_right_child.set_jump(leaf_15.clone().to_leaf());
        root_right_child_right_child_right_child.set_right(leaf_15.clone());
        root_right_child_right_child.set_jump(leaf_15.clone().to_leaf());
        root_right_child_right_child.set_right(root_right_child_right_child_right_child.clone());
        root_right_child.set_right(root_right_child_right_child.clone());
        root_right_child.set_jump(WrapLeaf(None));
        max_next.set_prev(leaf_15.clone().to_leaf());
        leaf_15.set_prev(leaf_9.clone().to_leaf());
        let tobe = BinaryTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
        };
        tree.add(15);
        assert_eq!(tree, tobe);
        check_use_print(tree);
    }

    fn check_use_print<T: ToUsize + Clone + PartialEq + Debug>(tree: BinaryTrie<T>) {
        let mut next = tree.min_prev.clone();
        while next.is_some() {
            println!("next = {:#?}", next.value());
            next = next.next();
        }
        let mut prev = tree.max_next.clone();
        while prev.is_some() {
            println!("prev = {:#?}", prev.value());
            prev = prev.prev();
        }
        println!("tree = {:#?}", tree);
        assert!(false);
    }
}
