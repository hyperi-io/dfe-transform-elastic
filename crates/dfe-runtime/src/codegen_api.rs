// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The free functions the transform modules call.
//!
//! One flat namespace, re-exported through [`crate::prelude`], so a transform
//! reads as a sequence of processor calls. They wrap the enrichment modules,
//! or stub what needs configuration this build may not have (`GeoIP`
//! databases, a Painless interpreter).

use std::collections::HashMap;
use std::sync::LazyLock;

use serde_json::Value;
use tracing::debug;

use crate::enrichment::user_agent;
use crate::error::Result;
use crate::event::Event;

/// Result from a registered domain lookup.
pub struct RegisteredDomainResult {
    /// Absent when the name is ITSELF a public suffix -- `com` has a top-level
    /// domain and nothing registered under it.
    pub registered_domain: Option<String>,
    pub top_level_domain: String,
    pub subdomain: Option<String>,
}

/// Look up `GeoIP` data for an IP address.
///
/// Returns a flat map of field names to values (e.g., "`country_iso_code`" -> "AU").
/// Uses the global auto-initialised enricher (auto-detects MMDB files).
pub fn geoip_lookup(db_name: &str, ip: &str) -> Result<HashMap<String, Value>> {
    Ok(crate::enrichment::geoip_global::geoip_lookup(db_name, ip))
}

/// Parse a User-Agent string into structured components.
pub fn parse_user_agent(ua: &str) -> Result<user_agent::UserAgentResult> {
    Ok(user_agent::parse(ua))
}

/// Compute a Community ID v1 hash for network flow identification.
pub fn community_id_v1(
    src_ip: &str,
    dst_ip: &str,
    src_port: u16,
    dst_port: u16,
    protocol: &str,
) -> std::result::Result<String, String> {
    crate::enrichment::community_id::community_id_v1(
        src_ip, dst_ip, src_port, dst_port, protocol, 0,
    )
}

/// Split a fully qualified domain name into its registered parts.
///
/// An ICANN suffix is honoured whole, which is the only way `211.52.31.172
/// .in-addr.arpa` registers `172.in-addr.arpa` rather than the last two labels.
///
/// A PRIVATE entry of THREE labels or more is the registered domain itself, so
/// `ec2-instance-connect.us-east-1.amazonaws.com` registers
/// `us-east-1.amazonaws.com` under `amazonaws.com`. A two-label private entry
/// lands on the same split either way, so it is left to the ICANN path.
///
/// An unlisted top-level domain gets NOTHING. Elasticsearch's processor is a
/// list lookup and a name off the list has no answer, so `domain.tld` has no
/// registered domain -- which is what the list is a dependency for.
///
/// The list is lowercase and the lookup is case-INSENSITIVE, so a DNS query
/// name arrives as `B.ROOT-SERVERS.NET` and registers `root-servers.net`.
/// Matching the bytes as they came found no suffix at all and wrote nothing.
///
/// The SUBDOMAIN is cut from the name as it arrived, though, so an uppercase
/// name gets none: the registered domain came off the lowercase list and does
/// not end the original text. `B.ROOT-SERVERS.NET` registers with no
/// subdomain, which is what Elasticsearch writes.
pub fn registered_domain_lookup(domain: &str) -> Option<RegisteredDomainResult> {
    let original = domain;
    let lowered;
    let domain = if domain.bytes().any(|b| b.is_ascii_uppercase()) {
        lowered = domain.to_ascii_lowercase();
        lowered.as_str()
    } else {
        domain
    };

    let parts: Vec<&str> = domain.rsplitn(3, '.').collect();
    if parts.len() < 2 {
        return None;
    }
    if !psl::suffix(parts[0].as_bytes()).is_some_and(|suffix| suffix.is_known()) {
        return None;
    }

    let listed = psl::suffix(domain.as_bytes())
        .and_then(|suffix| Some((std::str::from_utf8(suffix.as_bytes()).ok()?, suffix.typ()?)))
        .filter(|(suffix, _)| suffix.len() < domain.len());

    if let Some((private, _)) =
        listed.filter(|(suffix, typ)| *typ == psl::Type::Private && suffix.split('.').count() > 2)
        && let Some((_, tld)) = private.split_once('.')
    {
        return Some(RegisteredDomainResult {
            registered_domain: Some(private.to_string()),
            top_level_domain: tld.to_string(),
            subdomain: subdomain_of(original, private),
        });
    }

    let icann = listed
        .filter(|(_, typ)| *typ == psl::Type::Icann)
        .map(|(suffix, _)| suffix);

    // Everything the suffix does not cover, split at its last label.
    let suffix = icann.unwrap_or(parts[0]);
    let head = &domain[..domain.len() - suffix.len() - 1];
    let label = head.rsplit_once('.').map_or(head, |(_, label)| label);
    if label.is_empty() {
        return None;
    }

    let registered = format!("{label}.{suffix}");
    Some(RegisteredDomainResult {
        subdomain: subdomain_of(original, &registered),
        registered_domain: Some(registered),
        top_level_domain: suffix.to_string(),
    })
}

/// What precedes the registered domain in the name AS IT ARRIVED.
///
/// Elasticsearch cuts the subdomain out of the original text, and the
/// registered domain came off a lowercase list, so a name that does not end
/// with it -- an uppercase DNS query, say -- gets no subdomain at all.
fn subdomain_of(domain: &str, registered: &str) -> Option<String> {
    domain
        .strip_suffix(registered)
        .and_then(|rest| rest.strip_suffix('.'))
        .filter(|rest| !rest.is_empty())
        .map(str::to_string)
}

/// Execute a Painless script against an event.
///
/// Tries known common patterns first (drop nulls, command line extraction,
/// `keys_to_snake_case`, etc.). Falls back to a no-op for unrecognised scripts.
pub fn painless_exec(event: &mut Event, script: &str) -> Result<()> {
    painless_exec_params(event, script, &serde_json::Value::Null)
}

/// Execute a Painless script that carries a `params` block.
///
/// The recurring params shapes -- sentinel lists, field lists, lookup tables --
/// read their whole behaviour out of `params`, so the script text alone cannot
/// run them. The generated code passes the pipeline's params block verbatim.
pub fn painless_exec_params(
    event: &mut Event,
    script: &str,
    params: &serde_json::Value,
) -> Result<()> {
    if crate::painless_params::try_params_painless(event, script, params) {
        crate::painless_stats::record_handled();
        return Ok(());
    }
    if crate::painless_common::try_known_painless(event, script) {
        crate::painless_stats::record_handled();
        return Ok(());
    }
    // Counted, because an uncounted skip is indistinguishable from a script
    // that did nothing.
    crate::painless_stats::record_unhandled(script);
    debug!(
        script_len = script.len(),
        "painless_exec: unrecognised script skipped"
    );
    Ok(())
}

/// Run one processor over every element of an array field, the way Elastic's
/// `foreach` does: each element is exposed at `_ingest._value` for the body,
/// then collected back into the field.
///
/// The array is TAKEN, not cloned -- every element moves through
/// `_ingest._value` and back without an allocation, where the old inline
/// loop copied the whole array up front. The body only ever touches
/// `_ingest._value`; a generated body that names the field itself stays on
/// the cloning inline form, which the generator decides.
///
/// A body that fails part-way leaves what Elastic's shared-reference
/// iteration leaves: the processed prefix with its mutations, the failing
/// element -- still exposed at `_ingest._value`, as the inline loop left it
/// -- and the untouched rest, all back in the field.
///
/// # Errors
///
/// Whatever the body returned, after the field is restored.
pub fn foreach_array<F>(event: &mut Event, field: &str, mut body: F) -> Result<()>
where
    F: FnMut(&mut Event) -> Result<()>,
{
    let Some(items) = event.take_array(field) else {
        // A MAP is a foreach too: Elastic walks its entries, exposing the key
        // at `_ingest._key` and the value at `_ingest._value`, and rebuilds it
        // from whatever the body left in each. mimecast's attachment hashes
        // are a map, and skipping it lost every one of them.
        if event.get(field).is_some_and(Value::is_object) {
            return foreach_map(event, field, body);
        }
        return Ok(());
    };

    // A NESTED foreach borrows the same `_ingest._value` slot, so the
    // enclosing element is saved and put back afterwards -- Elasticsearch's
    // own processor restores the previous scope the same way. Without this
    // the inner loop replaced the outer's element and then wrote back INTO
    // the replacement.
    let enclosing = event.get("_ingest._value").cloned();

    // Probe the `_ingest._value` slot before consuming anything: the one way
    // the per-element set can fail is `_ingest` sitting there as a scalar,
    // and failing NOW lets the array go back untouched.
    if let Err(error) = event.set("_ingest._value", Value::Null) {
        event.set(field, Value::Array(items))?;
        return Err(error);
    }

    let mut out: Vec<Value> = Vec::with_capacity(items.len());
    let mut rest = items.into_iter();
    while let Some(item) = rest.next() {
        // Cannot fail: the probe above proved the path writable.
        event.set("_ingest._value", item)?;
        if let Err(error) = body(event) {
            if let Some(failed) = event.get("_ingest._value") {
                out.push(failed.clone());
            }
            out.extend(rest);
            event.set(field, Value::Array(out))?;
            return Err(error);
        }
        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
    }

    match enclosing {
        // Restore BEFORE the write-back: a nested loop's field lives inside
        // the restored element.
        Some(previous) => {
            event.set("_ingest._value", previous)?;
        }
        None => {
            event.remove("_ingest");
        }
    }
    event.set(field, Value::Array(out))?;
    Ok(())
}

/// [`foreach_array`] over a MAP: each entry's key and value are exposed, and
/// the map is rebuilt from what the body leaves in them.
///
/// A body that renames `_ingest._key` moves the entry, which is what
/// Elasticsearch's own processor does with the pair it puts back.
fn foreach_map<F>(event: &mut Event, field: &str, mut body: F) -> Result<()>
where
    F: FnMut(&mut Event) -> Result<()>,
{
    let Some(Value::Object(entries)) = event.get(field).cloned() else {
        return Ok(());
    };
    let enclosing_key = event.get("_ingest._key").cloned();
    let enclosing_value = event.get("_ingest._value").cloned();

    let mut out = serde_json::Map::with_capacity(entries.len());
    for (key, value) in entries {
        event.set("_ingest._key", Value::String(key))?;
        event.set("_ingest._value", value)?;
        body(event)?;
        let Some(Value::String(key)) = event.remove("_ingest._key") else {
            continue;
        };
        out.insert(key, event.remove("_ingest._value").unwrap_or(Value::Null));
    }

    for (path, previous) in [
        ("_ingest._key", enclosing_key),
        ("_ingest._value", enclosing_value),
    ] {
        match previous {
            Some(value) => event.set(path, value)?,
            None => {
                event.remove(path);
            }
        }
    }
    if event
        .get_object("_ingest")
        .is_some_and(serde_json::Map::is_empty)
    {
        event.remove("_ingest");
    }
    event.set(field, Value::Object(out))?;
    Ok(())
}

/// Parse one field's JSON string into `target`, the way Elastic's `json`
/// processor does.
///
/// An absent field is a no-op, exactly as the old inline `if let` was; a
/// present one that will not parse is the processor's failure, worded the
/// same way the inline `serde_json` block worded it so `on_failure` output
/// does not shift.
///
/// # Errors
///
/// Returns [`crate::TransformError::ParseError`] naming the FIELD when the
/// text is not JSON, or whatever `set` returns for an unwritable target.
pub fn parse_json_field(event: &mut Event, field: &str, target: &str) -> Result<()> {
    let Some(text) = event.get_string(field) else {
        return Ok(());
    };
    let parsed = parse_json_str(&text).map_err(|message| crate::TransformError::ParseError {
        path: field.into(),
        message,
    })?;
    event.set(target, parsed)?;
    Ok(())
}

