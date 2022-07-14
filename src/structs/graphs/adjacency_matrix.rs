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
    fn len_check(&self, i: usize, j: usize) -> bool {
        self.n < i || self.n < j
    }
}

impl Graph for AdjacencyMatrix {
    fn add_edge(&mut self, i: usize, j: usize) {
        if self.len_check(i, j) {
            panic!("out of index")
        }
        self.matrix[i][j] = true
    }
    fn has_edge(&self, i: usize, j: usize) -> bool {
        if self.len_check(i, j) {
            panic!("out of index")
        }
        let column = self.matrix.get(i).unwrap();
        *column.get(j).unwrap()
    }
    fn in_edges(&self, i: usize) -> Vec<usize> {
        if self.n < i {
            panic!("out of index")
        }
        self.matrix
            .get(i)
            .map(|v| {
                v.iter()
                    .enumerate()
                    .filter(|(_, b)| **b)
                    .map(|(u, _)| u)
                    .collect::<Vec<_>>()
            })
            .unwrap()
    }
    fn out_edges(&self, i: usize) -> Vec<usize> {
        vec![]
    }
    fn remove_edge(&mut self, i: usize, j: usize) {}
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
    }
}
