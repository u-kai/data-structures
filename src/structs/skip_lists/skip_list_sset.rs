use std::{cell::RefCell, fmt::Debug, rc::Rc};

use rand::{thread_rng, Rng};

use crate::interfaces::sset::SSet;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T: Clone + Debug + PartialEq + Eq + Default + PartialOrd + Ord> {
    x: T,
    height: usize,
    nexts: Vec<Option<Rc<RefCell<Node<T>>>>>,
}
impl<T: Clone + Debug + PartialEq + Eq + Default + PartialOrd + Ord> Node<T> {
    fn new(x: T, height: usize) -> Self {
        Self {
            x,
            height,
            nexts: vec![None; height + 1],
        }
    }
    fn set_next(&mut self, h: usize, next: Rc<RefCell<Node<T>>>) {
        *self.nexts.get_mut(h).unwrap() = Some(next.clone())
    }
    fn set_none(&mut self, h: usize) {
        *self.nexts.get_mut(h).unwrap() = None
    }
    fn get_next(&self, h: usize) -> Option<Rc<RefCell<Node<T>>>> {
        if self.nexts.get(h).unwrap().is_some() {
            Some(self.nexts[h].as_ref().unwrap().clone())
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
    fn update_height(&mut self, height: usize) {
        let new_height = if self.height() < height {
            let push_data_num = height - self.height();
            let push_datas = vec![None; push_data_num];
            self.sentinel.borrow_mut().nexts.extend(push_datas);
            height
        } else {
            self.height()
        };
        self.sentinel.borrow_mut().height = new_height;
    }
    fn add_base_book(&mut self, x: T, height: usize) -> bool {
        let u = &self.sentinel;
        let mut r = self.height() as isize;
        let mut comp = 0;
        while r >= 0 {
            // while u.nexts.get(r).is_some() && (u.nexts)
            r = -1;
        }

        true
    }
    fn add_base(&mut self, x: T, height: usize) -> bool {
        let node = Rc::new(RefCell::new(Node::new(x, height)));
        self.n += 1;
        self.update_height(height);
        for h in (0..=height).rev() {
            let next = self.get_next(h);
            if next.is_none() {
                self.set_next(h, node.clone())
            } else {
                let mut next_next = next.as_ref().unwrap().borrow().get_next(h);
                if next_next.is_none() {
                    next.as_ref()
                        .unwrap()
                        .borrow_mut()
                        .set_next(h, node.clone());
                    continue;
                }
                while next_next.is_some() {
                    let next_next_next = next_next.as_ref().unwrap().borrow_mut().get_next(h);
                    if next_next_next.is_none() {
                        next_next
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .set_next(h, node.clone());
                        break;
                    }
                    next_next = next_next_next;
                }
                next_next.unwrap().borrow_mut().set_next(h, node.clone());
            }
        }
        true
    }
    fn remove_book(&mut self, x: T) -> Option<T> {
        let mut removed = false;
        let u = self.sentinel.clone();
        let mut r = self.height();
        let mut comp = 0;
        while r >= 0 {
            r = r + 1;
        }
        None
    }
    fn height(&self) -> usize {
        self.sentinel.borrow().height
    }
    fn get_next(&self, h: usize) -> Option<Rc<RefCell<Node<T>>>> {
        self.sentinel.borrow().get_next(h)
    }
    fn set_next(&mut self, h: usize, next: Rc<RefCell<Node<T>>>) {
        self.sentinel.borrow_mut().set_next(h, next)
    }
    fn set_none(&mut self, h: usize) {
        self.sentinel.borrow_mut().set_none(h)
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
        if self.n == 0 {
            return None;
        }
        let mut next = self.get_next(self.height());
        if next.as_ref().unwrap().borrow().x == x {
            removed = true;
            let next_next = next.as_ref().unwrap().borrow().get_next(self.height());
            if next_next.is_some() {
                next.as_mut()
                    .unwrap()
                    .borrow_mut()
                    .set_next(self.height(), next_next.as_ref().unwrap().clone());
            }
            if next_next.is_none() {
                self.set_none(self.height());
                self.sentinel.borrow_mut().height -= 1;
            }
            next = self.get_next(self.height());
        }
        let mut h = self.height() as isize;
        while h >= 0 {
            if next.as_ref().unwrap().borrow().x == x {
                removed = true;
                let next_next = next.as_ref().unwrap().borrow().get_next(self.height());
                if next_next.is_some() {
                    next.as_mut()
                        .unwrap()
                        .borrow_mut()
                        .set_next(self.height(), next_next.as_ref().unwrap().clone());
                }
                if next_next.is_none() {
                    self.set_none(self.height());
                }
            }
            let next_next = next.as_ref().unwrap().borrow().get_next(h as usize);
            if next_next.is_some() {
                if next_next.as_ref().unwrap().borrow().x == x {
                    removed = true;
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
        if removed {
            self.n -= 1;
            return Some(x);
        }
        None
    }
    fn size(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
mod skip_list_sset_test {
    use super::*;
    #[test]
    fn remove_book_test() {
        let mut list = SkipListSSet::new();
        list.add(0);
        println!("add = {:?}", list);
        assert_eq!(list.remove(0), Some(0));
        println!("remove = {:?}", list);
        list.add(1);
        list.add(2);
        list.add(3);
        list.add(10);
        assert_eq!(list.remove(0), None);
        assert_eq!(list.remove(1), Some(1));
    }
    #[test]
    fn remove_test() {
        let mut list = SkipListSSet::new();
        list.add(0);
        println!("add = {:?}", list);
        assert_eq!(list.remove(0), Some(0));
        println!("remove = {:?}", list);
        list.add(1);
        list.add(2);
        list.add(3);
        list.add(10);
        assert_eq!(list.remove(0), None);
        assert_eq!(list.remove(1), Some(2));
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
    #[test]
    fn add_book_test() {
        let mut list = SkipListSSet::new();
        list.add_base_book(0, 0);
        list.add_base_book(1, 1);
        list.add_base_book(2, 3);
        list.add_base_book(3, 2);
        let mut sentinel = Node::new(0, 3);
        let zero_node = Rc::new(RefCell::new(Node::new(0, 0)));
        let one_node = Rc::new(RefCell::new(Node::new(1, 1)));
        let two_node = Rc::new(RefCell::new(Node::new(2, 3)));
        let three_node = Rc::new(RefCell::new(Node::new(3, 2)));
        two_node.borrow_mut().set_next(0, three_node.clone());
        two_node.borrow_mut().set_next(1, three_node.clone());
        two_node.borrow_mut().set_next(2, three_node.clone());
        one_node.borrow_mut().set_next(0, two_node.clone());
        one_node.borrow_mut().set_next(1, two_node.clone());
        zero_node.borrow_mut().set_next(0, one_node.clone());
        sentinel.set_next(0, zero_node.clone());
        sentinel.set_next(1, one_node.clone());
        sentinel.set_next(2, two_node.clone());
        sentinel.set_next(3, two_node.clone());
        let sentinel = Rc::new(RefCell::new(sentinel));
        let tobe = SkipListSSet { sentinel, n: 4 };
        assert_eq!(list, tobe);
        //azero_node.nexts = Some(vec![Box::new(one_node)]);
        //alet mut three_node = Node::new(3, 2);
    }
    #[test]
    fn add_test() {
        let mut list = SkipListSSet::new();
        list.add_base(0, 0);
        list.add_base(1, 1);
        list.add_base(2, 3);
        list.add_base(3, 2);
        let mut sentinel = Node::new(0, 3);
        let zero_node = Rc::new(RefCell::new(Node::new(0, 0)));
        let one_node = Rc::new(RefCell::new(Node::new(1, 1)));
        let two_node = Rc::new(RefCell::new(Node::new(2, 3)));
        let three_node = Rc::new(RefCell::new(Node::new(3, 2)));
        two_node.borrow_mut().set_next(0, three_node.clone());
        two_node.borrow_mut().set_next(1, three_node.clone());
        two_node.borrow_mut().set_next(2, three_node.clone());
        one_node.borrow_mut().set_next(0, two_node.clone());
        one_node.borrow_mut().set_next(1, two_node.clone());
        zero_node.borrow_mut().set_next(0, one_node.clone());
        sentinel.set_next(0, zero_node.clone());
        sentinel.set_next(1, one_node.clone());
        sentinel.set_next(2, two_node.clone());
        sentinel.set_next(3, two_node.clone());
        let sentinel = Rc::new(RefCell::new(sentinel));
        let tobe = SkipListSSet { sentinel, n: 4 };
        assert_eq!(list, tobe);
        //azero_node.nexts = Some(vec![Box::new(one_node)]);
        //alet mut three_node = Node::new(3, 2);
    }
}
