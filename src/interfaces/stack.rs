use std::fmt::Debug;

pub trait Stack<T: Clone + PartialEq + Eq + Debug> {
    fn push(&mut self, x: T) -> ();
    fn pop(&mut self) -> Option<T>;
}
