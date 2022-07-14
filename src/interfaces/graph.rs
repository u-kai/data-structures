pub trait Graph {
    fn add_edge(&mut self, i: usize, j: usize);
    fn remove_edge(&mut self, i: usize, j: usize);
    fn has_edge(&self, i: usize, j: usize) -> bool;
    fn out_edges(&self, i: usize) -> Vec<usize>;
    fn in_edges(&self, i: usize) -> Vec<usize>;
}
