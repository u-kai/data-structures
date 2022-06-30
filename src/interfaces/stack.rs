use std::fmt::Debug;

pub trait Stack<T: Clone + Debug + Eq + PartialEq> {
    fn push(&mut self, x: T) -> ();
    fn pop(&mut self) -> Option<T>;
}
