// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `akamai`'s SIEM scripts: the base64 rule columns folded row-wise into one
//! list of rules, and the pipe-delimited user-risk string parsed into a map.
//!
//! The vendor sends each rule attribute as its own `;`-delimited column of
//! base64, so the Nth cell of every column belongs to the Nth rule. A cell that
//! will not decode keeps the JDK's own complaint as its value, which is why the
//! decoder here reproduces `java.util.Base64`'s messages rather than reporting
//! its own: they are part of the vendor's output.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// Where the rule columns live.
const ATTACK_DATA: &str = "json.attackData";

/// The column whose length decides how many rules there are.
const RULES_COLUMN: &str = "json.attackData.rules";

/// `script_base64_decode_attackData_rule` in `akamai/siem`:
/// `akamai.siem.rules`, plus the `_rule_actions` and `_rule_tags` lists the
/// two `foreach` appends downstream read.
///
/// The row count is the `rules` column's, and a column shorter than that
/// contributes nothing to the rows past its end.
fn fold_attack_rules(event: &mut Event, params: &Value) {
    let Some(items) = params.get("items").and_then(Value::as_array) else {
        return;
    };
    let columns: Vec<(&str, String)> = items
        .iter()
        .filter_map(Value::as_str)
        .map(|key| (key, format!("{ATTACK_DATA}.{key}")))
        .collect();
    let Some(rows) = event.get_array(RULES_COLUMN).map(Vec::len) else {
        return;
    };

    let mut rules = Vec::with_capacity(rows);
    let mut actions = Vec::new();
    let mut tags = Vec::new();
    for row in 0..rows {
        let mut mapped = Map::new();
        for (key, path) in &columns {
            let Some(cell) = event.get_array(path).and_then(|column| column.get(row)) else {
                continue;
            };
            let data = painless_to_string(cell).replace(' ', "");
            match decode_base64(&data) {
                Ok(value) => {
                    if *key == "ruleTags" {
                        tags.push(Value::from(value.to_lowercase()));
                    } else if *key == "ruleActions" {
                        actions.push(Value::from(value.to_lowercase()));
                    }
                    mapped.insert((*key).to_owned(), Value::from(value));
                }
                Err(complaint) => {
                    mapped.insert((*key).to_owned(), Value::from(warning(&complaint, &data)));
                }
            }
        }
        rules.push(Value::Object(mapped));
    }

    // Painless raises on the assignment where the parent map is absent, and
    // the two `_rule_*` lists are only written once the rules are.
    if !event.has("akamai.siem") {
        return;
    }
    let _ = event.set("akamai.siem.rules", Value::Array(rules));
    let _ = event.set("_rule_actions", Value::Array(actions));
    let _ = event.set("_rule_tags", Value::Array(tags));
}

/// The value a failed decode leaves behind, the script's own truncation
/// included -- anything past ten characters is cut and marked.
fn warning(complaint: &str, data: &str) -> String {
    let shown = if data.chars().count() > 10 {
        let mut cut: String = data.chars().take(10).collect();
        cut.push_str("...");
        cut
    } else {
        data.to_owned()
    };
    format!("failed to decode base64 data: {complaint}: {shown}")
}

/// Decode base64 the way `java.util.Base64.getDecoder()` does, complaint and
/// all.
///
/// Painless `decodeBase64` is that decoder, and the script stores the message
/// it throws, so an implementation that merely declines the same inputs is not
/// enough -- the text has to match. Undecodable BYTES are a separate matter:
/// Java builds the string with a replacing decoder, which is what
/// `from_utf8_lossy` does.
#[allow(clippy::cast_possible_truncation)] // Java's own byte assembly.
fn decode_base64(text: &str) -> Result<String, String> {
    let src = text.as_bytes();
    let end = src.len();
    if end == 0 {
        return Ok(String::new());
    }
    if end < 2 {
        return Err("Input byte[] should at least have 2 bytes for base64 bytes".to_owned());
    }

    let mut out: Vec<u8> = Vec::with_capacity(end / 4 * 3 + 3);
    let mut at = 0usize;
    let mut bits: u32 = 0;
    let mut shift: i32 = 18;
    while at < end {
        let byte = src[at];
        at += 1;
        let Some(sextet) = sextet(byte) else {
            if byte != b'=' {
                return Err(format!("Illegal base64 character {}", java_hex(byte)));
            }
            // A padding byte is only legal as the last unit's tail: `xx=` with
            // the second `=` consumed here, or `xxx=` on its own.
            let dangling = shift == 6 && {
                let tail = at == end || src[at] != b'=';
                at += usize::from(at < end);
                tail
            };
            if dangling || shift == 18 {
                return Err("Input byte array has wrong 4-byte ending unit".to_owned());
            }
            break;
        };
        bits |= u32::from(sextet) << shift;
        shift -= 6;
        if shift < 0 {
            out.push((bits >> 16) as u8);
            out.push((bits >> 8) as u8);
            out.push(bits as u8);
            shift = 18;
            bits = 0;
        }
    }

    if shift == 6 {
        out.push((bits >> 16) as u8);
    } else if shift == 0 {
        out.push((bits >> 16) as u8);
        out.push((bits >> 8) as u8);
    } else if shift == 12 {
        return Err("Last unit does not have enough valid bits".to_owned());
    }
    if at < end {
        return Err(format!(
            "Input byte array has incorrect ending byte at {at}"
        ));
    }
    Ok(String::from_utf8_lossy(&out).into_owned())
}

/// The six bits a base64 character stands for, or nothing at all.
fn sextet(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

/// One byte as `Integer.toString(b, 16)` writes it, where `b` came out of a
/// Java `byte[]` and is therefore SIGNED.
#[allow(clippy::cast_possible_wrap)] // The sign is the point.
fn java_hex(byte: u8) -> String {
    let signed = i32::from(byte as i8);
    if signed < 0 {
        format!("-{:x}", -signed)
    } else {
        format!("{signed:x}")
    }
}

/// `script_userRiskData_general` in `akamai/siem`:
/// `akamai.siem.user_risk.general`, from the vendor's `key:value|key:value`
/// string.
///
/// A field with no colon in it is kept with a dash for a value, because the
/// pipeline's closing prune drops an empty one.
fn parse_user_risk_general(event: &mut Event, _params: &Value) {
    let Some(text) = event
        .get_str("json.userRiskData.general")
        .map(|held| held.trim().to_owned())
    else {
        return;
    };
    if text.is_empty() {
        return;
    }

    let mut fields: Vec<&str> = text.split('|').collect();
    // Java's `Pattern.split` drops the trailing empties and keeps the rest.
    while fields.last() == Some(&"") {
        fields.pop();
    }
    let mut general = Map::new();
    for field in fields {
        if field.is_empty() {
            continue;
        }
        match field.find(':') {
            None => {
                general.insert(field.to_owned(), Value::from("-"));
            }
            Some(at) => {
                general.insert(field[..at].to_owned(), Value::from(&field[at + 1..]));
            }
        }
    }
    if general.is_empty() {
        return;
    }
    let _ = event.set("akamai.siem.user_risk.general", Value::Object(general));
}

/// Every akamai script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "26af47fb381e55afc57a8edc20b0d9d9d3d965dd4775a03f74c9326ab9707e7e",
        source: "akamai",
        name: "fold_attack_rules",
        run: fold_attack_rules,
    },
    Entry {
        hash: "504f25868bec1aa2b5ed333bfd1fbe8083b17c59a04fa3805eba138f3abbefaa",
        source: "akamai",
        name: "parse_user_risk_general",
        run: parse_user_risk_general,
    },
];
