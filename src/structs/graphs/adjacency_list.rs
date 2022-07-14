use crate::{
    interfaces::{graph::Graph, queue::Queue, stack::Stack},
    structs::linked_lists::sl_list::SLList,
};

#[derive(Debug)]
pub struct AdjacencyList {
    n: usize,
    adj: Vec<Vec<usize>>,
}

impl AdjacencyList {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![vec![]; n],
        }
    }
    pub fn can_reach(&self, start: usize, target: usize) -> bool {
        if start == target {
            return true;
        }
        let mut queue = SLList::new_with(vec![start]);
        let mut seen = vec![false; self.n];
        while queue.size() > 0 {
            let i = queue.remove().unwrap();
            let i_adjacencies = self.out_edges(i);
            for adjacency in i_adjacencies {
                if adjacency == target {
                    return true;
                }
                if !seen[adjacency] {
                    queue.push(adjacency);
                    seen[adjacency] = true;
                }
            }
        }
        false
    }
    fn len_check(&self, i: usize) {
        if i > self.n - 1 {
            panic!("outbound")
        }
    }
}

impl Graph for AdjacencyList {
    fn add_edge(&mut self, i: usize, j: usize) {
        self.len_check(i);
        self.adj[i].push(j)
    }
    fn has_edge(&self, i: usize, j: usize) -> bool {
        self.adj[i].contains(&j)
    }
    fn in_edges(&self, i: usize) -> Vec<usize> {
        self.len_check(i);
        self.adj
            .iter()
            .enumerate()
            .filter(|(_j, list)| list.contains(&i))
            .map(|(j, _list)| j)
            .collect()
    }
    fn out_edges(&self, i: usize) -> Vec<usize> {
        self.len_check(i);
        self.adj[i].clone()
    }
    fn remove_edge(&mut self, i: usize, j: usize) {
        self.len_check(i);
        if let Some(remove_index) = self.adj[i].iter().position(|data| data == &j) {
            self.adj[i].remove(remove_index);
        };
    }
}

#[cfg(test)]
mod adjacency_list_test {
    use super::*;
    #[test]
    fn can_reach_test() {
        let mut al = AdjacencyList::new(6);
        al.add_edge(0, 1);
        al.add_edge(0, 2);
        al.add_edge(1, 3);
        al.add_edge(1, 4);
        al.add_edge(2, 5);
        assert!(al.can_reach(0, 5));
        assert!(!al.can_reach(5, 1));
        assert!(!al.can_reach(4, 0));
    }
    #[test]
    fn test() {
        let mut al = AdjacencyList::new(3);
        al.add_edge(0, 1);
        al.add_edge(0, 2);
        al.add_edge(2, 1);
        al.add_edge(1, 2);
        assert_eq!(al.out_edges(0), vec![1, 2]);
        assert_eq!(al.in_edges(1), vec![0, 2]);
        assert_eq!(al.in_edges(2), vec![0, 1]);
        assert_eq!(al.out_edges(2), vec![1]);
        assert!(al.has_edge(2, 1));
        al.remove_edge(0, 1);
        al.remove_edge(1, 2);
        assert_eq!(al.out_edges(0), vec![2]);
        assert!(!al.has_edge(1, 2));
    }
}
