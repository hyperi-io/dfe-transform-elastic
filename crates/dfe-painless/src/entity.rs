// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! aws cloudtrail's entity-classification script, read off its own text.
//!
//! One ~950-line Painless script owns the whole `*.entity.id` family for
//! cloudtrail: two dozen per-service `enrich*` functions gather related and
//! target entities into `TreeSet`s, then a classifier sorts every entity into
//! user, host, service or generic by the tables in `params` and the pattern of
//! the value itself. The per-service functions are PARSED, not transcribed --
//! a vendor adding a service or an event name flows through regeneration --
//! while the classifier tail is fixed logic driven by the params tables, with
//! its distinctive markers asserted so a restructured script declines loudly
//! rather than half-running.
//!
//! The parse runs once per call site: [`crate::params::params_pattern`]
//! only names this matcher when [`EntityScript::parse`] succeeds, and the
//! parsed script rides inside the pattern. A statement the parser cannot read
//! fails the WHOLE parse -- the script then falls through the dispatch as it
//! did before this matcher existed, and the corpus ratchet says so.

use std::collections::BTreeSet;

use serde_json::{Map, Value};

use dfe_core::event::Event;

/// Which of the script's two collection sets a statement feeds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SetRef {
    Related,
    Target,
}

/// One statement of an enrich function or the main body's preamble.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    /// `addField(enrichCtx.X, "path")` -- read the event path, add the string.
    AddField(SetRef, String),
    /// `addFields(enrichCtx.X, [ ... ])`.
    AddFields(SetRef, Vec<String>),
    /// `addValue(enrichCtx.X, String.valueOf(<local>))` -- a numeric read
    /// rendered as text, cloudtrail's ACL rule number.
    AddFieldAsString(SetRef, String),
    /// `addValues(enrichCtx.X, $("path", []))` -- a whole string array.
    AddArray(SetRef, String),
    /// `$("path", []).stream().forEach(v -> ...)`, or the nested
    /// `v.member?.stream().forEach(...)` whose source resolves in scope.
    ForEach {
        var: String,
        source: Source,
        body: Vec<Action>,
    },
    /// `addValue(enrichCtx.X, v.member)` inside a forEach; an empty member
    /// path adds the element itself.
    AddMember(SetRef, String),
    /// `addValues(enrichCtx.X, [ v.a, v.b ])` inside a forEach.
    AddMembers(SetRef, Vec<String>),
    /// An `eventName ==` ladder arm, with the chain folded into `otherwise`.
    IfEvent {
        names: Vec<String>,
        then: Vec<Action>,
        otherwise: Vec<Action>,
    },
    /// `eventName.contains("X")` -- lambda's versioned event names.
    IfEventContains {
        needles: Vec<String>,
        then: Vec<Action>,
        otherwise: Vec<Action>,
    },
    /// `if (<read> == "X")` over a `$()` read or a local bound to one.
    IfRead {
        path: String,
        values: Vec<String>,
        then: Vec<Action>,
        otherwise: Vec<Action>,
    },
    /// `if (<local> != null)` over a local bound to a read.
    IfPresent { path: String, then: Vec<Action> },
    /// `enrichCtx.actor = $("path", null)`.
    SetActor(String),
    /// The main body's `if (enrichCtx.actor == null) { actor = $(...) }`.
    SetActorIfAbsent(String),
    /// ssm `SendCommand`'s instance-id gathering, recognised by its `flatMap`:
    /// requestParameters.instanceIds, else every targets[].values[], and a
    /// lone `*` becomes the recipient account.
    SendCommand,
}

/// Where a forEach reads its array from.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Source {
    /// `$("path", [])` -- an event path.
    Event(String),
    /// `v.member.path` -- a member of the enclosing loop's element.
    Member(String),
}

/// One `void enrich*` function: its `eventSource` gate and its statements.
#[derive(Debug, Clone, PartialEq, Eq)]
struct EnrichFn {
    source: String,
    body: Vec<Action>,
}

/// The whole script, parsed once per call site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityScript {
    enrich: Vec<EnrichFn>,
    preamble: Vec<Action>,
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

/// The text with `//` comments removed, quote-aware.
fn strip_comments(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        let mut in_string: Option<char> = None;
        let mut cut = line.len();
        let bytes = line.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i] as char;
            match in_string {
                Some(quote) if c == quote => in_string = None,
                None if c == '"' || c == '\'' => in_string = Some(c),
                None if c == '/' && bytes.get(i + 1) == Some(&b'/') => {
                    cut = i;
                    break;
                }
                Some(_) | None => {}
            }
            i += 1;
        }
        out.push_str(&line[..cut]);
        out.push('\n');
    }
    out
}

/// The block opened by the FIRST `{` in `text`, and what follows its close.
fn brace_block(text: &str) -> Option<(&str, &str)> {
    let open = text.find('{')?;
    let mut depth = 0usize;
    for (at, c) in text[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let inner = &text[open + 1..open + at];
                    return Some((inner, &text[open + at + 1..]));
                }
            }
            _ => {}
        }
    }
    None
}

/// The parenthesised argument list opening at `text`'s first byte.
fn paren_args(text: &str) -> Option<(&str, &str)> {
    if !text.starts_with('(') {
        return None;
    }
    let mut depth = 0usize;
    let mut in_string: Option<char> = None;
    for (at, c) in text.char_indices() {
        match in_string {
            Some(quote) if c == quote => in_string = None,
            Some(_) => {}
            None => match c {
                '"' | '\'' => in_string = Some(c),
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some((&text[1..at], &text[at + 1..]));
                    }
                }
                _ => {}
            },
        }
    }
    None
}

