// parser_imp.rs — Rust helpers for parser.hom.
//
// Thread-local state (pos, err, gensym_counter) lives in parser.hom as
// @thread_local bindings; the generated _get/_set accessors are called here.
// PARSE_TOKENS stays in Rust because it needs a concrete Vec<Token> type.
//
// Public API groups:
//   Token inspection:  ps_peek_kind, ps_peek_ident, ps_peek_int, ...
//   Token matching:    ps_check, ps_consume, ps_expect, ps_same_line
//   AST constructors:  mk_expr_* (Box-wrapping variants), mk_type_list/dict/set, mk_variantdef_multi/positional
//   Option helpers:    some_expr, none_expr, some_type, none_type, ...
//   Utility:           split_block, names_to_pats, is_upper_first_str

use std::cell::RefCell;

// ─── Thread-local token list (pos/err/gensym live in parser.hom) ────────────

thread_local! {
    static PARSE_TOKENS: RefCell<Vec<Token>> = const { RefCell::new(vec![]) };
}

fn has_err_internal() -> bool {
    !parse_err_get().is_empty()
}

fn peek_token_internal() -> Token {
    PARSE_TOKENS.with(|t| {
        let tokens = t.borrow();
        let pos = parse_pos_get() as usize;
        let idx = pos.min(tokens.len() - 1);
        tokens[idx].clone()
    })
}

fn advance_internal() {
    let pos = parse_pos_get() as usize;
    let len = PARSE_TOKENS.with(|t| t.borrow().len());
    if pos < len {
        parse_pos_set(parse_pos_get() + 1);
    }
}

// TokenKind has @derive(Debug) in lexer.hom — extract variant name from "{:?}"
fn token_kind_str(kind: &TokenKind) -> String {
    format!("{:?}", kind)
        .split('(')
        .next()
        .unwrap_or("Eof")
        .to_string()
}

// Thin Rust wrapper — sets parse_err back to empty string.
// Kept here instead of .hom because assigning "" to a @thread_local String
// requires String::new() (not a &str literal), which the current bootstrap
// codegen doesn't add automatically for the BindMut→set path.
pub fn ps_clear_err() {
    parse_err_set(String::new());
}

// ─── Token inspection ───────────────────────────────────────────────────────

pub fn ps_peek_kind() -> String {
    if has_err_internal() {
        return "Eof".to_string();
    }
    let t = peek_token_internal();
    token_kind_str(&t.kind)
}

pub fn ps_peek_ident() -> String {
    let t = peek_token_internal();
    match &t.kind {
        TokenKind::Ident(n) => n.clone(),
        _ => String::new(),
    }
}

pub fn ps_peek_int() -> i64 {
    let t = peek_token_internal();
    match t.kind {
        TokenKind::Int(n) => n as i64,
        _ => 0,
    }
}

pub fn ps_peek_float() -> f64 {
    let t = peek_token_internal();
    match t.kind {
        TokenKind::Float(f) => f as f64,
        _ => 0.0,
    }
}

pub fn ps_peek_bool() -> bool {
    let t = peek_token_internal();
    match t.kind {
        TokenKind::Bool(b) => b,
        _ => false,
    }
}

pub fn ps_peek_str() -> String {
    let t = peek_token_internal();
    match &t.kind {
        TokenKind::Str(s) => s.clone(),
        _ => String::new(),
    }
}

pub fn ps_peek_char() -> String {
    let t = peek_token_internal();
    match t.kind {
        TokenKind::Char(c) => c.to_string(),
        _ => String::new(),
    }
}

// ─── Token matching ─────────────────────────────────────────────────────────

pub fn ps_check(kind: String) -> bool {
    if has_err_internal() {
        return false;
    }
    ps_peek_kind() == kind
}

pub fn ps_consume(kind: String) -> bool {
    if has_err_internal() {
        return false;
    }
    if ps_peek_kind() == kind {
        advance_internal();
        true
    } else {
        false
    }
}

pub fn ps_expect(kind: String) {
    if has_err_internal() {
        return;
    }
    let actual = ps_peek_kind();
    if actual == kind {
        advance_internal();
    } else {
        ps_set_err(format!("Expected {} but got {}", kind, actual));
    }
}

