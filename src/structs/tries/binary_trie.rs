use std::{
    cell::RefCell,
    fmt::Debug,
    ops::Deref,
    rc::{Rc, Weak},
};

#[derive(Debug, PartialEq)]
pub struct BinaryTrie<T: ToUsize + Clone + PartialEq + Debug> {
    root: StrongLinkNode<T>,
    min_prev: StrongLinkNode<T>,
    max_next: StrongLinkNode<T>,
    w: usize,
}
impl<T: ToUsize + Clone + PartialEq + Debug> BinaryTrie<T> {
    pub fn new(w: usize) -> Self {
        let mut min_prev = StrongLinkNode::new_node();
        let max_next = StrongLinkNode::new_node();
        min_prev.set_next(max_next.clone());
        Self {
            root: StrongLinkNode::<T>::new_node(),
            min_prev,
            max_next,
            w,
        }
    }
    fn find_prev(&self, x: T) -> StrongLinkNode<T> {
        let num_x = x.to_usize();
        let mut node = self.root.clone();
        for i in (1..=self.w).rev() {
            let binary = Self::calc_binary(num_x, i);
            let child = node.child(binary);
            if child.is_some() {
                node = child
            } else {
                node = node.jump();
                if node.is_none() {
                    return self.min_prev.clone();
                }
                if node.num() >= Some(num_x) {
                    node = node.prev()
                }
                return node;
            }
        }
        if node.num() == Some(num_x) {
            node.prev()
        } else {
            self.min_prev.clone()
        }
    }
    pub fn add(&mut self, x: T) -> bool {
        let num_x = x.to_usize();
        if num_x > 2_i32.pow(self.w as u32) as usize {
            panic!("num_x is too big! please use more large w at new method")
        }
        let leaf = StrongLinkNode::new_leaf(x);
        let mut node = self.root.clone();
        let mut prev = self.find_prev(leaf.value().unwrap());
        for i in (1..=self.w).rev() {
            let binary = Self::calc_binary(num_x, i);
            let child = node.child(binary);
            if child.is_some() {
                if i == 1 {
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
                if i != 1 {
                    let new_path_node = StrongLinkNode::new_node();
                    node.set_child(new_path_node.clone(), binary);
                    if node.child(binary.other()).is_none() {
                        node.set_jump(leaf.clone())
                    } else if node.jump().is_some() {
                        node.set_jump(StrongLinkNode(None))
                    }
                    node = new_path_node;
                    node.set_jump(leaf.clone())
                } else {
                    node.set_child(leaf.clone(), binary);
                    if node.child(binary.other()).is_none() {
                        node.set_jump(leaf.clone());
                    } else if node.jump().is_some() {
                        node.set_jump(StrongLinkNode(None))
                    }
                    let mut next = prev.next();
                    prev.set_next(leaf.clone());
                    next.set_prev(leaf.clone());
                }
            }
        }
        true
    }
    fn find_leaf(&self, x: T) -> StrongLinkNode<T> {
        let num_x = x.to_usize();
        let prev = self.find_prev(x);
        if prev.next().num() == Some(num_x) {
            prev.next()
        } else {
            StrongLinkNode(None)
        }
    }
    pub fn find(&self, x: T) -> bool {
        self.find_leaf(x).is_some()
    }
    fn calc_binary(i: usize, digit_num: usize) -> Binary {
        if (i >> (digit_num - 1) & 1) == 1 {
            Binary::One
        } else {
            Binary::Zero
        }
    }
    pub fn remove(&mut self, x: T) -> Option<T> {
        let num_x = x.to_usize();
        let remove_node = self.find_leaf(x);
        let mut prev = remove_node.prev();
        let mut next = remove_node.next();
        prev.set_next(next.clone());
        next.set_prev(prev.clone());

        if remove_node.is_some() {
            let mut parent = remove_node.parent();
            for i in 1..=self.w {
                if parent == self.root {
                    println!("root");
                    println!("remove_node {:#?} ", remove_node.value());
                }
                let binary = Self::calc_binary(num_x, i);
                let child = parent.child(binary);
                if !child.has_child() {
                    parent.set_child(StrongLinkNode(None), binary);
                }
                if parent.jump() == remove_node {
                    match binary {
                        Binary::Zero => {
                            let jump = parent.child(binary).get_max_child();
                            parent.set_jump(jump);
                        }
                        Binary::One => {
                            let jump = parent.child(binary).get_min_child();
                            parent.set_jump(jump);
                        }
                    }
                }
                if parent.child(binary).is_none() && parent.jump().is_none() {
                    match binary.other() {
                        Binary::Zero => {
                            let jump = parent.child(binary.other()).get_max_child();
                            parent.set_jump(jump);
                        }
                        Binary::One => {
                            let jump = parent.child(binary.other()).get_min_child();
                            parent.set_jump(jump);
                        }
                    }
                }
                parent = parent.parent();
            }
            remove_node.value()
        } else {
            None
        }
    }
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
        //check_use_print(tree);
        //let tobe = BinaryTrie::new(4);
        //assert_eq!(tree, tobe);
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
        let tree = BinaryTrie::new(4);
        assert_eq!(tree.find_prev(0).num(), None);
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
    fn find_test() {
        let mut tree = BinaryTrie::new(4);
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
        let mut tree = BinaryTrie::new(4);
        let mut root = StrongLinkNode::new_node();

        let mut root_left_child = StrongLinkNode::new_node();
        let mut root_left_child_left_child = StrongLinkNode::new_node();
        let mut root_left_child_left_child_right_child = StrongLinkNode::new_node();
        let mut leaf_3 = StrongLinkNode::new_leaf(3);

        root.set_jump(leaf_3.clone());
        root_left_child.set_jump(leaf_3.clone());
        root_left_child_left_child.set_jump(leaf_3.clone());
        root_left_child_left_child_right_child.set_jump(leaf_3.clone());
        root_left_child_left_child_right_child.set_right(leaf_3.clone());
        root_left_child_left_child.set_right(root_left_child_left_child_right_child.clone());
        root_left_child.set_left(root_left_child_left_child.clone());
        root.set_left(root_left_child.clone());
        tree.add(3);
        let mut min_prev = StrongLinkNode::new_node();
        let mut max_next = StrongLinkNode::new_node();
        min_prev.set_next(leaf_3.clone());
        max_next.set_prev(leaf_3.clone());
        let tobe: BinaryTrie<i32> = BinaryTrie {
            root: root.clone(),
            min_prev,
            max_next,
            w: 4,
        };
        rec_print(tobe.root.clone(), "tree_2 root");
        rec_print(tree.root.clone(), "tree root");
        assert_eq!(tree, tobe);

        let mut root_right_child = StrongLinkNode::new_node();
        let mut root_right_child_left_child = StrongLinkNode::new_node();
        let mut root_right_child_left_child_left_child = StrongLinkNode::new_node();
        let leaf_9 = StrongLinkNode::new_leaf(9);
        root.set_jump(StrongLinkNode(None));
        root_right_child.set_jump(leaf_9.clone());
        root_right_child_left_child.set_jump(leaf_9.clone());
        root_right_child_left_child_left_child.set_jump(leaf_9.clone());
        root_right_child_left_child_left_child.set_right(leaf_9.clone());
        root_right_child_left_child.set_left(root_right_child_left_child_left_child.clone());
        root_right_child.set_left(root_right_child_left_child.clone());
        root.set_right(root_right_child.clone());
        leaf_3.set_next(leaf_9.clone());
        let mut min_prev = StrongLinkNode::new_node();
        let mut max_next = StrongLinkNode::new_node();
        min_prev.set_next(leaf_3.clone());
        max_next.set_prev(leaf_9.clone());
        tree.add(9);
        let tobe = BinaryTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
        };
        assert_eq!(tree, tobe);
        let mut leaf_1 = StrongLinkNode::new_leaf(1);
        let mut root_left_child_left_child_left_child = StrongLinkNode::new_node();
        root_left_child_left_child_left_child.set_right(leaf_1.clone());
        root_left_child_left_child_left_child.set_jump(leaf_1.clone());
        min_prev.set_next(leaf_1.clone());
        leaf_1.set_next(leaf_3.clone());
        root_left_child_left_child.set_jump(StrongLinkNode(None));
        root_left_child_left_child.set_left(root_left_child_left_child_left_child.clone());
        tree.add(1);
        let tobe = BinaryTrie {
            root: root.clone(),
            w: 4,
            min_prev: min_prev.clone(),
            max_next: max_next.clone(),
        };
        assert_eq!(tree, tobe);
        let mut leaf_0 = StrongLinkNode::new_leaf(0);
        root_left_child_left_child_left_child.set_left(leaf_0.clone());
        root_left_child_left_child_left_child.set_jump(StrongLinkNode(None));
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
        let mut root_right_child_right_child = StrongLinkNode::new_node();
        let mut root_right_child_right_child_right_child = StrongLinkNode::new_node();
        root_right_child_right_child_right_child.set_jump(leaf_15.clone());
        root_right_child_right_child_right_child.set_right(leaf_15.clone());
        root_right_child_right_child.set_jump(leaf_15.clone());
        root_right_child_right_child.set_right(root_right_child_right_child_right_child.clone());
        root_right_child.set_right(root_right_child_right_child.clone());
        root_right_child.set_jump(StrongLinkNode(None));
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

        assert_eq!(tree.root.get_min_child(), leaf_0);
        assert_eq!(tree.root.get_max_child(), leaf_15);
        assert_eq!(root_right_child_left_child.get_max_child(), leaf_9);
        assert_eq!(tree, tree_2);
    }

