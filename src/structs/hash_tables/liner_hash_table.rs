use std::fmt::Debug;

use crate::{interfaces::uset::USet, structs::arrays::array_stack::ArrayStack};

use super::hash_gen::HashGen;

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

#[derive(Debug)]
pub struct LinerHashTable<T: Clone + Debug + Eq + PartialEq + Default + ToString> {
    array: ArrayStack<DataState<T>>,
    hashgen: HashGen,
    n: usize,
    q: usize,
    d: u32,
}

impl<T: Clone + Debug + Eq + PartialEq + Default + ToString> LinerHashTable<T> {
    pub fn new() -> Self {
        let d = 8;
        let len = 2_i32.pow(d) as usize;
        let array = ArrayStack::new_with_default(len);
        Self {
            d,
            array,
            hashgen: HashGen::new(33, 32, 8),
            q: 0,
            n: 0,
        }
    }
    fn resize(&mut self) {
        let mut d = 1;
        //while (1 << d) < 3 * self.n {
        //d += 1;
        //}
        //let mut new_array = ArrayStack::new_with_default(1 << d);

        //        for k in 0..self.array.size() {
        //if let Some(data) = self.array.get(k){
        //match data {
        //DataState::Exist(_)=>{

        //}
        //}
        //}
        //}
    }
}
impl<T: Clone + Debug + Eq + PartialEq + Default + ToString> USet<T> for LinerHashTable<T> {
    fn add(&mut self, x: T) -> bool {
        if self.find(x.clone()) {
            return false;
        }
        if 2 * (self.q + 1) > self.array.size() {
            self.resize()
        }
        let mut i = self.hashgen.hash(x.clone());
        if let Some(mut data) = self.array.get(i) {
            loop {
                match data {
                    DataState::Exist(_) => {
                        i = if i == (self.array.size() - 1) {
                            0
                        } else {
                            i + 1
                        };
                        if let Some(new_data) = self.array.get(i) {
                            data = new_data
                        } else {
                            return false;
                        }
                    }
                    DataState::Del => {
                        self.array.set(i, DataState::Exist(x.clone()));
                        self.n += 1;
                        return true;
                    }
                    DataState::Null => {
                        self.array.set(i, DataState::Exist(x.clone()));
                        self.n += 1;
                        self.q += 1;
                        return true;
                    }
                }
            }
        } else {
            false
        }
    }
    fn find(&self, x: T) -> bool {
        let mut i = self.hashgen.hash(x.clone());
        if let Some(mut data) = self.array.get(i) {
            loop {
                match &data {
                    DataState::Null => return false,
                    DataState::Del => {
                        i = if i == (self.array.size() - 1) {
                            0
                        } else {
                            i + 1
                        };
                        if let Some(new_data) = self.array.get(i) {
                            data = new_data
                        } else {
                            return false;
                        }
                    }
                    DataState::Exist(y) => {
                        if y == &x {
                            return true;
                        }
                        i = if i == (self.array.size() - 1) {
                            0
                        } else {
                            i + 1
                        };
                        if let Some(new_data) = self.array.get(i) {
                            data = new_data
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
        false
    }
    fn remove(&mut self, x: T) -> Option<T> {
        let mut i = self.hashgen.hash(x.clone());
        if let Some(mut data) = self.array.get(i) {
            loop {
                match &data {
                    DataState::Null => return None,
                    DataState::Del => {
                        i = if i == (self.array.size() - 1) {
                            0
                        } else {
                            i + 1
                        };
                        if let Some(new_data) = self.array.get(i) {
                            data = new_data
                        } else {
                            return None;
                        }
                    }
                    DataState::Exist(y) => {
                        if y == &x {
                            self.array.set(i, DataState::Del);
                            self.n -= 1;
                            return Some(x);
                        }
                        i = if i == (self.array.size() - 1) {
                            0
                        } else {
                            i + 1
                        };
                        if let Some(new_data) = self.array.get(i) {
                            data = new_data
                        } else {
                            return None;
                        }
                    }
                }
            }
        }
        None
    }
    fn size(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
mod liner_hash_table_test {
    use super::*;
    #[test]
    fn test() {
        let mut liner_hash_table = LinerHashTable::new();
        liner_hash_table.add(0);
        liner_hash_table.add(1);
        assert!(liner_hash_table.find(0));
        assert!(liner_hash_table.find(1));
        liner_hash_table.add(2);
        liner_hash_table.add(3);
        liner_hash_table.add(4);
        liner_hash_table.add(5);
        liner_hash_table.add(6);
        liner_hash_table.add(7);
        assert!(liner_hash_table.find(7));
        assert!(!liner_hash_table.find(8));
        assert_eq!(liner_hash_table.remove(1), Some(1));
        assert_eq!(liner_hash_table.remove(10), None);
    }
}
