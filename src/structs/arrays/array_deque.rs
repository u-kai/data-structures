use std::fmt::Debug;
#[derive(Debug)]
pub struct ArrayDeque<T: Clone + Default + Debug> {
    array: Box<[Option<T>]>,
    n: usize,
    j: usize,
}

impl<T: Clone + Default + Debug> ArrayDeque<T> {
    pub fn new() -> Self {
        let array = vec![];
        ArrayDeque {
            array: array.into_boxed_slice(),
            n: 0,
            j: 0,
        }
    }
    pub fn get(&self, i: usize) -> Option<T> {
        if i > self.n || i > self.array.len() {
            return None;
        }
        self.array
            .get((self.j + i) % self.array.len())?
            .as_ref()
            .cloned()
    }
    pub fn set(&mut self, i: usize, x: T) {
        let index = (self.j + i) % self.array.len();
        let maybe = self.array.get(index);
        if maybe.is_some() {
            *self.array.get_mut(index).unwrap() = Some(x)
        }
    }
    pub fn add(&mut self, i: usize, x: T) {
        if (self.n + 1) > self.array.len() {
            self.resize()
        }
        if i < self.n / 2 {
            self.j = if self.j == 0 {
                self.array.len() - 1
            } else {
                self.j - 1
            };
            for k in 0..(i - 1) {
                self.array.swap(
                    (self.j + k) % self.array.len(),
                    (self.j - k) % self.array.len(),
                )
            }
        } else {
            for k in ((i + 1)..=self.n).rev() {
                self.array.swap(
                    (self.j + k) % self.array.len(),
                    (self.j + k - 1) % self.array.len(),
                )
            }
        }
        self.array[(self.j + i) % self.array.len()] = Some(x);
        self.n += 1;
    }
    pub fn remove(&mut self, i: usize) -> Option<T> {
        if self.is_bound(i) {
            return None;
        }
        let x = self.array[(self.j + i) % self.array.len()].take();
        if i < self.n / 2 {
            for k in (1..=i).rev() {
                self.array.swap(
                    (self.j + k) % self.array.len(),
                    (self.j - k) % self.array.len(),
                );
            }
            self.j = (self.j + 1) % self.array.len();
        } else {
            for k in i..(self.n - 1) {
                self.array.swap(
                    (self.j + k) % self.array.len(),
                    (self.j + k + 1) % self.array.len(),
                )
            }
        }
        self.n -= 1;
        if 3 * self.n < self.array.len() {
            self.resize()
        }
        self.j = 0;
        x
    }
    fn resize(&mut self) {
        let new_array = vec![Default::default(); (self.n * 2).max(1)];
        let mut old_array = std::mem::replace(&mut self.array, new_array.into_boxed_slice());
        let len = old_array.len();
        for i in 0..self.n {
            self.array[i] = old_array[(i + self.j) % len].take();
        }
    }
    fn is_bound(&self, i: usize) -> bool {
        i > self.n || i > self.array.len()
    }
}

mod array_deque_test {
    use super::*;
    #[test]
    fn remove_test() {
        let mut deque = ArrayDeque::new();
        deque.add(0, "hello");
        deque.add(1, "world");
        assert_eq!(deque.remove(1).unwrap(), "world");
        assert_eq!(deque.remove(0).unwrap(), "hello");
        assert_eq!(deque.remove(3), None);
    }
    #[test]
    fn add_test() {
        let mut deque = ArrayDeque::new();
        deque.add(0, "hello");
        deque.add(1, "world");
        println!("{:?}", deque);
        assert_eq!(deque.get(0).unwrap(), "hello");
        assert_eq!(deque.get(1).unwrap(), "world");
        assert_eq!(deque.get(10), None);
    }
    #[test]
    fn get_test() {
        let deque = ArrayDeque {
            n: 1,
            j: 0,
            array: Box::new([Some("hello"), None, None, None]),
        };
        assert_eq!(deque.get(0).unwrap(), "hello");
        assert_eq!(deque.get(10), None);
    }
    #[test]
    fn set_test() {
        let mut deque = ArrayDeque::new();
        deque.add(0, "hello");
        deque.add(1, "world");
        deque.set(0, "see");
        deque.set(1, "you");
        assert_eq!(deque.get(0).unwrap(), "see");
        assert_eq!(deque.get(1).unwrap(), "you");
        assert_eq!(deque.get(10), None);
    }
}
