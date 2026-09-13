// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A list of records folded into a map whose keys are one member's VALUE.
//!
//! `anthropic_metrics` is the bare form -- three lines that give each rate limit
//! its own field beside the list it came from:
//!
//! ```painless
//! for (limit in ctx.anthropic.rate_limit.limits) {
//!   ctx.anthropic.rate_limit[limit.type] = limit.value;
//! }
//! ```
//!
//! `ti_recordedfuture` writes the same fold with three options on top: the key
//! lower-cased, a SUFFIX on it when the value came from the other member, a
//! side list every full hash is also appended to, and the walked path REPLACED
//! by the fold or removed when nothing folded.
//!
//! ```painless
//! def flat = [:];
//! ctx.related.hash = ctx.related.hash ?: [];
//! for (h in ctx.recordedfuture.identity_detection.password.hashes) {
//!   if (h.algorithm != null) {
//!     def algorithm = h.algorithm.toLowerCase();
//!     if (h.hash != null) {
//!       flat[algorithm] = h.hash;
//!       ctx.related.hash.add(h.hash);
//!     } else if (h.hash_prefix != null) {
//!       flat[algorithm + '_prefix'] = h.hash_prefix;
//!     }
//!   }
//! }
//! if (flat.size() > 0) {
//!   ctx.recordedfuture.identity_detection.password.hashes = flat;
//! } else {
//!   ctx.recordedfuture.identity_detection.password.remove('hashes');
//! }
//! ```
//!
//! The key is a value, so it cannot be read off the script and nothing
//! downstream can be written without it -- every `password.hashes.<algorithm>`
//! field and the whole of `related.hash`.
//!
//! `ti_recordedfuture`'s spelling binds `CollectingLadder` on its `.add(`,
//! `.size()` and `else if (` and is INERT there: that runner opens on
//! `if (ctx.` and every `if` in this script tests a local. So the arm sits ahead
//! of that trigger, which pushes without returning.
//!
//! Sitting there puts it ahead of `KeyValuePairs` as well, which reads the
//! `{key, value}` spelling of the same fold and is the narrower of the two --
//! it audits the whole loop body, keeps a record's null value, and writes the
//! empty map an empty list folds to. [`parse_record_fold`] declines that script
//! so the arm below keeps it.

use serde_json::{Map, Value};

use crate::params::{balanced, clean_path, is_ctx_path};
use dfe_core::Event;

/// One value member, and what the key carries when the value came from it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldArm {
    /// The member holding the value.
    member: String,
    /// What is appended to the key for a value from this member.
    suffix: String,
    /// A list every value from this arm is also appended to.
    side: Option<String>,
}

/// A list of records folded into a map keyed by one member's value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordFold {
    /// The list of records walked.
    list: String,
    /// The member whose value names the key.
    key: String,
    /// Whether the key is lower-cased on the way in.
    lower: bool,
    /// The arms in script order; the first a record carries is the one taken.
    arms: Vec<FoldArm>,
    /// The map the folded keys land in.
    target: String,
    /// Whether the fold REPLACES what sits at the target rather than joining it.
    replace: bool,
    /// Whether a fold that came to nothing removes the target.
    remove_when_empty: bool,
}

/// `for (<var> in ctx.<list>)`, as the variable, the list, and the loop body.
fn loop_body(script: &str) -> Option<(String, String, String)> {
    let (_, rest) = script.split_once("for (")?;
    let (head, after) = rest.split_once(')')?;
    let (var, source) = head.split_once(" in ")?;
    let var = var.trim().rsplit(' ').next()?.trim();
    let list = clean_path(source.trim().strip_prefix("ctx.")?);
    if var.is_empty() || !is_ctx_path(&list) {
        return None;
    }
    let (body, _) = balanced(after.trim_start(), '{', '}')?;
    Some((var.to_owned(), list, body.to_owned()))
}