/// Parse a JSON string into the document's own value type.
///
/// `serde_json`, not simd-json. simd-json is faster at building ITS tape;
/// getting a `serde_json::Value` out of it goes tape -> serde deserializer ->
/// `Value`, which is strictly more work than `serde_json` parsing straight
/// into the same tree, and the map here is an `IndexMap` because
/// `preserve_order` is not optional for parity. Measured in
/// `benches/json_parse.rs`: 6.7 vs 8.9 microseconds on an okta document and
/// 115 vs 271 nanoseconds on a small one, with half the allocations.
///
/// # Errors
///
/// Returns the parser's message, prefixed the way the generated modules
/// always worded it.
pub fn parse_json_str(text: &str) -> std::result::Result<Value, String> {
    serde_json::from_str::<Value>(text).map_err(|e| format!("failed to parse JSON: {e}"))
}

/// Turn keys whose NAME contains dots into the nested objects they describe.
///
/// `path` names the object to work on, empty for the document root, and
/// `field` the key to expand -- `*` for every dotted key it holds. A key that
/// is not there, or holds no dot, is left alone.
///
/// # Errors
///
/// Returns an error only if the event refuses a write.
pub fn dot_expand(event: &mut crate::Event, path: &str, field: &str) -> crate::Result<()> {
    // Decided on a BORROW, before anything is cloned. The rebuild below copies
    // the whole container, and for the root that is the entire document -- a
    // deep clone per event, on every source whose pipeline opens with a
    // `dot_expander`, whether or not a single key held a dot.
    let dotted = {
        let members = if path.is_empty() {
            event.as_value().as_object()
        } else {
            event.get(path).and_then(Value::as_object)
        };
        let Some(members) = members else {
            return Ok(());
        };
        members
            .keys()
            .any(|key| key.contains('.') && (field == "*" || key == field))
    };
    if !dotted {
        return Ok(());
    }

    let container = if path.is_empty() {
        event.as_value().clone()
    } else {
        match event.get(path) {
            Some(value) => value.clone(),
            None => return Ok(()),
        }
    };
    let Some(members) = container.as_object() else {
        return Ok(());
    };

    // Rebuilt whole rather than removed and re-set key by key: a dotted key
    // and the nested path that replaces it are spelled the same, so the two
    // operations would race over one name.
    let mut rebuilt = serde_json::Map::new();
    let mut expanded = false;
    for (key, value) in members {
        if key.contains('.') && (field == "*" || key == field) {
            expanded = true;
            let mut node = &mut rebuilt;
            let mut segments = key.split('.').peekable();
            while let Some(segment) = segments.next() {
                if segments.peek().is_none() {
                    node.insert(segment.to_string(), value.clone());
                    break;
                }
                node = node
                    .entry(segment.to_string())
                    .or_insert_with(|| Value::Object(serde_json::Map::new()))
                    .as_object_mut()
                    .ok_or_else(|| crate::TransformError::FieldNotFound { path: key.clone() })?;
            }
        } else {
            rebuilt.insert(key.clone(), value.clone());
        }
    }

    if expanded {
        if path.is_empty() {
            *event.as_value_mut() = Value::Object(rebuilt);
        } else {
            event.set(path, Value::Object(rebuilt))?;
        }
    }
    Ok(())
}

/// Convert a value the way Elastic's `convert` processor does.
///
/// An ARRAY is converted element by element -- "if the field value is an
/// array, all members will be converted" -- where stringifying the array
/// whole gave `"[\"0\"]"` for a list that should have been `["0"]`, and the
/// pipeline's own sentinel pass then had nothing it recognised to remove.
///
/// `kind` is the processor's `type`: `integer`, `long`, `float`, `double`,
/// `string`, `boolean` or `ip`. `auto` picks its type at ingest time and a
/// compiled transform cannot do that, so the generator refuses it and it
/// never reaches here.
///
/// # Errors
///
/// Returns the message Elastic's processor throws with, for the caller to
/// wrap in a `ParseError` naming the field.
pub fn convert_value(value: &Value, kind: &str) -> std::result::Result<Value, String> {
    if let Value::Array(items) = value {
        return items
            .iter()
            .map(|item| convert_value(item, kind))
            .collect::<std::result::Result<Vec<_>, _>>()
            .map(Value::Array);
    }

    let cannot = |target: &str| format!("cannot convert '{value}' to {target}");
    match kind {
        "integer" | "long" => match value {
            Value::String(s) => {
                let s = s.trim();
                let parsed = s.strip_prefix("0x").map_or_else(
                    || s.parse::<i64>().ok(),
                    |hex| i64::from_str_radix(hex, 16).ok(),
                );
                parsed.map(Value::from).ok_or_else(|| cannot("integer"))
            }
            #[allow(clippy::cast_possible_truncation)]
            Value::Number(n) => Ok(Value::from(
                n.as_i64()
                    .unwrap_or_else(|| n.as_f64().unwrap_or(0.0) as i64),
            )),
            Value::Bool(b) => Ok(Value::from(i64::from(*b))),
            _ => Err(cannot("integer")),
        },
        "float" | "double" => match value {
            Value::String(s) => s
                .trim()
                .parse::<f64>()
                .ok()
                .map(Value::from)
                .ok_or_else(|| cannot("float")),
            Value::Number(n) => Ok(Value::from(n.as_f64().unwrap_or(0.0))),
            Value::Bool(b) => Ok(Value::from(if *b { 1.0 } else { 0.0 })),
            _ => Err(cannot("float")),
        },
        "string" => match value {
            Value::String(_) => Ok(value.clone()),
            Value::Number(n) => Ok(Value::from(n.to_string())),
            Value::Bool(b) => Ok(Value::from(b.to_string())),
            Value::Null => Ok(Value::from("null")),
            other => Ok(Value::from(other.to_string())),
        },
        // Elastic accepts only the exact strings, case insensitively, and
        // throws on anything else.
        "boolean" => match value {
            Value::Bool(_) => Ok(value.clone()),
            Value::String(s) if s.eq_ignore_ascii_case("true") => Ok(Value::from(true)),
            Value::String(s) if s.eq_ignore_ascii_case("false") => Ok(Value::from(false)),
            _ => Err(cannot("boolean")),
        },
        "ip" => match value.as_str() {
            Some(s) if s.trim().parse::<std::net::IpAddr>().is_ok() => Ok(Value::from(s.trim())),
            _ => Err(cannot("IP")),
        },
        other => Err(format!("unknown convert type '{other}'")),
    }
}

/// The digest Elastic's `fingerprint` processor writes at its defaults.
///
/// Method `SHA-1`, no salt, and the result base64-encoded. Each value is
/// preceded by a single NUL, which is the processor's own delimiter; a field
/// NAME is not included for a scalar. Recovered from the corpus rather than
/// guessed -- `m365_defender`'s `process.entity_id` carries the answer next to
/// its inputs.
///
/// A value is rendered the way the document holds it: a string is its own
/// text, not its JSON with quotes around it.
#[must_use]
pub fn fingerprint_default(values: &[Value]) -> String {
    use base64::Engine as _;
    use sha1::{Digest, Sha1};

    let mut hasher = Sha1::new();
    for value in values {
        hasher.update([0u8]);
        hasher.update(crate::painless_helpers::painless_to_string(value).as_bytes());
    }
    base64::engine::general_purpose::STANDARD.encode(hasher.finalize())
}

/// Sort an array's elements, the way Elastic's `sort` processor does.
///
/// Elastic sorts by the elements' natural ordering, so the array has to be
/// all-numbers or all-strings; anything mixed throws in Java. Booleans sort
/// false before true, which is Java's `Boolean.compareTo`.
///
/// Returns `None` when the value is not an array, or holds something with no
/// natural ordering -- an object, a nested array, a null, or a mix of kinds.
/// That is the case Elastic throws on, so the caller raises.
#[must_use]
pub fn sort_values(value: &Value, descending: bool) -> Option<Vec<Value>> {
    let items = value.as_array()?;

    let mut sorted = items.clone();
    let all = |f: fn(&Value) -> bool| items.iter().all(f);
    if all(Value::is_string) {
        sorted.sort_by(|a, b| a.as_str().cmp(&b.as_str()));
    } else if all(Value::is_number) {
        sorted.sort_by(|a, b| {
            a.as_f64()
                .partial_cmp(&b.as_f64())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    } else if all(Value::is_boolean) {
        sorted.sort_by_key(Value::as_bool);
    } else if !items.is_empty() {
        return None;
    }

    if descending {
        sorted.reverse();
    }
    Some(sorted)
}

/// Percent-decode a string the way Elastic's `urldecode` processor does.
///
/// The processor calls Java's `URLDecoder.decode(value, "UTF-8")`, which is
/// the `application/x-www-form-urlencoded` reading rather than the RFC 3986
/// one: `+` becomes a SPACE. Zscaler's rule labels arrive that way.
///
/// Returns `None` on a malformed escape -- a `%` with fewer than two hex
/// digits after it -- which is where Java throws and the processor's
/// `on_failure` runs. Borrows when there is nothing to decode.
#[must_use]
pub fn url_decode(text: &str) -> Option<std::borrow::Cow<'_, str>> {
    if !text.contains('%') && !text.contains('+') {
        return Some(std::borrow::Cow::Borrowed(text));
    }

    let bytes = text.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' => {
                let hex = text.get(i + 1..i + 3)?;
                out.push(u8::from_str_radix(hex, 16).ok()?);
                i += 3;
            }
            byte => {
                out.push(byte);
                i += 1;
            }
        }
    }
    String::from_utf8(out).ok().map(std::borrow::Cow::Owned)
}

/// Join an array's elements into one separated string, the way Elastic's
/// `join` processor does.
///
/// Every element is stringified first -- Elastic calls `toString()` on each,
/// so a list of numbers joins as readily as a list of strings, and only a
/// nested object or array has no sensible rendering. Those are skipped rather
/// than written as their JSON, which is what `to_string` would give and is
/// never what the pipeline meant.
///
/// Returns `None` when the value is not an array, which is the case Elastic
/// throws on: writing a joined string for a scalar would invent one.
#[must_use]
pub fn join_values(value: &Value, separator: &str) -> Option<String> {
    let items = value.as_array()?;
    let mut out = String::new();
    // A `first` flag, not `out.is_empty()`: an empty string as the FIRST
    // element leaves `out` empty and would swallow the separator after it.
    let mut first = true;
    for item in items {
        let piece = match item {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null | Value::Array(_) | Value::Object(_) => continue,
        };
        if !first {
            out.push_str(separator);
        }
        first = false;
        out.push_str(&piece);
    }
    Some(out)
}