/// The first quoted literal in `text`.
fn quoted(text: &str) -> Option<String> {
    let start = text.find(['"', '\''])?;
    let quote = text[start..].chars().next()?;
    let rest = &text[start + 1..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

/// Every quoted literal in `text`, in order.
/// `enrichCtx.related` / `enrichCtx.target` as a [`SetRef`].
fn set_ref(text: &str) -> Option<SetRef> {
    match text.trim() {
        "enrichCtx.related" => Some(SetRef::Related),
        "enrichCtx.target" => Some(SetRef::Target),
        _ => None,
    }
}

/// A `$("path", <default>)` read's path.
fn dollar_path(text: &str) -> Option<String> {
    let rest = text.trim().strip_prefix("$(")?;
    quoted(rest)
}

/// A member expression `v.a?.b` as (var, member path with the `?`s dropped).
fn member_expr(text: &str) -> Option<(String, String)> {
    let text = text.trim();
    if text.is_empty()
        || !text
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '?' || c == '_')
    {
        return None;
    }
    let clean = text.replace('?', "");
    match clean.split_once('.') {
        Some((var, member)) => Some((var.to_string(), member.to_string())),
        None => Some((clean, String::new())),
    }
}

/// The parser's running state: locals bound to `$()` reads, and the loop
/// variables in scope, innermost last.
#[derive(Default)]
struct ParseScope {
    locals: Vec<(String, String)>,
    loop_vars: Vec<String>,
}

impl ParseScope {
    fn local_path(&self, name: &str) -> Option<&str> {
        self.locals
            .iter()
            .rev()
            .find(|(local, _)| local == name)
            .map(|(_, path)| path.as_str())
    }

    fn is_loop_var(&self, name: &str) -> bool {
        self.loop_vars.iter().any(|v| v == name)
    }
}

/// Parse a statement block into actions, or `None` on anything unreadable --
/// a partial read must not half-run a script.
#[allow(clippy::too_many_lines)] // One arm per statement form; splitting them hides the grammar.
fn parse_actions(mut text: &str, scope: &mut ParseScope) -> Option<Vec<Action>> {
    let mut out = Vec::new();

    loop {
        text = text.trim_start();
        if text.is_empty() {
            return Some(out);
        }

        // `addFields(enrichCtx.X, [ "p", ... ]);` -- before `addField`, whose
        // prefix it shares.
        if let Some(rest) = text.strip_prefix("addFields") {
            let (args, tail) = paren_args(rest)?;
            let (set, list) = args.split_once(',')?;
            out.push(Action::AddFields(
                set_ref(set)?,
                crate::common::quoted_members(list),
            ));
            text = tail.trim_start().strip_prefix(';')?;
            continue;
        }

        // `addValues(enrichCtx.X, <list or array read>);`
        if let Some(rest) = text.strip_prefix("addValues") {
            let (args, tail) = paren_args(rest)?;
            let (set, expr) = args.split_once(',')?;
            let set = set_ref(set)?;
            let expr = expr.trim();
            if expr.starts_with('[') {
                let inner = expr.strip_prefix('[')?.trim_end().strip_suffix(']')?;
                let mut members = Vec::new();
                for piece in inner.split(',') {
                    let (var, member) = member_expr(piece)?;
                    if !scope.is_loop_var(&var) {
                        return None;
                    }
                    members.push(member);
                }
                out.push(Action::AddMembers(set, members));
            } else {
                out.push(Action::AddArray(set, dollar_path(expr)?));
            }
            text = tail.trim_start().strip_prefix(';')?;
            continue;
        }

        // `addField(enrichCtx.X, "path");`
        if let Some(rest) = text.strip_prefix("addField") {
            let (args, tail) = paren_args(rest)?;
            let (set, path) = args.split_once(',')?;
            out.push(Action::AddField(set_ref(set)?, quoted(path)?));
            text = tail.trim_start().strip_prefix(';')?;
            continue;
        }

        // `addValue(enrichCtx.X, <expr>);`
        if let Some(rest) = text.strip_prefix("addValue") {
            let (args, tail) = paren_args(rest)?;
            let (set, expr) = args.split_once(',')?;
            let set = set_ref(set)?;
            let expr = expr.trim();
            if let Some(path) = dollar_path(expr) {
                out.push(Action::AddField(set, path));
            } else if let Some(inner) = expr.strip_prefix("String.valueOf(") {
                let local = inner.trim_end().strip_suffix(')')?.trim();
                out.push(Action::AddFieldAsString(
                    set,
                    scope.local_path(local)?.to_string(),
                ));
            } else {
                let (var, member) = member_expr(expr)?;
                if !scope.is_loop_var(&var) {
                    return None;
                }
                out.push(Action::AddMember(set, member));
            }
            text = tail.trim_start().strip_prefix(';')?;
            continue;
        }

        // `$("p", []).stream().forEach(v -> ...);` and the member-relative
        // `v.member?.stream().forEach(...)` form.
        if let Some(at) = text.find(".stream().forEach(")
            && text[..at].find(['\n', ';']).is_none()
        {
            let source_text = text[..at].trim();
            let source = if let Some(path) = dollar_path(source_text) {
                Source::Event(path)
            } else {
                let (var, member) = member_expr(source_text)?;
                if !scope.is_loop_var(&var) {
                    return None;
                }
                Source::Member(member)
            };

            let rest = &text[at + ".stream().forEach".len()..];
            let (lambda, tail) = paren_args(rest)?;
            let (var, body_text) = lambda.split_once("->")?;
            let var = var.trim().to_string();

            scope.loop_vars.push(var.clone());
            let body = if body_text.trim_start().starts_with('{') {
                let (inner, after) = brace_block(body_text)?;
                if !after.trim().is_empty() {
                    scope.loop_vars.pop();
                    return None;
                }
                parse_actions(inner, scope)
            } else {
                parse_actions(&format!("{};", body_text.trim()), scope)
            };
            scope.loop_vars.pop();

            out.push(Action::ForEach {
                var,
                source,
                body: body?,
            });
            text = tail.trim_start().strip_prefix(';')?;
            continue;
        }

        // Local bindings: `def x = $("p", d);` -- only reads are supported,
        // and only String/def/List spellings appear.
        if let Some(rest) = text
            .strip_prefix("def ")
            .or_else(|| text.strip_prefix("String "))
            .or_else(|| text.strip_prefix("List "))
        {
            let (statement, tail) = rest.split_once(';')?;
            let (name, value) = statement.split_once('=')?;
            let path = dollar_path(value)?;
            scope.locals.push((name.trim().to_string(), path));
            text = tail;
            continue;
        }

        // `enrichCtx.actor = $("p", null);`
        if let Some(rest) = text.strip_prefix("enrichCtx.actor") {
            let (statement, tail) = rest.split_once(';')?;
            let value = statement.trim().strip_prefix('=')?;
            out.push(Action::SetActor(dollar_path(value)?));
            text = tail;
            continue;
        }

        // The if/else-if ladders.
        if text.starts_with("if ") || text.starts_with("if(") {
            let (actions, tail) = parse_one_if(text, scope)?;
            out.extend(actions);
            text = tail;
            continue;
        }

        // A bare `return;` ends the readable statements of this block.
        if let Some(rest) = text.strip_prefix("return") {
            let (_, tail) = rest.split_once(';')?;
            text = tail;
            continue;
        }

        return None;
    }
}

/// Parse one `if ...` statement with its whole else-chain, returning its
/// actions and the text after the chain.
fn parse_one_if<'t>(text: &'t str, scope: &mut ParseScope) -> Option<(Vec<Action>, &'t str)> {
    let rest = text.strip_prefix("if")?.trim_start();
    let (condition, after) = paren_args(rest)?;
    let (block, mut tail) = brace_block(after)?;
    let condition = condition.trim();

    // A null guard whose block only returns carries nothing: evaluation
    // treats a missing value the same way the guard would.
    let returns_only = matches!(block.trim(), "return;" | "return true;" | "return");
    if returns_only && condition.ends_with("== null") {
        return Some((Vec::new(), tail));
    }

    // ssm's SendCommand arm, recognised by its flatMap before any general
    // parse -- its instance-id gathering is fixed logic.
    let then = if block.contains("flatMap(") {
        vec![Action::SendCommand]
    } else {
        parse_actions(block, scope)?
    };

    let otherwise = {
        let trimmed = tail.trim_start();
        if let Some(chain) = trimmed.strip_prefix("else") {
            let chain = chain.trim_start();
            if chain.starts_with("if ") || chain.starts_with("if(") {
                let (actions, rest) = parse_one_if(chain, scope)?;
                tail = rest;
                actions
            } else {
                let (inner, rest) = brace_block(chain)?;
                tail = rest;
                parse_actions(inner, scope)?
            }
        } else {
            Vec::new()
        }
    };

    let action = condition_action(condition, scope, then, otherwise)?;
    Some((vec![action], tail))
}