/// True if the current token is on the same line as the previous token.
pub fn ps_same_line() -> bool {
    if has_err_internal() {
        return false;
    }
    PARSE_TOKENS.with(|t| {
        let tokens = t.borrow();
        let pos = parse_pos_get() as usize;
        let cur = pos.min(tokens.len() - 1);
        cur > 0 && tokens[cur].pos.line == tokens[cur - 1].pos.line
    })
}

// ─── Advance + extract ──────────────────────────────────────────────────────

/// Advance and return the ident name. Sets error if current token is not Ident.
pub fn ps_advance_ident() -> String {
    if has_err_internal() {
        return String::new();
    }
    let t = peek_token_internal();
    match &t.kind {
        TokenKind::Ident(n) => {
            let name = n.clone();
            advance_internal();
            name
        }
        _ => {
            ps_set_err(format!("Expected identifier, got {}", token_kind_str(&t.kind)));
            String::new()
        }
    }
}

/// Advance and return the kind string of the consumed token.
pub fn ps_advance_kind() -> String {
    if has_err_internal() {
        return "Eof".to_string();
    }
    let t = peek_token_internal();
    let k = token_kind_str(&t.kind);
    advance_internal();
    k
}

// ─── BinOp / UnOp from string ───────────────────────────────────────────────

fn str_to_binop(s: &str) -> BinOp {
    match s {
        "Add" => BinOp::Add,
        "Sub" => BinOp::Sub,
        "Mul" => BinOp::Mul,
        "Div" => BinOp::Div,
        "Mod" => BinOp::Mod,
        "Eq" => BinOp::Eq,
        "Neq" => BinOp::Neq,
        "Lt" => BinOp::Lt,
        "Gt" => BinOp::Gt,
        "Le" => BinOp::Le,
        "Ge" => BinOp::Ge,
        "And" => BinOp::And,
        "Or" => BinOp::Or,
        "In" => BinOp::In,
        "NotIn" => BinOp::NotIn,
        _ => panic!("str_to_binop: unknown '{}'", s),
    }
}

fn str_to_unop(s: &str) -> UnOp {
    match s {
        "Not" => UnOp::Not,
        "Neg" => UnOp::Neg,
        _ => panic!("str_to_unop: unknown '{}'", s),
    }
}

// ─── AST constructors: Expr ─────────────────────────────────────────────────

pub fn mk_expr_char_from_str(s: String) -> Expr {
    Expr::Char(s.chars().next().unwrap_or('\0'))
}
pub fn mk_expr_field(base: Expr, name: String) -> Expr {
    Expr::Field(Box::new(base), name)
}
pub fn mk_expr_index(base: Expr, idx: Expr) -> Expr {
    Expr::Index(Box::new(base), Box::new(idx))
}
pub fn mk_expr_slice(
    base: Expr,
    from: Option<Expr>,
    to: Option<Expr>,
    step: Option<Expr>,
) -> Expr {
    Expr::Slice(
        Box::new(base),
        from.map(Box::new),
        to.map(Box::new),
        step.map(Box::new),
    )
}
pub fn mk_expr_binop(op: String, lhs: Expr, rhs: Expr) -> Expr {
    Expr::BinOp(str_to_binop(&op), Box::new(lhs), Box::new(rhs))
}
pub fn mk_expr_unop(op: String, expr: Expr) -> Expr {
    Expr::UnOp(str_to_unop(&op), Box::new(expr))
}
pub fn mk_expr_pipe(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Pipe(Box::new(lhs), Box::new(rhs))
}
pub fn mk_expr_lambda(
    params: Vec<Param>,
    ret_ty: Option<TypeExpr>,
    void_mark: Option<TypeExpr>,
    stmts: Vec<Stmt>,
    final_expr: Expr,
) -> Expr {
    Expr::Lambda {
        generics: vec![],
        params,
        ret_ty,
        void_mark,
        stmts,
        final_expr: Box::new(final_expr),
    }
}

