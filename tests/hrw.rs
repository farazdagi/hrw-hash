use {hrw_hash::HrwNodes, std::collections::HashMap};

#[test]
fn hrw() {
    let mut nodes = vec![];
    for i in 0..10 {
        nodes.push(format!("node{}", i));
    }

    let hrw = HrwNodes::new(nodes);
    let shard_id = 0;
    let replicas = hrw.sorted(&shard_id).take(3).collect::<Vec<_>>();
    assert_eq!(replicas.len(), 3);
    assert_eq!(replicas, vec!["node1", "node6", "node4"]);
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
            (128, 1, 0.79),
            (128, 2, 0.83),
            (128, 3, 0.84),
            (256, 1, 0.71),
            (256, 2, 0.80),
            (256, 3, 0.84),
            (512, 1, 0.55),
            (512, 2, 0.66),
            (512, 3, 0.72),
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

    for nodes_count in [16, 32, 64, 128, 256, 512] {
        for nodes_per_shard in [1, 2, 3] {
            check_distribution(TestCase {
                nodes_count,
                nodes_per_shard,
                expected_ratio: expected_min_max_ratio(nodes_count, nodes_per_shard),
            });
        }
    }

    fn check_distribution(t: TestCase) {
        use std::hash::{Hash, Hasher};

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
