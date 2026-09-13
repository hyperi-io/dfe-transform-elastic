// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! ECS fields written from the members of a list's items, under nested guards.
//!
//! All three Atlassian audit streams ship one script that turns
//! `<package>.audit.affected_objects` and `<package>.audit.changed_values` into
//! the ECS user block: `group.name`, `group.id`, `user.target.name`,
//! `user.target.id`, `user.target.group.*`, `user.changes.*`, and through the
//! `append` processors after it, `related.user`. Unmatched it is the largest
//! single item left in the corpus -- 336 fields and 91 events across
//! `atlassian_confluence`, `atlassian_jira` and `atlassian_bitbucket`.
//!
//! **The three scripts DIFFER, and the differences are the job.** They share a
//! skeleton -- a guard on `event.action`, five map allocations, then a walk of
//! each list under an outer test on the item's `type` and an inner test on a
//! list of action literals -- but confluence binds the group name and id to
//! locals and re-splits them with a regex when the stream is cloud-hosted, and
//! reads the target user's name out of a `uri` member with a second regex. A
//! matcher keyed to jira's spelling silently declines the other two, so this
//! one reads the tables OFF the script.
//!
//! It is therefore a small INTERPRETER over a closed grammar rather than a
//! reader for one script: a guard, an allocation, a walk, a put, a local, a
//! regex match. Anything outside that grammar declines the WHOLE script --
//! writing some of the user block and not the rest makes a source read as
//! needing polish rather than a different matcher, which is the trap this
//! project has fallen into before.

use regex::Regex;
use serde_json::{Map, Value};

use dfe_core::Event;

// -- The grammar ---------------------------------------------------------

/// A value one statement reads.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    /// `ctx.<path>`, with the null-safe navigation flattened.
    Field(String),
    /// `ctx.<list>[<index>].<member>` -- a member of the item the enclosing
    /// walk is on. `member` is empty for the item itself.
    Member {
        list: String,
        index: String,
        member: String,
    },
    /// A `String`, `Map` or `def` local.
    Local(String),
    /// A quoted literal.
    Literal(String),
    /// `new HashMap()`.
    EmptyMap,
    /// `/<regex>/.matcher(<subject>)`, the regex held by index into
    /// [`ItemWrites::regexes`].
    Matcher { regex: usize, subject: Box<Expr> },
    /// `<matcher>.group(<n>)`.
    Group { matcher: String, group: usize },
}

/// The condition of an `if`.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Test {
    /// `<expr> == null`.
    IsNull(Expr),
    /// `<expr> != null`.
    NotNull(Expr),
    /// `<expr> == '<literal>'`.
    Equals(Expr, String),
    /// `[<literals>].contains(<expr>)` -- the action table.
    OneOf(Vec<String>, Expr),
    /// `<matcher>.find()`.
    Finds(String),
}

/// One statement of the restricted Painless this module reads.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Stmt {
    /// `return;` -- leaves the whole script, not just the block.
    Return,
    /// `Map`/`String`/`def <name> = <expr>;`.
    Declare { name: String, value: Expr },
    /// `<name> = <expr>;` over a local already declared.
    Assign { name: String, value: Expr },
    /// `<receiver>.put('<key>', <value>);`, resolved to the dotted path it
    /// writes. Both halves are literals, so the join is done once at parse
    /// time rather than per event.
    Put { path: String, value: Expr },
    /// `if (<test>) { <body> }`. No `else` arm: see [`statements`].
    If { test: Test, body: Vec<Stmt> },
    /// `for (def <index> = 0; <index> < ctx.<list>.length; <index>++) { .. }`.
    Walk {
        index: String,
        list: String,
        body: Vec<Stmt>,
    },
}

/// A Painless regex literal, compiled when the script was read.
///
/// Compiling at PARSE time is what makes a Java pattern Rust's `regex` cannot
/// build DECLINE the script rather than panic on the first event to reach it.
/// Java has lookaround and backreferences and Rust does not, and a
/// `Regex::new(..).unwrap()` at the call site would only say so in production.
#[derive(Debug, Clone)]
struct ScriptRegex {
    source: String,
    compiled: Regex,
}

/// Two regexes are the same regex when they are the same pattern; the compiled
/// program carries no equality of its own.
impl PartialEq for ScriptRegex {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Eq for ScriptRegex {}

/// The whole script: its statements, and every regex literal it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemWrites {
    body: Vec<Stmt>,
    regexes: Vec<ScriptRegex>,
}

// -- Reading the script --------------------------------------------------

/// What the scanner is inside of, so punctuation in a literal is not read as
/// code.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Span {
    Code,
    Quoted(char),
    Regex,
}

/// Walk `text`, offering every position that sits in CODE to `at_code` with
/// the nesting depth before that character. The first position it accepts is
/// the answer.
///
/// Not a `find`, because both literal kinds hold code punctuation: a Painless
/// regex spells a repetition `{8}` and a group `(.+)`, so counting either as
/// nesting cuts a block in the wrong place, and an action literal may hold any
/// punctuation at all.
fn scan(text: &str, mut at_code: impl FnMut(usize, char, usize) -> bool) -> Option<usize> {
    let mut span = Span::Code;
    let mut escaped = false;
    let mut depth = 0usize;
    let mut previous = None;
    for (offset, c) in text.char_indices() {
        match span {
            Span::Quoted(quote) => {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == quote {
                    span = Span::Code;
                }
            }
            Span::Regex => {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '/' {
                    span = Span::Code;
                }
            }
            Span::Code => {
                if at_code(offset, c, depth) {
                    return Some(offset);
                }
                match c {
                    '\'' | '"' => {
                        span = Span::Quoted(c);
                        escaped = false;
                    }
                    // A slash opens a regex literal only where a VALUE may
                    // start. Anywhere else it is a division.
                    '/' if matches!(previous, Some('=' | '(' | ',')) => {
                        span = Span::Regex;
                        escaped = false;
                    }
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => depth = depth.checked_sub(1)?,
                    _ => {}
                }
            }
        }
        if !c.is_whitespace() {
            previous = Some(c);
        }
    }
    None
}

/// The offset of the delimiter matching the one that opens at `at`.
fn matching(text: &str, at: usize) -> Option<usize> {
    let close = match text[at..].chars().next()? {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        _ => return None,
    };
    scan(&text[at..], |_, c, depth| c == close && depth == 1).map(|offset| at + offset)
}

/// The offset of `needle` outside every nested group, quote and regex literal.
fn top_level_find(text: &str, needle: &str) -> Option<usize> {
    scan(text, |offset, _, depth| {
        depth == 0 && text[offset..].starts_with(needle)
    })
}

/// The text after `word` when `text` starts with it as a whole word.
fn keyword<'a>(text: &'a str, word: &str) -> Option<&'a str> {
    let rest = text.strip_prefix(word)?;
    (!rest.starts_with(|c: char| c.is_alphanumeric() || c == '_')).then_some(rest)
}

