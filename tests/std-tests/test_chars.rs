// Tests for chars.rs — extracted from #[cfg(test)] mod tests
// Included into chars_mod in lib.rs (after chars.rs is included).
//
// Helpers inspect the FIRST character only — multi-char inputs are
// classified by their first char (not by all chars). Empty → false.

#[test]
fn test_is_alpha_lowercase() {
    assert!(is_alpha("a".to_string()));
    assert!(is_alpha("z".to_string()));
}

#[test]
fn test_is_alpha_uppercase() {
    assert!(is_alpha("A".to_string()));
    assert!(is_alpha("Z".to_string()));
}

#[test]
fn test_is_alpha_multi_char_uses_first() {
    // First char alpha → true (regardless of remaining chars).
    assert!(is_alpha("abc".to_string()));
    assert!(is_alpha("a1".to_string()));
    assert!(is_alpha("a ".to_string()));
}

#[test]
fn test_is_alpha_false_digit_first() {
    assert!(!is_alpha("3".to_string()));
    assert!(!is_alpha("1abc".to_string()));
}

#[test]
fn test_is_alpha_false_space_first() {
    assert!(!is_alpha(" ".to_string()));
    assert!(!is_alpha(" a".to_string()));
}

#[test]
fn test_is_alpha_empty() {
    assert!(!is_alpha("".to_string()));
}

#[test]
fn test_is_alnum_letters() {
    assert!(is_alnum("a".to_string()));
    assert!(is_alnum("Z".to_string()));
}

#[test]
fn test_is_alnum_digits() {
    assert!(is_alnum("0".to_string()));
    assert!(is_alnum("9".to_string()));
}

#[test]
fn test_is_alnum_underscore() {
    // is_alnum treats '_' as alnum (matches Homun ident continuation).
    assert!(is_alnum("_".to_string()));
    assert!(is_alnum("_foo".to_string()));
}

#[test]
fn test_is_alnum_multi_char_uses_first() {
    assert!(is_alnum("a3".to_string()));
    assert!(is_alnum("foo123".to_string()));
    assert!(is_alnum("a_b".to_string()));
}

#[test]
fn test_is_alnum_false_space_first() {
    assert!(!is_alnum(" ".to_string()));
    assert!(!is_alnum(" a".to_string()));
}

#[test]
fn test_is_alnum_empty() {
    assert!(!is_alnum("".to_string()));
}

#[test]
fn test_is_digit_single() {
    for d in '0'..='9' {
        assert!(is_digit(d.to_string()));
    }
}

#[test]
fn test_is_digit_multi_char_uses_first() {
    assert!(is_digit("123".to_string()));
    assert!(is_digit("007".to_string()));
    assert!(is_digit("1a".to_string()));
}

#[test]
fn test_is_digit_false_alpha_first() {
    assert!(!is_digit("a".to_string()));
    assert!(!is_digit("a1".to_string()));
}

#[test]
fn test_is_digit_false_space_first() {
    assert!(!is_digit(" ".to_string()));
}

#[test]
fn test_is_digit_empty() {
    assert!(!is_digit("".to_string()));
}

#[test]
fn test_is_whitespace_space() {
    assert!(is_whitespace(" ".to_string()));
}

#[test]
fn test_is_whitespace_tab() {
    assert!(is_whitespace("\t".to_string()));
}

#[test]
fn test_is_whitespace_newline() {
    assert!(is_whitespace("\n".to_string()));
    assert!(is_whitespace("\r".to_string()));
}

#[test]
fn test_is_whitespace_multi_char_uses_first() {
    assert!(is_whitespace("   ".to_string()));
    assert!(is_whitespace(" a".to_string()));
}

#[test]
fn test_is_whitespace_false_letter_first() {
    assert!(!is_whitespace("a".to_string()));
    assert!(!is_whitespace("a ".to_string()));
}

#[test]
fn test_is_whitespace_false_digit_first() {
    assert!(!is_whitespace("1".to_string()));
}

#[test]
fn test_is_whitespace_empty() {
    assert!(!is_whitespace("".to_string()));
}

#[test]
fn test_is_newline_true() {
    assert!(is_newline("\n".to_string()));
}

#[test]
fn test_is_newline_false() {
    assert!(!is_newline("\r".to_string()));
    assert!(!is_newline(" ".to_string()));
    assert!(!is_newline("a".to_string()));
    assert!(!is_newline("\r\n".to_string()));
    assert!(!is_newline("".to_string()));
}