    fn check_use_print<T: ToUsize + Clone + PartialEq + Debug>(tree: BinaryTrie<T>) {
        let mut next = tree.min_prev.clone();
        println!("from prev");
        println!();
        while next.is_some() {
            println!("next = {:#?}", next.value());
            next = next.next();
        }
        let mut prev = tree.max_next.clone();
        println!("from next");
        println!();
        while prev.is_some() {
            println!("prev = {:#?}", prev.value());
            prev = prev.prev();
        }
        let mut node = tree.root.clone();
        rec_print(node.clone(), "root");
        assert!(false);
    }
    fn rec_print<T: ToUsize + Clone + PartialEq + Debug>(node: StrongLinkNode<T>, node_name: &str) {
        println!();
        println!("{}", node_name);
        if node.value().is_some() {
            println!("value = {:?}", node.value());
        }
        println!("parent = {:?}", node.parent().is_some());
        println!(
            "left = {:?}  right = {:?}",
            node.left().is_some(),
            node.right().is_some()
        );
        if node.prev().is_some() {
            println!("prev = {:?}", node.prev().value());
        }
        if node.next().is_some() {
            println!("next = {:?}", node.next().value());
        }
        if node.jump().is_some() {
            println!("jump = {:?}", node.jump().value());
        }
        if node.left().is_some() {
            rec_print(node.left(), format!("{}-left", node_name).as_str());
        }
        if node.right().is_some() {
            rec_print(node.right(), format!("{}-right", node_name).as_str());
        }
    }
}

