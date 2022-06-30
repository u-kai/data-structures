use std::fmt::Debug;

pub trait List<T: Clone + Debug + Eq + PartialEq> {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> Option<T>;
    fn set(&mut self, i: usize, x: T) -> ();
    fn add(&mut self, i: usize, x: T) -> ();
    fn remove(&mut self, i: usize) -> Result<T, String>;
}
