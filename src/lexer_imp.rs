// lexer_imp.rs — Type re-exports and helper accessors for lexer.hom.
//
// Importing this file via `use lexer_imp` in lexer.hom sets has_rs_dep=true
// in the homunc sema checker, disabling undefined-reference checks for dep/*
// functions and runtime functions that are available at include!() time in
// lib.rs but unknown to homunc's static checker.
//
// Re-exports:
//   Pos, Token, TokenKind   — from crate::lexer (struct/enum defs stay in Rust)
//
// Pos constructors / accessors:
//   make_pos(line, col) -> Pos          — i64 args (Homun int → usize cast)
//   pos_line(p) -> i64
//   pos_col(p) -> i64
//   pos_inc_col(p) -> Pos               — advance col by 1
//   pos_add_col(p, n) -> Pos            — advance col by n
//   pos_newline(p) -> Pos               — increment line, reset col to 1
//
// Token constructors (value-carrying variants):
//   make_token_int(val, pos) -> Token
//   make_token_float(val, pos) -> Token
//   make_token_bool(val, pos) -> Token
//   make_token_str(val, pos) -> Token
//   make_token_char(val, pos) -> Token
//   make_token_ident(val, pos) -> Token
//   make_token_none(pos) -> Token
//
// Token constructor (parameterless variants via string dispatch):
//   make_token(kind, pos) -> Token
//     Accepts: "Use","Struct","Enum","For","In","While","Do","If","Else",
//              "Match","Break","Continue","And","Or","Not","As","Rec",
//              "Assign","Arrow","FatArrow","Pipe","Dot","Plus","Minus",
//              "Star","Slash","Percent","Eq","Neq","Lt","Gt","Le","Ge",
//              "Colon","Comma","Semi","Underscore","At","Question",
//              "LParen","RParen","LBrace","RBrace","LBracket","RBracket","Eof"
//
// Char-testing helpers:
//   is_alpha(c) -> bool
//   is_digit(c) -> bool
//   is_alnum(c) -> bool      — alphanumeric or '_'
//   is_whitespace(c) -> bool
//   is_newline(c) -> bool

pub use crate::lexer::{Pos, Token, TokenKind};

// ── Pos constructors / accessors ─────────────────────────────────────────────

/// Create a Pos from (line, col).  Homun int is i64; cast to usize here.
pub fn make_pos(line: i64, col: i64) -> Pos {
    Pos {
        line: line as usize,
        col: col as usize,
    }
}

/// Return the line number as i64.
pub fn pos_line(p: Pos) -> i64 {
    p.line as i64
}

/// Return the column number as i64.
pub fn pos_col(p: Pos) -> i64 {
    p.col as i64
}

/// Return a new Pos with col incremented by 1.
pub fn pos_inc_col(p: Pos) -> Pos {
    Pos {
        line: p.line,
        col: p.col + 1,
    }
}

/// Return a new Pos with col incremented by n.
pub fn pos_add_col(p: Pos, n: i64) -> Pos {
    Pos {
        line: p.line,
        col: p.col + n as usize,
    }
}

/// Return a new Pos for the start of the next line (line+1, col=1).
pub fn pos_newline(p: Pos) -> Pos {
    Pos {
        line: p.line + 1,
        col: 1,
    }
}

// ── Value-carrying token constructors ────────────────────────────────────────

pub fn make_token_int(val: i64, pos: Pos) -> Token {
    Token {
        kind: TokenKind::Int(val),
        pos,
    }
}

pub fn make_token_float(val: f64, pos: Pos) -> Token {
    Token {
        kind: TokenKind::Float(val),
        pos,
    }
}

pub fn make_token_bool(val: bool, pos: Pos) -> Token {
    Token {
        kind: TokenKind::Bool(val),
        pos,
    }
}

pub fn make_token_str(val: String, pos: Pos) -> Token {
    Token {
        kind: TokenKind::Str(val),
        pos,
    }
}

pub fn make_token_char(val: char, pos: Pos) -> Token {
    Token {
        kind: TokenKind::Char(val),
        pos,
    }
}

pub fn make_token_ident(val: String, pos: Pos) -> Token {
    Token {
        kind: TokenKind::Ident(val),
        pos,
    }
}

pub fn make_token_none(pos: Pos) -> Token {
    Token {
        kind: TokenKind::None,
        pos,
    }
}

// ── Parameterless-variant constructor (string dispatch) ───────────────────────

/// Construct a Token whose kind carries no payload.
/// Pass the variant name as a string (e.g. "Assign", "LParen", "Eof").
/// Panics on unknown kind name — intended for use with literal constants only.
pub fn make_token(kind: String, pos: Pos) -> Token {
    let k = match kind.as_str() {
        // Keywords
        "Use" => TokenKind::Use,
        "Struct" => TokenKind::Struct,
        "Enum" => TokenKind::Enum,
        "For" => TokenKind::For,
        "In" => TokenKind::In,
        "While" => TokenKind::While,
        "Do" => TokenKind::Do,
        "If" => TokenKind::If,
        "Else" => TokenKind::Else,
        "Match" => TokenKind::Match,
        "Break" => TokenKind::Break,
        "Continue" => TokenKind::Continue,
        "And" => TokenKind::And,
        "Or" => TokenKind::Or,
        "Not" => TokenKind::Not,
        "As" => TokenKind::As,
        "Rec" => TokenKind::Rec,
        // Operators
        "Assign" => TokenKind::Assign,
        "Arrow" => TokenKind::Arrow,
        "FatArrow" => TokenKind::FatArrow,
        "Pipe" => TokenKind::Pipe,
        "Dot" => TokenKind::Dot,
        "Plus" => TokenKind::Plus,
        "Minus" => TokenKind::Minus,
        "Star" => TokenKind::Star,
        "Slash" => TokenKind::Slash,
        "Percent" => TokenKind::Percent,
        "Eq" => TokenKind::Eq,
        "Neq" => TokenKind::Neq,
        "Lt" => TokenKind::Lt,
        "Gt" => TokenKind::Gt,
        "Le" => TokenKind::Le,
        "Ge" => TokenKind::Ge,
        "Colon" => TokenKind::Colon,
        "Comma" => TokenKind::Comma,
        "Semi" => TokenKind::Semi,
        "Underscore" => TokenKind::Underscore,
        "At" => TokenKind::At,
        "Question" => TokenKind::Question,
        // Delimiters
        "LParen" => TokenKind::LParen,
        "RParen" => TokenKind::RParen,
        "LBrace" => TokenKind::LBrace,
        "RBrace" => TokenKind::RBrace,
        "LBracket" => TokenKind::LBracket,
        "RBracket" => TokenKind::RBracket,
        "Eof" => TokenKind::Eof,
        _ => panic!("make_token: unknown parameterless kind '{}'", kind),
    };
    Token { kind: k, pos }
}

// ── Char-testing helpers ──────────────────────────────────────────────────────

/// True if c is an alphabetic character (Unicode-aware, matches Homun ident start).
pub fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
}

/// True if c is an ASCII decimal digit.
pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

/// True if c is alphanumeric or underscore (matches Homun ident continuation).
pub fn is_alnum(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// True if c is Unicode whitespace.
pub fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

/// True if c is a newline character.
pub fn is_newline(c: char) -> bool {
    c == '\n'
}
