// ============================================================
// Homun Runtime — set.rs: HashSet mutation helpers
// Part of stdlib, no external crates required.
//
// Usage in .hom:
//   use set
//
//   s ::= set_new()
//   set_add(s, "alpha")
//   set_add(s, "beta")
//   removed := set_remove(s, "alpha")   // bool
//   n := len(s)
//   set_clear(s)
//
// `@{}` parses as an empty dict; use `set_new()` to construct an empty set.
// Read-only membership is already supported via the `in` operator.
//
// All mutators take `&mut HashSet<T>`. Callers must declare the binding mutable
// (`s ::= ...`); the codegen passes the first arg as `&mut s` because each
// `set_*` function is registered in `register_known_dep_fns`.
// ============================================================

use std::collections::HashSet;
use std::hash::Hash;

/// Construct a new empty `HashSet`.
pub fn set_new<T: Eq + Hash>() -> HashSet<T> {
    HashSet::new()
}

/// Insert `x` into `s`. Idempotent — duplicate inserts are no-ops.
pub fn set_add<T: Eq + Hash>(s: &mut HashSet<T>, x: T) {
    s.insert(x);
}

/// Remove `x` from `s`. Returns `true` if the element was present.
pub fn set_remove<T: Eq + Hash>(s: &mut HashSet<T>, x: T) -> bool {
    s.remove(&x)
}

/// Drop every element in `s`.
pub fn set_clear<T>(s: &mut HashSet<T>) {
    s.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
