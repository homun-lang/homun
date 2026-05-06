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

