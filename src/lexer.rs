/// Lexer: type definitions for Homun tokens.
/// The lex() function has moved to lexer_hom (compiled from src/lexer.hom).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Pos,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Char(char),
    None,
    // Identifiers & Keywords
    Ident(String),
    Use,
    Struct,
    Enum,
    For,
    In,
    While,
    Do,
    If,
    Else,
    Match,
    Break,
    Continue,
    And,
    Or,
    Not,
    As,
    Rec,
    // Operators
    MutAssign, // ::=
    Assign,    // :=
    Arrow,     // ->
    FatArrow,  // =>
    Pipe,      // |
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,  // ==
    Neq, // !=
    Lt,
    Gt,
    Le, // <=
    Ge, // >=
    Colon,
    Comma,
    Semi,
    Underscore,
    At,
    Question,
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Eof,
}
