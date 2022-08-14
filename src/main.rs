use structs::tries::binary_trie::{BinaryTrie, ToUsize};

mod interfaces;
mod structs;
mod types;
impl ToUsize for u32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
fn main() {
    let mut tree = BinaryTrie::<u32>::new(4);
    tree.add(3);
    assert_eq!(tree, tree);
}
