use structs::binary_tree::binary_tree::BinaryTree;

mod interfaces;
mod structs;
mod types;

fn main() {
    let mut tree = BinaryTree::new(7);
    tree.add(3);
    tree.add(1);
    tree.add(5);
    tree.add(4);
    tree.add(6);
    tree.add(11);
    tree.add(9);
    tree.add(8);
    tree.add(13);
    tree.add(12);
    tree.add(14);
    tree.remove(11);
    tree.remove(1);
    tree.remove(9);
    println!("{:#?}", tree);
}
