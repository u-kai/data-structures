use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read, Write},
    os::unix::thread,
};

use rand::{thread_rng, Fill, Rng};
use structs::{
    binary_tree::binary_easy::BinarySearchTree,
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
    let mut rng = thread_rng();
    let mut array = [0i8; 256];
    rng.fill(&mut array);
    let mut tree = BinarySearchTree::new();
    for i in &array {
        tree.add(*i);
    }
    let find_data = 127i8;
    let array_timer = || array.contains(&find_data);
    let tree_timer = || tree.find(find_data);
    calc_time!(array_timer);
    calc_time!(tree_timer);
}
