// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One RECORD built per list item that passes a literal guard, appended to an
//! ECS list.
//!
//! `proofpoint_essentials` turns its message parts into `email.attachments` this
//! way, keeping only the parts the vendor marked as attached:
//!
//! ```painless
//! ctx.email = ctx.email ?: [:];
//! ctx.email.attachments = ctx.email.attachments ?: [];
//! for (attachment in ctx.proofpoint_essentials.threat.message_parts) {
//!   if (attachment.disposition == 'attached') {
//!     def o = [:];
//!     o.file = [:];
//!     o.file.hash = [:];
//!     o.file.hash.md5 = attachment.md5;
//!     o.file.name = attachment.filename;
//!     ctx.email.attachments.add(o);
//!   }
//! }
//! ```
//!
//! Distinct from [`crate::append_records`], which appends each record REBUILT
//! from its own keys: here every member is renamed into a nested path the
//! script names, and a guard decides which items contribute at all.
//!
//! `CollectFromList` claimed this on the guard alone and collected
//! `disposition` into `email.attachments` -- the guard's own member, in place
//! of the file record. That arm now reads what is APPENDED and declines a
//! record it did not build from a member.

use serde_json::{Map, Value};

use crate::params::clean_path;
use dfe_core::Event;

/// A guarded record build over one list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardedRecords {
    /// The list walked.
    list: String,
    /// `(member, literal)` an item must match to contribute.
    guard: Option<(String, String)>,
    /// `(path inside the record, the item member it reads)`, in script order.
    writes: Vec<(String, String)>,
    /// The list the built records are appended to.
    target: String,
}

/// `for (<var> in ctx.<list>)`, as its two parts.
fn loop_over_ctx(script: &str) -> Option<(String, String)> {
    let (_, rest) = script.split_once("for (")?;
    let (head, _) = rest.split_once(')')?;
    let (var, source) = head.split_once(" in ")?;
    let var = var.trim().rsplit(' ').next()?.trim();
    let source = source.trim().strip_prefix("ctx.")?;
    (!var.is_empty()).then(|| (var.to_string(), clean_path(source)))
}

/// Read the whole pattern, or `None` where the script is a different one.
#[must_use]
pub fn parse_guarded_records(script: &str) -> Option<GuardedRecords> {
    let (var, list) = loop_over_ctx(script)?;

    // The append names both the target and the local the record was built in.
    let (head, rest) = script.split_once(".add(")?;
    let local: String = rest
        .trim_start()
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if local.is_empty() {
        return None;
    }
    let target = clean_path(
        head.rsplit(['\n', ';', '{', '}'])
            .next()?
            .trim()
            .strip_prefix("ctx.")?,
    );

    // The record must be BUILT here, not appended from the list -- that is
    // `CollectFromList`'s script, and claiming it would collect one member.
    if !script.contains(&format!("{local} = [:]"))
        && !script.contains(&format!("{local} = new HashMap()"))
    {
        return None;
    }

    let assignment = format!("{local}.");
    let source = format!("{var}.");
    let mut writes = Vec::new();
    for statement in script.split(';') {
        let statement = statement.trim_start_matches(['\n', ' ', '{', '}']).trim();
        let Some(rest) = statement.strip_prefix(&assignment) else {
            continue;
        };
        let Some((path, value)) = rest.split_once(" = ") else {
            continue;
        };
        // The scaffolding lines allocate the nested maps; only a read of the
        // item carries a value.
        let Some(member) = value.trim().strip_prefix(&source) else {
            continue;
        };
        let (path, member) = (clean_path(path.trim()), clean_path(member.trim()));
        if !path.is_empty() && !member.is_empty() {
            writes.push((path, member));
        }
    }
    if writes.is_empty() {
        return None;
    }

    Some(GuardedRecords {
        list,
        guard: literal_guard(script, &var),
        writes,
        target,
    })
}

/// `if (<var>.<member> == '<literal>')`, the test that decides which items
/// contribute.
fn literal_guard(script: &str, var: &str) -> Option<(String, String)> {
    let needle = format!("if ({var}.");
    let (_, rest) = script.split_once(&needle)?;
    let (condition, _) = rest.split_once(')')?;
    let (member, literal) = condition.split_once("==")?;
    let literal = literal.trim();
    let quote = literal.chars().next().filter(|c| matches!(c, '\'' | '"'))?;
    let inner = &literal[quote.len_utf8()..];
    let end = inner.find(quote)?;
    Some((member.trim().to_string(), inner[..end].to_string()))
}

