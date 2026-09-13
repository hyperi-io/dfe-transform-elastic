// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A member renamed in every record of a list, gated on the VALUE'S RUNTIME
//! TYPE.
//!
//! The vendor ships one key carrying two different things, and Elasticsearch's
//! mapping cannot hold both -- so the pipeline moves only the spelling the
//! mapping rejects and leaves the other under its original name. The type test
//! is the whole pattern rather than a detail on it:
//! `ti_recordedfuture_playbook_alert` keeps `old`/`new` on an `assignee_change`
//! because they are MAPS, keeps `added`/`removed` on an `entities_change`
//! because they are lists of MAPS, and moves only the String-typed ones onto
//! the `_str` names.
//!
//! ```painless
//! for (change in panel.changes) {
//!     if (change.containsKey("old") && change.old instanceof String) {
//!         change.old_str = change.old;
//!         change.remove("old");
//!     }
//!     if (change.containsKey("added") && change.added instanceof List
//!         && change.added.size() > 0 && change.added[0] instanceof String) {
//!         change.added_str = change.added;
//!         change.remove("added");
//!     }
//! }
//! ```
//!
//! `trend_micro_vision_one_alert` writes the same idea the other way up --
//! `indicator.put('value_object', indicator.remove('value'))` under an
//! `instanceof Map` -- so both spellings are read here. A rename that ignored
//! the guards would rewrite members Elasticsearch leaves alone, in the same
//! event.
//!
//! The rename MOVES the key to the end, because Painless removes it and assigns
//! a new one. That is [`crate::records::RecordRenames`]'s ordering and the
//! runner below keeps it: `shift_remove` then a plain insert, never
//! `Map::remove`, which under `preserve_order` would drop the LAST key into the
//! freed slot.

use serde_json::Value;

use crate::params::{balanced, clean_path};
use dfe_core::Event;

/// The runtime type a guard admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberType {
    /// `instanceof String`.
    Text,
    /// `instanceof Map`.
    Map,
    /// `instanceof List`, non-empty, first element a String -- the list-typed
    /// guard spelled out, because a list of MAPS is the case it must decline.
    TextList,
}

impl MemberType {
    /// Does this value answer the guard?
    fn admits(self, value: &Value) -> bool {
        match self {
            Self::Text => value.is_string(),
            Self::Map => value.is_object(),
            Self::TextList => value
                .as_array()
                .and_then(|items| items.first())
                .is_some_and(Value::is_string),
        }
    }
}

/// One member moved to a new name, and the type that decides whether it moves.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TypedRename {
    from: String,
    to: String,
    admits: MemberType,
}

/// Type-gated member renames over every record of a list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedMemberRename {
    /// The `ctx.` path of the list walked.
    list: String,
    /// The member of each of that list's records holding the records actually
    /// renamed, where the walk is two deep.
    nested: Option<String>,
    /// In the order the script renames them, which is the order the moved keys
    /// end up in.
    renames: Vec<TypedRename>,
}

impl TypedMemberRename {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        list: impl Into<String>,
        nested: Option<String>,
        renames: Vec<(String, String, MemberType)>,
    ) -> Self {
        Self {
            list: list.into(),
            nested,
            renames: renames
                .into_iter()
                .map(|(from, to, admits)| TypedRename { from, to, admits })
                .collect(),
        }
    }
}

