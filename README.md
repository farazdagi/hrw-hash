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

## Motivation

Given list of nodes (as `IntoIterator<Item = Node>`) the aim is to produce *sorted* list of
references to these nodes (`Iterator<Item = &Node>`) for any given key.

This sorted list serves as priority-based set of destination nodes for the key. With the first node
being the primary replica, the second one being the secondary replica, and so on.

Both weighted and non-weighted nodes are supported.

## Usage

For non-weighted nodes:

``` rust
use hrw_hash::{HrwNodes, HrwNode};

// Anything that implements `Hash + Eq` can be used as node.
#[derive(Debug, PartialEq, Eq, Hash)]
struct MyNode(u64);

// Mark the node as eligible for HRW hashing.
impl HrwNode for MyNode {}

// Create a new HRW instance with the list of nodes.
let nodes: Vec<MyNode> = (0..10).map(|i| MyNode(i)).collect();
let hrw = HrwNodes::new(nodes);

// For a given key, get list of nodes sorted by their priority.
let key = 42;
let replicas: Vec<&MyNode> = hrw.sorted(&key).take(3).collect();
assert_eq!(replicas, vec![&MyNode(6), &MyNode(5), &MyNode(2)]);
```

For weighted nodes, which can have different capacities, the only difference is that you have to
implement the `capacity()` method of `HrwNode` trait:

``` rust
use hrw_hash::{HrwNode, HrwNodes};

// Anything that implements `Hash + Eq` can be used as node.
#[derive(Debug, PartialEq, Eq, Hash)]
struct MyNode {
    id: u64,
    capacity: usize,
}

// Mark the node as eligible for HRW hashing.
// The `capacity()` method returns the capacity of the node.
impl HrwNode for MyNode {
    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl MyNode {
    fn new(id: u64, capacity: usize) -> Self {
        Self { id, capacity }
    }
}

let mut nodes = Vec::new();
for i in 0..100 {
    // Regular nodes, have the same capacity.
    nodes.push(MyNode::new(i, 1));
}
// Add some nodes with higher capacities.
nodes.push(MyNode::new(100, 50));
nodes.push(MyNode::new(101, 20));

let hrw = HrwNodes::new(nodes);

// Nodes `100` and `101` have higher capacity, so they will be
// selected more often -- even though there are many other nodes.
assert_eq!(
    hrw.sorted(&"foobar1").next(), // primary replica for the key
    Some(&MyNode::new(29, 1))      // one of the regular nodes
);
assert_eq!(
    hrw.sorted(&"foobar2").next(),
    Some(&MyNode::new(78, 1))      // one of the regular nodes
);
assert_eq!(
    hrw.sorted(&"foobar3").next(),
    Some(&MyNode::new(100, 50))    // the higher capacity node
);
assert_eq!(
    hrw.sorted(&"foobar4").next(), 
    Some(&MyNode::new(101, 20))    // the higher capacity node
);
assert_eq!(
    hrw.sorted(&"foobar5").next(), 
    Some(&MyNode::new(100, 50))    // the higher capacity node
);
```

## License

MIT
