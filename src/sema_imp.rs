// sema_imp.rs — Zero-impact trigger for has_rs_dep + sema error-list helpers.
//
// Importing this file via `use sema_imp` in sema.hom sets has_rs_dep=true in
// the homunc sema checker, which disables undefined-reference checks for dep/*
// functions (scope_*, stmt_kind, expr_kind, pat_kind, etc.) that are unknown
// to the homunc static checker but available at include!() time in lib.rs.
//
// Helper functions work with Vec<String> (Homun @[str]) for error lists.

/// Return an empty error list.
pub fn errs_empty() -> Vec<String> {
    vec![]
}

/// Return a single-element error list.
pub fn errs_one(msg: String) -> Vec<String> {
    vec![msg]
}

/// Concatenate two error lists.
pub fn errs_join(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    let mut r = a;
    r.extend(b);
    r
}

/// True if the expression is a Lambda variant.
/// Used in check_stmt's ThreadLocal arm where a direct Homun match on
/// Expr.Lambda cannot be written (struct-like variant, boxed final_expr).
pub fn expr_is_lambda(e: Expr) -> bool {
    matches!(e, Expr::Lambda { .. })
}