/// The first for-each in `text`: its variable, what it walks, and what follows
/// the header.
///
/// Painless spells the loop `for (v in x)` and `for (def v : x)`, and
/// `trend_micro` declares a counted `for (int i = 0; ...)` in the same file, so
/// the scan skips a header carrying neither separator rather than stopping at
/// the first `for (`.
fn first_foreach(text: &str) -> Option<(&str, &str, &str)> {
    let mut from = 0;
    while let Some(hit) = text[from..].find("for (") {
        let open = from + hit + "for ".len();
        from = open + 1;
        let Some((header, after)) = balanced(&text[open..], '(', ')') else {
            continue;
        };
        let Some((var, walked)) = header
            .split_once(" in ")
            .or_else(|| header.split_once(" : "))
        else {
            continue;
        };
        let var = var.trim().trim_start_matches("def ").trim();
        if var.is_empty() || !var.chars().all(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        return Some((var, walked.trim(), after));
    }
    None
}

/// The `ctx.` path an expression names, following one local binding.
///
/// `recordedfuture` walks a local: `def panel_log= ctx.<path>;` and then
/// `for (panel in panel_log)`. Reading only the inline `ctx.` spelling would
/// decline the whole script over where the vendor put the semicolon.
fn ctx_path(script: &str, expression: &str) -> Option<String> {
    let direct = |term: &str| {
        let path = clean_path(
            term.trim()
                .trim_start_matches("ctx")
                .trim_start_matches(['?', '.']),
        );
        (!path.is_empty() && !path.contains(['(', '[', ' ', ','])).then_some(path)
    };
    if expression.trim_start().starts_with("ctx") {
        return direct(expression);
    }
    let local = expression.trim();
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    // `def <local> = ctx.<path>;`, with the vendor's spacing either side of the
    // `=` taken as it comes.
    for statement in script.split([';', '\n']) {
        let Some((head, bound)) = statement.split_once('=') else {
            continue;
        };
        if head.trim().rsplit([' ', '\t']).next() != Some(local) || !head.contains("def ") {
            continue;
        }
        if bound.trim_start().starts_with("ctx") {
            return direct(bound);
        }
    }
    None
}

/// Every `if (<guard>) { <body> }` in `text`, both taken with balanced
/// delimiters so a `containsKey("old")` inside the guard does not close it.
fn if_blocks(text: &str) -> Vec<(&str, &str)> {
    let mut blocks = Vec::new();
    let mut from = 0;
    while let Some(hit) = text[from..].find("if (") {
        let open = from + hit + "if ".len();
        from = open + 1;
        let Some((guard, after)) = balanced(&text[open..], '(', ')') else {
            continue;
        };
        let Some((body, _)) = balanced(after.trim_start(), '{', '}') else {
            continue;
        };
        blocks.push((guard, body));
    }
    blocks
}

/// The quoted argument of `<call>` where it opens at `at` in `text`.
fn quoted_argument<'a>(text: &'a str, call: &str) -> Option<&'a str> {
    let (_, rest) = text.split_once(call)?;
    let rest = rest.trim_start();
    let quote = rest.chars().next().filter(|c| *c == '"' || *c == '\'')?;
    let (inner, _) = rest[quote.len_utf8()..].split_once(quote)?;
    (!inner.is_empty() && inner.chars().all(|c| c.is_alphanumeric() || c == '_')).then_some(inner)
}

/// The type the guard tests the member for, or `None` where it tests something
/// this cannot reproduce.
///
/// The `instanceof` has to be applied to the MEMBER the `containsKey` named, in
/// one of the two spellings the vendors use -- a type test over anything else
/// is a different guard and declines rather than being assumed away.
fn member_type(guard: &str, var: &str, member: &str) -> Option<MemberType> {
    let dotted = format!("{var}.{member}");
    let called = [
        format!("{var}.get('{member}')"),
        format!("{var}.get(\"{member}\")"),
    ];
    let tested = |kind: &str| {
        let suffix = format!(" instanceof {kind}");
        guard.contains(&format!("{dotted}{suffix}"))
            || called
                .iter()
                .any(|call| guard.contains(&format!("{call}{suffix}")))
    };
    if tested("String") {
        return Some(MemberType::Text);
    }
    if tested("Map") {
        return Some(MemberType::Map);
    }
    // A bare `instanceof List` says nothing about the elements, and the element
    // type is exactly what separates the lists that move from the ones that
    // stay. Only the spelled-out form is read.
    if tested("List")
        && guard.contains(&format!("{dotted}.size() > 0"))
        && guard.contains(&format!("{dotted}[0] instanceof String"))
    {
        return Some(MemberType::TextList);
    }
    None
}