/// Close the gap between a delimiter and an opening quote, and nothing else.
///
/// Elasticsearch's CSV processor treats `, "a,b"` as a quoted field; a strict
/// reader treats the space as content, so the field is unquoted and its
/// embedded commas shift every column after it. Trimming the whole field would
/// fix that and also strip whitespace the vendor keeps -- panw's descriptions
/// carry it -- so only the run between a delimiter and a quote is removed.
///
/// Borrows unless there is something to remove.
#[must_use]
pub fn csv_close_quote_gap(line: &str, delimiter: char, quote: char) -> std::borrow::Cow<'_, str> {
    let mut out: Option<String> = None;
    let mut inside = false;
    let mut at_field_start = true;
    let mut chars = line.char_indices().peekable();

    while let Some((index, c)) = chars.next() {
        if c == quote {
            inside = !inside;
            at_field_start = false;
        } else if c == delimiter && !inside {
            at_field_start = true;
        } else if at_field_start && c.is_whitespace() && !inside {
            // Whitespace opening a field: drop it only if a quote follows the
            // whole run, which is what makes the field a quoted one.
            let run: String = std::iter::once(c)
                .chain(std::iter::from_fn(|| {
                    chars.next_if(|(_, n)| n.is_whitespace()).map(|(_, n)| n)
                }))
                .collect();
            if chars.peek().is_some_and(|(_, n)| *n == quote) {
                out.get_or_insert_with(|| line[..index].to_string());
                continue;
            }
            if let Some(kept) = out.as_mut() {
                kept.push_str(&run);
            }
            at_field_start = false;
            continue;
        } else {
            at_field_start = false;
        }
        if let Some(kept) = out.as_mut() {
            kept.push(c);
        }
    }

    out.map_or(std::borrow::Cow::Borrowed(line), std::borrow::Cow::Owned)
}

/// Convert a grok pattern string to a regex pattern string.
///
/// Expands `%{NAME:field}` to named capture groups with type-appropriate
/// sub-patterns. Returns `(regex_string, field_map)` where `field_map` maps
/// safe capture names back to original dotted field paths.
///
/// Phase 3 will replace grok with native dfe-parse parsers.
pub fn grok_to_regex(pattern: &str) -> String {
    grok_to_regex_with_map(pattern).0
}

/// Like `grok_to_regex` but also returns a map of `capture_name` → `original_field_path`.
///
/// This is needed because regex capture names can't contain dots, so
/// `user.name` becomes `user_name` in the regex. The map lets callers
/// restore the original dotted path when setting fields.
pub fn grok_to_regex_with_map(
    pattern: &str,
) -> (String, std::collections::HashMap<String, String>) {
    let (regex, field_map, _) = grok_to_regex_typed(pattern);
    (regex, field_map)
}

/// As [`grok_to_regex_with_map`], plus the captures Elastic types as numbers.
///
/// A `%{NUMBER:bytes:long}` suffix is a type, not part of the field name, and
/// dropping it leaves every numeric field a string.
#[must_use]
pub fn grok_to_regex_typed(
    pattern: &str,
) -> (
    String,
    std::collections::HashMap<String, String>,
    std::collections::HashMap<String, bool>,
) {
    use std::fmt::Write as _;

    let mut result = String::with_capacity(pattern.len());
    let mut field_map = std::collections::HashMap::new();
    let mut numeric = std::collections::HashMap::new();
    // Every group name emitted so far. One ledger for both kinds of capture,
    // because a `%{DATA:process.name}` and a literal `(?P<process_name>)` in
    // the same pattern collide just as surely as two of either.
    let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '(' && starts_named_group(&chars) {
            // A named group written out in full, either by the vendor or by
            // the generator inlining a pattern definition. Java's engine takes
            // the same name in several alternation branches; Rust's rejects
            // the pattern outright, so each repeat is renamed and pointed at
            // the same destination -- only the branch that matched writes.
            let prefix: String = chars.by_ref().take_while(|c| *c != '<').collect();
            let name: String = chars.by_ref().take_while(|c| *c != '>').collect();
            let safe = unique_group_name(&sanitise_group_name(&name), &mut used);
            field_map.insert(safe.clone(), name);
            let _ = write!(result, "({prefix}<{safe}>");
            continue;
        }
        if c == '%' && chars.peek() == Some(&'{') {
            chars.next(); // consume '{'
            let mut name = String::new();
            let mut field = String::new();
            let mut in_field = false;

            for ch in chars.by_ref() {
                if ch == '}' {
                    break;
                } else if ch == ':' && !in_field {
                    in_field = true;
                } else if in_field {
                    field.push(ch);
                } else {
                    name.push(ch);
                }
            }

            let sub_pattern = grok_pattern_regex(&name);

            if field.is_empty() {
                // A few builtins carry their own destination, as Elastic's own
                // registry defines them -- used bare, they still capture.
                if let Some((safe, path, inner)) = grok_implicit_capture(&name) {
                    used.insert(safe.to_string());
                    field_map.insert(safe.to_string(), path.to_string());
                    numeric.insert(safe.to_string(), true);
                    let _ = write!(result, "{inner}");
                } else {
                    let _ = write!(result, "({sub_pattern})");
                }
            } else {
                // Strip Elastic type suffix (e.g., "source.ip:ip" → "source.ip")
                let mut parts = field.splitn(2, ':');
                let field_name = parts.next().unwrap_or(&field);
                // Java allows one field name in several alternation branches
                // and Rust's engine rejects a duplicate group name outright, so
                // the whole pattern fails to compile and matches nothing. Each
                // repeat gets a group of its own pointing at the same field;
                // only the branch that matched writes.
                let safe_field = unique_group_name(&sanitise_group_name(field_name), &mut used);

                if matches!(parts.next(), Some("long" | "int" | "float" | "double")) {
                    numeric.insert(safe_field.clone(), true);
                }
                field_map.insert(safe_field.clone(), field_name.to_string());
                let _ = write!(result, "(?P<{safe_field}>{sub_pattern})");
            }
        } else if c == '\\' {
            // Copy an escape whole, so the char it protects is never read as
            // syntax on the next turn of the loop.
            result.push(c);
            if let Some(escaped) = chars.next() {
                result.push(escaped);
            }
        } else if c == '{' && !opens_repetition(&chars) {
            // Elasticsearch groks with Oniguruma, which reads a brace that is
            // not a valid repetition as ordinary text. Rust's engine reads it
            // as a repetition with nothing to repeat and rejects the pattern.
            // Elastic's own SYSLOG_HEADER carries `(?:{DATA})?` -- a `%` short
            // of a pattern reference, and so a harmless optional literal there
            // and a dead grok here.
            result.push_str(r"\{");
        } else {
            result.push(c);
        }
    }

    resolve_capture_paths(&mut field_map, &mut numeric);
    (result, field_map, numeric)
}

/// Does an opening paren begin a named capture, rather than a look-behind?
///
/// `(?P<x>` and `(?<x>` are captures; `(?<=` and `(?<!` are assertions, and
/// renaming inside one would corrupt the pattern.
fn starts_named_group(chars: &std::iter::Peekable<std::str::Chars<'_>>) -> bool {
    let mut ahead = chars.clone();
    if ahead.next() != Some('?') {
        return false;
    }
    match ahead.next() {
        Some('P') => ahead.next() == Some('<'),
        Some('<') => !matches!(ahead.next(), Some('=' | '!') | None),
        _ => false,
    }
}

/// Does a brace begin a repetition -- `{2}`, `{2,}`, `{2,5}` -- or is it text?
fn opens_repetition(chars: &std::iter::Peekable<std::str::Chars<'_>>) -> bool {
    let inner: String = chars.clone().take(16).take_while(|c| *c != '}').collect();
    if inner.is_empty() || inner.len() == 16 {
        return false;
    }
    let (low, high) = inner.split_once(',').unwrap_or((inner.as_str(), ""));
    !low.is_empty()
        && low.bytes().all(|b| b.is_ascii_digit())
        && high.bytes().all(|b| b.is_ascii_digit())
}

/// Reduce a capture name to what a regex group name may hold.
///
/// Elastic's grok reads the name as `[pattern:]field[:type]` and the vendor
/// pipelines write dotted paths straight into it, so a name arrives carrying
/// dots and colons that Rust's engine will not accept. The real destination is
/// recovered from the name by [`resolve_capture_paths`]; this only has to
/// produce something the engine can compile.
fn sanitise_group_name(name: &str) -> String {
    let mut safe: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    if safe.starts_with(|c: char| c.is_ascii_digit()) {
        safe.insert(0, 'g');
    }
    if safe.is_empty() {
        safe.push('g');
    }
    safe
}

/// Return `base`, or the first `base__N` nobody has taken.
fn unique_group_name(base: &str, used: &mut std::collections::HashSet<String>) -> String {
    if used.insert(base.to_string()) {
        return base.to_string();
    }
    for n in 2.. {
        let candidate = format!("{base}__{n}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
    }
    unreachable!("the counter is unbounded")
}

/// Numeric type suffixes Elastic's grok understands.
const NUMERIC_TYPES: [&str; 4] = ["int", "long", "float", "double"];

/// Read a grok capture name as Elastic does: `[pattern:]field[:type]`.
///
/// The vendor pipelines write the internal form by hand -- `cisco_asa`'s repeat
/// counter is `(?<INT:_temp_.cisco.message_repeats:int>\d+)` -- so the field is
/// the middle part and the type the last, not the whole string.
///
/// Returns the destination path and whether Elastic types it as a number.
#[must_use]
pub fn read_capture_name(name: &str) -> (&str, bool) {
    let parts: Vec<&str> = name.split(':').collect();
    match parts.as_slice() {
        [_, field, ty] if NUMERIC_TYPES.contains(ty) => (field, true),
        [_, field, _] => (field, false),
        [field, ty] if NUMERIC_TYPES.contains(ty) => (field, true),
        _ => (name, false),
    }
}

/// Rewrite every destination in `field_map` to the path Elastic would write,
/// marking the numeric ones. Idempotent, so it can run again once a caller's
/// own mapping has been substituted in.
pub fn resolve_capture_paths<S: std::hash::BuildHasher>(
    field_map: &mut std::collections::HashMap<String, String, S>,
    numeric: &mut std::collections::HashMap<String, bool, S>,
) {
    for (capture, path) in field_map.iter_mut() {
        let (field, is_numeric) = read_capture_name(path);
        if is_numeric {
            numeric.insert(capture.clone(), true);
        }
        if field.len() != path.len() {
            *path = field.to_string();
        }
    }
}

/// Builtins whose Elastic definition captures a field of its own.
///
/// `%{SYSLOG5424PRI}` is written without a field name throughout the vendor
/// pipelines because the destination is part of the pattern. Elastic's registry
/// defines it as `<%{NONNEGINT:syslog5424_pri}>`, and the pipelines that use it
/// then remove that field by name -- writing it to `log.syslog.priority`
/// instead left a field Elastic does not emit on every event.
///
/// Returns `(safe capture name, dotted path, the regex to emit)`.
fn grok_implicit_capture(name: &str) -> Option<(&'static str, &'static str, &'static str)> {
    match name {
        "SYSLOG5424PRI" => Some((
            "syslog5424_pri",
            "syslog5424_pri",
            r"<(?P<syslog5424_pri>\d{1,5})>",
        )),
        _ => None,
    }
}

/// A month name, abbreviated or spelled out.
///
/// `\w+` would do here too, but it also matches a bare word, so a pattern
/// meant to anchor on a date matches text that holds none.
const MONTH: &str = concat!(
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?",
    r"|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)",
);

/// A clock time. Elastic's `SECOND` carries an optional fraction, so a
/// pattern anchored on `%{TIME}` has to accept `13:20:48.739`.
const TIME: &str = r"\d{1,2}:\d{2}(?::\d{2}(?:[.,]\d+)?)?";

/// `%{MONTHDAY}/%{MONTH}/%{YEAR}:%{TIME} %{INT}` -- the Apache common-log
/// date, which the AWS load-balancer and cloudfront pipelines grok.
const HTTPDATE: &str = concat!(
    r"\d{1,2}/",
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?",
    r"|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)",
    r"/\d{4}:\d{1,2}:\d{2}(?::\d{2}(?:[.,]\d+)?)? [+-]?\d+",
);

/// `%{MONTH} +%{MONTHDAY} %{TIME}` -- the BSD syslog date, fraction and all.
const SYSLOG_TIMESTAMP: &str = concat!(
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?",
    r"|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)",
    r" +\d{1,2} \d{1,2}:\d{2}(?::\d{2}(?:[.,]\d+)?)?",
);

/// Cisco's syslog date: the year may sit on either side of the time, and the
/// seconds may carry a fraction -- `Jan  6 2022 20:52:12.861`.
const CISCO_TIMESTAMP: &str = concat!(
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?",
    r"|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)",
    r" +\d{1,2}(?: \d{4})? \d{2}:\d{2}:\d{2}(?:\.\d+)?(?: \d{4})?",
);

/// Elastic's own `IPV6`, verbatim from logstash-patterns-core's ecs-v1 set.
///
/// The `[0-9a-fA-F:]+` this replaces matched any run of hex and colons -- a
/// bare `2a02` included -- and `IP` carried no v6 branch at all, so a grok
/// reading a v6 address failed outright and took every capture in the pattern
/// with it. `cisco_ios`'s syslog header is exactly that shape.
const IPV6: &str = r"((([0-9A-Fa-f]{1,4}:){7}([0-9A-Fa-f]{1,4}|:))|(([0-9A-Fa-f]{1,4}:){6}(:[0-9A-Fa-f]{1,4}|((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){5}(((:[0-9A-Fa-f]{1,4}){1,2})|:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){4}(((:[0-9A-Fa-f]{1,4}){1,3})|((:[0-9A-Fa-f]{1,4})?:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){3}(((:[0-9A-Fa-f]{1,4}){1,4})|((:[0-9A-Fa-f]{1,4}){0,2}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){2}(((:[0-9A-Fa-f]{1,4}){1,5})|((:[0-9A-Fa-f]{1,4}){0,3}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){1}(((:[0-9A-Fa-f]{1,4}){1,6})|((:[0-9A-Fa-f]{1,4}){0,4}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(:(((:[0-9A-Fa-f]{1,4}){1,7})|((:[0-9A-Fa-f]{1,4}){0,5}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:)))(%.+)?";

/// The simplified `IPV4`. Elastic's own carries `(?<![0-9])` look-around,
/// which the regex crate rejects -- and a grok that will not compile matches
/// nothing at all, which is worse than accepting `999.999.999.999`.
const IPV4: &str = r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}";