/// Write `value` at a dotted `path` inside a record being built.
fn put_nested(record: &mut Map<String, Value>, path: &str, value: Value) {
    let Some((head, rest)) = path.split_once('.') else {
        record.insert(path.to_string(), value);
        return;
    };
    let entry = record
        .entry(head.to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    if !entry.is_object() {
        *entry = Value::Object(Map::new());
    }
    if let Value::Object(inner) = entry {
        put_nested(inner, rest, value);
    }
}

/// Build one record per passing item and append them to the target.
pub fn run_guarded_records(event: &mut Event, pattern: &GuardedRecords) -> bool {
    let Some(Value::Array(items)) = event.get(&pattern.list).cloned() else {
        return true;
    };

    let mut built = Vec::new();
    for item in &items {
        let Value::Object(members) = item else {
            continue;
        };
        if let Some((member, wanted)) = &pattern.guard
            && members.get(member).and_then(Value::as_str) != Some(wanted.as_str())
        {
            continue;
        }
        let mut record = Map::new();
        for (path, member) in &pattern.writes {
            // A member the vendor did not send writes nothing, which is what
            // Painless does with a null: the key is never created.
            match members.get(member) {
                Some(Value::Null) | None => {}
                Some(value) => put_nested(&mut record, path, value.clone()),
            }
        }
        if !record.is_empty() {
            built.push(Value::Object(record));
        }
    }
    if built.is_empty() {
        return true;
    }

    // Appended to whatever the target already holds, exactly as the script's
    // `?: []` preamble leaves it.
    let mut list = match event.get(&pattern.target) {
        Some(Value::Array(existing)) => existing.clone(),
        _ => Vec::new(),
    };
    list.extend(built);
    let _ = event.set(&pattern.target, Value::Array(list));
    true
}

// The script constant is quoted verbatim from a generated call site, which
// spells it `r#"..."#`. Keeping it character-identical is what lets a script be
// copied straight from a module into a test.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::needless_raw_string_hashes)]
mod tests {
    use super::*;
    use crate::common::normalise;

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/proofpoint_essentials_threat/`, in
    /// the escaped one-line form a stored script arrives in.
    const SCRIPT: &str = r#"ctx.email = ctx.email ?: [:];\nctx.email.attachments = ctx.email.attachments ?: [];\nfor (attachment in ctx.proofpoint_essentials.threat.message_parts) {\n  if (attachment.disposition == 'attached') {\n    def o = [:];\n    o.file = [:];\n    o.file.hash = [:];\n    o.file.hash.md5 = attachment.md5;\n    o.file.hash.sha256 = attachment.sha256;\n    o.file.name = attachment.filename;\n    o.file.mime_type = attachment.content_type;\n    ctx.email.attachments.add(o);\n  }\n}\n"#;

    fn pattern() -> GuardedRecords {
        parse_guarded_records(&normalise(SCRIPT)).expect("the record build is read")
    }

    #[test]
    fn the_list_guard_target_and_every_renamed_member_are_read() {
        let pattern = pattern();
        assert_eq!(pattern.list, "proofpoint_essentials.threat.message_parts");
        assert_eq!(pattern.target, "email.attachments");
        assert_eq!(
            pattern.guard,
            Some(("disposition".to_string(), "attached".to_string()))
        );
        assert_eq!(
            pattern.writes,
            vec![
                ("file.hash.md5".to_string(), "md5".to_string()),
                ("file.hash.sha256".to_string(), "sha256".to_string()),
                ("file.name".to_string(), "filename".to_string()),
                ("file.mime_type".to_string(), "content_type".to_string()),
            ]
        );
    }

    /// The captured event: two parts, one inline and one attached, and only
    /// the attached one becomes an attachment.
    #[test]
    fn only_the_items_the_guard_passes_become_records() {
        let mut event = Event::new(serde_json::json!({ "proofpoint_essentials": { "threat": {
            "message_parts": [
                {
                    "content_type": "text/html", "disposition": "inline",
                    "filename": "text.html", "md5": "7d79", "sha256": "486e"
                },
                {
                    "content_type": "application/octet-stream", "disposition": "attached",
                    "filename": "demo.docx.lrf", "md5": "5eb6", "sha256": "b94d"
                }
            ]
        } } }));
        assert!(run_guarded_records(&mut event, &pattern()));
        assert_eq!(
            event.get("email.attachments"),
            Some(&serde_json::json!([{
                "file": {
                    "hash": { "md5": "5eb6", "sha256": "b94d" },
                    "name": "demo.docx.lrf",
                    "mime_type": "application/octet-stream"
                }
            }]))
        );
    }

    /// No part is attached, so nothing is written -- an empty list here would
    /// be a field Elasticsearch does not emit.
    #[test]
    fn a_list_where_nothing_passes_the_guard_writes_no_field() {
        let mut event = Event::new(serde_json::json!({ "proofpoint_essentials": { "threat": {
            "message_parts": [
                { "content_type": "text/html", "disposition": "inline", "filename": "text.html" }
            ]
        } } }));
        assert!(run_guarded_records(&mut event, &pattern()));
        assert!(!event.has("email.attachments"));
    }

    #[test]
    fn a_member_the_vendor_omits_leaves_its_key_out_of_the_record() {
        let mut event = Event::new(serde_json::json!({ "proofpoint_essentials": { "threat": {
            "message_parts": [
                { "disposition": "attached", "filename": "bare.txt", "md5": null }
            ]
        } } }));
        assert!(run_guarded_records(&mut event, &pattern()));
        assert_eq!(
            event.get("email.attachments"),
            Some(&serde_json::json!([{ "file": { "name": "bare.txt" } }]))
        );
    }

    #[test]
    fn records_are_appended_to_a_list_the_document_already_carries() {
        let mut event = Event::new(serde_json::json!({
            "email": { "attachments": [{ "file": { "name": "earlier.txt" } }] },
            "proofpoint_essentials": { "threat": { "message_parts": [
                { "disposition": "attached", "filename": "later.txt" }
            ] } }
        }));
        assert!(run_guarded_records(&mut event, &pattern()));
        assert_eq!(
            event.get("email.attachments"),
            Some(&serde_json::json!([
                { "file": { "name": "earlier.txt" } },
                { "file": { "name": "later.txt" } }
            ]))
        );
    }

    /// A walk that appends a MEMBER rather than a record it built is
    /// `CollectFromList`'s script, and declines here.
    #[test]
    fn a_walk_that_appends_a_member_is_declined() {
        let script = "for (addr in ctx.cisco.computer.network_addresses) {\n\
            if (addr.ip != null) { ctx.host.ip.add(addr.ip); }\n}";
        assert!(parse_guarded_records(&normalise(script)).is_none());
    }
}
