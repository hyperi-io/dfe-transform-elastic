// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `system`'s Windows security-descriptor enrichment, transcribed.
//!
//! Five Windows security events carry a descriptor as an SDDL string and a
//! group membership as a space-separated SID run, and neither is readable as
//! shipped. The script translates each SID through the pipeline's own
//! `AccountSIDDescription` table, splits the SDDL into its ACEs, and writes a
//! rendered line per ACE beside the raw value. `windows_forwarded` ships the
//! same script, so the transcription serves both.
//!
//! Two of its readings look like defects and are the vendor's own. The DACL
//! pattern's inner `.*` is greedy, so the run it hands the ACE scanner reaches
//! past the DACL and takes the SACL's entries too -- which are then written a
//! second time under the `Sacl` names. And `perms` is rendered by Java's
//! `AbstractCollection.toString`, so a list lands in the line as `[a, b]`.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// Where every field this reads and writes sits.
const EVENT_DATA: &str = "winlog.event_data";

/// The event codes the descriptor half runs for. Anything else returns after
/// the two remote-account lookups.
const SDDL_CODES: [&str; 5] = ["4670", "4817", "4907", "4908", "4675"];

/// The three accounts an ACE names that are worth an entry in `related.user`.
const NOTABLE: [&str; 3] = ["Administrator", "Guest", "KRBTGT"];

/// `system/security`, the untagged SDDL script in `default` and `standard`.
fn enrich_security_descriptor(event: &mut Event, params: &Value) {
    for (source, target) in [
        ("RemoteMachineID", "RemoteMachineDescription"),
        ("RemoteUserID", "RemoteUserDescription"),
    ] {
        let Some(sid) = event.get_string(&format!("{EVENT_DATA}.{source}")) else {
            continue;
        };
        // A SID the table does not name stores a null, which is what Painless
        // `put` does with a missing lookup -- the module's own prune takes it.
        let named = lookup(params, "AccountSIDDescription", &sid).unwrap_or(Value::Null);
        let _ = event.set(&format!("{EVENT_DATA}.{target}"), named);
    }

    if !event
        .get_str("event.code")
        .is_some_and(|code| SDDL_CODES.contains(&code))
    {
        return;
    }

    for name in ["OldSd", "NewSd"] {
        if let Some(sddl) = event.get_string(&format!("{EVENT_DATA}.{name}")) {
            enrich_sddl(event, params, &sddl, name);
        }
    }
    if let Some(sids) = event.get_string(&format!("{EVENT_DATA}.SidList")) {
        split_sid_list(event, params, &sids);
    }
}

/// One descriptor, written out under `<name>Owner`, `<name>Group`,
/// `<name>Dacl<i>` and `<name>Sacl<i>`.
fn enrich_sddl(event: &mut Event, params: &Value, sddl: &str, name: &str) {
    for (prefix, suffix) in [("O:", "Owner"), ("G:", "Group")] {
        if let Some(sid) = leading_sid(sddl, prefix) {
            let named = translate_sid(params, &sid);
            let _ = event.set(&format!("{EVENT_DATA}.{name}{suffix}"), named);
        }
    }

    let dacl = dfe_core::cached_regex!(r"(D:([A-Z]*(\(.*\))*))");
    let sacl = dfe_core::cached_regex!(r"(S:([A-Z]*(\(.*\))*))?$");
    for (pattern, group, suffix) in [(dacl, 1, "Dacl"), (sacl, 0, "Sacl")] {
        let Some(run) = pattern
            .fast()
            .and_then(|re| re.captures(sddl))
            .and_then(|caught| caught.get(group))
            .map(|m| m.as_str().to_owned())
        else {
            continue;
        };
        for (index, ace) in aces(&run).enumerate() {
            let translated = translate_acl(params, &ace);
            let grantee = translated.get("grantee").cloned().unwrap_or(Value::Null);
            let line = format!(
                "{} :{} ({})",
                crate::helpers::java_to_string(&grantee),
                crate::helpers::java_to_string(translated.get("type").unwrap_or(&Value::Null)),
                crate::helpers::java_to_string(translated.get("perms").unwrap_or(&Value::Null)),
            );
            let _ = event.set(&format!("{EVENT_DATA}.{name}{suffix}{index}"), line);
            if grantee.as_str().is_some_and(|who| NOTABLE.contains(&who)) {
                let _ = event.append_unique("related.user", grantee);
            }
        }
    }
}

/// The ACEs inside a run: each `(...)` holding neither a `*` nor a `)`.
fn aces(run: &str) -> impl Iterator<Item = String> + '_ {
    dfe_core::cached_regex!(r"\([^*\)]*\)")
        .fast()
        .into_iter()
        .flat_map(move |re| re.find_iter(run))
        .map(|m| m.as_str().replace(['(', ')'], ""))
}

