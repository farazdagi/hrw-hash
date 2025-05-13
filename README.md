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
- [x] Allow weighted nodes.
- [x] Optimized for performance and memory usage. No wasted re-hashing.
- [ ] Allow massive number of nodes (`O(log(n))` lookup time, instead of `O(n)`).

## Motivation

Given an iterator of nodes (`IntoIterator<Item = Node>`) the aim is to produce sorted list of
references to these nodes (`Iterator<Item = &Node>`) for any given key.

This list serves as priority-sorted list of destination nodes for the key.

Both weighted and non-weighted nodes are supported.

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

For weighted nodes, which can have different capacities:

``` rust
use hrw_hash::{WeightedHrwNodes, WeightedNode};

// Define node
// (anything that implements `Hash + Eq` can be used as node).
#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    id: u64,
    capacity: usize,
}

impl Node {
    fn new(id: u64, capacity: usize) -> Self {
        Self { id, capacity }
    }
}

// Implement `WeightedNode` trait for the node.
impl WeightedNode for Node {
    fn capacity(&self) -> usize {
        self.capacity
    }
}

let mut nodes = Vec::new();
for i in 0..100 {
    // Regular nodes, have the same capacity.
    nodes.push(Node::new(i, 1));
}
// Add some nodes with higher capacities.
nodes.push(Node::new(100, 50));
nodes.push(Node::new(101, 20));

let hrw = WeightedHrwNodes::new(nodes);

// Nodes `100` and `101` have higher capacity, so they will be
// selected more often -- even though there are many other nodes.
assert_eq!(hrw.sorted(&"foobar1").next(), Some(&Node::new(29, 1)));
assert_eq!(hrw.sorted(&"foobar2").next(), Some(&Node::new(78, 1)));
assert_eq!(hrw.sorted(&"foobar3").next(), Some(&Node::new(100, 50)));
assert_eq!(hrw.sorted(&"foobar4").next(), Some(&Node::new(101, 20)));
assert_eq!(hrw.sorted(&"foobar5").next(), Some(&Node::new(100, 50)));
```

## License

MIT
