// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Windows security descriptors, as the security pipeline renders them.
//!
//! Events 4670, 4817, 4907 and 4908 carry an SDDL string -- `D:(A;;GA;;;SY)`
//! and friends -- and the pipeline expands each access-control entry into a
//! readable line: `Local system :Access Allowed ([Generic All])`. Every table
//! it needs is in the script's own `params`, so nothing here is hard-coded:
//! the SID descriptions, the domain-relative RIDs, the ACE types, the
//! two-letter permission names and the numeric permission flags all come from
//! the pipeline.
//!
//! The vendor's own quirks are reproduced rather than corrected, because the
//! value Elasticsearch stores is what parity is measured against:
//!
//! - the Owner and Group patterns capture the `O:` / `G:` prefix WITH the two
//!   letters, so the lookup is on `O:BA` and never finds anything;
//! - a SID whose description is absent renders as the literal text `null`,
//!   which is Java concatenating a null reference;
//! - a domain SID's RID is read as the LAST FIVE digits at most, so a long
//!   trailing number is truncated rather than skipped.

use serde_json::{Map, Value};

use dfe_core::event::Event;

/// The `params` tables this needs, by the names the script gives them.
struct Tables<'a> {
    sids: Option<&'a Map<String, Value>>,
    domain_sids: Option<&'a Map<String, Value>>,
    ace_types: Option<&'a Map<String, Value>>,
    permissions: Option<&'a Map<String, Value>>,
    permission_flags: Option<&'a Map<String, Value>>,
}

impl<'a> Tables<'a> {
    fn read(params: &'a Map<String, Value>) -> Self {
        let table = |name: &str| params.get(name).and_then(Value::as_object);
        Self {
            sids: table("AccountSIDDescription"),
            domain_sids: table("DomainSpecificSID"),
            ace_types: table("AceTypes"),
            permissions: table("PermissionDescription"),
            permission_flags: table("PermsFlags"),
        }
    }
}

/// The event codes the SDDL half of the script runs for.
const SDDL_CODES: [&str; 4] = ["4670", "4817", "4907", "4908"];

/// Users the script collects into `related.user` as it renders each entry.
const RELATED_GRANTEES: [&str; 3] = ["Administrator", "Guest", "KRBTGT"];

/// Run the security-descriptor half of `security_standard`'s script.
pub(crate) fn run(event: &mut Event, params: &Map<String, Value>) -> bool {
    let tables = Tables::read(params);

    // These two run for every event code, ahead of the SDDL gate.
    for (source, target) in [
        ("RemoteMachineID", "RemoteMachineDescription"),
        ("RemoteUserID", "RemoteUserDescription"),
    ] {
        let path = format!("winlog.event_data.{source}");
        if let Some(id) = event.get_str(&path).map(str::to_string)
            && let Some(description) = tables.sids.and_then(|table| table.get(&id)).cloned()
        {
            let _ = event.set(&format!("winlog.event_data.{target}"), description);
        }
    }

    let code = event.get_as_string("event.code").unwrap_or_default();
    if !SDDL_CODES.contains(&code.as_str()) {
        return true;
    }

    let mut related = Vec::new();
    for prefix in ["OldSd", "NewSd"] {
        let path = format!("winlog.event_data.{prefix}");
        let Some(sddl) = event.get_str(&path).map(str::to_string) else {
            continue;
        };
        enrich(event, &sddl, prefix, &tables, &mut related);
    }

    if let Some(sids) = event
        .get_str("winlog.event_data.SidList")
        .map(str::to_string)
    {
        split_sid_list(event, &sids, &tables);
    }

    for grantee in related {
        let _ = event.append_unique("related.user", Value::String(grantee));
    }
    true
}

/// One descriptor: owner, group, then each DACL and SACL entry in turn.
fn enrich(
    event: &mut Event,
    sddl: &str,
    prefix: &str,
    tables: &Tables<'_>,
    related: &mut Vec<String>,
) {
    // `^O:[A-Z]{2}` and `^G:[A-Z]{2}` -- anchored, and the capture keeps the
    // marker, which is the vendor's own reading.
    for (marker, suffix) in [("O:", "Owner"), ("G:", "Group")] {
        if let Some(rest) = sddl.strip_prefix(marker)
            && rest.len() >= 2
            && rest[..2].chars().all(|c| c.is_ascii_uppercase())
        {
            let captured = format!("{marker}{}", &rest[..2]);
            let translated = translate_sid(&captured, tables);
            let _ = event.set(
                &format!("winlog.event_data.{prefix}{suffix}"),
                Value::String(translated.unwrap_or_else(|| "null".to_string())),
            );
        }
    }

    for (marker, suffix) in [("D:", "Dacl"), ("S:", "Sacl")] {
        let Some(at) = sddl.find(marker) else {
            continue;
        };
        for (index, ace) in aces(&sddl[at..]).enumerate() {
            let (grantee, line) = render_ace(ace, tables);
            let _ = event.set(
                &format!("winlog.event_data.{prefix}{suffix}{index}"),
                Value::String(line),
            );
            if let Some(grantee) = grantee
                && RELATED_GRANTEES.contains(&grantee.as_str())
                && !related.contains(&grantee)
            {
                related.push(grantee);
            }
        }
    }
}

