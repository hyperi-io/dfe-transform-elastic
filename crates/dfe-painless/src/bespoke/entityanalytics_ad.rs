// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `entityanalytics_ad`'s entity scripts, transcribed.
//!
//! Active Directory hands its identifiers over as base64 of the raw LDAP
//! attribute, so `objectSid` and `objectGUID` arrive as bytes rather than the
//! `S-1-5-...` and dashed-hex spellings anything downstream reads. The rename
//! walk decodes them on the way past, and the rest of the module is the group,
//! account-control and distinguished-name work that hangs off the decoded
//! values.

use std::collections::HashSet;

use base64::Engine as _;
use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::{java_bucket, java_string_set_order, java_table_size};

/// The subtree every one of these pipelines works over.
const AD: &str = "activedirectory";

/// `entityanalytics_ad/entity`, the `common` pipeline's rename script:
/// `ctx.activedirectory` rebuilt with the vendor's LDAP attribute names
/// mapped to their snake-case spellings, `objectSid` and `objectGUID` decoded
/// on the way.
///
/// The walk reproduces one asymmetry in the vendor's script: a LIST under a
/// key the map does NOT rename keeps its ORIGINAL elements, so the rewritten
/// members are thrown away for that key alone.
fn rename_keys(event: &mut Event, params: &Value) {
    let Some(key_map) = params.as_object() else {
        return;
    };
    let Some(source) = event.get_object(AD).cloned() else {
        return;
    };
    let renamed = rename_map(&source, key_map);
    let _ = event.set(AD, Value::Object(renamed));
}

/// One map through the rename, recursively.
fn rename_map(src: &Map<String, Value>, key_map: &Map<String, Value>) -> Map<String, Value> {
    let mut dst = Map::with_capacity(src.len());
    for (key, value) in src {
        let renamed = key_map.get(key).and_then(Value::as_str);
        match value {
            Value::Object(nested) => {
                let walked = Value::Object(rename_map(nested, key_map));
                dst.insert(renamed.unwrap_or(key).to_owned(), walked);
            }
            Value::Array(items) => {
                // The vendor's script builds the rewritten list either way and
                // then keeps it only where the key itself was renamed.
                match renamed {
                    Some(target) => {
                        let walked: Vec<Value> = items
                            .iter()
                            .map(|item| match item {
                                Value::Object(nested) => Value::Object(rename_map(nested, key_map)),
                                other => other.clone(),
                            })
                            .collect();
                        dst.insert(target.to_owned(), Value::Array(walked));
                    }
                    None => {
                        dst.insert(key.clone(), value.clone());
                    }
                }
            }
            other => {
                let decoded = match (other.as_str(), key.as_str()) {
                    (Some(text), "objectGUID") => {
                        guid(text).map_or_else(|| other.clone(), Value::from)
                    }
                    (Some(text), "objectSid") => {
                        sid(text).map_or_else(|| other.clone(), Value::from)
                    }
                    _ => other.clone(),
                };
                dst.insert(renamed.unwrap_or(key).to_owned(), decoded);
            }
        }
    }
    dst
}

/// The dashed-hex GUID the script builds out of the raw 16 bytes.
///
/// The first three groups are byte-reversed, which is how a Windows GUID is
/// stored, and the remainder is read straight through with a dash after the
/// second byte.
fn guid(text: &str) -> Option<String> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(text)
        .ok()?;
    if bytes.len() < 11 {
        return None;
    }
    let mut uid = String::with_capacity(36);
    for index in (0..4).rev() {
        hex_byte(&mut uid, bytes[index]);
    }
    uid.push('-');
    for index in (4..6).rev() {
        hex_byte(&mut uid, bytes[index]);
    }
    uid.push('-');
    for index in (6..8).rev() {
        hex_byte(&mut uid, bytes[index]);
    }
    uid.push('-');
    for (index, byte) in bytes.iter().enumerate().skip(8) {
        if index == 10 {
            uid.push('-');
        }
        hex_byte(&mut uid, *byte);
    }
    Some(uid)
}

/// One byte as two lowercase hex digits.
fn hex_byte(out: &mut String, byte: u8) {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    out.push(char::from(DIGITS[usize::from(byte >> 4)]));
    out.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
}