/// Read a condition into the action that evaluates it.
fn condition_action(
    condition: &str,
    scope: &ParseScope,
    then: Vec<Action>,
    otherwise: Vec<Action>,
) -> Option<Action> {
    // `eventName == "A" || eventName == "B" ...`
    if condition.starts_with("eventName ==") || condition.starts_with("eventName==") {
        let mut names = Vec::new();
        for term in condition.split("||") {
            let term = term.trim();
            let rest = term.strip_prefix("eventName")?.trim_start();
            let rest = rest.strip_prefix("==")?;
            names.push(quoted(rest)?);
        }
        return Some(Action::IfEvent {
            names,
            then,
            otherwise,
        });
    }

    // `eventName.contains("X")`
    if condition.starts_with("eventName.contains(") {
        let mut needles = Vec::new();
        for term in condition.split("||") {
            let rest = term.trim().strip_prefix("eventName.contains")?;
            needles.push(quoted(rest)?);
        }
        return Some(Action::IfEventContains {
            needles,
            then,
            otherwise,
        });
    }

    // `enrichCtx.actor == null` -- the main body's fallback actor.
    if condition == "enrichCtx.actor == null" {
        let path = match then.as_slice() {
            [Action::SetActor(path)] => path.clone(),
            _ => return None,
        };
        return Some(Action::SetActorIfAbsent(path));
    }

    // `<local or read> == "X"` -- sts's userType ladder and the main body's
    // IdentityCenterUser check.
    if let Some((lhs, rhs)) = condition.split_once("==") {
        let lhs = lhs.trim();
        let path = dollar_path(lhs).or_else(|| scope.local_path(lhs).map(str::to_string))?;
        let mut values = vec![quoted(rhs)?];
        // A disjunction over one subject reads each term's literal.
        for term in condition.split("||").skip(1) {
            values.push(quoted(term)?);
        }
        return Some(Action::IfRead {
            path,
            values,
            then,
            otherwise,
        });
    }

    // `<local> != null`
    if let Some(lhs) = condition.strip_suffix("!= null") {
        let path = scope.local_path(lhs.trim())?.to_string();
        if !otherwise.is_empty() {
            return None;
        }
        return Some(Action::IfPresent { path, then });
    }

    None
}

