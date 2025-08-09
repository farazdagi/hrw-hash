use {rapidhash::v3::rapidhash_v3, std::hash::Hasher};

/// Default hasher used in the library.
///
/// Relies on the `rapidhash`, which is a portable and fast hashing.
/// Additionally, it is designed to stay stable across different platforms, Rust
/// versions, and package releases --- thus, it is safe to assume that the
/// key-hash pairs will not change over time, and the same key will always
/// hash to the same value.
#[derive(Default)]
pub struct DefaultHasher(Vec<u8>);

impl Hasher for DefaultHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }

    fn finish(&self) -> u64 {
        rapidhash_v3(&self.0)
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
