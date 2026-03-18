// dep/scope.rs — HashSet<String> scope wrapper for .hom sema/codegen.
//
// Provides a set of bound names that can be passed through .hom code.
// All functions take and return owned values, so .hom's clone-everything
// codegen works correctly:  scope_insert returns the modified scope
// (caller must capture the return to keep the mutation).
//
// IMPORTANT: uses fully-qualified `std::collections::HashSet` rather than
// `use` statements to avoid E0252 "defined multiple times" when multiple
// dep .rs files are inlined together by the homunc build system.
//
// ─── Exported API ────────────────────────────────────────────────────────────
//
//   Scope = HashSet<String>
//     scope_new()                    -> Scope          (empty scope)
//     scope_contains(sc, name)       -> bool
//     scope_insert(sc, name)         -> Scope          (returns modified scope)
//     scope_clone(sc)                -> Scope          (identity — codegen already clones)
//     scope_from_list(list)          -> Scope          (build from Vec<String>)
//     scope_union(a, b)              -> Scope          (new scope = a ∪ b)
//     scope_to_list(sc)              -> Vec<String>    (snapshot as sorted list)

pub type Scope = std::collections::HashSet<String>;

/// Create a new empty scope.
pub fn scope_new() -> Scope {
    std::collections::HashSet::new()
}

/// Return `true` if `name` is present in `sc`.
pub fn scope_contains(sc: Scope, name: String) -> bool {
    sc.contains(&name)
}

/// Insert `name` into `sc` and return the modified scope.
/// Caller must capture the return: `sc := scope_insert(sc, name)`.
pub fn scope_insert(mut sc: Scope, name: String) -> Scope {
    sc.insert(name);
    sc
}

/// Return the scope as-is (the real cloning is done by the codegen at the call site).
pub fn scope_clone(sc: Scope) -> Scope {
    sc
}

/// Build a scope pre-populated from all elements of `list`.
pub fn scope_from_list<S: Into<String>>(list: Vec<S>) -> Scope {
    list.into_iter().map(|s| s.into()).collect()
}

/// Return a new scope containing all names from both `a` and `b`.
pub fn scope_union(mut a: Scope, b: Scope) -> Scope {
    a.extend(b);
    a
}

/// Return all names in `sc` as a sorted Vec<String>.
/// Sorting ensures deterministic output for tests.
pub fn scope_to_list(sc: Scope) -> Vec<String> {
    let mut v: Vec<String> = sc.into_iter().collect();
    v.sort();
    v
}
