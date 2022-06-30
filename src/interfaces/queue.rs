use std::fmt::Debug;

pub trait Queue<T: Clone + Debug + Eq + PartialEq> {
    fn add(&mut self, x: T) -> ();
    fn remove(&mut self) -> Option<T>;
}
