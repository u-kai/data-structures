pub struct BinaryTrie {}

struct Node {
    x: u8,
    right: Box<Node>,
    left: Box<Node>,
}
