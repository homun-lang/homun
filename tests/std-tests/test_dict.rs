// Tests for dict.rs — extracted from #[cfg(test)] mod tests
// Included into dict_mod in lib.rs (after dict.rs is included).

#[test]
fn test_from_pairs_empty() {
    let d: HashMap<String, i32> = dict_from_pairs(vec![]);
    assert!(d.is_empty());
}

#[test]
fn test_from_pairs_single() {
    let d = dict_from_pairs(vec![("a".to_string(), 1)]);
    assert_eq!(d.len(), 1);
    assert_eq!(d["a"], 1);
}

#[test]
fn test_from_pairs_multiple() {
    let d = dict_from_pairs(vec![
        ("a".to_string(), 1),
        ("b".to_string(), 2),
        ("c".to_string(), 3),
    ]);
    assert_eq!(d.len(), 3);
    assert_eq!(d["a"], 1);
    assert_eq!(d["b"], 2);
    assert_eq!(d["c"], 3);
}

#[test]
fn test_from_pairs_duplicate_key_last_wins() {
    let d = dict_from_pairs(vec![("x".to_string(), 10), ("x".to_string(), 20)]);
    assert_eq!(d.len(), 1);
    assert_eq!(d["x"], 20);
}

#[test]
fn test_from_pairs_int_keys() {
    let d = dict_from_pairs(vec![(0, "zero".to_string()), (1, "one".to_string())]);
    assert_eq!(d[&0], "zero");
    assert_eq!(d[&1], "one");
}

#[test]
fn test_from_pairs_sugiyama_pattern() {
    let ordering = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let pairs: Vec<(String, usize)> = ordering
        .into_iter()
        .enumerate()
        .map(|(pos, node)| (node, pos))
        .collect();
    let position = dict_from_pairs(pairs);
    assert_eq!(position["A"], 0);
    assert_eq!(position["B"], 1);
    assert_eq!(position["C"], 2);
}

#[test]
fn test_zip_empty() {
    let d: HashMap<String, i32> = dict_zip(vec![], vec![]);
    assert!(d.is_empty());
}

#[test]
fn test_zip_equal_lengths() {
    let keys = vec!["x".to_string(), "y".to_string(), "z".to_string()];
    let values = vec![10, 20, 30];
    let d = dict_zip(keys, values);
    assert_eq!(d.len(), 3);
    assert_eq!(d["x"], 10);
    assert_eq!(d["y"], 20);
    assert_eq!(d["z"], 30);
}

#[test]
fn test_zip_more_keys_than_values() {
    let keys = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let values = vec![1, 2];
    let d = dict_zip(keys, values);
    assert_eq!(d.len(), 2);
    assert!(d.contains_key("a"));
    assert!(d.contains_key("b"));
    assert!(!d.contains_key("c"));
}

#[test]
fn test_zip_more_values_than_keys() {
    let keys = vec!["a".to_string()];
    let values = vec![1, 2, 3];
    let d = dict_zip(keys, values);
    assert_eq!(d.len(), 1);
    assert_eq!(d["a"], 1);
}

#[test]
fn test_zip_single_pair() {
    let d = dict_zip(vec!["only".to_string()], vec![42]);
    assert_eq!(d["only"], 42);
}

#[test]
fn test_clone_empty() {
    let original: HashMap<String, i32> = HashMap::new();
    let cloned = dict_clone(original.clone());
    assert!(cloned.is_empty());
}

#[test]
fn test_clone_preserves_entries() {
    let mut original = HashMap::new();
    original.insert("a".to_string(), 1);
    original.insert("b".to_string(), 2);
    let cloned = dict_clone(original.clone());
    assert_eq!(cloned.len(), 2);
    assert_eq!(cloned["a"], 1);
    assert_eq!(cloned["b"], 2);
}

#[test]
fn test_clone_is_independent() {
    let mut original = HashMap::new();
    original.insert("key".to_string(), 100);
    let mut cloned = dict_clone(original.clone());
    cloned.insert("key".to_string(), 999);
    assert_eq!(original["key"], 100);
    assert_eq!(cloned["key"], 999);
}

#[test]
fn test_clone_original_independent() {
    let mut original = HashMap::new();
    original.insert("k".to_string(), 1);
    let cloned = dict_clone(original.clone());
    original.insert("k".to_string(), 2);
    assert_eq!(cloned["k"], 1);
}
