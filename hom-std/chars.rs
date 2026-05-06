// ============================================================
// Homun Runtime — chars.rs: Character Classification
// Part B3 — stdlib, no external crates required.
//
// Usage in .hom:
//   use chars
//   is_alpha("a")        // true
//   is_alnum("3")        // true
//   is_digit("7")        // true
//   is_whitespace(" ")   // true
//   is_newline("\n")     // true
//
// All helpers take a 1-char String (as emitted by homunc codegen for .hom
// string args, which always calls .to_string() on string literals). They
// inspect the FIRST character only — multi-char inputs are classified by
// their first char. Returns false for empty input.
//
// (Tests live in tests/std-tests/test_chars.rs.)
// ============================================================

/// True if c starts with an alphabetic character (Unicode-aware,
/// matches Homun identifier start).
pub fn is_alpha(c: String) -> bool {
    c.chars().next().is_some_and(|ch| ch.is_alphabetic())
}

/// True if c starts with an ASCII decimal digit (0–9).
pub fn is_digit(c: String) -> bool {
    c.chars().next().is_some_and(|ch| ch.is_ascii_digit())
}

/// True if c starts with an alphanumeric character or underscore '_'
/// (matches Homun identifier continuation). Note: Unicode's
/// `char::is_alphanumeric` does NOT include `_`, but this helper does
/// — it mirrors the lexer's ident-continuation rule.
pub fn is_alnum(c: String) -> bool {
    c.chars()
        .next()
        .is_some_and(|ch| ch.is_alphanumeric() || ch == '_')
}

/// True if c starts with a Unicode whitespace character.
pub fn is_whitespace(c: String) -> bool {
    c.chars().next().is_some_and(|ch| ch.is_whitespace())
}

/// True if c is exactly the newline character "\n".
pub fn is_newline(c: String) -> bool {
    c == "\n"
}
