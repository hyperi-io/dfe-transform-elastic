// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Painless intermediate representation.
//!
//! A thin, Painless-specific IR that captures the semantics of Painless
//! scripts without being tied to either the ANTLR parse tree or the Rust
//! output format. The visitor builds this IR from the parse tree, and the
//! emitter renders it to Rust source strings.

use serde_json::Value;

/// A path segment in a `ctx` field access chain.
///
/// `ctx.event.duration` → `[Static("event"), Static("duration")]`
/// `ctx[variable]` → `[Dynamic(LocalVar { name: "variable" })]`
#[derive(Debug, Clone)]
pub enum PathSegment {
    /// A literal field name: `.event`, `.duration`
    Static(String),
    /// A variable key: `[someVar]`, `[entry.getKey()]`
    Dynamic(Box<Expr>),
}

/// A binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Ushr,

    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Boolean
    And,
    Or,

    // String
    Find,
    Match,
}

/// A unary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Neg,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
}

/// A compound assignment operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundOp {
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    ShlAssign,
    ShrAssign,
    UshrAssign,
}

/// A Painless expression.
#[derive(Debug, Clone)]
pub enum Expr {
    /// Access a field on the event context: `ctx.event.duration`
    CtxAccess { path: Vec<PathSegment> },

    /// Assign a value to a field on the event context: `ctx.event.duration = val`
    CtxAssign {
        path: Vec<PathSegment>,
        value: Box<Expr>,
    },

    /// Remove a field from the event context: `ctx.remove('field')`
    CtxRemove { path: Vec<PathSegment> },

    /// Access a pipeline parameter: `params.key` or `params[key]`
    ParamAccess { path: Vec<PathSegment> },

    /// Reference a local variable: `x`
    LocalVar { name: String },

    /// A literal value: `"str"`, `42`, `true`, `null`
    Literal { value: Value },

    /// Binary operation: `a + b`, `a && b`, `a == b`
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },

    /// Unary operation: `!x`, `-x`, `++i`
    UnaryOp { op: UnaryOp, operand: Box<Expr> },

    /// Method call on a receiver: `s.replace(a, b)`, `list.add(v)`
    MethodCall {
        receiver: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },

    /// Static method call: `Long.parseLong(s)`, `Arrays.asList(x)`
    StaticCall {
        class: String,
        method: String,
        args: Vec<Expr>,
    },

    /// Type check: `x instanceof Map`
    InstanceOf { expr: Box<Expr>, type_name: String },

    /// Type cast: `(int) x`, `(char) x`
    Cast { type_name: String, expr: Box<Expr> },

    /// `new HashMap()`
    NewMap,

    /// `new ArrayList()`
    NewList,

    /// Map literal: `['key': value, ...]` or `[:]`
    MapLiteral { entries: Vec<(Expr, Expr)> },

    /// Array/list literal: `[a, b, c]` or `{a, b, c}`
    ArrayLiteral { elements: Vec<Expr> },

    /// Lambda expression: `v -> expr` or `(k, v) -> expr`
    Lambda {
        params: Vec<String>,
        body: Box<Stmt>,
    },

    /// Call a locally-defined function: `splitUnquoted(input, ",")`
    FunctionCall { name: String, args: Vec<Expr> },

    /// Regex literal: `/pattern/`
    Regex { pattern: String },

    /// Null-safe field access: `x?.field`
    NullSafeAccess { base: Box<Expr>, field: String },

    /// Bracket access: `x[key]`, `x[0]`
    BraceAccess { base: Box<Expr>, index: Box<Expr> },

    /// Ternary conditional: `cond ? a : b`
    Ternary {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },

    /// Compound assignment: `x += 1`, `x &= mask`
    CompoundAssign {
        target: Box<Expr>,
        op: CompoundOp,
        value: Box<Expr>,
    },

    /// Simple assignment to a local variable: `x = expr`
    Assign { target: Box<Expr>, value: Box<Expr> },
}

/// A Painless statement.
#[derive(Debug, Clone)]
pub enum Stmt {
    /// A block of statements: `{ stmt1; stmt2; ... }`
    Block(Vec<Stmt>),

    /// An expression statement: `expr;`
    Expr(Expr),

    /// Variable declaration: `def x = expr` or `int x = expr`
    VarDecl {
        name: String,
        type_name: Option<String>,
        value: Option<Expr>,
    },

    /// If statement: `if (cond) { ... } else { ... }`
    If {
        cond: Expr,
        then_body: Box<Stmt>,
        else_body: Option<Box<Stmt>>,
    },

    /// C-style for loop: `for (init; cond; update) { body }`
    ForC {
        init: Box<Stmt>,
        cond: Expr,
        update: Expr,
        body: Box<Stmt>,
    },

    /// For-each loop: `for (def x : iterable) { body }`
    ForEach {
        var: String,
        iter: Expr,
        body: Box<Stmt>,
    },

    /// Return statement: `return expr;` or `return;`
    Return { value: Option<Expr> },

    /// Local function definition (collected before main body).
    FunctionDef {
        name: String,
        params: Vec<FunctionParam>,
        body: Box<Stmt>,
    },

    /// Empty statement (no-op, e.g. lone semicolon).
    Empty,
}

/// A function parameter with optional type annotation.
#[derive(Debug, Clone)]
pub struct FunctionParam {
    pub name: String,
    pub type_name: Option<String>,
}

/// A complete transpiled Painless script.
///
/// Separates function definitions (emitted as Rust helper functions)
/// from the main body (emitted inline in the transform method).
#[derive(Debug, Clone)]
pub struct PainlessScript {
    /// Local function definitions to emit as standalone Rust functions.
    pub functions: Vec<Stmt>,
    /// The main body statements to emit inline.
    pub body: Vec<Stmt>,
}
