use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct BinaryHeap<T: Clone + Debug + PartialEq + PartialOrd> {
    array: Box<[Option<T>]>,
    n: usize,
}

impl<T: Clone + Debug + PartialEq + PartialOrd> BinaryHeap<T> {
    pub fn new() -> Self {
        BinaryHeap {
            array: Box::new([]),
            n: 0,
        }
    }
    pub fn add(&mut self, x: T) -> bool {
        if self.n + 1 > self.array.len() {
            self.resize()
        }
        self.array[self.n] = Some(x);
        self.bubble_up(self.n);
        self.n += 1;
        true
    }
    pub fn remove(&mut self) -> Option<T> {
        let removed = self.array[0].take();
        self.n -= 1;
        self.array.swap(0, self.n);
        self.trickle_down(0);
        if self.n * 3 < self.array.len() {
            self.resize()
        }
        removed
    }
    fn trickle_down(&mut self, mut i: usize) {
        loop {
            let mut j = -1;
            let r = Self::right_index(i);
            if r < self.n && self.array[r] < self.array[i] {
                let l = Self::left_index(i);
                if self.array[l] < self.array[r] {
                    j = l as isize
                } else {
                    j = r as isize
                }
            } else {
                let l = Self::left_index(i);
                if l < self.n && self.array[l] < self.array[r] {
                    j = l as isize
                }
            }
            if j >= 0 {
                self.array.swap(i, j as usize);
                i = j as usize
            } else {
                return;
            }
        }
    }
    fn bubble_up(&mut self, mut i: usize) {
        if i == 0 {
            return;
        }
        let mut parent_index = Self::parent_index(i);
        while i > 0 && self.array[i] < self.array[parent_index] {
            println!("parent = {:?}", self.array[parent_index]);
            self.array.swap(i, parent_index);
            i = parent_index;
            if i == 0 {
                return;
            }
            parent_index = Self::parent_index(i);
        }
    }
    fn left_index(i: usize) -> usize {
        i * 2 + 1
    }
    fn right_index(i: usize) -> usize {
        i * 2 + 2
    }
    fn parent_index(i: usize) -> usize {
        (i - 1) / 2
    }
    fn resize(&mut self) {
        let new_array = vec![None; (self.n * 2).max(1)];
        let mut new_array = new_array.into_boxed_slice();
        for i in 0..self.n {
            new_array[i] = self.array[i].take();
        }
        self.array = new_array;
    }
}
#[cfg(test)]
mod binary_heap_test {
    use super::*;
    #[test]
    fn remove_test() {
        let mut test_node = BinaryHeap {
            array: Box::new([
                Some(4),
                Some(9),
                Some(6),
                Some(17),
                Some(26),
                Some(8),
                Some(16),
                Some(19),
                Some(69),
                Some(32),
                Some(93),
                Some(55),
                Some(50),
                None,
                None,
            ]),
            n: 13,
        };
        assert_eq!(test_node.remove(), Some(4));
        let mut tobe = BinaryHeap {
            array: Box::new([
                Some(6),
                Some(9),
                Some(8),
                Some(17),
                Some(26),
                Some(50),
                Some(16),
                Some(19),
                Some(69),
                Some(32),
                Some(93),
                Some(55),
                None,
                None,
                None,
            ]),
            n: 12,
        };
        assert_eq!(test_node, tobe);
    }
    #[test]
    fn add_test() {
        let mut test_node = BinaryHeap {
            array: Box::new([
                Some(4),
                Some(9),
                Some(8),
                Some(17),
                Some(26),
                Some(50),
                Some(16),
                Some(19),
                Some(69),
                Some(32),
                Some(93),
                Some(55),
                None,
                None,
                None,
            ]),
            n: 12,
        };
        test_node.add(6);
        let tobe = BinaryHeap {
            array: Box::new([
                Some(4),
                Some(9),
                Some(6),
                Some(17),
                Some(26),
                Some(8),
                Some(16),
                Some(19),
                Some(69),
                Some(32),
                Some(93),
                Some(55),
                Some(50),
                None,
                None,
            ]),
            n: 13,
        };
        assert_eq!(test_node, tobe);
    }
}
