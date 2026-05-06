// Tests for path.rs — extracted from #[cfg(test)] mod tests
// Included into path_mod in lib.rs (after path.rs is included).

#[test]
fn test_path_join_basic() {
    let result = path_join("/home/user", "file.txt");
    assert!(result.ends_with("file.txt"));
}

#[test]
fn test_path_parent_basic() {
    assert_eq!(path_parent("/home/user/file.txt"), "/home/user");
}

#[test]
fn test_path_parent_root() {
    assert_eq!(path_parent("/"), "");
}

#[test]
fn test_path_strip_prefix_match() {
    assert_eq!(path_strip_prefix("/a/b/c", "/a"), "b/c");
}

#[test]
fn test_path_strip_prefix_no_match() {
    assert_eq!(path_strip_prefix("/a/b/c", "/x"), "/a/b/c");
}

#[test]
fn test_path_join_string_type() {
    let d = "/tmp".to_string();
    let n = "out.rs".to_string();
    let r = path_join(d, n);
    assert!(r.ends_with("out.rs"));
}
