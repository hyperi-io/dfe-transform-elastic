// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Arithmetic a pipeline writes as a one-line Painless assignment.
//!
//! The single largest unmatched script class in the catalogue: 37 distinct
//! expressions across roughly twenty packages, every one of them
//! `ctx.<target> = <expr>`. They were being counted as scripts that ran and
//! were doing nothing.
//!
//! goflow2 is the clearest cost. `ctx.goflow2.time_flow_start_ns =
//! (ctx.goflow2?.time_flow_start_ns / 1000000)` converts nanoseconds to
//! milliseconds, and the `date` processor two steps later reads the result as
//! `UNIX_MS`. With the script unmatched the field stays in nanoseconds, every
//! one of its twelve events dates to the wrong century, and a second expression
//! -- `bytes * sampling_rate` -- takes `network.bytes` and `network.packets`
//! with it.
//!
//! **The numeric type is the whole difficulty, not the parse.** Painless is
//! Java: `/` on two integral operands is INTEGER division and on anything else
//! is floating point. goflow2 depends on the integral case
//! (`1722384059314899647 / 1000000` must be `1722384059314`), and
//! `aws.ec2.metrics.CPUUtilization.avg / 100` and
//! `cyberarkpas.monitor.cpu_usage/100.0` depend on the other. A wrong promotion
//! writes a wrong number into every source at once, which is worse than writing
//! nothing -- so the rule is explicit here and tested on both halves.

use serde_json::{Map, Value, json};

use dfe_core::Event;

/// A floating literal, held by its bits.
///
/// `KnownPattern` and `PainlessPlan` are both `Eq` and an `f64` is not, because
/// NaN breaks reflexivity. A Painless literal is a decimal written in the
/// source and is never a NaN, so the bits are its identity and comparing them
/// is exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatLit(u64);

impl FloatLit {
    /// Hold `value`.
    #[must_use]
    pub fn new(value: f64) -> Self {
        Self(value.to_bits())
    }

    /// The value back, bit for bit.
    #[must_use]
    pub fn get(self) -> f64 {
        f64::from_bits(self.0)
    }
}

/// A scalar written from an expression over other fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalarExpression {
    /// The dotted path the result is written to.
    target: String,
    /// The right-hand side, already parsed.
    expr: Expr,
}

impl ScalarExpression {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(target: impl Into<String>, expr: Expr) -> Self {
        Self {
            target: target.into(),
            expr,
        }
    }

    /// Whether the expression reads `params`, which decides which lane can run
    /// it: the text-only lane is never handed a params block.
    #[must_use]
    pub fn reads_params(&self) -> bool {
        self.expr.reads_params()
    }

    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    #[must_use]
    pub fn direct_call(&self) -> String {
        format!(
            "scalar_expression(event, &ScalarExpression::new({}, {}));",
            rust_str(&self.target),
            self.expr.constructor(),
        )
    }
}

/// A Rust string literal, escaped for the generated source.
#[cfg(feature = "codegen")]
fn rust_str(value: &str) -> String {
    format!("{value:?}")
}

/// One node of a parsed expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// A dotted path read from the document.
    Field(String),
    /// A member of the processor's `params` block.
    Param(String),
    /// An integral literal, `1000000` or `1000000000L`.
    Int(i64),
    /// A floating literal, `100.0`.
    Float(FloatLit),
    /// A quoted literal, which makes any `+` around it a concatenation.
    Str(String),
    /// Two operands and the operator between them.
    Binary(Box<Expr>, Op, Box<Expr>),
    /// A leading `-`.
    Negate(Box<Expr>),
    /// `Math.round(..)`, which lands on a long.
    Round(Box<Expr>),
    /// `Math.abs(..)`.
    Abs(Box<Expr>),
    /// A cast to an integral type, which TRUNCATES rather than rounding.
    ToLong(Box<Expr>),
    /// A cast to a floating type.
    ToDouble(Box<Expr>),
    /// `Double.parseDouble(..)` or `Float.parseFloat(..)`: a string read as a
    /// number.
    ParseNumber(Box<Expr>),
}