/// Whether a path segment is a plain Painless name.
fn is_name(text: &str) -> bool {
    !text.is_empty()
        && !text.starts_with(|c: char| c.is_ascii_digit())
        && text.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// One plain name, trimmed.
fn name_of(text: &str) -> Option<String> {
    let text = text.trim();
    is_name(text).then(|| text.to_owned())
}

/// The contents of a whole quoted literal, or nothing when `text` is not one.
fn quoted(text: &str) -> Option<String> {
    let quote = text.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    let body = &text[quote.len_utf8()..];
    let mut escaped = false;
    for (at, c) in body.char_indices() {
        if escaped {
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == quote {
            // The literal has to be the WHOLE expression, or something is
            // being done to it that this runner does not model.
            if !body[at + c.len_utf8()..].trim().is_empty() {
                return None;
            }
            return Some(unescape(&body[..at]));
        }
    }
    None
}

/// A quoted literal's body with its escapes resolved.
fn unescape(text: &str) -> String {
    if !text.contains('\\') {
        return text.to_owned();
    }
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

/// The offset of the first unescaped `/`.
fn unescaped_slash(text: &str) -> Option<usize> {
    let mut escaped = false;
    for (at, c) in text.char_indices() {
        if escaped {
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '/' {
            return Some(at);
        }
    }
    None
}

/// `<left> = <right>` split at the ASSIGNMENT, never at a comparison.
fn split_assignment(text: &str) -> Option<(&str, &str)> {
    let at = top_level_find(text, "=")?;
    let previous = text[..at].chars().next_back();
    if text[at + 1..].starts_with('=')
        || matches!(
            previous,
            Some('=' | '!' | '<' | '>' | '+' | '-' | '*' | '/' | '&' | '|')
        )
    {
        return None;
    }
    Some((&text[..at], &text[at + 1..]))
}

/// `<left> <op> <right>` split at a top-level comparison.
fn split_comparison<'a>(text: &'a str, op: &str) -> Option<(&'a str, &'a str)> {
    let at = top_level_find(text, op)?;
    Some((&text[..at], &text[at + op.len()..]))
}

/// The members of `[<literal>, <literal>, ..]`, or nothing when one of them is
/// not a literal.
fn quoted_list(text: &str) -> Option<Vec<String>> {
    let mut members = Vec::new();
    let mut rest = text;
    loop {
        let Some(at) = top_level_find(rest, ",") else {
            members.push(quoted(rest.trim())?);
            return Some(members);
        };
        members.push(quoted(rest[..at].trim())?);
        rest = &rest[at + 1..];
    }
}

/// The two arguments of a call, split at the top-level comma.
fn split_argument(text: &str) -> Option<(&str, &str)> {
    let at = top_level_find(text, ",")?;
    Some((&text[..at], &text[at + 1..]))
}

/// The dotted path a `put` receiver names, empty for `ctx` itself.
fn receiver_path(text: &str) -> Option<String> {
    let flat = text.trim().replace("?.", ".");
    if flat == "ctx" {
        return Some(String::new());
    }
    let body = flat.strip_prefix("ctx.")?;
    body.split('.').all(is_name).then(|| body.to_owned())
}

/// `ctx.<path>` as the value it reads, with the null-safe navigation flattened
/// and at most one `[<index>]` subscript.
fn path_expr(text: &str) -> Option<Expr> {
    let flat = text.trim().replace("?.", ".");
    let body = flat.strip_prefix("ctx.")?;

    let mut before: Vec<&str> = Vec::new();
    let mut after: Vec<&str> = Vec::new();
    let mut index: Option<String> = None;
    for segment in body.split('.') {
        let (key, subscript) = match segment.split_once('[') {
            Some((key, rest)) => (key, Some(rest.strip_suffix(']')?)),
            None => (segment, None),
        };
        if !is_name(key) {
            return None;
        }
        match (index.is_some(), subscript) {
            (false, None) => before.push(key),
            (false, Some(subscript)) => {
                before.push(key);
                index = Some(name_of(subscript)?);
            }
            (true, None) => after.push(key),
            // A second subscript is a nesting the runner does not model.
            (true, Some(_)) => return None,
        }
    }

    let list = before.join(".");
    match index {
        None => Some(Expr::Field(list)),
        Some(index) => Some(Expr::Member {
            list,
            index,
            member: after.join("."),
        }),
    }
}

/// One expression, or nothing when it is a form this runner cannot evaluate.
fn parse_expr(text: &str, regexes: &mut Vec<ScriptRegex>) -> Option<Expr> {
    let text = text.trim();
    if text == "new HashMap()" {
        return Some(Expr::EmptyMap);
    }
    if let Some(body) = text.strip_prefix('/') {
        // Offsets are into `text`, so the closing slash sits one past the
        // offset `body` reports.
        let close = unescaped_slash(body)? + 1;
        let source = &text[1..close];
        // `.matcher(` follows the closing slash, and its `(` is the last of it.
        if !text[close + 1..].starts_with(".matcher(") {
            return None;
        }
        let open = close + ".matcher(".len();
        let end = matching(text, open)?;
        if !text[end + 1..].trim().is_empty() {
            return None;
        }
        let subject = parse_expr(&text[open + 1..end], regexes)?;
        let compiled = Regex::new(source).ok()?;
        regexes.push(ScriptRegex {
            source: source.to_owned(),
            compiled,
        });
        return Some(Expr::Matcher {
            regex: regexes.len() - 1,
            subject: Box::new(subject),
        });
    }
    if let Some(literal) = quoted(text) {
        return Some(Expr::Literal(literal));
    }
    if let Some((matcher, tail)) = text.split_once(".group(") {
        return Some(Expr::Group {
            matcher: name_of(matcher)?,
            group: tail.strip_suffix(')')?.trim().parse().ok()?,
        });
    }
    if text == "ctx" || text.starts_with("ctx.") || text.starts_with("ctx?.") {
        return path_expr(text);
    }
    name_of(text).map(Expr::Local)
}

/// The condition of an `if`, or nothing when it is one this runner cannot
/// decide.
fn parse_test(text: &str, regexes: &mut Vec<ScriptRegex>) -> Option<Test> {
    let text = text.trim();
    if text.starts_with('[') {
        let close = matching(text, 0)?;
        let members = quoted_list(&text[1..close])?;
        if !text[close + 1..].starts_with(".contains(") {
            return None;
        }
        let open = close + ".contains(".len();
        let end = matching(text, open)?;
        if !text[end + 1..].trim().is_empty() {
            return None;
        }
        return Some(Test::OneOf(
            members,
            parse_expr(&text[open + 1..end], regexes)?,
        ));
    }
    if let Some(matcher) = text.strip_suffix(".find()") {
        return Some(Test::Finds(name_of(matcher)?));
    }
    if let Some((left, right)) = split_comparison(text, "!=") {
        if right.trim() != "null" {
            return None;
        }
        return Some(Test::NotNull(parse_expr(left, regexes)?));
    }
    if let Some((left, right)) = split_comparison(text, "==") {
        let right = right.trim();
        if right == "null" {
            return Some(Test::IsNull(parse_expr(left, regexes)?));
        }
        return Some(Test::Equals(parse_expr(left, regexes)?, quoted(right)?));
    }
    None
}

/// `for (def <i> = 0; <i> < ctx.<list>.length; <i>++)` -- the only loop this
/// runner walks: one ascending index over one list, start to end.
fn parse_walk_header(text: &str) -> Option<(String, String)> {
    let mut parts = text.split(';');

    let (name, start) = split_assignment(keyword(parts.next()?.trim(), "def")?)?;
    let name = name_of(name)?;
    if start.trim() != "0" {
        return None;
    }

    let (left, right) = parts.next()?.trim().split_once('<')?;
    if name_of(left)? != name {
        return None;
    }
    let Expr::Field(list) = path_expr(right.trim().strip_suffix(".length")?)? else {
        return None;
    };

    if parts.next()?.trim() != format!("{name}++") || parts.next().is_some() {
        return None;
    }
    Some((name, list))
}

/// One statement that holds no block, or nothing when it is a form this runner
/// cannot carry out.
fn parse_simple(text: &str, regexes: &mut Vec<ScriptRegex>) -> Option<Stmt> {
    if text == "return" {
        return Some(Stmt::Return);
    }
    for word in ["Map", "String", "def"] {
        if let Some(rest) = keyword(text, word) {
            let (name, value) = split_assignment(rest)?;
            return Some(Stmt::Declare {
                name: name_of(name)?,
                value: parse_expr(value, regexes)?,
            });
        }
    }
    if let Some(at) = text.find(".put(") {
        let open = at + ".put(".len() - 1;
        let end = matching(text, open)?;
        if !text[end + 1..].trim().is_empty() {
            return None;
        }
        let (key, value) = split_argument(&text[open + 1..end])?;
        let key = quoted(key.trim())?;
        let target = receiver_path(&text[..at])?;
        let path = if target.is_empty() {
            key
        } else {
            format!("{target}.{key}")
        };
        return Some(Stmt::Put {
            path,
            value: parse_expr(value, regexes)?,
        });
    }
    let (name, value) = split_assignment(text)?;
    Some(Stmt::Assign {
        name: name_of(name)?,
        value: parse_expr(value, regexes)?,
    })
}

/// Every statement of one block, or nothing when the block holds a form the
/// grammar does not cover.
///
/// Declining the WHOLE script on ONE unreadable statement is the point. A
/// matcher that runs the statements it understands and skips the rest writes
/// some of the ECS user block and not all of it, which reads as a source
/// needing polish rather than one needing a different matcher.
fn statements(text: &str, regexes: &mut Vec<ScriptRegex>) -> Option<Vec<Stmt>> {
    let mut out = Vec::new();
    let mut rest = text;
    loop {
        let block = rest.trim_start();
        if block.is_empty() {
            return Some(out);
        }
        // An `else` arm is not in the grammar, and reading only the `if` half
        // of one would write the branch the vendor did not take. mattermost
        // ships this same processor as an if/else-if chain and declines here.
        if keyword(block, "else").is_some() {
            return None;
        }

        let after_if = keyword(block, "if");
        let branch = after_if.is_some();
        let Some(after) = after_if.or_else(|| keyword(block, "for")) else {
            let end = top_level_find(block, ";")?;
            out.push(parse_simple(block[..end].trim(), regexes)?);
            rest = &block[end + 1..];
            continue;
        };

        let head = after.trim_start();
        if !head.starts_with('(') {
            return None;
        }
        let open = block.len() - head.len();
        let close = matching(block, open)?;

        let tail = &block[close + 1..];
        let brace = close + 1 + (tail.len() - tail.trim_start().len());
        if !block[brace..].starts_with('{') {
            return None;
        }
        let end = matching(block, brace)?;
        let body = statements(&block[brace + 1..end], regexes)?;

        out.push(if branch {
            Stmt::If {
                test: parse_test(&block[open + 1..close], regexes)?,
                body,
            }
        } else {
            let (index, list) = parse_walk_header(&block[open + 1..close])?;
            Stmt::Walk { index, list, body }
        });
        rest = &block[end + 1..];
    }
}

// -- Proving the whole of it is readable ---------------------------------

/// What the walks and declarations in scope have bound, for the check that
/// every name a statement reads is one the script itself binds.
#[derive(Default)]
struct Bound {
    /// `(<index variable>, <the list its walk is over>)`.
    indices: Vec<(String, String)>,
    /// `(<name>, <whether it is a matcher>)`.
    names: Vec<(String, bool)>,
}

impl Bound {
    fn holds(&self, name: &str, matcher: bool) -> bool {
        self.names
            .iter()
            .rev()
            .find(|(held, _)| held == name)
            .is_some_and(|(_, is_matcher)| *is_matcher == matcher)
    }

    fn expr(&self, expr: &Expr) -> bool {
        match expr {
            // The item has to come from the walk that is over ITS list, or the
            // index means something else entirely.
            Expr::Member { list, index, .. } => self
                .indices
                .iter()
                .any(|(held, over)| held == index && over == list),
            Expr::Local(name) => self.holds(name, false),
            Expr::Group { matcher, .. } => self.holds(matcher, true),
            Expr::Matcher { subject, .. } => self.expr(subject),
            Expr::Field(_) | Expr::Literal(_) | Expr::EmptyMap => true,
        }
    }

    fn test(&self, test: &Test) -> bool {
        match test {
            Test::IsNull(expr)
            | Test::NotNull(expr)
            | Test::Equals(expr, _)
            | Test::OneOf(_, expr) => self.expr(expr),
            Test::Finds(matcher) => self.holds(matcher, true),
        }
    }

    fn block(&mut self, body: &[Stmt]) -> bool {
        let (indices, names) = (self.indices.len(), self.names.len());
        let readable = body.iter().all(|stmt| self.stmt(stmt));
        self.indices.truncate(indices);
        self.names.truncate(names);
        readable
    }

    fn stmt(&mut self, stmt: &Stmt) -> bool {
        match stmt {
            Stmt::Return => true,
            Stmt::Declare { name, value } => {
                if !self.expr(value) {
                    return false;
                }
                self.names
                    .push((name.clone(), matches!(value, Expr::Matcher { .. })));
                true
            }
            Stmt::Assign { name, value } => self.holds(name, false) && self.expr(value),
            Stmt::Put { value, .. } => self.expr(value),
            Stmt::If { test, body } => self.test(test) && self.block(body),
            Stmt::Walk { index, list, body } => {
                self.indices.push((index.clone(), list.clone()));
                let readable = self.block(body);
                self.indices.pop();
                readable
            }
        }
    }
}

/// Whether any statement in the tree satisfies `wanted`.
fn any(body: &[Stmt], wanted: &impl Fn(&Stmt) -> bool) -> bool {
    body.iter().any(|stmt| {
        wanted(stmt)
            || match stmt {
                Stmt::If { body, .. } | Stmt::Walk { body, .. } => any(body, wanted),
                _ => false,
            }
    })
}

impl ItemWrites {
    /// Whether the script is one this runner can carry out in full.
    fn is_readable(&self) -> bool {
        // A walk and a write are the pattern's signature. Without both this is
        // some other script that happens to parse, and every arm above owns
        // its own kind better than a general reader placed last.
        any(&self.body, &|stmt| matches!(stmt, Stmt::Walk { .. }))
            && any(&self.body, &|stmt| matches!(stmt, Stmt::Put { .. }))
            && Bound::default().block(&self.body)
    }
}

/// Read the whole script, or decline.
#[must_use]
pub fn parse_item_writes(script: &str) -> Option<ItemWrites> {
    let mut regexes = Vec::new();
    let body = statements(script, &mut regexes)?;
    let pattern = ItemWrites { body, regexes };
    pattern.is_readable().then_some(pattern)
}

// -- Running it ----------------------------------------------------------

/// A Painless `Matcher` local: the regex it was built from, the string it was
/// given, and what its last `find()` captured.
struct Matcher<'p> {
    name: &'p str,
    regex: usize,
    subject: Option<String>,
    groups: Option<Vec<Option<String>>>,
}

/// What the enclosing walks and blocks have bound, innermost last.
///
/// Vectors rather than maps: a block binds one or two names, and a linear
/// scan over two entries beats hashing a string on a per-event path.
#[derive(Default)]
struct Scope<'p> {
    items: Vec<(&'p str, Value)>,
    locals: Vec<(&'p str, Value)>,
    matchers: Vec<Matcher<'p>>,
}

impl Scope<'_> {
    fn mark(&self) -> (usize, usize, usize) {
        (self.items.len(), self.locals.len(), self.matchers.len())
    }

    fn rewind(&mut self, (items, locals, matchers): (usize, usize, usize)) {
        self.items.truncate(items);
        self.locals.truncate(locals);
        self.matchers.truncate(matchers);
    }

    fn item(&self, index: &str) -> Option<&Value> {
        self.items
            .iter()
            .rev()
            .find(|(name, _)| *name == index)
            .map(|(_, item)| item)
    }

    fn local(&self, name: &str) -> Option<&Value> {
        self.locals
            .iter()
            .rev()
            .find(|(held, _)| *held == name)
            .map(|(_, value)| value)
    }

    fn assign(&mut self, name: &str, value: Value) {
        if let Some((_, slot)) = self.locals.iter_mut().rev().find(|(held, _)| *held == name) {
            *slot = value;
        }
    }

    fn groups(&self, name: &str) -> Option<&Vec<Option<String>>> {
        self.matchers
            .iter()
            .rev()
            .find(|held| held.name == name)?
            .groups
            .as_ref()
    }

    /// Run the matcher's regex over the string it was given, the way Java's
    /// `Matcher.find` does, and keep what it captured for `group`.
    fn find(&mut self, name: &str, pattern: &ItemWrites) -> bool {
        let Some(held) = self
            .matchers
            .iter_mut()
            .rev()
            .find(|held| held.name == name)
        else {
            return false;
        };
        let Some(regex) = pattern.regexes.get(held.regex) else {
            return false;
        };
        // Bound to a local first, so the borrow of the subject ends before the
        // captures are stored beside it.
        let groups: Option<Vec<Option<String>>> = held
            .subject
            .as_deref()
            .and_then(|subject| regex.compiled.captures(subject))
            .map(|caught| {
                caught
                    .iter()
                    .map(|group| group.map(|found| found.as_str().to_owned()))
                    .collect()
            });
        held.groups = groups;
        held.groups.is_some()
    }
}

/// Whether the statements after this one still run.
enum Flow {
    Next,
    Stop,
}

/// The member at a dotted path inside one list item.
fn member_of(item: &Value, member: &str) -> Value {
    if member.is_empty() {
        return item.clone();
    }
    let mut current = item;
    for segment in member.split('.') {
        match current.get(segment) {
            Some(next) => current = next,
            None => return Value::Null,
        }
    }
    current.clone()
}

/// One expression's value. An absent field is null, exactly as Painless reads
/// it, so a `put` of one stores an explicit null and the drop-empty script
/// every one of these pipelines ends with takes it away again.
fn eval(event: &Event, expr: &Expr, scope: &Scope<'_>) -> Value {
    match expr {
        Expr::Field(path) => event.get(path).cloned().unwrap_or(Value::Null),
        Expr::Member { index, member, .. } => scope
            .item(index)
            .map_or(Value::Null, |item| member_of(item, member)),
        Expr::Local(name) => scope.local(name).cloned().unwrap_or(Value::Null),
        Expr::Literal(text) => Value::String(text.clone()),
        Expr::EmptyMap => Value::Object(Map::new()),
        Expr::Group { matcher, group } => scope
            .groups(matcher)
            .and_then(|groups| groups.get(*group).cloned().flatten())
            .map_or(Value::Null, Value::String),
        // Only ever bound by a `def`, which reads it before it gets here.
        Expr::Matcher { .. } => Value::Null,
    }
}

/// Whether one condition holds.
fn decide(event: &Event, test: &Test, pattern: &ItemWrites, scope: &mut Scope<'_>) -> bool {
    match test {
        Test::IsNull(expr) => eval(event, expr, scope).is_null(),
        Test::NotNull(expr) => !eval(event, expr, scope).is_null(),
        // Painless compares a `def` string by value, and anything that is not
        // a string is not equal to a string literal.
        Test::Equals(expr, literal) => {
            matches!(eval(event, expr, scope), Value::String(held) if &held == literal)
        }
        Test::OneOf(literals, expr) => {
            matches!(eval(event, expr, scope), Value::String(held) if literals.contains(&held))
        }
        Test::Finds(matcher) => scope.find(matcher, pattern),
    }
}

/// Every item of the list, in order.
fn walk<'p>(
    event: &mut Event,
    index: &'p str,
    list: &str,
    body: &'p [Stmt],
    pattern: &'p ItemWrites,
    scope: &mut Scope<'p>,
) -> Flow {
    // The list is cloned ONCE and its items moved out one at a time: the body
    // writes to the event, so a borrow of the list cannot be held across it.
    let Some(items) = event.get(list).and_then(Value::as_array).cloned() else {
        return Flow::Next;
    };
    for item in items {
        scope.items.push((index, item));
        let flow = run(event, body, pattern, scope);
        scope.items.pop();
        if matches!(flow, Flow::Stop) {
            return Flow::Stop;
        }
    }
    Flow::Next
}

/// One block of statements, in order.
fn run<'p>(
    event: &mut Event,
    body: &'p [Stmt],
    pattern: &'p ItemWrites,
    scope: &mut Scope<'p>,
) -> Flow {
    let mark = scope.mark();
    for stmt in body {
        let flow = match stmt {
            // Painless `return` leaves the SCRIPT, not the block.
            Stmt::Return => Flow::Stop,
            Stmt::Declare { name, value } => {
                match value {
                    Expr::Matcher { regex, subject } => {
                        let subject = eval(event, subject, scope);
                        scope.matchers.push(Matcher {
                            name: name.as_str(),
                            regex: *regex,
                            subject: subject.as_str().map(str::to_owned),
                            groups: None,
                        });
                    }
                    other => {
                        let held = eval(event, other, scope);
                        scope.locals.push((name.as_str(), held));
                    }
                }
                Flow::Next
            }
            Stmt::Assign { name, value } => {
                let held = eval(event, value, scope);
                scope.assign(name, held);
                Flow::Next
            }
            Stmt::Put { path, value } => {
                let held = eval(event, value, scope);
                let _ = event.set(path, held);
                Flow::Next
            }
            Stmt::If { test, body } => {
                if decide(event, test, pattern, scope) {
                    run(event, body, pattern, scope)
                } else {
                    Flow::Next
                }
            }
            Stmt::Walk { index, list, body } => walk(event, index, list, body, pattern, scope),
        };
        if matches!(flow, Flow::Stop) {
            scope.rewind(mark);
            return Flow::Stop;
        }
    }
    scope.rewind(mark);
    Flow::Next
}

