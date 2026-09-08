// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One record assembled from whichever named fields the event carries.
//!
//! Suricata reports a DNS answer as four flat fields -- `rrname`, `rdata`,
//! `rrtype`, `ttl` -- and ECS wants them as one member of `dns.answers` under
//! its own names. The pipeline reads each into a local, puts the ones that are
//! present into a map, and writes the map as a ONE-ELEMENT list when it came to
//! anything:
//!
//! ```painless
//! def name = ctx?.suricata?.eve?.dns?.rrname;
//! def data = ctx?.suricata?.eve?.dns?.rdata;
//! def type = ctx?.suricata?.eve?.dns?.rrtype;
//! def ttl  = ctx?.suricata?.eve?.dns?.ttl;
//!
//! def answer = [:];
//! if (name != null) { answer["name"] = name; }
//! ...
//! if (!answer.isEmpty()) { ctx.dns.answers = [answer]; }
//!
//! if (type == "A" || type == "AAAA") { ctx.dns.resolved_ip = [data]; }
//! ```
//!
//! The trailing arm is part of the same matcher rather than one of its own,
//! because `try_known_painless` claims a WHOLE script: reading only the record
//! would leave `dns.resolved_ip` to a second pass that has no way to run.
//!
//! Suricata writes it at three call sites and nothing else in the tree does.
//! Unclaimed it costs the source `dns.answers` on 17 events, `dns.resolved_ip`
//! on 6 and `related.ip` on 6 behind it.

use serde_json::{Map, Value};

use crate::Event;
use crate::painless_params::clean_path;

/// The second write: one field copied out when another names a listed value.
///
/// Both paths are resolved at PARSE time through the locals the script bound,
/// so nothing has to carry a local's name into the run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordAlso {
    /// The `ctx` path whose value decides.
    decider: String,
    /// The values that trigger the write, in the order the script tests them.
    values: Vec<String>,
    /// The `ctx` path that is written.
    member: String,
    /// Where it goes, as a one-element list.
    target: String,
}

/// A record built from the fields the event carries, written as a list of one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordFromFields {
    /// `(key in the record, ctx path)`, in the order the script writes them --
    /// the document is insertion-ordered, so the order is behaviour.
    members: Vec<(String, String)>,
    target: String,
    also: Option<RecordAlso>,
}

/// The `ctx` path a `def <name> = ctx?...;` line reads, keyed by the local.
fn locals(script: &str) -> Vec<(String, String)> {
    script
        .split(';')
        .filter_map(|statement| {
            let (name, value) = statement.trim().strip_prefix("def ")?.split_once('=')?;
            let path = value
                .trim()
                .strip_prefix("ctx?.")
                .or(value.trim().strip_prefix("ctx."))?;
            let path = clean_path(path);
            (!path.is_empty() && !path.contains(['(', ' ', '[']))
                .then(|| (name.trim().to_owned(), path))
        })
        .collect()
}

/// Read the assembly, or decline it.
///
/// Every member has to come from a local the script bound to a `ctx` path, and
/// the guard on each has to name that same local: a record whose members are
/// computed, or guarded on something else, is a different script and running
/// half of it would write a record the vendor does not.
#[must_use]
pub fn parse_record_from_fields(script: &str) -> Option<RecordFromFields> {
    let bound = locals(script);
    let path_of = |name: &str| {
        bound
            .iter()
            .find(|(local, _)| local == name)
            .map(|(_, path)| path.clone())
    };

    // `def <rec> = [:];` -- the map the members are collected into.
    let record = script.split(';').find_map(|statement| {
        let (name, value) = statement.trim().strip_prefix("def ")?.split_once('=')?;
        (value.trim() == "[:]").then(|| name.trim().to_owned())
    })?;

    // `if (<local> != null) { <rec>["<key>"] = <local>; }`
    let mut members = Vec::new();
    for block in script.split("if (").skip(1) {
        let Some((guard, body)) = block.split_once(')') else {
            continue;
        };
        let Some(local) = guard.trim().strip_suffix("!= null").map(str::trim) else {
            continue;
        };
        let Some(assignment) = body
            .split_once('{')
            .and_then(|(_, rest)| rest.split_once('}'))
        else {
            continue;
        };
        let Some((key, value)) = assignment.0.trim().split_once('=') else {
            continue;
        };
        let key = key
            .trim()
            .strip_prefix(&format!("{record}["))
            .and_then(|rest| rest.strip_suffix(']'))
            .map(|inner| inner.trim().trim_matches(['"', '\'']).to_owned());
        let (Some(key), Some(path)) = (key, path_of(local)) else {
            continue;
        };
        if value.trim().trim_end_matches(';').trim() != local {
            return None;
        }
        members.push((key, path));
    }
    if members.is_empty() {
        return None;
    }

    // `if (!<rec>.isEmpty()) { ctx.<target> = [<rec>]; }`
    let (_, written) = script.split_once(&format!("!{record}.isEmpty()"))?;
    let written = written.split_once('{')?.1.split_once('}')?.0;
    let (target, value) = written.trim().split_once('=')?;
    let target = clean_path(target.trim().strip_prefix("ctx.")?);
    if target.is_empty() || value.trim().trim_end_matches(';').trim() != format!("[{record}]") {
        return None;
    }

    Some(RecordFromFields {
        members,
        target,
        also: parse_also(script, &bound),
    })
}

