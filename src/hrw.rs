use {
    crate::{DefaultNodeHasher, HrwNode, NodeHasher, hasher::merge},
    std::{cmp, collections::HashMap, hash::Hash},
};

/// Nodes sorted using the HRW algorithm.
pub struct HrwNodes<N, H = DefaultNodeHasher> {
    nodes: HashMap<N, u64>,
    hasher: H,
    total_capacity: usize,
}

impl<N: HrwNode> HrwNodes<N> {
    /// Create a new instance with the default hasher.
    pub fn new<I>(nodes: I) -> Self
    where
        I: IntoIterator<Item = N>,
    {
        Self::with_hasher(DefaultNodeHasher {}, nodes)
    }
}

impl<N, H> HrwNodes<N, H>
where
    N: HrwNode,
    H: NodeHasher,
{
    /// Create a new instance with a custom hasher.
    pub fn with_hasher<I>(hasher: H, nodes: I) -> Self
    where
        I: IntoIterator<Item = N>,
    {
        let mut total_capacity = 0;
        let nodes = nodes
            .into_iter()
            .map(|node| {
                let hash = hasher.hash(&node);
                total_capacity += node.capacity();
                (node, hash)
            })
            .collect();

        Self {
            nodes,
            hasher,
            total_capacity,
        }
    }

    /// Sort the nodes using the HRW algorithm.
    pub fn sorted<K: Hash>(&self, key: &K) -> impl Iterator<Item = &N> {
        let key_hash = &self.hasher.hash(key);
        let mut nodes = self
            .nodes
            .iter()
            .map(|(node, node_hash)| {
                let weight = node.capacity() as f64 / self.total_capacity as f64;
                let hash = merge(node_hash, key_hash) as f64 / u64::MAX as f64;
                let score = Score((1.0 / -hash.ln()) * weight);
                (score, node)
            })
            .collect::<Vec<_>>();

        nodes.sort_unstable_by(|a, b| a.0.cmp(&b.0).reverse());
        nodes.into_iter().map(|(_, node)| node)
    }
}

/// Score as positive floating point number.
///
/// Makes it easier to sort the nodes by score.
#[derive(Clone, Copy, PartialEq)]
struct Score(f64);

impl Eq for Score {}

impl cmp::PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Score {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or_else(|| panic!("Cannot compare scores: {} and {}", self.0, other.0))
    }
}
