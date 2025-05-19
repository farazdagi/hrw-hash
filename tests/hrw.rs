use {
    hrw_hash::{HrwNode, HrwNodes},
    std::{
        collections::HashMap,
        hash::{Hash, Hasher},
    },
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    id: u16,
    name: String,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl HrwNode for Node {}

impl Node {
    fn new(id: u16) -> Self {
        Self {
            id,
            name: format!("node{}", id),
        }
    }
}

#[test]
fn hrw() {
    let hrw = HrwNodes::new((0..10).map(|i| Node {
        id: i,
        name: format!("node{}", i),
    }));
    let shard_id = 42;
    let replicas = hrw.sorted(&shard_id).take(3).collect::<Vec<_>>();
    assert_eq!(replicas, vec![&Node::new(4), &Node::new(5), &Node::new(6)]);
}

#[test]
fn fair_distribution() {
    fn expected_min_max_ratio(nodes_count: usize, replication_factor: usize) -> f64 {
        // nodes, shards per node, min/max ratio
        let categories = vec![
            (16, 1, 0.95),
            (16, 2, 0.95),
            (16, 3, 0.96),
            (32, 1, 0.90),
            (32, 2, 0.94),
            (32, 3, 0.94),
            (64, 1, 0.85),
            (64, 2, 0.88),
            (64, 3, 0.92),
            (128, 1, 0.77),
            (128, 2, 0.83),
            (128, 3, 0.84),
            (256, 1, 0.69),
            (256, 2, 0.76),
            (256, 3, 0.82),
            (512, 1, 0.55),
            (512, 2, 0.69),
            (512, 3, 0.74),
        ];

        for category in categories {
            if nodes_count <= category.0 && replication_factor == category.1 {
                return category.2;
            }
        }

        unreachable!()
    }

    #[derive(Debug)]
    struct TestCase {
        nodes_count: usize,
        nodes_per_shard: usize,
        expected_ratio: f64,
    }

    // Debug builds are really slow.
    let counts = if cfg!(debug_assertions) {
        vec![16, 32, 64]
    } else {
        vec![16, 32, 64, 128, 256, 512]
    };
    for nodes_count in counts {
        for nodes_per_shard in [1, 2, 3] {
            check_distribution(TestCase {
                nodes_count,
                nodes_per_shard,
                expected_ratio: expected_min_max_ratio(nodes_count, nodes_per_shard),
            });
        }
    }

    fn check_distribution(t: TestCase) {
        // Key space sharded into parts: shard id is mapped to responsible replicas.
        let mut keyspace: HashMap<_, Vec<&Node>> = HashMap::new();

        let nodes = (0..t.nodes_count as u16)
            .map(|i| Node {
                id: i,
                name: format!("node{}", i),
            })
            .collect::<Vec<_>>();
        let hrw = HrwNodes::new(nodes);

        for shard_id in 0..u16::MAX {
            let proposed_replicas = hrw
                .sorted(&shard_id)
                .take(t.nodes_per_shard)
                .collect::<Vec<_>>();
            keyspace.insert(shard_id, proposed_replicas);
        }

        // Now calculate min, max, and average number of replicas assigned to each
        // shard.
        let mut replicas_count = HashMap::new();
        for (_shard_id, replicas) in keyspace.iter() {
            for replica in replicas {
                let count = replicas_count.entry(replica).or_insert(0);
                *count += 1;
            }
        }

        let cnt_shards = keyspace.len();
        let cnt_shards_from_replicas =
            replicas_count.values().copied().sum::<usize>() / t.nodes_per_shard;
        assert_eq!(cnt_shards, cnt_shards_from_replicas,);

        let min = replicas_count.values().copied().min().unwrap();
        let max = replicas_count.values().copied().max().unwrap();
        let min_max_ratio = min as f64 / max as f64;
        dbg!(&t, min_max_ratio);
        assert!(
            min_max_ratio >= t.expected_ratio,
            "distribution is not fair enough (min: {} max: {} ratio: {} expected: {})",
            min,
            max,
            min_max_ratio,
            t.expected_ratio
        );
    }
}

#[test]
fn weighted_distribution() {
    #[derive(PartialEq, Eq, Hash)]
    struct Node {
        id: u64,
        capacity: usize,
    }

    impl Node {
        fn new(id: u64, capacity: usize) -> Self {
            Self { id, capacity }
        }
    }

    impl HrwNode for Node {
        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    // 3 nodes with total capacity of 50.
    let mut nodes = vec![Node::new(1, 5), Node::new(2, 15), Node::new(3, 30)];
    // 50 nodes with total capacity of 50.
    for id in 4..54 {
        nodes.push(Node::new(id, 1));
    }

    let nodes = HrwNodes::new(nodes);

    let mut counts = HashMap::new();
    for key in 0..u16::MAX {
        let proposed_replica = nodes.sorted(&key).take(1).collect::<Vec<_>>()[0];
        counts
            .entry(proposed_replica.id)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    // On perfectly balanced distribution, each node should be chosen at least `k`
    // times, i.e. one share of capacity gives node `k` occurrences.
    let k = u16::MAX / 100;
    for id in 1u64..54 {
        let count = *counts.get(&id).unwrap_or(&0);
        let k = match id {
            1 => k * 5,
            2 => k * 15,
            3 => k * 30,
            _ => k * 1,
        };
        // make sure that count diffs no more than 10% of the expected value
        let diff = (count as f64 - k as f64) / k as f64;
        assert!(
            diff.abs() < 0.1,
            "Node {}: expected {}, got {}",
            id,
            k,
            count
        );
    }
}

#[test]
fn blanket_implementation() {
    let hrw = HrwNodes::new((0..10).map(|i| i));
    let replicas = hrw.sorted(&42).take(3).collect::<Vec<_>>();
    assert_eq!(replicas, vec![&9, &1, &2]);

    let hrw = HrwNodes::new((0..10).map(|i| format!("node{}", i)));
    let replicas = hrw.sorted(&42).take(3).collect::<Vec<_>>();
    assert_eq!(replicas, vec![&"node6", &"node0", &"node7"]);

    let nodes: Vec<u16> = (0..10).map(|i| i).collect();
    let hrw = HrwNodes::new(nodes);
    let replicas = hrw.sorted(&42).take(3).collect::<Vec<_>>();
    assert_eq!(replicas, vec![&4, & 5, &6]);
}
