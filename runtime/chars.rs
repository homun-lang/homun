// ============================================================
// Homun Runtime — chars.rs: Character Classification
// Part B3 — stdlib, no external crates required.
//
// Usage in .hom:
//   use chars
//   is_alpha("a")   // true
//   is_alnum("3")   // true
//   is_digit("7")   // true
//   is_ws(" ")      // true
//
// Functions accept impl AsRef<str> so they work with both:
//   - &str literals (used in Rust tests)
//   - String values (emitted by homunc codegen for .hom string args,
//     which always calls .to_string() on string literals)
// Returns false for empty input.
// ============================================================

/// True if every character in `s` is alphabetic (Unicode).
pub fn is_alpha(s: impl AsRef<str>) -> bool {
    let s = s.as_ref();
    !s.is_empty() && s.chars().all(|c| c.is_alphabetic())
}

/// True if every character in `s` is alphanumeric (Unicode).
pub fn is_alnum(s: impl AsRef<str>) -> bool {
    let s = s.as_ref();
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric())
}

/// True if every character in `s` is an ASCII decimal digit (0–9).
pub fn is_digit(s: impl AsRef<str>) -> bool {
    let s = s.as_ref();
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

/// True if every character in `s` is ASCII whitespace (space, tab, \n, \r).
pub fn is_ws(s: impl AsRef<str>) -> bool {
    let s = s.as_ref();
    !s.is_empty() && s.chars().all(|c| c.is_ascii_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_alpha ────────────────────────────────────────────
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

    // Verify String type works (as emitted by homunc codegen)
    #[test]
    fn test_is_alpha_string_type() {
        assert!(is_alpha("a".to_string()));
        assert!(!is_alpha("3".to_string()));
        assert!(!is_alpha("".to_string()));
    }

    // ── is_alnum ────────────────────────────────────────────
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

    // Verify String type works (as emitted by homunc codegen)
    #[test]
    fn test_is_alnum_string_type() {
        assert!(is_alnum("a3".to_string()));
        assert!(!is_alnum("_".to_string()));
    }

    // ── is_digit ────────────────────────────────────────────
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

    // Verify String type works (as emitted by homunc codegen)
    #[test]
    fn test_is_digit_string_type() {
        assert!(is_digit("7".to_string()));
        assert!(!is_digit("a".to_string()));
    }

    // ── is_ws ───────────────────────────────────────────────
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

    // Verify String type works (as emitted by homunc codegen)
    #[test]
    fn test_is_ws_string_type() {
        assert!(is_ws(" ".to_string()));
        assert!(!is_ws("a".to_string()));
    }
}