/// Every `(...)` group, with the parentheses stripped.
///
/// `\([^*\)]*\)` -- so a group holding a `)` of its own ends at that `)`,
/// which is what the vendor's pattern does.
fn aces(section: &str) -> impl Iterator<Item = &str> {
    section.split('(').skip(1).filter_map(|rest| {
        let ace = rest.split(')').next()?;
        (!ace.is_empty()).then_some(ace)
    })
}

/// `<grantee> :<type> ([<perm>, <perm>])`, and the grantee for `related.user`.
fn render_ace(ace: &str, tables: &Tables<'_>) -> (Option<String>, String) {
    let fields: Vec<&str> = ace.split(';').collect();

    let grantee = fields.get(5).and_then(|sid| translate_sid(sid, tables));
    let kind = fields
        .first()
        .and_then(|kind| tables.ace_types?.get(*kind)?.as_str())
        .map(str::to_string);
    let perms = fields.get(2).map(|mask| permissions(mask, tables));

    let line = format!(
        "{} :{} ([{}])",
        grantee.clone().unwrap_or_else(|| "null".to_string()),
        kind.unwrap_or_else(|| "null".to_string()),
        perms.unwrap_or_default().join(", "),
    );
    (grantee, line)
}

/// The permission names an access mask spells.
///
/// A `0x`-prefixed mask is tested against the numeric flag table in the
/// table's own order; anything else is read two letters at a time.
fn permissions(mask: &str, tables: &Tables<'_>) -> Vec<String> {
    if let Some(hex) = mask.strip_prefix("0x") {
        let Ok(code) = u64::from_str_radix(hex, 16) else {
            return vec![mask.to_string()];
        };
        let mut named = Vec::new();
        if let Some(flags) = tables.permission_flags {
            for (key, description) in flags {
                let Some(flag) = key
                    .strip_prefix("0x")
                    .and_then(|hex| u64::from_str_radix(hex, 16).ok())
                else {
                    continue;
                };
                if flag != 0 && code & flag == flag {
                    named.push(
                        description
                            .as_str()
                            .map_or_else(|| "null".to_string(), str::to_string),
                    );
                }
            }
        }
        if named.is_empty() {
            named.push(mask.to_string());
        }
        return named;
    }

    // `.{1,2}` -- pairs, and a lone trailing character is a group of one.
    let letters: Vec<char> = mask.chars().collect();
    letters
        .chunks(2)
        .map(|pair| {
            let key: String = pair.iter().collect();
            tables
                .permissions
                .and_then(|table| table.get(&key))
                .and_then(Value::as_str)
                .map_or_else(|| "null".to_string(), str::to_string)
        })
        .collect()
}

/// A SID's description, or the SID itself where the tables do not name it.
fn translate_sid(sid: &str, tables: &Tables<'_>) -> Option<String> {
    let sid = sid.trim();
    if let Some(table) = tables.sids
        && table.contains_key(sid)
    {
        return table.get(sid).and_then(Value::as_str).map(str::to_string);
    }
    if !sid.starts_with("S-1-5-21") {
        return Some(sid.to_string());
    }
    // `[0-9]{1,5}$`: the trailing digits, at most five of them, so a long
    // trailing number is truncated to its last five rather than skipped.
    let trailing = sid.len() - sid.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    if trailing == 0 {
        return Some(sid.to_string());
    }
    let rid = &sid[sid.len() - trailing.min(5)..];
    tables
        .domain_sids
        .and_then(|table| table.get(rid))
        .and_then(Value::as_str)
        .map(str::to_string)
}

