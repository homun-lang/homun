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
// Implementation note:
//   Uses Rc<RefCell<BinaryHeap<...>>> so that Homun's &mut calling
//   convention passes a mutable reference to the Rc. Rc::clone() is a
//   cheap reference-count increment, not a deep copy, so all
//   "copies" of a Heap value share one BinaryHeap.
//
//   BinaryHeap wrapped with Reverse<i32> gives min-heap semantics
//   (smallest priority value is popped first).
//
//   priority and return types use i32 to match .hom's int type.
//   item accepts impl AsRef<str> to work with both &str literals
//   (Rust tests) and String values (homunc codegen emits .to_string()
//   on all string literals when passing as function arguments).
// ============================================================

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::Rc;

/// Priority queue: min-heap keyed by i32, storing String items.
/// Rc<RefCell<...>> allows interior mutability; functions take &mut Heap
/// so that codegen emits &mut h instead of h.clone().
pub type Heap = Rc<RefCell<BinaryHeap<(Reverse<i32>, String)>>>;

/// Create a new empty min-heap.
pub fn heap_new() -> Heap {
    Rc::new(RefCell::new(BinaryHeap::new()))
}

/// Push `item` onto the heap with the given `priority`.
/// Items with lower priority values are popped first (min-heap).
/// Accepts impl AsRef<str> so that &str literals and String values
/// (emitted by homunc for .hom string args) both work.
pub fn heap_push(h: &mut Heap, priority: i32, item: impl AsRef<str>) {
    h.borrow_mut()
        .push((Reverse(priority), item.as_ref().to_string()));
}

/// Pop and return the `(priority, item)` pair with the lowest priority.
/// Returns `None` if the heap is empty.
pub fn heap_pop(h: &mut Heap) -> Option<(i32, String)> {
    h.borrow_mut().pop().map(|(Reverse(p), s)| (p, s))
}

/// Return the number of items in the heap (i32 for .hom int compatibility).
pub fn heap_len(h: &mut Heap) -> i32 {
    h.borrow().len() as i32
}

/// Return `true` if the heap contains no items.
pub fn heap_is_empty(h: &mut Heap) -> bool {
    h.borrow().is_empty()
}
