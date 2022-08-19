use std::fmt::Debug;

use super::btree::BIndex;
#[derive(Debug, PartialEq, Eq)]
pub(super) struct BlockStore<T> {
    pub block_list: Vec<T>,
    pub free_list: Vec<BIndex>,
}
impl<T> BlockStore<T> {
    pub fn new() -> Self {
        Self {
            block_list: Vec::new(),
            free_list: Vec::new(),
        }
    }
    pub fn read_block(&mut self, i: usize) -> Option<&T> {
        self.block_list.get(i)
    }
    //pub fn update_block(&mut self, i: usize, b: &T) {
    //*self.block_list.get_mut(i).unwrap() = b;
    //}
    pub fn write_block(&mut self, i: usize, b: T) {
        match self.block_list.get_mut(i) {
            Some(block) => *block = b,
            None => self.block_list.push(b),
        };
    }
    pub fn place_block(&mut self, b: T) -> Option<usize> {
        None
    }
    pub fn free_block(&mut self, i: usize) {}
    pub fn block_list_len(&self) -> usize {
        self.block_list.len()
    }
}