/// Run the script against one event.
///
/// Always HANDLED: the opening guard returning early is the script doing its
/// job, and reporting "not handled" there would count a working matcher as a
/// gap.
pub fn item_writes(event: &mut Event, pattern: &ItemWrites) -> bool {
    let mut scope = Scope::default();
    run(event, &pattern.body, pattern, &mut scope);
    true
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::plan::{PainlessPlan, painless_exec_plan};

    /// `pipelines/atlassian_jira/audit/default.yml`, "Add ECS User fields",
    /// verbatim -- in the ESCAPED form the generated call site holds it, so
    /// the test goes through `normalise` the way production does.
    const JIRA: &str = r#"if (ctx?.event?.action == null) {\n    return;\n} if (ctx.group == null) {\n  Map map = new HashMap();\n  ctx.put(\"group\", map);\n} if (ctx.user == null) {\n  Map map = new HashMap();\n  ctx.put(\"user\", map);\n} if (ctx.user?.target == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"target\", map);\n} if (ctx.user?.changes == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"changes\", map);\n} if (ctx.user?.target?.group == null) {\n  Map map = new HashMap();\n  ctx.user.target.put(\"group\", map);\n} if(ctx.jira?.audit?.affected_objects != null) {\n  for (def j = 0; j < ctx.jira?.audit?.affected_objects.length; j++) {\n    if(ctx.jira?.audit?.affected_objects[j]?.type == 'GROUP') {\n      if(['jira.auditing.group.created', 'jira.auditing.group.deleted', 'Group created', 'Group deleted'].contains(ctx.event.action)) {\n        ctx.group.put(\"name\", ctx.jira?.audit?.affected_objects[j]?.name);\n      }\n      if(['jira.auditing.user.added.to.group', 'jira.auditing.user.removed.from.group', 'User added to group', 'User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.group.put(\"name\", ctx.jira?.audit?.affected_objects[j]?.name);\n        ctx.user.target.group.put(\"id\", ctx.jira?.audit?.affected_objects[j]?.id);\n      }\n    }\n    if(ctx.jira?.audit?.affected_objects[j]?.type == 'USER') {\n      if(['jira.auditing.user.created', 'jira.auditing.user.deleted','jira.auditing.user.password.changed','jira.auditing.user.updated','jira.auditing.user.added.to.group', 'jira.auditing.user.removed.from.group', 'User created', 'User deleted', 'User added to group', 'User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"name\", ctx.jira?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.jira?.audit?.affected_objects[j]?.id);\n      }\n    }\n  }\n} if(ctx.jira?.audit?.changed_values != null) {\n  for (def j = 0; j < ctx.jira?.audit?.changed_values.length; j++) {\n    if(['jira.auditing.user.renamed', 'User renamed'].contains(ctx.event.action)) {\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.username') {\n        ctx.user.changes.put(\"name\", ctx.jira?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"name\", ctx.jira?.audit?.changed_values[j]?.from);\n      }\n    }\n    if(['jira.auditing.user.created','jira.auditing.user.updated', 'User created', 'User updated'].contains(ctx.event.action)) {\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.username') {\n        ctx.user.changes.put(\"name\", ctx.jira?.audit?.changed_values[j]?.to);\n        if(ctx.jira?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"name\", ctx.jira?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.email') {\n        ctx.user.changes.put(\"email\", ctx.jira?.audit?.changed_values[j]?.to);\n        if(ctx.jira?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"email\", ctx.jira?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.fullname') {\n        ctx.user.changes.put(\"full_name\", ctx.jira?.audit?.changed_values[j]?.to);\n        if(ctx.jira?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"full_name\", ctx.jira?.audit?.changed_values[j]?.from);\n        }\n      }\n    }\n  }\n}"#;

    /// `pipelines/atlassian_confluence/audit/default.yml`, verbatim. The same
    /// job as jira's with two regexes and a pair of locals in it.
    const CONFLUENCE: &str = r#"if (ctx?.event?.action == null) {\n    return;\n} if (ctx.group == null) {\n  Map map = new HashMap();\n  ctx.put(\"group\", map);\n} if (ctx.user == null) {\n  Map map = new HashMap();\n  ctx.put(\"user\", map);\n} if (ctx.user?.target == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"target\", map);\n} if (ctx.user?.changes == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"changes\", map);\n} if (ctx.user?.target?.group == null) {\n  Map map = new HashMap();\n  ctx.user.target.put(\"group\", map);\n} if(ctx.confluence?.audit?.affected_objects != null) {\n  for (def j = 0; j < ctx.confluence?.audit?.affected_objects.length; j++) {\n    if(ctx.confluence?.audit?.affected_objects[j]?.type == 'Group') {\n      String group_name = ctx.confluence?.audit?.affected_objects[j]?.name;\n      String group_id = ctx.confluence?.audit?.affected_objects[j]?.id;\n      if(ctx._config?.atlassian_cloud != null) {\n          def m = /(.+):(\\b[0-9a-f]{8}\\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\\b[0-9a-f]{12}\\b)$/.matcher(group_name);\n          if (m.find()) {\n            group_name = m.group(1);\n            group_id = m.group(2);\n          }\n      }\n      if(['audit.logging.summary.group.created', 'audit.logging.summary.group.deleted', 'Group created', 'Group deleted'].contains(ctx.event.action)) {\n        ctx.group.put(\"name\", group_name);\n        ctx.group.put(\"id\", group_id);\n      }\n      if(['audit.logging.summary.group.membership.added', 'audit.logging.summary.group.membership.removed', 'User added to group','User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.group.put(\"name\", group_name);\n        ctx.user.target.group.put(\"id\", group_id);\n      }\n    }\n    if(ctx.confluence?.audit?.affected_objects[j]?.type == 'User') {\n      if(['audit.logging.summary.user.created', 'audit.logging.summary.user.deleted', 'audit.logging.summary.user.password.changed','audit.logging.summary.user.updated', 'User created', 'User deleted', 'User details updated'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"full_name\", ctx.confluence?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.confluence?.audit?.affected_objects[j]?.id);\n        if(ctx.confluence?.audit?.affected_objects[j]?.uri != null) {\n          def m = /\\?username=([a-zA-Z0-9._-]+)$/.matcher(ctx.confluence?.audit?.affected_objects[j]?.uri);\n          if (m.find()) {\n            ctx.user.target.put(\"name\", m.group(1));\n          }\n        }\n      }\n      if(['audit.logging.summary.group.membership.added', 'audit.logging.summary.group.membership.removed', 'User added to group', 'User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"name\", ctx.confluence?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.confluence?.audit?.affected_objects[j]?.id);\n      }\n      if(['audit.logging.summary.login.success', 'audit.logging.summary.login.failed'].contains(ctx.event.action)) {\n        ctx.user.put(\"full_name\", ctx.confluence?.audit?.affected_objects[j]?.name);\n        ctx.user.put(\"id\", ctx.confluence?.audit?.affected_objects[j]?.id);\n        if(ctx.confluence?.audit?.affected_objects[j]?.uri != null) {\n          def m = /\\?username=([a-zA-Z0-9._-]+)$/.matcher(ctx.confluence?.audit?.affected_objects[j]?.uri);\n          if (m.find()) {\n            ctx.user.put(\"name\", m.group(1));\n          }\n        }\n      }\n    }\n  }\n} if(ctx.confluence?.audit?.changed_values != null) {\n  for (def j = 0; j < ctx.confluence?.audit?.changed_values.length; j++) {\n    if(['audit.logging.summary.user.renamed', 'User renamed'].contains(ctx.event.action)) {\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'audit.logging.changed.value.username') {\n        ctx.user.changes.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.from);\n      }\n    }\n    if(['audit.logging.summary.user.created','audit.logging.summary.user.updated', 'User created', 'User details updated'].contains(ctx.event.action)) {\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'Username') {\n        ctx.user.changes.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.to);\n          ctx.user.target.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        if(ctx.confluence?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'Email') {\n        ctx.user.changes.put(\"email\", ctx.confluence?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"email\", ctx.confluence?.audit?.changed_values[j]?.to);\n        if(ctx.confluence?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"email\", ctx.confluence?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'Display name') {\n        ctx.user.changes.put(\"full_name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"full_name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        if(ctx.confluence?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"full_name\", ctx.confluence?.audit?.changed_values[j]?.from);\n        }\n      }\n    }\n  }\n}"#;

    /// `pipelines/atlassian_bitbucket/audit/default.yml`, verbatim. Jira's
    /// skeleton with its own action table and a `group.id` jira does not write.
    const BITBUCKET: &str = r#"if (ctx?.event?.action == null) {\n    return;\n} if (ctx.group == null) {\n  Map map = new HashMap();\n  ctx.put(\"group\", map);\n} if (ctx.user == null) {\n  Map map = new HashMap();\n  ctx.put(\"user\", map);\n} if (ctx.user?.target == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"target\", map);\n} if (ctx.user?.changes == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"changes\", map);\n} if (ctx.user?.target?.group == null) {\n  Map map = new HashMap();\n  ctx.user.target.put(\"group\", map);\n} if(ctx.bitbucket?.audit?.affected_objects != null) {\n  for (def j = 0; j < ctx.bitbucket?.audit?.affected_objects.length; j++) {\n    if(ctx.bitbucket?.audit?.affected_objects[j]?.type == 'GROUP') {\n      if(['bitbucket.service.user.audit.action.groupcreated', 'bitbucket.service.user.audit.action.groupdeleted'].contains(ctx.event.action)) {\n        ctx.group.put(\"name\", ctx.bitbucket?.audit?.affected_objects[j]?.name);\n        ctx.group.put(\"id\", ctx.bitbucket?.audit?.affected_objects[j]?.id);\n      }\n      if(['bitbucket.service.user.audit.action.groupmembershipscreated.user', 'bitbucket.service.user.audit.action.groupmembershipdeleted'].contains(ctx.event.action)) {\n        ctx.user.target.group.put(\"name\", ctx.bitbucket?.audit?.affected_objects[j]?.name);\n        ctx.user.target.group.put(\"id\", ctx.bitbucket?.audit?.affected_objects[j]?.id);\n      }\n    }\n    if(ctx.bitbucket?.audit?.affected_objects[j]?.type == 'USER') {\n      if(['bitbucket.service.user.audit.action.usercreated', 'bitbucket.service.user.audit.action.userdeleted', 'bitbucket.service.user.audit.action.userrenamed', 'bitbucket.service.user.audit.action.usercredentialupdated','bitbucket.service.user.audit.action.groupmembershipscreated.user', 'bitbucket.service.user.audit.action.groupmembershipdeleted'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"name\", ctx.bitbucket?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.bitbucket?.audit?.affected_objects[j]?.id);\n      }\n    }\n  }\n} if(ctx.bitbucket?.audit?.changed_values != null) {\n  for (def j = 0; j < ctx.bitbucket?.audit?.changed_values.length; j++) {\n    if(['bitbucket.service.user.audit.action.userrenamed'].contains(ctx.event.action)) {\n      if(ctx.bitbucket?.audit?.changed_values[j]?.i18nKey == 'bitbucket.service.user.audit.attribute.user.name') {\n        ctx.user.changes.put(\"name\", ctx.bitbucket?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"name\", ctx.bitbucket?.audit?.changed_values[j]?.from);\n      }\n    }\n  }\n}"#;

    /// Run a script the way a generated call site does: through the plan, so
    /// the test proves the LADDER reaches this matcher and not only that the
    /// parser can read the text.
    fn through_the_ladder(script: &str, input: Value) -> Event {
        let plan = PainlessPlan::new(script);
        assert!(plan.matches(), "no matcher claims the script");
        let mut event = Event::new(input);
        painless_exec_plan(&mut event, &plan).expect("the plan runs");
        event
    }

    /// jira's `User added to group`, from
    /// `testdata/compat/atlassian_jira/audit/test-audit-api`: the GROUP arm
    /// writes the target group and the USER arm the target user, off the same
    /// list, chosen by each item's own `type`.
    #[test]
    fn each_item_writes_the_arm_its_own_type_selects() {
        let event = through_the_ladder(
            JIRA,
            json!({
                "event": { "action": "jira.auditing.user.added.to.group" },
                "user": { "name": "Anonymous" },
                "jira": { "audit": { "affected_objects": [
                    { "type": "GROUP", "name": "jira-software-users" },
                    { "type": "USER", "name": "test.user", "id": "JIRAUSER10000" }
                ] } }
            }),
        );

        assert_eq!(
            event.get("user.target.group.name"),
            Some(&json!("jira-software-users"))
        );
        assert_eq!(event.get("user.target.name"), Some(&json!("test.user")));
        assert_eq!(event.get("user.target.id"), Some(&json!("JIRAUSER10000")));
        // jira's GROUP arm writes no id, so the group has none to carry.
        assert_eq!(event.get("user.target.group.id"), Some(&json!(null)));
        // The author's own name is left where it was.
        assert_eq!(event.get("user.name"), Some(&json!("Anonymous")));
    }

    /// jira's `user.created`: the changed values fill `user.changes.*`, and
    /// `from` being absent leaves `user.target.name` as the affected object
    /// wrote it. Elastic's captured output for this event carries exactly
    /// these five fields.
    #[test]
    fn a_changed_value_with_no_from_leaves_the_target_alone() {
        let event = through_the_ladder(
            JIRA,
            json!({
                "event": { "action": "jira.auditing.user.created" },
                "jira": { "audit": {
                    "affected_objects": [
                        { "type": "USER", "name": "test.user", "id": "JIRAUSER10000" }
                    ],
                    "changed_values": [
                        { "i18nKey": "common.words.username", "to": "test.user" },
                        { "i18nKey": "common.words.email", "to": "test.user@example.com" },
                        { "i18nKey": "common.words.fullname", "to": "Alex" }
                    ]
                } }
            }),
        );

        assert_eq!(event.get("user.changes.name"), Some(&json!("test.user")));
        assert_eq!(
            event.get("user.changes.email"),
            Some(&json!("test.user@example.com"))
        );
        assert_eq!(event.get("user.changes.full_name"), Some(&json!("Alex")));
        assert_eq!(event.get("user.target.name"), Some(&json!("test.user")));
        assert_eq!(event.get("user.target.id"), Some(&json!("JIRAUSER10000")));
    }

    /// jira's `user.renamed` writes the OLD name to the target and the new one
    /// to the changes, with no guard on either -- so a missing `from` stores an
    /// explicit null rather than keeping what was there.
    #[test]
    fn the_rename_arm_writes_both_ends_unguarded() {
        let event = through_the_ladder(
            JIRA,
            json!({
                "event": { "action": "jira.auditing.user.renamed" },
                "jira": { "audit": { "changed_values": [
                    { "i18nKey": "common.words.username", "to": "admin.user1", "from": "admin.user" }
                ] } }
            }),
        );
        assert_eq!(event.get("user.changes.name"), Some(&json!("admin.user1")));
        assert_eq!(event.get("user.target.name"), Some(&json!("admin.user")));
    }

    /// An action outside every table writes nothing but the empty containers
    /// the script allocates, which the drop-empty script after it removes.
    #[test]
    fn an_action_no_table_names_writes_nothing() {
        let event = through_the_ladder(
            JIRA,
            json!({
                "event": { "action": "jira.auditing.project.created" },
                "jira": { "audit": { "affected_objects": [
                    { "type": "USER", "name": "test.user" }
                ] } }
            }),
        );
        // Only the containers the script allocates, `user.target.group`
        // nested inside `user.target` the way Painless puts it there.
        assert_eq!(event.get("user.target"), Some(&json!({ "group": {} })));
        assert_eq!(event.get("group"), Some(&json!({})));
    }

    /// The opening guard leaves the script before it allocates anything.
    #[test]
    fn no_action_returns_before_the_allocations() {
        let event = through_the_ladder(JIRA, json!({ "jira": { "audit": {} } }));
        assert!(event.get("group").is_none());
        assert!(event.get("user").is_none());
    }

    /// confluence's cloud stream carries the group id INSIDE the name, and the
    /// script splits it back out with a regex -- but only where `_config`
    /// marks the stream cloud-hosted. Captured output for
    /// `test-audit-cloud[7]` is the group name without its uuid and the uuid
    /// as the id.
    #[test]
    fn the_cloud_group_name_is_split_back_into_name_and_id() {
        let event = through_the_ladder(
            CONFLUENCE,
            json!({
                "_config": { "atlassian_cloud": true },
                "event": { "action": "User added to group" },
                "confluence": { "audit": { "affected_objects": [
                    {
                        "type": "Group",
                        "name": "jira-software-users:b8d81944-e737-4da0-94ca-165fa5c0635c"
                    },
                    { "type": "User", "name": "ASP" }
                ] } }
            }),
        );
        assert_eq!(
            event.get("user.target.group.name"),
            Some(&json!("jira-software-users"))
        );
        assert_eq!(
            event.get("user.target.group.id"),
            Some(&json!("b8d81944-e737-4da0-94ca-165fa5c0635c"))
        );
        assert_eq!(event.get("user.target.name"), Some(&json!("ASP")));
    }

    /// Without the cloud marker the same name is left whole, because the regex
    /// is behind a guard on `_config` rather than on the name.
    #[test]
    fn the_self_hosted_group_name_is_left_whole() {
        let event = through_the_ladder(
            CONFLUENCE,
            json!({
                "event": { "action": "audit.logging.summary.group.created" },
                "confluence": { "audit": { "affected_objects": [
                    { "type": "Group", "name": "confluence-users", "id": "confluence-users" }
                ] } }
            }),
        );
        assert_eq!(event.get("group.name"), Some(&json!("confluence-users")));
        assert_eq!(event.get("group.id"), Some(&json!("confluence-users")));
    }

    /// confluence reads the target user's NAME out of the affected object's
    /// `uri`, and its display name out of `name` -- two different ECS fields
    /// off one item.
    #[test]
    fn the_target_user_name_comes_out_of_the_uri() {
        let event = through_the_ladder(
            CONFLUENCE,
            json!({
                "event": { "action": "audit.logging.summary.user.updated" },
                "confluence": { "audit": { "affected_objects": [
                    {
                        "type": "User",
                        "name": "asdf asdfasdf",
                        "id": "2c9680837d4a3682017d67821e520003",
                        "uri": "/confluence/admin/users/viewuser.action?username=asdf123"
                    }
                ] } }
            }),
        );
        assert_eq!(
            event.get("user.target.full_name"),
            Some(&json!("asdf asdfasdf"))
        );
        assert_eq!(event.get("user.target.name"), Some(&json!("asdf123")));
        assert_eq!(
            event.get("user.target.id"),
            Some(&json!("2c9680837d4a3682017d67821e520003"))
        );
    }

    /// confluence's login arms write the AUTHOR's fields, not the target's --
    /// the same item, a different destination, chosen by the action table.
    #[test]
    fn the_login_arm_writes_the_author_not_the_target() {
        let event = through_the_ladder(
            CONFLUENCE,
            json!({
                "event": { "action": "audit.logging.summary.login.success" },
                "confluence": { "audit": { "affected_objects": [
                    { "type": "User", "name": "Joe Bob", "id": "-2" }
                ] } }
            }),
        );
        assert_eq!(event.get("user.full_name"), Some(&json!("Joe Bob")));
        assert_eq!(event.get("user.id"), Some(&json!("-2")));
        // The target arms did not fire, so nothing but the allocation is there.
        assert_eq!(event.get("user.target"), Some(&json!({ "group": {} })));
    }

    /// bitbucket's own table, with the `group.id` jira's script never writes.
    #[test]
    fn bitbucket_writes_its_own_table() {
        let event = through_the_ladder(
            BITBUCKET,
            json!({
                "event": { "action": "bitbucket.service.user.audit.action.groupcreated" },
                "bitbucket": { "audit": { "affected_objects": [
                    { "type": "GROUP", "name": "asdf", "id": "asdf" }
                ] } }
            }),
        );
        assert_eq!(event.get("group.name"), Some(&json!("asdf")));
        assert_eq!(event.get("group.id"), Some(&json!("asdf")));
    }

    /// bitbucket's rename, off `changed_values` alone.
    #[test]
    fn bitbucket_reads_its_own_changed_value_key() {
        let event = through_the_ladder(
            BITBUCKET,
            json!({
                "event": { "action": "bitbucket.service.user.audit.action.userrenamed" },
                "bitbucket": { "audit": {
                    "affected_objects": [
                        { "type": "USER", "name": "test", "id": "3" }
                    ],
                    "changed_values": [
                        {
                            "i18nKey": "bitbucket.service.user.audit.attribute.user.name",
                            "to": "test.user",
                            "from": "test"
                        }
                    ]
                } }
            }),
        );
        assert_eq!(event.get("user.changes.name"), Some(&json!("test.user")));
        assert_eq!(event.get("user.target.name"), Some(&json!("test")));
        assert_eq!(event.get("user.target.id"), Some(&json!("3")));
    }

    /// mattermost ships the same processor as an if/else-if chain over fields
    /// rather than a walk. Reading only the `if` half would write the branch
    /// the vendor did not take, so the whole script declines.
    #[test]
    fn an_else_arm_declines() {
        let script = "if (['patchUser'].contains(ctx.event.action)) {\n  \
             if(ctx.user.target.name != ctx.mattermost?.audit?.patch?.name) {\n    \
             ctx.user.changes.put(\"name\", ctx.mattermost?.audit?.patch?.name);\n  }\n\
             } else if (['createTeam'].contains(ctx.event.action)) {\n  \
             ctx.group.put(\"name\", ctx.mattermost?.audit?.team?.name);\n}";
        assert!(parse_item_writes(script).is_none());
    }

    /// A statement the grammar does not hold declines the WHOLE script, even
    /// though every other statement in it reads cleanly.
    #[test]
    fn one_unreadable_statement_declines_the_whole_script() {
        let readable = "if(ctx.a?.items != null) {\n  \
             for (def j = 0; j < ctx.a?.items.length; j++) {\n    \
             ctx.user.put(\"name\", ctx.a?.items[j]?.name);\n  }\n}";
        assert!(parse_item_writes(readable).is_some());

        let with_a_call = "if(ctx.a?.items != null) {\n  \
             for (def j = 0; j < ctx.a?.items.length; j++) {\n    \
             ctx.user.put(\"name\", ctx.a?.items[j]?.name.toLowerCase());\n  }\n}";
        assert!(parse_item_writes(with_a_call).is_none());
    }

    /// A name no statement binds declines: an item read outside the walk over
    /// its list means the index is somebody else's.
    #[test]
    fn an_unbound_name_declines() {
        let script = "if(ctx.a?.items != null) {\n  \
             for (def j = 0; j < ctx.a?.items.length; j++) {\n    \
             ctx.user.put(\"name\", ctx.b?.other[j]?.name);\n  }\n}";
        assert!(parse_item_writes(script).is_none());
    }

    /// A Java regex Rust cannot build declines at PARSE time. A lookahead is
    /// the pattern that would otherwise panic on the first event to reach the
    /// call site.
    #[test]
    fn a_regex_rust_cannot_build_declines() {
        let script = "if(ctx.a?.items != null) {\n  \
             for (def j = 0; j < ctx.a?.items.length; j++) {\n    \
             def m = /(?!x)(.+)/.matcher(ctx.a?.items[j]?.name);\n    \
             if (m.find()) {\n      ctx.user.put(\"name\", m.group(1));\n    }\n  }\n}";
        assert!(parse_item_writes(script).is_none());
    }

    /// A walk with no write, and a write with no walk, are both somebody
    /// else's pattern -- this arm sits last and must only take its own.
    #[test]
    fn a_script_without_both_a_walk_and_a_write_declines() {
        assert!(
            parse_item_writes(
                "if(ctx.a?.items != null) {\n  \
                 for (def j = 0; j < ctx.a?.items.length; j++) {\n    \
                 def m = /(.+)/.matcher(ctx.a?.items[j]?.name);\n  }\n}"
            )
            .is_none()
        );
        assert!(parse_item_writes("ctx.user.put(\"name\", ctx.a?.name);").is_none());
    }

    /// The scanner counts nesting in CODE only: a regex literal's `{8}` and
    /// `(.+)` are text, and counting either would cut the block short.
    #[test]
    fn a_regex_literals_braces_are_not_nesting() {
        let script = "if(ctx.a?.items != null) {\n  \
             for (def j = 0; j < ctx.a?.items.length; j++) {\n    \
             def m = /(.+):([0-9a-f]{4})$/.matcher(ctx.a?.items[j]?.name);\n    \
             if (m.find()) {\n      ctx.user.put(\"name\", m.group(1));\n      \
             ctx.user.put(\"id\", m.group(2));\n    }\n  }\n}";
        let pattern = parse_item_writes(script).expect("the literal is text, not nesting");

        let mut event = Event::new(json!({ "a": { "items": [{ "name": "host:beef" }] } }));
        assert!(item_writes(&mut event, &pattern));
        assert_eq!(event.get("user.name"), Some(&json!("host")));
        assert_eq!(event.get("user.id"), Some(&json!("beef")));
    }
}