/// The `S-1-5-...` SID the script builds out of the raw bytes.
///
/// The authority is assembled into a 32-BIT int, so the two shifts the script
/// asks for past bit 31 wrap the way Java's `<<` does -- byte 2 lands back on
/// bits 8-15 and byte 3 on bits 0-7. Every real SID carries zeros there, so
/// the wrap is invisible, but the arithmetic is the script's and is kept.
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)] // Java's own int arithmetic.
fn sid(text: &str) -> Option<String> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(text)
        .ok()?;
    if bytes.len() < 8 {
        return None;
    }
    let mut uid = format!("S-{}-", bytes[0] as i8);
    let mut auth: i32 = 0;
    for (offset, byte) in bytes[2..8].iter().enumerate() {
        let shift = (8 * (5 - offset)) & 31;
        auth |= (i32::from(*byte)) << shift;
    }
    uid.push_str(&auth.to_string());

    let subauths = usize::from(bytes[1]);
    let mut at = 8usize;
    for _ in 0..subauths {
        if at + 4 > bytes.len() {
            return None;
        }
        let mut subauth: u32 = 0;
        for step in 0..4 {
            subauth |= u32::from(bytes[at + step]) << (8 * step);
        }
        uid.push('-');
        uid.push_str(&subauth.to_string());
        at += 4;
    }
    Some(uid)
}

/// `entityanalytics_ad/entity`, the user pipeline's `uac_list` script: the
/// account-control bitmask spelled out as its flag names.
fn uac_list_user(event: &mut Event, params: &Value) {
    uac_list(event, params, "activedirectory.user");
}

/// `entityanalytics_ad/entity`, the device pipeline's `uac_list` script: the
/// same bitmask, under the device subtree.
fn uac_list_device(event: &mut Event, params: &Value) {
    uac_list(event, params, "activedirectory.device");
}

/// The flag names a `userAccountControl` value carries, in the order a Java
/// `HashMap` over the params hands them back.
fn uac_list(event: &mut Event, params: &Value, subtree: &str) {
    let Some(flags) = params.as_object() else {
        return;
    };
    let Some(raw) = event.get(&format!("{subtree}.user_account_control")) else {
        return;
    };
    let Some(value) = java_decode_value(raw) else {
        return;
    };

    let table = java_table_size(flags.len());
    let mut entries: Vec<(usize, usize, &String, &Value)> = flags
        .iter()
        .enumerate()
        .map(|(position, (key, name))| (java_bucket(key, table), position, key, name))
        .collect();
    entries.sort_by_key(|(bucket, position, ..)| (*bucket, *position));

    let named: Vec<Value> = entries
        .into_iter()
        .filter(|(_, _, key, _)| java_decode(key).is_some_and(|flag| value & flag != 0))
        .map(|(_, _, _, name)| name.clone())
        .collect();
    if named.is_empty() {
        return;
    }
    let _ = event.set(&format!("{subtree}.uac_list"), Value::Array(named));
}

/// `Long.decode` over whatever the document holds at the path.
fn java_decode_value(raw: &Value) -> Option<i64> {
    match raw {
        Value::String(text) => java_decode(text),
        Value::Number(number) => number.as_i64(),
        _ => None,
    }
}

/// `Long.decode`: an optional sign, then `0x`, `#` or a leading `0` choosing
/// the radix, and decimal otherwise.
fn java_decode(text: &str) -> Option<i64> {
    let (negative, body) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text.strip_prefix('+').unwrap_or(text)),
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
    let magnitude = i64::from_str_radix(digits, radix).ok()?;
    Some(if negative { -magnitude } else { magnitude })
}

/// `entityanalytics_ad/entity`, the user pipeline's `handle_user_group_details`:
/// the group names and SIDs a user belongs to, and whether any of them is one
/// of the privileged well-known RIDs.
fn user_group_details_user(event: &mut Event, params: &Value) {
    user_group_details(event, params, "activedirectory.user");
}

/// `entityanalytics_ad/entity`, the device pipeline's `handle_user_group_details`:
/// the same collection, with the privileged flag written under the device.
fn user_group_details_device(event: &mut Event, params: &Value) {
    user_group_details(event, params, "activedirectory.device");
}

