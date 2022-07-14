use crate::interfaces::graph::Graph;

pub struct AdjacencyMatrix {
    n: usize,
    matrix: Vec<Vec<bool>>,
}

impl AdjacencyMatrix {
    pub fn new(n: usize) -> Self {
        AdjacencyMatrix {
            n,
            matrix: vec![vec![false; n]; n],
        }
    }
    fn is_over_len(&self, i: usize, j: usize) -> bool {
        self.n < i || self.n < j
    }
    fn len_check(&self, i: usize, j: usize) -> () {
        if self.is_over_len(i, j) {
            panic!("out of index")
        }
    }
    fn i_check(&self, i: usize) -> () {
        if self.n < i {
            panic!("out of index")
        }
    }
}

impl Graph for AdjacencyMatrix {
    fn add_edge(&mut self, i: usize, j: usize) {
        self.matrix[i][j] = true
    }
    fn has_edge(&self, i: usize, j: usize) -> bool {
        self.len_check(i, j);
        let column = self.matrix.get(i).unwrap();
        *column.get(j).unwrap()
    }
    fn in_edges(&self, i: usize) -> Vec<usize> {
        self.i_check(i);
        self.matrix
            .get(i)
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_, b)| **b)
            .map(|(u, _)| u)
            .collect::<Vec<_>>()
    }
    fn out_edges(&self, i: usize) -> Vec<usize> {
        vec![]
    }
    fn remove_edge(&mut self, i: usize, j: usize) {
        self.len_check(i, j);
        self.matrix[i][j] = false
    }
}
#[cfg(test)]
mod adjacency_matrix_test {
    use super::*;
    #[test]
    fn test() {
        let mut am = AdjacencyMatrix::new(3);
        am.add_edge(0, 1);
        am.add_edge(0, 2);
        assert_eq!(am.in_edges(0), vec![1, 2]);
        assert_eq!(am.in_edges(2), vec![]);
        am.remove_edge(0, 1);
        assert_eq!(am.in_edges(0), vec![2]);
    }
}
