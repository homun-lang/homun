// ============================================================
// Homun Runtime — str_ext.rs: String Extras
// Part B5 — stdlib, no external crates required.
//
// Usage in .hom:
//   use str_ext
//
//   s := str_repeat(" ", 10)       // "          "
//   s := str_pad_center("hi", 10)  // "    hi    "
//
// Used by: Canvas padding, node label centering.
//
// Type notes:
//   - s accepts impl AsRef<str> to work with both &str literals (Rust
//     tests) and String values (homunc codegen emits .to_string() on
//     all string literals when passing as function arguments).
//   - n and width use i32 to match .hom's int type; negative values
//     are treated as 0.
// ============================================================

/// Repeat string `s` exactly `n` times.
/// Returns an empty string when `n` <= 0.
pub fn str_repeat(s: impl AsRef<str>, n: i32) -> String {
    if n <= 0 {
        return String::new();
    }
    s.as_ref().repeat(n as usize)
}

/// Center `s` within a field of `width` characters by padding with spaces.
/// If `s` is already at least `width` chars wide, it is returned unchanged.
/// When the total padding is odd, the extra space goes on the right.
/// width <= 0 returns `s` unchanged.
pub fn str_pad_center(s: impl AsRef<str>, width: i32) -> String {
    let s = s.as_ref();
    let len = s.chars().count();
    let width = if width <= 0 { 0usize } else { width as usize };
    if len >= width {
        return s.to_string();
    }
    let total_pad = width - len;
    let left = total_pad / 2;
    let right = total_pad - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── str_repeat ──────────────────────────────────────────

    #[test]
    fn test_str_repeat_zero() {
        assert_eq!(str_repeat("abc", 0), "");
    }

    #[test]
    fn test_str_repeat_negative() {
        assert_eq!(str_repeat("abc", -1), "");
    }

    #[test]
    fn test_str_repeat_once() {
        assert_eq!(str_repeat("hello", 1), "hello");
    }

    #[test]
    fn test_str_repeat_multiple() {
        assert_eq!(str_repeat("ab", 3), "ababab");
    }

    #[test]
    fn test_str_repeat_single_char() {
        assert_eq!(str_repeat(" ", 5), "     ");
        assert_eq!(str_repeat("-", 4), "----");
    }

    #[test]
    fn test_str_repeat_empty_string() {
        assert_eq!(str_repeat("", 10), "");
    }

    // Verify String type works (as emitted by homunc codegen)
    #[test]
    fn test_str_repeat_string_type() {
        assert_eq!(str_repeat(" ".to_string(), 3), "   ");
        assert_eq!(str_repeat("-".to_string(), 2), "--");
    }

    // ── str_pad_center ──────────────────────────────────────

    #[test]
    fn test_str_pad_center_even_padding() {
        let result = str_pad_center("hi", 6);
        assert_eq!(result, "  hi  ");
        assert_eq!(result.chars().count(), 6);
    }

    #[test]
    fn test_str_pad_center_odd_padding() {
        let result = str_pad_center("hi", 7);
        assert_eq!(result, "  hi   ");
        assert_eq!(result.chars().count(), 7);
    }

    #[test]
    fn test_str_pad_center_already_full_width() {
        assert_eq!(str_pad_center("hello", 5), "hello");
    }

    #[test]
    fn test_str_pad_center_wider_than_width() {
        assert_eq!(str_pad_center("toolong", 4), "toolong");
    }

    #[test]
    fn test_str_pad_center_empty_string() {
        let result = str_pad_center("", 4);
        assert_eq!(result, "    ");
    }

    #[test]
    fn test_str_pad_center_single_char_even_width() {
        let result = str_pad_center("X", 5);
        assert_eq!(result, "  X  ");
        assert_eq!(result.chars().count(), 5);
    }

    #[test]
    fn test_str_pad_center_width_zero() {
        assert_eq!(str_pad_center("a", 0), "a");
    }

    #[test]
    fn test_str_pad_center_width_one_exact() {
        assert_eq!(str_pad_center("x", 1), "x");
    }

    // Verify String type works (as emitted by homunc codegen)
    #[test]
    fn test_str_pad_center_string_type() {
        assert_eq!(str_pad_center("hi".to_string(), 6), "  hi  ");
        assert_eq!(str_pad_center("".to_string(), 4), "    ");
    }
}
