use {
    crate::{DefaultNodeHasher, Node, NodeHasher, hasher::merge},
    std::{collections::HashMap, hash::Hash},
};

/// Nodes sorted using the HRW algorithm.
pub struct HrwNodes<N, H = DefaultNodeHasher> {
    nodes: HashMap<N, u64>,
    hasher: H,
}

impl<N: Node> HrwNodes<N> {
    pub fn new<I>(nodes: I) -> Self
    where
        I: IntoIterator<Item = N>,
    {
        Self::with_hasher(DefaultNodeHasher {}, nodes)
    }
}

impl<N, H> HrwNodes<N, H>
where
    N: Node,
    H: NodeHasher,
{
    pub fn with_hasher<I>(hasher: H, nodes: I) -> Self
    where
        I: IntoIterator<Item = N>,
    {
        Self {
            nodes: nodes
                .into_iter()
                .map(|node| {
                    let hash = hasher.hash(&node);
                    (node, hash)
                })
                .collect(),
            hasher,
        }
    }

    pub fn sorted<K: Hash>(&self, key: &K) -> impl Iterator<Item = &N> {
        let key_hash = &self.hasher.hash(key);
        let mut nodes = self
            .nodes
            .iter()
            .map(|(node, node_hash)| (merge(node_hash, key_hash), node))
            .collect::<Vec<_>>();

        nodes.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        nodes.into_iter().map(|(_, node)| node)
    }
}