/// The new name the block moves `member` to, or `None` where the block does
/// anything else.
///
/// Both spellings have to REMOVE the old key as well as write the new one: a
/// block that only copies leaves the vendor's key in place, which is a
/// different document.
fn renamed_to(body: &str, var: &str, member: &str) -> Option<String> {
    let removed = [
        format!("{var}.remove('{member}')"),
        format!("{var}.remove(\"{member}\")"),
    ];
    if !removed.iter().any(|call| body.contains(call)) {
        return None;
    }

    // `<var>.put('<to>', <var>.remove('<member>'));`
    if let Some(to) = quoted_argument(body, &format!("{var}.put(")) {
        return Some(to.to_owned());
    }

    // `<var>.<to> = <var>.<member>;`
    let assigned = format!("= {var}.{member}");
    let (head, tail) = body.split_once(&assigned)?;
    if !tail.trim_start().starts_with(';') {
        return None;
    }
    let to = head
        .trim_end()
        .rsplit(['\n', ';', '{', '}', ' '])
        .next()?
        .trim()
        .strip_prefix(&format!("{var}."))?;
    (!to.is_empty() && to.chars().all(|c| c.is_alphanumeric() || c == '_')).then(|| to.to_owned())
}

/// Read the walk, the renames and each one's type guard, or decline.
///
/// EVERY guarded block in the record body has to read as a typed rename. A
/// script that renames two members and also does something else is not this
/// one, and claiming it would run the renames and drop the rest in silence.
#[must_use]
pub fn parse_typed_member_rename(script: &str) -> Option<TypedMemberRename> {
    let (outer_var, walked, after) = first_foreach(script)?;
    let list = ctx_path(script, walked)?;
    let (outer_body, _) = balanced(after.trim_start(), '{', '}')?;

    // A second walk over a member of the first walk's records: the renames are
    // one level further down, and the outer guard is the member being a list.
    let (record_var, record_body, nested) = match first_foreach(outer_body) {
        Some((inner_var, inner_walked, inner_after)) => {
            let member = inner_walked.strip_prefix(&format!("{outer_var}."))?;
            if member.is_empty() || !member.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return None;
            }
            let (inner_body, _) = balanced(inner_after.trim_start(), '{', '}')?;
            (inner_var, inner_body, Some(member.to_owned()))
        }
        None => (outer_var, outer_body, None),
    };

    let blocks = if_blocks(record_body);
    if blocks.is_empty() {
        return None;
    }
    let mut renames = Vec::with_capacity(blocks.len());
    for (guard, body) in blocks {
        let from = quoted_argument(guard, &format!("{record_var}.containsKey("))?;
        let admits = member_type(guard, record_var, from)?;
        let to = renamed_to(body, record_var, from)?;
        if to == from {
            return None;
        }
        renames.push(TypedRename {
            from: from.to_owned(),
            to,
            admits,
        });
    }

    Some(TypedMemberRename {
        list,
        nested,
        renames,
    })
}

/// Move each guarded member to its new name, in every record the walk reaches.
///
/// A record that is not a map, a member the record does not carry, and a value
/// the guard declines are all left exactly as they stand -- which is what the
/// script's own `containsKey` and `instanceof` say.
#[must_use]
pub fn typed_member_rename(event: &mut Event, pattern: &TypedMemberRename) -> bool {
    let Some(Value::Array(mut records)) = event.get(&pattern.list).cloned() else {
        // Every one of these scripts walks a list the pipeline has already
        // built; an absent one leaves the document alone.
        return true;
    };

    for record in &mut records {
        match &pattern.nested {
            None => rename_members(record, &pattern.renames),
            Some(member) => {
                let Some(Value::Array(inner)) = record
                    .as_object_mut()
                    .and_then(|holder| holder.get_mut(member.as_str()))
                else {
                    continue;
                };
                for held in inner {
                    rename_members(held, &pattern.renames);
                }
            }
        }
    }

    let _ = event.set(&pattern.list, Value::Array(records));
    true
}

