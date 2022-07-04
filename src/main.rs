use structs::linked_lists::dl_list::DLList;

use crate::interfaces::list::List;
mod interfaces;
mod structs;
mod types;

fn main() {
    let mut list = DLList::new();
    list.add(0, "*****");
    list.add(1, "hello");
    list.add(2, "world");
    list.add(3, "*****");
    println!("{:?}", list.get(0));
    println!("{:?}", list.get(1));
    println!("{:?}", list.get(2));
    println!("{:?}", list.get(3));
    println!("{:?}", list.get(0));
    println!("{:?}", list.get(1));
    println!("{:?}", list.get(2));
    println!("{:?}", list.get(3));
    println!("{:?}", list.get(0));
    println!("{:?}", list.get(1));
    println!("{:?}", list.get(2));
    println!("{:?}", list.get(3));
    list.remove(3);
    list.set(0, "#####");
    println!("{:?}", list);
    println!("{:?}", list.get(0));
    println!("{:?}", list.get(1));
    println!("{:?}", list.get(2));
    println!("{:?}", list.get(3));
}
