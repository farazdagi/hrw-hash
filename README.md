# hrw-hash

[![crates.io](https://img.shields.io/crates/d/hrw-hash.svg)](https://crates.io/crates/hrw-hash)
[![docs.rs](https://docs.rs/hrw-hash/badge.svg)](https://docs.rs/hrw-hash)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![dependencies](https://deps.rs/repo/github/farazdagi/hrw-hash/status.svg)](https://deps.rs/repo/github/farazdagi/mpchash)

A minimalistic implementation of the Highest Random Weight (HRW) aka Rendezvous hashing algorithm as
described in the
["A name-based mapping scheme for Rendezvous"](https://www.eecs.umich.edu/techreports/cse/96/CSE-TR-316-96.pdf),
by Thaler and Ravishankar (1996).

The weighted variant of the HRW algorithm is implemented using Logarithmic Method as described in
["Weighted distributed hash tables"](https://dl.acm.org/doi/10.1145/1073970.1074008), by
Schindelhauer and Schomaker (2005).

To constrain the number of hashing operations, the implementation hashes nodes and keys only once
(instead of `nodes * keys` hashes). This optimization idea is well presented in the
["Rendezvous Hashing: The Path to Faster Hashes Calculation"](https://www.npiontko.pro/2024/12/23/computation-efficient-rendezvous-hashing)
blog.

## Features

- [x] Absolutely minimalistic implementation with sane defaults.
- [ ] Allow weighted nodes.
- [ ] Allow massive number of nodes (`O(log(n))` lookup time, instead of `O(n)`).
- [x] Optimized for performance and memory usage. No wasted re-hashing.

## Usage

For non-weighted nodes:

``` rust
use hrw_hash::HrwNodes;

// Anything that implements `IntoIterator<Item = Hash + Eq>` can
// be used as list of target nodes.
let hrw = HrwNodes::new((0..10).map(|i| format!("node{}", i)));

// For a given key, get the iterator to node references
// (sorted by their weight).
let shard_id = 0;
let replicas: Vec<&String> = hrw.sorted(&shard_id)
                                .take(3).collect();
assert_eq!(replicas, vec!["node1", "node6", "node4"]);
```

## License

MIT