pub fn mk_expr_lambda_generics(
    generics: Vec<String>,
    params: Vec<Param>,
    ret_ty: Option<TypeExpr>,
    void_mark: Option<TypeExpr>,
    stmts: Vec<Stmt>,
    final_expr: Expr,
) -> Expr {
    Expr::Lambda {
        generics,
        params,
        ret_ty,
        void_mark,
        stmts,
        final_expr: Box::new(final_expr),
    }
}
pub fn mk_expr_call(func: Expr, args: Vec<Expr>) -> Expr {
    Expr::Call(Box::new(func), args)
}
pub fn mk_expr_if(
    cond: Expr,
    then_stmts: Vec<Stmt>,
    then_expr: Expr,
    else_clause: Option<(Vec<Stmt>, Box<Expr>)>,
) -> Expr {
    Expr::If(Box::new(cond), then_stmts, Box::new(then_expr), else_clause)
}
pub fn mk_expr_match(scrutinee: Expr, arms: Vec<MatchArm>) -> Expr {
    Expr::Match(Box::new(scrutinee), arms)
}
pub fn mk_expr_for(var: String, iter: Expr, stmts: Vec<Stmt>, final_expr: Option<Expr>) -> Expr {
    Expr::For(var, Box::new(iter), stmts, final_expr.map(Box::new))
}
pub fn mk_expr_while(cond: Expr, stmts: Vec<Stmt>, final_expr: Option<Expr>) -> Expr {
    Expr::While(Box::new(cond), stmts, final_expr.map(Box::new))
}
pub fn mk_expr_block(stmts: Vec<Stmt>, final_expr: Expr) -> Expr {
    Expr::Block(stmts, Box::new(final_expr))
}
pub fn mk_expr_break_none() -> Expr {
    Expr::Break(std::option::Option::None)
}
pub fn mk_expr_try_unwrap(inner: Expr) -> Expr {
    Expr::TryUnwrap(Box::new(inner))
}
pub fn mk_expr_early_return(val: Expr) -> Expr {
    Expr::EarlyReturn(Box::new(val))
}

// ─── AST constructors: TypeExpr ─────────────────────────────────────────────

pub fn mk_type_list(inner: TypeExpr) -> TypeExpr {
    TypeExpr::List(Box::new(inner))
}
pub fn mk_type_dict(k: TypeExpr, v: TypeExpr) -> TypeExpr {
    TypeExpr::Dict(Box::new(k), Box::new(v))
}
pub fn mk_type_set(inner: TypeExpr) -> TypeExpr {
    TypeExpr::Set(Box::new(inner))
}

// ─── AST constructors: other ────────────────────────────────────────────────

pub fn mk_variantdef_multi(name: String, fnames: Vec<String>, ftys: Vec<TypeExpr>) -> VariantDef {
    let fields = fnames
        .into_iter()
        .zip(ftys)
        .map(|(n, ty)| {
            let opt_name = if n.is_empty() { None } else { Some(n) };
            (opt_name, ty)
        })
        .collect();
    VariantDef { name, fields }
}
pub fn mk_variantdef_positional(name: String, ftys: Vec<TypeExpr>) -> VariantDef {
    let fields = ftys.into_iter().map(|ty| (None, ty)).collect();
    VariantDef { name, fields }
}

// ─── Option helpers ─────────────────────────────────────────────────────────

pub fn some_expr(e: Expr) -> Option<Expr> {
    Some(e)
}
pub fn none_expr() -> Option<Expr> {
    std::option::Option::None
}
pub fn some_type(t: TypeExpr) -> Option<TypeExpr> {
    Some(t)
}
pub fn none_type() -> Option<TypeExpr> {
    std::option::Option::None
}
pub fn some_str(s: String) -> Option<String> {
    Some(s)
}
pub fn none_str() -> Option<String> {
    std::option::Option::None
}

/// Construct a Some else-clause for Expr::If.
pub fn some_else(stmts: Vec<Stmt>, expr: Expr) -> Option<(Vec<Stmt>, Box<Expr>)> {
    Some((stmts, Box::new(expr)))
}

/// Construct a None else-clause for Expr::If.
pub fn no_else() -> Option<(Vec<Stmt>, Box<Expr>)> {
    std::option::Option::None
}

// ─── Utility ────────────────────────────────────────────────────────────────

/// Split a block's statements into (stmts, final_expr).
/// If the last statement is an Expression, it becomes the final_expr.
/// Otherwise final_expr is the unit tuple.
pub fn split_block(stmts: Vec<Stmt>) -> (Vec<Stmt>, Expr) {
    if stmts.is_empty() {
        return (vec![], Expr::Tuple(vec![]));
    }
    let mut stmts = stmts;
    match stmts.last() {
        Some(Stmt::Expression(_)) => {
            if let Stmt::Expression(e) = stmts.pop().unwrap() {
                (stmts, e)
            } else {
                unreachable!()
            }
        }
        _ => (stmts, Expr::Tuple(vec![])),
    }
}