/// A bare member name, or `None` where the fragment is anything richer.
fn member(fragment: &str) -> Option<&str> {
    let name = fragment.trim();
    (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')).then_some(name)
}

/// A single- or double-quoted literal's contents.
fn quoted(text: &str) -> Option<&str> {
    let text = text.trim();
    let quote = text.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    text.strip_prefix(quote)?.strip_suffix(quote)
}

/// The subject a statement's left-hand side ends on, past any brace or newline.
fn trailing_token(text: &str) -> &str {
    text.trim_end()
        .rsplit(['\n', '\r', '\t', ' ', '{', '}', ';'])
        .next()
        .unwrap_or("")
        .trim()
}

/// The member a local key was DECLARED from, and whether it was lower-cased.
fn key_from_local(script: &str, local: &str, var: &str) -> Option<(String, bool)> {
    let reads = format!("{var}.");
    script.split([';', '\n']).find_map(|statement| {
        let (lhs, rhs) = statement.split_once('=')?;
        let mut words = lhs.split_whitespace();
        // A declarator, not a reassignment: `algorithm = something_else` later
        // in the script would name a key the fold never uses.
        if words.next_back()? != local || words.next().is_none() {
            return None;
        }
        let rhs = rhs.trim();
        let (read, lower) = rhs
            .strip_suffix(".toLowerCase()")
            .map_or((rhs, false), |read| (read, true));
        Some((member(read.strip_prefix(&reads)?)?.to_owned(), lower))
    })
}

/// The key member, whether it is lower-cased, and the suffix this write adds.
fn read_key(expr: &str, var: &str, script: &str) -> Option<(String, bool, String)> {
    let mut parts = expr.split('+');
    let base = parts.next()?.trim();
    let mut suffix = String::new();
    for part in parts {
        suffix.push_str(quoted(part)?);
    }
    let (key, lower) = match base.strip_prefix(&format!("{var}.")) {
        Some(read) => (member(read)?.to_owned(), false),
        None => key_from_local(script, member(base)?, var)?,
    };
    Some((key, lower, suffix))
}

/// Whether the script DECLARES `local` as an empty map.
fn declares_empty_map(script: &str, local: &str) -> bool {
    script.split([';', '\n']).any(|statement| {
        let Some((lhs, rhs)) = statement.split_once('=') else {
            return false;
        };
        let mut words = lhs.split_whitespace();
        let Some(name) = words.next_back() else {
            return false;
        };
        if name != local || words.next().is_none() {
            return false;
        }
        let value = rhs.trim();
        value.starts_with("[:]") || value.starts_with("new HashMap()")
    })
}

/// The `ctx.` path a local map is handed to after the loop.
fn handed_to_ctx(script: &str, local: &str) -> Option<String> {
    script.split([';', '\n']).find_map(|statement| {
        let (lhs, rhs) = statement.split_once('=')?;
        if rhs.trim() != local {
            return None;
        }
        let path = clean_path(trailing_token(lhs).strip_prefix("ctx.")?);
        is_ctx_path(&path).then_some(path)
    })
}

/// Whether the script removes the target when the fold came to nothing.
fn removes_target(script: &str, target: &str) -> bool {
    let Some((parent, leaf)) = target.rsplit_once('.') else {
        return false;
    };
    ["'", "\""]
        .iter()
        .any(|quote| script.contains(&format!("ctx.{parent}.remove({quote}{leaf}{quote})")))
}

/// Where the fold lands, whether it replaces what is there, and whether an
/// empty fold removes it.
fn destination(script: &str, receiver: &str) -> Option<(String, bool, bool)> {
    if let Some(path) = receiver.strip_prefix("ctx.") {
        let path = clean_path(path);
        return is_ctx_path(&path).then_some((path, false, false));
    }
    if !declares_empty_map(script, member(receiver)?) {
        return None;
    }
    let target = handed_to_ctx(script, receiver)?;
    let removes = removes_target(script, &target);
    Some((target, true, removes))
}

/// Every `ctx.<path>.add(<var>.<member>)` in the body, as member to path.
///
/// An append this cannot read declines the whole script: the fold is right and
/// a list the vendor also fills is left empty, which reads as a correct
/// transform over a short list.
fn side_lists(body: &str, var: &str) -> Option<Vec<(String, String)>> {
    let reads = format!("{var}.");
    let mut found = Vec::new();
    for (at, _) in body.match_indices(".add(") {
        let path = clean_path(trailing_token(&body[..at]).strip_prefix("ctx.")?);
        if !is_ctx_path(&path) {
            return None;
        }
        let argument = body[at + ".add(".len()..].split(')').next()?;
        found.push((member(argument.strip_prefix(&reads)?)?.to_owned(), path));
    }
    Some(found)
}

/// Read the fold, or decline the script.
///
/// Every subscript write in the loop has to store one member of the record
/// under a key built from another, all of them into the same map and off the
/// same key member. A write this cannot read declines the whole script rather
/// than folding the half of it that parsed.
#[must_use]
pub fn parse_record_fold(script: &str) -> Option<RecordFold> {
    let (var, list, body) = loop_body(script)?;
    let sides = side_lists(&body, &var)?;

    let mut destination_of: Option<(String, bool, bool)> = None;
    let mut key_of: Option<(String, bool)> = None;
    let mut arms: Vec<FoldArm> = Vec::new();
    let mut previous_end = 0usize;

    for (at, _) in body.match_indices("] = ") {
        let (receiver_text, key_expr) = body[..at].rsplit_once('[')?;
        let receiver = trailing_token(receiver_text);
        let start = at + "] = ".len();
        let value = body[start..].split([';', '\n']).next()?;
        let held = member(value.trim().strip_prefix(&format!("{var}."))?)?;

        let destination = destination(script, receiver)?;
        let (key, lower, suffix) = read_key(key_expr, &var, script)?;
        // Two maps, or two key members, is a fan-out this cannot express.
        if destination_of
            .as_ref()
            .is_some_and(|held| *held != destination)
            || key_of
                .as_ref()
                .is_some_and(|held| held.0 != key || held.1 != lower)
        {
            return None;
        }
        destination_of = Some(destination);
        key_of = Some((key, lower));

        // The arms are ALTERNATIVES, so a second write reachable on its own is
        // a different script and the first-present rule would drop it.
        if !arms.is_empty() && !body[previous_end..at].contains("else") {
            return None;
        }
        previous_end = start;

        arms.push(FoldArm {
            member: held.to_owned(),
            suffix,
            side: sides
                .iter()
                .find(|(from, _)| from == held)
                .map(|(_, path)| path.clone()),
        });
    }

    let (target, replace, remove_when_empty) = destination_of?;
    let (key, lower) = key_of?;
    if arms.is_empty() || is_key_value_fold(replace, &key, &arms) {
        return None;
    }
    Some(RecordFold {
        list,
        key,
        lower,
        arms,
        target,
        replace,
        remove_when_empty,
    })
}

/// Whether this is the `{key, value}` fold `KeyValuePairs` reads.
///
/// That arm sits below this one and is the narrower of the two: it audits the
/// whole loop body, keeps a record's NULL value, and writes the empty map an
/// empty list folds to. This reader does none of the three, so a script it
/// claims stays with the arm that can.
fn is_key_value_fold(replace: bool, key: &str, arms: &[FoldArm]) -> bool {
    replace && key == "key" && arms.len() == 1 && arms[0].member == "value"
}

/// Fold the list into its map, and feed whatever side lists the arms name.
///
/// A record whose key member is absent or is not text is SKIPPED rather than
/// folded under a name chosen for it, and so is one carrying none of the arms'
/// members -- there is nothing to store and Painless would write a null the
/// prune beside these call sites removes again.
#[must_use]
pub fn record_fold(event: &mut Event, pattern: &RecordFold) -> bool {
    let Some(Value::Array(records)) = event.get(&pattern.list).cloned() else {
        return true;
    };

    let mut folded = Map::new();
    let mut appended: Vec<(&str, Value)> = Vec::with_capacity(records.len());
    for record in &records {
        let Some(name) = record.get(&pattern.key).and_then(Value::as_str) else {
            continue;
        };
        let key = if pattern.lower {
            name.to_lowercase()
        } else {
            name.to_owned()
        };
        if key.is_empty() {
            continue;
        }
        for arm in &pattern.arms {
            let Some(value) = record.get(&arm.member).filter(|value| !value.is_null()) else {
                continue;
            };
            folded.insert(format!("{key}{}", arm.suffix), value.clone());
            if let Some(path) = &arm.side {
                appended.push((path, value.clone()));
            }
            break;
        }
    }

    if pattern.replace {
        if folded.is_empty() {
            if pattern.remove_when_empty {
                event.remove(&pattern.target);
            }
        } else {
            let _ = event.set(&pattern.target, Value::Object(folded));
        }
    } else {
        // Sibling keys on the map that holds the list, which stays where it is.
        let mut held = match event.get(&pattern.target) {
            Some(Value::Object(map)) => map.clone(),
            _ => Map::new(),
        };
        held.extend(folded);
        let _ = event.set(&pattern.target, Value::Object(held));
    }

    for (path, value) in appended {
        let _ = event.append(path, value);
    }
    true
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/anthropic_metrics_rate_limit/default.rs`.
    const RATE_LIMIT: &str = r"for (limit in ctx.anthropic.rate_limit.limits) {\n  ctx.anthropic.rate_limit[limit.type] = limit.value;\n}";

    /// Verbatim from
    /// `crates/dfe-transforms/src/filebeat/ti_recordedfuture_identity_detection/default.rs`.
    const HASHES: &str = r"def flat = [:];\nctx.related = ctx.related ?: [:];\nctx.related.hash = ctx.related.hash ?: [];\nfor (h in ctx.recordedfuture.identity_detection.password.hashes) {\n  if (h.algorithm != null) {\n    def algorithm = h.algorithm.toLowerCase();\n    if (h.hash != null) {\n      flat[algorithm] = h.hash;\n      ctx.related.hash.add(h.hash);\n    } else if (h.hash_prefix != null) {\n      flat[algorithm + '_prefix'] = h.hash_prefix;\n    }\n  }\n}\nif (flat.size() > 0) {\n  ctx.recordedfuture.identity_detection.password.hashes = flat;\n} else {\n  ctx.recordedfuture.identity_detection.password.remove('hashes');\n}";

    #[test]
    fn the_bare_fold_writes_siblings_of_the_list() {
        assert_eq!(
            parse_record_fold(&crate::common::normalise(RATE_LIMIT)),
            Some(RecordFold {
                list: "anthropic.rate_limit.limits".to_owned(),
                key: "type".to_owned(),
                lower: false,
                arms: vec![FoldArm {
                    member: "value".to_owned(),
                    suffix: String::new(),
                    side: None,
                }],
                target: "anthropic.rate_limit".to_owned(),
                replace: false,
                remove_when_empty: false,
            })
        );
    }

    #[test]
    fn the_hash_fold_reads_all_three_options() {
        assert_eq!(
            parse_record_fold(&crate::common::normalise(HASHES)),
            Some(RecordFold {
                list: "recordedfuture.identity_detection.password.hashes".to_owned(),
                key: "algorithm".to_owned(),
                lower: true,
                arms: vec![
                    FoldArm {
                        member: "hash".to_owned(),
                        suffix: String::new(),
                        side: Some("related.hash".to_owned()),
                    },
                    FoldArm {
                        member: "hash_prefix".to_owned(),
                        suffix: "_prefix".to_owned(),
                        side: None,
                    },
                ],
                target: "recordedfuture.identity_detection.password.hashes".to_owned(),
                replace: true,
                remove_when_empty: true,
            })
        );
    }

    /// The document the anthropic capture expects: a field per limit beside the
    /// list, which stays.
    #[test]
    fn each_rate_limit_gets_its_own_field() {
        let mut event = Event::new(json!({ "anthropic": { "rate_limit": {
            "group_type": "model_group",
            "limits": [
                { "type": "requests_per_minute", "value": 4000 },
                { "type": "output_tokens_per_minute", "value": 800_000 },
            ],
            "type": "rate_limit",
        }}}));
        assert!(crate::common::try_known_painless(&mut event, RATE_LIMIT));
        assert_eq!(
            event.get("anthropic.rate_limit.requests_per_minute"),
            Some(&json!(4000))
        );
        assert_eq!(
            event.get("anthropic.rate_limit.output_tokens_per_minute"),
            Some(&json!(800_000))
        );
        assert_eq!(
            event.get("anthropic.rate_limit.group_type"),
            Some(&json!("model_group"))
        );
        assert!(event.get("anthropic.rate_limit.limits").is_some());
    }

    /// The `ti_recordedfuture` capture's first document: four algorithms, keys
    /// lower-cased, and `related.hash` in the order the vendor sent them.
    #[test]
    fn the_hashes_fold_under_their_lower_cased_algorithm() {
        let mut event = Event::new(json!({ "recordedfuture": { "identity_detection": {
            "password": { "hashes": [
                { "algorithm": "SHA1", "hash": "1465901e" },
                { "algorithm": "SHA256", "hash": "ca527550" },
                { "algorithm": "NTLM", "hash": "aa2a03d6" },
                { "algorithm": "MD5", "hash": "bd55f770" },
            ]},
        }}}));
        assert!(crate::common::try_known_painless(&mut event, HASHES));
        assert_eq!(
            event.get("recordedfuture.identity_detection.password.hashes"),
            Some(&json!({
                "sha1": "1465901e",
                "sha256": "ca527550",
                "ntlm": "aa2a03d6",
                "md5": "bd55f770",
            }))
        );
        assert_eq!(
            event.get("related.hash"),
            Some(&json!(["1465901e", "ca527550", "aa2a03d6", "bd55f770"]))
        );
    }

    /// A prefix takes the suffixed key and feeds NO side list, which is the
    /// second capture document exactly.
    #[test]
    fn a_prefix_is_suffixed_and_never_reaches_related_hash() {
        let mut event = Event::new(json!({ "recordedfuture": { "identity_detection": {
            "password": { "hashes": [
                { "algorithm": "SHA256", "hash_prefix": "b82d246e" },
                { "algorithm": "NTLM", "hash": "0eda17a1" },
            ]},
        }}}));
        assert!(crate::common::try_known_painless(&mut event, HASHES));
        assert_eq!(
            event.get("recordedfuture.identity_detection.password.hashes"),
            Some(&json!({ "sha256_prefix": "b82d246e", "ntlm": "0eda17a1" }))
        );
        assert_eq!(event.get("related.hash"), Some(&json!(["0eda17a1"])));
    }

    /// A record carrying both members takes the FIRST arm, which is what the
    /// vendor's `else if` says.
    #[test]
    fn a_record_with_both_members_takes_the_first_arm() {
        let mut event = Event::new(json!({ "recordedfuture": { "identity_detection": {
            "password": { "hashes": [
                { "algorithm": "MD5", "hash": "full", "hash_prefix": "part" },
            ]},
        }}}));
        assert!(crate::common::try_known_painless(&mut event, HASHES));
        assert_eq!(
            event.get("recordedfuture.identity_detection.password.hashes"),
            Some(&json!({ "md5": "full" }))
        );
    }

    /// Nothing folded removes the walked path rather than leaving the list.
    #[test]
    fn an_empty_fold_removes_the_target() {
        let mut event = Event::new(json!({ "recordedfuture": { "identity_detection": {
            "password": { "hashes": [{ "note": "no algorithm" }] },
        }}}));
        assert!(crate::common::try_known_painless(&mut event, HASHES));
        assert_eq!(
            event.get("recordedfuture.identity_detection.password.hashes"),
            None
        );
    }

    /// `related.hash` is joined, not replaced -- an earlier processor's hashes
    /// stay in front of the ones this appends.
    #[test]
    fn an_existing_related_hash_is_appended_to() {
        let mut event = Event::new(json!({
            "related": { "hash": ["already-here"] },
            "recordedfuture": { "identity_detection": { "password": { "hashes": [
                { "algorithm": "MD5", "hash": "bd55f770" },
            ]}}},
        }));
        assert!(crate::common::try_known_painless(&mut event, HASHES));
        assert_eq!(
            event.get("related.hash"),
            Some(&json!(["already-here", "bd55f770"]))
        );
    }

    /// Two writes that are not alternatives would both run, and taking the
    /// first would drop the second.
    #[test]
    fn independent_writes_decline_the_script() {
        let script = r"def m = [:];\nfor (r in ctx.a.list) {\n  m[r.k] = r.one;\n  m[r.k] = r.two;\n}\nctx.out = m;";
        assert!(parse_record_fold(&crate::common::normalise(script)).is_none());
    }

    /// A write whose value is not a bare member -- a call, a sum, a literal --
    /// declines rather than folding the readable half.
    #[test]
    fn an_unreadable_value_declines_the_script() {
        let script =
            r"def m = [:];\nfor (r in ctx.a.list) {\n  m[r.k] = r.one.toString();\n}\nctx.out = m;";
        assert!(parse_record_fold(&crate::common::normalise(script)).is_none());
    }

    /// A local map nothing hands to a `ctx.` path writes nowhere.
    #[test]
    fn a_local_map_never_handed_to_ctx_declines_the_script() {
        let script = r"def m = [:];\nfor (r in ctx.a.list) {\n  m[r.k] = r.one;\n}";
        assert!(parse_record_fold(&crate::common::normalise(script)).is_none());
    }

    /// `KeyValuePairs`'s own script -- azure's sign-in details, verbatim from
    /// `crates/dfe-transforms/src/filebeat/azure_signinlogs/default.rs`. The
    /// narrower arm keeps it.
    #[test]
    fn the_key_value_fold_is_left_to_its_own_arm() {
        let script = r"def tmp = [:];\nfor (item in ctx.azure.signinlogs.properties.authentication_processing_details) {\n    tmp[item.key] = item.value;\n}\nctx.azure.signinlogs.properties.authentication_processing_details = tmp;\n";
        assert!(parse_record_fold(&crate::common::normalise(script)).is_none());
    }

    /// The same member names folded into a SIBLING map is not that script --
    /// `KeyValuePairs` demands the fold land back on the list it read.
    #[test]
    fn key_and_value_folded_elsewhere_still_read() {
        let script = r"for (item in ctx.a.list) {\n  ctx.a[item.key] = item.value;\n}";
        assert!(parse_record_fold(&crate::common::normalise(script)).is_some());
    }

    /// `GroupRecords`'s bucket seed is a subscript write too, and its value is
    /// an empty list rather than a member.
    #[test]
    fn a_bucket_seed_declines_the_script() {
        let script = r"ArrayList rows = ctx.a.list;\nfor (int i = 0; i < rows.length; i++) {\n  String k = rows[i]['node']['kind'];\n  if (!ctx.out.bucket.containsKey(k)) {\n    ctx.out.bucket[k] = [];\n  }\n  ctx.out.bucket[k].add(rows[i]['node']);\n}\n";
        assert!(parse_record_fold(&crate::common::normalise(script)).is_none());
    }
}
