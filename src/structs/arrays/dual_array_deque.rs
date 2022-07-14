use std::fmt::Debug;

use super::array_stack::ArrayStack;

pub struct DualArrayDeque<T: Clone + Default + Debug> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T: Clone + Default + Debug> DualArrayDeque<T> {
    pub fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }
}
