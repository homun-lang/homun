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

/// Insert `(k, v)` into `d`, replacing any existing entry under `k`.
pub fn dict_insert<K: Eq + Hash, V>(d: &mut HashMap<K, V>, k: K, v: V) {
    d.insert(k, v);
}

/// Remove the entry for `k` from `d`. Returns the old value if the key was
/// present, `None` otherwise.
pub fn dict_remove<K: Eq + Hash, V>(d: &mut HashMap<K, V>, k: K) -> Option<V> {
    d.remove(&k)
}

/// Drop every entry in `d`.
pub fn dict_clear<K, V>(d: &mut HashMap<K, V>) {
    d.clear();
}

