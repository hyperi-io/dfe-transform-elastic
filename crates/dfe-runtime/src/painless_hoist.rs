// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One member lifted out of every record of a list, into a list of its own.
//!
//! zeek's DNS stream zips its answers and TTLs into `dns.answers`, lets a
//! `foreach` processor tag each record whose data parsed as an address, then
//! takes the tags back out and drops them:
//!
//! ```painless
//! def answers = ctx.dns.answers;
//! def iplist = new ArrayList();
//! for (def i = 0; i < ctx.dns.answers.length; i++) {
//!   if (answers[i].containsKey("tmpip")) {
//!     iplist.add(answers[i].tmpip);
//!     answers[i].remove("tmpip");
//!   }
//! }
//! ctx.dns.resolved_ip = iplist;
//! ```
//!
//! `RecordRenames` in `painless_records` reads the nearest sibling -- suricata's
//! `for (def answer : ctx.dns.answers)` -- and cannot take this one: it walks by
//! ITEM rather than by index, its trigger is a `remove` bound to a local, and it
//! writes its gathered list only when the walk came to something, where zeek
//! writes an empty one. The empty list is the answer on 2 of the stream's
//! 7 events.

use serde_json::Value;

use crate::Event;
use crate::painless_params::clean_path;

/// One member gathered out of a list's records and removed from each.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoistMember {
    list: String,
    /// The key taken out of each record that carries it.
    member: String,
    target: String,
}

/// The `ctx` path a `def <name> = ctx...;` line reads, keyed by the local.
fn locals(script: &str) -> Vec<(String, String)> {
    script
        .split(';')
        .filter_map(|statement| {
            let (name, value) = statement.trim().strip_prefix("def ")?.split_once('=')?;
            let path = clean_path(
                value
                    .trim()
                    .strip_prefix("ctx")?
                    .trim_start_matches(['?', '.']),
            );
            (!path.is_empty() && !path.contains(['(', ' ', '[']))
                .then(|| (name.trim().to_owned(), path))
        })
        .collect()
}

/// Read the hoist, or decline it.
///
/// The key gathered and the key removed have to be the same one, taken from the
/// loop's own record: a walk that moves some other member is a different script
/// and running this half of it would drop a field the vendor keeps.
#[must_use]
pub fn parse_hoist_member(script: &str) -> Option<HoistMember> {
    let bound = locals(script);

    // `def <acc> = new ArrayList();` -- what the gathered members collect into.
    let accumulator = script.split(';').find_map(|statement| {
        let (name, value) = statement.trim().strip_prefix("def ")?.split_once('=')?;
        (value.trim() == "new ArrayList()").then(|| name.trim().to_owned())
    })?;

    // `if (<local>[i].containsKey("<key>"))`
    let (head, rest) = script.split_once("[i].containsKey(")?;
    let name = head.rsplit(['(', ' ', '\n', '!']).next()?.trim();
    let list = bound
        .iter()
        .find(|(local, _)| local == name)
        .map(|(_, path)| path.clone())?;
    let (member, body) = rest.split_once(')')?;
    let member = member.trim().trim_matches(['"', '\'']).to_owned();
    if member.is_empty() {
        return None;
    }

    // `<acc>.add(<local>[i].<key>); <local>[i].remove("<key>");`
    let gathers = format!("{accumulator}.add({name}[i].{member})");
    let (dropped, _) = body
        .split_once(&format!("{name}[i].remove("))?
        .1
        .split_once(')')?;
    if !body.contains(&gathers) || dropped.trim().trim_matches(['"', '\'']) != member {
        return None;
    }

    // `ctx.<target> = <acc>;`, anchored on the LAST such write.
    let (written, _) = script.rsplit_once(&format!("= {accumulator};"))?;
    let target = written.trim().rsplit(['\n', ' ', '}', ';']).next()?;
    let target = clean_path(
        target
            .trim()
            .strip_prefix("ctx")?
            .trim_start_matches(['?', '.']),
    );
    (!target.is_empty() && !target.contains(['(', '['])).then_some(HoistMember {
        list,
        member,
        target,
    })
}

