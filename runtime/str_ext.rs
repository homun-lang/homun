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
// ============================================================

/// Repeat string `s` exactly `n` times.
/// Returns an empty string when `n` is 0.
pub fn str_repeat(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// Center `s` within a field of `width` characters by padding with spaces.
/// If `s` is already at least `width` chars wide, it is returned unchanged.
/// When the total padding is odd, the extra space goes on the right.
pub fn str_pad_center(s: &str, width: usize) -> String {
    let len = s.chars().count();
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
        // repeating an empty string any number of times stays empty
        assert_eq!(str_repeat("", 10), "");
    }

    // ── str_pad_center ──────────────────────────────────────

    #[test]
    fn test_str_pad_center_even_padding() {
        // "hi" len=2, width=6 → 2 spaces on each side
        let result = str_pad_center("hi", 6);
        assert_eq!(result, "  hi  ");
        assert_eq!(result.chars().count(), 6);
    }

    #[test]
    fn test_str_pad_center_odd_padding() {
        // "hi" len=2, width=7 → 2 left, 3 right (extra goes right)
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
        // string longer than target width → returned unchanged
        assert_eq!(str_pad_center("toolong", 4), "toolong");
    }

    #[test]
    fn test_str_pad_center_empty_string() {
        // centering empty string → all spaces
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
        // width 0 with any string → unchanged (len >= 0 always)
        assert_eq!(str_pad_center("a", 0), "a");
    }

    #[test]
    fn test_str_pad_center_width_one_exact() {
        assert_eq!(str_pad_center("x", 1), "x");
    }
}
