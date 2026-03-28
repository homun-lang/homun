// Tests for chars.rs — extracted from #[cfg(test)] mod tests
// Included into chars_mod in lib.rs (after chars.rs is included).

#[test]
fn test_is_alpha_lowercase() {
    assert!(is_alpha("a"));
    assert!(is_alpha("z"));
    assert!(is_alpha("abc"));
}

#[test]
fn test_is_alpha_uppercase() {
    assert!(is_alpha("A"));
    assert!(is_alpha("Z"));
    assert!(is_alpha("Hello"));
}

#[test]
fn test_is_alpha_false_digit() {
    assert!(!is_alpha("3"));
    assert!(!is_alpha("a1"));
}

#[test]
fn test_is_alpha_false_space() {
    assert!(!is_alpha(" "));
    assert!(!is_alpha("a "));
}

#[test]
fn test_is_alpha_empty() {
    assert!(!is_alpha(""));
}

#[test]
fn test_is_alpha_string_type() {
    assert!(is_alpha("a".to_string()));
    assert!(!is_alpha("3".to_string()));
    assert!(!is_alpha("".to_string()));
}

#[test]
fn test_is_alnum_letters() {
    assert!(is_alnum("a"));
    assert!(is_alnum("Z"));
}

#[test]
fn test_is_alnum_digits() {
    assert!(is_alnum("0"));
    assert!(is_alnum("9"));
    assert!(is_alnum("42"));
}

#[test]
fn test_is_alnum_mixed() {
    assert!(is_alnum("a3"));
    assert!(is_alnum("foo123"));
}

#[test]
fn test_is_alnum_false_special() {
    assert!(!is_alnum("_"));
    assert!(!is_alnum("a_b"));
    assert!(!is_alnum(" "));
}

#[test]
fn test_is_alnum_empty() {
    assert!(!is_alnum(""));
}

#[test]
fn test_is_alnum_string_type() {
    assert!(is_alnum("a3".to_string()));
    assert!(!is_alnum("_".to_string()));
}

#[test]
fn test_is_digit_single() {
    for d in '0'..='9' {
        assert!(is_digit(&d.to_string()));
    }
}

#[test]
fn test_is_digit_multi() {
    assert!(is_digit("123"));
    assert!(is_digit("007"));
}

#[test]
fn test_is_digit_false_alpha() {
    assert!(!is_digit("a"));
    assert!(!is_digit("1a"));
}

#[test]
fn test_is_digit_false_space() {
    assert!(!is_digit(" "));
}

#[test]
fn test_is_digit_empty() {
    assert!(!is_digit(""));
}

#[test]
fn test_is_digit_string_type() {
    assert!(is_digit("7".to_string()));
    assert!(!is_digit("a".to_string()));
}

#[test]
fn test_is_ws_space() {
    assert!(is_ws(" "));
    assert!(is_ws("   "));
}

#[test]
fn test_is_ws_tab() {
    assert!(is_ws("\t"));
}

#[test]
fn test_is_ws_newline() {
    assert!(is_ws("\n"));
    assert!(is_ws("\r\n"));
}

#[test]
fn test_is_ws_false_letter() {
    assert!(!is_ws("a"));
    assert!(!is_ws(" a"));
}

#[test]
fn test_is_ws_false_digit() {
    assert!(!is_ws("1"));
}

#[test]
fn test_is_ws_empty() {
    assert!(!is_ws(""));
}

#[test]
fn test_is_ws_string_type() {
    assert!(is_ws(" ".to_string()));
    assert!(!is_ws("a".to_string()));
}
