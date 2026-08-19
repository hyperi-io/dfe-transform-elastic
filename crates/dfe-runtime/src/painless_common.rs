// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Common Painless script patterns implemented in Rust.
//!
//! The same handful of script shapes recur across the Elastic pipelines --
//! drop-empty, snake-case keys, sum both directions -- so they are written
//! once here rather than once per source. [`try_known_painless`] matches a
//! script against them and runs the Rust equivalent.

use std::borrow::Cow;

use serde_json::{Map, Value, json};

use crate::error::Result;
use crate::event::Event;
use crate::painless_helpers::{SnakeRule, to_snake_case};

/// A script's text with its JSON escapes resolved.
///
/// The matchers below all scan this rather than the raw literal. Borrowing
/// when there is nothing to resolve is what makes [`crate::cached_script`]
/// worth having: the macro resolves the escapes once per call site, so every
/// event after the first takes the borrow and allocates nothing.
pub fn normalise(script: &str) -> Cow<'_, str> {
    if script.contains("\\n") || script.contains("\\\"") {
        Cow::Owned(script.replace("\\n", "\n").replace("\\\"", "\""))
    } else {
        Cow::Borrowed(script)
    }
}

/// A Painless script with its escapes resolved once per CALL SITE.
///
/// The script is a literal that never changes, but `painless_exec` was
/// re-resolving its escapes on every event -- two allocations over the whole
/// script text, per script, per event. Resolving at the site makes
/// [`normalise`] a borrow from then on. Same shape as [`crate::cached_grok`].
#[macro_export]
macro_rules! cached_script {
    ($script:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        SITE.get_or_init(|| $crate::painless_common::normalise($script).into_owned())
            .as_str()
    }};
}

/// Recursively drop null and empty values from the event.
///
/// This is the most common Painless script across all Elastic pipelines:
/// ```painless
/// boolean drop(Object o) {
///   if (o == null || o == "") return true;
///   if (o instanceof Map) { ((Map) o).values().removeIf(v -> drop(v)); return ((Map) o).size() == 0; }
///   if (o instanceof List) { ((List) o).removeIf(v -> drop(v)); return ((List) o).length == 0; }
///   return false;
/// }
/// drop(ctx);
/// ```
pub fn drop_empty_recursive(event: &mut Event) {
    let inner = event.as_value_mut();
    drop_value(inner);
}

fn drop_value(value: &mut Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) if s.is_empty() => true,
        Value::Object(map) => {
            let keys_to_remove: Vec<String> = map
                .iter_mut()
                .filter_map(|(k, v)| if drop_value(v) { Some(k.clone()) } else { None })
                .collect();
            for key in keys_to_remove {
                map.remove(&key);
            }
            map.is_empty()
        }
        Value::Array(arr) => {
            arr.retain_mut(|v| !drop_value(v));
            arr.is_empty()
        }
        _ => false,
    }
}

/// Convert a Painless `keys_to_snake_case` operation.
///
/// Converts camelCase JSON object keys to `snake_case` recursively.
/// Common in Okta and other pipelines for normalising field names.
pub fn keys_to_snake_case(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let entries: Vec<(String, Value)> = map
                .iter()
                .map(|(k, v)| {
                    let snake = to_snake_case(k, SnakeRule::BeforeEveryUpper);
                    let mut v = v.clone();
                    keys_to_snake_case(&mut v);
                    (snake, v)
                })
                .collect();
            map.clear();
            for (k, v) in entries {
                map.insert(k, v);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                keys_to_snake_case(v);
            }
        }
        _ => {}
    }
}

/// Extract process fields from a command line string.
///
/// Sets: `process.command_line`, process.args, process.executable
pub fn extract_process_fields(
    event: &mut Event,
    cmd_field: &str,
    target_prefix: &str,
) -> Result<()> {
    let cmd = match event.get_string(cmd_field) {
        Some(c) if !c.trim().is_empty() => c,
        _ => return Ok(()),
    };

    let trimmed = cmd.trim();
    let args: Vec<&str> = trimmed
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect();

    event.set(&format!("{target_prefix}.command_line"), json!(trimmed))?;
    event.set(&format!("{target_prefix}.args"), json!(args))?;
    if let Some(exe) = args.first() {
        event.set(&format!("{target_prefix}.executable"), json!(exe))?;
    }

    Ok(())
}

/// Convert an epoch timestamp to ISO8601 string and set on event.
///
/// Auto-detects epoch precision by magnitude (ported from dfe-loader):
/// - > 1e18 → nanoseconds
/// - > 1e15 → microseconds
/// - > 1e12 → milliseconds
/// - else   → seconds
pub fn epoch_to_timestamp(event: &mut Event, source_field: &str, target_field: &str) -> Result<()> {
    let epoch = match event.get(source_field) {
        Some(Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(Value::String(s)) => s.parse::<f64>().unwrap_or(0.0),
        _ => return Ok(()),
    };

    if epoch <= 0.0 {
        return Ok(());
    }

    let (secs, nanos) = if epoch > 1e18 {
        ((epoch / 1e9) as i64, ((epoch % 1e9) as u32))
    } else if epoch > 1e15 {
        ((epoch / 1e6) as i64, (((epoch % 1e6) * 1000.0) as u32))
    } else if epoch > 1e12 {
        ((epoch / 1e3) as i64, (((epoch % 1e3) * 1_000_000.0) as u32))
    } else {
        (epoch as i64, ((epoch.fract() * 1e9) as u32))
    };

    if let Some(dt) = chrono::DateTime::from_timestamp(secs, nanos) {
        // Use millisecond precision format matching Elastic convention
        event.set(
            target_field,
            json!(dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()),
        )?;
    }

    Ok(())
}

/// The ECS field a `ctx.source.X + ctx.destination.X` script totals into.
///
/// Both `bytes` and `packets` appear verbatim across the network sources.
fn sum_of_directions(script: &str) -> Option<&'static str> {
    for unit in ["bytes", "packets"] {
        let target = format!("ctx.network.{unit}");
        if script.contains(&target)
            && script.contains(&format!("ctx.source.{unit}"))
            && script.contains(&format!("ctx.destination.{unit}"))
        {
            return Some(if unit == "bytes" { "bytes" } else { "packets" });
        }
    }
    None
}