/// `if (<local> == "<v>" || <local> == "<v>") { ctx.<t> = [<other>]; }`
fn parse_also(script: &str, bound: &[(String, String)]) -> Option<RecordAlso> {
    let (_, rest) = script.rsplit_once("if (")?;
    let (guard, body) = rest.split_once(')')?;

    let path_of = |name: &str| {
        bound
            .iter()
            .find(|(local, _)| local == name)
            .map(|(_, path)| path.clone())
    };

    let mut decider = None;
    let mut values = Vec::new();
    for term in guard.split("||") {
        let (local, literal) = term.trim().split_once("==")?;
        let local = local.trim().to_owned();
        if *decider.get_or_insert(local.clone()) != local {
            return None;
        }
        values.push(literal.trim().trim_matches(['"', '\'']).to_owned());
    }
    let decider = path_of(&decider?)?;
    if values.is_empty() {
        return None;
    }

    let written = body.split_once('{')?.1.split_once('}')?.0;
    let (target, value) = written.trim().split_once('=')?;
    let target = clean_path(target.trim().strip_prefix("ctx.")?);
    let member = path_of(
        value
            .trim()
            .trim_end_matches(';')
            .trim()
            .strip_prefix('[')?
            .strip_suffix(']')?
            .trim(),
    )?;
    if target.is_empty() {
        return None;
    }

    Some(RecordAlso {
        decider,
        values,
        member,
        target,
    })
}

/// Build the record, or leave both targets alone.
///
/// A member the event does not carry is left out rather than written null,
/// which is what the script's own `!= null` guard says -- and an empty record
/// writes nothing at all, which is why an event with no DNS section comes out
/// with no `dns.answers`.
#[must_use]
pub fn record_from_fields(event: &mut Event, pattern: &RecordFromFields) -> bool {
    let mut record = Map::new();
    for (key, path) in &pattern.members {
        if let Some(value) = event.get(path).filter(|value| !value.is_null()) {
            record.insert(key.clone(), value.clone());
        }
    }
    if !record.is_empty() {
        let _ = event.set(&pattern.target, Value::Array(vec![Value::Object(record)]));
    }

    if let Some(also) = &pattern.also {
        let decides = event
            .get_str(&also.decider)
            .is_some_and(|held| also.values.iter().any(|value| value == held));
        if decides && let Some(value) = event.get(&also.member).cloned() {
            let _ = event.set(&also.target, Value::Array(vec![value]));
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from `suricata_eve/default.rs`, in the escaped one-line form
    /// the call site holds.
    #[test]
    fn a_dns_answer_is_assembled_from_the_flat_fields() {
        let script = r#"def name = ctx?.suricata?.eve?.dns?.rrname;\ndef data = ctx?.suricata?.eve?.dns?.rdata;\ndef type = ctx?.suricata?.eve?.dns?.rrtype;\ndef ttl = ctx?.suricata?.eve?.dns?.ttl;\n\ndef answer = [:];\nif (name != null) {\n    answer[\"name\"] = name;\n}\nif (data != null) {\n    answer[\"data\"] = data;\n}\nif (type != null) {\n    answer[\"type\"] = type;\n}\nif (ttl != null) {\n    answer[\"ttl\"] = ttl;\n}\nif (!answer.isEmpty()) {\n    ctx.dns.answers = [answer];\n}\n\nif (type == \"A\" || type == \"AAAA\") {\n    ctx.dns.resolved_ip = [data];\n}\n"#;
        let mut event = Event::new(json!({ "suricata": { "eve": { "dns": {
            "rrname": "example.com",
            "rdata": "93.184.216.34",
            "rrtype": "A",
            "ttl": 300,
        }}}}));

        let parsed = parse_record_from_fields(&crate::painless_common::normalise(script))
            .expect("the assembly is recognised");
        assert_eq!(
            parsed.members,
            vec![
                ("name".to_owned(), "suricata.eve.dns.rrname".to_owned()),
                ("data".to_owned(), "suricata.eve.dns.rdata".to_owned()),
                ("type".to_owned(), "suricata.eve.dns.rrtype".to_owned()),
                ("ttl".to_owned(), "suricata.eve.dns.ttl".to_owned()),
            ]
        );

        assert!(crate::painless_common::try_known_painless(
            &mut event, script
        ));
        assert_eq!(
            event.get("dns.answers"),
            Some(&json!([{
                "name": "example.com",
                "data": "93.184.216.34",
                "type": "A",
                "ttl": 300,
            }]))
        );
        assert_eq!(
            event.get("dns.resolved_ip"),
            Some(&json!(["93.184.216.34"]))
        );

        // A member the event does not carry is left OUT, not written null, and
        // a type that is neither A nor AAAA writes no resolved address.
        let mut partial = Event::new(json!({ "suricata": { "eve": { "dns": {
            "rrname": "example.com",
            "rrtype": "NS",
        }}}}));
        assert!(crate::painless_common::try_known_painless(
            &mut partial,
            script
        ));
        assert_eq!(
            partial.get("dns.answers"),
            Some(&json!([{ "name": "example.com", "type": "NS" }]))
        );
        assert_eq!(partial.get("dns.resolved_ip"), None);

        // Nothing at all writes nothing at all, which is what the script's own
        // `isEmpty` guard says.
        let mut quiet = Event::new(json!({ "suricata": { "eve": {} } }));
        assert!(crate::painless_common::try_known_painless(
            &mut quiet, script
        ));
        assert_eq!(quiet.get("dns.answers"), None);
    }

    /// A member that is not the local its guard tested is a different script.
    #[test]
    fn a_record_declines_a_member_from_somewhere_else() {
        let script = r#"def name = ctx.a.one;\ndef other = ctx.a.two;\ndef rec = [:];\nif (name != null) {\n  rec[\"name\"] = other;\n}\nif (!rec.isEmpty()) {\n  ctx.b.list = [rec];\n}\n"#;
        assert!(parse_record_from_fields(&crate::painless_common::normalise(script)).is_none());
    }
}