/// The markers the fixed classification tail is keyed on. Each one names a
/// piece of logic the evaluator hardcodes; a script missing any has changed
/// pattern, and the parse declines rather than half-running.
const CLASSIFIER_MARKERS: [&str; 10] = [
    r#"field("target.entity.id").set(enrichCtx.target)"#,
    "params.userResourceTypes.containsKey",
    "params.hostResourceTypes.containsKey",
    "params.serviceResourceTypes.containsKey",
    "params.hostIdPrefixes",
    "params.serviceIdPrefixes",
    "params.userIdentityTypes.containsKey",
    r#"startsWith("AKIA")"#,
    r#""/aws-service-role/""#,
    r#"field("related.entity").set(enrichCtx.related)"#,
];

impl EntityScript {
    /// Parse the script, or `None` when any statement is unreadable.
    #[must_use]
    pub fn parse(script: &str) -> Option<Self> {
        let text = strip_comments(script);

        // Every `void enrich*` function: gate first, then statements.
        let mut enrich = Vec::new();
        let mut rest = text.as_str();
        while let Some(at) = rest.find("void enrich") {
            let (body, after) = brace_block(&rest[at..])?;
            rest = after;

            // `if (eventSource != "X") { return; }` opens every function.
            let gated = body.trim_start();
            let gate = gated.strip_prefix("if")?.trim_start();
            let (condition, after_gate) = paren_args(gate)?;
            let source = condition
                .trim()
                .strip_prefix("eventSource")
                .and_then(|rest| rest.trim_start().strip_prefix("!="))
                .and_then(quoted)?;
            let (gate_block, statements) = brace_block(after_gate)?;
            if gate_block.trim() != "return;" {
                return None;
            }

            let mut scope = ParseScope::default();
            enrich.push(EnrichFn {
                source,
                body: parse_actions(statements, &mut scope)?,
            });
        }
        if enrich.is_empty() {
            return None;
        }

        // The main body runs from the enrichCtx initialisation to the
        // classification tail, which is matched by marker rather than parsed.
        let main_at = rest.find("Map enrichCtx")?;
        let main = &rest[main_at..];
        let tail_at = main.find("if (!enrichCtx.target.isEmpty())")?;
        let (preamble_text, tail) = main.split_at(tail_at);

        for marker in CLASSIFIER_MARKERS {
            if !tail.contains(marker) {
                return None;
            }
        }

        let mut preamble = Vec::new();
        let mut scope = ParseScope::default();
        for statement in readable_preamble(preamble_text) {
            preamble.extend(parse_actions(&statement, &mut scope)?);
        }

        Some(Self { enrich, preamble })
    }
}

/// The preamble's statements, with the ones that carry no behaviour --
/// initialisation, the eventSource/eventName locals, and the enrich calls the
/// evaluator applies itself -- dropped before parsing.
fn readable_preamble(text: &str) -> Vec<String> {
    let mut kept = Vec::new();
    let mut rest = text.trim_start();
    while !rest.is_empty() {
        // Statements end at `;`, except an if-block which ends at its braces.
        let statement = if rest.starts_with("if ") || rest.starts_with("if(") {
            let Some((_, after)) = paren_args(rest.trim_start_matches("if").trim_start()) else {
                break;
            };
            let Some((_, tail)) = brace_block(after) else {
                break;
            };
            let taken = &rest[..rest.len() - tail.len()];
            let piece = taken.to_string();
            rest = tail.trim_start();
            piece
        } else {
            let Some(end) = rest.find(';') else { break };
            let piece = rest[..=end].to_string();
            rest = rest[end + 1..].trim_start();
            piece
        };

        let flat = statement.split_whitespace().collect::<Vec<_>>().join(" ");
        let skippable = flat.starts_with("Map enrichCtx")
            || flat.starts_with("enrichCtx.related = new")
            || flat.starts_with("enrichCtx.target = new")
            || flat.starts_with("String eventSource")
            || flat.starts_with("String eventName")
            || (flat.starts_with("enrich") && flat.contains("(enrichCtx, eventSource, eventName)"));
        if !skippable {
            kept.push(statement);
        }
    }
    kept
}

// ---------------------------------------------------------------------------
// Evaluation
// ---------------------------------------------------------------------------

/// What the collection phase gathers before anything is written.
#[derive(Default)]
struct Collected {
    related: BTreeSet<String>,
    target: BTreeSet<String>,
    actor: Option<String>,
}

impl Collected {
    fn set_mut(&mut self, set: SetRef) -> &mut BTreeSet<String> {
        match set {
            SetRef::Related => &mut self.related,
            SetRef::Target => &mut self.target,
        }
    }

    /// `addValue`'s own rule: non-null, non-empty STRINGS only -- the script
    /// takes a String parameter, so anything else would have thrown.
    fn add(&mut self, set: SetRef, value: &Value) {
        if let Some(text) = value.as_str()
            && !text.is_empty()
        {
            self.set_mut(set).insert(text.to_string());
        }
    }
}