/// `network.{unit} = source.{unit} + destination.{unit}`.
///
/// Elastic's script would throw on a missing side; skipping instead is the
/// behaviour the surrounding pipeline already relies on.
///
/// The addition saturates. Both operands come off the wire, so a vendor that
/// reports a nonsense byte count must not panic a debug build or wrap to a
/// negative total in a release one.
fn try_sum_directions(event: &mut Event, unit: &str) -> bool {
    let Some(source) = event.get_i64(&format!("source.{unit}")) else {
        return true;
    };
    let Some(destination) = event.get_i64(&format!("destination.{unit}")) else {
        return true;
    };
    let total = source.saturating_add(destination);
    let _ = event.set(&format!("network.{unit}"), json!(total));
    true
}

/// `event.duration = <field> * 1_000_000_000`, seconds to nanoseconds.
///
/// Returns false when the field name cannot be read out of the SCRIPT: that is
/// a shape this code does not actually understand, and counting it as handled
/// would inflate the coverage figure. A field the script names but the EVENT
/// lacks is a different thing -- the script would have done nothing either.
fn try_duration_to_nanos(event: &mut Event, script: &str) -> bool {
    let Some(field) = script
        .split("Long.parseLong(ctx.")
        .nth(1)
        .and_then(|rest| rest.split(')').next())
    else {
        return false;
    };

    let seconds = event
        .get_i64(field)
        .or_else(|| event.get_str(field).and_then(|s| s.parse::<i64>().ok()));

    if let Some(seconds) = seconds {
        // A duration above ~9.2 seconds-worth of i64 nanoseconds saturates
        // rather than wrapping to a negative event.duration.
        let _ = event.set(
            "event.duration",
            json!(seconds.saturating_mul(1_000_000_000)),
        );
    }
    true
}

/// IANA protocol number `0` means no transport was identified.
fn try_iana_zero_transport(event: &mut Event) -> bool {
    let iana = event
        .get_str("network.iana_number")
        .map(String::from)
        .or_else(|| event.get_i64("network.iana_number").map(|n| n.to_string()));

    if iana.as_deref() == Some("0") {
        let _ = event.set("network.transport", json!("unknown"));
    }
    true
}

/// The source and destination arrays of an append-if-absent script.
///
/// The shape is `for (x in ctx.A) { if (!ctx.B.contains(x)) ctx.B.add(x) }`,
/// which the network sources use to fold resolved addresses into
/// `related.ip`.
fn append_unique_fields(script: &str) -> Option<(&'static str, &'static str)> {
    let appends_uniquely = script.contains(".contains(") && script.contains(".add(");
    if !appends_uniquely {
        return None;
    }
    if script.contains("ctx.dns?.resolved_ip") && script.contains("ctx.related.ip") {
        return Some(("dns.resolved_ip", "related.ip"));
    }
    None
}

/// Re-key an array of maps into an object indexed by position.
///
/// Azure writes this out longhand -- a loop that builds `target[String.valueOf(i)]`
/// and copies each field under its `snake_case` name, guarding the optional ones:
///
/// ```painless
/// if (ctx.a.targetResources != null) {
///   ctx.a.target_resources = new HashMap();
///   for (def i = 0; i < ctx.a.targetResources.length; i++) {
///     String index = String.valueOf(i);
///     ctx.a.target_resources[index] = new HashMap();
///     ctx.a.target_resources[index].display_name = ctx.a.targetResources[i].displayName;
///     ...
///   }
///   ctx.a.properties.remove('targetResources');
/// }
/// ```
///
/// Every rename in it is `to_snake_case`, and every guard is "skip a null", so
/// the loop is those two rules applied recursively.
fn try_array_to_indexed_object(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::{clean_path, ctx_path_before};

    let Some(source) = ctx_path_before(script, " != null") else {
        return false;
    };
    let Some(target) = ctx_path_before(script, " = new HashMap()") else {
        return false;
    };
    let (source, target) = (clean_path(&source), clean_path(&target));
    if source == target {
        return false;
    }

    // A missing source is not a failure -- the script's own `if` guards it.
    let Some(Value::Array(items)) = event.get(&source).cloned() else {
        return true;
    };

    let rekeyed = index_keyed(&Value::Array(items));
    event.remove(&source);
    let _ = event.set(&target, rekeyed);
    true
}

/// Join two optional fields, falling back to whichever one is present.
///
/// ```painless
/// String reason = ctx?.a?.failure_reason;
/// String details = ctx?.a?.additional_details;
/// if (reason != null && details != null) { ctx['message'] = reason + ' (' + details + ')'; }
/// else if (reason != null) { ctx['message'] = reason; }
/// else if (details != null) { ctx['message'] = details; }
/// ```
fn try_join_optional(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::clean_path;

    let bindings = string_bindings(script);
    let [(first, first_path), (second, second_path)] = bindings.as_slice() else {
        return false;
    };
    let Some(target) = bracket_assignment_target(script) else {
        return false;
    };

    let a = event.get_str(&clean_path(first_path)).map(str::to_string);
    let b = event.get_str(&clean_path(second_path)).map(str::to_string);

    let joined = match (a, b) {
        (Some(a), Some(b)) => {
            let Some(expr) = both_present_expression(script) else {
                return false;
            };
            concat_expression(&expr, &[(first.as_str(), &a), (second.as_str(), &b)])
        }
        (Some(a), None) => a,
        (None, Some(b)) => b,
        // Neither present -- the script assigns nothing.
        (None, None) => return true,
    };

    let _ = event.set(&target, json!(joined));
    true
}

/// The `String <name> = ctx...;` bindings, in source order.
fn string_bindings(script: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for segment in script.split("String ").skip(1) {
        let Some((name, rest)) = segment.split_once(" = ctx") else {
            continue;
        };
        let path = rest.split(';').next().unwrap_or("");
        out.push((
            name.trim().to_string(),
            path.trim_start_matches(['?', '.']).to_string(),
        ));
    }
    out
}

/// The field a `ctx['<name>'] =` assignment writes to.
fn bracket_assignment_target(script: &str) -> Option<String> {
    let start = script.find("ctx['")? + "ctx['".len();
    let end = script[start..].find('\'')?;
    Some(script[start..start + end].to_string())
}

/// The right-hand side of the branch taken when BOTH fields are present.
fn both_present_expression(script: &str) -> Option<String> {
    let head = script.find("&&")? + 2;
    let start = head + assignment_offset(&script[head..])?;
    let end = script[start..].find(';')?;
    Some(script[start..start + end].trim().to_string())
}

