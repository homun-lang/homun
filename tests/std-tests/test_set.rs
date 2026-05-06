// Tests for set.rs — extracted from #[cfg(test)] mod tests
// Included into set_mod in lib.rs (after set.rs is included).

#[test]
fn test_set_new_is_empty() {
    let s: HashSet<String> = set_new();
    assert!(s.is_empty());
}

#[test]
fn test_set_add_inserts() {
    let mut s = set_new();
    set_add(&mut s, "a".to_string());
    set_add(&mut s, "b".to_string());
    assert_eq!(s.len(), 2);
    assert!(s.contains("a"));
    assert!(s.contains("b"));
}

#[test]
fn test_set_add_duplicate_is_noop() {
    let mut s = set_new();
    set_add(&mut s, "x".to_string());
    set_add(&mut s, "x".to_string());
    assert_eq!(s.len(), 1);
}

#[test]
fn test_set_remove_existing_returns_true() {
    let mut s = set_new();
    set_add(&mut s, "a".to_string());
    let removed = set_remove(&mut s, "a".to_string());
    assert!(removed);
    assert!(s.is_empty());
}

#[test]
fn test_set_remove_missing_returns_false() {
    let mut s: HashSet<String> = set_new();
    let removed = set_remove(&mut s, "ghost".to_string());
    assert!(!removed);
}

#[test]
fn test_set_clear_empties() {
    let mut s = set_new();
    set_add(&mut s, "a".to_string());
    set_add(&mut s, "b".to_string());
    set_clear(&mut s);
    assert!(s.is_empty());
}

#[test]
fn test_set_clear_on_empty_is_safe() {
    let mut s: HashSet<String> = set_new();
    set_clear(&mut s);
    assert!(s.is_empty());
}

#[test]
fn test_set_int_elements() {
    let mut s: HashSet<i32> = set_new();
    set_add(&mut s, 1);
    set_add(&mut s, 2);
    set_add(&mut s, 1);
    assert_eq!(s.len(), 2);
    assert!(set_remove(&mut s, 1));
    assert_eq!(s.len(), 1);
}
