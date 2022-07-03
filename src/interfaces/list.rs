use std::fmt::Debug;

pub trait List<T: Clone + Default + Debug> {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> Option<T>;
    fn set(&mut self, i: usize, x: T) -> ();
    fn add(&mut self, i: usize, x: T) -> ();
    fn remove(&mut self, i: usize) -> Option<T>;
}
