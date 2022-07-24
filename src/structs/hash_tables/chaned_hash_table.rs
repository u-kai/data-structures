use std::{fmt::Debug, hash::Hash};

use crate::{interfaces::uset::USet, structs::arrays::array_stack::ArrayStack};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChanedHashTable<T: Clone + Hash + Eq + PartialEq + Debug + Default> {
    array: Box<[Option<ArrayStack<T>>]>,
    n: usize,
}

impl<T: Clone + Hash + Eq + PartialEq + Debug + Default> ChanedHashTable<T> {
    pub fn new() -> Self {
        Self {
            array: Box::new([]),
            n: 0,
        }
    }
    fn resize(&mut self) {
        let expand_array_len = (self.n * 2).max(1);
        let v = vec![None; expand_array_len];
        let mut new_array = v.into_boxed_slice();
        for i in 0..self.n {
            new_array[i] = self.array[i].take();
        }
        self.array = new_array
    }
    fn hash(x: T) -> usize {
        0
    }
    fn set(&mut self, i: usize, x: T) {
        if let Some(array) = self.array.get_mut(i).unwrap() {
            let len = array.size();
            array.add(len, x)
        } else {
            let mut array = ArrayStack::new();
            array.add(0, x);
            *self.array.get_mut(i).unwrap() = Some(array)
        }
    }
}

impl<T: Clone + Hash + Eq + PartialEq + Debug + Default> USet<T> for ChanedHashTable<T> {
    fn add(&mut self, x: T) -> bool {
        if self.find(x.clone()) {
            return false;
        }
        if (self.n + 1) > self.array.len() {
            self.resize();
        }
        self.set(Self::hash(x.clone()), x);
        self.n += 1;
        true
    }

    fn remove(&mut self, x: T) -> Option<T> {
        let j = Self::hash(x.clone());
        if self.array.get(j).unwrap().is_none() {
            return None;
        }
        let list_len = self.array.get(j).as_ref().unwrap().as_ref().unwrap().size();
        for i in 0..list_len {
            let y = self
                .array
                .get(j)
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .get(i)
                .unwrap();
            if x == y {
                self.array
                    .get_mut(j)
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .remove(i);
                self.n -= 1;
                return Some(y);
            }
        }
        None
    }
    fn size(&self) -> usize {
        self.n
    }
    fn find(&self, x: T) -> bool {
        let j = Self::hash(x.clone());
        if self.array.get(j).is_none() {
            return false;
        }
        if self.array.get(j).unwrap().is_none() {
            return false;
        }
        let list_len = self.array.get(j).as_ref().unwrap().as_ref().unwrap().size();
        for i in 0..list_len {
            let y = self
                .array
                .get(j)
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .get(i)
                .unwrap();
            if x == y {
                return true;
            };
        }
        false
    }
}

#[cfg(test)]
mod chaned_hash_table_test {

    use super::*;

    #[test]
    fn test() {
        let mut hash_table = ChanedHashTable::new();
        hash_table.add(0);
        hash_table.add(1);
        hash_table.add(2);
        hash_table.add(3);
        hash_table.add(4);
        hash_table.add(5);
        println!("{:?}", hash_table);
        assert_eq!(hash_table.remove(5), Some(5));
        assert_eq!(hash_table.find(0), true);
        assert_eq!(hash_table.find(1), true);
        assert_eq!(hash_table.find(2), true);
        assert_eq!(hash_table.find(3), true);
        assert_eq!(hash_table.find(10), false);
    }
}