/// `SidList` split on spaces, and each entry's description beside it.
fn split_sid_list(event: &mut Event, sids: &str, tables: &Tables<'_>) {
    let entries: Vec<&str> = sids.split(' ').collect();
    let described: Vec<Value> = entries
        .iter()
        .map(|entry| {
            let cleaned: String = entry
                .chars()
                .filter(|c| !matches!(c, '%' | '{' | '}' | ' '))
                .collect();
            Value::String(translate_sid(&cleaned, tables).unwrap_or_else(|| "null".to_string()))
        })
        .collect();

    let _ = event.set(
        "winlog.event_data.SidList",
        Value::Array(
            entries
                .into_iter()
                .map(|s| Value::String(s.to_string()))
                .collect(),
        ),
    );
    let _ = event.set("winlog.event_data.SidListDesc", Value::Array(described));
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use serde_json::json;

    /// The tables the script ships, trimmed to what these events use.
    fn params() -> Map<String, Value> {
        json!({
            "AccountSIDDescription": {
                "SY": "Local system",
                "NS": "Network service account",
                "WD": "Everyone",
            },
            "DomainSpecificSID": { "500": "Administrator" },
            "AceTypes": { "A": "Access Allowed", "AU": "System Audit" },
            "PermissionDescription": {
                "GA": "Generic All",
                "RC": "Read Permissions",
                "DC": "Delete All Child Objects",
                "LC": "List Contents",
                "RP": "Read All Properties",
                "CR": "All Extended Rights",
                "SD": "Delete",
                "WD": "Modify Permissions",
                "WO": "Modify Owner",
            },
            "PermsFlags": { "0x10000000": "Generic All", "0x00020000": "Read Control" },
        })
        .as_object()
        .expect("the tables are an object")
        .clone()
    }

    /// Verbatim from the 4670 corpus capture. `OW` is not in the SID table, so
    /// it renders as itself rather than being invented.
    #[test]
    fn a_dacl_renders_one_line_per_entry() {
        let mut event = Event::new(json!({
            "event": { "code": 4670 },
            "winlog": { "event_data": {
                "NewSd": "D:(A;;GA;;;SY)(A;;RC;;;OW)(A;;GA;;;S-1-5-80-123231216)",
                "OldSd": "D:(A;;GA;;;SY)(A;;GA;;;NS)",
            }},
        }));

        assert!(run(&mut event, &params()));

        let data = "winlog.event_data";
        assert_eq!(
            event.get_str(&format!("{data}.NewSdDacl0")),
            Some("Local system :Access Allowed ([Generic All])")
        );
        assert_eq!(
            event.get_str(&format!("{data}.NewSdDacl1")),
            Some("OW :Access Allowed ([Read Permissions])")
        );
        assert_eq!(
            event.get_str(&format!("{data}.NewSdDacl2")),
            Some("S-1-5-80-123231216 :Access Allowed ([Generic All])")
        );
        assert_eq!(
            event.get_str(&format!("{data}.OldSdDacl1")),
            Some("Network service account :Access Allowed ([Generic All])")
        );
    }

    /// Verbatim from the 4907 capture: a SACL, and a permission string read
    /// two letters at a time.
    #[test]
    fn a_sacl_reads_its_permissions_in_pairs() {
        let mut event = Event::new(json!({
            "event": { "code": "4907" },
            "winlog": { "event_data": { "NewSd": "S:ARAI(AU;SAFA;DCLCRPCRSDWDWO;;;WD)" }},
        }));

        assert!(run(&mut event, &params()));
        assert_eq!(
            event.get_str("winlog.event_data.NewSdSacl0"),
            Some(
                "Everyone :System Audit ([Delete All Child Objects, List Contents, \
                 Read All Properties, All Extended Rights, Delete, Modify Permissions, \
                 Modify Owner])"
            )
        );
    }

    /// Verbatim from the 4817 capture. A domain SID resolves through its RID,
    /// an unlisted RID renders as the literal `null` Java concatenates, and a
    /// named grantee joins `related.user`.
    #[test]
    fn a_domain_sid_resolves_through_its_rid() {
        let mut event = Event::new(json!({
            "event": { "code": 4817 },
            "winlog": { "event_data": { "NewSd":
                "S:(AU;SA;GA;;;S-1-5-21-2024912787-2692429404-2351956786-500)(AU;SA;GA;;;S-1-5-21-2024912787-2692429404-2351956786-1000)" }},
        }));

        assert!(run(&mut event, &params()));
        assert_eq!(
            event.get_str("winlog.event_data.NewSdSacl0"),
            Some("Administrator :System Audit ([Generic All])")
        );
        assert_eq!(
            event.get_str("winlog.event_data.NewSdSacl1"),
            Some("null :System Audit ([Generic All])")
        );
        assert_eq!(event.get("related.user"), Some(&json!(["Administrator"])));
    }

    /// An event code the script does not name leaves the descriptor alone,
    /// and the two remote-id descriptions still run.
    #[test]
    fn another_event_code_expands_nothing() {
        let mut event = Event::new(json!({
            "event": { "code": 4624 },
            "winlog": { "event_data": {
                "NewSd": "D:(A;;GA;;;SY)",
                "RemoteUserID": "WD",
            }},
        }));

        assert!(run(&mut event, &params()));
        assert!(!event.has("winlog.event_data.NewSdDacl0"));
        assert_eq!(
            event.get_str("winlog.event_data.RemoteUserDescription"),
            Some("Everyone")
        );
    }

    /// A numeric access mask is tested against the flag table instead.
    #[test]
    fn a_hex_mask_reads_the_flag_table() {
        let mut event = Event::new(json!({
            "event": { "code": 4670 },
            "winlog": { "event_data": { "NewSd": "D:(A;;0x10020000;;;SY)" }},
        }));

        assert!(run(&mut event, &params()));
        assert_eq!(
            event.get_str("winlog.event_data.NewSdDacl0"),
            Some("Local system :Access Allowed ([Generic All, Read Control])")
        );
    }
}
