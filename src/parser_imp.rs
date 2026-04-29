// parser_imp.rs — Type definitions and helper functions for parser.hom.
//
// Uses thread-local state for tokens, position, and error. This avoids
// expensive state cloning — .hom code just calls ps_peek_kind(), ps_advance(),
// etc. without passing state around.
//
// Once an error is set (ps_set_err), all peek/check/consume/expect become
// no-ops, so parsing "unwinds" naturally without explicit error checks.
//
// Public API groups:
//   State management:  ps_init, ps_save, ps_restore, ps_advance, ps_pos
//   Error handling:    ps_has_err, ps_get_err, ps_set_err, ps_clear_err
//   Token inspection:  ps_peek_kind, ps_peek_ident, ps_peek_int, ...
//   Token matching:    ps_check, ps_consume, ps_expect, ps_same_line
//   AST constructors:  mk_stmt_*, mk_expr_*, mk_pat_*, mk_type_*, mk_param, ...
//   Option helpers:    some_expr, none_expr, some_type, none_type, ...
//   Utility:           split_block, names_to_pats, is_upper_first_str

use std::cell::RefCell;

// ─── Thread-local parser state ──────────────────────────────────────────────

thread_local! {
    static PARSE_TOKENS: RefCell<Vec<Token>> = const { RefCell::new(vec![]) };
    static PARSE_POS: RefCell<usize> = const { RefCell::new(0) };
    static PARSE_ERR: RefCell<String> = const { RefCell::new(String::new()) };
    static GENSYM_COUNTER: RefCell<usize> = const { RefCell::new(0) };
}

fn has_err_internal() -> bool {
    PARSE_ERR.with(|e| !e.borrow().is_empty())
}

fn peek_token_internal() -> Token {
    PARSE_TOKENS.with(|t| {
        let tokens = t.borrow();
        let pos = PARSE_POS.with(|p| *p.borrow());
        let idx = pos.min(tokens.len() - 1);
        tokens[idx].clone()
    })
}

fn advance_internal() {
    PARSE_POS.with(|p| {
        let mut pos = p.borrow_mut();
        let len = PARSE_TOKENS.with(|t| t.borrow().len());
        if *pos < len {
            *pos += 1;
        }
    });
}

fn token_kind_str(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Int(_) => "Int",
        TokenKind::Float(_) => "Float",
        TokenKind::Bool(_) => "Bool",
        TokenKind::Str(_) => "Str",
        TokenKind::Char(_) => "Char",
        TokenKind::None => "None",
        TokenKind::Ident(_) => "Ident",
        TokenKind::Use => "Use",
        TokenKind::Struct => "Struct",
        TokenKind::Enum => "Enum",
        TokenKind::For => "For",
        TokenKind::In => "In",
        TokenKind::While => "While",
        TokenKind::Do => "Do",
        TokenKind::If => "If",
        TokenKind::Else => "Else",
        TokenKind::Match => "Match",
        TokenKind::Break => "Break",
        TokenKind::Continue => "Continue",
        TokenKind::And => "And",
        TokenKind::Or => "Or",
        TokenKind::Not => "Not",
        TokenKind::As => "As",
        TokenKind::Rec => "Rec",
        TokenKind::MutAssign => "MutAssign",
        TokenKind::DoubleColon => "DoubleColon",
        TokenKind::Assign => "Assign",
        TokenKind::Arrow => "Arrow",
        TokenKind::FatArrow => "FatArrow",
        TokenKind::Pipe => "Pipe",
        TokenKind::Dot => "Dot",
        TokenKind::Plus => "Plus",
        TokenKind::Minus => "Minus",
        TokenKind::Star => "Star",
        TokenKind::Slash => "Slash",
        TokenKind::Percent => "Percent",
        TokenKind::Eq => "Eq",
        TokenKind::Neq => "Neq",
        TokenKind::Lt => "Lt",
        TokenKind::Gt => "Gt",
        TokenKind::Le => "Le",
        TokenKind::Ge => "Ge",
        TokenKind::Colon => "Colon",
        TokenKind::Comma => "Comma",
        TokenKind::Semi => "Semi",
        TokenKind::Underscore => "Underscore",
        TokenKind::At => "At",
        TokenKind::Bang => "Bang",
        TokenKind::Question => "Question",
        TokenKind::LParen => "LParen",
        TokenKind::RParen => "RParen",
        TokenKind::LBrace => "LBrace",
        TokenKind::RBrace => "RBrace",
        TokenKind::LBracket => "LBracket",
        TokenKind::RBracket => "RBracket",
        TokenKind::Eof => "Eof",
    }
    .to_string()
}

