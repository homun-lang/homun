// ============================================================
// Homun Runtime — dict.rs: Dict Builder Helpers
// Part B4 — stdlib, no external crates required.
//
// Usage in .hom:
//   use dict
//
//   // Build dict from list of (key, value) pairs
//   d := dict_from_pairs(@[("a", 1), ("b", 2)])
//
//   // Build dict from two parallel lists
//   d := dict_zip(keys, values)
//
//   // Clone a dict
//   d2 := dict_clone(d)
//
// Used by: Sugiyama layout (layer assignment, coordinate maps, node lookups).
//
// Why needed: Python uses dict comprehensions 8+ times in sugiyama.py for
// building lookup tables like `{node: pos for pos, node in enumerate(ordering)}`.
// These builders replace that pattern without verbose loop boilerplate in .hom.
//
// Type note:
//   dict_clone takes HashMap by value (not by reference) so that .hom code
//   like `d2 := dict_clone(d)` works correctly: homunc codegen emits
//   `dict_clone(d.clone())` which passes an owned HashMap.
// ============================================================

use std::collections::HashMap;
use std::hash::Hash;

/// Build a `HashMap` from a `Vec` of `(key, value)` pairs.
/// If the same key appears multiple times, the last value wins.
pub fn dict_from_pairs<K: Eq + Hash, V>(pairs: Vec<(K, V)>) -> HashMap<K, V> {
    pairs.into_iter().collect()
}

/// Build a `HashMap` by zipping `keys` and `values` together.
/// If the slices differ in length, excess elements from the longer one are ignored.
pub fn dict_zip<K: Eq + Hash, V>(keys: Vec<K>, values: Vec<V>) -> HashMap<K, V> {
    keys.into_iter().zip(values.into_iter()).collect()
}

/// Return a clone of `d`.
/// Takes `d` by value so that `dict_clone(d.clone())` from .hom codegen works:
/// the incoming clone is returned as-is (zero extra copies).
pub fn dict_clone<K: Eq + Hash + Clone, V: Clone>(d: HashMap<K, V>) -> HashMap<K, V> {
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── dict_from_pairs ─────────────────────────────────────

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
        // Mirrors: position = {node: pos for pos, node in enumerate(ordering)}
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

    // ── dict_zip ────────────────────────────────────────────

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

    // ── dict_clone ──────────────────────────────────────────

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
        // Mutating the clone does not affect the original.
        let mut original = HashMap::new();
        original.insert("key".to_string(), 100);
        let mut cloned = dict_clone(original.clone());
        cloned.insert("key".to_string(), 999);
        assert_eq!(original["key"], 100);
        assert_eq!(cloned["key"], 999);
    }

    #[test]
    fn test_clone_original_independent() {
        // Mutating the original does not affect the clone.
        let mut original = HashMap::new();
        original.insert("k".to_string(), 1);
        let cloned = dict_clone(original.clone());
        original.insert("k".to_string(), 2);
        assert_eq!(cloned["k"], 1);
    }
}