/// Apply the renames to one record, in the order the script writes them.
fn rename_members(record: &mut Value, renames: &[TypedRename]) {
    let Some(members) = record.as_object_mut() else {
        return;
    };
    for rename in renames {
        if !members
            .get(rename.from.as_str())
            .is_some_and(|held| rename.admits.admits(held))
        {
            continue;
        }
        // `shift_remove` and a plain insert: the moved key goes to the END,
        // which is where Painless's remove-then-assign leaves it. `Map::remove`
        // is `swap_remove` under `preserve_order` and would reorder the record.
        if let Some(value) = members.shift_remove(rename.from.as_str()) {
            members.insert(rename.to.clone(), value);
        }
    }
}

#[cfg(test)]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`.
#[allow(
    clippy::needless_raw_string_hashes,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic
)]
mod tests {
    use super::*;
    use crate::common::normalise;
    use serde_json::json;

    /// Verbatim from `ti_recordedfuture_playbook_alert/default.rs`, in the
    /// escaped one-line form the call site holds.
    const PANEL_LOG: &str = r#"def panel_log= ctx.recordedfuture.playbook_alert.panel_log_v2;\nfor (panel in panel_log) {\n    if (panel.containsKey(\"changes\") && panel.changes instanceof List) {\n        for (change in panel.changes) {\n            if (change.containsKey(\"old\") && change.old instanceof String) {\n                change.old_str = change.old;\n                change.remove(\"old\");\n            }\n            if (change.containsKey(\"new\") && change.new instanceof String) {\n                change.new_str = change.new;\n                change.remove(\"new\");\n            }\n            if (change.containsKey(\"added\") && change.added instanceof List && change.added.size() > 0 && change.added[0] instanceof String) {\n                change.added_str = change.added;\n                change.remove(\"added\");\n            }\n            if (change.containsKey(\"removed\") && change.removed instanceof List && change.removed.size() > 0 && change.removed[0] instanceof String) {\n                change.removed_str = change.removed;\n                change.remove(\"removed\");\n            }\n        }\n    }\n}"#;

    /// Verbatim from `trend_micro_vision_one_alert/default.rs`.
    const INDICATORS: &str = r#"for (def indicator : ctx.trend_micro_vision_one.alert.indicators) {\n  if (indicator.containsKey('value') && indicator.get('value') instanceof Map) {\n    indicator.put('value_object', indicator.remove('value'));\n  }\n}\n"#;

    #[test]
    fn a_nested_walk_reads_both_type_guards() {
        let pattern = parse_typed_member_rename(&normalise(PANEL_LOG)).unwrap();
        assert_eq!(
            pattern,
            TypedMemberRename::new(
                "recordedfuture.playbook_alert.panel_log_v2",
                Some("changes".to_owned()),
                vec![
                    ("old".into(), "old_str".into(), MemberType::Text),
                    ("new".into(), "new_str".into(), MemberType::Text),
                    ("added".into(), "added_str".into(), MemberType::TextList),
                    ("removed".into(), "removed_str".into(), MemberType::TextList),
                ],
            )
        );
    }

    #[test]
    fn the_put_spelling_reads_the_same_rename() {
        let pattern = parse_typed_member_rename(&normalise(INDICATORS)).unwrap();
        assert_eq!(
            pattern,
            TypedMemberRename::new(
                "trend_micro_vision_one.alert.indicators",
                None,
                vec![("value".into(), "value_object".into(), MemberType::Map)],
            )
        );
    }

