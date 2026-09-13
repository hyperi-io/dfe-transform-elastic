// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One label chosen from a table of substrings, written as a single-element
//! list.
//!
//! `trellix_edr_cloud` grades its `_eventType` twice this way -- once into
//! `event.category` and once into `event.type` -- and spells the table as a
//! counted loop over parallel `ArrayList` literals:
//!
//! ```painless
//! ArrayList items = new ArrayList(['access', 'creat', 'delet']);
//! ArrayList set_items = new ArrayList(['access', 'creation', 'deletion']);
//! for (def j = 0; j < items.length; j++) {
//!     if (event_type.contains(items[j])) { ctx.event.type = [set_items[j]]; }
//! }
//! ```
//!
//! The loop is UNROLLED at parse time rather than reasoned about: each pass
//! keeps the rungs in the order the chain tests them, so the first match within
//! a pass fires and the last pass to fire wins -- which is what the vendor's
//! `else if` chain and its continuing loop do between them. Flattening the two
//! into one ordered table instead needs the rungs reversed, and gets the answer
//! wrong the moment a pass carries a rung the index does not select.

use std::collections::HashMap;

use serde_json::{Value, json};

use dfe_core::event::Event;

use crate::common::{ctx_path_bound_to, strip_case_fold, strip_line_comments};
use crate::params::clean_path;

/// The table one script grades a field against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeedleTable {
    subject: String,
    fold_case: bool,
    target: String,
    /// Written before the table is walked, unless the field it names already
    /// carries the literal beside it.
    fallback: Option<Fallback>,
    /// One entry per loop pass, each holding the chain's rungs in test order.
    passes: Vec<Vec<Rung>>,
}

/// The pre-loop write and the list membership that suppresses it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Fallback {
    label: String,
    unless: Option<(String, String)>,
}

/// One rung: the substring that selects it and the label it writes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rung {
    needle: String,
    /// A substring that vetoes the rung, which is how trellix keeps
    /// `process created` out of its `creat` arm.
    unless: Option<String>,
    label: String,
}

