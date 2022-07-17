use std::{cell::RefCell, fmt::Debug, rc::Rc};

use rand::{prelude::ThreadRng, thread_rng, Rng};

use crate::interfaces::sset::SSet;
#[derive(Clone, Debug, PartialEq, Eq)]
struct Node<T: Clone + Debug + PartialEq + Eq + Default> {
    x: T,
    height: usize,
    nexts: Vec<Option<Rc<RefCell<Node<T>>>>>,
}
impl<T: Clone + Debug + PartialEq + Eq + Default> Node<T> {
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
    fn get_next(&self, h: usize) -> Option<Rc<RefCell<Node<T>>>> {
        if self.nexts.get(h).unwrap().is_some() {
            Some(self.nexts[h].as_ref().unwrap().clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SkipListSSet<T: Clone + Debug + PartialEq + Eq + Default> {
    sentinel: Node<T>,
    n: usize,
}

impl<T: Clone + Debug + PartialEq + Eq + Default> SkipListSSet<T> {
    pub fn new() -> Self {
        let sentinel = Node::new(Default::default(), 0);
        Self { n: 0, sentinel }
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
    fn change_height(&mut self, height: usize) {
        self.sentinel.height = if self.sentinel.height < height {
            let push_data_num = height - self.sentinel.height;
            let push_datas = vec![None; push_data_num];
            self.sentinel.nexts.extend(push_datas);
            height
        } else {
            self.sentinel.height
        };
    }
    fn add_base(&mut self, x: T, height: usize) -> bool {
        let node = Rc::new(RefCell::new(Node::new(x, height)));
        self.n += 1;
        self.change_height(height);
        if self.sentinel.height == 0 {
            if self.sentinel.nexts[0].is_none() {
                self.sentinel.nexts[0] = Some(node.clone());
                return true;
            }
        }
        for h in (0..=height).rev() {
            let next = self.sentinel.get_next(h);
            if next.is_none() {
                self.sentinel.set_next(h, node.clone())
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
}

impl<T: Clone + Debug + PartialEq + Eq + Default> SSet<T> for SkipListSSet<T> {
    fn add(&mut self, x: T) -> bool {
        let height = self.gen_height();
        self.add_base(x, height)
    }
    fn find(&self, x: T) -> bool {
        true
    }
    fn remove(&mut self, x: T) -> Option<T> {
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
        let tobe = SkipListSSet { sentinel, n: 4 };
        assert_eq!(list, tobe);
        //azero_node.nexts = Some(vec![Box::new(one_node)]);
        //alet mut three_node = Node::new(3, 2);
    }
}