    /// The guards are the pattern: a String moves and a Map of the same name
    /// stays, in the SAME walk.
    #[test]
    fn only_the_typed_members_move() {
        let pattern = parse_typed_member_rename(&normalise(PANEL_LOG)).unwrap();
        let mut event = Event::new(json!({ "recordedfuture": { "playbook_alert": {
            "panel_log_v2": [ { "changes": [
                { "old": { "id": "uhash" }, "new": { "id": "uhash" }, "type": "assignee_change" },
                { "old": "New", "new": "InProgress", "type": "status_change" },
                { "removed": [ { "id": "ip:1" } ], "added": [ { "id": "ip:1" } ],
                  "type": "entities_change" },
                { "removed": [ "task:a" ], "added": [ "task:b" ], "type": "action_change" },
            ] } ]
        } } }));
        assert!(typed_member_rename(&mut event, &pattern));

        let changes = event
            .get("recordedfuture.playbook_alert.panel_log_v2")
            .and_then(|v| v.as_array()?.first()?.get("changes")?.as_array().cloned())
            .unwrap();
        let keys = |at: usize| {
            changes[at]
                .as_object()
                .unwrap()
                .keys()
                .cloned()
                .collect::<Vec<_>>()
        };
        // Maps and lists of maps keep the vendor's own names.
        assert_eq!(keys(0), ["old", "new", "type"]);
        assert_eq!(keys(2), ["removed", "added", "type"]);
        // Strings and lists of strings take the `_str` name, at the END, which
        // is where remove-then-assign leaves them -- in the order the SCRIPT
        // renames them, not the order the record happens to carry.
        assert_eq!(keys(1), ["type", "old_str", "new_str"]);
        assert_eq!(keys(3), ["type", "added_str", "removed_str"]);
    }

    #[test]
    fn a_map_member_moves_under_the_map_guard() {
        let pattern = parse_typed_member_rename(&normalise(INDICATORS)).unwrap();
        let mut event = Event::new(json!({ "trend_micro_vision_one": { "alert": {
            "indicators": [
                { "type": "host", "value": { "guid": "0F9BD1AA" } },
                { "type": "url", "value": "https://example.com" },
            ]
        } } }));
        assert!(typed_member_rename(&mut event, &pattern));

        let indicators = event
            .get("trend_micro_vision_one.alert.indicators")
            .and_then(|v| v.as_array().cloned())
            .unwrap();
        assert!(indicators[0].get("value_object").is_some());
        assert!(indicators[0].get("value").is_none());
        // A String under a Map guard is left exactly where the vendor put it.
        assert_eq!(
            indicators[1].get("value").and_then(Value::as_str),
            Some("https://example.com")
        );
    }

    /// A block that is not a typed rename declines the WHOLE script, rather
    /// than running the renames it could read and dropping the rest.
    #[test]
    fn a_walk_doing_anything_else_declines() {
        let script = r#"for (def entity : ctx.a.b) {\n  if (entity.containsKey('entity_value')) {\n    def ev = entity.remove('entity_value');\n    if (ev instanceof Map) {\n      entity.put('value', ev);\n    } else {\n      entity.put('value', ['account_value': ev]);\n    }\n  }\n}\n"#;
        assert!(parse_typed_member_rename(&normalise(script)).is_none());
    }

    /// A list of MAPS is what the element test exists to decline, so a guard
    /// that stops at `instanceof List` is not read as this pattern.
    #[test]
    fn a_bare_list_guard_declines() {
        let script = r#"for (def r : ctx.a.b) {\n  if (r.containsKey('x') && r.x instanceof List) {\n    r.x_str = r.x;\n    r.remove('x');\n  }\n}\n"#;
        assert!(parse_typed_member_rename(&normalise(script)).is_none());
    }

    /// A block that copies without removing leaves the vendor's key in place,
    /// which is a different document and a different matcher.
    #[test]
    fn a_copy_without_a_remove_declines() {
        let script = r#"for (def r : ctx.a.b) {\n  if (r.containsKey('x') && r.x instanceof String) {\n    r.x_str = r.x;\n  }\n}\n"#;
        assert!(parse_typed_member_rename(&normalise(script)).is_none());
    }
}