/// The four operators these scripts use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    /// `+`, which is CONCATENATION whenever either side is a string.
    Add,
    /// `-`.
    Sub,
    /// `*`.
    Mul,
    /// `/`, integer division on two integral operands.
    Div,
}

impl Expr {
    /// The Rust literal that rebuilds this node, for the generated call site.
    #[cfg(feature = "codegen")]
    fn constructor(&self) -> String {
        match self {
            Self::Field(path) => format!("Expr::Field({}.into())", rust_str(path)),
            Self::Param(name) => format!("Expr::Param({}.into())", rust_str(name)),
            Self::Int(value) => format!("Expr::Int({value})"),
            // `{:?}` on an f64 always writes a decimal point, so the literal
            // parses back as a float rather than an integer.
            Self::Float(value) => format!("Expr::Float(FloatLit::new({:?}))", value.get()),
            Self::Str(text) => format!("Expr::Str({}.into())", rust_str(text)),
            Self::Binary(left, op, right) => format!(
                "Expr::Binary(Box::new({}), Op::{op:?}, Box::new({}))",
                left.constructor(),
                right.constructor(),
            ),
            Self::Negate(inner) => format!("Expr::Negate(Box::new({}))", inner.constructor()),
            Self::Round(inner) => format!("Expr::Round(Box::new({}))", inner.constructor()),
            Self::Abs(inner) => format!("Expr::Abs(Box::new({}))", inner.constructor()),
            Self::ToLong(inner) => format!("Expr::ToLong(Box::new({}))", inner.constructor()),
            Self::ToDouble(inner) => format!("Expr::ToDouble(Box::new({}))", inner.constructor()),
            Self::ParseNumber(inner) => {
                format!("Expr::ParseNumber(Box::new({}))", inner.constructor())
            }
        }
    }

    /// Whether any leaf reads `params`, which decides which lane can run it.
    #[must_use]
    pub fn reads_params(&self) -> bool {
        match self {
            Self::Param(_) => true,
            Self::Field(_) | Self::Int(_) | Self::Float(_) | Self::Str(_) => false,
            Self::Binary(left, _, right) => left.reads_params() || right.reads_params(),
            Self::Negate(inner)
            | Self::Round(inner)
            | Self::Abs(inner)
            | Self::ToLong(inner)
            | Self::ToDouble(inner)
            | Self::ParseNumber(inner) => inner.reads_params(),
        }
    }
}

/// A value mid-evaluation, carrying the distinction Painless carries.
#[derive(Debug, Clone, PartialEq)]
enum Scalar {
    /// An integral value. Two of these divide as integers.
    Int(i64),
    /// A floating value.
    Float(f64),
    /// A string, which turns any `+` it touches into a concatenation.
    Str(String),
}

impl Scalar {
    /// The JSON this lands in the document as.
    fn into_value(self) -> Value {
        match self {
            Self::Int(value) => json!(value),
            Self::Float(value) => json!(value),
            Self::Str(text) => json!(text),
        }
    }

    /// This value as a float, for an operation either side of which is one.
    ///
    /// The widening loses precision above 2^53, and so does Painless: this is
    /// Java's own `long`-to-`double` promotion, and matching it is the point.
    #[allow(clippy::cast_precision_loss)]
    fn as_float(&self) -> Option<f64> {
        match self {
            Self::Int(value) => Some(*value as f64),
            Self::Float(value) => Some(*value),
            Self::Str(_) => None,
        }
    }
}

/// Read a JSON value as a scalar, keeping integral and floating apart.
///
/// `serde_json` answers `as_i64` for a whole float too, so the check is on the
/// number's own form: a value the document holds as `70.0` is a double in
/// Painless and must not turn a division integral.
fn scalar_of(value: &Value) -> Option<Scalar> {
    match value {
        Value::Number(number) => {
            if number.is_f64() {
                number.as_f64().map(Scalar::Float)
            } else {
                number.as_i64().map(Scalar::Int)
            }
        }
        Value::String(text) => Some(Scalar::Str(text.clone())),
        Value::Bool(_) | Value::Null | Value::Array(_) | Value::Object(_) => None,
    }
}

