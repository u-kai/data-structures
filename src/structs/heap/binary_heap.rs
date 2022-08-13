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
    fn bubble_up(&mut self, mut i: usize) {
        if i == 0 {
            return;
        }
        let mut parent_index = Self::parent_index(i);
        while i > 0 && self.array[i] < self.array[parent_index] {
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
    fn binary_heap_test() {
        let tobe = BinaryHeap {
            array: Box::new([
                Some(0),
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
                Some(9),
                Some(10),
                Some(11),
                Some(12),
                Some(13),
                Some(14),
                None,
            ]),
            n: 15,
        };
        let mut heap = BinaryHeap::new();
        heap.add(0);
        heap.add(1);
        heap.add(2);
        heap.add(3);
        heap.add(4);
        heap.add(5);
        heap.add(6);
        heap.add(7);
        heap.add(8);
        heap.add(9);
        heap.add(10);
        heap.add(11);
        heap.add(12);
        heap.add(13);
        heap.add(14);
        assert_eq!(heap, tobe);
    }
}