impl NeedleTable {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        subject: impl Into<String>,
        fold_case: bool,
        target: impl Into<String>,
        passes: Vec<Vec<(String, Option<String>, String)>>,
    ) -> Self {
        Self {
            subject: subject.into(),
            fold_case,
            target: target.into(),
            fallback: None,
            passes: passes
                .into_iter()
                .map(|pass| {
                    pass.into_iter()
                        .map(|(needle, unless, label)| Rung {
                            needle,
                            unless,
                            label,
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

/// Grade the subject against the table, writing the chosen label as a list.
pub fn needle_table(event: &mut Event, pattern: &NeedleTable) -> bool {
    let Some(subject) = event.get_as_string(&pattern.subject) else {
        return true;
    };
    let subject = if pattern.fold_case {
        subject.to_lowercase()
    } else {
        subject
    };

    let mut chosen: Option<&str> = None;
    if let Some(fallback) = &pattern.fallback {
        let suppressed = fallback.unless.as_ref().is_some_and(|(path, literal)| {
            event.get_array(path).is_some_and(|items| {
                items
                    .iter()
                    .any(|item| item.as_str() == Some(literal.as_str()))
            })
        });
        if !suppressed {
            chosen = Some(&fallback.label);
        }
    }
    for pass in &pattern.passes {
        let fired = pass.iter().find(|rung| {
            subject.contains(&rung.needle)
                && !rung
                    .unless
                    .as_ref()
                    .is_some_and(|veto| subject.contains(veto.as_str()))
        });
        if let Some(rung) = fired {
            chosen = Some(&rung.label);
        }
    }
    if let Some(label) = chosen {
        let _ = event.set(&pattern.target, json!([label]));
    }
    true
}

/// Read the table, or `None` where the script is not one counted loop over
/// literal lists writing one field.
pub fn parse_needle_table(script: &str) -> Option<NeedleTable> {
    let script = strip_line_comments(script);
    // One loop, and it is the tail of the script: a second loop, or work after
    // this one, is a script this reader has not understood whole.
    if script.matches("for (").count() != 1 {
        return None;
    }
    let (head, rest) = script.split_once("for (def ")?;
    let (header, body) = rest.split_once(')')?;
    let body = body.split_once('{')?.1;

    let lists = literal_lists(&script);
    let (index, passes_over) = counted_loop(header)?;
    let count = lists.get(passes_over.as_str())?.len();

    let (subject_local, subject, fold_case) = subject_binding(head)?;
    let elements = element_bindings(body, &lists);

    let mut target: Option<String> = None;
    let mut chain: Vec<(Operand, Option<String>, Operand)> = Vec::new();
    for segment in body.split("if (").skip(1) {
        let (condition, arm) = segment.split_once('{')?;
        let (needle, unless) = arm_condition(condition, &subject_local)?;
        let (writes, label) = arm_write(arm)?;
        if *target.get_or_insert(writes.clone()) != writes {
            return None;
        }
        chain.push((needle, unless, label));
    }
    let target = target?;
    if chain.is_empty() {
        return None;
    }

    let mut passes = Vec::with_capacity(count);
    for pass in 0..count {
        let mut rungs = Vec::with_capacity(chain.len());
        for (needle, unless, label) in &chain {
            rungs.push(Rung {
                needle: resolve(needle, pass, &lists, &elements, index)?,
                unless: unless.clone(),
                label: resolve(label, pass, &lists, &elements, index)?,
            });
        }
        passes.push(rungs);
    }

    // An `if` ahead of the loop is the fallback write; one this cannot read
    // whole declines, because running its write unconditionally sets a label the
    // vendor withholds.
    let fallback = if head.contains("if (") {
        Some(pre_loop_fallback(head, &target)?)
    } else {
        None
    };
    // Every field the script writes has to be the one field this reads, or the
    // rest goes unrun under a matcher that claimed the script.
    if writes_elsewhere(&script, &target) {
        return None;
    }

    Some(NeedleTable {
        subject,
        fold_case,
        target,
        fallback,
        passes,
    })
}

/// A list element, a fixed element, or a literal, as the script names it.
#[derive(Debug, Clone)]
enum Operand {
    Literal(String),
    /// `items[j]` -- the element this pass selects.
    Indexed(String, String),
    /// `items[4]` -- one fixed element.
    Fixed(String, usize),
    /// A local bound to `items[j]` earlier in the loop body.
    Element(String),
}

/// Resolve an operand for one pass of the loop.
fn resolve(
    operand: &Operand,
    pass: usize,
    lists: &HashMap<String, Vec<String>>,
    elements: &HashMap<String, String>,
    index: &str,
) -> Option<String> {
    match operand {
        Operand::Literal(literal) => Some(literal.clone()),
        Operand::Fixed(list, at) => lists.get(list)?.get(*at).cloned(),
        Operand::Indexed(list, variable) if variable == index => {
            lists.get(list)?.get(pass).cloned()
        }
        Operand::Indexed(..) => None,
        Operand::Element(local) => lists.get(elements.get(local)?)?.get(pass).cloned(),
    }
}

/// `<var> = 0; <var> < <list>.length; <var>++` -- the index and the list it counts.
fn counted_loop(header: &str) -> Option<(&str, String)> {
    let mut clauses = header.split(';');
    let (index, start) = clauses.next()?.split_once('=')?;
    let index = index.trim().rsplit(' ').next()?.trim();
    if start.trim() != "0" || index.is_empty() {
        return None;
    }
    let bound = clauses.next()?.split_once('<')?.1.trim();
    let list = bound.strip_suffix(".length").or_else(|| {
        bound
            .strip_suffix(".size()")
            .or_else(|| bound.strip_suffix(".length()"))
    })?;
    Some((index, list.trim().to_owned()))
}

/// Every `ArrayList <name> = new ArrayList([...])` or `def <name> = [...]`.
fn literal_lists(script: &str) -> HashMap<String, Vec<String>> {
    let mut lists = HashMap::new();
    for statement in script.split([';', '\n']) {
        let Some((declaration, value)) = statement.split_once('=') else {
            continue;
        };
        let Some(name) = declaration.trim().rsplit(' ').next().map(str::trim) else {
            continue;
        };
        if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let value = value.trim();
        let value = value
            .strip_prefix("new ArrayList(")
            .and_then(|rest| rest.trim_end().strip_suffix(')'))
            .unwrap_or(value);
        let Some(Value::Array(members)) = crate::params::literal_value(value.trim()) else {
            continue;
        };
        let names: Vec<String> = members
            .iter()
            .filter_map(|member| member.as_str().map(str::to_owned))
            .collect();
        if names.len() == members.len() && !names.is_empty() {
            lists.insert(name.to_owned(), names);
        }
    }
    lists
}

/// The local the script grades, and the field it was read from.
fn subject_binding(head: &str) -> Option<(String, String, bool)> {
    for statement in head.split([';', '\n']) {
        let statement = statement.trim();
        let Some((declaration, value)) = statement.split_once('=') else {
            continue;
        };
        if !value.trim().starts_with("ctx.") {
            continue;
        }
        let name = declaration.trim().rsplit(' ').next()?.trim();
        if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let path = ctx_path_bound_to(head, name)?;
        let (path, fold) = strip_case_fold(&path);
        return Some((name.to_owned(), clean_path(&path), fold));
    }
    None
}

/// Locals bound to the pass's element -- `String key = items[j];`.
fn element_bindings(body: &str, lists: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
    let mut elements = HashMap::new();
    for statement in body.split([';', '\n']) {
        let Some((declaration, value)) = statement.trim().split_once('=') else {
            continue;
        };
        let Some(name) = declaration.trim().rsplit(' ').next() else {
            continue;
        };
        let Some((list, _)) = value.trim().split_once('[') else {
            continue;
        };
        if lists.contains_key(list.trim()) {
            elements.insert(name.trim().to_owned(), list.trim().to_owned());
        }
    }
    elements
}

/// The needle an arm tests and the substring that vetoes it.
fn arm_condition(condition: &str, subject: &str) -> Option<(Operand, Option<String>)> {
    let mut needle = None;
    let mut unless = None;
    for clause in condition.split("&&") {
        let at = clause.find(".contains(")?;
        let (read, argument) = clause.split_at(at);
        let read = read.trim_start_matches(['(', '!', ' ']).trim();
        if read != subject {
            return None;
        }
        let argument = argument[".contains(".len()..].split_once(')')?.0;
        let operand = operand(argument)?;
        if clause.trim_start().starts_with(['!', '(']) && clause.contains('!') {
            // A veto only ever names a literal: a list element would make the
            // exclusion depend on the pass, which the vendor does not write.
            let Operand::Literal(literal) = operand else {
                return None;
            };
            unless = Some(literal);
        } else if needle.replace(operand).is_some() {
            return None;
        }
    }
    Some((needle?, unless))
}

/// The field an arm writes and the operand it writes as a one-element list.
fn arm_write(arm: &str) -> Option<(String, Operand)> {
    let statement = arm.split("} else").next().unwrap_or(arm);
    let statement = statement.split([';', '\n']).find(|s| s.contains("ctx."))?;
    let (target, value) = statement.split_once('=')?;
    let target = clean_path(target.trim().strip_prefix("ctx.")?);
    let value = value.trim();
    let inner = value.strip_prefix('[')?.strip_suffix(']')?;
    Some((target, operand(inner)?))
}

/// `if (!ctx.<field>.contains('<literal>')) { ctx.<target> = ['<label>']; }`
/// ahead of the loop, which trellix uses to default `event.type` to `info`.
///
/// `None` where this cannot read the `if` whole -- a guard left unread runs the
/// write unconditionally. The caller decides what a `head` with no `if` at all
/// means, because that is a table with no fallback rather than a failed read.
fn pre_loop_fallback(head: &str, target: &str) -> Option<Fallback> {
    let mut segments = head.split("if (").skip(1);
    let segment = segments.next()?;
    if segments.next().is_some() {
        return None;
    }
    let (condition, arm) = segment.split_once('{')?;
    let (writes, label) = arm_write(arm)?;
    let Operand::Literal(label) = label else {
        return None;
    };
    if writes != target {
        return None;
    }

    let rest = condition.trim().strip_prefix('!')?;
    let at = rest.find(".contains(")?;
    let path = clean_path(rest[..at].trim().strip_prefix("ctx.")?);
    let literal = rest[at + ".contains(".len()..].split_once(')')?.0;
    let Operand::Literal(literal) = operand(literal)? else {
        return None;
    };
    Some(Fallback {
        label,
        unless: Some((path, literal)),
    })
}

/// Whether the script assigns to any `ctx.` field other than the one read.
fn writes_elsewhere(script: &str, target: &str) -> bool {
    script.split([';', '\n']).any(|statement| {
        statement
            .trim()
            .strip_prefix("ctx.")
            .and_then(|rest| rest.split_once('='))
            .is_some_and(|(path, _)| clean_path(path.trim()) != target)
    })
}

/// One operand as the script wrote it.
fn operand(text: &str) -> Option<Operand> {
    let text = text.trim();
    if let Some(quote) = text.chars().next().filter(|c| *c == '\'' || *c == '"') {
        let inner = &text[quote.len_utf8()..];
        return Some(Operand::Literal(inner.split_once(quote)?.0.to_owned()));
    }
    if let Some((list, index)) = text.split_once('[') {
        let index = index.trim_end().strip_suffix(']')?.trim();
        let list = list.trim().to_owned();
        return Some(match index.parse::<usize>() {
            Ok(at) => Operand::Fixed(list, at),
            Err(_) => Operand::Indexed(list, index.to_owned()),
        });
    }
    (!text.is_empty() && text.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| Operand::Element(text.to_owned()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// `trellix_edr_cloud/event`, both scripts verbatim.
    const CATEGORY: &str = "def event_category = ctx.json._eventType.toLowerCase(); ArrayList items = new ArrayList(['process', 'file', 'api', 'network', 'registry','authentication']); for (def j = 0; j < items.length; j++) {\n    String key = items[j];\n    if (event_category.contains('user')){\n        ctx.event.category = [items[5]];\n    } else if (event_category.contains('reg')){\n        ctx.event.category = [items[4]];\n    } else if (event_category.contains('dns')){\n        ctx.event.category = [items[3]];\n    } else if (event_category.contains(key)){\n        ctx.event.category = [key];\n    }\n}";

    const TYPE: &str = "def event_type = ctx.json._eventType.toLowerCase(); if (!ctx.event.category.contains('registry')){\n\n\n  ctx.event.type = ['info']\n} ArrayList items = new ArrayList(['access', 'creat', 'delet', 'modif', 'change']); ArrayList set_items = new ArrayList(['access', 'creation', 'deletion', 'change', 'change']); for (def j = 0; j < items.length; j++) {\n\n\n    if (event_type.contains(items[j]) && (!event_type.contains('process created'))){\n        ctx.event.type = [set_items[j]];\n    }\n}";

    fn graded(script: &str, event_type: &str, category: &Value) -> Event {
        let mut event = Event::new(json!({
            "json": { "_eventType": event_type },
            "event": { "category": category },
        }));
        let pattern = parse_needle_table(script).expect("declined");
        needle_table(&mut event, &pattern);
        event
    }

    #[test]
    fn the_list_element_the_subject_names_wins() {
        let event = graded(CATEGORY, "File Deleted", &Value::Null);
        assert_eq!(event.get("event.category"), Some(&json!(["file"])));
    }

    /// The three fixed rungs sit ahead of the indexed one in every pass, so they
    /// win over a later list element that also matches.
    #[test]
    fn a_fixed_rung_beats_the_indexed_one() {
        let event = graded(CATEGORY, "User Process Logon", &Value::Null);
        assert_eq!(
            event.get("event.category"),
            Some(&json!(["authentication"]))
        );
        let event = graded(CATEGORY, "Registry Value Set", &Value::Null);
        assert_eq!(event.get("event.category"), Some(&json!(["registry"])));
    }

    #[test]
    fn the_parallel_list_supplies_the_label() {
        let event = graded(TYPE, "File Deleted", &json!(["file"]));
        assert_eq!(event.get("event.type"), Some(&json!(["deletion"])));
    }

    /// The pre-loop default stands where no rung fires, and is suppressed by the
    /// membership its guard names.
    #[test]
    fn the_pre_loop_default_is_guarded() {
        let event = graded(TYPE, "File Opened", &json!(["file"]));
        assert_eq!(event.get("event.type"), Some(&json!(["info"])));
        let event = graded(TYPE, "Something Else", &json!(["registry"]));
        assert_eq!(event.get("event.type"), None);
    }

    #[test]
    fn the_veto_keeps_process_created_out_of_the_creation_arm() {
        let event = graded(TYPE, "Process Created", &json!(["process"]));
        assert_eq!(event.get("event.type"), Some(&json!(["info"])));
    }

    /// A script writing a second field is not this pattern, whatever the loop
    /// looks like.
    #[test]
    fn a_second_write_declines() {
        let script = "def v = ctx.a.b.toLowerCase(); ArrayList items = new ArrayList(['x', 'y']); ctx.event.kind = 'event'; for (def j = 0; j < items.length; j++) {\n    if (v.contains(items[j])){\n        ctx.event.type = [items[j]];\n    }\n}";
        assert!(parse_needle_table(script).is_none());
    }
}
