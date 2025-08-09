# hrw-hash

[![crates.io](https://img.shields.io/crates/d/hrw-hash.svg)](https://crates.io/crates/hrw-hash)
[![docs.rs](https://docs.rs/hrw-hash/badge.svg)](https://docs.rs/hrw-hash)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![dependencies](https://deps.rs/repo/github/farazdagi/hrw-hash/status.svg)](https://deps.rs/repo/github/farazdagi/hrw-hash)

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

### Non-weighted nodes

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
assert_eq!(replicas, vec![&MyNode(4), &MyNode(7), &MyNode(0)]);
```

### Weighted nodes

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
    hrw.sorted(&"key1").next(),  // primary replica for the key
    Some(&MyNode::new(33, 1))    // one of the regular nodes
);
assert_eq!(
    hrw.sorted(&"key2").next(),
    Some(&MyNode::new(100, 50))  // higher capacity node
);
assert_eq!(
    hrw.sorted(&"key3").next(),
    Some(&MyNode::new(100, 50))  // higher capacity node
);
assert_eq!(
    hrw.sorted(&"key4").next(),
    Some(&MyNode::new(101, 20))  // higher capacity node
);
assert_eq!(
    hrw.sorted(&"key5").next(),
    Some(&MyNode::new(101, 20))  // higher capacity node
);
```

### Default implementation of `HrwNode` trait

Numeric primitive types can also be used as HRW nodes (`u8`, `u16`, `u32`, `u64`, `usize`, `i8`,
`i16`, `i32`, `i64`, `isize`, `char`). This is done to allow passing node indexes or IDs directly.

Additionally, mostly for testing purposes, `String`, `&str`, `&[u8]` do implement `HrwNode` trait as
well.

``` rust
use hrw_hash::{HrwNode, HrwNodes};

// String as node
let nodes: Vec<String> = (0..10).map(|i| format!("node{}", i)).collect();
let hrw = HrwNodes::new(nodes);
let replicas = hrw.sorted(&42).take(3).collect::<Vec<_>>();
assert_eq!(replicas, vec![&"node3", &"node1", &"node5"]);

// u16 as node
let nodes: Vec<u16> = (0..10).map(|i| i).collect();
let hrw = HrwNodes::new(nodes);
let replicas = hrw.sorted(&42).take(3).collect::<Vec<_>>();
assert_eq!(replicas, vec![&8, &4, &7]);
```

### Custom hasher

To give you full control over how values are hashed, you can specify custom hasher builder:

``` rust
// Assuming you have the following in your `Cargo.toml`:
// twox-hash = { version = "2.1", features = ["std", "xxhash3_64"] }
use twox_hash::XxHash3_64;
use std::hash::BuildHasherDefault;
use hrw_hash::{HrwNode, HrwNodes};

let nodes = (0..10).map(|i| i);

// Pass in `XXHash3_64` hasher builder.
let hrw = HrwNodes::with_build_hasher(BuildHasherDefault::<XxHash3_64>::default(), nodes);

let replicas = hrw.sorted(&42).take(3).collect::<Vec<_>>();
assert_eq!(replicas, vec![&1, &2, &7]);
```

Note: the default hasher (`rapidhash v.3`) is guaranteed to be stable across architectures, Rust
versions, and crate updates. This means that the same key will always hash to the same value,
regardless of the platform or Rust version used to compile the code.

## License

MIT
