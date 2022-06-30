use structs::linked_lists::sl_list::SLList;

use crate::interfaces::stack::Stack;

mod interfaces;
mod structs;
mod types;
fn main() {
    let mut data = SLList::new();
    data.push(1);
    println!("{:?}", data);
    data.push(2);
    println!("{:?}", data);
    //data.push(3);
    //println!("{:?}", data);
    //data.pop();
    //println!("{:?}", data);
}
