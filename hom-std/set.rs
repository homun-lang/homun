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
