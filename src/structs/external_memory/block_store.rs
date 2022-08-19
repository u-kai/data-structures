pub trait BlockStore<T> {
    fn read_block(&self, i: usize) -> &T;
    fn write_block(&mut self, i: usize, b: T) -> ();
    fn place_block(&mut self, b: T) -> usize;
    fn free_block(&mut self, i: usize) -> ();
}

pub struct BTree {}

struct Node<T> {
    x: T,
    children: Box<Vec<Node<T>>>,
    keys: Vec<Option<usize>>,
}