impl<T: ToUsize + Clone + PartialEq + Debug> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        //if self.x != other.x {
        //println!("self.x = {:?} other.x = {:?}", self.x, other.x)
        //}
        //if self.children != other.children {
        //println!(
        //"self.children = {:?} other.children = {:?}",
        //self.children, other.children
        //);
        //println!();
        //println!("node = {:#?}", self);
        //println!();
        //println!("other = {:#?}", other);
        //}
        //if self.parent.value() != other.parent.value() {
        //println!();
        //println!(
        //"self.parent = {:?} other.parent = {:?}",
        //self.parent.value(),
        //other.parent.value()
        //);
        //println!();
        //println!("node = {:#?}", self);
        //println!();
        //println!("other = {:#?}", other);
        //}
        //if self.prev.value() != other.prev.value() {
        //println!("self.prev = {:?} other.prev = {:?}", self.prev, other.prev)
        //}
        //if self.next.value() != other.next.value() {
        //println!("self.next = {:?} other.next = {:?}", self.next, other.next)
        //}
        //if self.jump.value() != other.jump.value() {
        //println!("self.jump = {:?} other.jump = {:?}", self.jump, other.jump)
        //}
        self.x == other.x
            && self.children == other.children
            && self.parent.value() == other.parent.value()
            && self.prev.value() == other.prev.value()
            && self.next.value() == other.next.value()
            && self.jump.value() == other.jump.value()
    }
}
impl<T: ToUsize + Clone + PartialEq + Debug> PartialEq for WeakLinkNode<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_node = self.0.as_ref().map(|node| node.upgrade());
        let other_node = other.0.as_ref().map(|node| node.upgrade());
        self_node == other_node
    }
}
impl<T: ToUsize + Clone + PartialEq + Debug> PartialEq for StrongLinkNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[derive(Debug, PartialEq, Clone)]