/// Push a (String, Expr) pair onto a Vec (for struct literal fields / dict building).
pub fn push_name_expr_pair(
    mut pairs: Vec<(String, Expr)>,
    name: String,
    expr: Expr,
) -> Vec<(String, Expr)> {
    pairs.push((name, expr));
    pairs
}

/// Push an (Expr, Expr) pair onto a Vec (for dict building).
pub fn push_expr_pair(
    mut pairs: Vec<(Expr, Expr)>,
    k: Expr,
    v: Expr,
) -> Vec<(Expr, Expr)> {
    pairs.push((k, v));
    pairs
}

/// Create an empty Vec<(String, Expr)>.
pub fn new_name_expr_pairs() -> Vec<(String, Expr)> {
    vec![]
}

/// Create an empty Vec<(Expr, Expr)>.
pub fn new_expr_pairs() -> Vec<(Expr, Expr)> {
    vec![]
}

/// Starting from a position right after `@` or `@!`, capture text until
/// either (a) end-of-line at depth 0, or (b) all opened `()`, `[]`, `{}`
/// have closed. Inside string literals `"..."` brackets are skipped and `\"`
/// is respected. Trailing whitespace is trimmed from the returned string.
///
/// Return: (captured_body, new_byte_pos_in_src)
/// Condition (a): pos points TO the `\n` (not consumed).
/// Condition (b): pos advances past the immediately-following `\n` if present.
pub fn capture_attr_body(src: &str, start: usize) -> (String, usize) {
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut pos = start;
    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut buf: Vec<u8> = Vec::new();

    while pos < len {
        let b = bytes[pos];

        if in_string {
            buf.push(b);
            if b == b'\\' && pos + 1 < len {
                pos += 1;
                buf.push(bytes[pos]);
            } else if b == b'"' {
                in_string = false;
            }
            pos += 1;
            continue;
        }

        match b {
            b'"' => {
                in_string = true;
                buf.push(b);
                pos += 1;
            }
            b'(' | b'[' | b'{' => {
                depth += 1;
                buf.push(b);
                pos += 1;
            }
            b')' | b']' | b'}' => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
                buf.push(b);
                pos += 1;
                if depth == 0 {
                    // condition (b): all brackets closed — consume trailing \n
                    if pos < len && bytes[pos] == b'\n' {
                        pos += 1;
                    }
                    break;
                }
            }
            b'\n' => {
                if depth == 0 {
                    // condition (a): stop, leave \n unconsumed
                    break;
                }
                buf.push(b);
                pos += 1;
            }
            _ => {
                buf.push(b);
                pos += 1;
            }
        }
    }

    let s = String::from_utf8(buf).expect("source is valid UTF-8");
    (s.trim_end().to_string(), pos)
}

// ─── @! inner-attribute helpers ─────────────────────────────────────────────

/// Peek at the kind of the token one position ahead (pos+1).
pub fn ps_peek_next_kind() -> String {
    if has_err_internal() {
        return "Eof".to_string();
    }
    PARSE_TOKENS.with(|t| {
        let tokens = t.borrow();
        let pos = parse_pos_get() as usize;
        let next_idx = (pos + 1).min(tokens.len() - 1);
        token_kind_str(&tokens[next_idx].kind)
    })
}

fn token_to_body_str(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Ident(s) => s.clone(),
        TokenKind::Int(n) => n.to_string(),
        TokenKind::Float(f) => f.to_string(),
        TokenKind::Bool(b) => b.to_string(),
        TokenKind::Str(s) => format!("\"{}\"", s),
        TokenKind::Char(c) => format!("'{}'", c),
        TokenKind::LParen => "(".to_string(),
        TokenKind::RParen => ")".to_string(),
        TokenKind::LBrace => "{".to_string(),
        TokenKind::RBrace => "}".to_string(),
        TokenKind::LBracket => "[".to_string(),
        TokenKind::RBracket => "]".to_string(),
        TokenKind::Comma => ",".to_string(),
        TokenKind::Dot => ".".to_string(),
        TokenKind::Colon => ":".to_string(),
        TokenKind::Semi => ";".to_string(),
        TokenKind::Underscore => "_".to_string(),
        TokenKind::Plus => "+".to_string(),
        TokenKind::Minus => "-".to_string(),
        TokenKind::Star => "*".to_string(),
        TokenKind::Slash => "/".to_string(),
        TokenKind::Percent => "%".to_string(),
        TokenKind::Eq => "==".to_string(),
        TokenKind::Neq => "!=".to_string(),
        TokenKind::Lt => "<".to_string(),
        TokenKind::Gt => ">".to_string(),
        TokenKind::Le => "<=".to_string(),
        TokenKind::Ge => ">=".to_string(),
        TokenKind::Pipe => "|".to_string(),
        TokenKind::At => "@".to_string(),
        TokenKind::Bang => "!".to_string(),
        TokenKind::Question => "?".to_string(),
        _ => String::new(),
    }
}

