#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod hasher;
mod hrw;

use std::hash::Hash;

pub use {hasher::DefaultHasher, hrw::HrwNodes};

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

macro_rules! impl_hrwnode {
    ($($t:ty),*) => {
        $(impl HrwNode for $t {})*
    };
}

impl_hrwnode!(
    u8,
    u16,
    u32,
    u64,
    usize,
    i8,
    i16,
    i32,
    i64,
    isize,
    char,
    String,
    &str,
    &[u8]
);