/// One ACE's `grantee`, `type` and `perms`, off its semicolon-separated parts.
fn translate_acl(params: &Value, ace: &str) -> Map<String, Value> {
    let parts: Vec<&str> = ace.split(';').collect();
    let mut out = Map::new();
    if parts.len() >= 6 {
        out.insert("grantee".to_owned(), translate_sid(params, parts[5]));
    }
    if let Some(kind) = parts.first() {
        out.insert(
            "type".to_owned(),
            lookup(params, "AceTypes", kind).unwrap_or(Value::Null),
        );
    }
    if let Some(mask) = parts.get(2) {
        let perms = if mask.starts_with("0x") {
            translate_permission_mask(params, mask)
        } else {
            // `/.{1,2}/` walked with `find()` is the string in pairs, the last
            // one short where the length is odd.
            mask.chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|pair| {
                    let flag: String = pair.iter().collect();
                    lookup(params, "PermissionDescription", &flag).unwrap_or(Value::Null)
                })
                .collect()
        };
        out.insert("perms".to_owned(), Value::Array(perms));
    }
    out
}

/// The named flags a hexadecimal access mask carries, or the mask itself where
/// it carries none.
fn translate_permission_mask(params: &Value, mask: &str) -> Vec<Value> {
    let Some(code) = java_decode(mask) else {
        return vec![Value::String(mask.to_owned())];
    };
    let flags = params.get("PermsFlags").and_then(Value::as_object);
    let mut named = Vec::new();
    if let Some(flags) = flags {
        // `entrySet()` walks a Java `HashMap`, so the order is the hash
        // table's rather than the document's.
        for (key, value) in hash_order(flags) {
            if java_decode(key).is_some_and(|flag| code & flag == flag) {
                named.push(value.clone());
            }
        }
    }
    if named.is_empty() {
        named.push(Value::String(mask.to_owned()));
    }
    named
}

/// A SID as the pipeline's tables name it.
///
/// A domain SID the account table does not carry is looked up by its RID
/// instead, which is what makes one table serve every domain.
fn translate_sid(params: &Value, sid: &str) -> Value {
    if let Some(named) = lookup(params, "AccountSIDDescription", sid) {
        return named;
    }
    if !sid.starts_with("S-1-5-21") {
        return Value::String(sid.to_owned());
    }
    let rid = dfe_core::cached_regex!(r"[0-9]{1,5}$")
        .fast()
        .and_then(|re| re.find(sid))
        .map(|m| m.as_str().to_owned());
    match rid {
        // A RID the table does not carry stores a null, not the SID: the
        // script returns the lookup whatever it answered.
        Some(rid) => lookup(params, "DomainSpecificSID", &rid).unwrap_or(Value::Null),
        None => Value::String(sid.to_owned()),
    }
}

/// The space-separated SID run, as a list beside the names it resolves to.
fn split_sid_list(event: &mut Event, params: &Value, sids: &str) {
    let list: Vec<&str> = sids.split(' ').collect();
    let described: Vec<Value> = list
        .iter()
        .map(|sid| translate_sid(params, &sid.replace(['%', '{', '}', ' '], "")))
        .collect();
    let _ = event.set(
        &format!("{EVENT_DATA}.SidList"),
        Value::Array(
            list.iter()
                .map(|sid| Value::String((*sid).to_owned()))
                .collect(),
        ),
    );
    let _ = event.set(
        &format!("{EVENT_DATA}.SidListDesc"),
        Value::Array(described),
    );
}

/// `O:` or `G:` followed by a two-letter SID abbreviation, at the START.
fn leading_sid(sddl: &str, prefix: &str) -> Option<String> {
    let rest = sddl.strip_prefix(prefix)?;
    let abbreviation: String = rest.chars().take(2).collect();
    (abbreviation.len() == 2 && abbreviation.bytes().all(|b| b.is_ascii_uppercase()))
        .then(|| format!("{prefix}{abbreviation}"))
}

/// One entry of a params table, by key.
fn lookup(params: &Value, table: &str, key: &str) -> Option<Value> {
    params.get(table)?.get(key).cloned()
}

/// A params map walked in Java `HashMap` order.
fn hash_order(map: &Map<String, Value>) -> Vec<(&String, &Value)> {
    let table = crate::helpers::java_table_size(map.len());
    let mut entries: Vec<(usize, usize, &String, &Value)> = map
        .iter()
        .enumerate()
        .map(|(position, (key, value))| {
            (
                crate::helpers::java_bucket(key, table),
                position,
                key,
                value,
            )
        })
        .collect();
    entries.sort_by_key(|(bucket, position, ..)| (*bucket, *position));
    entries
        .into_iter()
        .map(|(_, _, key, value)| (key, value))
        .collect()
}

/// `Long.decode`: an optional sign, then `0x`, `0X` or `#` for hexadecimal, a
/// leading `0` for octal, and decimal otherwise.
fn java_decode(text: &str) -> Option<i64> {
    let (sign, body) = match text.strip_prefix('-') {
        Some(rest) => (-1i64, rest),
        None => (1i64, text.strip_prefix('+').unwrap_or(text)),
    };
    let (radix, digits) = if let Some(rest) = body.strip_prefix("0x").or(body.strip_prefix("0X")) {
        (16, rest)
    } else if let Some(rest) = body.strip_prefix('#') {
        (16, rest)
    } else if body.len() > 1 && body.starts_with('0') {
        (8, &body[1..])
    } else {
        (10, body)
    };
    i64::from_str_radix(digits, radix).ok().map(|n| sign * n)
}

/// Every `system` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "5ab51c82dadc0002629689347d7ebcfc15f69c7208ff325fc4939848bf7930ea",
    source: "system",
    name: "enrich_security_descriptor",
    run: enrich_security_descriptor,
}];