/// Consume the Bang token and collect all remaining tokens on the same line,
/// reconstructing the attribute body string (e.g. "allow(dead_code)").
/// Called after the caller has already consumed the At token.
pub fn ps_collect_attr_body() -> String {
    if has_err_internal() {
        return String::new();
    }
    let attr_line = peek_token_internal().pos.line;
    advance_internal(); // consume Bang
    let mut parts: Vec<String> = Vec::new();
    loop {
        if has_err_internal() {
            break;
        }
        let t = peek_token_internal();
        if t.pos.line != attr_line {
            break;
        }
        match &t.kind {
            TokenKind::Eof => break,
            kind => {
                parts.push(token_to_body_str(kind));
                advance_internal();
            }
        }
    }
    parts.concat()
}

/// Collect the body of an outer attribute starting at the current token (the
/// ident after `@`). Consumes the ident and an optional balanced bracket group
/// `(...)` / `[...]` on the same line. Stops at `@`, end-of-line, or Eof.
/// Returns the reconstructed body string (e.g. `"derive(Clone)"`, `"inline"`).
/// Called after the caller has already consumed the `At` token.
pub fn ps_collect_outer_attr_body() -> String {
    if has_err_internal() {
        return String::new();
    }
    let mut parts: Vec<String> = Vec::new();

    // Consume the leading ident.
    let t = peek_token_internal();
    let attr_line = t.pos.line;
    match &t.kind {
        TokenKind::Ident(n) => {
            parts.push(n.clone());
            advance_internal();
        }
        _ => return String::new(),
    }

    // Optionally consume a balanced bracket group on the same line.
    let t = peek_token_internal();
    if t.pos.line == attr_line {
        let open = match &t.kind {
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                Some(token_to_body_str(&t.kind))
            }
            _ => None,
        };
        if let Some(open_str) = open {
            parts.push(open_str);
            advance_internal();
            let mut depth: i32 = 1;
            while depth > 0 {
                if has_err_internal() {
                    break;
                }
                let t = peek_token_internal();
                match &t.kind {
                    TokenKind::Eof => break,
                    TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                        depth += 1;
                        parts.push(token_to_body_str(&t.kind));
                        advance_internal();
                    }
                    TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                        depth -= 1;
                        parts.push(token_to_body_str(&t.kind));
                        advance_internal();
                    }
                    kind => {
                        parts.push(token_to_body_str(kind));
                        advance_internal();
                    }
                }
            }
        }
    }

    parts.concat()
}

// ─── Public entry point ─────────────────────────────────────────────────────

/// Parse a token list into a Program (Vec<Stmt>).
/// This is the public API — called from main_imp.rs and resolver_imp.rs.
/// Calls parse_program() which is defined in the .hom-compiled code below.
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, String> {
    PARSE_TOKENS.with(|t| *t.borrow_mut() = tokens);
    parse_pos_set(0);
    parse_err_set(String::new());
    gensym_counter_set(0);
    let program = parse_program();
    let err = parse_err_get();
    if !err.is_empty() {
        Err(err)
    } else {
        Ok(program)
    }
}

#[cfg(test)]
mod tests {
    use super::capture_attr_body;

    #[test]
    fn capture_attr_body_parens() {
        let (s, pos) = capture_attr_body("derive(Clone, Debug)\nFoo := ...", 0);
        assert_eq!(s, "derive(Clone, Debug)");
        assert_eq!(pos, 21);
    }

    #[test]
    fn capture_attr_body_string_escape() {
        let (s, pos) = capture_attr_body("cfg(any(unix, target_os = \"macos\"))\n...", 0);
        assert_eq!(s, "cfg(any(unix, target_os = \"macos\"))");
        let _ = pos;
    }

    #[test]
    fn capture_attr_body_plain() {
        let (s, pos) = capture_attr_body("inline\n", 0);
        assert_eq!(s, "inline");
        assert_eq!(pos, 6);
    }
}
