// Tests for re.rs — extracted from #[cfg(test)] mod tests
// Included directly into hom_tests in lib.rs (re functions from runtime).

#[test]
fn test_re_is_match_digit_present() {
    assert!(re_is_match("[0-9]+", "hello 42 world"));
}

#[test]
fn test_re_is_match_digit_absent() {
    assert!(!re_is_match("[0-9]+", "no digits here"));
}

#[test]
fn test_re_is_match_identifier() {
    assert!(re_is_match("[a-zA-Z_][a-zA-Z0-9_]*", "foo_bar"));
    assert!(!re_is_match("[a-zA-Z_][a-zA-Z0-9_]*", "123"));
}

#[test]
fn test_re_is_match_empty_text() {
    assert!(!re_is_match("[a-z]+", ""));
}

#[test]
fn test_re_is_match_full_string() {
    assert!(re_is_match("^hello$", "hello"));
    assert!(!re_is_match("^hello$", "hello world"));
}

#[test]
fn test_re_is_match_string_type() {
    assert!(re_is_match("[0-9]+".to_string(), "hello 42".to_string()));
    assert!(!re_is_match("[0-9]+".to_string(), "no digits".to_string()));
}

#[test]
fn test_re_match_at_start() {
    let (matched, text, end) = re_match("[a-zA-Z_][a-zA-Z0-9_]*", "hello world", 0);
    assert!(matched);
    assert_eq!(text, "hello");
    assert_eq!(end, 5);
}

#[test]
fn test_re_match_at_offset() {
    let (matched, text, end) = re_match("[a-zA-Z_][a-zA-Z0-9_]*", "hello world", 6);
    assert!(matched);
    assert_eq!(text, "world");
    assert_eq!(end, 11);
}

#[test]
fn test_re_match_anchored_mid_word() {
    let (matched, text, end) = re_match("[a-zA-Z_]+", "hello world", 3);
    assert!(matched);
    assert_eq!(text, "lo");
    assert_eq!(end, 5);
}

#[test]
fn test_re_match_no_match_at_pos() {
    let (matched, text, end) = re_match("[0-9]+", "hello 42", 0);
    assert!(!matched);
    assert_eq!(text, "");
    assert_eq!(end, 0);
}

#[test]
fn test_re_match_digits_at_offset() {
    let (matched, text, end) = re_match("[0-9]+", "abc 123 def", 4);
    assert!(matched);
    assert_eq!(text, "123");
    assert_eq!(end, 7);
}

#[test]
fn test_re_match_empty_text() {
    let (matched, _, _) = re_match("[a-z]+", "", 0);
    assert!(!matched);
}

#[test]
fn test_re_match_pos_at_end() {
    let text = "hello";
    let (matched, _, _) = re_match("[a-z]+", text, text.len() as i32);
    assert!(!matched);
}

#[test]
fn test_re_match_pos_beyond_end() {
    let text = "hello";
    let beyond = (text.len() + 1) as i32;
    let (matched, _, ret_pos) = re_match("[a-z]+", text, beyond);
    assert!(!matched);
    assert_eq!(ret_pos, beyond);
}

#[test]
fn test_re_match_cached_repeated_calls() {
    let pattern = "[a-z]+";
    let text = "foobar";
    for _ in 0..10 {
        let (matched, m, end) = re_match(pattern, text, 0);
        assert!(matched);
        assert_eq!(m, "foobar");
        assert_eq!(end, 6);
    }
}

#[test]
fn test_re_is_match_cached_repeated_calls() {
    let pattern = r"\d+";
    for _ in 0..10 {
        assert!(re_is_match(pattern, "x99y"));
    }
}

#[test]
fn test_re_match_string_type() {
    let (matched, text, end) = re_match(
        "[a-zA-Z_][a-zA-Z0-9_]*".to_string(),
        "hello world".to_string(),
        0,
    );
    assert!(matched);
    assert_eq!(text, "hello");
    assert_eq!(end, 5);
}

#[test]
fn test_re_match_node_id_pattern() {
    let pattern = "[a-zA-Z_][a-zA-Z0-9_]*";
    let (matched, m, end) = re_match(pattern, "node_A123 rest", 0);
    assert!(matched);
    assert_eq!(m, "node_A123");
    assert_eq!(end, 9);
}

#[test]
fn test_re_match_arrow_pattern() {
    let (matched, m, end) = re_match("-->", "--> next", 0);
    assert!(matched);
    assert_eq!(m, "-->");
    assert_eq!(end, 3);
}

#[test]
fn test_re_match_quoted_string() {
    let pattern = r#""[^"]*""#;
    let (matched, m, end) = re_match(pattern, r#""hello" world"#, 0);
    assert!(matched);
    assert_eq!(m, "\"hello\"");
    assert_eq!(end, 7);
}

#[test]
fn test_re_match_whitespace_skip() {
    let pattern = r"[ \t]+";
    let (matched, m, end) = re_match(pattern, "  \tabc", 0);
    assert!(matched);
    assert_eq!(m, "  \t");
    assert_eq!(end, 3);
}

#[test]
fn test_re_match_multiple_patterns_cached() {
    let text = "flowchart LR";
    let (m1, t1, _) = re_match("[a-z]+", text, 0);
    let (m2, t2, _) = re_match("[A-Z]+", text, 10);
    assert!(m1);
    assert_eq!(t1, "flowchart");
    assert!(m2);
    assert_eq!(t2, "LR");
}