/// The group walk itself.
///
/// `ctx.user.group.name` and `.id` are Java `HashSet`s, so the members come
/// out in bucket order rather than the order the groups were read in.
fn user_group_details(event: &mut Event, params: &Value, subtree: &str) {
    let Some(groups) = event.get_array("activedirectory.groups").cloned() else {
        return;
    };

    let mut names: Vec<String> = Vec::with_capacity(groups.len());
    let mut ids: Vec<String> = Vec::with_capacity(groups.len());
    let mut seen_names: HashSet<String> = HashSet::new();
    let mut seen_ids: HashSet<String> = HashSet::new();
    let mut privileged: Option<Value> = None;
    let mut stamped: Vec<Value> = Vec::with_capacity(groups.len());

    for group in &groups {
        let mut group = group.clone();
        if let Some(name) = group.get("name").and_then(Value::as_str)
            && seen_names.insert(name.to_owned())
        {
            names.push(name.to_owned());
        }
        let Some(object_sid) = group
            .get("object_sid")
            .and_then(Value::as_str)
            .map(str::to_owned)
        else {
            stamped.push(group);
            continue;
        };
        // The script stamps the SID back onto the group as `id`, which the
        // rename downstream carries into the shipped document.
        if let Value::Object(map) = &mut group {
            map.insert("id".to_owned(), Value::from(object_sid.clone()));
        }
        stamped.push(group);
        if seen_ids.insert(object_sid.clone()) {
            ids.push(object_sid.clone());
        }
        let Some(at) = object_sid.rfind('-') else {
            continue;
        };
        if privileged.is_none()
            && let Some(flag) = params.get(&object_sid[at + 1..])
        {
            privileged = Some(flag.clone());
        }
    }

    let _ = event.set("activedirectory.groups", Value::Array(stamped));
    if !names.is_empty() {
        let _ = event.set(
            "user.group.name",
            Value::Array(java_string_set_order(names)),
        );
    }
    if !ids.is_empty() {
        let _ = event.set("user.group.id", Value::Array(java_string_set_order(ids)));
    }
    let member = privileged.unwrap_or(Value::Bool(false));
    // Painless raises on the assignment where the parent map is absent.
    if event.has(subtree) {
        let _ = event.set(&format!("{subtree}.privileged_group_member"), member);
    }
}

/// `entityanalytics_ad/entity`, `set_user_relationships`: the entity
/// relationships a user's `managedObjects` and `directReports` spell out.
fn user_relationships(event: &mut Event, _params: &Value) {
    relationships(event, "activedirectory.user", "user");
}

/// `entityanalytics_ad/entity`, `set_device_relationships`: the same, for a
/// device.
fn device_relationships(event: &mut Event, _params: &Value) {
    relationships(event, "activedirectory.device", "host");
}

/// The two relationship builds, which differ only in whose subtree they read
/// and which ECS entity they write under.
fn relationships(event: &mut Event, subtree: &str, entity: &str) {
    let managed = event.get(&format!("{subtree}.managed_objects")).cloned();
    let reports = event.get(&format!("{subtree}.direct_reports")).cloned();
    if !event.has(&format!("{entity}.entity.relationships")) {
        let _ = event.set(
            &format!("{entity}.entity.relationships"),
            Value::Object(Map::new()),
        );
    }
    if let Some(dns) = managed.filter(|value| !value.is_null()) {
        let _ = event.set(
            &format!("{entity}.entity.relationships.administers"),
            build_host_rel(&dns),
        );
    }
    if let Some(dns) = reports.filter(|value| !value.is_null()) {
        let _ = event.set(
            &format!("{entity}.entity.relationships.supervises"),
            build_user_rel(&dns),
        );
    }
}

/// One distinguished name split into its id, its first `CN=` and the domain
/// its `DC=` parts spell.
///
/// A comma escaped with a backslash is part of the value, not a separator.
fn parse_dn(dn: &str) -> (String, Option<String>, Option<String>) {
    let bytes = dn.as_bytes();
    let mut name: Option<String> = None;
    let mut domain_parts: Vec<&str> = Vec::new();
    let mut start = 0usize;
    while start < bytes.len() {
        let mut end = start;
        loop {
            match dn[end..].find(',') {
                None => {
                    end = bytes.len();
                    break;
                }
                Some(at) => {
                    end += at;
                    if end > 0 && bytes[end - 1] == b'\\' {
                        end += 1;
                        continue;
                    }
                    break;
                }
            }
        }
        let part = dn[start..end].replace("\\,", ",");
        let part = part.trim();
        if name.is_none() && part.len() > 3 && part[..3].eq_ignore_ascii_case("CN=") {
            name = Some(part[3..].to_owned());
        } else if part.len() > 3 && part[..3].eq_ignore_ascii_case("DC=") {
            domain_parts.push(&dn[start + 3..end]);
        }
        start = end + 1;
    }
    let domain = if domain_parts.is_empty() {
        None
    } else {
        Some(domain_parts.join("."))
    };
    (dn.to_owned(), name, domain)
}

