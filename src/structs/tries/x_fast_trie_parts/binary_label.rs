use crate::structs::tries::binary_trie::Binary;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct BinaryLabel {
    label: Vec<Binary>,
    max_depth: usize,
}
impl BinaryLabel {
    pub fn new(max_depth: usize, label_depth: usize, usized_data: usize) -> Self {
        let mut v = Vec::new();
        for depth in 0..label_depth {
            v.push(Binary::calc_binary(usized_data, max_depth - depth))
        }
        Self {
            max_depth,
            label: v,
        }
    }
    pub fn is_same(&self, label_depth: usize, usized_data: usize) -> bool {
        for depth in 0..label_depth {
            if self.label[depth] != Binary::calc_binary(usized_data, self.max_depth - depth) {
                return false;
            }
        }
        true
    }
}
mod binary_label_test {
    use super::*;
    #[test]
    fn new_test() {
        let max_depth = 4;
        let label_depth = 2;
        let usized_data = 1;
        let binary_label = BinaryLabel::new(max_depth, label_depth, usized_data);
        let tobe = BinaryLabel {
            label: vec![Binary::Zero, Binary::Zero],
            max_depth,
        };
        assert_eq!(binary_label, tobe);
    }
    #[test]
    fn is_same_test() {
        let max_depth = 4;
        let label_depth = 2;
        let usized_data = 1;
        let binary_label = BinaryLabel::new(max_depth, label_depth, usized_data);
        assert!(binary_label.is_same(label_depth, usized_data));
        assert!(binary_label.is_same(label_depth, 2));
        assert!(binary_label.is_same(label_depth, 3));
        assert!(!binary_label.is_same(label_depth, 5));

        let max_depth = 4;
        let label_depth = 3;
        let usized_data = 1;
        let binary_label = BinaryLabel::new(max_depth, label_depth, usized_data);
        let tobe = BinaryLabel {
            label: vec![Binary::Zero, Binary::Zero, Binary::Zero],
            max_depth,
        };
        assert_eq!(binary_label, tobe);
        assert!(binary_label.is_same(label_depth, usized_data));

        let max_depth = 4;
        let label_depth = 3;
        let usized_data = 15;
        let binary_label = BinaryLabel::new(max_depth, label_depth, usized_data);
        let tobe = BinaryLabel {
            label: vec![Binary::One, Binary::One, Binary::One],
            max_depth,
        };
        assert_eq!(binary_label, tobe);
        assert!(binary_label.is_same(label_depth, usized_data));
    }
}