/// The offset just past the first ASSIGNMENT `=`, skipping `!=` `==` `<=` `>=`.
fn assignment_offset(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    for (i, c) in bytes.iter().enumerate() {
        if *c != b'=' {
            continue;
        }
        let before = i.checked_sub(1).map(|p| bytes[p]);
        if matches!(before, Some(b'!' | b'=' | b'<' | b'>')) || bytes.get(i + 1) == Some(&b'=') {
            continue;
        }
        return Some(i + 1);
    }
    None
}

/// Evaluate a `a + ' (' + b + ')'` concatenation against the bound variables.
fn concat_expression(expr: &str, bound: &[(&str, &String)]) -> String {
    let mut out = String::new();
    for token in expr.split('+') {
        let token = token.trim();
        if let Some(literal) = token
            .strip_prefix('\'')
            .and_then(|t| t.strip_suffix('\''))
            .or_else(|| token.strip_prefix('"').and_then(|t| t.strip_suffix('"')))
        {
            out.push_str(literal);
        } else if let Some((_, value)) = bound.iter().find(|(name, _)| *name == token) {
            out.push_str(value);
        }
    }
    out
}

/// Collapse an array of `{key, value}` maps into one object.
///
/// `[{key: 'k1', value: 'v1'}]` becomes `{k1: 'v1'}`, in place.
fn try_key_value_pairs(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::{clean_path, ctx_path_before};

    let Some(field) = ctx_path_before(script, " = tmp") else {
        return false;
    };
    let field = clean_path(&field);

    // Already an object, or absent -- the processor's `instanceof List` guards
    // both, so there is nothing to do either way.
    let Some(Value::Array(items)) = event.get(&field).cloned() else {
        return true;
    };

    let mut out = Map::new();
    for item in &items {
        let Some(obj) = item.as_object() else {
            continue;
        };
        let Some(key) = obj.get("key").and_then(Value::as_str) else {
            continue;
        };
        out.insert(
            key.to_string(),
            obj.get("value").cloned().unwrap_or(Value::Null),
        );
    }
    let _ = event.set(&field, Value::Object(out));
    true
}

/// An array of maps as an object keyed "0", "1", ...; keys `snake_cased`,
/// nulls dropped, recursively.
fn index_keyed(value: &Value) -> Value {
    match value {
        Value::Array(items) if items.iter().any(Value::is_object) => {
            let mut out = Map::new();
            for (i, item) in items.iter().enumerate() {
                out.insert(i.to_string(), index_keyed(item));
            }
            Value::Object(out)
        }
        Value::Object(map) => {
            let mut out = Map::new();
            for (k, v) in map {
                if v.is_null() {
                    continue;
                }
                out.insert(
                    to_snake_case(k, SnakeRule::BeforeEveryUpper),
                    index_keyed(v),
                );
            }
            Value::Object(out)
        }
        other => other.clone(),
    }
}

/// Flatten a field into an array field, one entry per element.
///
/// The vendor scripts write both branches of this and pick at runtime on the
/// source's type, so this does the same rather than guessing from the text:
///
/// ```painless
/// if (ctx.crowdstrike.event.Tags instanceof List) {
///     for (tag in ctx.crowdstrike.event.Tags) {
///         if (tag instanceof Map) { ctx.tags.add(tag["Key"] + ":" + tag["ValueString"]); }
///     }
/// } else if (ctx.crowdstrike.event.Tags instanceof String) {
///     for (value in ctx.crowdstrike.event.Tags.splitOnToken(',')) { ctx.tags.add(value.trim()); }
/// }
/// ```
fn try_append_each(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::{clean_path, ctx_path_before};

    let Some(target) = ctx_path_before(script, ".add(") else {
        return false;
    };
    let Some(source) = append_source_path(script) else {
        return false;
    };
    // The map branch joins two keys; without both there is nothing to build.
    let map_keys = quoted_after(script, "tag[");
    let separator = quoted_after(script, ".splitOnToken(")
        .into_iter()
        .next()
        .unwrap_or_else(|| ",".to_string());

    // A missing source is not a failure -- the processor's `if` guards it.
    let entries: Vec<Value> = match event.get(&source) {
        Some(Value::String(s)) => s
            .split(&separator)
            .map(str::trim)
            .filter(|p| !p.is_empty())
            .map(|p| json!(p))
            .collect(),
        Some(Value::Array(items)) => {
            if map_keys.len() < 2 {
                return false;
            }
            items
                .iter()
                .filter_map(|item| {
                    let obj = item.as_object()?;
                    let k = obj.get(&map_keys[0])?.as_str()?;
                    let v = obj.get(&map_keys[1])?.as_str()?;
                    Some(json!(format!("{k}:{v}")))
                })
                .collect()
        }
        _ => return true,
    };

    let mut existing = match event.get(&clean_path(&target)) {
        Some(Value::Array(arr)) => arr.clone(),
        _ => Vec::new(),
    };
    existing.extend(entries);
    let _ = event.set(&clean_path(&target), Value::Array(existing));
    true
}

/// The field the append reads from -- the one the `instanceof` ladder tests.
fn append_source_path(script: &str) -> Option<String> {
    use crate::painless_params::{clean_path, ctx_path_before};

    ctx_path_before(script, " instanceof List")
        .or_else(|| ctx_path_before(script, " instanceof String"))
        .or_else(|| ctx_path_before(script, ".splitOnToken("))
        .map(|p| clean_path(&p))
}

/// Every single- or double-quoted string that follows an occurrence of `after`.
fn quoted_after(script: &str, after: &str) -> Vec<String> {
    let mut found = Vec::new();
    for segment in script.split(after).skip(1) {
        let mut chars = segment.char_indices();
        let Some((_, quote)) = chars.next() else {
            continue;
        };
        if quote != '\'' && quote != '"' {
            continue;
        }
        if let Some(end) = segment[1..].find(quote) {
            found.push(segment[1..=end].to_string());
        }
    }
    found
}

/// Append every element of `from` into `into`, skipping ones already present.
fn try_append_unique(event: &mut Event, from: &str, into: &str) -> bool {
    let Some(Value::Array(source)) = event.get(from).cloned() else {
        return true;
    };

    let mut target = match event.get(into).cloned() {
        Some(Value::Array(existing)) => existing,
        _ => Vec::new(),
    };
    for item in source {
        if !target.contains(&item) {
            target.push(item);
        }
    }
    let _ = event.set(into, Value::Array(target));
    true
}

