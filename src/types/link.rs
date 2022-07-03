use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type StrongLink<T> = Rc<RefCell<T>>;
pub type WeakLink<T> = Weak<RefCell<T>>;
