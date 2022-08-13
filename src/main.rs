use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

mod interfaces;
mod structs;
mod types;
trait OffTreadExt: Iterator {
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}
impl<T> OffTreadExt for T
where
    T: Iterator + Send + 'static,
    T::Item: Send + 'static,
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        let (s, r) = mpsc::sync_channel(1);
        thread::spawn(move || {
            for item in self {
                if s.send(item).is_err() {
                    break;
                }
            }
        });
        r.into_iter()
    }
}

fn lock(data: &Mutex<i32>) -> i32 {
    println!("hello");
    let data = data.lock();
    data.unwrap().clone()
}
fn world(data: &Mutex<i32>) -> i32 {
    println!("world");
    let data = data.lock();
    data.unwrap().clone()
}
fn main() {
    let data = Mutex::new(5);
    {
        let _ = &data.lock();
    }
    let _ = &data.lock();
}