/// Decompose a syslog PRI into ECS `log.syslog.{facility,severity}.{code,name}`.
///
/// The PRI is read from wherever the script found it: `log.syslog.priority`
/// for the generic pipelines, or a vendor field such as
/// `cisco_nexus.log.priority_number`.
fn try_syslog_priority(event: &mut Event, script: &str) -> bool {
    let pri = priority_source(script)
        .and_then(|field| read_u16(event, field))
        .or_else(|| read_u16(event, "log.syslog.priority"));

    let Some(pri) = pri else {
        return true;
    };

    let (facility, severity) = crate::syslog_pri::decompose(pri);
    let _ = event.set("log.syslog.facility.code", json!(facility));
    let _ = event.set("log.syslog.severity.code", json!(severity));
    if let Some(name) = crate::syslog_pri::facility_name(facility) {
        let _ = event.set("log.syslog.facility.name", json!(name));
    }
    if let Some(name) = crate::syslog_pri::severity_name(severity) {
        let _ = event.set("log.syslog.severity.name", json!(name));
    }
    true
}

/// The vendor field a priority script reads, when it is not the ECS one.
fn priority_source(script: &str) -> Option<&str> {
    script
        .split("ctx.")
        .find(|s| s.starts_with("cisco_nexus.log.priority_number"))
        .map(|_| "cisco_nexus.log.priority_number")
}

/// A field as a `u16`, whether it is stored as a number or a string.
fn read_u16(event: &Event, field: &str) -> Option<u16> {
    event
        .get_i64(field)
        .and_then(|n| u16::try_from(n).ok())
        .or_else(|| event.get_str(field).and_then(|s| s.parse::<u16>().ok()))
}

/// Check if a Painless script source matches a known pattern.
///
/// Returns true if the script was handled, false if it should fall through
/// to the generic `painless_exec` stub.
pub fn try_known_painless(event: &mut Event, script: &str) -> bool {
    let normalised = normalise(script);

    // Pattern: network.bytes / network.packets as the sum of both directions.
    if let Some(total) = sum_of_directions(&normalised) {
        return try_sum_directions(event, total);
    }

    // Pattern: seconds to nanoseconds for event.duration.
    if normalised.contains("ctx.event.duration")
        && normalised.contains("Long.parseLong")
        && normalised.contains("1000000000")
    {
        return try_duration_to_nanos(event, &normalised);
    }

    // Pattern: IANA protocol number 0 means the transport is unknown.
    if normalised.contains("ctx.network.iana_number")
        && normalised.contains("ctx.network.transport")
    {
        return try_iana_zero_transport(event);
    }

    // Pattern: decompose a syslog PRI into ECS facility and severity.
    if normalised.contains("log.syslog") && normalised.contains("priority") {
        return try_syslog_priority(event, &normalised);
    }

    // Pattern: append one array into another, skipping duplicates.
    if let Some((from, into)) = append_unique_fields(&normalised) {
        return try_append_unique(event, from, into);
    }

    // Pattern: re-key an array of maps into an object indexed by position.
    if normalised.contains("new HashMap()") && normalised.contains("String.valueOf(") {
        return try_array_to_indexed_object(event, &normalised);
    }

    // Pattern: collapse an array of `{key, value}` maps into one object.
    if normalised.contains("[item.key] = item.value") {
        return try_key_value_pairs(event, &normalised);
    }

    // Pattern: join two optional fields, each alone if the other is absent.
    if normalised.matches("String ").count() == 2 && normalised.contains("} else if (") {
        return try_join_optional(event, &normalised);
    }

    // Pattern: flatten a field into an array, either by splitting a delimited
    // string or by joining each map's two keys.
    if normalised.contains(".add(")
        && (normalised.contains(".splitOnToken(") || normalised.contains("instanceof Map"))
    {
        return try_append_each(event, &normalised);
    }

    // Pattern: drop null/empty values recursively
    if normalised.contains("drop(ctx)") && normalised.contains("removeIf") {
        drop_empty_recursive(event);
        return true;
    }

    // Pattern: keys_to_snake_case
    if normalised.contains("keys_to_snake_case") || normalised.contains("keysToSnakeCase") {
        if let Some(field) = extract_target_field(&normalised) {
            if let Some(val) = event.get(&field).cloned() {
                let mut val = val;
                keys_to_snake_case(&mut val);
                let _ = event.set(&field, val);
            }
        } else {
            // Apply to entire event
            let inner = event.as_value_mut();
            keys_to_snake_case(inner);
        }
        return true;
    }

    // Pattern: CommandLine → process fields
    if normalised.contains("CommandLine") && normalised.contains("process") {
        if normalised.contains("ParentCommandLine") {
            let _ = extract_process_fields(
                event,
                "crowdstrike.event.ParentCommandLine",
                "process.parent",
            );
        } else {
            let _ = extract_process_fields(event, "crowdstrike.event.CommandLine", "process");
        }
        return true;
    }

    // Pattern: ProcessStartTime epoch → @timestamp or process.start
    if normalised.contains("ProcessStartTime") || normalised.contains("processStartTime") {
        let _ = epoch_to_timestamp(event, "crowdstrike.event.ProcessStartTime", "process.start");
        return true;
    }

    // Pattern: email split — splitOnToken("@") → user.email, user.domain, user.name
    // Used in Okta, O365, Azure, and many other sources
    if normalised.contains("splitOnToken") && normalised.contains('@') {
        return try_email_split(event, &normalised);
    }

    // Pattern: okta risk_behaviors extraction from flattened.behaviors
    // Extracts keys with value "POSITIVE" into an array
    if normalised.contains("POSITIVE") && normalised.contains("risk_behaviors") {
        return try_risk_behaviors(event);
    }

    // Pattern: Azure category → event type/category mapping via params lookup
    if normalised.contains("activitylogs")
        && normalised.contains("category")
        && normalised.contains("params.get")
    {
        return try_azure_category_to_event_type(event);
    }

    // Pattern: Azure activitylogs event_category assignment
    if normalised.contains("event_category") && normalised.contains("eventCategory") {
        return try_azure_event_category(event);
    }

    // Pattern: replace dots in map keys (Azure identity claims)
    // Matches: ctx.temp_claims[key.replace('.', '_')] = ...
    if normalised.contains("replace('.'") && normalised.contains("keySet()") {
        return try_replace_dots_in_keys(event, &normalised);
    }

    // Pattern: okta.target array key renames + user/group extraction
    // Renames alternateId→alternate_id, displayName→display_name in each element,
    // filters detailEntry, extracts first user/usergroup targets
    if normalised.contains("alternateId")
        && normalised.contains("alternate_id")
        && normalised.contains("okta")
    {
        return try_okta_target_rename(event);
    }

    false
}