enum BinaryTrieValue<T: ToUsize + Clone + PartialEq> {
    PathNode,
    Leaf(T),
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Binary {
    Zero,
    One,
}
impl Binary {
    fn other(&self) -> Self {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Zero,
        }
    }
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
    children: [StrongLinkNode<T>; 2],
    jump: StrongLinkNode<T>,
    parent: WeakLinkNode<T>,
    prev: WeakLinkNode<T>,
    next: StrongLinkNode<T>,
}

impl<T: ToUsize + Clone + PartialEq> Node<T> {
    fn new_leaf(x: T) -> Self {
        Self {
            x: BinaryTrieValue::new_leaf(x),
            children: [StrongLinkNode::new_none(), StrongLinkNode::new_none()],
            jump: StrongLinkNode::new_none(),
            parent: WeakLinkNode::new_none(),
            prev: WeakLinkNode::new_none(),
            next: StrongLinkNode::new_none(),
        }
    }
    fn new_node() -> Self {
        Self {
            x: BinaryTrieValue::new_node(),
            children: [StrongLinkNode::new_none(), StrongLinkNode::new_none()],
            jump: StrongLinkNode::new_none(),
            parent: WeakLinkNode::new_none(),
            prev: WeakLinkNode::new_none(),
            next: StrongLinkNode::new_none(),
        }
    }
}
#[derive(Debug)]
struct StrongLinkNode<T: ToUsize + Clone + PartialEq>(Option<Rc<RefCell<Node<T>>>>);
impl<T: ToUsize + Clone + PartialEq> StrongLinkNode<T> {
    fn new_leaf(x: T) -> Self {
        Self(Some(Rc::new(RefCell::new(Node::new_leaf(x)))))
    }
    fn clone(&self) -> Self {
        StrongLinkNode(self.0.as_ref().map(|node| node.clone()))
    }
    fn get_min_child(&self) -> StrongLinkNode<T> {
        let mut node = self.clone();
        while node.left().is_some() {
            node = node.left();
        }
        if node.value().is_some() {
            node
        } else {
            node.jump()
        }
    }
    fn get_max_child(&self) -> StrongLinkNode<T> {
        let mut node = self.clone();
        while node.right().is_some() {
            node = node.right();
        }
        if node.value().is_some() {
            node
        } else {
            node.jump()
        }
    }
    fn next(&self) -> StrongLinkNode<T> {
        if let Some(next) = self.0.as_ref().map(|node| node.borrow().next.clone()) {
            next
        } else {
            StrongLinkNode(None)
        }
    }
    fn prev(&self) -> StrongLinkNode<T> {
        if let Some(prev) = self
            .0
            .as_ref()
            .map(|node| node.borrow().prev.clone().to_node())
        {
            prev
        } else {
            StrongLinkNode(None)
        }
    }
    fn to_weak(&self) -> WeakLinkNode<T> {
        WeakLinkNode(self.0.as_ref().map(|node| Rc::downgrade(node)))
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
    fn has_child(&self) -> bool {
        self.right().is_some() || self.left().is_some()
    }
    fn has_right_and_left_child(&self) -> bool {
        self.right().is_some() && self.left().is_some()
    }
    fn child(&self, binary: Binary) -> StrongLinkNode<T> {
        self.0
            .as_ref()
            .map(|node| match binary {
                Binary::Zero => node.borrow().children[0].clone(),
                Binary::One => node.borrow().children[1].clone(),
            })
            .unwrap_or(StrongLinkNode(None))
    }
    fn left(&self) -> StrongLinkNode<T> {
        self.child(Binary::Zero)
    }
    fn right(&self) -> StrongLinkNode<T> {
        self.child(Binary::One)
    }
    fn parent(&self) -> StrongLinkNode<T> {
        self.0
            .as_ref()
            .map(|node| node.borrow().parent.clone().to_node())
            .unwrap_or(StrongLinkNode(None))
    }
    fn jump(&self) -> StrongLinkNode<T> {
        self.0
            .as_ref()
            .map(|node| node.borrow().jump.clone())
            .unwrap_or(StrongLinkNode(None))
    }
    fn set_jump(&mut self, leaf: StrongLinkNode<T>) {
        self.0.as_ref().map(|node| node.borrow_mut().jump = leaf);
    }
    fn set_next(&mut self, leaf: StrongLinkNode<T>) {
        leaf.0
            .as_ref()
            .map(|node| node.borrow_mut().prev = self.clone().to_weak());
        self.0.as_ref().map(|node| node.borrow_mut().next = leaf);
    }
    fn set_prev(&mut self, leaf: StrongLinkNode<T>) {
        leaf.0
            .as_ref()
            .map(|node| node.borrow_mut().next = self.clone());
        self.0
            .as_ref()
            .map(|node| node.borrow_mut().prev = leaf.to_weak());
    }
    fn set_child(&mut self, node: StrongLinkNode<T>, binary: Binary) {
        node.clone().set_parent(self.clone());
        match binary {
            Binary::Zero => {
                self.0
                    .as_ref()
                    .map(|this| this.borrow_mut().children[0] = node);
            }
            Binary::One => {
                self.0
                    .as_ref()
                    .map(|this| this.borrow_mut().children[1] = node);
            }
        }
    }
    fn set_left(&mut self, node: StrongLinkNode<T>) {
        self.set_child(node, Binary::Zero)
    }
    fn set_right(&mut self, node: StrongLinkNode<T>) {
        self.set_child(node, Binary::One)
    }
    fn set_parent(&mut self, node: StrongLinkNode<T>) {
        self.0
            .as_ref()
            .map(|this| this.borrow_mut().parent = node.to_weak());
    }
    fn new_node() -> Self {
        Self(Some(Rc::new(RefCell::new(Node::new_node()))))
    }
    fn new_none() -> Self {
        Self(None)
    }
}
impl<T: ToUsize + Clone + PartialEq> Deref for StrongLinkNode<T> {
    type Target = Option<Rc<RefCell<Node<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug)]
struct WeakLinkNode<T: ToUsize + Clone + PartialEq>(Option<Weak<RefCell<Node<T>>>>);
impl<T: ToUsize + Clone + PartialEq> WeakLinkNode<T> {
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
        WeakLinkNode(self.0.as_ref().map(|leaf| leaf.clone()))
    }
    fn to_node(self) -> StrongLinkNode<T> {
        self.0
            .as_ref()
            .map(|leaf| {
                let leaf = leaf.upgrade();
                if leaf.is_some() {
                    StrongLinkNode(leaf)
                } else {
                    StrongLinkNode(None)
                }
            })
            .unwrap_or(StrongLinkNode(None))
    }
}
impl<T: ToUsize + Clone + PartialEq> Deref for WeakLinkNode<T> {
    type Target = Option<Weak<RefCell<Node<T>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait ToUsize {
    fn to_usize(&self) -> usize;
}
