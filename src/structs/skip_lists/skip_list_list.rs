use rand::{thread_rng, Rng};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::interfaces::list::List;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Node<T: Clone + Debug + Default + PartialEq + Eq> {
    x: T,
    nexts: Vec<Option<Rc<RefCell<Node<T>>>>>,
    lengths: Vec<Option<usize>>,
    height: usize,
}
impl<T: Clone + Debug + Default + PartialEq + Eq> Node<T> {
    fn new(x: T, height: usize) -> Self {
        Self {
            x,
            nexts: vec![None; height + 1],
            lengths: vec![None; height + 1],
            height: height,
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
    fn get_length(&self, h: usize) -> usize {
        if self.lengths.get(h).is_some() {
            if self.lengths.get(h).unwrap().is_some() {
                self.lengths[h].as_ref().unwrap().clone()
            } else {
                0
            }
        } else {
            0
        }
    }
    fn set_next(&mut self, h: usize, next: Option<Rc<RefCell<Node<T>>>>) {
        if self.nexts.get(h).is_some() {
            *self.nexts.get_mut(h).unwrap() = next
        } else {
            self.nexts.push(next);
        }
    }
    fn set_length(&mut self, h: usize, l: usize) {
        let l = if l == 0 { None } else { Some(l) };
        if self.lengths.get(h).is_some() {
            *self.lengths.get_mut(h).unwrap() = l
        } else {
            self.lengths.push(l);
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SkipListList<T: Clone + Debug + Default + PartialEq + Eq> {
    sentinel: Rc<RefCell<Node<T>>>,
    n: usize,
}
impl<T: Clone + Debug + Default + PartialEq + Eq> SkipListList<T> {
    pub fn new() -> Self {
        let sentinel = Rc::new(RefCell::new(Node::new(Default::default(), 0)));
        Self { sentinel, n: 0 }
    }
    fn add_base(&mut self, i: usize, x: T, h: usize) {
        if h > self.height() {
            self.change_height(h)
        }
        let new_node = Rc::new(RefCell::new(Node::new(x, h)));
        let mut prev_node = self.sentinel.clone();
        let mut r = self.height() as isize;
        let mut prev_node_index = -1;
        while r >= 0 {
            loop {
                let next = prev_node.borrow().get_next(r as usize);
                if let Some(next) = next {
                    let to_next_len =
                        (prev_node.borrow().get_length(r as usize) as isize) + prev_node_index;
                    if to_next_len < i as isize {
                        prev_node_index = to_next_len;
                        prev_node = next;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if prev_node_index >= i as isize {
                let prev_len = prev_node.borrow().get_length(r as usize);
                prev_node.borrow_mut().set_length(r as usize, prev_len + 1);
            }
            if r <= new_node.borrow().height as isize {
                let next = prev_node.borrow().get_next(r as usize);
                if next.is_some() {
                    new_node.borrow_mut().set_next(r as usize, next);
                    let len = ((i as isize) - prev_node_index) as usize;
                    new_node.borrow_mut().set_length(r as usize, len);
                }
                prev_node
                    .borrow_mut()
                    .set_next(r as usize, Some(new_node.clone()));
                let prev_len = ((i as isize) - prev_node_index) as usize;
                prev_node.borrow_mut().set_length(r as usize, prev_len);
            }
            r -= 1;
        }
        self.n += 1;
    }
    fn change_height(&mut self, h: usize) {
        let diff = h - self.height();
        self.sentinel.borrow_mut().height = h;
        for _ in 0..diff {
            self.sentinel.borrow_mut().set_length(h, 0);
            self.sentinel.borrow_mut().set_next(h, None);
        }
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
}
impl<T: Clone + Debug + Default + PartialEq + Eq> List<T> for SkipListList<T> {
    fn add(&mut self, i: usize, x: T) -> () {
        let h = self.gen_height();
        self.add_base(i, x, h);
    }
    fn get(&self, i: usize) -> Option<T> {
        None
    }
    fn remove(&mut self, i: usize) -> Option<T> {
        None
    }
    fn set(&mut self, i: usize, x: T) -> () {}
    fn size(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
mod skip_list_list_test {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn add_test() {
        let mut list = SkipListList::new();
        list.add_base(0, 0, 0);
        list.add_base(1, 1, 1);
        list.add_base(2, 2, 0);
        list.add_base(3, 3, 4);
        let sentinel = Rc::new(RefCell::new(Node::new(0, 4)));
        let zero_node = Rc::new(RefCell::new(Node::new(0, 0)));
        let one_node = Rc::new(RefCell::new(Node::new(1, 1)));
        let two_node = Rc::new(RefCell::new(Node::new(2, 0)));
        let three_node = Rc::new(RefCell::new(Node::new(3, 4)));

        zero_node.borrow_mut().set_length(0, 1);
        zero_node.borrow_mut().set_next(0, Some(one_node.clone()));

        one_node.borrow_mut().set_next(0, Some(two_node.clone()));
        one_node.borrow_mut().set_length(0, 1);

        one_node.borrow_mut().set_next(1, Some(three_node.clone()));
        one_node.borrow_mut().set_length(1, 2);

        two_node.borrow_mut().set_next(0, Some(three_node.clone()));
        two_node.borrow_mut().set_length(0, 1);

        sentinel.borrow_mut().set_length(0, 1);
        sentinel.borrow_mut().set_next(0, Some(zero_node.clone()));

        sentinel.borrow_mut().set_length(1, 2);
        sentinel.borrow_mut().set_next(1, Some(one_node.clone()));

        sentinel.borrow_mut().set_length(2, 4);
        sentinel.borrow_mut().set_next(2, Some(three_node.clone()));

        sentinel.borrow_mut().set_length(3, 4);
        sentinel.borrow_mut().set_next(3, Some(three_node.clone()));

        sentinel.borrow_mut().set_length(4, 4);
        sentinel.borrow_mut().set_next(4, Some(three_node.clone()));
        assert_eq!(list, SkipListList { n: 4, sentinel })
    }
    //#[test]
    //fn find_pred_test() {
    //let mut sentinel = Node::new(0, 1);
    //let zero_node = Rc::new(RefCell::new(Node::new(0, 0)));
    //let one_node = Rc::new(RefCell::new(Node::new(1, 1)));

    //let list = SkipListList {
    //sentinel: Rc::new(RefCell::new(sentinel)),
    //n: 2,
    //};
    //assert_eq!(list.find(0), Some(0));
    //assert_eq!(list.find(1), Some(1));
    //}
}
