use std::fmt::Debug;

pub trait Queue<T: Clone + PartialEq + Eq + Debug> {
    fn add(&mut self, x: T) -> ();
    fn remove(&mut self) -> Option<T>;
}
