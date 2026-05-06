// ============================================================
// Homun Runtime — re.rs: Regex Pattern Matching + Caching
// Part B2 — REQUIRES external crate: regex = "1"
//
// DEPENDENCY NOTE: Projects that `use re` must add to Cargo.toml:
//   [dependencies]
//   regex = "1"
//
// Usage in .hom:
//   use re
//
//   matched, text, end := re_match("[a-zA-Z_][a-zA-Z0-9_]*", src, pos)
//   if (matched) do {
//     print("found: ${text} ending at ${end}")
//   }
//
//   ok := re_is_match("[0-9]+", "hello 42 world")
//
// API:
//   re_match(pattern, text, pos) -> (bool, String, int)
//     Anchored match at byte offset `pos`.  Returns (matched, matched_str, end_pos).
//     Equivalent to Python's re.compile(pattern).match(text, pos).
//     pos and end_pos use i32 to match .hom's int type.
//
//   re_is_match(pattern, text) -> bool
//     True if pattern matches anywhere in text.
//     Equivalent to Python's re.search(pattern, text) is not None.
//
// Both functions accept impl AsRef<str> for pattern and text, so they
// work with both &str literals (Rust tests) and String values (homunc
// codegen emits .to_string() on string literals when passing as args).
//
// Caching:
//   Patterns are compiled once per thread and cached in a thread-local
//   HashMap<String, Regex>.  Subsequent calls with the same pattern string
//   reuse the compiled Regex object.
// ============================================================

use regex::Regex;
use std::cell::RefCell;

thread_local! {
    static REGEX_CACHE: RefCell<std::collections::HashMap<String, Regex>> =
        RefCell::new(std::collections::HashMap::new());
}

/// Get or compile a Regex for `pattern`, using the thread-local cache.
/// Panics with a clear message if the pattern is invalid.
fn get_or_compile(pattern: &str) -> Regex {
    REGEX_CACHE.with(|cache| {
        let mut map = cache.borrow_mut();
        if !map.contains_key(pattern) {
            let re = Regex::new(pattern)
                .unwrap_or_else(|e| panic!("re: invalid regex pattern {:?}: {}", pattern, e));
            map.insert(pattern.to_string(), re);
        }
        // Regex is cheap to clone (Arc-backed)
        map.get(pattern).unwrap().clone()
    })
}

/// Match `pattern` anchored at byte offset `pos` in `text`.
///
/// Returns `(matched, captured_text, end_pos)` where:
/// - `matched`        — true if the pattern matches starting exactly at `pos`
/// - `captured_text`  — the matched substring (empty string when no match)
/// - `end_pos`        — byte offset just after the match (`pos` when no match)
///
/// `pos` and `end_pos` are `i32` to match .hom's `int` type.
/// Accepts impl AsRef<str> for pattern and text.
///
/// Equivalent to Python's `re.compile(pattern).match(text, pos)`.
pub fn re_match(pattern: impl AsRef<str>, text: impl AsRef<str>, pos: i32) -> (bool, String, i32) {
    let pattern = pattern.as_ref();
    let text = text.as_ref();
    let pos = pos as usize;
    if pos > text.len() {
        return (false, String::new(), pos as i32);
    }
    let re = get_or_compile(pattern);
    let haystack = &text[pos..];
    match re.find(haystack) {
        Some(m) if m.start() == 0 => {
            let matched_str = m.as_str().to_string();
            let end = (pos + m.end()) as i32;
            (true, matched_str, end)
        }
        _ => (false, String::new(), pos as i32),
    }
}

/// Return `true` if `pattern` matches anywhere in `text`.
///
/// Accepts impl AsRef<str> for pattern and text.
/// Equivalent to Python's `re.search(pattern, text) is not None`.
pub fn re_is_match(pattern: impl AsRef<str>, text: impl AsRef<str>) -> bool {
    let pattern = pattern.as_ref();
    let text = text.as_ref();
    let re = get_or_compile(pattern);
    re.is_match(text)
}

