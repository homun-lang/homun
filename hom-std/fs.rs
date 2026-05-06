// ============================================================
// Homun Runtime — fs.rs: Filesystem Utilities
//
// Usage in .hom:
//   use fs
//   content := fs_read(p)?
//   exists  := fs_exists(p)
//   is_dir  := fs_is_dir(p)
//   fs_write(p, content)?
//
// Functions accept impl AsRef<str> for &str / String interop.
// Fallible functions return Result<T, String> so ? works.
// ============================================================

/// Read a file's entire contents. Returns Err on failure.
pub fn fs_read(path: impl AsRef<str>) -> Result<String, String> {
    let p = path.as_ref();
    std::fs::read_to_string(p).map_err(|e| format!("Cannot read {}: {}", p, e))
}

/// Write content to a file, creating or overwriting it. Returns Err on failure.
pub fn fs_write(path: impl AsRef<str>, content: impl AsRef<str>) -> Result<(), String> {
    let p = path.as_ref();
    std::fs::write(p, content.as_ref()).map_err(|e| format!("Cannot write {}: {}", p, e))
}

/// True if the path exists (file or directory).
pub fn fs_exists(path: impl AsRef<str>) -> bool {
    std::path::Path::new(path.as_ref()).exists()
}

/// True if the path exists and is a directory.
pub fn fs_is_dir(path: impl AsRef<str>) -> bool {
    std::path::Path::new(path.as_ref()).is_dir()
}
