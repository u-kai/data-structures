use std::fmt::Debug;

pub trait USet<T: Clone + Debug + PartialEq + Eq> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: T) -> Option<T>;
    fn find(&self, x: T) -> bool;
}
