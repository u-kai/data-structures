use std::fmt::Debug;

use crate::{interfaces::uset::USet, structs::arrays::array_stack::ArrayStack};

#[derive(Debug, Clone, PartialEq, Eq)]
enum DataState<T: Clone + Debug + Eq + PartialEq + Default> {
    Exist(T),
    Null,
    Del,
}
impl<T: Clone + Debug + Eq + PartialEq + Default> Default for DataState<T> {
    fn default() -> Self {
        DataState::Null
    }
}

pub struct LinerHashTable<T: Clone + Debug + Eq + PartialEq + Default> {
    array: ArrayStack<DataState<T>>,
    n: usize,
    q: usize,
    d: u32,
}

impl<T: Clone + Debug + Eq + PartialEq + Default> LinerHashTable<T> {
    fn new() -> Self {
        let d = 8;
        let len = 2_i32.pow(d) as usize;
        let array = ArrayStack::new_with_len(len);
        Self {
            d,
            array,
            q: 0,
            n: 0,
        }
    }
}
impl<T: Clone + Debug + Eq + PartialEq + Default> USet<T> for LinerHashTable<T> {
    fn add(&mut self, x: T) -> bool {
        true
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