/// Whatever the field holds, as the list of distinguished names the script
/// walks.
fn dn_list(dns: &Value) -> Vec<&str> {
    match dns {
        Value::Array(items) => items.iter().filter_map(Value::as_str).collect(),
        Value::String(text) => vec![text.as_str()],
        _ => Vec::new(),
    }
}

/// The `administers` relationship: each name becomes a host, with the domain
/// carried alongside as a user domain.
fn build_host_rel(dns: &Value) -> Value {
    let mut ids: Vec<Value> = Vec::new();
    let mut names: Vec<Value> = Vec::new();
    let mut domains: Vec<Value> = Vec::new();
    for dn in dn_list(dns) {
        if dn.is_empty() {
            continue;
        }
        let (id, name, domain) = parse_dn(dn);
        ids.push(Value::from(id));
        if let Some(name) = name {
            let fqdn = match &domain {
                Some(domain) => format!("{}.{domain}", name.to_lowercase()),
                None => name,
            };
            names.push(Value::from(fqdn));
        }
        if let Some(domain) = domain {
            domains.push(Value::from(domain));
        }
    }

    let mut host = Map::new();
    if !ids.is_empty() {
        host.insert("id".to_owned(), Value::Array(ids));
    }
    if !names.is_empty() {
        host.insert("name".to_owned(), Value::Array(names));
    }
    let mut rel = Map::new();
    rel.insert("host".to_owned(), Value::Object(host));
    if !domains.is_empty() {
        let mut user = Map::new();
        user.insert("domain".to_owned(), Value::Array(domains));
        rel.insert("user".to_owned(), Value::Object(user));
    }
    Value::Object(rel)
}

/// The `supervises` relationship: each name becomes a user, keeping the `CN=`
/// as written rather than lowercasing it into an FQDN.
fn build_user_rel(dns: &Value) -> Value {
    let mut ids: Vec<Value> = Vec::new();
    let mut names: Vec<Value> = Vec::new();
    let mut domains: Vec<Value> = Vec::new();
    for dn in dn_list(dns) {
        if dn.is_empty() {
            continue;
        }
        let (id, name, domain) = parse_dn(dn);
        ids.push(Value::from(id));
        if let Some(name) = name {
            names.push(Value::from(name));
        }
        if let Some(domain) = domain {
            domains.push(Value::from(domain));
        }
    }

    let mut user = Map::new();
    if !ids.is_empty() {
        user.insert("id".to_owned(), Value::Array(ids));
    }
    if !names.is_empty() {
        user.insert("name".to_owned(), Value::Array(names));
    }
    if !domains.is_empty() {
        user.insert("domain".to_owned(), Value::Array(domains));
    }
    let mut rel = Map::new();
    rel.insert("user".to_owned(), Value::Object(user));
    Value::Object(rel)
}

/// Every `entityanalytics_ad` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "1fc6614a979b8062cde1cda5e56bfa3cb14dbee54d9fe48dd301265e6054455f",
        source: "entityanalytics_ad",
        name: "rename_keys",
        run: rename_keys,
    },
    Entry {
        hash: "554eb2db69e5149aeab09babf2d27299a2255e2c1125da8914026f7db71ca733",
        source: "entityanalytics_ad",
        name: "uac_list_user",
        run: uac_list_user,
    },
    Entry {
        hash: "68485626173ada45c0d167f6e2564bd412da486197370cdcd7722e08defe7089",
        source: "entityanalytics_ad",
        name: "uac_list_device",
        run: uac_list_device,
    },
    Entry {
        hash: "0601159c806d560f62cbd5e3024e28e8ba8328ee0a156b1049c7e415f380df80",
        source: "entityanalytics_ad",
        name: "user_group_details_user",
        run: user_group_details_user,
    },
    Entry {
        hash: "48d486ab13787b38a61fadbd89731f5e4fe7010eb5440db5bdcdae0507a8fbd5",
        source: "entityanalytics_ad",
        name: "user_group_details_device",
        run: user_group_details_device,
    },
    Entry {
        hash: "c54b89496f23087ae27acb2f5693a5cb509db4c6cf9cbf9aa7890bc32519ad2c",
        source: "entityanalytics_ad",
        name: "user_relationships",
        run: user_relationships,
    },
    Entry {
        hash: "0b58725d27662d90f95ee8f9d2d426ac93fa0ac7406f537c911581eb7be1ead6",
        source: "entityanalytics_ad",
        name: "device_relationships",
        run: device_relationships,
    },
];
