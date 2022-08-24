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
    for i in 0..10 {
        tree.add(i);
    }
    //let mut file = File::create("tree.txt").unwrap();

    let string = tree.to_string();

    fn string_conect(s1: String, s2: String) -> String {
        let mut result = String::new();
        let s1_lines = s1.lines().collect::<Vec<_>>();
        let s2_lines = s2.lines().collect::<Vec<_>>();
        let max_len = s1_lines.len().max(s2_lines.len());
        for i in 0..max_len {
            let s1 = s1_lines.get(i).unwrap_or(&"");
            let s2 = s2_lines.get(i).unwrap_or(&"");
            result = format!("{}{}{}\n", result, s1, s2,)
        }
        result
    }
    let s1 = " 0 \n 1 \n".to_string();
    let s2 = " 1 \n 3 \n".to_string();
    println!("{}", string)

    //let _ = file.write(&mut string.as_bytes());
    //println!("{:#?}", tree);
}
fn indent(mut s1: String, mut s2: String) -> String {
    let s1_len = s1.len();
    let s2_len = s2.len();
    let max_len = s1_len.max(s2_len);
    for i in 0..=max_len {
        if s1_len < i {
            s1.push(' ')
        }
        if s2_len < i {
            s2.push(' ')
        }
    }
    format!("{}\n{}", s1, s2)
}

#[test]
fn test_indent() {
    let s1 = "hello".to_string();
    let s2 = "world!!".to_string();
    assert_eq!(
        indent(s1, s2),
        "hello  
world!!"
    )
}
