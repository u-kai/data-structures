use std::{cell::RefCell, fmt::Debug, rc::Rc};

use rand::{thread_rng, Rng};

use crate::interfaces::sset::SSet;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T: Clone + Debug + PartialEq + Eq + Default + PartialOrd + Ord> {
    x: T,
    height: usize,
    nexts: Vec<Option<Rc<RefCell<Node<T>>>>>,
}
impl<T: Clone + Debug + PartialEq + Eq + Default + PartialOrd + Ord> Drop for Node<T> {
    fn drop(&mut self) {
        println!("node : {:?} is droped!!", self.x);
    }
}
impl<T: Clone + Debug + PartialEq + Eq + Default + PartialOrd + Ord> Node<T> {
    fn new(x: T, height: usize) -> Self {
        Self {
            x,
            height,
            nexts: vec![None; height + 1],
        }
    }
    fn set_next(&mut self, h: usize, next: Option<Rc<RefCell<Node<T>>>>) {
        if self.nexts.get(h).is_some() {
            *self.nexts.get_mut(h).unwrap() = next
        } else {
            self.nexts.push(next);
        }
    }
    fn get_next(&self, h: usize) -> Option<Rc<RefCell<Node<T>>>> {
        if self.nexts.get(h).is_some() {
            if self.nexts.get(h).unwrap().is_some() {
                Some(self.nexts[h].as_ref().unwrap().clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SkipListSSet<T: Clone + Debug + PartialEq + Eq + Default + PartialOrd + Ord> {
    sentinel: Rc<RefCell<Node<T>>>,
    n: usize,
}

impl<T: Clone + Debug + PartialEq + Eq + Default + PartialOrd + Ord> SkipListSSet<T> {
    #[allow(unused)]
    pub fn new() -> Self {
        let sentinel = Rc::new(RefCell::new(Node::new(Default::default(), 0)));
        Self { n: 0, sentinel }
    }
    pub fn find_pred_node(&self, x: T) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        let mut next = self.get_next(self.height());
        let mut h = self.height() as isize;
        while h >= 0 {
            if next.as_ref().unwrap().borrow().x == x {
                return Some(x);
            }
            let next_next = next.as_ref().unwrap().borrow().get_next(h as usize);
            if next_next.is_some() {
                if next_next.as_ref().unwrap().borrow().x == x {
                    return Some(x);
                }
                next = Some(next_next.as_ref().unwrap().clone());
            }
            if next_next.is_none() {
                if h == 0 {
                    return None;
                }
                h -= 1;
                next = self.get_next(h as usize);
            }
        }
        None
    }
    fn add_base(&mut self, x: T, height: usize) -> bool {
        let mut prev = self.sentinel.clone();
        let mut h = self.height() as isize;
        let mut stack = Vec::new();
        while h >= 0 {
            let next = prev.borrow().get_next(h as usize);
            match next {
                Some(next) => {
                    let next_value = &next.borrow().x;
                    if &x < next_value {
                        h -= 1;
                        stack.push(prev.clone())
                    }
                    if &x > next_value {
                        prev = next.clone();
                    }
                    if &x == next_value {
                        return false;
                    }
                }
                None => {
                    stack.push(prev.clone());
                    h -= 1;
                }
            }
        }
        let new_node = Rc::new(RefCell::new(Node::new(x.clone(), height)));
        for h in 0..=height {
            let prev_node = stack.pop();
            match prev_node {
                Some(prev_node) => {
                    let next_node = prev_node.borrow().get_next(h);
                    new_node.borrow_mut().set_next(h, next_node);
                    prev_node.borrow_mut().set_next(h, Some(new_node.clone()));
                }
                None => {
                    self.change_height(h);
                    self.set_next(h, Some(new_node.clone()));
                }
            }
        }
        self.n += 1;
        true
    }
    fn gen_height(&self) -> usize {
        let mut height = 0;
        let mut rng = thread_rng();
        let mut random: bool = rng.gen();
        while random {
            height += 1;
            random = rng.gen();
        }
        height
    }
    fn height(&self) -> usize {
        self.sentinel.borrow().height
    }
    fn change_height(&mut self, h: usize) {
        self.sentinel.borrow_mut().height = h;
    }
    fn get_next(&self, h: usize) -> Option<Rc<RefCell<Node<T>>>> {
        self.sentinel.borrow().get_next(h)
    }
    fn set_next(&mut self, h: usize, next: Option<Rc<RefCell<Node<T>>>>) {
        self.sentinel.borrow_mut().set_next(h, next)
    }
}

impl<T: Clone + Debug + PartialEq + Eq + Default + PartialOrd + Ord> SSet<T> for SkipListSSet<T> {
    fn add(&mut self, x: T) -> bool {
        let height = self.gen_height();
        self.add_base(x, height)
    }
    fn find(&self, x: T) -> bool {
        self.find_pred_node(x).is_some()
    }
    fn remove(&mut self, x: T) -> Option<T> {
        let mut removed = false;
        let mut prev = self.sentinel.clone();
        let mut h = self.height() as isize;
        while h >= 0 {
            let next = prev.borrow().get_next(h as usize);
            match next {
                Some(next) => {
                    let next_value = &next.borrow().x;
                    if &x < next_value {
                        h -= 1;
                    }
                    if &x > next_value {
                        prev = next.clone();
                    }
                    if &x == next_value {
                        removed = true;
                        let next_next = next.borrow().get_next(h as usize);
                        prev.borrow_mut().set_next(h as usize, next_next);
                        h -= 1;
                    }
                }
                None => {
                    h -= 1;
                }
            }
        }
        let mut height = self.height();
        let new_sentinel = self
            .sentinel
            .borrow()
            .nexts
            .iter()
            .filter_map(|node| {
                if node.is_none() && height != 0 {
                    height -= 1;
                    return None;
                }
                node.clone()
            })
            .map(|node| Some(node))
            .collect::<Vec<_>>();
        self.change_height(height);
        self.sentinel.borrow_mut().nexts = new_sentinel;
        if removed {
            self.n -= 1;
            Some(x)
        } else {
            None
        }
    }
    fn size(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
mod skip_list_sset_test {
    use super::*;

    #[test]
    fn add_test() {
        let mut list = SkipListSSet::new();
        assert!(list.add(0));
        assert!(list.add(1));
        assert!(list.add(3));
        assert!(list.add(4));
        assert!(!list.add(1));
    }
    #[test]
    fn add_base_test() {
        let mut list = SkipListSSet::new();
        list.add_base(0, 0);
        list.add_base(1, 1);
        list.add_base(2, 3);
        list.add_base(3, 2);
        list.add_base(10, 0);
        list.add_base(7, 10);
        let mut sentinel = Node::new(0, 10);
        let zero_node = Rc::new(RefCell::new(Node::new(0, 0)));
        let one_node = Rc::new(RefCell::new(Node::new(1, 1)));
        let two_node = Rc::new(RefCell::new(Node::new(2, 3)));
        let three_node = Rc::new(RefCell::new(Node::new(3, 2)));
        let ten_node = Rc::new(RefCell::new(Node::new(10, 0)));
        let seven_node = Rc::new(RefCell::new(Node::new(7, 10)));
        two_node.borrow_mut().set_next(0, Some(three_node.clone()));
        two_node.borrow_mut().set_next(1, Some(three_node.clone()));
        two_node.borrow_mut().set_next(2, Some(three_node.clone()));
        one_node.borrow_mut().set_next(0, Some(two_node.clone()));
        one_node.borrow_mut().set_next(1, Some(two_node.clone()));
        zero_node.borrow_mut().set_next(0, Some(one_node.clone()));
        three_node
            .borrow_mut()
            .set_next(0, Some(seven_node.clone()));
        three_node
            .borrow_mut()
            .set_next(1, Some(seven_node.clone()));
        three_node
            .borrow_mut()
            .set_next(2, Some(seven_node.clone()));
        two_node.borrow_mut().set_next(3, Some(seven_node.clone()));
        seven_node.borrow_mut().set_next(0, Some(ten_node.clone()));
        sentinel.set_next(0, Some(zero_node.clone()));
        sentinel.set_next(1, Some(one_node.clone()));
        sentinel.set_next(2, Some(two_node.clone()));
        sentinel.set_next(3, Some(two_node.clone()));
        sentinel.set_next(4, Some(seven_node.clone()));
        sentinel.set_next(5, Some(seven_node.clone()));
        sentinel.set_next(6, Some(seven_node.clone()));
        sentinel.set_next(7, Some(seven_node.clone()));
        sentinel.set_next(8, Some(seven_node.clone()));
        sentinel.set_next(9, Some(seven_node.clone()));
        sentinel.set_next(10, Some(seven_node.clone()));
        let sentinel = Rc::new(RefCell::new(sentinel));
        let tobe = SkipListSSet { sentinel, n: 6 };
        assert_eq!(list, tobe);
        assert!(!list.add_base(1, 4))
    }
    #[test]
    fn remove_test() {
        let mut list = SkipListSSet::new();
        list.add_base(0, 0);
        assert_eq!(list.remove(0), Some(0));
        assert_eq!(list.remove(0), None);
        list.add_base(0, 0);
        list.add_base(1, 1);
        list.add_base(2, 3);
        list.add_base(3, 2);
        list.add_base(7, 10);
        list.add_base(9, 11);
        list.add_base(10, 0);
        assert_eq!(list.remove(1), Some(1));
        assert_eq!(list.remove(8), None);
        assert_eq!(list.remove(9), Some(9));
        assert_eq!(list.remove(3), Some(3));
        let mut sentinel = Node::new(0, 10);
        let zero_node = Rc::new(RefCell::new(Node::new(0, 0)));
        let two_node = Rc::new(RefCell::new(Node::new(2, 3)));
        let seven_node = Rc::new(RefCell::new(Node::new(7, 10)));
        let ten_node = Rc::new(RefCell::new(Node::new(10, 0)));
        zero_node.borrow_mut().set_next(0, Some(two_node.clone()));
        two_node.borrow_mut().set_next(0, Some(seven_node.clone()));
        two_node.borrow_mut().set_next(1, Some(seven_node.clone()));
        two_node.borrow_mut().set_next(2, Some(seven_node.clone()));
        two_node.borrow_mut().set_next(3, Some(seven_node.clone()));
        seven_node.borrow_mut().set_next(0, Some(ten_node.clone()));
        sentinel.set_next(0, Some(zero_node.clone()));
        sentinel.set_next(1, Some(two_node.clone()));
        sentinel.set_next(2, Some(two_node.clone()));
        sentinel.set_next(3, Some(two_node.clone()));
        sentinel.set_next(4, Some(seven_node.clone()));
        sentinel.set_next(5, Some(seven_node.clone()));
        sentinel.set_next(6, Some(seven_node.clone()));
        sentinel.set_next(7, Some(seven_node.clone()));
        sentinel.set_next(8, Some(seven_node.clone()));
        sentinel.set_next(9, Some(seven_node.clone()));
        sentinel.set_next(10, Some(seven_node.clone()));
        let sentinel = Rc::new(RefCell::new(sentinel));
        let tobe = SkipListSSet { sentinel, n: 4 };

        assert_eq!(list, tobe);
    }
    #[test]
    fn find_test() {
        let mut list = SkipListSSet::new();
        list.add(0);
        list.add(1);
        list.add(2);
        list.add(3);
        list.add(10);
        assert!(list.find(0));
        assert!(list.find(1));
        assert!(list.find(2));
        assert!(list.find(3));
        assert!(!list.find(4));
        assert!(list.find(10));
    }
}
