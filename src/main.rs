use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
};

use structs::tries::{
    binary_trie::{BinaryTrie, ToUsize},
    x_fast_trie::XFastTrie,
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
    let mut t = BinaryTrie::new(32);
    let mut a = move || {
        for i in 0..1 {
            t.add(i);
        }
        t.find(9999);
    };
    //let mut b = move || {
    //for i in 0..10000 {
    //x.add(i);
    //}
    //x.find(&999999);
    //};
    calc_time!(a);
    //calc_time!(b);
}
