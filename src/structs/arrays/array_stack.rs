use std::fmt::Debug;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ArrayStack<T: Debug + Clone + Default> {
    array: Box<[Option<T>]>,
    n: usize,
}
impl<T: Debug + Clone + Default> Iterator for ArrayStack<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.remove(0) {
            Some(next)
        } else {
            None
        }
    }
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
    pub fn get(&self, i: usize) -> Option<T> {
        self.array.get(i).unwrap().clone()
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
    pub fn remove(&mut self, i: usize) -> Option<T> {
        if self.array.get(i).is_none() {
            return None;
        }
        let x = self.array[i].take();
        if x.is_none() {
            return None;
        }
        for j in i..(self.n - 1) {
            self.array.swap(j, j + 1);
        }
        self.n -= 1;
        if self.array.len() >= 3 * self.n {
            self.resize()
        }
        x
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
    fn iter_test() {
        let mut array_stack = ArrayStack::new();
        array_stack.add(0, "world");
        array_stack.add(0, "hello");
        assert_eq!(array_stack.next(), Some("hello"));
        assert_eq!(array_stack.next(), Some("world"));
        assert_eq!(array_stack.next(), None);
    }
    #[test]
    fn remove_test() {
        let mut array_stack = ArrayStack::new();
        array_stack.add(0, "world");
        array_stack.add(0, "hello");
        array_stack.remove(0);
        assert_eq!(
            array_stack,
            ArrayStack {
                n: 1,
                array: Box::new([Some("world"), None])
            }
        )
    }
    #[test]
    fn add_test() {
        let mut array_stack = ArrayStack::new();
        array_stack.add(0, "hello");
        array_stack.add(1, "world");
        array_stack.add(0, "good");
        array_stack.add(1, "bye");
        assert_eq!(
            array_stack,
            ArrayStack {
                n: 4,
                array: Box::new([Some("good"), Some("bye"), Some("hello"), Some("world")])
            }
        )
    }
}
