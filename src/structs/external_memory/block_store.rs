use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use super::indexs::BIndex;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BlockStore<T: Clone> {
    pub block_list: Vec<Option<Block<T>>>,
    pub free_list: Vec<BIndex>,
}
impl<T: Clone> BlockStore<T> {
    pub fn new() -> Self {
        Self {
            block_list: Vec::new(),
            free_list: Vec::new(),
        }
    }
    pub fn read_block(&self, index: BIndex) -> Option<Block<T>> {
        if let Some(block) = self.block_list.get(*index) {
            return block.as_ref().map(|block| (*block).clone());
        }
        None
    }
    //pub fn read_mut_block(&mut self, index: BIndex) -> Option<&mut Block<T>> {
    //self.block_list.get_mut(*index)
    //}
    pub fn add_new_block(&mut self, data: T) {
        if self.free_list.is_empty() {
            let new_index = self.block_list.len();
            let block = Block::new(new_index.into(), data);
            self.block_list.push(Some(block));
            return;
        };
        let index = self.free_list.pop().unwrap();
        self.block_list[*index] = Some(Block::new(index, data));
    }
    pub fn update_block(&mut self, index: BIndex, data: T) {
        match self.block_list.get_mut(*index) {
            Some(block) => *block = Some(Block::new(index, data)),
            None => {
                self.place_data(data);
            }
        };
    }
    pub fn write_block(&mut self, block: Block<T>) {
        match self.block_list.get_mut(*block.index) {
            Some(old_block) => *old_block = Some(block),
            None => self.block_list.push(Some(block)),
        };
    }
    //pub fn update_block(&mut self, i: usize, b: T) {
    //*self.block_list.get_mut(i).unwrap() = Block::new();
    //}
    pub fn place_block(&mut self, data: T) -> BIndex {
        if self.free_list.is_empty() {
            let new_index = self.block_list.len();
            let block = Block::new(new_index.into(), data);
            self.block_list.push(Some(block));
            new_index.into()
        } else {
            let free_index = self.free_list.pop().unwrap();
            let block = Block::new(free_index.into(), data);
            self.block_list[*free_index] = Some(block);
            free_index.into()
        }
    }
    pub fn place_data(&mut self, b: T) -> BIndex {
        if self.free_list.is_empty() {
            let new_index = self.block_list.len();
            let block = Block::new(new_index.into(), b);
            self.block_list.push(Some(block));
            (self.block_list.len() - 1).into()
        } else {
            let index = self.free_list.pop().unwrap();
            self.block_list[*index] = Some(Block::new(index, b));
            index
        }
    }
    pub fn free_block(&mut self, i: BIndex) {
        self.block_list[*i] = None;
        self.free_list.push(i)
    }
    pub fn block_list_len(&self) -> usize {
        self.block_list.len()
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) struct Block<T> {
    index: BIndex,
    data: T,
}
impl<T> Block<T> {
    pub fn new(index: BIndex, data: T) -> Self {
        Self { index, data }
    }
    pub fn index(&self) -> BIndex {
        self.index
    }
    //pub fn provide_as_mut(&mut self) -> (BIndex, &mut T) {
    //(self.index, &mut self.data)
    //}
}

impl<T> Deref for Block<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Block<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