/// A hostname as Elastic defines it, for the composites below.
const HOSTNAME: &str = r"[a-zA-Z0-9._-]+";

/// `%{IP}` is `(?:%{IPV6}|%{IPV4})` and `%{IPORHOST}` is `(?:%{IP}|%{HOSTNAME})`,
/// v6 first, exactly as Elastic orders them. Built once because `concat!` will
/// not expand a const, and this runs when a pattern compiles, never per event.
static IP: LazyLock<String> = LazyLock::new(|| format!("(?:{IPV6}|{IPV4})"));
static IPORHOST: LazyLock<String> = LazyLock::new(|| format!("(?:{IPV6}|{IPV4}|{HOSTNAME})"));

/// Map well-known grok pattern names to their regex equivalents.
fn grok_pattern_regex(name: &str) -> &'static str {
    match name {
        "USER" | "USERNAME" | "HOSTNAME" => HOSTNAME,
        "IP" => IP.as_str(),
        "IPV4" => IPV4,
        "IPV6" => IPV6,
        "POSINT" | "PORT" | "NONNEGINT" => r"\d+",
        "INT" => r"[+-]?\d+",
        "NUMBER" | "BASE10NUM" => r"[+-]?(?:\d+\.?\d*|\.\d+)",
        // Elastic's own, look-behind and all, so it compiles on fancy-regex.
        // Without the guard `deadbeef` would match starting at `eadbeef`.
        "BASE16NUM" => r"(?<![0-9A-Fa-f])(?:[+-]?(?:0x)?(?:[0-9A-Fa-f]+))",
        "NOTSPACE" | "URI" | "URIPROTO" => r"\S+",
        "GREEDYDATA" => r".*",
        "DATA" => r".*?",
        "WORD" => r"\w+",
        "MONTH" => MONTH,
        "MAC" => r"(?:[0-9a-fA-F]{2}[:-]){5}[0-9a-fA-F]{2}",
        "EMAILADDRESS" => r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
        "PATH" | "UNIXPATH" | "WINPATH" => r"[^\s]+",
        // Azure custom patterns (from pipeline pattern_definitions)
        "SUBID" => {
            r"(?:\{)?[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}(?:\})?"
        }
        // A path segment. `PROVIDER` on the catch-all `.+?` matched lazily,
        // so `/providers/Microsoft.aadiam` yielded "M".
        "GROUPID" | "PROVIDERNAME" | "PROVIDER" | "NAMESPACE" | "RULE" | "NAME" => r"[^/]+",
        "MONTHDAY" | "MONTHNUM" => r"\d{1,2}",
        "YEAR" => r"\d{4}",
        // Elastic's own, and each part earns its shape. HOUR takes one digit
        // or two, because an offset is written `-5:00` as often as `-05:00`.
        // SECOND carries an optional fraction, without which checkpoint's
        // `16:39:12.000Z` leaves `.000Z` for the next literal to fail on.
        "HOUR" => r"(?:2[0-3]|[01]?\d)",
        "MINUTE" => r"[0-5]\d",
        "SECOND" => r"(?:[0-5]?\d|60)(?:[:.,]\d+)?",
        "ISO8601_TIMEZONE" => r"(?:Z|[+-](?:2[0-3]|[01]?\d)(?::?[0-5]\d))",
        // Whitespace, not "anything". The catch-all below made every pattern
        // containing %{SPACE} match arbitrary text -- 152 sites' worth.
        "SPACE" => r"\s*",
        "TIME" => TIME,
        "HTTPDATE" => HTTPDATE,
        "IPORHOST" | "SYSLOGHOST" => IPORHOST.as_str(),
        // Elastic accepts the abbreviation or the full name, either case.
        "DAY" => {
            r"(?i:Mon(?:day)?|Tue(?:sday)?|Wed(?:nesday)?|Thu(?:rsday)?|Fri(?:day)?|Sat(?:urday)?|Sun(?:day)?)"
        }
        "SYSLOGPRI" => r"<\d+>",
        "SYSLOG5424PRI" => r"<\d{1,5}>",
        // Printable ASCII minus space, `=`, `]` and `"` -- RFC 5424's own
        // definition, which is what bounds a structured-data name.
        "SYSLOG5424PRINTASCII" => r"[!#-<>-\\\^-~]+",
        "SYSLOGTIMESTAMP" => SYSLOG_TIMESTAMP,
        "CISCOTIMESTAMP" => CISCO_TIMESTAMP,
        "CISCOMAC" => r"(?:[A-Fa-f0-9]{4}\.){2}[A-Fa-f0-9]{4}",
        // Elastic quotes with any of the three, and reading only the double
        // form left cisco_meraki's `ssid=''` unmatched -- which took the whole
        // key-value line with it, on every airmarshal event.
        "QS" | "QUOTEDSTRING" => r#"(?:"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|`(?:[^`\\]|\\.)*`)"#,
        "LOGLEVEL" => r"(?i:emerg|alert|crit|err|warn|notice|info|debug|trace)\w*",
        // The fraction takes a comma as well as a dot: Elastic's SECOND does,
        // and hadoop's log4j writes `05:04:53,776` -- one missing comma
        // failed the whole emr grok on every event.
        "TIMESTAMP_ISO8601" => {
            r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:[.,]\d+)?(?:Z|[+-]\d{2}:?\d{2})?"
        }
        // Unknown name. `.+?` captures arbitrary text rather than failing, so
        // the field is populated with the wrong thing and nothing says so --
        // `unknown_grok_patterns` exists to make that visible.
        _ => ".+?",
    }
}

/// Pattern names still falling through to the catch-all, and how often.
///
/// The vendor-specific ones (`CISCO_*`, `NEXUS_*`, `IPV6PORTSEP`, ...) come
/// from `pattern_definitions` in the upstream ingest pipelines and have to be
/// carried across before they can be defined here.
#[must_use]
pub fn is_known_grok_pattern(name: &str) -> bool {
    grok_pattern_regex(name) != ".+?"
}

/// Check whether an IP address is in a private/internal range.
///
/// Recognises RFC 1918 (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16),
/// loopback (127.0.0.0/8), and link-local (169.254.0.0/16).
pub fn is_internal_ip(ip: &str) -> bool {
    use std::net::IpAddr;
    match ip.parse::<IpAddr>() {
        Ok(IpAddr::V4(v4)) => v4.is_private() || v4.is_loopback() || v4.is_link_local(),
        Ok(IpAddr::V6(v6)) => v6.is_loopback(),
        Err(_) => false,
    }
}

/// Resolve a field PATH that carries mustache references, against the event.
///
/// A processor's `field` may name itself from the document -- `cisco_meraki`
/// renames `cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac`, so the
/// subtree it reads is whatever the event's subtype says. Emitting the template
/// verbatim gave a path nothing ever matched.
///
/// Returns `None` when a reference resolves to nothing, because the name it
/// would build has an empty segment and matches no field either.
#[must_use]
pub fn resolve_path(event: &Event, template: &str) -> Option<String> {
    if !template.contains("{{") {
        return Some(template.to_string());
    }

    let mut resolved = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(open) = rest.find("{{") {
        resolved.push_str(&rest[..open]);

        // Mustache spells an unescaped reference with three braces and an
        // escaped one with two; a field path wants the value either way.
        let after = &rest[open..];
        let inner = after.trim_start_matches('{');
        let close = "}".repeat(after.len() - inner.len());
        let (name, tail) = inner.split_once(&close)?;

        let value = event.get_as_string(name.trim())?;
        if value.is_empty() {
            return None;
        }
        resolved.push_str(&value);
        rest = tail;
    }
    resolved.push_str(rest);
    Some(resolved)
}

/// The pieces of a URI reference, borrowed from the string they came from.
struct UriRef<'a> {
    scheme: Option<&'a str>,
    user_info: Option<&'a str>,
    host: Option<&'a str>,
    port: Option<&'a str>,
    path: &'a str,
    query: Option<&'a str>,
    fragment: Option<&'a str>,
}

/// Split a URI reference into its parts, per RFC 3986's generic syntax.
///
/// A relative reference is the case that matters and the one a URL crate will
/// not take: fortinet's `url` field is a bare path far more often than a whole
/// URL, and rejecting those loses `url.path` on every one of them.
fn split_uri(uri: &str) -> UriRef<'_> {
    let (rest, fragment) = match uri.split_once('#') {
        Some((before, after)) => (before, Some(after)),
        None => (uri, None),
    };
    let (rest, query) = match rest.split_once('?') {
        Some((before, after)) => (before, Some(after)),
        None => (rest, None),
    };

    // A scheme runs to the first `:`, but only when nothing before it could
    // make that colon part of a path or an authority instead.
    let (rest, scheme) = match rest.find(':') {
        Some(colon) if is_scheme(&rest[..colon]) => (&rest[colon + 1..], Some(&rest[..colon])),
        _ => (rest, None),
    };

    let Some(after_slashes) = rest.strip_prefix("//") else {
        return UriRef {
            scheme,
            user_info: None,
            host: None,
            port: None,
            path: rest,
            query,
            fragment,
        };
    };

    let end = after_slashes.find('/').unwrap_or(after_slashes.len());
    let (authority, path) = after_slashes.split_at(end);

    let (user_info, host_port) = match authority.rsplit_once('@') {
        Some((user, host)) => (Some(user), host),
        None => (None, authority),
    };

    // Only a colon after the closing bracket separates an IPv6 host from its
    // port -- the address is full of them.
    let colon = match host_port.rfind(']') {
        Some(bracket) => host_port[bracket..].find(':').map(|at| bracket + at),
        None => host_port.rfind(':'),
    };
    let (host, port) = match colon {
        Some(at) => (&host_port[..at], Some(&host_port[at + 1..])),
        None => (host_port, None),
    };

    UriRef {
        scheme,
        user_info,
        host: Some(host),
        port,
        path,
        query,
        fragment,
    }
}

