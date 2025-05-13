#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

use {
    rapidhash::RapidHasher,
    std::{
        collections::HashMap,
        fmt,
        hash::{Hash, Hasher},
    },
};

pub trait Node: fmt::Debug + Hash + PartialEq + Eq {}

impl<T> Node for T where T: fmt::Debug + Hash + PartialEq + Eq {}

pub trait WeightedNode: Node {
    /// Capacity of the node.
    ///
    /// The capacity
    /// what portion of the keyspace the affects the score of the node, thus the
    /// higher the capacity, the more likely the node will be chosen.
    ///
    /// Capacities of all nodes are summed up to determine the total capacity of
    /// the keyspace. The relative capacity of the node is then ratio of the
    /// node's capacity to the total capacity of the keyspace.
    fn capacity(&self) -> usize;
}

pub trait NodeHasher {
    fn hash_key<K: Hash>(&self, key: &K) -> u64;
}

pub struct DefaultNodeHasher;

impl NodeHasher for DefaultNodeHasher {
    fn hash_key<K: Hash>(&self, key: &K) -> u64 {
        let mut hasher = RapidHasher::default();
        key.hash(&mut hasher);
        hasher.finish()
    }
}

pub struct Hrw<N, H = DefaultNodeHasher> {
    nodes: HashMap<N, u64>,
    hasher: H,
}

impl<N: Node> Hrw<N> {
    pub fn new<I>(nodes: I) -> Self
    where
        I: IntoIterator<Item = N>,
    {
        Self::with_hasher(DefaultNodeHasher {}, nodes)
    }
}

impl<N, H> Hrw<N, H>
where
    N: Node,
    H: NodeHasher,
{
    pub fn with_hasher<I>(build_hasher: H, nodes: I) -> Self
    where
        I: IntoIterator<Item = N>,
    {
        Self {
            nodes: nodes
                .into_iter()
                .map(|node| {
                    // Pre-calculate node hashes (optimization described in
                    // https://www.npiontko.pro/2024/12/23/computation-efficient-rendezvous-hashing)
                    let hash = build_hasher.hash_key(&node);
                    (node, hash)
                })
                .collect(),
            hasher: build_hasher,
        }
    }

    pub fn sorted<K: Hash>(&self, key: &K) -> impl Iterator<Item = &N> {
        let key_hash = self.hasher.hash_key(key);
        let mut nodes = self
            .nodes
            .iter()
            .map(|(node, node_hash)| (merge(node_hash, &key_hash), node_hash, node))
            .collect::<Vec<_>>();

        nodes.sort_unstable_by(|a, b| {
            // Sort by the first element (the merged hash) in descending order
            // and then by the second element (the node hash) in ascending order.
            a.0.cmp(&b.0).reverse().then_with(|| a.1.cmp(&b.1))
        });
        nodes.into_iter().map(|(_, _, node)| node)
    }
}

#[inline]
fn merge(a: &u64, b: &u64) -> u64 {
    let mut distance = *a ^ *b;
    distance ^= distance >> 33;
    distance = distance.wrapping_mul(0xff51_afd7_ed55_8ccd);
    distance ^= distance >> 33;
    distance = distance.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    distance ^= distance >> 33;
    distance
}
