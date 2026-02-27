// ============================================================
// Homun Runtime — heap.rs: Priority Queue (min-heap)
// Part B1 — stdlib, no external crates required.
//
// Usage in .hom:
//   use heap
//
//   h := heap_new()
//   heap_push(h, 5, "node_a")    // priority, item
//   heap_push(h, 2, "node_b")
//   item := heap_pop(h)           // returns lowest priority first (min-heap)
//   n := heap_len(h)
//   empty := heap_is_empty(h)
//
// Implementation note:
//   Wraps std::collections::BinaryHeap with std::cmp::Reverse so that
//   the smallest priority value is popped first (min-heap semantics).
//   Items are Strings; priorities are i64.
// ============================================================

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Priority queue type: min-heap keyed by i64, storing String items.
pub type Heap = BinaryHeap<(Reverse<i64>, String)>;

/// Create a new empty min-heap.
pub fn heap_new() -> Heap {
    BinaryHeap::new()
}

/// Push `item` onto the heap with the given `priority`.
/// Items with lower priority values are popped first.
pub fn heap_push(h: &mut Heap, priority: i64, item: String) {
    h.push((Reverse(priority), item));
}

/// Pop and return the `(priority, item)` pair with the lowest priority.
/// Returns `None` if the heap is empty.
pub fn heap_pop(h: &mut Heap) -> Option<(i64, String)> {
    h.pop().map(|(Reverse(p), s)| (p, s))
}

/// Return the number of items in the heap.
pub fn heap_len(h: &Heap) -> usize {
    h.len()
}

/// Return `true` if the heap contains no items.
pub fn heap_is_empty(h: &Heap) -> bool {
    h.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── heap_new ────────────────────────────────────────────
    #[test]
    fn test_heap_new_is_empty() {
        let h = heap_new();
        assert!(heap_is_empty(&h));
        assert_eq!(heap_len(&h), 0);
    }

    // ── heap_push / heap_len ─────────────────────────────────
    #[test]
    fn test_heap_push_increases_len() {
        let mut h = heap_new();
        heap_push(&mut h, 10, "a".to_string());
        assert_eq!(heap_len(&h), 1);
        heap_push(&mut h, 5, "b".to_string());
        assert_eq!(heap_len(&h), 2);
    }

    #[test]
    fn test_heap_is_empty_after_push() {
        let mut h = heap_new();
        heap_push(&mut h, 1, "x".to_string());
        assert!(!heap_is_empty(&h));
    }

    // ── heap_pop ────────────────────────────────────────────
    #[test]
    fn test_heap_pop_empty_returns_none() {
        let mut h = heap_new();
        assert_eq!(heap_pop(&mut h), None);
    }

    #[test]
    fn test_heap_pop_single_item() {
        let mut h = heap_new();
        heap_push(&mut h, 42, "only".to_string());
        assert_eq!(heap_pop(&mut h), Some((42, "only".to_string())));
        assert!(heap_is_empty(&h));
    }

    #[test]
    fn test_heap_min_order_two_items() {
        let mut h = heap_new();
        heap_push(&mut h, 10, "high".to_string());
        heap_push(&mut h, 2, "low".to_string());
        let (p, item) = heap_pop(&mut h).unwrap();
        assert_eq!(p, 2);
        assert_eq!(item, "low");
    }

    #[test]
    fn test_heap_min_order_many_items() {
        let mut h = heap_new();
        heap_push(&mut h, 5, "five".to_string());
        heap_push(&mut h, 1, "one".to_string());
        heap_push(&mut h, 3, "three".to_string());
        heap_push(&mut h, 2, "two".to_string());
        heap_push(&mut h, 4, "four".to_string());

        let mut pops: Vec<i64> = Vec::new();
        while let Some((p, _)) = heap_pop(&mut h) {
            pops.push(p);
        }
        assert_eq!(pops, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_heap_pop_decreases_len() {
        let mut h = heap_new();
        heap_push(&mut h, 1, "a".to_string());
        heap_push(&mut h, 2, "b".to_string());
        heap_pop(&mut h);
        assert_eq!(heap_len(&h), 1);
        heap_pop(&mut h);
        assert_eq!(heap_len(&h), 0);
        assert!(heap_is_empty(&h));
    }

    // ── negative and zero priorities ────────────────────────
    #[test]
    fn test_heap_negative_priorities() {
        let mut h = heap_new();
        heap_push(&mut h, -5, "neg5".to_string());
        heap_push(&mut h, 0, "zero".to_string());
        heap_push(&mut h, -1, "neg1".to_string());

        let (p1, _) = heap_pop(&mut h).unwrap();
        let (p2, _) = heap_pop(&mut h).unwrap();
        let (p3, _) = heap_pop(&mut h).unwrap();
        assert_eq!(p1, -5);
        assert_eq!(p2, -1);
        assert_eq!(p3, 0);
    }

    #[test]
    fn test_heap_zero_priority() {
        let mut h = heap_new();
        heap_push(&mut h, 0, "origin".to_string());
        assert_eq!(heap_pop(&mut h), Some((0, "origin".to_string())));
    }

    // ── tie-breaking (same priority) ────────────────────────
    // BinaryHeap breaks ties by the second element (String lexicographic order,
    // reversed). We only check that all items are returned, not their order.
    #[test]
    fn test_heap_same_priority_all_returned() {
        let mut h = heap_new();
        heap_push(&mut h, 1, "alpha".to_string());
        heap_push(&mut h, 1, "beta".to_string());
        heap_push(&mut h, 1, "gamma".to_string());

        let mut items: Vec<String> = Vec::new();
        while let Some((p, item)) = heap_pop(&mut h) {
            assert_eq!(p, 1);
            items.push(item);
        }
        items.sort();
        assert_eq!(items, vec!["alpha", "beta", "gamma"]);
    }

    // ── A* typical usage simulation ──────────────────────────
    #[test]
    fn test_heap_astar_simulation() {
        // Simulate: frontier queue in A* where priority = f-score
        let mut frontier = heap_new();
        heap_push(&mut frontier, 10, "A".to_string());
        heap_push(&mut frontier, 7, "B".to_string());
        heap_push(&mut frontier, 15, "C".to_string());
        heap_push(&mut frontier, 3, "D".to_string());

        // D has lowest f-score; should come out first
        let (p, node) = heap_pop(&mut frontier).unwrap();
        assert_eq!(p, 3);
        assert_eq!(node, "D");

        // Add a new node discovered from D
        heap_push(&mut frontier, 5, "E".to_string());

        let (p2, node2) = heap_pop(&mut frontier).unwrap();
        assert_eq!(p2, 5);
        assert_eq!(node2, "E");
    }
}