/// Whether `candidate` is a scheme: `ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`.
fn is_scheme(candidate: &str) -> bool {
    let mut chars = candidate.chars();
    chars.next().is_some_and(char::is_alphabetic)
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
}

/// Percent-decode a string the way `java.net.URI` does: `%XX` only, no `+`
/// handling, and `None` on a malformed escape or invalid UTF-8 -- the cases
/// where Java rejects the whole URI.
fn percent_decode_strict(text: &str) -> Option<String> {
    if !text.contains('%') {
        return None;
    }
    let bytes = text.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = text.get(i + 1..i + 3)?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// The extension of a path, or `None` when its last segment has no dot.
///
/// Elasticsearch takes the extension from the segment after the LAST `/` and
/// gives a path with no slash at all none -- `elastic/elasticsearch#105689`, and
/// the corpus agrees: `/api/v2/cmdb/log.fortianalyzer/setting` and the bare
/// `subdomain.domain.tld` both come back without one, `/virus/eicar.com` gives
/// `com`, and `/config/` gives none.
fn path_extension(path: &str) -> Option<&str> {
    let segment = &path[path.rfind('/')?..];
    let dot = segment.rfind('.')?;
    let extension = &segment[dot + 1..];
    (!extension.is_empty()).then_some(extension)
}

/// Write one `kv` pair, APPENDING where the field already holds something.
///
/// Elastic's processor asks `hasField` first and appends when it does, so a key
/// that repeats in the line collects a list. Overwriting kept only the last:
/// checkpoint logs several `match_id` and `rule_action` per connection and we
/// reported one of each.
///
/// # Errors
///
/// Propagates a failure to set or append.
pub fn kv_put(event: &mut Event, path: &str, value: impl AsRef<str>) -> Result<()> {
    let value = value.as_ref();
    if event.get(path).is_some() {
        event.append(path, value)
    } else {
        event.set(path, value)
    }
}

/// Elastic's string processors: `lowercase`, `uppercase` and `trim`.
///
/// Each is an `AbstractStringProcessor` there, and every one of them walks a
/// LIST element by element -- the same rule [`gsub_field`] needed.
///
/// # Errors
///
/// Propagates a failure to set the target.
pub fn map_strings(
    event: &mut Event,
    field: &str,
    target: &str,
    each: impl Fn(&str) -> String,
) -> Result<()> {
    let mapped = match event.get(field) {
        Some(Value::String(text)) => Value::String(each(text)),
        Some(Value::Array(items)) => Value::Array(
            items
                .iter()
                .map(|item| match item {
                    Value::String(text) => Value::String(each(text)),
                    other => other.clone(),
                })
                .collect(),
        ),
        _ => return Ok(()),
    };
    event.set(target, mapped)
}

/// Elastic's `gsub` processor: replace every match in `field` into `target`.
///
/// A LIST is rewritten element by element, which is what Elasticsearch's own
/// processor does and what a string-only reader skipped: mimecast APPENDS the
/// sender address, so the field it then scrubs `<>` out of is a one-element
/// list, and the empty sender survived into `email.from.address`.
///
/// # Errors
///
/// Propagates a failure to set the target.
pub fn gsub_field(
    event: &mut Event,
    field: &str,
    target: &str,
    pattern: &crate::grok_cache::Pattern,
    replacement: &str,
) -> Result<()> {
    let rewrite = |text: &str| Value::String(pattern.replace_all(text, replacement).into_owned());
    let replaced = match event.get(field) {
        Some(Value::String(text)) => rewrite(text),
        Some(Value::Array(items)) => Value::Array(
            items
                .iter()
                .map(|item| match item {
                    Value::String(text) => rewrite(text),
                    other => other.clone(),
                })
                .collect(),
        ),
        _ => return Ok(()),
    };
    event.set(target, replaced)
}

/// Whether `java.net.URI` accepts every character of `text`.
///
/// Its legal set is RFC 2396's unreserved, reserved and escaped, widened to any
/// non-ASCII character that is neither a control nor a space. So a raw space, a
/// control character, one of the excluded ASCII punctuation marks, or a `%` not
/// followed by two hex digits throws `URISyntaxException` -- which is where
/// Elasticsearch drops to the `java.net.URL` fallback below.
fn java_uri_legal(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if !b.is_ascii() {
            // A multi-byte character: only the leading byte is inspected, and
            // every non-ASCII scalar but a control or space char is legal.
            i += 1;
            continue;
        }
        if b == b'%' {
            let hex = bytes.get(i + 1..i + 3);
            if !hex.is_some_and(|h| h.iter().all(u8::is_ascii_hexdigit)) {
                return false;
            }
            i += 3;
            continue;
        }
        if !(b.is_ascii_alphanumeric() || b"-_.!~*'();/?:@&=+$,[]#".contains(&b)) {
            return false;
        }
        i += 1;
    }
    text.chars().all(|c| !c.is_control() && !is_space_char(c))
}

/// Java's `Character.isSpaceChar`: the Unicode space separators.
fn is_space_char(c: char) -> bool {
    c == ' '
        || c == '\u{a0}'
        || c == '\u{1680}'
        || ('\u{2000}'..='\u{200a}').contains(&c)
        || matches!(
            c,
            '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}'
        )
}

/// Whether `java.net.URL` -- what Elasticsearch falls back to when
/// `java.net.URI` refuses the text -- would parse it.
///
/// It is far more forgiving about characters and far less about two things: it
/// needs a protocol the JDK ships a handler for, and it runs the port through a
/// bare `Integer.parseInt`, so a port like `80-` throws out of the constructor
/// and the whole processor fails. Both together are why an ALB log's
/// `http://host:80-/ ` yields NO `url.*` at all.
fn java_url_parses(uri: &UriRef<'_>) -> bool {
    let Some(scheme) = uri.scheme else {
        return false;
    };
    if !matches!(
        scheme.to_ascii_lowercase().as_str(),
        "http" | "https" | "file" | "ftp" | "jar" | "mailto"
    ) {
        return false;
    }
    uri.port
        .is_none_or(|port| port.is_empty() || port.bytes().all(|b| b.is_ascii_digit()))
}

