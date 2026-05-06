// Tests for fs.rs — extracted from #[cfg(test)] mod tests
// Included into fs_mod in lib.rs (after fs.rs is included).

#[test]
fn test_fs_exists_missing() {
    assert!(!fs_exists("/nonexistent/path/xyz"));
}

#[test]
fn test_fs_is_dir_false_for_missing() {
    assert!(!fs_is_dir("/nonexistent/path/xyz"));
}

#[test]
fn test_fs_read_write_roundtrip() {
    let dir = std::env::temp_dir();
    let p = dir.join("homun_fs_test.txt");
    let path_str = p.to_string_lossy().into_owned();
    fs_write(&path_str, "hello").unwrap();
    let content = fs_read(&path_str).unwrap();
    assert_eq!(content, "hello");
    let _ = std::fs::remove_file(&p);
}

#[test]
fn test_fs_read_missing_returns_err() {
    assert!(fs_read("/nonexistent/file.txt").is_err());
}

#[test]
fn test_fs_exists_string_type() {
    assert!(!fs_exists("/nonexistent".to_string()));
}
