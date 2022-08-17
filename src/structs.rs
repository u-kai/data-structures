pub mod arrays;
pub mod binary_tree;
pub mod graphs;
pub mod hash_tables;
pub mod heap;
pub mod linked_lists;
pub mod skip_lists;
pub mod tries {
    pub mod binary_trie;
    pub(super) mod nodes {
        pub(super) mod node;
        pub(super) mod strong_link;
        pub(super) mod weak_link;
    }
    pub mod x_fast_trie;
}

pub mod tries_2 {
    pub(super) mod leaf;
    pub(super) mod node;
    pub mod x_fast_trie;
}
