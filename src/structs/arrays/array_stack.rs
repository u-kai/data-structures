use std::fmt::Debug;

#[derive(Eq, PartialEq, Debug)]
pub struct ArrayStack<T: Debug + Clone + Default> {
    array: Box<[Option<T>]>,
    n: usize,
}

impl<T: Debug + Clone + Default> ArrayStack<T> {
    pub fn new() -> Self {
        ArrayStack {
            array: Box::new([]),
            n: 0,
        }
    }
    pub fn size(&self) -> usize {
        self.n
    }
    pub fn add(&mut self, i: usize, x: T) {
        if (self.n + 1) > self.array.len() {
            self.resize();
        }
        for j in ((i + 1)..=self.n).rev() {
            self.array.swap(j, j - 1);
        }
        self.array[i] = Some(x);
        self.n += 1;
    }
    fn resize(&mut self) {
        let new_array: Vec<Option<T>> = vec![Default::default(); (self.n * 2).max(1)];
        let mut new_array = new_array.into_boxed_slice();
        for i in 0..self.n {
            new_array[i] = self.array[i].take();
        }
        self.array = new_array;
    }
}

#[cfg(test)]

mod array_stack_test {
    use super::*;
    #[test]
    fn add_test() {
        let mut array_stack = ArrayStack::new();
        array_stack.add(0, "hello");
        array_stack.add(1, "world");
        assert_eq!(
            array_stack,
            ArrayStack {
                n: 2,
                array: Box::new([Some("hello"), Some("world")])
            }
        )
    }
}
