use {
    rapidhash::RapidHasher,
    std::hash::{Hash, Hasher},
};

/// Hasher used to hash both nodes and keys.
pub trait NodeHasher {
    fn hash<K: Hash>(&self, key: &K) -> u64;
}

/// Default hasher used in the library.
pub struct DefaultNodeHasher;

impl NodeHasher for DefaultNodeHasher {
    fn hash<K: Hash>(&self, key: &K) -> u64 {
        let mut hasher = RapidHasher::default();
        key.hash(&mut hasher);
        hasher.finish()
    }
}

#[inline]
pub(crate) fn merge(a: &u64, b: &u64) -> u64 {
    let mut distance = *a ^ *b;
    distance ^= distance >> 33;
    distance = distance.wrapping_mul(0xff51_afd7_ed55_8ccd);
    distance ^= distance >> 33;
    distance = distance.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    distance ^= distance >> 33;
    distance
}
