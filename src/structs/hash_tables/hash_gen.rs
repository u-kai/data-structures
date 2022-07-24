use std::{fmt::Debug, ops::Div};

pub struct HashGen {
    z: usize,
    w: u32,
    d: u32,
}

impl HashGen {
    pub fn new(z: usize, w: u32, d: u32) -> Self {
        Self { z, w, d }
    }
    pub fn from_usize(&self, x: usize) -> usize {
        ((self.z * x) % (2_i64.pow(self.w as u32) as usize))
            .div(2_i32.pow((self.w - self.d) as u32) as usize)
    }
    pub fn hash<T: Clone + Debug + ToString>(&self, x: T) -> usize {
        let str = x.to_string();
        match str.parse::<usize>() {
            Ok(num) => self.from_usize(num),
            Err(_) => {
                let bytes = str.bytes().len();
                self.from_usize(bytes)
            }
        }
    }
}

#[cfg(test)]
mod hash_gen_test {
    use super::HashGen;

    #[test]
    fn test() {
        let hash_gen = HashGen::new(4102541685, 32, 8);
        assert_eq!(hash_gen.from_usize(42), 30);
    }
}
