// Tests for heap.rs — extracted from #[cfg(test)] mod tests
// Included directly into hom_tests in lib.rs (heap functions from runtime).
// All heap functions now take &mut Heap (T23).

#[test]
fn test_heap_new_is_empty() {
    let mut h = heap_new();
    assert!(heap_is_empty(&mut h));
    assert_eq!(heap_len(&mut h), 0);
}

#[test]
fn test_heap_push_increases_len() {
    let mut h = heap_new();
    heap_push(&mut h, 10, "a");
    assert_eq!(heap_len(&mut h), 1);
    heap_push(&mut h, 5, "b");
    assert_eq!(heap_len(&mut h), 2);
}

#[test]
fn test_heap_is_empty_after_push() {
    let mut h = heap_new();
    heap_push(&mut h, 1, "x");
    assert!(!heap_is_empty(&mut h));
}

#[test]
fn test_heap_push_string_type() {
    let mut h = heap_new();
    heap_push(&mut h, 5, "alpha".to_string());
    heap_push(&mut h, 2, "beta".to_string());
    assert_eq!(heap_len(&mut h), 2);
}

#[test]
fn test_heap_pop_empty_returns_none() {
    let mut h = heap_new();
    assert_eq!(heap_pop(&mut h), None);
}

#[test]
fn test_heap_pop_single_item() {
    let mut h = heap_new();
    heap_push(&mut h, 42, "only");
    assert_eq!(heap_pop(&mut h), Some((42, "only".to_string())));
    assert!(heap_is_empty(&mut h));
}

#[test]
fn test_heap_min_order_two_items() {
    let mut h = heap_new();
    heap_push(&mut h, 10, "high");
    heap_push(&mut h, 2, "low");
    let (p, item) = heap_pop(&mut h).unwrap();
    assert_eq!(p, 2);
    assert_eq!(item, "low");
}

#[test]
fn test_heap_min_order_many_items() {
    let mut h = heap_new();
    heap_push(&mut h, 5, "five");
    heap_push(&mut h, 1, "one");
    heap_push(&mut h, 3, "three");
    heap_push(&mut h, 2, "two");
    heap_push(&mut h, 4, "four");

    let mut pops: Vec<i32> = Vec::new();
    while let Some((p, _)) = heap_pop(&mut h) {
        pops.push(p);
    }
    assert_eq!(pops, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_heap_pop_decreases_len() {
    let mut h = heap_new();
    heap_push(&mut h, 1, "a");
    heap_push(&mut h, 2, "b");
    heap_pop(&mut h);
    assert_eq!(heap_len(&mut h), 1);
    heap_pop(&mut h);
    assert_eq!(heap_len(&mut h), 0);
    assert!(heap_is_empty(&mut h));
}

#[test]
fn test_heap_negative_priorities() {
    let mut h = heap_new();
    heap_push(&mut h, -5, "neg5");
    heap_push(&mut h, 0, "zero");
    heap_push(&mut h, -1, "neg1");

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
    heap_push(&mut h, 0, "origin");
    assert_eq!(heap_pop(&mut h), Some((0, "origin".to_string())));
}

#[test]
fn test_heap_same_priority_all_returned() {
    let mut h = heap_new();
    heap_push(&mut h, 1, "alpha");
    heap_push(&mut h, 1, "beta");
    heap_push(&mut h, 1, "gamma");

    let mut items: Vec<String> = Vec::new();
    while let Some((p, item)) = heap_pop(&mut h) {
        assert_eq!(p, 1);
        items.push(item);
    }
    items.sort();
    assert_eq!(items, vec!["alpha", "beta", "gamma"]);
}

#[test]
fn test_heap_astar_simulation() {
    let mut frontier = heap_new();
    heap_push(&mut frontier, 10, "A");
    heap_push(&mut frontier, 7, "B");
    heap_push(&mut frontier, 15, "C");
    heap_push(&mut frontier, 3, "D");

    let (p, node) = heap_pop(&mut frontier).unwrap();
    assert_eq!(p, 3);
    assert_eq!(node, "D");

    heap_push(&mut frontier, 5, "E");

    let (p2, node2) = heap_pop(&mut frontier).unwrap();
    assert_eq!(p2, 5);
    assert_eq!(node2, "E");
}
