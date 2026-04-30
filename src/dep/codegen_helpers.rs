// dep/codegen_helpers.rs — String and type helpers for .hom codegen module.
//
// These functions are extracted from codegen.rs so that the future codegen.hom
// can call them through the dep bridge.  They contain Rust-specific logic that
// cannot be expressed in Homun:
//   - parse_interp() / escape_str() : character-level string scanning
//   - codegen_type()                : recursive TypeExpr → Rust type mapping
//   - codegen_params_mut()          : Param[] → "mut p: T" strings
//   - infer_generics()              : count un-typed params → T/U/V generic list
//   - is_str_expr() / is_list_expr(): expr-kind predicates for operator dispatch
//
// All functions take owned values (not references) so that .hom-generated code,
// which wraps every argument in `.clone()`, can call them without type errors.

// ─── Indentation ─────────────────────────────────────────────────────────────

/// Returns `n * 4` spaces as an indentation string.
pub fn ind(n: i32) -> String {
    " ".repeat((n * 4) as usize)
}

/// Concatenate two `Vec<String>` values (helper for codegen.hom list concat).
pub fn vec_extend_strings(mut a: Vec<String>, b: Vec<String>) -> Vec<String> {
    a.extend(b);
    a
}

// ─── Expression predicates ───────────────────────────────────────────────────

/// Returns `true` if the expression is known to produce a string value.
/// Used by `cg_bin_op` to decide whether `+` should emit string concatenation.
pub fn is_str_expr(expr: Expr) -> bool {
    match expr {
        Expr::Str(_) => true,
        Expr::BinOp(BinOp::Add, l, r) => is_str_expr(*l) || is_str_expr(*r),
        Expr::Call(f, _) => {
            if let Expr::Var(n) = *f {
                n == "str"
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Returns `true` if the expression is known to produce a list (Vec) value.
/// Used by `cg_bin_op` to decide whether `+` should emit `homun_concat`.
pub fn is_list_expr(expr: Expr) -> bool {
    match expr {
        Expr::List(_) | Expr::Slice(_, _, _, _) => true,
        Expr::BinOp(BinOp::Add, l, r) => is_list_expr(*l) || is_list_expr(*r),
        _ => false,
    }
}

// ─── Utilities ───────────────────────────────────────────────────────────────

/// Returns `true` if the first character of `s` is an ASCII uppercase letter.
/// Used in codegen to distinguish enum variants (PascalCase) from struct fields.
pub fn is_upper_first(s: String) -> bool {
    s.chars().next().is_some_and(|c| c.is_uppercase())
}

// ─── Homun macro names ───────────────────────────────────────────────────────

/// Names of Homun builtins that are emitted as Rust macros (`name!(...)`)
/// rather than regular function calls.
pub const HOMUN_MACROS: &[&str] = &[
    "range", "len", "filter", "map", "reduce", "slice", "dict", "set",
];

/// Returns `true` if `name` is a Homun macro name.
pub fn is_homun_macro(name: String) -> bool {
    HOMUN_MACROS.contains(&name.as_str())
}

// ─── Self-recursive type registry ───────────────────────────────────────────

thread_local! {
    static SELF_RECURSIVE_TYPES: std::cell::RefCell<std::collections::HashSet<String>> =
        std::cell::RefCell::new(std::collections::HashSet::new());
}

/// Mark `name` as a self-recursive type for the duration of its variant emission.
pub fn register_recursive_type(name: String) {
    SELF_RECURSIVE_TYPES.with(|s| s.borrow_mut().insert(name));
}

/// Clear the self-recursive type registry after variant emission is done.
pub fn clear_recursive_types() {
    SELF_RECURSIVE_TYPES.with(|s| s.borrow_mut().clear());
}

/// Returns true if `name` is currently registered as a self-recursive type.
pub fn is_self_recursive_type(name: String) -> bool {
    SELF_RECURSIVE_TYPES.with(|s| s.borrow().contains(&name))
}

// ─── Thread-local variable registry ─────────────────────────────────────────

thread_local! {
    static THREAD_LOCAL_VARS: std::cell::RefCell<std::collections::HashSet<String>> =
        std::cell::RefCell::new(std::collections::HashSet::new());
}

/// Register `name` as a @thread_local binding.
pub fn register_thread_local_var(name: String) {
    THREAD_LOCAL_VARS.with(|s| s.borrow_mut().insert(name));
}

/// Returns true if `name` was declared as a @thread_local binding.
pub fn is_thread_local_var(name: String) -> bool {
    THREAD_LOCAL_VARS.with(|s| s.borrow().contains(&name))
}

/// Clear the thread-local variable registry (call at start of each compilation).
pub fn clear_thread_local_vars() {
    THREAD_LOCAL_VARS.with(|s| s.borrow_mut().clear());
}

// ─── Preamble helpers ────────────────────────────────────────────────────────

/// Standard derive attributes for Homun-generated structs and enums.
pub fn derive_attrs() -> String {
    "#[derive(Debug, Clone, PartialEq)]".to_string()
}

/// Format the generic type-parameter clause for a function.
/// Returns `"<T: Clone, U: Clone>"` when non-empty, or `""` when empty.
pub fn generics_str(generics: Vec<String>) -> String {
    if generics.is_empty() {
        String::new()
    } else {
        format!("<{}>", generics.join(", "))
    }
}
