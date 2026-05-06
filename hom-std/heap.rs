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
//   heap_pop(h)                   // discards lowest-priority item
//   n := heap_len(h)
//   empty := heap_is_empty(h)
//
// heap_push/heap_pop take &mut Heap (codegen emits &mut h via
// register_known_dep_fns). heap_len/heap_is_empty take &Heap.
// BinaryHeap wrapped with Reverse<i32> gives min-heap semantics.
// priority/return types use i32 for .hom int compatibility.
// item accepts impl AsRef<str> for both &str and String args.
// ============================================================

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Priority queue: min-heap keyed by i32, storing String items.
pub type Heap = BinaryHeap<(Reverse<i32>, String)>;

/// Create a new empty min-heap.
pub fn heap_new() -> Heap {
    BinaryHeap::new()
}

/// Push `item` with `priority` (lower priority = popped first).
pub fn heap_push(h: &mut Heap, priority: i32, item: impl AsRef<str>) {
    h.push((Reverse(priority), item.as_ref().to_string()));
}

/// Pop and return `(priority, item)` with the lowest priority, or `None`.
pub fn heap_pop(h: &mut Heap) -> Option<(i32, String)> {
    h.pop().map(|(Reverse(p), s)| (p, s))
}

/// Return the number of items in the heap (i32 for .hom int compatibility).
pub fn heap_len(h: &Heap) -> i32 {
    h.len() as i32
}

/// Return `true` if the heap contains no items.
pub fn heap_is_empty(h: &Heap) -> bool {
    h.is_empty()
}