/// Apply one operator, with Painless's promotion.
fn apply(left: &Scalar, op: Op, right: &Scalar) -> Option<Scalar> {
    // Java's `+` is concatenation the moment either side is a String, and that
    // is how four of these scripts build a message or a URL.
    if op == Op::Add && matches!((left, right), (Scalar::Str(_), _) | (_, Scalar::Str(_))) {
        return Some(Scalar::Str(format!("{}{}", render(left), render(right))));
    }

    if let (Scalar::Int(a), Scalar::Int(b)) = (left, right) {
        return Some(Scalar::Int(match op {
            Op::Add => a.checked_add(*b)?,
            Op::Sub => a.checked_sub(*b)?,
            Op::Mul => a.checked_mul(*b)?,
            // Integer division, which is the whole reason this type is tracked.
            Op::Div => a.checked_div(*b)?,
        }));
    }

    let (a, b) = (left.as_float()?, right.as_float()?);
    let value = match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => a * b,
        Op::Div => a / b,
    };
    // An infinity or a NaN has no JSON spelling, and Elasticsearch refuses the
    // document rather than indexing one, so nothing is written.
    value.is_finite().then_some(Scalar::Float(value))
}

/// A scalar as Java would write it into a concatenation.
fn render(value: &Scalar) -> String {
    match value {
        Scalar::Int(number) => number.to_string(),
        Scalar::Float(number) => number.to_string(),
        Scalar::Str(text) => text.clone(),
    }
}

/// Evaluate one node against the document and the params block.
///
/// The two float-to-integer casts below are Java's own: `Math.round` returns a
/// `long` and a `(long)` cast truncates, and both saturate at the type's edge
/// exactly as Rust's `as` does. Clippy's warning is about a conversion that
/// might not be intended; here it IS the semantics being reproduced.
#[allow(clippy::cast_possible_truncation)]
fn eval(expr: &Expr, event: &Event, params: Option<&Map<String, Value>>) -> Option<Scalar> {
    match expr {
        Expr::Field(path) => event.get(path).and_then(scalar_of),
        Expr::Param(name) => params?.get(name).and_then(scalar_of),
        Expr::Int(value) => Some(Scalar::Int(*value)),
        Expr::Float(value) => Some(Scalar::Float(value.get())),
        Expr::Str(text) => Some(Scalar::Str(text.clone())),
        Expr::Binary(left, op, right) => {
            let left = eval(left, event, params)?;
            let right = eval(right, event, params)?;
            apply(&left, *op, &right)
        }
        Expr::Negate(inner) => match eval(inner, event, params)? {
            Scalar::Int(value) => Some(Scalar::Int(value.checked_neg()?)),
            Scalar::Float(value) => Some(Scalar::Float(-value)),
            Scalar::Str(_) => None,
        },
        // Java rounds half toward positive infinity; `f64::round` rounds half
        // away from zero, which differs at -2.5.
        Expr::Round(inner) => {
            let value = eval(inner, event, params)?.as_float()?;
            value
                .is_finite()
                .then(|| Scalar::Int((value + 0.5).floor() as i64))
        }
        Expr::Abs(inner) => match eval(inner, event, params)? {
            Scalar::Int(value) => Some(Scalar::Int(value.checked_abs()?)),
            Scalar::Float(value) => Some(Scalar::Float(value.abs())),
            Scalar::Str(_) => None,
        },
        // A cast truncates toward zero; it does not round.
        Expr::ToLong(inner) => {
            let value = eval(inner, event, params)?.as_float()?;
            value.is_finite().then_some(Scalar::Int(value as i64))
        }
        Expr::ToDouble(inner) => {
            let value = eval(inner, event, params)?.as_float()?;
            value.is_finite().then_some(Scalar::Float(value))
        }
        Expr::ParseNumber(inner) => {
            let value = match eval(inner, event, params)? {
                Scalar::Str(text) => text.trim().parse::<f64>().ok()?,
                number => number.as_float()?,
            };
            value.is_finite().then_some(Scalar::Float(value))
        }
    }
}

