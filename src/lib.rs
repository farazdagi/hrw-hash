#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod hasher;
mod hrw;

use std::hash::Hash;

pub use {
    hasher::{DefaultNodeHasher, NodeHasher},
    hrw::HrwNodes,
};

/// Target node which will be used for the hashing.
pub trait HrwNode: Hash + PartialEq + Eq {
    /// Capacity of the node.
    ///
    /// The capacity
    /// what portion of the keyspace the affects the score of the node, thus the
    /// higher the capacity, the more likely the node will be chosen.
    ///
    /// Capacities of all nodes are summed up to determine the total capacity of
    /// the keyspace. The relative capacity of the node is then ratio of the
    /// node's capacity to the total capacity of the keyspace.
    fn capacity(&self) -> usize {
        1
    }
}