/// Handle the email split Painless pattern.
///
/// Painless patterns like:
/// ```painless
/// String[] splitmail = ctx.user.id.splitOnToken("@");
/// if (splitmail.length != 2) { return; }
/// ctx.user.email = ctx.user.id;
/// ctx.user.domain = splitmail[1];
/// ctx.user.name = splitmail[0];
/// ```
///
/// Also handles prefixed variants: user.target, source.user, destination.user
fn try_email_split(event: &mut Event, script: &str) -> bool {
    // Detect which field prefix this script operates on
    let prefix = if script.contains("ctx.user.target.id") {
        "user.target"
    } else if script.contains("ctx.source.user.id") {
        "source.user"
    } else if script.contains("ctx.destination.user.id") {
        "destination.user"
    } else if script.contains("ctx.user.id") {
        "user"
    } else {
        return false;
    };

    let id_field = format!("{prefix}.id");
    let email_val = match event.get_string(&id_field) {
        Some(v) if v.contains('@') => v,
        _ => return true, // Field missing or not an email — script returns early
    };

    let parts: Vec<&str> = email_val.split('@').collect();
    if parts.len() != 2 {
        return true; // Script returns early on non-standard email
    }

    let _ = event.set(&format!("{prefix}.email"), json!(email_val));
    let _ = event.set(&format!("{prefix}.name"), json!(parts[0]));
    let _ = event.set(&format!("{prefix}.domain"), json!(parts[1]));
    true
}

/// Extract risk behaviors from `okta.debug_context.debug_data.flattened.behaviors`.
///
/// The Painless script iterates the behaviors object and collects keys
/// where the value is "POSITIVE" into an array at `risk_behaviors`.
fn try_risk_behaviors(event: &mut Event) -> bool {
    // No behaviors, or not an object -- the script returns early.
    let Some(Value::Object(behaviors)) = event
        .get("okta.debug_context.debug_data.flattened.behaviors")
        .cloned()
    else {
        return true;
    };

    let positive: Vec<Value> = behaviors
        .iter()
        .filter(|(_, v)| v.as_str() == Some("POSITIVE"))
        .map(|(k, _)| json!(k))
        .collect();

    if !positive.is_empty() {
        let _ = event.set(
            "okta.debug_context.debug_data.risk_behaviors",
            Value::Array(positive),
        );
    }

    true
}

/// Handle the Okta target array key rename + user/group extraction pattern.
///
/// The Painless script:
/// 1. Renames `alternateId→alternate_id`, `displayName→display_name` in each target element
/// 2. Filters detailEntry to only keep methodTypeUsed and methodUsedVerifiedProperties
/// 3. Extracts first "User" type target → `okta_target_user`
/// 4. Extracts first "`UserGroup`" type target → `okta_target_group`
fn try_okta_target_rename(event: &mut Event) -> bool {
    // No target array -- the script returns early.
    let Some(Value::Array(target)) = event.get("okta.target").cloned() else {
        return true;
    };

    let mut result = Vec::with_capacity(target.len());
    let mut target_user: Option<Value> = None;
    let mut target_group: Option<Value> = None;

    for item in &target {
        if let Some(obj) = item.as_object() {
            let mut new_obj = serde_json::Map::new();

            for (k, v) in obj {
                let new_key = match k.as_str() {
                    "alternateId" => "alternate_id",
                    "displayName" => "display_name",
                    // Filtered in place: the script narrows the map and drops
                    // the key only when nothing survives, so the name stays.
                    "detailEntry" => {
                        if let Some(de) = v.as_object() {
                            let filtered: serde_json::Map<String, Value> = de
                                .iter()
                                .filter(|(k, _)| {
                                    k.as_str() == "methodTypeUsed"
                                        || k.as_str() == "methodUsedVerifiedProperties"
                                })
                                .map(|(k, v)| (k.clone(), v.clone()))
                                .collect();
                            if !filtered.is_empty() {
                                new_obj.insert("detailEntry".to_string(), Value::Object(filtered));
                            }
                        }
                        continue;
                    }
                    other => other,
                };
                new_obj.insert(new_key.to_string(), v.clone());
            }

            let new_val = Value::Object(new_obj.clone());

            // Extract first user/usergroup targets
            if let Some(type_val) = new_obj.get("type").and_then(|v| v.as_str()) {
                let type_lower = type_val.to_lowercase();
                if type_lower == "user" && target_user.is_none() {
                    target_user = Some(new_val.clone());
                } else if type_lower == "usergroup" && target_group.is_none() {
                    target_group = Some(new_val.clone());
                }
            }

            result.push(new_val);
        } else {
            result.push(item.clone());
        }
    }

    let _ = event.set("okta.target", Value::Array(result));

    if let Some(user) = target_user {
        let _ = event.set("okta_target_user", user);
    }
    if let Some(group) = target_group {
        let _ = event.set("okta_target_group", group);
    }

    true
}

/// Azure category → event type mapping.
///
/// Maps activitylogs.category to event.type via params lookup:
/// write/action → `["change"]`, read → `["access"]`, delete → `["deletion"]`
fn try_azure_category_to_event_type(event: &mut Event) -> bool {
    let category = match event.get_str("azure.activitylogs.category") {
        Some(c) => c.to_lowercase(),
        None => return true, // No category — script returns early
    };

    let event_types: Option<Vec<&str>> = match category.as_str() {
        "write" | "action" => Some(vec!["change"]),
        "read" => Some(vec!["access"]),
        "delete" => Some(vec!["deletion"]),
        _ => None,
    };

    if let Some(types) = event_types {
        for t in types {
            let _ = event.set("event.type", json!([t]));
        }
    }

    true
}