/// Write the expression's value to its target.
///
/// Returns true whenever the pattern is one this understands -- a field the
/// document does not carry is a script that would have thrown in Painless and
/// left the document alone, which is not a failure to RECOGNISE it. Returning
/// false there would push the ladder on to a matcher that fits even less well.
pub fn scalar_expression(event: &mut Event, pattern: &ScalarExpression) -> bool {
    write(event, pattern, None)
}

/// As [`scalar_expression`], against a `params` block the script reads.
pub fn scalar_expression_params(
    event: &mut Event,
    pattern: &ScalarExpression,
    params: &Map<String, Value>,
) -> bool {
    write(event, pattern, Some(params))
}

/// The two entry points' one body.
fn write(
    event: &mut Event,
    pattern: &ScalarExpression,
    params: Option<&Map<String, Value>>,
) -> bool {
    if let Some(value) = eval(&pattern.expr, event, params) {
        let _ = event.set(&pattern.target, value.into_value());
    }
    true
}

/// Read `ctx.<target> = <expr>` off a script, or decline.
///
/// Declines on anything it does not fully understand rather than parsing the
/// part it recognises: a half-read expression writes a wrong number, and a
/// wrong number is worse than the absent field the corpus already counts.
#[must_use]
pub fn parse_scalar_expression(script: &str) -> Option<ScalarExpression> {
    let script = script.trim();
    // One statement only; a preamble such as `ctx.network = new HashMap();`
    // makes this a different pattern. A line break is not a statement
    // separator -- azure's ai_foundry wraps a single divide across two lines.
    let body = script.strip_suffix(';').unwrap_or(script).trim();
    if body.contains(';') {
        return None;
    }
    let (target, rhs) = body.split_once('=')?;
    // `==`, `>=` and friends are conditions, not assignments.
    if rhs.starts_with('=') || target.ends_with(['!', '<', '>', '+', '-', '*', '/']) {
        return None;
    }
    let target = path_of(target.trim())?;

    let mut parser = Parser {
        rest: rhs.trim_start(),
    };
    let expr = parser.expression()?;
    if !parser.rest.trim().is_empty() {
        return None;
    }
    // An expression with no operator and no wrapper is a plain copy, which the
    // `set`/`rename` processors express and which has its own patterns.
    if matches!(
        expr,
        Expr::Field(_) | Expr::Param(_) | Expr::Int(_) | Expr::Float(_)
    ) {
        return None;
    }
    Some(ScalarExpression::new(target, expr))
}

/// A `ctx` or `params` path, with the null-safe navigation stripped.
fn path_of(term: &str) -> Option<String> {
    let term = term.trim().replace("?.", ".");
    let path = term.strip_prefix("ctx.")?;
    if path.is_empty()
        || !path
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "._@$".contains(c))
    {
        return None;
    }
    Some(path.to_owned())
}

/// A recursive-descent reader over what is left of the expression.
struct Parser<'a> {
    rest: &'a str,
}

