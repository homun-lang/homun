// codegen_imp.rs — Type aliases and accessors for codegen.hom.
//
// Importing this file via `use codegen_imp` in codegen.hom sets has_rs_dep=true
// in the homunc sema checker, disabling undefined-reference checks for dep/*
// functions (scope_*, stmt_kind, expr_kind, codegen_type, etc.) and for
// runtime functions (join, push, etc.) that are available at include!() time
// in lib.rs but unknown to the homunc static checker.
//
// Type aliases:
//   RsContent = HashMap<String, String>   — resolved .rs file content map
//   HomFiles  = HashSet<String>           — resolved .hom dependency names
//
// Accessor helpers (owned-value signatures for .hom interop):
//   rs_content_get(map, key)  -> Option<String>
//   hom_files_contains(set, key) -> bool
//
// fn-signature registry:
//   fn_mut_params_insert / fn_defaults_insert / current_mut_ref_params_*
//   is_mut_ref_param / is_param_mutable_in_call / fn_defaults_get_for
//   (Logic lives in codegen.hom; these are raw storage + primitive accessors.)

pub type RsContent = std::collections::HashMap<String, String>;
pub type HomFiles = std::collections::HashSet<String>;

// ─── fn-signature registry ────────────────────────────────────────────────────

use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static FN_MUT_PARAMS: RefCell<HashMap<String, Vec<bool>>> =
        RefCell::new(HashMap::new());
    static FN_DEFAULTS: RefCell<HashMap<String, Vec<Option<Expr>>>> =
        RefCell::new(HashMap::new());
    static CURRENT_MUT_REF_PARAMS: RefCell<std::collections::HashSet<String>> =
        RefCell::new(std::collections::HashSet::new());
}

pub fn fn_mut_params_insert(name: String, flags: Vec<bool>) {
    FN_MUT_PARAMS.with(|m| { m.borrow_mut().insert(name, flags); });
}

pub fn fn_defaults_insert(name: String, defaults: Vec<Option<Expr>>) {
    FN_DEFAULTS.with(|m| { m.borrow_mut().insert(name, defaults); });
}

pub fn current_mut_ref_params_clear() {
    CURRENT_MUT_REF_PARAMS.with(|m| m.borrow_mut().clear());
}

pub fn current_mut_ref_params_add(name: String) {
    CURRENT_MUT_REF_PARAMS.with(|m| { m.borrow_mut().insert(name); });
}

pub fn is_mut_ref_param(name: String) -> bool {
    CURRENT_MUT_REF_PARAMS.with(|m| m.borrow().contains(&name))
}

pub fn is_param_mutable_in_call(fn_name: String, arg_idx: i32) -> bool {
    FN_MUT_PARAMS.with(|m| {
        m.borrow()
            .get(&fn_name)
            .and_then(|flags| flags.get(arg_idx as usize).copied())
            .unwrap_or(false)
    })
}

pub fn fn_defaults_get_for(fn_name: String) -> Vec<Option<Expr>> {
    FN_DEFAULTS.with(|m| m.borrow().get(&fn_name).cloned().unwrap_or_default())
}

/// Look up a key in the rs_content map.  Returns None if absent.
pub fn rs_content_get(map: RsContent, key: String) -> Option<String> {
    map.get(&key).cloned()
}

/// Return true if the hom_files set contains the given key.
pub fn hom_files_contains(set: HomFiles, key: String) -> bool {
    set.contains(&key)
}