/// Azure activitylogs `event_category` conditional assignment.
///
/// Sets `azure.activitylogs.event_category` based on:
/// 1. `properties.eventCategory` if present
/// 2. "Policy" if `properties.policies` present
/// 3. "Administrative" as default
fn try_azure_event_category(event: &mut Event) -> bool {
    let category = if let Some(v) = event.get_str("azure.activitylogs.properties.eventCategory") {
        v.to_string()
    } else if event.has("azure.activitylogs.properties.policies") {
        "Policy".to_string()
    } else {
        "Administrative".to_string()
    };

    let _ = event.set("azure.activitylogs.event_category", json!(category));
    true
}

/// Replace dots with underscores in map keys at a given field path.
///
/// Common Azure pattern — identity claims have dots in URLs that Elastic normalises:
/// ```painless
/// for (String key : ctx.azure.activitylogs.identity.claims.keySet()) {
///   ctx.temp_claims[key.replace('.', '_')] = ctx.azure.activitylogs.identity.claims.get(key);
/// }
/// ctx.azure.activitylogs.identity.claims = ctx.temp_claims;
/// ```
fn try_replace_dots_in_keys(event: &mut Event, script: &str) -> bool {
    // Extract the field path by finding `ctx.<path>.keySet()`
    let field_path = if let Some(keyset_pos) = script.find(".keySet()") {
        // Walk backwards from .keySet() to find `ctx.`
        let before = &script[..keyset_pos];
        if let Some(ctx_pos) = before.rfind("ctx.") {
            let path = &before[ctx_pos + 4..];
            path.replace("?.", ".").replace('?', "")
        } else {
            return false;
        }
    } else {
        return false;
    };

    // Navigate to the parent object via JSON pointer to avoid dotted-path
    // issues with keys that contain literal dots (e.g., URL-like claim names)
    let pointer = format!("/{}", field_path.replace('.', "/"));
    let inner = event.as_value_mut();
    let resolved = inner.pointer_mut(&pointer);
    // Field missing or not an object -- skip.
    let Some(Value::Object(obj)) = resolved else {
        return true;
    };

    let new_map: Map<String, Value> = obj
        .iter()
        .map(|(k, v)| (k.replace('.', "_"), v.clone()))
        .collect();

    *obj = new_map;
    true
}