impl Parser<'_> {
    /// `term (('+' | '-') term)*`
    fn expression(&mut self) -> Option<Expr> {
        let mut left = self.term()?;
        loop {
            self.skip_space();
            let op = match self.rest.as_bytes().first() {
                Some(b'+') => Op::Add,
                // Only a BINARY minus: `1000 -5` is a subtraction and `(-5)` is
                // a negation, and the difference is whether an operand precedes
                // it -- which, here, one always has.
                Some(b'-') => Op::Sub,
                _ => return Some(left),
            };
            self.rest = &self.rest[1..];
            let right = self.term()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
    }

    /// `factor (('*' | '/') factor)*`
    fn term(&mut self) -> Option<Expr> {
        let mut left = self.factor()?;
        loop {
            self.skip_space();
            let op = match self.rest.as_bytes().first() {
                Some(b'*') => Op::Mul,
                Some(b'/') => Op::Div,
                _ => return Some(left),
            };
            self.rest = &self.rest[1..];
            let right = self.factor()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
    }

    /// `'-'? primary`
    fn factor(&mut self) -> Option<Expr> {
        self.skip_space();
        if let Some(rest) = self.rest.strip_prefix('-') {
            self.rest = rest;
            return Some(Expr::Negate(Box::new(self.factor()?)));
        }
        self.primary()
    }

    /// A literal, a path, a call, a cast, or a parenthesised expression.
    fn primary(&mut self) -> Option<Expr> {
        self.skip_space();
        for (call, build) in CALLS {
            if let Some(rest) = self.rest.strip_prefix(call) {
                self.rest = rest.trim_start();
                let inner = self.parenthesised()?;
                return Some(build(Box::new(inner)));
            }
        }
        for (cast, build) in CASTS {
            if let Some(rest) = self.rest.strip_prefix(cast) {
                self.rest = rest;
                return Some(build(Box::new(self.factor()?)));
            }
        }
        if self.rest.starts_with('(') {
            return self.parenthesised();
        }
        if self.rest.starts_with('\'') || self.rest.starts_with('"') {
            return self.string();
        }
        if self.rest.starts_with(|c: char| c.is_ascii_digit()) {
            return self.number();
        }
        self.path()
    }

    /// `'(' expression ')'`
    fn parenthesised(&mut self) -> Option<Expr> {
        self.skip_space();
        self.rest = self.rest.strip_prefix('(')?;
        let inner = self.expression()?;
        self.skip_space();
        self.rest = self.rest.strip_prefix(')')?;
        Some(inner)
    }

    /// A single- or double-quoted literal, with no escapes -- none of these
    /// scripts carry one, and guessing at an escape would corrupt the text.
    fn string(&mut self) -> Option<Expr> {
        let quote = self.rest.chars().next()?;
        let body = &self.rest[quote.len_utf8()..];
        let end = body.find(quote)?;
        let text = &body[..end];
        if text.contains('\\') {
            return None;
        }
        self.rest = &body[end + quote.len_utf8()..];
        Some(Expr::Str(text.to_owned()))
    }

    /// An integral or floating literal, with Java's `L`/`D`/`F` suffix.
    fn number(&mut self) -> Option<Expr> {
        let end = self
            .rest
            .find(|c: char| !c.is_ascii_digit() && c != '.' && c != '_')
            .unwrap_or(self.rest.len());
        let (text, rest) = self.rest.split_at(end);
        let text = text.replace('_', "");
        // The suffix names the width, which changes nothing here -- only
        // whether the literal is integral, and the decimal point says that.
        self.rest = rest
            .strip_prefix(['L', 'l', 'D', 'd', 'F', 'f'])
            .unwrap_or(rest);
        if text.contains('.') {
            text.parse()
                .ok()
                .map(|value| Expr::Float(FloatLit::new(value)))
        } else {
            text.parse().ok().map(Expr::Int)
        }
    }

    /// A `ctx` or `params` path.
    fn path(&mut self) -> Option<Expr> {
        let end = self
            .rest
            .find(|c: char| !c.is_ascii_alphanumeric() && !"._?@$".contains(c))
            .unwrap_or(self.rest.len());
        let (text, rest) = self.rest.split_at(end);
        self.rest = rest;
        let text = text.trim_end_matches('.').replace("?.", ".");
        if let Some(name) = text.strip_prefix("params.") {
            return (!name.is_empty() && !name.contains('.')).then(|| Expr::Param(name.to_owned()));
        }
        let path = text.strip_prefix("ctx.")?;
        (!path.is_empty()).then(|| Expr::Field(path.to_owned()))
    }

    /// Advance past whitespace, line breaks included -- a wrapped expression is
    /// still one expression.
    fn skip_space(&mut self) {
        self.rest = self.rest.trim_start();
    }
}

/// The wrapping calls these scripts use.
const CALLS: &[(&str, fn(Box<Expr>) -> Expr)] = &[
    ("Math.round", Expr::Round),
    ("Math.abs", Expr::Abs),
    ("Double.parseDouble", Expr::ParseNumber),
    ("Float.parseFloat", Expr::ParseNumber),
    ("Long.parseLong", Expr::ParseNumber),
    ("Integer.parseInt", Expr::ParseNumber),
];

/// The casts, which bind tighter than any operator and truncate rather than
/// round.
const CASTS: &[(&str, fn(Box<Expr>) -> Expr)] = &[
    ("(Long)", Expr::ToLong),
    ("(long)", Expr::ToLong),
    ("(Integer)", Expr::ToLong),
    ("(int)", Expr::ToLong),
    ("(Double)", Expr::ToDouble),
    ("(double)", Expr::ToDouble),
    ("(Float)", Expr::ToDouble),
    ("(float)", Expr::ToDouble),
];

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    fn run(script: &str, document: Value) -> Event {
        let pattern = parse_scalar_expression(script).expect("the expression is recognised");
        let mut event = Event::new(document);
        assert!(scalar_expression(&mut event, &pattern));
        event
    }

    /// goflow2's timestamp: an integral divide, and the `date` processor after
    /// it reads `UNIX_MS`. A double here dates every event to the wrong
    /// century.
    #[test]
    fn two_integers_divide_as_integers() {
        let event = run(
            "ctx.goflow2.time_flow_start_ns = (ctx.goflow2?.time_flow_start_ns / 1000000);",
            json!({ "goflow2": { "time_flow_start_ns": 1_722_384_059_314_899_647_i64 } }),
        );
        assert_eq!(
            event.get("goflow2.time_flow_start_ns"),
            Some(&json!(1_722_384_059_314_i64))
        );
    }

    /// The other half of the same rule: a fractional operand keeps the result
    /// fractional, so a percentage does not truncate to zero.
    #[test]
    fn a_fractional_operand_keeps_the_result_fractional() {
        let event = run(
            "ctx.host.cpu.usage = ctx.cyberarkpas.monitor.cpu_usage/100.0;",
            json!({ "cyberarkpas": { "monitor": { "cpu_usage": 37 } } }),
        );
        assert_eq!(event.get("host.cpu.usage"), Some(&json!(0.37)));
    }

    /// A value the DOCUMENT holds as a double is a double, whatever its digits
    /// -- `as_i64` answers for a whole float and would turn this integral.
    #[test]
    fn a_whole_double_is_still_a_double() {
        let event = run("ctx.a = ctx.b / 3;", json!({ "b": 10.0 }));
        let got = event.get("a").and_then(Value::as_f64).expect("a float");
        assert!((got - 3.333_333_333_333_333_5).abs() < 1e-12, "{got}");
    }

    /// goflow2's other script, and the one that carries `network.bytes`.
    #[test]
    fn two_fields_multiply() {
        let event = run(
            "ctx.goflow2.flow_size = ctx.goflow2?.bytes * ctx.goflow2?.sampling_rate;",
            json!({ "goflow2": { "bytes": 70, "sampling_rate": 1000 } }),
        );
        assert_eq!(event.get("goflow2.flow_size"), Some(&json!(70_000)));
    }

    /// zeek's certificate window: a subtraction inside a divide inside a round.
    #[test]
    fn a_round_wraps_a_whole_expression() {
        let event = run(
            "ctx.zeek.kerberos.valid.days = Math.round( (ctx.zeek.kerberos.valid.until \
             - ctx.zeek.kerberos.valid.from) / 86400 )",
            json!({ "zeek": { "kerberos": { "valid": { "until": 1_700_086_400_i64, "from": 1_699_000_000_i64 } } } }),
        );
        assert_eq!(event.get("zeek.kerberos.valid.days"), Some(&json!(12)));
    }

    /// A cast truncates where a round would not, and the two appear in the
    /// same catalogue.
    #[test]
    fn a_cast_truncates_and_a_round_does_not() {
        let truncated = run(
            "ctx.a = (Long)(Float.parseFloat(ctx.t) * 1000);",
            json!({ "t": "1.9995" }),
        );
        assert_eq!(truncated.get("a"), Some(&json!(1999)));

        let rounded = run("ctx.a = Math.round(ctx.t * 1000)", json!({ "t": 1.9995 }));
        assert_eq!(rounded.get("a"), Some(&json!(2000)));
    }

    /// Precedence, because these scripts mix the two tiers without brackets.
    #[test]
    fn multiplication_binds_tighter_than_addition() {
        let event = run("ctx.a = ctx.b + ctx.c * 10", json!({ "b": 5, "c": 3 }));
        assert_eq!(event.get("a"), Some(&json!(35)));
    }

    /// Java's `+` is concatenation the moment either side is a String, which is
    /// how four of these build a message or a URL.
    #[test]
    fn a_string_operand_makes_the_plus_a_concatenation() {
        let event = run(
            "ctx.vulnerability.reference = 'https://example.test/?cve=' + ctx.vulnerability.id;",
            json!({ "vulnerability": { "id": "CVE-2026-1" } }),
        );
        assert_eq!(
            event.get_str("vulnerability.reference"),
            Some("https://example.test/?cve=CVE-2026-1")
        );
    }

    /// A `params` member resolves from the block, and the same expression
    /// declines on the text-only lane rather than reading nothing.
    #[test]
    fn a_params_member_resolves_from_its_block() {
        let pattern =
            parse_scalar_expression("ctx.event.duration = ctx.ses.duration * params.S_TO_NS;")
                .expect("the expression is recognised");
        assert!(pattern.reads_params());

        let mut event = Event::new(json!({ "ses": { "duration": 3 } }));
        let params = json!({ "S_TO_NS": 1_000_000_000_i64 });
        let params = params.as_object().expect("a params block");
        assert!(scalar_expression_params(&mut event, &pattern, params));
        assert_eq!(event.get("event.duration"), Some(&json!(3_000_000_000_i64)));
    }

    /// A field the document does not carry leaves the target ALONE. Painless
    /// would have thrown, so writing anything is inventing a value.
    #[test]
    fn an_absent_field_writes_nothing() {
        let pattern = parse_scalar_expression("ctx.a = ctx.b * 2").expect("recognised");
        let mut event = Event::new(json!({ "c": 1 }));
        assert!(scalar_expression(&mut event, &pattern));
        assert_eq!(event.get("a"), None);
    }

    /// A divide by zero throws in Painless rather than yielding infinity, so
    /// the integral path declines instead of panicking.
    #[test]
    fn an_integer_divide_by_zero_writes_nothing() {
        let pattern = parse_scalar_expression("ctx.a = ctx.b / ctx.c").expect("recognised");
        let mut event = Event::new(json!({ "b": 1, "c": 0 }));
        assert!(scalar_expression(&mut event, &pattern));
        assert_eq!(event.get("a"), None);
    }

    /// What it must NOT claim. Each of these is a different pattern, and
    /// reading the part that looks arithmetic would write a wrong value.
    #[test]
    fn anything_it_does_not_fully_understand_is_declined() {
        for script in [
            // A preamble, so the assignment is not the whole script.
            "ctx.network = new HashMap();\nctx.network.bytes = ctx.a + ctx.b;",
            // A method call it has no evaluation for.
            "ctx.message = ctx.message.substring(0, ctx.message.length() - 1);",
            // A comparison, not an assignment.
            "ctx.a == ctx.b + 1",
            // A plain copy, which the `set` processor expresses.
            "ctx.a = ctx.b;",
            // A target that is not a ctx path.
            "def total = ctx.a + ctx.b;",
            // An operand that is neither a path, a literal, nor a call.
            "ctx.a = ctx.b + someLocal;",
        ] {
            assert!(
                parse_scalar_expression(script).is_none(),
                "claimed a script it cannot evaluate: {script}"
            );
        }
    }
}
