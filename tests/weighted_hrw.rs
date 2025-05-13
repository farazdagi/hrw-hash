use {
    hrw_hash::{WeightedHrwNodes, WeightedNode},
    std::collections::HashMap,
};

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

    impl WeightedNode for Node {
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

    let nodes = WeightedHrwNodes::new(nodes);

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