/// Try to extract a target field from a Painless script like `ctx.field_name`.
fn extract_target_field(script: &str) -> Option<String> {
    // Look for patterns like ctx.okta.request or ctx.field
    for line in script.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("ctx.") && !trimmed.contains('(') {
            let field = trimmed
                .trim_start_matches("ctx.")
                .trim_end_matches(';')
                .trim();
            if !field.is_empty() && !field.contains(' ') {
                return Some(field.replace("?.", ".").replace('?', ""));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The scripts below are the verbatim text the transform modules pass
    /// to `painless_exec`, so a change upstream shows up here as a miss.
    const SUM_BYTES: &str = "ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes";
    const SUM_PACKETS: &str = "ctx.network.packets = ctx.source.packets + ctx.destination.packets";
    const DURATION_NANOS: &str =
        "ctx.event.duration = Long.parseLong(ctx.fortinet.firewall.duration) * 1000000000";
    const IANA_ZERO: &str = "def iana_number = ctx.network.iana_number;\nif (iana_number == '0') \
                             {\n    ctx.network.transport = 'unknown';\n}";
    const APPEND_DNS: &str = "def dnsIPs = ctx.dns?.resolved_ip;\nif (dnsIPs != null) {\n  \
                              for (ip in dnsIPs) {\n    if (!ctx.related.ip.contains(ip)) \
                              {\n ctx.related.ip.add(ip);\n }\n  }\n}";

    /// Verbatim from `pipelines/crowdstrike/default.yml`.
    const APPEND_TAGS: &str = "if (ctx.crowdstrike.event.Tags instanceof List) {\n    for (tag in \
         ctx.crowdstrike.event.Tags) {\n        if (tag instanceof Map) {\n          \
         ctx.tags.add(tag[\"Key\"] + \":\" + tag[\"ValueString\"]);\n        }\n    }\n} else if \
         (ctx.crowdstrike.event.Tags instanceof String) {\n    def values = \
         ctx.crowdstrike.event.Tags.splitOnToken(',');\n    for (value in values) {\n        \
         ctx.tags.add(value.trim());\n    }\n}";

    #[test]
    fn splits_a_delimited_string_onto_the_end_of_the_array() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": { "Tags": "SensorGroupingTags/TEACHER, FalconGroupingTags/X" }},
            "tags": ["preserve_original_event"],
        }));

        assert!(try_known_painless(&mut event, APPEND_TAGS));

        assert_eq!(
            event.get("tags"),
            Some(&json!([
                "preserve_original_event",
                "SensorGroupingTags/TEACHER",
                "FalconGroupingTags/X"
            ]))
        );
    }

    /// The same script's other branch: the field arrives as maps, not a string.
    #[test]
    fn joins_each_map_pair_onto_the_end_of_the_array() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": { "Tags": [
                { "Key": "env", "ValueString": "prod" },
                { "Key": "team", "ValueString": "sec" },
            ]}},
            "tags": ["preserve_original_event"],
        }));

        assert!(try_known_painless(&mut event, APPEND_TAGS));

        assert_eq!(
            event.get("tags"),
            Some(&json!(["preserve_original_event", "env:prod", "team:sec"]))
        );
    }

    #[test]
    fn an_absent_source_leaves_the_array_alone() {
        let mut event = Event::new(json!({ "tags": ["preserve_original_event"] }));
        assert!(try_known_painless(&mut event, APPEND_TAGS));
        assert_eq!(event.get("tags"), Some(&json!(["preserve_original_event"])));
    }

    #[test]
    fn sums_bytes_and_packets_across_directions() {
        let mut event = Event::new(json!({
            "source": { "bytes": 100, "packets": 3 },
            "destination": { "bytes": 250, "packets": 4 },
        }));

        assert!(try_known_painless(&mut event, SUM_BYTES));
        assert!(try_known_painless(&mut event, SUM_PACKETS));

        assert_eq!(event.get_i64("network.bytes"), Some(350));
        assert_eq!(event.get_i64("network.packets"), Some(7));
    }

    /// Elastic's script throws when a side is missing; skipping is what the
    /// surrounding pipeline already relies on.
    #[test]
    fn a_missing_direction_leaves_the_total_unset() {
        let mut event = Event::new(json!({ "source": { "bytes": 100 } }));
        assert!(try_known_painless(&mut event, SUM_BYTES));
        assert!(!event.has("network.bytes"));
    }

    #[test]
    fn converts_a_duration_from_seconds_to_nanoseconds() {
        let mut event = Event::new(json!({ "fortinet": { "firewall": { "duration": 42 } } }));
        assert!(try_known_painless(&mut event, DURATION_NANOS));
        assert_eq!(event.get_i64("event.duration"), Some(42_000_000_000));
    }

    /// Both operands come off the wire. A vendor reporting a nonsense count
    /// must cost a saturated total, not a debug panic or a negative release
    /// one -- these are byte counts a dashboard sums.
    #[test]
    fn a_nonsense_byte_count_saturates_rather_than_wrapping() {
        let mut event = Event::new(json!({
            "source": { "bytes": i64::MAX },
            "destination": { "bytes": 1 },
        }));

        assert!(try_known_painless(&mut event, SUM_BYTES));
        assert_eq!(event.get_i64("network.bytes"), Some(i64::MAX));
    }

    #[test]
    fn a_nonsense_duration_saturates_rather_than_wrapping() {
        let mut event = Event::new(json!({ "fortinet": { "firewall": { "duration": i64::MAX } } }));

        assert!(try_known_painless(&mut event, DURATION_NANOS));
        assert_eq!(event.get_i64("event.duration"), Some(i64::MAX));
    }

    /// A duration script whose field name this code cannot read is NOT
    /// handled. Counting it would inflate the coverage figure with scripts
    /// nothing actually ran.
    #[test]
    fn an_unreadable_duration_script_is_not_counted_as_handled() {
        let mut event = Event::new(json!({}));
        let script = "ctx.event.duration = Long.parseLong(something) * 1000000000";
        assert!(!try_known_painless(&mut event, script));
    }

    /// The vendor field is often a string, because it came out of a grok.
    #[test]
    fn a_string_duration_converts_too() {
        let mut event = Event::new(json!({ "fortinet": { "firewall": { "duration": "7" } } }));
        assert!(try_known_painless(&mut event, DURATION_NANOS));
        assert_eq!(event.get_i64("event.duration"), Some(7_000_000_000));
    }

    #[test]
    fn iana_zero_means_the_transport_is_unknown() {
        let mut event = Event::new(json!({ "network": { "iana_number": "0" } }));
        assert!(try_known_painless(&mut event, IANA_ZERO));
        assert_eq!(event.get_str("network.transport"), Some("unknown"));

        let mut event = Event::new(json!({ "network": { "iana_number": "6" } }));
        assert!(try_known_painless(&mut event, IANA_ZERO));
        assert!(!event.has("network.transport"));
    }

    #[test]
    fn syslog_priority_decomposes_into_facility_and_severity() {
        // Verbatim from the fortinet transform.
        const PRIORITY: &str = "if (ctx.log?.syslog?.priority != null) {\n  \
             def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  \
             ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  \
             facility['code'] = ctx.log.syslog.priority>>3;\n  \
             ctx.log.syslog['facility'] = facility;\n}";

        let mut event = Event::new(json!({ "log": { "syslog": { "priority": 165 } } }));
        assert!(try_known_painless(&mut event, PRIORITY));

        // 165 = local4(20) * 8 + notice(5).
        assert_eq!(event.get_i64("log.syslog.facility.code"), Some(20));
        assert_eq!(event.get_str("log.syslog.facility.name"), Some("local4"));
        assert_eq!(event.get_i64("log.syslog.severity.code"), Some(5));
        assert_eq!(event.get_str("log.syslog.severity.name"), Some("notice"));
    }

    #[test]
    fn append_unique_skips_duplicates_and_keeps_order() {
        let mut event = Event::new(json!({
            "dns": { "resolved_ip": ["1.1.1.1", "2.2.2.2", "1.1.1.1"] },
            "related": { "ip": ["1.1.1.1"] },
        }));

        assert!(try_known_painless(&mut event, APPEND_DNS));
        assert_eq!(
            event.get("related.ip"),
            Some(&json!(["1.1.1.1", "2.2.2.2"]))
        );
    }

    /// The destination array may not exist yet.
    #[test]
    fn append_unique_creates_the_target_array() {
        let mut event = Event::new(json!({ "dns": { "resolved_ip": ["9.9.9.9"] } }));
        assert!(try_known_painless(&mut event, APPEND_DNS));
        assert_eq!(event.get("related.ip"), Some(&json!(["9.9.9.9"])));
    }

    #[test]
    fn drop_empty_removes_nulls() {
        let mut event = Event::new(json!({
            "a": "keep",
            "b": null,
            "c": "",
            "d": {"e": null, "f": "keep"},
            "g": [null, "", "keep"]
        }));
        drop_empty_recursive(&mut event);
        assert_eq!(event.get_str("a"), Some("keep"));
        assert!(!event.has("b"));
        assert!(!event.has("c"));
        assert!(event.has("d.f"));
        assert!(!event.has("d.e"));
    }

    #[test]
    fn keys_to_snake_case_converts() {
        let mut val = json!({
            "eventType": "login",
            "clientIp": "1.2.3.4",
            "nested": {"displayName": "test"}
        });
        keys_to_snake_case(&mut val);
        assert!(val.get("event_type").is_some());
        assert!(val.get("client_ip").is_some());
        assert!(val.get("eventType").is_none());
    }

    #[test]
    fn extract_process_from_cmd() {
        let mut event = Event::new(json!({
            "crowdstrike": {"event": {"CommandLine": "C:\\Windows\\Explorer.EXE /factory"}}
        }));
        extract_process_fields(&mut event, "crowdstrike.event.CommandLine", "process").unwrap();
        assert_eq!(
            event.get_str("process.command_line"),
            Some("C:\\Windows\\Explorer.EXE /factory")
        );
        assert_eq!(
            event.get_str("process.executable"),
            Some("C:\\Windows\\Explorer.EXE")
        );
    }

    #[test]
    fn epoch_to_iso8601() {
        let mut event = Event::new(json!({"ts": 1_536_846_339}));
        epoch_to_timestamp(&mut event, "ts", "@timestamp").unwrap();
        let ts = event.get_str("@timestamp").unwrap();
        assert!(ts.starts_with("2018-09-13"));
    }

    #[test]
    fn known_painless_drop_nulls() {
        let mut event = Event::new(json!({"a": null, "b": "keep"}));
        let script = r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#;
        assert!(try_known_painless(&mut event, script));
        assert!(!event.has("a"));
        assert!(event.has("b"));
    }

    #[test]
    fn email_split_user() {
        let mut event = Event::new(json!({"user": {"id": "john@example.com"}}));
        let script = r#"String[] splitmail = ctx.user.id.splitOnToken("@"); ctx.user.email = ctx.user.id; ctx.user.domain = splitmail[1]; ctx.user.name = splitmail[0];"#;
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_str("user.email"), Some("john@example.com"));
        assert_eq!(event.get_str("user.name"), Some("john"));
        assert_eq!(event.get_str("user.domain"), Some("example.com"));
    }

    #[test]
    fn email_split_no_at_sign() {
        let mut event = Event::new(json!({"user": {"id": "not-an-email"}}));
        let script = r#"String[] splitmail = ctx.user.id.splitOnToken("@");"#;
        assert!(try_known_painless(&mut event, script));
        // Should not set email/name/domain when no @ present
        assert!(!event.has("user.email"));
    }

    #[test]
    fn email_split_target_user() {
        let mut event = Event::new(json!({"user": {"target": {"id": "admin@corp.io"}}}));
        let script = r#"String[] splitmail = ctx.user.target.id.splitOnToken("@"); ctx.user.target.email = ctx.user.target.id;"#;
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_str("user.target.email"), Some("admin@corp.io"));
        assert_eq!(event.get_str("user.target.name"), Some("admin"));
    }

    #[test]
    fn risk_behaviors_positive() {
        let mut event = Event::new(json!({
            "okta": {"debug_context": {"debug_data": {"flattened": {"behaviors": {
                "New Geo-Location": "POSITIVE",
                "New Device": "NEGATIVE",
                "Velocity": "POSITIVE"
            }}}}}
        }));
        let script = r"if POSITIVE risk_behaviors";
        assert!(try_known_painless(&mut event, script));
        let behaviors = event.get("okta.debug_context.debug_data.risk_behaviors");
        assert!(behaviors.is_some());
        let arr = behaviors.unwrap().as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn risk_behaviors_none_positive() {
        let mut event = Event::new(json!({
            "okta": {"debug_context": {"debug_data": {"flattened": {"behaviors": {
                "New Device": "NEGATIVE"
            }}}}}
        }));
        let script = r"if POSITIVE risk_behaviors";
        assert!(try_known_painless(&mut event, script));
        // No POSITIVE entries — risk_behaviors should not be set
        assert!(!event.has("okta.debug_context.debug_data.risk_behaviors"));
    }

    #[test]
    fn okta_target_rename_and_extract() {
        let mut event = Event::new(json!({
            "okta": {"target": [
                {"type": "User", "alternateId": "user@test.com", "displayName": "Test User", "id": "001", "detailEntry": {"extra": "removed", "methodTypeUsed": "push"}},
                {"type": "UserGroup", "alternateId": "admins", "displayName": "Admins", "id": "002", "detailEntry": null}
            ]}
        }));
        let script =
            r"def target = ctx.okta.target; alternateId alternate_id displayName display_name okta";
        assert!(try_known_painless(&mut event, script));

        // Check renamed fields
        let target = event.get("okta.target").unwrap().as_array().unwrap();
        let first = target[0].as_object().unwrap();
        assert!(first.contains_key("alternate_id"));
        assert!(first.contains_key("display_name"));
        assert!(!first.contains_key("alternateId"));

        // detailEntry is narrowed in place, keeping its own name.
        let de = first.get("detailEntry").unwrap().as_object().unwrap();
        assert!(de.contains_key("methodTypeUsed"));
        assert!(!de.contains_key("extra"));

        // Check user/group extraction
        assert!(event.has("okta_target_user"));
        assert!(event.has("okta_target_group"));
    }

    #[test]
    fn replace_dots_in_keys_azure_claims() {
        let mut event = Event::new(json!({
            "azure": {"activitylogs": {"identity": {"claims": {
                "http://schemas.microsoft.com/identity/claims/id": "test123",
                "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name": "user"
            }}}}
        }));
        let script = r"if (ctx.azure.activitylogs.identity.claims != null) {\n  ctx.temp_claims = new HashMap();\n  for (String key : ctx.azure.activitylogs.identity.claims.keySet()) {\n    ctx.temp_claims[key.replace('.', '_')] = ctx.azure.activitylogs.identity.claims.get(key);\n  }\n  ctx.azure.activitylogs.identity.claims = ctx.temp_claims; ctx.remove('temp_claims');\n}";
        assert!(try_known_painless(&mut event, script));
        // Verify dots replaced with underscores in claim keys
        let claims = event
            .as_value()
            .pointer("/azure/activitylogs/identity/claims")
            .expect("claims should exist");
        let obj = claims.as_object().expect("claims should be object");
        // Original dotted keys should be replaced
        assert!(!obj.contains_key("http://schemas.microsoft.com/identity/claims/id"));
        assert!(obj.contains_key("http://schemas_microsoft_com/identity/claims/id"));
        assert_eq!(
            obj.get("http://schemas_microsoft_com/identity/claims/id")
                .unwrap(),
            "test123"
        );
    }

    #[test]
    fn azure_event_category_default() {
        let mut event = Event::new(json!({
            "azure": {"activitylogs": {"properties": {}}}
        }));
        let script = r"if (ctx?.azure?.activitylogs?.properties?.eventCategory != null) { ctx.azure.activitylogs.event_category = ctx.azure.activitylogs.properties.eventCategory; } else { ctx.azure.activitylogs.event_category = 'Administrative'; }";
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get_str("azure.activitylogs.event_category"),
            Some("Administrative")
        );
    }

    #[test]
    fn drop_empty_nested_arrays() {
        let mut event = Event::new(json!({
            "keep": "yes",
            "nested": {"arr": [null, "", {"inner": null}]}
        }));
        drop_empty_recursive(&mut event);
        assert!(event.has("keep"));
        // nested.arr should be empty after removing all null/empty items
        assert!(!event.has("nested"));
    }

    #[test]
    fn keys_to_snake_case_already_snake() {
        let mut val = json!({"already_snake": "yes", "alreadylower": "yes"});
        keys_to_snake_case(&mut val);
        assert!(val.get("already_snake").is_some());
        assert!(val.get("alreadylower").is_some());
    }
}
