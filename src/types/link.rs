use std::{cell::RefCell, rc::Rc};

pub type StrongLink<T> = Rc<RefCell<T>>;
