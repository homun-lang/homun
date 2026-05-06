// ============================================================
// Homun Runtime — path.rs: Path Utilities
//
// Usage in .hom:
//   use path
//   joined   := path_join(dir, name)
//   parent   := path_parent(p)
//   abs      := path_canonicalize(p)?
//   stripped := path_strip_prefix(p, prefix)
//
// Functions accept impl AsRef<str> for &str / String interop.
// Fallible functions return Result<String, String> so ? works.
// ============================================================

/// Join two path components. Equivalent to PathBuf::push.
pub fn path_join(dir: impl AsRef<str>, name: impl AsRef<str>) -> String {
    std::path::Path::new(dir.as_ref())
        .join(name.as_ref())
        .to_string_lossy()
        .into_owned()
}

/// Return the parent directory of a path. Returns "" if none.
pub fn path_parent(path: impl AsRef<str>) -> String {
    std::path::Path::new(path.as_ref())
        .parent()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Canonicalize a path (resolve symlinks, `..`). Returns Err if path doesn't exist.
pub fn path_canonicalize(path: impl AsRef<str>) -> Result<String, String> {
    let p = path.as_ref();
    std::fs::canonicalize(p)
        .map(|pb| pb.to_string_lossy().into_owned())
        .map_err(|e| format!("Cannot canonicalize {}: {}", p, e))
}

/// Strip a prefix from a path. Returns the original path string if prefix doesn't match.
pub fn path_strip_prefix(path: impl AsRef<str>, prefix: impl AsRef<str>) -> String {
    let p = std::path::Path::new(path.as_ref());
    p.strip_prefix(prefix.as_ref())
        .map(|stripped| stripped.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.as_ref().to_owned())
}
