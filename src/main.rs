use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read, Write},
    os::unix::thread,
};

use rand::{thread_rng, Fill, Rng};
use structs::{
    binary_tree::binary_easy::BinarySearchTree,
    external_memory::btree::BTree,
    tries::{
        binary_trie::{BinaryTrie, ToUsize},
        x_fast_trie::XFastTrie,
    },
};

mod interfaces;

mod macros;

mod structs;
mod types;

impl ToUsize for i32 {
    fn to_usize(&self) -> usize {
        self.clone() as usize
    }
}
fn main() {
    let mut tree = BTree::new();
    for i in 0..31 {
        tree.add(i);
    }
    println!("{:#?}", tree);
}
