// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One member taken from the LAST element of a list, into a map the script
//! creates.
//!
//! `lyve_cloud`'s audit stream reaches the client's own address through a chain
//! of proxies. `related.ip` holds every hop in order and the vendor's comment
//! says which one matters -- "setting client's ip as the last element of the
//! ' related.ip ' field ( the rest are proxies)":
//!
//! ```painless
//! ctx.client = new HashMap();
//! ctx.client["ip"] = ctx.related.ip[-1];
//! ```
//!
//! Unmatched it costs the whole source: `client.ip` and `source.ip` on all 10
//! events, with nothing extra, because `source` is a later
//! `copy_from: client` and the two geoip processors read `client.ip` and find
//! nothing.
//!
//! **The map creation is required, not incidental.** Painless will not create
//! a parent, so a script assigning into `ctx.client` without building it first
//! is a different script that throws, and claiming it would report a success
//! Elasticsearch does not have.
//!
//! This is deliberately NOT part of `FirstElement`. That reads `[0];` and
//! would need a last-element flag, a quoted-subscript target and a parent
//! creation to cover this -- and it has already claimed eight scripts it could
//! not write by reading a closing bracket alone, which is what
//! `collect_present` exists to undo.

use serde_json::Value;

use dfe_core::Event;

use crate::params::clean_path;

/// A member assigned from a list's last element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastElementMember {
    /// The map the script creates and writes into.
    parent: String,
    /// The member key, which the vendor writes as a quoted subscript.
    key: String,
    /// The list the value comes from.
    array: String,
}

/// Strip the quoting a subscript key carries, escaped or not.
///
/// A stored script arrives with its quotes escaped, so the key reads `\"ip\"`
/// rather than `"ip"`.
fn unquote(raw: &str) -> &str {
    raw.trim()
        .trim_matches(|c| c == '"' || c == '\'' || c == '\\')
}

/// Read the take, or decline it.
///
/// Exactly one `[-1];` is allowed: a script taking two last elements writes two
/// members, and reading only the first would leave the other missing with
/// nothing to show for it.
#[must_use]
pub fn parse_last_element_member(script: &str) -> Option<LastElementMember> {
    if script.matches("[-1];").count() != 1 {
        return None;
    }
    let at = script.find("[-1];")?;

    // `= ctx.<array>[-1];`
    let (lhs, _) = script.split_at(at);
    let (before, array) = lhs.rsplit_once("= ctx.")?;
    let array = clean_path(array.trim());
    if array.is_empty() {
        return None;
    }

    // `ctx.<parent>[<quoted key>]`
    let subscript = before.trim().strip_suffix(']')?;
    let (target, key) = subscript.rsplit_once('[')?;
    let key = unquote(key);
    if key.is_empty() || !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let parent = clean_path(target.trim().rsplit("ctx.").next()?.trim());
    if parent.is_empty() {
        return None;
    }

    // The script has to BUILD the map it writes into; Painless will not.
    let built = format!("ctx.{parent} = new HashMap()");
    if !script.contains(&built) {
        return None;
    }

    Some(LastElementMember {
        parent,
        key: key.to_owned(),
        array,
    })
}

/// Write the member, or leave the document alone.
pub fn run_last_element_member(event: &mut Event, pattern: &LastElementMember) -> bool {
    let Some(Value::Array(items)) = event.get(&pattern.array) else {
        return false;
    };
    let Some(last) = items.last().cloned() else {
        return false;
    };
    let target = format!("{}.{}", pattern.parent, pattern.key);
    event.set(&target, last).is_ok()
}

#[cfg(test)]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`. Keeping them character-identical is what lets a script
// be copied straight from a module into a test.
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from `lyve_cloud_audit/audit_lc.rs`, escapes and all.
    const LYVE_CLOUD: &str =
        r#"ctx.client = new HashMap(); ctx.client[\"ip\"] = ctx.related.ip[-1];"#;

    #[test]
    fn the_last_hop_is_the_client_and_the_rest_are_proxies() {
        let pattern = parse_last_element_member(LYVE_CLOUD).expect("declined lyve_cloud's take");
        let mut event = Event::new(json!({
            "related": { "ip": ["10.0.0.1", "203.0.113.7", "81.2.69.144"] },
        }));
        assert!(run_last_element_member(&mut event, &pattern));
        assert_eq!(event.get("client.ip"), Some(&json!("81.2.69.144")));
    }

    #[test]
    fn a_script_that_does_not_build_its_map_is_declined() {
        let script = r#"ctx.client[\"ip\"] = ctx.related.ip[-1];"#;
        assert!(parse_last_element_member(script).is_none());
    }

    #[test]
    fn two_takes_are_declined_rather_than_half_written() {
        let script = r#"ctx.client = new HashMap(); ctx.client[\"ip\"] = ctx.related.ip[-1]; ctx.client[\"port\"] = ctx.related.port[-1];"#;
        assert!(parse_last_element_member(script).is_none());
    }

    #[test]
    fn a_first_element_take_is_left_to_its_own_reader() {
        let script = r#"ctx.client = new HashMap(); ctx.client[\"ip\"] = ctx.related.ip[0];"#;
        assert!(parse_last_element_member(script).is_none());
    }

    #[test]
    fn an_empty_list_writes_nothing() {
        let pattern = parse_last_element_member(LYVE_CLOUD).expect("declined lyve_cloud's take");
        let mut event = Event::new(json!({ "related": { "ip": [] } }));
        assert!(!run_last_element_member(&mut event, &pattern));
        assert_eq!(event.get("client.ip"), None);
    }
}