// ─── Public state management ────────────────────────────────────────────────

pub fn ps_init(tokens: Vec<Token>) {
    PARSE_TOKENS.with(|t| *t.borrow_mut() = tokens);
    PARSE_POS.with(|p| *p.borrow_mut() = 0);
    PARSE_ERR.with(|e| *e.borrow_mut() = String::new());
    GENSYM_COUNTER.with(|c| *c.borrow_mut() = 0);
}

/// Generate a unique temporary name like "_sd0", "_sd1", etc.
pub fn ps_gensym(prefix: String) -> String {
    GENSYM_COUNTER.with(|c| {
        let n = *c.borrow();
        *c.borrow_mut() = n + 1;
        format!("{}{}", prefix, n)
    })
}

pub fn ps_save() -> i32 {
    PARSE_POS.with(|p| *p.borrow() as i32)
}

pub fn ps_restore(pos: i32) {
    PARSE_POS.with(|p| *p.borrow_mut() = pos as usize);
}

pub fn ps_advance() {
    if has_err_internal() {
        return;
    }
    advance_internal();
}

pub fn ps_pos() -> i32 {
    PARSE_POS.with(|p| *p.borrow() as i32)
}

// ─── Error handling ─────────────────────────────────────────────────────────

pub fn ps_has_err() -> bool {
    has_err_internal()
}

pub fn ps_get_err() -> String {
    PARSE_ERR.with(|e| e.borrow().clone())
}

/// Set error — only the first error is kept.
pub fn ps_set_err(msg: String) {
    PARSE_ERR.with(|e| {
        let mut err = e.borrow_mut();
        if err.is_empty() {
            *err = msg;
        }
    });
}

pub fn ps_clear_err() {
    PARSE_ERR.with(|e| *e.borrow_mut() = String::new());
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
        TokenKind::Int(n) => n,
        _ => 0,
    }
}

