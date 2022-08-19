pub mod arrays;
pub mod binary_tree;
pub mod external_memory {
    pub mod block_store;
    pub(super) mod helper;
}
pub mod graphs;
pub mod hash_tables;
pub mod heap;
pub mod linked_lists;
pub mod skip_lists;
pub mod tries {
    pub mod binary_trie;
    pub(self) mod helper;
    pub(super) mod nodes {
        pub(super) mod node;
        pub(super) mod strong_link;
        pub(super) mod weak_link;
    }
    pub mod x_fast_trie;
    pub(super) mod x_fast_trie_parts {
        pub(super) mod binary_label;
        pub(super) mod hash_table;
    }
}
