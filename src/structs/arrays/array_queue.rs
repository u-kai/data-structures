use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq)]
struct ArrayQueue<T: Clone + Debug + Default> {
    n: usize,
    j: usize,
    array: Box<[Option<T>]>,
}

impl<T: Clone + Debug + Default> ArrayQueue<T> {
    pub fn new() -> Self {
        let data = vec![Default::default(); 4];
        ArrayQueue {
            n: 0,
            j: 0,
            array: data.into_boxed_slice(),
        }
    }
    pub fn add(&mut self, x: T) {
        if (self.n + 1) > self.array.len() {
            self.resize()
        }
        *self
            .array
            .get_mut((self.j + self.n) % self.array.len())
            .unwrap() = Some(x);
        self.n += 1;
    }
    fn remove(&mut self) -> Option<T> {
        if self.array.len() == 0 {
            return None;
        }
        let x = self.array[self.j].take();
        self.j = (self.j + 1) % self.array.len();
        self.n -= 1;
        if self.array.len() >= 3 * self.n {
            self.resize();
        }
        x
    }
    fn resize(&mut self) {
        let new_array = vec![Default::default(); self.n * 2];
        let mut old_array = std::mem::replace(&mut self.array, new_array.into_boxed_slice());
        let len = old_array.len();
        for i in 0..self.n {
            self.array[i] = old_array[(i + self.j) % len].take();
        }
        self.j = 0;
    }
}

#[cfg(test)]
mod array_queue_test {
    use super::*;
    #[test]
    fn remove_test() {
        let mut array = ArrayQueue::new();
        array.add("hello");
        array.add("world");
        array.add("goodbye");
        assert_eq!(array.remove().unwrap(), "hello");
        assert_eq!(array.remove().unwrap(), "world");
        assert_eq!(array.remove().unwrap(), "goodbye");
        assert_eq!(array.remove(), None);
        assert_eq!(array.remove(), None);
    }
    #[test]
    fn add_test() {
        let mut array = ArrayQueue::new();
        array.add("hello");
        array.add("world");
        assert_eq!(
            array,
            ArrayQueue {
                n: 2,
                j: 0,
                array: Box::new([Some("hello"), Some("world"), None, None])
            }
        );
        array.add("goodbye");
        array.add("world");
        array.add("thanks");
        assert_eq!(
            array,
            ArrayQueue {
                n: 5,
                j: 0,
                array: Box::new([
                    Some("hello"),
                    Some("world"),
                    Some("goodbye"),
                    Some("world"),
                    Some("thanks"),
                    None,
                    None,
                    None
                ])
            }
        );
        array.add("you");
    }
}
