use structs::tries::binary_trie::{BinaryTrie, ToUsize};

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
    let mut tree = BinaryTrie::new(4);
    tree.add(1);
    tree.find(1);
    tree.remove(1);
}