/// Split `field` into ECS `url.*` components under `target`.
///
/// Elastic's `uri_parts` processor. Returns whether anything was written, so
/// the caller can run its `on_failure` block: a value that is not a string, an
/// empty one, or one neither `java.net.URI` nor `java.net.URL` will parse,
/// writes nothing.
///
/// # Errors
///
/// Returns [`crate::TransformError`] if a component cannot be set.
pub fn uri_parts(
    event: &mut Event,
    field: &str,
    target: &str,
    keep_original: bool,
    remove_if_successful: bool,
) -> Result<bool> {
    let Some(original) = event.get_string(field) else {
        return Ok(false);
    };
    if original.is_empty() {
        return Ok(false);
    }

    let uri = split_uri(&original);
    let uri_legal = java_uri_legal(&original);
    if !uri_legal && !java_url_parses(&uri) {
        return Ok(false);
    }
    let mut parts = serde_json::Map::new();

    if let Some(scheme) = uri.scheme {
        parts.insert("scheme".into(), Value::String(scheme.to_owned()));
    }
    if let Some(user_info) = uri.user_info {
        parts.insert("user_info".into(), Value::String(user_info.to_owned()));
        match user_info.split_once(':') {
            Some((username, password)) => {
                parts.insert("username".into(), Value::String(username.to_owned()));
                parts.insert("password".into(), Value::String(password.to_owned()));
            }
            None => {
                parts.insert("username".into(), Value::String(user_info.to_owned()));
            }
        }
    }
    if let Some(host) = uri.host.filter(|h| !h.is_empty()) {
        parts.insert("domain".into(), Value::String(host.to_owned()));
    }
    // A port that is not a number is left out rather than stored as text: the
    // ECS field is numeric and Elastic's processor drops it the same way.
    if let Some(port) = uri.port.and_then(|p| p.parse::<u32>().ok()) {
        parts.insert("port".into(), Value::Number(port.into()));
    }
    if uri.path.is_empty() {
        // Elasticsearch's processor goes through java.net.URI, whose getPath
        // returns the empty string for a URL with an authority and no path --
        // and the processor writes it. Verbatim in the umbrella corpus.
        if uri.host.is_some() {
            parts.insert("path".into(), Value::String(String::new()));
        }
    } else {
        parts.insert("path".into(), Value::String(uri.path.to_owned()));
        if let Some(extension) = path_extension(uri.path) {
            parts.insert("extension".into(), Value::String(extension.to_owned()));
        }
    }
    if let Some(query) = uri.query {
        // java.net.URI's getQuery DECODES percent escapes once, and that is
        // what Elasticsearch emits -- umbrella's double-encoded overwolf URLs
        // carry the proof. The URL fallback decodes nothing, so a text java.net
        // .URI refused keeps its query as written.
        let decoded = if uri_legal {
            percent_decode_strict(query).unwrap_or_else(|| query.to_owned())
        } else {
            query.to_owned()
        };
        parts.insert("query".into(), Value::String(decoded));
    }
    if let Some(fragment) = uri.fragment {
        parts.insert("fragment".into(), Value::String(fragment.to_owned()));
    }
    if keep_original {
        parts.insert("original".into(), Value::String(original.clone()));
    }

    // Elastic replaces the target wholesale with the parsed object, so a
    // scalar sitting there is gone before the parts land. Writing leaf by leaf
    // into a string instead fails on the FIRST leaf and loses every part with
    // it -- which is what happened to `url` on cisco_meraki's security events,
    // where the processor reads and writes the same field.
    if event.get(target).is_some_and(|v| !v.is_object()) {
        event.remove(target);
    }

    // Set leaf by leaf rather than replacing the target: the fortinet pipeline
    // writes `url.domain` from another field before and after this runs, and a
    // wholesale replace would discard it.
    for (key, value) in parts {
        event.set(&format!("{target}.{key}"), value)?;
    }

    if remove_if_successful && field != target {
        event.remove(field);
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- dot_expander ---

    /// A dotted key becomes the nested path it spells, and the others are
    /// carried across untouched.
    #[test]
    fn a_dotted_key_expands_into_objects() {
        let mut event = Event::new(json!({
            "aws.cloudwatch.namespace": "AWS/EC2",
            "aws.ec2.metrics.CPUUtilization.avg": 42,
            "message": "left alone",
        }));
        dot_expand(&mut event, "", "*").unwrap();
        assert_eq!(event.get_str("aws.cloudwatch.namespace"), Some("AWS/EC2"));
        assert_eq!(
            event.get("aws.ec2.metrics.CPUUtilization.avg"),
            Some(&json!(42))
        );
        assert_eq!(event.get_str("message"), Some("left alone"));
        // Nested for real, not still a flat key.
        assert!(event.get("aws").is_some_and(|v| v.is_object()));
    }

    /// Naming ONE field expands that key and leaves the rest flat.
    #[test]
    fn a_named_field_expands_alone() {
        let mut event = Event::new(json!({"a.b": 1, "c.d": 2}));
        dot_expand(&mut event, "", "a.b").unwrap();
        assert_eq!(event.get("a.b"), Some(&json!(1)));
        assert!(event.get("a").is_some_and(|v| v.is_object()));
        assert!(event.as_value().get("c.d").is_some(), "c.d stays flat");
    }

    /// A container with no dotted key is left exactly as it was, without the
    /// deep clone the rebuild would otherwise cost on every event.
    #[test]
    fn a_container_with_no_dotted_key_is_untouched() {
        let before = json!({"message": "plain", "event": {"action": "x"}});
        let mut event = Event::new(before.clone());
        dot_expand(&mut event, "", "*").unwrap();
        assert_eq!(event.as_value(), &before);
    }

    /// Scoped to a path, and a path that is not there is not an error.
    #[test]
    fn a_scoped_expansion_only_touches_its_own_object() {
        let mut event = Event::new(json!({"json": {"a.b": 1}, "top.level": 2}));
        dot_expand(&mut event, "json", "*").unwrap();
        assert_eq!(event.get("json.a.b"), Some(&json!(1)));
        assert!(
            event.as_value().get("top.level").is_some(),
            "the root is out of scope"
        );

        dot_expand(&mut event, "absent", "*").unwrap();
    }

    // --- uri_parts: the java.net.URI behaviours the corpus pinned ---

    /// Verbatim from the umbrella corpus: the raw query is double-encoded
    /// and Elasticsearch emits it decoded ONCE.
    #[test]
    fn a_query_is_percent_decoded_once() {
        let mut event = Event::new(json!({ "src": "http://h/p?Extra=%255b%2522x%2522%255d" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert_eq!(event.get("url.query"), Some(&json!("Extra=%5b%22x%22%5d")));
    }

    /// Verbatim from the cloudfront corpus: `.tld` is not on the Public Suffix
    /// List, and Elasticsearch writes nothing rather than inventing a
    /// registration under it.
    #[test]
    fn an_unlisted_top_level_domain_registers_nothing() {
        assert!(registered_domain_lookup("domain.tld").is_none());
        assert!(registered_domain_lookup("host.invalidtld").is_none());
        assert!(registered_domain_lookup("localhost").is_none());
    }

    /// A two-label PRIVATE entry lands on the same split whichever half of the
    /// list is read, so it stays on the ICANN path: the corpus has
    /// Elasticsearch registering `akamaized.net` under `net`.
    #[test]
    fn a_private_suffix_keeps_the_two_label_split() {
        let result = registered_domain_lookup("static-s-msn-com.akamaized.net").expect("known");
        assert_eq!(result.registered_domain.as_deref(), Some("akamaized.net"));
        assert_eq!(result.top_level_domain, "net");
        assert_eq!(result.subdomain.as_deref(), Some("static-s-msn-com"));
    }

    /// Verbatim from the `microsoft_dnsserver` audit corpus: a DNS query name
    /// arrives uppercase. The list is lowercase and the lookup ignores case,
    /// but the SUBDOMAIN is cut from the name as it arrived -- so the
    /// registered domain, which came off the list, does not end the original
    /// text and no subdomain is written at all.
    #[test]
    fn an_uppercase_name_registers_without_a_subdomain() {
        let result = registered_domain_lookup("B.ROOT-SERVERS.NET").expect("the suffix is listed");
        assert_eq!(
            result.registered_domain.as_deref(),
            Some("root-servers.net")
        );
        assert_eq!(result.top_level_domain, "net");
        assert_eq!(result.subdomain, None);

        // The same name in lowercase does get one.
        let lower = registered_domain_lookup("b.root-servers.net").expect("the suffix is listed");
        assert_eq!(lower.registered_domain.as_deref(), Some("root-servers.net"));
        assert_eq!(lower.subdomain.as_deref(), Some("b"));
    }

    /// A PRIVATE entry of three labels or more IS the registered domain, with
    /// its own first label peeled off as the top-level domain. Both cases are
    /// verbatim from the route53 corpus.
    #[test]
    fn a_long_private_suffix_is_the_registered_domain() {
        let short = registered_domain_lookup("ec2-instance-connect.us-east-1.amazonaws.com")
            .expect("known");
        assert_eq!(
            short.registered_domain.as_deref(),
            Some("us-east-1.amazonaws.com")
        );
        assert_eq!(short.top_level_domain, "amazonaws.com");
        assert_eq!(short.subdomain.as_deref(), Some("ec2-instance-connect"));

        let long = registered_domain_lookup(
            "amazonlinux-2-repos-us-east-1.s3.dualstack.us-east-1.amazonaws.com",
        )
        .expect("known");
        assert_eq!(
            long.registered_domain.as_deref(),
            Some("s3.dualstack.us-east-1.amazonaws.com")
        );
        assert_eq!(long.top_level_domain, "dualstack.us-east-1.amazonaws.com");
        assert_eq!(
            long.subdomain.as_deref(),
            Some("amazonlinux-2-repos-us-east-1")
        );
    }

    /// An ICANN suffix IS honoured whole -- reverse DNS is the case that needs
    /// it, and `co.uk` is the same shape.
    #[test]
    fn an_icann_suffix_is_honoured_whole() {
        let reverse = registered_domain_lookup("211.52.31.172.in-addr.arpa").expect("known");
        assert_eq!(
            reverse.registered_domain.as_deref(),
            Some("172.in-addr.arpa")
        );
        assert_eq!(reverse.top_level_domain, "in-addr.arpa");
        assert_eq!(reverse.subdomain.as_deref(), Some("211.52.31"));

        let uk = registered_domain_lookup("a.example.co.uk").expect("known");
        assert_eq!(uk.registered_domain.as_deref(), Some("example.co.uk"));
        assert_eq!(uk.top_level_domain, "co.uk");
        assert_eq!(uk.subdomain.as_deref(), Some("a"));
    }

    #[test]
    fn a_two_label_name_has_no_subdomain() {
        let result = registered_domain_lookup("example.com").expect("a known suffix");
        assert_eq!(result.registered_domain.as_deref(), Some("example.com"));
        assert_eq!(result.top_level_domain, "com");
        assert_eq!(result.subdomain, None);
    }

    /// A malformed escape fails java.net.URI wholesale, so the raw text is
    /// what the java.net.URL fallback -- which decodes nothing -- keeps.
    #[test]
    fn a_query_that_will_not_decode_is_kept_raw() {
        let mut event = Event::new(json!({ "src": "http://h/p?bad=%ZZ" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert_eq!(event.get("url.query"), Some(&json!("bad=%ZZ")));
    }

    /// Verbatim from the elb corpus: a space fails java.net.URI and the `80-`
    /// port fails the java.net.URL fallback's `Integer.parseInt`, so
    /// Elasticsearch writes NO url parts at all.
    #[test]
    fn a_url_neither_java_parser_takes_writes_nothing() {
        let mut event =
            Event::new(json!({ "src": "http://internal-service-alb.example.com:80-/ " }));
        assert!(!uri_parts(&mut event, "src", "url", true, false).unwrap());
        assert!(event.get("url.scheme").is_none());
        assert!(event.get("url.original").is_none());

        // The space alone still goes through the URL fallback.
        let mut spaced = Event::new(json!({ "src": "http://h/a b" }));
        assert!(uri_parts(&mut spaced, "src", "url", false, false).unwrap());
        assert_eq!(spaced.get("url.path"), Some(&json!("/a b")));
    }

    /// A URL with an authority and no path gets `path: ""`, which is what
    /// java.net.URI's getPath returns and the processor writes.
    #[test]
    fn a_pathless_url_with_a_host_writes_an_empty_path() {
        let mut event = Event::new(json!({ "src": "http://example.com?q=1" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert_eq!(event.get("url.path"), Some(&json!("")));

        // A pure relative reference with no path still writes nothing.
        let mut event = Event::new(json!({ "src": "?q=1" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert_eq!(event.get("url.path"), None);
    }

    // --- foreach_array ---

    /// The happy path: every element moves through `_ingest._value`, the
    /// mutated list lands back on the field, and `_ingest` is gone.
    #[test]
    fn foreach_array_rebuilds_the_field_from_the_mutated_elements() {
        let mut event = Event::new(json!({ "tags": ["a", "b"] }));
        foreach_array(&mut event, "tags", |event| {
            let text = event.get_str("_ingest._value").unwrap().to_uppercase();
            event.set("_ingest._value", text)
        })
        .unwrap();
        assert_eq!(event.as_value(), &json!({ "tags": ["A", "B"] }));
    }

    /// A MAP is a foreach too: the key and the value are both exposed, and the
    /// map is rebuilt from what the body leaves in them. mimecast's attachment
    /// hashes arrive that way, and skipping the map lost every one.
    #[test]
    fn foreach_array_walks_a_map_by_key_and_value() {
        let mut event = Event::new(json!({ "hash": { "sha1": "aa", "md5": "bb" } }));
        let mut seen: Vec<String> = Vec::new();
        foreach_array(&mut event, "hash", |event| {
            seen.push(event.get_str("_ingest._key").unwrap().to_string());
            let text = event.get_str("_ingest._value").unwrap().to_uppercase();
            event.set("_ingest._value", text)
        })
        .unwrap();

        assert_eq!(
            event.as_value(),
            &json!({ "hash": { "sha1": "AA", "md5": "BB" } })
        );
        assert_eq!(seen, ["sha1", "md5"]);
    }

    /// A value that is not an array is a no-op, exactly as the inline
    /// `if let Some(Value::Array(..))` was -- and it is NOT removed.
    #[test]
    fn foreach_array_leaves_a_non_array_alone() {
        let mut event = Event::new(json!({ "tags": "scalar" }));
        foreach_array(&mut event, "tags", |_| {
            panic!("the body must not run");
        })
        .unwrap();
        assert_eq!(event.as_value(), &json!({ "tags": "scalar" }));
    }

    /// A body that fails part-way leaves the processed prefix, the failing
    /// element -- still exposed at `_ingest._value` -- and the untouched
    /// rest, all back in the field.
    #[test]
    fn foreach_array_restores_the_field_when_the_body_fails() {
        let mut event = Event::new(json!({ "tags": ["a", "bad", "c"] }));
        let error = foreach_array(&mut event, "tags", |event| {
            let text = event.get_str("_ingest._value").unwrap().to_string();
            if text == "bad" {
                return Err(crate::TransformError::FieldNotFound { path: text });
            }
            event.set("_ingest._value", text.to_uppercase())
        })
        .unwrap_err();
        assert!(error.to_string().contains("bad"));
        assert_eq!(event.get("tags"), Some(&json!(["A", "bad", "c"])));
        assert_eq!(event.get("_ingest._value"), Some(&json!("bad")));
    }

    /// An empty array still clears `_ingest` and writes the field back,
    /// which is what the inline loop did.
    #[test]
    fn foreach_array_handles_an_empty_array() {
        let mut event = Event::new(json!({ "tags": [], "_ingest": { "old": 1 } }));
        foreach_array(&mut event, "tags", |_| Ok(())).unwrap();
        assert_eq!(event.as_value(), &json!({ "tags": [] }));
    }

    // --- parse_json_field ---

    /// The SIMD path and `serde_json` agree on what a document MEANS --
    /// nesting, numbers at both integer extremes, floats, escapes, unicode.
    #[test]
    fn simd_parse_agrees_with_serde_json() {
        for text in [
            r#"{"a": {"b": [1, 2.5, -3, 18446744073709551615, -9223372036854775808]}}"#,
            r#"{"s": "line\nbreak \"quoted\" é", "t": true, "n": null}"#,
            r#"[{"k": "v"}, [], {}, ""]"#,
            "42",
            r#""bare string""#,
        ] {
            let via_simd = parse_json_str(text).unwrap();
            let via_serde: Value = serde_json::from_str(text).unwrap();
            assert_eq!(via_simd, via_serde, "input: {text}");
        }
    }

    /// An absent field is a no-op, which is what the old inline `if let` did.
    #[test]
    fn parse_json_field_skips_an_absent_field() {
        let mut event = Event::new(json!({ "other": 1 }));
        parse_json_field(&mut event, "message", "json").unwrap();
        assert_eq!(event.as_value(), &json!({ "other": 1 }));
    }

    /// A field that will not parse is the processor's failure, naming the
    /// field and keeping the wording the generated `on_failure` blocks wrote.
    #[test]
    fn parse_json_field_fails_with_the_inline_blocks_wording() {
        let mut event = Event::new(json!({ "message": "{not json" }));
        let err = parse_json_field(&mut event, "message", "json").unwrap_err();
        let text = err.to_string();
        assert!(text.contains("message"), "{text}");
        assert!(text.contains("failed to parse JSON:"), "{text}");
    }

    /// The parsed object lands whole on the target.
    #[test]
    fn parse_json_field_writes_the_target() {
        let mut event = Event::new(json!({ "message": r#"{"a": [1, 2]}"# }));
        parse_json_field(&mut event, "message", "json").unwrap();
        assert_eq!(event.get("json"), Some(&json!({ "a": [1, 2] })));
    }

    /// Verbatim from `proofpoint_on_demand/message`, which converts a LIST of
    /// suborg recipients to string. Rendering the array whole gave the string
    /// `["0"]`, which the pipeline's own sentinel pass does not recognise.
    #[test]
    fn convert_walks_an_array_element_by_element() {
        assert_eq!(
            convert_value(&json!([0, 1]), "string").unwrap(),
            json!(["0", "1"])
        );
        assert_eq!(
            convert_value(&json!(["7", "8"]), "long").unwrap(),
            json!([7, 8])
        );
    }

    /// One bad member fails the whole conversion, which is where Elastic
    /// throws and the processor's `on_failure` runs.
    #[test]
    fn one_unconvertible_member_fails_the_array() {
        assert!(convert_value(&json!(["7", "not a number"]), "long").is_err());
    }

    /// Verbatim from the compat corpus: `m365_defender/event/test-device`
    /// carries both the fingerprint's input and the digest Elasticsearch
    /// wrote, so this pins the encoding rather than describing it.
    #[test]
    fn the_default_fingerprint_is_base64_sha1_over_nul_delimited_values() {
        let value = json!("4248|2022-11-07T17:07:41.698Z|de6509d550e605faf3bbeac0905ab9590fe12345");
        assert_eq!(
            fingerprint_default(std::slice::from_ref(&value)),
            "utLjuzbrOqM8u+fh65n5nL10vuE="
        );
    }

    /// A string is hashed as its own text. Hashing its JSON would fold the
    /// quotes into the digest and nothing would ever match.
    #[test]
    fn a_string_is_fingerprinted_without_its_quotes() {
        assert_eq!(
            fingerprint_default(&[json!("4248")]),
            fingerprint_default(&[json!(4248)]),
        );
    }

    /// Verbatim from `tests/fixtures/cisco/umbrella`: a space before the quote
    /// of a field that itself holds commas.
    #[test]
    fn a_gap_before_a_quote_closes_so_the_field_reads_as_quoted() {
        let line = r#""2015-01-16 17:48:41","AD", "AD,ADSite,Network", "10.10.1.100""#;
        let closed = csv_close_quote_gap(line, ',', '"');
        assert_eq!(
            closed,
            r#""2015-01-16 17:48:41","AD","AD,ADSite,Network","10.10.1.100""#
        );
    }

    /// Whitespace that is part of a value is content, not a gap.
    #[test]
    fn whitespace_inside_a_field_is_left_alone() {
        for line in [
            r#""a"," spaced value ","b""#,
            "a, spaced value ,b",
            r#""a", b c,"d""#,
        ] {
            assert_eq!(csv_close_quote_gap(line, ',', '"'), line, "{line}");
        }
    }

    /// A line with nothing to close is returned borrowed.
    #[test]
    fn a_line_with_no_gap_is_not_reallocated() {
        let line = r#""a","b","c""#;
        assert!(matches!(
            csv_close_quote_gap(line, ',', '"'),
            std::borrow::Cow::Borrowed(_)
        ));
    }

    /// A delimiter inside a quoted field does not open a new one.
    #[test]
    fn a_quoted_delimiter_does_not_start_a_field() {
        let line = r#""a,  b", "c""#;
        assert_eq!(csv_close_quote_gap(line, ',', '"'), r#""a,  b","c""#);
    }

    // --- resolve_path ---

    // --- duplicate grok capture names ---

    /// Verbatim from `pipelines/cisco/ios/default.yml`. Java takes one field
    /// name in several alternation branches; Rust's engine rejects a duplicate
    /// group name outright, so the whole pattern failed to compile and the
    /// `BADAUTH` grok matched nothing on eighteen events.
    #[test]
    fn a_field_named_twice_compiles_and_both_branches_write_it() {
        let pattern = r"from %{DATA:source.address}(\(%{INT:source.port}\)|\:%{INT:source.port})";
        let (expanded, field_map, _) = grok_to_regex_typed(pattern);

        let re = regex::Regex::new(&expanded).expect("a repeated field name still compiles");
        assert_eq!(
            field_map.values().filter(|p| *p == "source.port").count(),
            2,
            "both groups must point at the same field"
        );

        for input in ["from 192.168.0.1(64999)", "from 192.168.0.1:64999"] {
            let caps = re.captures(input).unwrap_or_else(|| panic!("{input}"));
            let port = field_map
                .iter()
                .filter(|(_, path)| *path == "source.port")
                .find_map(|(group, _)| caps.name(group))
                .unwrap_or_else(|| panic!("{input}"));
            assert_eq!(port.as_str(), "64999", "{input}");
        }
    }

    /// Verbatim from `pipelines/cisco/meraki/events.yml`, where the subtree a
    /// rename reads is named by the event's own subtype.
    #[test]
    fn resolve_path_names_a_field_from_the_document() {
        let event = Event::new(json!({
            "cisco_meraki": { "event_subtype": "disassociation" },
        }));

        assert_eq!(
            resolve_path(
                &event,
                "cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac"
            )
            .as_deref(),
            Some("cisco_meraki.disassociation.client_mac")
        );
    }

    /// A reference that resolves to nothing would build a path with an empty
    /// segment, which matches no field -- so it is no path at all.
    #[test]
    fn resolve_path_refuses_an_unresolvable_reference() {
        let event = Event::new(json!({ "cisco_meraki": { "event_subtype": "" } }));
        assert_eq!(
            resolve_path(
                &event,
                "cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac"
            ),
            None
        );

        let empty = Event::new(json!({}));
        assert_eq!(resolve_path(&empty, "a.{{{missing}}}.b"), None);
    }

    /// A path with no reference in it is itself, and costs nothing to ask for.
    #[test]
    fn resolve_path_passes_a_plain_path_through() {
        let event = Event::new(json!({}));
        assert_eq!(
            resolve_path(&event, "client.mac").as_deref(),
            Some("client.mac")
        );
    }

    /// Mustache spells an escaped reference with two braces and an unescaped
    /// one with three; a field path wants the value either way.
    #[test]
    fn resolve_path_reads_both_brace_forms() {
        let event = Event::new(json!({ "kind": "assoc" }));
        assert_eq!(
            resolve_path(&event, "a.{{kind}}.b").as_deref(),
            Some("a.assoc.b")
        );
        assert_eq!(
            resolve_path(&event, "a.{{{kind}}}.b").as_deref(),
            Some("a.assoc.b")
        );
    }

    // --- uri_parts ---

    /// Every one of these is a real fortinet input paired with what
    /// Elasticsearch 9.2.2 produced for it, taken from `testdata/compat`.
    #[test]
    fn uri_parts_matches_the_captured_elasticsearch_output() {
        let cases: &[(&str, Value)] = &[
            ("/config/", json!({ "path": "/config/" })),
            ("/", json!({ "path": "/" })),
            (
                "http://172.16.200.55/virus/eicar.com",
                json!({
                    "scheme": "http",
                    "domain": "172.16.200.55",
                    "path": "/virus/eicar.com",
                    "extension": "com",
                }),
            ),
            (
                "/ips/sig1.pdf",
                json!({ "path": "/ips/sig1.pdf", "extension": "pdf" }),
            ),
            (
                "/api/v2/monitor/system/usb-log?vdom=root",
                json!({ "path": "/api/v2/monitor/system/usb-log", "query": "vdom=root" }),
            ),
            // The dotted DIRECTORY must not become an extension.
            (
                "/api/v2/cmdb/log.fortianalyzer/setting?vdom=root",
                json!({ "path": "/api/v2/cmdb/log.fortianalyzer/setting", "query": "vdom=root" }),
            ),
            // Neither must a bare hostname, which java.net.URI reads as a
            // relative PATH. m365_defender and zscaler both ship these.
            (
                "subdomain.domain.tld",
                json!({ "path": "subdomain.domain.tld" }),
            ),
            ("url.com", json!({ "path": "url.com" })),
            (
                "https://172.16.200.88/dlp/files/fortiauto.pdf",
                json!({
                    "scheme": "https",
                    "domain": "172.16.200.88",
                    "path": "/dlp/files/fortiauto.pdf",
                    "extension": "pdf",
                }),
            ),
        ];

        for (input, expected) in cases {
            let mut event = Event::new(json!({ "src": input }));
            assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
            assert_eq!(event.get("url"), Some(expected), "input: {input}");
        }
    }

    #[test]
    fn uri_parts_splits_an_authority_with_credentials_and_a_port() {
        let mut event =
            Event::new(json!({ "src": "https://bob:hunter2@example.com:8443/a?b=1#c" }));
        assert!(uri_parts(&mut event, "src", "url", true, false).unwrap());

        assert_eq!(event.get_str("url.scheme"), Some("https"));
        assert_eq!(event.get_str("url.domain"), Some("example.com"));
        assert_eq!(event.get("url.port"), Some(&json!(8443)));
        assert_eq!(event.get_str("url.user_info"), Some("bob:hunter2"));
        assert_eq!(event.get_str("url.username"), Some("bob"));
        assert_eq!(event.get_str("url.password"), Some("hunter2"));
        assert_eq!(event.get_str("url.path"), Some("/a"));
        assert_eq!(event.get_str("url.query"), Some("b=1"));
        assert_eq!(event.get_str("url.fragment"), Some("c"));
        assert_eq!(
            event.get_str("url.original"),
            Some("https://bob:hunter2@example.com:8443/a?b=1#c")
        );
    }

    /// An IPv6 authority is full of colons, so only the one after the closing
    /// bracket separates the port.
    #[test]
    fn uri_parts_keeps_an_ipv6_host_whole() {
        let mut event = Event::new(json!({ "src": "http://[2001:db8::1]:8080/x" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());

        assert_eq!(event.get_str("url.domain"), Some("[2001:db8::1]"));
        assert_eq!(event.get("url.port"), Some(&json!(8080)));

        let mut event = Event::new(json!({ "src": "http://[2001:db8::1]/x" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert_eq!(event.get_str("url.domain"), Some("[2001:db8::1]"));
        assert_eq!(event.get("url.port"), None);
    }

    /// The caller runs its `on_failure` on a false return, so a value that is
    /// not a usable string must say so rather than writing an empty subtree.
    #[test]
    fn uri_parts_reports_what_it_could_not_use() {
        let mut event = Event::new(json!({ "src": "", "num": 7 }));
        assert!(!uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert!(!uri_parts(&mut event, "num", "url", false, false).unwrap());
        assert!(!uri_parts(&mut event, "absent", "url", false, false).unwrap());
        assert!(!event.has("url"));
    }

    /// The fortinet pipeline writes `url.domain` from the hostname field and
    /// then parses a path-only `url` over the top, so the parse must add to the
    /// subtree rather than replace it.
    #[test]
    fn uri_parts_adds_to_the_target_rather_than_replacing_it() {
        let mut event = Event::new(json!({ "src": "/config/", "url": { "domain": "elastic.co" } }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());

        assert_eq!(event.get_str("url.domain"), Some("elastic.co"));
        assert_eq!(event.get_str("url.path"), Some("/config/"));
    }

    /// `cisco_meraki`'s security events parse `url` INTO `url`. A string sitting
    /// on the target has to go first -- writing `url.scheme` into a string
    /// fails on that first leaf and every other part is lost with it, which is
    /// how the event ended up with a scalar `url` and no parts at all.
    #[test]
    fn uri_parts_replaces_a_scalar_sitting_on_the_target() {
        let mut event = Event::new(json!({ "url": "http://www.eicar.org/download/eicar.com.txt" }));
        assert!(uri_parts(&mut event, "url", "url", true, false).unwrap());

        assert_eq!(
            event.get_str("url.original"),
            Some("http://www.eicar.org/download/eicar.com.txt")
        );
        assert_eq!(event.get_str("url.scheme"), Some("http"));
        assert_eq!(event.get_str("url.domain"), Some("www.eicar.org"));
        assert_eq!(event.get_str("url.path"), Some("/download/eicar.com.txt"));
        assert_eq!(event.get_str("url.extension"), Some("txt"));
    }

    // --- is_internal_ip ---

    #[test]
    fn internal_ip_rfc1918_class_a() {
        assert!(is_internal_ip("10.0.0.1"));
        assert!(is_internal_ip("10.255.255.255"));
    }

    #[test]
    fn internal_ip_rfc1918_class_b() {
        assert!(is_internal_ip("172.16.0.1"));
        assert!(is_internal_ip("172.31.255.255"));
        assert!(!is_internal_ip("172.32.0.1"));
    }

    #[test]
    fn internal_ip_rfc1918_class_c() {
        assert!(is_internal_ip("192.168.0.1"));
        assert!(is_internal_ip("192.168.255.255"));
    }

    #[test]
    fn internal_ip_loopback() {
        assert!(is_internal_ip("127.0.0.1"));
        assert!(is_internal_ip("::1"));
    }

    #[test]
    fn internal_ip_link_local() {
        assert!(is_internal_ip("169.254.0.1"));
    }

    #[test]
    fn internal_ip_public() {
        assert!(!is_internal_ip("8.8.8.8"));
        assert!(!is_internal_ip("175.16.199.1"));
        assert!(!is_internal_ip("1.1.1.1"));
    }

    #[test]
    fn internal_ip_invalid() {
        assert!(!is_internal_ip("not-an-ip"));
        assert!(!is_internal_ip(""));
    }

    // --- registered_domain_lookup ---

    #[test]
    fn registered_domain_simple() {
        let r = registered_domain_lookup("www.example.com").unwrap();
        assert_eq!(r.registered_domain.as_deref(), Some("example.com"));
        assert_eq!(r.top_level_domain, "com");
        assert_eq!(r.subdomain.as_deref(), Some("www"));
    }

    #[test]
    fn registered_domain_empty() {
        assert!(registered_domain_lookup("").is_none());
    }

    // --- grok_to_regex ---

    #[test]
    fn grok_simple_ip_field() {
        let regex = grok_to_regex("%{IP:source.ip}");
        assert!(regex.contains("(?P<source_ip>"));
    }

    #[test]
    fn grok_field_map_restores_dots() {
        let (_, map) = grok_to_regex_with_map("%{USER:user.name}");
        assert_eq!(map.get("user_name").unwrap(), "user.name");
    }

    #[test]
    fn grok_no_field() {
        let regex = grok_to_regex("%{NOTSPACE}");
        assert!(regex.contains(r"\S+"));
        assert!(!regex.contains("(?P<"));
    }

    #[test]
    fn grok_multiple_patterns() {
        let regex = grok_to_regex("%{IP:src}:%{POSINT:port}");
        assert!(regex.contains("(?P<src>"));
        assert!(regex.contains("(?P<port>"));
    }

    #[test]
    fn grok_unknown_pattern_fallback() {
        let regex = grok_to_regex("%{UNKNOWN_THING:field}");
        assert!(regex.contains(".+?")); // fallback
    }

    // --- parse_user_agent ---

    #[test]
    fn parse_ua_returns_ok() {
        let result = parse_user_agent("Mozilla/5.0 (Windows NT 10.0) Chrome/91.0");
        assert!(result.is_ok());
        let ua = result.unwrap();
        assert_eq!(ua.name.as_deref(), Some("Chrome"));
    }

    #[test]
    fn parse_ua_empty() {
        let result = parse_user_agent("");
        assert!(result.is_ok());
    }

    // --- painless_exec ---

    #[test]
    fn painless_exec_unknown_script_noop() {
        let _guard = crate::painless_stats::serialised();
        let mut event = Event::new(json!({"field": "value"}));
        let result = painless_exec(&mut event, "unknown_script_that_does_nothing();");
        assert!(result.is_ok());
        // Field should be unchanged
        assert_eq!(event.get_str("field"), Some("value"));
    }

    #[test]
    fn painless_exec_drop_empty_known() {
        let _guard = crate::painless_stats::serialised();
        let mut event = Event::new(json!({"a": "", "b": "keep", "c": null}));
        let result = painless_exec(
            &mut event,
            r#"boolean drop(Object o) { if (o == null || o == "") { return true; } }"#,
        );
        assert!(result.is_ok());
    }

    // --- community_id_v1 ---

    #[test]
    fn community_id_tcp() {
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 1234, 80, "tcp");
        assert!(result.is_ok());
        let id = result.unwrap();
        assert!(id.starts_with("1:"));
    }

    #[test]
    fn community_id_invalid_ip() {
        let result = community_id_v1("not-an-ip", "5.6.7.8", 1234, 80, "tcp");
        assert!(result.is_err());
    }

    // --- geoip_lookup ---

    #[test]
    fn geoip_lookup_no_db() {
        // Reaches the process-global cache, so it takes the same lock the
        // enrichment tests do.
        let _guard = crate::enrichment::geoip_global::test_guard();
        // Without MMDB files loaded, should return empty map (not panic)
        let result = geoip_lookup("geoip_city", "8.8.8.8");
        assert!(result.is_ok());
    }

    // --- grok edge cases ---

    #[test]
    fn grok_type_suffix_stripped() {
        // Elastic grok uses %{IP:field:type} — the :type must be stripped
        let (regex, map) = grok_to_regex_with_map("%{IP:source.ip:ip}");
        assert!(regex.contains("(?P<source_ip>"));
        assert!(!regex.contains(":ip"));
        assert_eq!(map.get("source_ip").unwrap(), "source.ip");
    }

    #[test]
    fn grok_empty_pattern() {
        let regex = grok_to_regex("");
        assert_eq!(regex, "");
    }

    #[test]
    fn grok_literal_only() {
        let regex = grok_to_regex("hello world");
        assert_eq!(regex, "hello world");
    }

    #[test]
    fn grok_consecutive_patterns() {
        let regex = grok_to_regex("%{IP:src}%{POSINT:port}");
        assert!(regex.contains("(?P<src>"));
        assert!(regex.contains("(?P<port>"));
    }

    #[test]
    fn grok_long_type_suffix() {
        let (regex, map) = grok_to_regex_with_map("%{NUMBER:count:long}");
        assert!(regex.contains("(?P<count>"));
        assert!(!regex.contains(":long"));
        assert_eq!(map.get("count").unwrap(), "count");
    }

    // --- Boundary value tests ---

    #[test]
    fn internal_ip_boundary_first_last() {
        assert!(is_internal_ip("10.0.0.0"));
        assert!(is_internal_ip("10.255.255.255"));
        assert!(is_internal_ip("192.168.0.0"));
        assert!(is_internal_ip("192.168.255.255"));
        assert!(!is_internal_ip("0.0.0.0")); // not private
        assert!(!is_internal_ip("255.255.255.255")); // broadcast
    }

    #[test]
    fn community_id_boundary_ips() {
        // Loopback
        let result = community_id_v1("127.0.0.1", "127.0.0.1", 80, 80, "tcp");
        assert!(result.is_ok());
        // IPv6 loopback
        let result = community_id_v1("::1", "::1", 80, 80, "tcp");
        assert!(result.is_ok());
        // Zero port
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 0, 0, "icmp");
        assert!(result.is_ok());
        // Max port
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 65535, 65535, "tcp");
        assert!(result.is_ok());
    }

    #[test]
    fn community_id_unknown_protocol_errors() {
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 80, 80, "unknown_protocol");
        // Unknown protocols are rejected, not defaulted — correct behaviour
        assert!(result.is_err());
    }

    #[test]
    fn registered_domain_deeply_nested() {
        let r = registered_domain_lookup("deep.sub.example.com").unwrap();
        assert_eq!(r.registered_domain.as_deref(), Some("example.com"));
        assert_eq!(r.subdomain.as_deref(), Some("deep.sub"));
    }

    #[test]
    fn grok_regex_actually_matches() {
        // Verify the compiled regex can actually match input
        let regex_str = grok_to_regex("^%{IP:src}:%{POSINT:port}$");
        let re = regex::Regex::new(&regex_str).expect("regex should compile");
        let caps = re.captures("10.0.0.1:8080").expect("should match");
        assert_eq!(caps.name("src").unwrap().as_str(), "10.0.0.1");
        assert_eq!(caps.name("port").unwrap().as_str(), "8080");
    }

    #[test]
    fn grok_regex_rejects_non_matching() {
        let regex_str = grok_to_regex("^%{IP:src}$");
        let re = regex::Regex::new(&regex_str).expect("regex should compile");
        assert!(re.captures("not-an-ip").is_none());
        assert!(re.captures("").is_none());
    }

    #[test]
    fn parse_ua_returns_other_for_unknown() {
        let result = parse_user_agent("SomeRandomBot/1.0").unwrap();
        assert_eq!(result.name.as_deref(), Some("Other"));
    }

    // --- Mutation resistance test ---
    // If you comment out the type-suffix stripping in grok_to_regex_with_map,
    // this test MUST fail (verifies the test catches the bug)
    #[test]
    fn grok_type_suffix_compiles_as_valid_regex() {
        let (regex, _) = grok_to_regex_with_map("%{IP:source.ip:ip}:%{POSINT:port:long}");
        // This MUST compile — if type suffix isn't stripped, it won't
        let re = regex::Regex::new(&regex);
        assert!(
            re.is_ok(),
            "grok regex with type suffixes should compile: {:?}",
            re.err()
        );
    }
}
