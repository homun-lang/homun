/// Type definitions for the Homun multi-file resolver.
///
/// Logic is implemented in `resolver.hom` (compiled to `resolver_hom` module).
/// These structs are kept here so that `resolver_imp.rs` can re-export them via
/// `pub use crate::resolver::{ResolvedFile, ResolvedProgram};`.
use std::collections::HashSet;
use std::path::PathBuf;

/// A single compiled file's output plus its exported names.
#[derive(Clone)]
pub struct ResolvedFile {
    pub path: PathBuf,
    pub rust_code: String,
    pub exports: HashSet<String>,
}

/// Result of resolving an entire dependency graph.
#[derive(Clone)]
pub struct ResolvedProgram {
    /// Compiled fragments in topological order (leaves first).
    pub files: Vec<ResolvedFile>,
}
