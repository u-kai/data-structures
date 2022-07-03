use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Ref, RefCell},
    rc::{Rc, Weak},
};

use structs::linked_lists::{dl_list::DLList, sl_list::SLList};

use crate::interfaces::stack::Stack;

mod interfaces;
mod structs;
mod types;

fn main() {
    let mut list = DLList::new();
    list.add(0, "*****");
    list.add(1, "hello");
    list.add(2, "world");
    list.add(3, "*****");
    println!("{:?}", list);
    println!("{:?}", list.get(0));
    println!("{:?}", list.get(1));
    println!("{:?}", list.get(2));
    println!("{:?}", list.get(3));
    println!("{:?}", list.get(4));
}