/// A value the way `String.valueOf` renders the JSON numbers that reach it.
fn value_of(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

/// The value a member path names on an element; empty path is the element.
fn member<'v>(element: &'v Value, path: &str) -> Option<&'v Value> {
    if path.is_empty() {
        return Some(element);
    }
    let mut current = element;
    for segment in path.split('.') {
        current = current.as_object()?.get(segment)?;
    }
    Some(current)
}

/// The elements in scope, innermost last.
type Scope<'v> = Vec<(String, &'v Value)>;

fn eval_actions<'v>(
    event: &'v Event,
    actions: &[Action],
    scope: &mut Scope<'v>,
    out: &mut Collected,
) {
    for action in actions {
        eval_action(event, action, scope, out);
    }
}

#[allow(clippy::too_many_lines)] // One arm per statement form, same as the parser.
fn eval_action<'v>(event: &'v Event, action: &Action, scope: &mut Scope<'v>, out: &mut Collected) {
    match action {
        Action::AddField(set, path) => {
            if let Some(value) = event.get(path) {
                out.add(*set, value);
            }
        }
        Action::AddFields(set, paths) => {
            for path in paths {
                if let Some(value) = event.get(path) {
                    out.add(*set, value);
                }
            }
        }
        Action::AddFieldAsString(set, path) => {
            if let Some(text) = event.get(path).and_then(value_of) {
                out.set_mut(*set).insert(text);
            }
        }
        Action::AddArray(set, path) => {
            if let Some(Value::Array(items)) = event.get(path) {
                for item in items {
                    out.add(*set, item);
                }
            }
        }
        Action::ForEach { var, source, body } => {
            let items = match source {
                Source::Event(path) => event.get(path),
                Source::Member(path) => scope.last().and_then(|(_, element)| member(element, path)),
            };
            let Some(Value::Array(items)) = items else {
                return;
            };
            for item in items {
                scope.push((var.clone(), item));
                eval_actions(event, body, scope, out);
                scope.pop();
            }
        }
        Action::AddMember(set, path) => {
            if let Some((_, element)) = scope.last()
                && let Some(value) = member(element, path)
            {
                out.add(*set, value);
            }
        }
        Action::AddMembers(set, paths) => {
            if let Some((_, element)) = scope.last() {
                let element = *element;
                for path in paths {
                    if let Some(value) = member(element, path) {
                        out.add(*set, value);
                    }
                }
            }
        }
        Action::IfEvent {
            names,
            then,
            otherwise,
        } => {
            let held = event
                .get_str("json.eventName")
                .is_some_and(|name| names.iter().any(|n| n == name));
            eval_actions(event, if held { then } else { otherwise }, scope, out);
        }
        Action::IfEventContains {
            needles,
            then,
            otherwise,
        } => {
            let held = event
                .get_str("json.eventName")
                .is_some_and(|name| needles.iter().any(|n| name.contains(n.as_str())));
            eval_actions(event, if held { then } else { otherwise }, scope, out);
        }
        Action::IfRead {
            path,
            values,
            then,
            otherwise,
        } => {
            let held = event
                .get_str(path)
                .is_some_and(|read| values.iter().any(|v| v == read));
            eval_actions(event, if held { then } else { otherwise }, scope, out);
        }
        Action::IfPresent { path, then } => {
            if event.get(path).is_some_and(|v| !v.is_null()) {
                eval_actions(event, then, scope, out);
            }
        }
        Action::SetActor(path) => {
            out.actor = event.get_str(path).map(str::to_string);
        }
        Action::SetActorIfAbsent(path) => {
            if out.actor.is_none() {
                out.actor = event.get_str(path).map(str::to_string);
            }
        }
        Action::SendCommand => {
            let mut ids: Vec<String> = match event.get("json.requestParameters.instanceIds") {
                Some(Value::Array(items)) => items.iter().filter_map(value_of).collect(),
                _ => Vec::new(),
            };
            if ids.is_empty()
                && let Some(Value::Array(targets)) = event.get("json.requestParameters.targets")
            {
                for target in targets {
                    if let Some(Value::Array(values)) =
                        target.as_object().and_then(|t| t.get("values"))
                    {
                        ids.extend(values.iter().filter_map(value_of));
                    }
                }
            }
            if ids.len() == 1 && ids[0] == "*" {
                ids = event
                    .get_str("json.recipientAccountId")
                    .map(|account| vec![account.to_string()])
                    .unwrap_or_default();
            }
            for id in ids {
                if !id.is_empty() {
                    out.target.insert(id);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Classification -- the script's fixed tail, driven by the params tables.
// ---------------------------------------------------------------------------

/// Where one entity value lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    User,
    Host,
    Service,
    Generic,
}

/// A params table's `containsKey`.
fn table_has(params: &Map<String, Value>, table: &str, key: &str) -> bool {
    params
        .get(table)
        .and_then(Value::as_object)
        .is_some_and(|t| t.contains_key(key))
}

/// A params list's members.
fn prefixes<'p>(params: &'p Map<String, Value>, list: &str) -> Vec<&'p str> {
    params
        .get(list)
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

/// An ARN's resource type: field 6 of the colon split, up to its first `/`.
fn arn_resource_type(target: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = target.split(':').collect();
    if parts.len() < 6 {
        return None;
    }
    let resource = parts[5];
    let resource_type = match resource.find('/') {
        Some(at) if at > 0 => &resource[..at],
        _ => resource,
    };
    Some((parts[2], resource_type))
}

/// Classify one TARGET value, exactly as the script's tail does.
fn classify_target(target: &str, params: &Map<String, Value>) -> Kind {
    if target.starts_with("arn:") {
        if let Some((service, resource_type)) = arn_resource_type(target) {
            if target.contains("/AWSServiceRoleFor") || target.contains("/aws-service-role/") {
                return Kind::Service;
            }
            if table_has(params, "userResourceTypes", resource_type) {
                return Kind::User;
            }
            if table_has(params, "hostResourceTypes", resource_type) {
                return Kind::Host;
            }
            if table_has(params, "serviceResourceTypes", resource_type) {
                return Kind::Service;
            }
            if service == "s3" {
                return Kind::Service;
            }
        }
        return Kind::Generic;
    }

    if target.len() == 20 && target.starts_with("AKIA") {
        return Kind::User;
    }
    for prefix in prefixes(params, "hostIdPrefixes") {
        if target.starts_with(prefix) {
            return Kind::Host;
        }
    }
    for prefix in prefixes(params, "serviceIdPrefixes") {
        if target.starts_with(prefix) {
            return Kind::Service;
        }
    }
    Kind::Generic
}

/// Classify the ACTOR, exactly as the script's tail does. `None` means the
/// generic `entity.id` fallback.
fn classify_actor(actor: &str, user_type: Option<&str>, params: &Map<String, Value>) -> Kind {
    if user_type == Some("AWSService") {
        return Kind::Service;
    }
    if actor.contains("/AWSServiceRoleFor") || actor.contains("/aws-service-role/") {
        return Kind::Service;
    }
    if let Some(user_type) = user_type
        && table_has(params, "userIdentityTypes", user_type)
    {
        return Kind::User;
    }
    if actor.starts_with("arn:")
        && let Some((_, resource_type)) = arn_resource_type(actor)
    {
        if matches!(
            resource_type,
            "user" | "role" | "assumed-role" | "federated-user"
        ) {
            return Kind::User;
        }
        if resource_type == "instance" {
            return Kind::Host;
        }
    }
    if actor.starts_with("i-") {
        return Kind::Host;
    }
    Kind::Generic
}

/// A set as the JSON array Elasticsearch ships it in.
///
/// The script collects into `TreeSet`s, but nothing sorted reaches the
/// document: the ingest document's copy constructor rebuilds every `Set` as a
/// plain `HashSet`, so the members come out in hash-table order -- bucket
/// ascending, and within a bucket the order they went in, which is the sorted
/// one. `target.entity.id` proves it, shipping `sgr-...` ahead of `sg-...`.
fn as_array(values: impl IntoIterator<Item = String>) -> Value {
    let mut values: Vec<String> = values.into_iter().collect();
    let table = copied_set_table(values.len());
    values.sort_by_cached_key(|value| crate::helpers::java_bucket(value, table));
    Value::Array(values.into_iter().map(Value::String).collect())
}

/// The table a `HashSet` built from a set of `entries` ends up holding.
///
/// `new HashSet<>(size)` starts at the power of two at or above that size
/// rather than the default 16, so the table is smaller than
/// [`crate::helpers::java_table_size`] answers and the buckets differ. It still
/// doubles once the count crosses three quarters of it, which a two-member set
/// does immediately.
fn copied_set_table(entries: usize) -> usize {
    let mut capacity = 1usize;
    while capacity < entries {
        capacity *= 2;
    }
    while entries > capacity * 3 / 4 {
        capacity *= 2;
    }
    capacity
}

/// Run a parsed entity script against one event.
pub(crate) fn run_entity_script(
    event: &mut Event,
    script: &EntityScript,
    params: &Map<String, Value>,
) -> bool {
    let mut out = Collected::default();
    let mut scope: Scope<'_> = Vec::new();

    eval_actions(event, &script.preamble, &mut scope, &mut out);

    let source = event.get_str("json.eventSource").map(str::to_string);
    for enrich in &script.enrich {
        if source.as_deref() == Some(enrich.source.as_str()) {
            eval_actions(event, &enrich.body, &mut scope, &mut out);
        }
    }

    let Collected {
        mut related,
        target,
        actor,
    } = out;

    if !target.is_empty() {
        let _ = event.set("target.entity.id", as_array(target.iter().cloned()));

        let mut kinds: [BTreeSet<String>; 4] = Default::default();
        for value in &target {
            let kind = classify_target(value, params) as usize;
            kinds[kind].insert(value.clone());
        }
        let [users, hosts, services, generic] = kinds;
        if !users.is_empty() {
            let _ = event.set("user.target.entity.id", as_array(users));
        }
        if !hosts.is_empty() {
            let _ = event.set("host.target.entity.id", as_array(hosts));
        }
        if !services.is_empty() {
            let _ = event.set("service.target.entity.id", as_array(services));
        }
        if !generic.is_empty() {
            let _ = event.set("entity.target.id", as_array(generic));
        }

        related.extend(target);
    }

    if let Some(actor) = actor {
        let _ = event.set("actor.entity.id", as_array([actor.clone()]));
        let user_type = event.get_str("json.userIdentity.type").map(str::to_string);
        let field = match classify_actor(&actor, user_type.as_deref(), params) {
            Kind::User => "user.entity.id",
            Kind::Host => "host.entity.id",
            Kind::Service => "service.entity.id",
            Kind::Generic => "entity.id",
        };
        let _ = event.set(field, as_array([actor]));
    }

    let _ = event.set("related.entity", as_array(related));
    true
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use serde_json::json;

    use super::*;

    /// A representative slice of the real script: helpers, two enrich
    /// functions covering the ladder, forEach, nested forEach, addValues,
    /// locals, `String.valueOf` and `SendCommand`, then the main body and the
    /// classifier tail's markers.
    const SCRIPT: &str = r#"
        void addFields(Set entities, List fields) {
          for (String field : fields) {
            addField(entities, field);
          }
        }
        void addField(Set entities, String fieldName) {
          addValue(entities, $(fieldName, null));
        }
        boolean addValue(Set entities, String value) {
          if (value == null || value == "") {
            return false;
          }
          return entities.add(value);
        }

        void enrichEc2(def enrichCtx, def eventSource, def eventName) {
          if (eventSource != "ec2.amazonaws.com") {
              return;
          }
          addFields(enrichCtx.related, [
            "json.requestParameters.groupId",
            "json.requestParameters.vpcId"
          ]);
          $("json.responseElements.instancesSet.items", []).stream().forEach(instances -> {
            addValues(enrichCtx.related, [
              instances.instanceId,
              instances.iamInstanceProfile?.arn
            ]);
            instances.networkInterfaceSet?.items?.stream().forEach(networks -> {
              addValue(enrichCtx.related, networks.networkInterfaceId);
            });
          });
          if (eventName == "DeleteNetworkAclEntry") {
            addField(enrichCtx.target, "json.requestParameters.networkAclId");
            def ruleNumber = $("json.requestParameters.ruleNumber", null);
            if (ruleNumber != null) {
              addValue(enrichCtx.target, String.valueOf(ruleNumber));
            }
          } else if (eventName == "GetPasswordData"
                    || eventName == "ModifyImageAttribute") {
            addField(enrichCtx.target, "json.requestParameters.instanceId");
          }
        }

        void enrichSsm(def enrichCtx, def eventSource, def eventName) {
          if (eventSource != "ssm.amazonaws.com") {
            return;
          }
          if (eventName == "SendCommand") {
            List instanceIds = $("json.requestParameters.instanceIds", []);
            if (instanceIds.isEmpty()) {
              instanceIds = $("json.requestParameters.targets", []).stream().flatMap(target -> target.values.stream()).collect(Collectors.toList());
            }
            if (instanceIds.size() == 1 && instanceIds.get(0) == "*") {
              instanceIds = [ $("json.recipientAccountId", null) ];
            }
            addValues(enrichCtx.target, instanceIds);
          }
        }

        Map enrichCtx = [:];
        enrichCtx.related = new TreeSet();
        enrichCtx.target = new TreeSet();

        enrichCtx.actor = $("json.userIdentity.arn", null);
        if (enrichCtx.actor == null) {
          enrichCtx.actor = $("json.userIdentity.onBehalfOf.userId", null);
        }

        addFields(enrichCtx.related, [
          "json.userIdentity.accessKeyId",
          "json.userIdentity.arn",
          "json.userIdentity.userName"
        ]);

        if ($("json.userIdentity.type", null) == "IdentityCenterUser") {
          addField(enrichCtx.related, "json.userIdentity.onBehalfOf.identityStoreArn");
        }

        $("json.resources", []).stream().forEach(f -> addValue(enrichCtx.related, f.ARN));

        String eventSource = $("json.eventSource", null);
        String eventName = $("json.eventName", null);

        enrichEc2(enrichCtx, eventSource, eventName);
        enrichSsm(enrichCtx, eventSource, eventName);

        if (!enrichCtx.target.isEmpty()) {
          field("target.entity.id").set(enrichCtx.target);
          for (def targetValue : enrichCtx.target) {
            if (target.contains("/AWSServiceRoleFor") || target.contains("/aws-service-role/")) {
              serviceTargets.add(target);
            } else if (params.userResourceTypes.containsKey(resourceType)) {
              userTargets.add(target);
            } else if (params.hostResourceTypes.containsKey(resourceType)) {
              hostTargets.add(target);
            } else if (params.serviceResourceTypes.containsKey(resourceType)) {
              serviceTargets.add(target);
            }
            if (target.length() == 20 && target.startsWith("AKIA")) {
              userTargets.add(target);
            }
            for (def prefix : params.hostIdPrefixes) { }
            for (def prefix : params.serviceIdPrefixes) { }
          }
          if (!userTargets.isEmpty()) {
            field("user.target.entity.id").set(userTargets);
          }
          enrichCtx.related.addAll(enrichCtx.target);
        }
        if (enrichCtx.actor != null) {
          field("actor.entity.id").set([ enrichCtx.actor ]);
          if (params.userIdentityTypes.containsKey(userType)) {
            field("user.entity.id").set([ enrichCtx.actor ]);
          }
        }
        field("related.entity").set(enrichCtx.related);
    "#;

    fn params() -> Map<String, Value> {
        json!({
            "userResourceTypes": { "user": true, "role": true, "assumed-role": true },
            "hostResourceTypes": { "instance": true },
            "serviceResourceTypes": { "network-acl": true },
            "hostIdPrefixes": ["i-"],
            "serviceIdPrefixes": ["sg-", "acl-"],
            "userIdentityTypes": { "IAMUser": true, "Root": true, "AssumedRole": true },
        })
        .as_object()
        .cloned()
        .unwrap()
    }

    fn parsed() -> EntityScript {
        EntityScript::parse(SCRIPT).expect("the representative script parses")
    }

    #[test]
    fn the_representative_script_parses() {
        let script = parsed();
        assert_eq!(script.enrich.len(), 2);
        assert_eq!(script.enrich[0].source, "ec2.amazonaws.com");
        assert_eq!(script.enrich[1].source, "ssm.amazonaws.com");
    }

    #[test]
    fn related_gathers_fields_members_and_nested_members() {
        let mut event = Event::new(json!({
            "json": {
                "eventSource": "ec2.amazonaws.com",
                "eventName": "RunInstances",
                // An all-zero key id shaped like AWS's, not a credential.
                // nosemgrep: generic.secrets.security.detected-aws-access-key-id-value.detected-aws-access-key-id-value
                "userIdentity": { "arn": "arn:aws:iam::1:user/alice", "accessKeyId": "AKIA0000000000000000" },
                "requestParameters": { "groupId": "sg-1", "vpcId": "vpc-9" },
                "responseElements": { "instancesSet": { "items": [
                    { "instanceId": "i-abc",
                      "iamInstanceProfile": { "arn": "arn:aws:iam::1:instance-profile/p" },
                      "networkInterfaceSet": { "items": [ { "networkInterfaceId": "eni-7" } ] } }
                ] } },
            }
        }));
        assert!(run_entity_script(&mut event, &parsed(), &params()));

        let related = event.get("related.entity").unwrap();
        for expected in [
            "sg-1",
            "vpc-9",
            "i-abc",
            "arn:aws:iam::1:instance-profile/p",
            "eni-7",
            "arn:aws:iam::1:user/alice",
            // nosemgrep: generic.secrets.security.detected-aws-access-key-id-value.detected-aws-access-key-id-value
            "AKIA0000000000000000",
        ] {
            assert!(
                related.as_array().unwrap().iter().any(|v| v == expected),
                "missing {expected} in {related}"
            );
        }
    }

    #[test]
    fn the_event_ladder_takes_only_its_arm() {
        let mut event = Event::new(json!({
            "json": {
                "eventSource": "ec2.amazonaws.com",
                "eventName": "DeleteNetworkAclEntry",
                "requestParameters": { "networkAclId": "acl-3", "ruleNumber": 100,
                                        "instanceId": "i-should-not-appear" },
            }
        }));
        assert!(run_entity_script(&mut event, &parsed(), &params()));

        assert_eq!(
            event.get("target.entity.id"),
            Some(&json!(["100", "acl-3"])),
            "the rule number is stringified, and both members share a bucket"
        );
        // acl-3 classifies as service by prefix; "100" is generic.
        assert_eq!(
            event.get("service.target.entity.id"),
            Some(&json!(["acl-3"]))
        );
        assert_eq!(event.get("entity.target.id"), Some(&json!(["100"])));
    }

    #[test]
    fn send_command_falls_back_through_targets_and_wildcard() {
        let mut event = Event::new(json!({
            "json": {
                "eventSource": "ssm.amazonaws.com",
                "eventName": "SendCommand",
                "recipientAccountId": "123456789012",
                "requestParameters": { "targets": [ { "values": ["*"] } ] },
            }
        }));
        assert!(run_entity_script(&mut event, &parsed(), &params()));
        assert_eq!(
            event.get("target.entity.id"),
            Some(&json!(["123456789012"]))
        );
    }

    #[test]
    fn the_actor_classifies_by_identity_type_then_arn() {
        let cases = [
            // IAMUser type wins via the table.
            (
                json!({"type": "IAMUser", "arn": "arn:aws:iam::1:user/alice"}),
                "user.entity.id",
            ),
            // AWSService goes to service even with a user-shaped arn.
            (
                json!({"type": "AWSService", "arn": "arn:aws:iam::1:user/alice"}),
                "service.entity.id",
            ),
            // A service-linked role is a service whatever the type says.
            (
                json!({"type": "Unknown", "arn": "arn:aws:iam::1:role/aws-service-role/x/AWSServiceRoleForX"}),
                "service.entity.id",
            ),
            // An unknown type falls to the arn's resource type.
            (
                json!({"type": "Unknown", "arn": "arn:aws:sts::1:assumed-role/r/s"}),
                "user.entity.id",
            ),
        ];
        for (identity, field) in cases {
            let mut event = Event::new(json!({
                "json": { "eventSource": "none", "userIdentity": identity }
            }));
            assert!(run_entity_script(&mut event, &parsed(), &params()));
            let actor = event.get("actor.entity.id").cloned();
            assert!(actor.is_some(), "actor always written when present");
            assert_eq!(
                event.get(field),
                actor.as_ref(),
                "{identity} classifies to {field}"
            );
        }
    }

    #[test]
    fn the_fallback_actor_is_on_behalf_of() {
        let mut event = Event::new(json!({
            "json": { "userIdentity": { "onBehalfOf": { "userId": "u-1" } } }
        }));
        assert!(run_entity_script(&mut event, &parsed(), &params()));
        assert_eq!(event.get("actor.entity.id"), Some(&json!(["u-1"])));
        // u-1 is no arn, no AKIA, no known prefix: the generic entity.id.
        assert_eq!(event.get("entity.id"), Some(&json!(["u-1"])));
    }

    #[test]
    fn an_unreadable_statement_fails_the_whole_parse() {
        let broken = SCRIPT.replace(
            "addField(enrichCtx.target, \"json.requestParameters.networkAclId\");",
            "frobnicate(enrichCtx.target);",
        );
        assert!(EntityScript::parse(&broken).is_none());
    }

    #[test]
    fn a_missing_classifier_marker_fails_the_parse() {
        let restructured = SCRIPT.replace("params.userIdentityTypes.containsKey", "something.else");
        assert!(EntityScript::parse(&restructured).is_none());
    }
}
