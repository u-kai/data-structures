use std::fmt::Debug;

use super::helper::{build_B2_none, B};

pub trait BlockStore<T> {
    fn read_block(&self, i: usize) -> &T;
    fn write_block(&mut self, i: usize, b: T) -> ();
    fn place_block(&mut self, b: T) -> usize;
    fn free_block(&mut self, i: usize) -> ();
}

type BIndex = usize;
pub struct BTree {
    root_index: BIndex,
}

struct NodeList<T> {
    block_list: Vec<Node<T>>,
    free_list: Vec<BIndex>,
}

struct Node<T> {
    keys: [Option<T>; 2 * B],
    children: [Option<BIndex>; 2 * B],
}
impl<T> Node<T>
where
    T: Clone + Debug + PartialEq + PartialOrd + Ord + Default,
{
    fn new() -> Self {
        Self {
            keys: build_B2_none::<T>(),
            children: build_B2_none::<BIndex>(),
        }
    }
}

//macro_rules! allocate_array {
//($elem:ident,$num:literal) => {
//let mut v = vec![$elem; $num];
//v.into_boxed_slice()
//};
//}

#[cfg(test)]
mod block_store_test {
    #[test]
    fn allocate_array_test() {
        //#[derive(Default, PartialEq, Eq, Debug)]
        //struct Wrap(String);
        //let array: [Option<Wrap>; 8] = [None; 8];
        ////let array: [Option<String>; 8] = vec![None; 8].;
        //let tobe: [Option<Wrap>; 8] = [None, None, None, None, None, None, None, None];
        //assert_eq!(array, tobe);
    }
}