pub fn ps_peek_float() -> f64 {
    let t = peek_token_internal();
    match t.kind {
        TokenKind::Float(f) => f,
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
        let pos = PARSE_POS.with(|p| *p.borrow());
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

// ─── AST constructors: Stmt ─────────────────────────────────────────────────

pub fn mk_stmt_bind(name: String, expr: Expr) -> Stmt {
    Stmt::Bind(name, expr, Vec::new())
}
pub fn mk_stmt_bind_mut(name: String, expr: Expr) -> Stmt {
    Stmt::BindMut(name, expr)
}
pub fn mk_stmt_bind_pat(pat: Pat, expr: Expr) -> Stmt {
    Stmt::BindPat(pat, expr)
}
pub fn mk_stmt_bind_pat_mut(pat: Pat, expr: Expr) -> Stmt {
    Stmt::BindPatMut(pat, expr)
}
pub fn mk_stmt_assign(lhs: Expr, rhs: Expr) -> Stmt {
    Stmt::Assign(lhs, rhs)
}
pub fn mk_stmt_use(path: Vec<String>) -> Stmt {
    Stmt::Use(path)
}
pub fn mk_stmt_struct_def(name: String, fields: Vec<FieldDef>) -> Stmt {
    Stmt::StructDef(name, fields, Vec::new())
}
pub fn mk_stmt_enum_def(name: String, variants: Vec<VariantDef>) -> Stmt {
    Stmt::EnumDef(name, variants, Vec::new())
}
pub fn mk_stmt_expression(expr: Expr) -> Stmt {
    Stmt::Expression(expr)
}
pub fn mk_stmt_inner_attr(body: String) -> Stmt {
    Stmt::InnerAttr(body)
}
pub fn mk_stmt_thread_local(name: String, expr: Expr) -> Stmt {
    Stmt::ThreadLocal(name, expr)
}

// ─── AST constructors: Expr ─────────────────────────────────────────────────

pub fn mk_expr_int(n: i64) -> Expr {
    Expr::Int(n)
}
pub fn mk_expr_float(f: f64) -> Expr {
    Expr::Float(f)
}
pub fn mk_expr_bool(b: bool) -> Expr {
    Expr::Bool(b)
}
pub fn mk_expr_str(s: String) -> Expr {
    Expr::Str(s)
}
pub fn mk_expr_char_from_str(s: String) -> Expr {
    Expr::Char(s.chars().next().unwrap_or('\0'))
}
pub fn mk_expr_none() -> Expr {
    Expr::None
}
pub fn mk_expr_var(name: String) -> Expr {
    Expr::Var(name)
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
pub fn mk_expr_list(items: Vec<Expr>) -> Expr {
    Expr::List(items)
}
pub fn mk_expr_dict(pairs: Vec<(Expr, Expr)>) -> Expr {
    Expr::Dict(pairs)
}
pub fn mk_expr_set(items: Vec<Expr>) -> Expr {
    Expr::Set(items)
}
pub fn mk_expr_tuple(items: Vec<Expr>) -> Expr {
    Expr::Tuple(items)
}
pub fn mk_expr_struct_lit(name: Option<String>, fields: Vec<(String, Expr)>) -> Expr {
    Expr::Struct(name, fields)
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
pub fn mk_expr_continue() -> Expr {
    Expr::Continue
}
pub fn mk_expr_try_unwrap(inner: Expr) -> Expr {
    Expr::TryUnwrap(Box::new(inner))
}
pub fn mk_expr_early_return(val: Expr) -> Expr {
    Expr::EarlyReturn(Box::new(val))
}

// ─── AST constructors: Pat ──────────────────────────────────────────────────

pub fn mk_pat_wild() -> Pat {
    Pat::Wild
}
pub fn mk_pat_var(name: String) -> Pat {
    Pat::Var(name)
}
pub fn mk_pat_lit(expr: Expr) -> Pat {
    Pat::Lit(expr)
}
pub fn mk_pat_tuple(pats: Vec<Pat>) -> Pat {
    Pat::Tuple(pats)
}
pub fn mk_pat_enum(name: String, payload: Option<Pat>) -> Pat {
    match payload {
        None => Pat::Enum(name, vec![]),
        Some(p) => Pat::Enum(name, vec![p]),
    }
}
pub fn mk_pat_enum_multi(name: String, pats: Vec<Pat>) -> Pat {
    Pat::Enum(name, pats)
}
pub fn mk_pat_none() -> Pat {
    Pat::None
}
pub fn mk_pat_or(pats: Vec<Pat>) -> Pat {
    Pat::Or(pats)
}

// ─── AST constructors: TypeExpr ─────────────────────────────────────────────

pub fn mk_type_name(name: String) -> TypeExpr {
    TypeExpr::Name(name)
}
pub fn mk_type_list(inner: TypeExpr) -> TypeExpr {
    TypeExpr::List(Box::new(inner))
}
pub fn mk_type_dict(k: TypeExpr, v: TypeExpr) -> TypeExpr {
    TypeExpr::Dict(Box::new(k), Box::new(v))
}
pub fn mk_type_set(inner: TypeExpr) -> TypeExpr {
    TypeExpr::Set(Box::new(inner))
}
pub fn mk_type_tuple(items: Vec<TypeExpr>) -> TypeExpr {
    TypeExpr::Tuple(items)
}
pub fn mk_type_generic(name: String, params: Vec<TypeExpr>) -> TypeExpr {
    TypeExpr::Generic(name, params)
}
pub fn mk_type_void() -> TypeExpr {
    TypeExpr::Void
}
pub fn mk_type_infer() -> TypeExpr {
    TypeExpr::Infer
}

// ─── AST constructors: other ────────────────────────────────────────────────

pub fn mk_param(
    name: String,
    ty: Option<TypeExpr>,
    mutable: bool,
    default: Option<Expr>,
) -> Param {
    Param {
        name,
        ty,
        mutable,
        default,
    }
}

pub fn mk_matcharm(pat: Pat, guard: Option<Expr>, body: Expr) -> MatchArm {
    MatchArm { pat, guard, body }
}

pub fn mk_fielddef(name: String, ty: Option<TypeExpr>) -> FieldDef {
    FieldDef { name, ty }
}

pub fn mk_variantdef(name: String, payload: Option<TypeExpr>) -> VariantDef {
    VariantDef {
        name,
        fields: match payload {
            None => vec![],
            Some(ty) => vec![(None, ty)],
        },
    }
}
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
pub fn some_pat(p: Pat) -> Option<Pat> {
    Some(p)
}
pub fn none_pat() -> Option<Pat> {
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

/// Convert a list of name strings to Pat values ("_" → Pat::Wild, else Pat::Var).
pub fn names_to_pats(names: Vec<String>) -> Vec<Pat> {
    names
        .into_iter()
        .map(|n| {
            if n == "_" {
                Pat::Wild
            } else {
                Pat::Var(n)
            }
        })
        .collect()
}

/// Negate an i64 value (avoids .hom int literal type mismatch).
pub fn neg_i64(n: i64) -> i64 {
    -n
}

/// Negate an f64 value (avoids .hom float literal type mismatch).
pub fn neg_f64(f: f64) -> f64 {
    -f
}

/// Returns true if the first character is uppercase.
pub fn is_upper_first_str(s: String) -> bool {
    s.chars().next().is_some_and(|c| c.is_uppercase())
}

/// Concatenate two Vec<Pat>.
pub fn vec_concat_pats(mut a: Vec<Pat>, b: Vec<Pat>) -> Vec<Pat> {
    a.extend(b);
    a
}

/// Concatenate two Vec<Expr>.
pub fn vec_concat_exprs(mut a: Vec<Expr>, b: Vec<Expr>) -> Vec<Expr> {
    a.extend(b);
    a
}

/// Concatenate two Vec<String>.
pub fn vec_concat_strs(mut a: Vec<String>, b: Vec<String>) -> Vec<String> {
    a.extend(b);
    a
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

// ─── Outer-attribute constructors ───────────────────────────────────────────

pub fn mk_stmt_bind_attrs(name: String, expr: Expr, attrs: Vec<String>) -> Stmt {
    Stmt::Bind(name, expr, attrs)
}
pub fn mk_stmt_struct_def_attrs(name: String, fields: Vec<FieldDef>, attrs: Vec<String>) -> Stmt {
    Stmt::StructDef(name, fields, attrs)
}
pub fn mk_stmt_enum_def_attrs(name: String, variants: Vec<VariantDef>, attrs: Vec<String>) -> Stmt {
    Stmt::EnumDef(name, variants, attrs)
}

// ─── @! inner-attribute helpers ─────────────────────────────────────────────

/// Peek at the kind of the token one position ahead (pos+1).
pub fn ps_peek_next_kind() -> String {
    if has_err_internal() {
        return "Eof".to_string();
    }
    PARSE_TOKENS.with(|t| {
        let tokens = t.borrow();
        let pos = PARSE_POS.with(|p| *p.borrow());
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
    ps_init(tokens);
    let program = parse_program();
    if ps_has_err() {
        Err(ps_get_err())
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
