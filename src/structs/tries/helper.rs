use std::fmt::Debug;

use super::{
    binary_trie::{BinaryTrie, ToUsize},
    nodes::strong_link::StrongLinkNode,
};

pub fn check_prev_next<T: ToUsize + Clone + PartialEq + Debug>(tree: BinaryTrie<T>) {
    let mut next = tree.min_prev.clone();
    println!("from prev");
    println!();
    while next.is_some() {
        println!("next = {:?}", next.value());
        next = next.next();
    }
    let mut prev = tree.max_next.clone();
    println!("from next");
    println!();
    while prev.is_some() {
        println!("prev = {:?}", prev.value());
        prev = prev.prev();
    }
}
pub fn rec_assert<T: ToUsize + Clone + PartialEq + Debug>(
    name: String,
    node: StrongLinkNode<T>,
    other: StrongLinkNode<T>,
) {
    println!();
    let s = name;
    println!("{}", s);
    println!(
        "value : self = {:?}, other = {:?}",
        node.value(),
        other.value()
    );
    println!(
        "parent : self = {:?}, other = {:?}",
        node.parent().is_some(),
        other.parent().is_some()
    );
    assert_eq!(node.parent().is_some(), other.parent().is_some());

    println!(
        "left : self = {:?}  other = {:?}",
        node.left().is_some(),
        other.left().is_some()
    );
    println!(
        "right : self = {:?}  other = {:?}",
        node.right().is_some(),
        other.right().is_some()
    );
    assert_eq!(node.left().is_some(), other.left().is_some());
    assert_eq!(node.right().is_some(), other.right().is_some());

    println!(
        "prev : self = {:?} other = {:?}",
        node.prev().value(),
        other.prev().value()
    );
    assert_eq!(node.prev().value(), other.prev().value());

    println!(
        "next : self = {:?} other = {:?} ",
        node.next().value(),
        other.next().value()
    );
    assert_eq!(node.next().value(), other.next().value());

    println!(
        "jump self = {:?} other = {:?}",
        node.jump().value(),
        other.jump().value()
    );
    assert_eq!(node.jump().value(), other.jump().value());

    if node.left().is_some() {
        rec_assert(format!("{}-left", s), node.left(), other.left());
    }
    if node.right().is_some() {
        rec_assert(format!("{}-right", s), node.right(), other.right());
    }
}