/// Lift the member out of every record that carries it.
///
/// The gathered list is written even when it is EMPTY, which is what the
/// script's unguarded `ctx.<target> = <acc>;` says.
#[must_use]
pub fn hoist_member(event: &mut Event, pattern: &HoistMember) -> bool {
    let Some(Value::Array(records)) = event.get(&pattern.list).cloned() else {
        return true;
    };

    let mut gathered = Vec::new();
    let mut kept = Vec::with_capacity(records.len());
    for record in records {
        let Value::Object(mut members) = record else {
            kept.push(record);
            continue;
        };
        if let Some(value) = members.shift_remove(pattern.member.as_str()) {
            gathered.push(value);
        }
        kept.push(Value::Object(members));
    }

    let _ = event.set(&pattern.list, Value::Array(kept));
    let _ = event.set(&pattern.target, Value::Array(gathered));
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/zeek_dns/default.rs`, in the escaped
    /// one-line form the call site holds.
    const ZEEK_DNS_HOIST: &str = r#"def answers = ctx.dns.answers; def iplist = new ArrayList(); for (def i = 0; i < ctx.dns.answers.length; i++) {\n  if (answers[i].containsKey(\"tmpip\")) {\n    iplist.add(answers[i].tmpip);\n    answers[i].remove(\"tmpip\");\n  }\n} ctx.dns.resolved_ip = iplist;"#;

    #[test]
    fn the_zeek_resolved_addresses_are_hoisted_out_of_their_records() {
        assert_eq!(
            parse_hoist_member(&crate::painless_common::normalise(ZEEK_DNS_HOIST)),
            Some(HoistMember {
                list: "dns.answers".to_owned(),
                member: "tmpip".to_owned(),
                target: "dns.resolved_ip".to_owned(),
            })
        );

        // The written value, not the parse.
        let mut event = Event::new(json!({ "dns": { "answers": [
            { "data": "proxy.example.com", "ttl": 119 },
            { "data": "89.160.20.156", "ttl": 59, "tmpip": "89.160.20.156" },
        ]}}));
        assert!(crate::painless_common::try_known_painless(
            &mut event,
            ZEEK_DNS_HOIST
        ));
        assert_eq!(
            event.get("dns.resolved_ip"),
            Some(&json!(["89.160.20.156"]))
        );
        // The tag is REMOVED from the record it was taken from.
        assert_eq!(
            event.get("dns.answers"),
            Some(&json!([
                { "data": "proxy.example.com", "ttl": 119 },
                { "data": "89.160.20.156", "ttl": 59 },
            ]))
        );

        // No record carrying the tag still writes the empty list, which is what
        // the script's unguarded assignment says.
        let mut none = Event::new(json!({ "dns": { "answers": [
            { "data": "proxy.example.com", "ttl": 119 },
        ]}}));
        assert!(crate::painless_common::try_known_painless(
            &mut none,
            ZEEK_DNS_HOIST
        ));
        assert_eq!(none.get("dns.resolved_ip"), Some(&json!([])));

        // An absent list writes nothing at all.
        let mut quiet = Event::new(json!({ "dns": {} }));
        assert!(crate::painless_common::try_known_painless(
            &mut quiet,
            ZEEK_DNS_HOIST
        ));
        assert_eq!(quiet.get("dns.resolved_ip"), None);
    }

    /// A gather and a removal naming DIFFERENT keys moves a member rather than
    /// hoisting one.
    #[test]
    fn a_hoist_removing_another_key_is_declined() {
        let script = r#"def rows = ctx.a.list; def acc = new ArrayList(); for (def i = 0; i < rows.length; i++) {\n  if (rows[i].containsKey(\"one\")) {\n    acc.add(rows[i].one);\n    rows[i].remove(\"two\");\n  }\n} ctx.a.out = acc;"#;
        assert!(parse_hoist_member(&crate::painless_common::normalise(script)).is_none());
    }

    /// A walk that gathers a member it never removes leaves the record whole,
    /// so it is a different script.
    #[test]
    fn a_hoist_that_keeps_the_member_is_declined() {
        let script = r#"def rows = ctx.a.list; def acc = new ArrayList(); for (def i = 0; i < rows.length; i++) {\n  if (rows[i].containsKey(\"one\")) {\n    acc.add(rows[i].one);\n  }\n} ctx.a.out = acc;"#;
        assert!(parse_hoist_member(&crate::painless_common::normalise(script)).is_none());
    }
}
