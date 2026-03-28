// Tests for str_ext.rs — extracted from #[cfg(test)] mod tests
// Included into str_ext_mod in lib.rs (after str_ext.rs is included).

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

#[test]
fn test_str_repeat_string_type() {
    assert_eq!(str_repeat(" ".to_string(), 3), "   ");
    assert_eq!(str_repeat("-".to_string(), 2), "--");
}

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

#[test]
fn test_str_pad_center_string_type() {
    assert_eq!(str_pad_center("hi".to_string(), 6), "  hi  ");
    assert_eq!(str_pad_center("".to_string(), 4), "    ");
}
