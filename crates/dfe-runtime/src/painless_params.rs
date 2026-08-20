// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Painless script patterns whose behaviour lives in the processor's `params`.
//!
//! An Elastic `script` processor may carry a `params` block, and the recurring
//! shapes read their whole configuration from it -- the sentinel list to strip,
//! the field names to convert, the lookup table to merge. Matching on script
//! text alone cannot execute any of them, because the text says only *that* a
//! param is read, never what it holds.
//!
//! [`try_params_painless`] is tried before the text-only matchers in
//! [`crate::painless_common::try_known_painless`], so a shape with a params
//! block runs against the pipeline's real table rather than a transcribed copy.

use serde_json::{Map, Value};

use crate::event::Event;
use crate::painless_helpers::{filetime_to_unix_ms, remove_sentinel_values};

/// A processor's `params` block, parsed once per CALL SITE.
///
/// The block is emitted as a JSON string rather than a `json!` literal: the
/// macro expands once per key, and the o365 operation table alone is deep
/// enough to blow rustc's recursion limit. Parsing it behind a per-site
/// `OnceLock` costs one parse per process and nothing per event -- the same
/// shape [`crate::cached_grok`] uses, and for the same reason.
#[macro_export]
macro_rules! cached_params {
    ($json:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<::serde_json::Value> = ::std::sync::OnceLock::new();
        SITE.get_or_init(|| {
            ::serde_json::from_str($json).expect("codegen emits a valid params block")
        })
    }};
}

/// The FILETIME threshold, as it is written in the vendor scripts.
const FILETIME_LITERAL: &str = "0x0100000000000000L";

/// Run a Painless script whose behaviour is carried by its `params` block.
///
/// Returns false when nothing matched, so the caller falls through to the
/// text-only matchers.
pub fn try_params_painless(event: &mut Event, script: &str, params: &Value) -> bool {
    let Some(params) = params.as_object() else {
        return false;
    };
    let normalised = crate::painless_common::normalise(script);

    // Pattern: strip the vendor's sentinel values out of a map.
    if normalised.contains(".entrySet().removeIf(") && normalised.contains("entry.getValue()") {
        return try_sentinel_removal(event, &normalised, params);
    }

    // Pattern: convert every named field from Windows FILETIME to UNIX ms.
    if normalised.contains(FILETIME_LITERAL) && normalised.contains("for (def field : params.") {
        return try_filetime_field_list(event, &normalised, params);
    }

    // Pattern: decode a bitfield into one boolean label per set bit.
    if normalised.contains("params.entrySet()") && normalised.contains("& flag") {
        return try_bit_flags(event, &normalised, params);
    }

    // Pattern: look a field up in a static table and merge the row into ctx.
    if normalised.contains("params.get(") && normalised.contains("forEach((k, v) ->") {
        return try_lookup_merge(event, &normalised, params);
    }

    // Pattern: look a row up in a nested table and fan its columns out,
    // appending the list-valued ones rather than replacing them.
    if normalised.contains("params.get(") && normalised.matches(".get(").count() >= 3 {
        return try_lookup_columns(event, &normalised, params);
    }

    // Pattern: normalise a field through a params table, keeping the input
    // when the table has no row for it.
    if normalised.contains("params.get(") {
        return try_lookup_normalise(event, &normalised, params);
    }

    // Pattern: index a params array by a numeric field.
    if normalised.contains(".put(") && normalised.contains("params") {
        return try_indexed_lookup(event, &normalised, params);
    }

    // Pattern: scale a numeric field by a params constant.
    if normalised.contains("* params.") {
        return try_scale(event, &normalised, params);
    }

    // Pattern: strip a params-named marker out of a string field.
    if normalised.contains(".replace(params.") {
        return try_replace(event, &normalised, params);
    }

    // Pattern: union a table row's list columns into the ECS arrays.
    if normalised.contains("addUnique(") && normalised.contains("params[ctx.") {
        return try_add_unique_row(event, &normalised, params);
    }

    false
}

/// `def p = params[ctx.<subject>];` then, per column,
/// `ctx.<target> = addUnique(ctx.<target>, p.<column>);`
///
/// Elastic's `gen` emits this as an `ecs_category_type` sub-pipeline for every
/// package that maps a vendor event name onto ECS categorisation, so the table
/// is the params block and a vendor adding an event type is picked up by
/// regenerating rather than by editing Rust.
///
/// `addUnique` is a set union, and the ECS arrays it writes are compared as
/// sets -- `tests/compare-policy.yaml` names both -- so the Java `HashSet`
/// iteration order it comes out in is not reproduced.
///
/// A subject with no row throws in Painless, and the sub-pipeline's `on_failure`
/// turns that into `event.kind: pipeline_error`. Nothing here models the throw,
/// so an unknown event type leaves the event as it was.
fn try_add_unique_row(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(subject) = ctx_path_between(script, "params[ctx.", "]") else {
        return false;
    };
    let bindings = add_unique_bindings(script);
    if bindings.is_empty() {
        return false;
    }

    let Some(key) = event.get_as_string(&subject) else {
        return true;
    };
    let Some(Value::Object(row)) = params.get(&key) else {
        return true;
    };
    let row = row.clone();

    for (target, column) in bindings {
        let Some(Value::Array(additions)) = row.get(&column) else {
            continue;
        };
        let mut merged = match event.get(&target) {
            Some(Value::Array(existing)) => existing.clone(),
            Some(other) => vec![other.clone()],
            None => Vec::new(),
        };
        for addition in additions {
            if !merged.contains(addition) {
                merged.push(addition.clone());
            }
        }
        let _ = event.set(&target, Value::Array(merged));
    }
    true
}

/// Every `ctx.<target> = addUnique(ctx.<target>, p.<column>)` in the script, as
/// the pair it names.
fn add_unique_bindings(script: &str) -> Vec<(String, String)> {
    const CALL: &str = "= addUnique(";

    let mut pairs = Vec::new();
    let mut at = 0;
    while let Some(found) = script[at..].find(CALL) {
        let call = at + found;
        let head = &script[..call];
        at = call + CALL.len();

        // The last `ctx.` before the call is the assignment's left-hand side;
        // the one inside the call is the same path read back.
        let Some(start) = head.rfind("ctx.") else {
            continue;
        };
        let target = clean_path(&head[start + "ctx.".len()..]);

        let Some((_, column)) = script[at..].split_once(',') else {
            continue;
        };
        let Some((_, column)) = column.trim_start().split_once('.') else {
            continue;
        };
        let end = column
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(column.len());
        pairs.push((target, column[..end].to_string()));
    }
    pairs
}

/// `long value = ctx.<source>;` then, per params entry,
/// `if ((value & flag) != 0) { <target>[entry.getKey()] = true; }`
///
/// The params block IS the flag table, so a vendor adding a bit is picked up by
/// regenerating rather than by editing Rust. Only set bits are written; a clear
/// bit leaves the label absent rather than false, which is what the script does.
///
/// panw's decryption-log flags are the shape, and they gate two of its largest
/// remaining blockers -- `labels.nat_translated` and `labels.captive_portal`.
fn try_bit_flags(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(source) = ctx_path_between(script, "long value = ctx.", ";") else {
        return false;
    };
    // The map the labels land in, from `ctx['labels'] = labels;`.
    let target = quoted_after(script, "ctx[").unwrap_or_else(|| "labels".to_owned());

    let Some(bits) = event.get(&source).and_then(as_u64_flags) else {
        return false;
    };

    for (name, flag) in params {
        // Elastic's own script decodes a string flag, because Kibana has been
        // known to hand the params block back with the numbers stringified.
        let Some(flag) = as_u64_flags(flag) else {
            continue;
        };
        if flag != 0 && bits & flag != 0 {
            let path = format!("{target}.{name}");
            if event.set(&path, Value::Bool(true)).is_err() {
                return false;
            }
        }
    }
    true
}

/// A bitfield, however the pipeline happens to carry it.
///
/// A `0x`-prefixed string is what `Long.decode` in the vendor script exists to
/// handle, and a float is what a large flag becomes if it round-trips as JSON.
fn as_u64_flags(value: &Value) -> Option<u64> {
    match value {
        Value::Number(n) => n
            .as_u64()
            .or_else(|| n.as_i64().and_then(|i| u64::try_from(i).ok()))
            .or_else(|| n.as_f64().map(|f| f as u64)),
        Value::String(s) => {
            let s = s.trim();
            s.strip_prefix("0x")
                .or_else(|| s.strip_prefix("0X"))
                .map_or_else(|| s.parse().ok(), |hex| u64::from_str_radix(hex, 16).ok())
        }
        _ => None,
    }
}

/// `ctx.<path>.entrySet().removeIf(entry -> params.<name>.contains(entry.getValue()))`
///
/// The sentinel list is the params entry, so a vendor adding `"-"` to it is
/// picked up by regenerating rather than by editing Rust.
fn try_sentinel_removal(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(path) = ctx_path_before(script, ".entrySet().removeIf(") else {
        return false;
    };
    let Some(Value::Array(sentinels)) = params_ref(script, params, "params.") else {
        return false;
    };
    let sentinels = sentinels.clone();

    // A missing map is not a failure -- the `if` on the processor already
    // guards it, and Elastic's own script returns without touching ctx.
    if let Some(Value::Object(map)) = pointer_mut(event, &path) {
        remove_sentinel_values(map, &sentinels);
    }
    true
}

/// `for (def field : params.<name>) { ctx.<path>[field] = convertToUnix(...) }`
///
/// Only the numeric conversion is modelled: a value that is neither a number
/// nor a digit string is left alone, which is what the vendor script's
/// `instanceof` ladder does.
fn try_filetime_field_list(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(container) = ctx_path_before(script, "[field]") else {
        return false;
    };
    let Some(Value::Array(fields)) = params_ref(script, params, "for (def field : params.") else {
        return false;
    };

    for field in fields.clone() {
        let Some(name) = field.as_str() else { continue };
        let path = format!("{container}.{name}");
        let converted = match event.get(&path) {
            Some(Value::Number(n)) => n.as_i64().map(filetime_to_unix_ms),
            // The script skips a string holding a fractional value.
            Some(Value::String(s)) if !s.contains('.') => {
                s.parse::<i64>().ok().map(filetime_to_unix_ms)
            }
            _ => None,
        };
        if let Some(v) = converted {
            let _ = event.set(&path, v);
        }
    }
    true
}

/// `params.get(<key>)` then `forEach((k, v) -> ctx.<target>[k] = v)`
///
/// The table is the params block itself, keyed by a field's value. A chained
/// second `.get(` means a two-level table, which this does not model, so it is
/// left to fall through and be counted as unhandled.
fn try_lookup_merge(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    if script.contains(").get(") {
        return false;
    }
    let Some(target) = ctx_path_between(script, "forEach((k, v) -> ctx.", "[k] = v") else {
        return false;
    };
    let Some(key_expr) = last_call_argument(script, "params.get(") else {
        return false;
    };
    let Some(key) = resolve_key(event, script, &key_expr) else {
        // The keyed field is absent, and every one of these scripts opens by
        // returning when that is so.
        return true;
    };
    let Some(Value::Object(row)) = params.get(&key) else {
        return true;
    };

    for (k, v) in row.clone() {
        let _ = event.set(&format!("{target}.{k}"), v);
    }
    true
}

/// `def row = params.get('<table>').get(ctx.<subject>);` then a column each:
/// `def c = row.get('<key>'); for (def x : c) { ctx.<path>.add(x) }` or
/// `ctx.<path> = c;`, with `ctx.<path> = ctx.<subject>` when the row is absent.
///
/// Cisco Meraki's event map is the shape: one vendor subtype expands into an
/// ECS action plus additions to `event.type` and `event.category`. Appending
/// matters -- the pipeline has already put `info` in `event.type`, and a
/// replacing write drops it.
fn try_lookup_columns(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(Value::Object(table)) = params_ref(script, params, "params.get('") else {
        return false;
    };
    // The table is bound to a local first, and the row lookup goes through it.
    let Some(table_local) = script
        .split_once("def ")
        .and_then(|(_, rest)| rest.split_once(" = params.get("))
        .map(|(name, _)| name.trim().to_string())
    else {
        return false;
    };
    let Some(key_expr) = last_call_argument(script, &format!("{table_local}.get(")) else {
        return false;
    };
    let Some(key) = resolve_key(event, script, &key_expr) else {
        return true;
    };

    let Some(Value::Object(row)) = table.get(&key) else {
        // Every one of these scripts falls back to writing the raw subtype.
        if let Some(target) = fallback_target(script) {
            let _ = event.set(&target, Value::String(key));
        }
        return true;
    };
    let row = row.clone();

    for (local, column) in column_bindings(script) {
        let Some(value) = row.get(&column) else {
            continue;
        };
        match appended_target(script, &local) {
            Some(path) => {
                let mut existing = match event.get(&path) {
                    Some(Value::Array(a)) => a.clone(),
                    _ => Vec::new(),
                };
                match value {
                    Value::Array(items) => existing.extend(items.iter().cloned()),
                    other => existing.push(other.clone()),
                }
                let _ = event.set(&path, Value::Array(existing));
            }
            None => {
                if let Some(path) = assigned_target(script, &local) {
                    let _ = event.set(&path, value.clone());
                }
            }
        }
    }
    true
}

/// `def <local> = <row>.get('<column>')` pairs, minus the row lookup itself.
fn column_bindings(script: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for segment in script.split("def ").skip(1) {
        let Some((local, rest)) = segment.split_once(" = ") else {
            continue;
        };
        if rest.contains("params") {
            continue;
        }
        let Some(column) = rest
            .split_once(".get('")
            .and_then(|(_, s)| s.split_once('\''))
        else {
            continue;
        };
        found.push((local.trim().to_string(), column.0.to_string()));
    }
    found
}

/// The `ctx.` path a `for (def x : <local>) { ctx.<path>.add(x) }` appends to.
fn appended_target(script: &str, local: &str) -> Option<String> {
    let needle = format!(" : {local})");
    let (_, tail) = script.split_once(&needle)?;
    let end = tail.find('}')?;
    ctx_path_before(&tail[..end], ".add(")
}

/// The `ctx.` path a bare `ctx.<path> = <local>;` assignment writes.
fn assigned_target(script: &str, local: &str) -> Option<String> {
    let needle = format!("= {local};");
    ctx_path_before(script, &needle)
}

/// The `ctx.` path the no-row branch writes the raw subject into.
fn fallback_target(script: &str) -> Option<String> {
    let (head, _) = script.split_once("== null")?;
    let (_, tail) = script[head.len()..].split_once('{')?;
    ctx_path_before(tail, " = ")
}

/// `def k = ctx.<path>.toLowerCase(); def v = params.get(k);`
/// `if (v != null) { ctx.<path> = v; return; } ctx.<path> = k;`
///
/// A vendor-vocabulary-to-ECS map: fortinet's `outgoing` is ECS `outbound`.
/// The fallback writes the LOOKUP KEY back, not the original, so a value the
/// table misses still comes out lower-cased -- and the pipeline's own
/// allow-list check downstream then sees the same string Elastic would.
fn try_lookup_normalise(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(key_expr) = last_call_argument(script, "params.get(") else {
        return false;
    };
    // Both writes are to the same field, so either assignment names the target.
    let Some(target) = ctx_assignment_target(script) else {
        return false;
    };
    let Some(key) = resolve_key(event, script, &key_expr) else {
        return true;
    };

    let value = params.get(&key).cloned().unwrap_or(Value::String(key));
    let _ = event.set(&target, value);
    true
}

/// `ctx.<target>.put('<key>', params['<name>'][<numeric field>])`
///
/// The bounds check the vendor writes around it is the array's own length, so
/// an index outside it simply leaves the field unset.
fn try_indexed_lookup(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(container) = ctx_path_before(script, ".put(") else {
        return false;
    };
    let Some(key) = quoted_after(script, ".put(") else {
        return false;
    };
    let Some(Value::Array(table)) = params_indexed(script, params) else {
        return false;
    };
    // The index is the only ctx field the script reads, and the processor's own
    // `if` guards it being absent -- so a value we cannot read means the path
    // came out wrong, and saying "handled" would hide that.
    let Some(index_path) = ctx_path_before(script, ";") else {
        return false;
    };
    let Some(index) = event
        .get_as_string(&index_path)
        .and_then(|s| s.trim().parse::<usize>().ok())
    else {
        return false;
    };

    if let Some(value) = table.get(index).cloned() {
        let _ = event.set(&format!("{container}.{key}"), value);
    }
    true
}

/// `ctx.<field> = ctx.<field> * params.<name>`
fn try_scale(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some((head, _)) = script.split_once("* params.") else {
        return false;
    };
    let Some(path) = crate::painless_common::painless_path(head) else {
        return false;
    };
    let Some(factor) = params_ref(script, params, "* params.").and_then(Value::as_f64) else {
        return false;
    };
    let Some(current) = event.get(&path).and_then(Value::as_f64) else {
        return true;
    };

    let scaled = current * factor;
    // Whole results stay integers: a duration in nanoseconds is not a float.
    if scaled.fract() == 0.0 && scaled.abs() < 9.007_199_254_740_992e15 {
        #[allow(clippy::cast_possible_truncation)]
        let _ = event.set(&path, scaled as i64);
    } else {
        let _ = event.set(&path, scaled);
    }

    // The same script often derives an end instant from the scaled duration.
    #[allow(clippy::cast_possible_truncation)]
    if script.contains(".plusNanos(") {
        add_nanos(event, script, scaled as i64);
    }
    true
}

/// `ctx.<target> = ZonedDateTime.parse(<start>).plusNanos(nanos)`
///
/// The start instant is a ctx field bound to a local earlier in the script.
fn add_nanos(event: &mut Event, script: &str, nanos: i64) {
    use crate::painless_common::painless_path;

    use crate::painless_common::ctx_path_bound_to;

    let Some((head, _)) = script.split_once(".plusNanos(") else {
        return;
    };
    // `parse(<local>)`, where the local was bound to a ctx path earlier.
    let Some(source) = head
        .rsplit_once("parse(")
        .map(|(_, name)| name.trim_end_matches(')').trim())
        .and_then(|name| ctx_path_bound_to(script, name).or_else(|| painless_path(name)))
    else {
        return;
    };
    // The assignment target is whatever sits left of the `=` on this line.
    let Some(target) = head
        .rsplit_once('=')
        .and_then(|(lhs, _)| painless_path(lhs))
    else {
        return;
    };
    let Some(start) = event.get_as_string(&source) else {
        return;
    };
    let Ok(start) = chrono::DateTime::parse_from_rfc3339(&start) else {
        return;
    };

    let end = start + chrono::TimeDelta::nanoseconds(nanos);
    let _ = event.set(
        &target,
        Value::String(
            end.with_timezone(&chrono::Utc)
                .format("%Y-%m-%dT%H:%M:%S%.3f%:z")
                .to_string(),
        ),
    );
}

/// `ctx.<field> = ctx.<field>.replace(params.<name>, '<replacement>')`
fn try_replace(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(path) = ctx_path_before(script, ".replace(params.") else {
        return false;
    };
    let Some(needle) = params_ref(script, params, ".replace(params.").and_then(Value::as_str)
    else {
        return false;
    };
    let replacement = quoted_after(script, ",").unwrap_or_default();
    let Some(current) = event.get_str(&path) else {
        return true;
    };

    let replaced = current.replace(needle, &replacement);
    let _ = event.set(&path, replaced);
    true
}

/// The first single- or double-quoted string following `after`.
fn quoted_after(script: &str, after: &str) -> Option<String> {
    let tail = &script[script.find(after)? + after.len()..];
    let start = tail.find(['\'', '"'])?;
    let quote = tail.as_bytes()[start] as char;
    let end = tail[start + 1..].find(quote)?;
    Some(tail[start + 1..=start + end].to_string())
}

/// Resolve the expression inside `params.get(...)` to a table key.
///
/// It is either a `ctx.` path written inline or a `def` bound to one earlier in
/// the script, and either may be lower-cased before the lookup.
fn resolve_key(event: &Event, script: &str, expr: &str) -> Option<String> {
    let lower = expr.contains(".toLowerCase()");
    let expr = expr.replace(".toLowerCase()", "");
    let expr = expr.trim();

    let (path, lower) = if let Some(rest) = expr.strip_prefix("ctx.") {
        (rest.to_string(), lower)
    } else {
        let binding = format!("def {expr} = ctx.");
        let start = script.find(&binding)? + binding.len();
        let tail = &script[start..];
        let end = tail.find([';', '\n']).unwrap_or(tail.len());
        let bound = &tail[..end];
        (
            bound.replace(".toLowerCase()", "").trim().to_string(),
            lower || bound.contains(".toLowerCase()"),
        )
    };

    let value = event.get_as_string(&clean_path(&path))?;
    Some(if lower { value.to_lowercase() } else { value })
}

/// The params entry an indexed reference names, in either form Painless allows:
/// `params['LogLevel'][i]` or `params.LogLevel[i]`.
fn params_indexed<'a>(script: &str, params: &'a Map<String, Value>) -> Option<&'a Value> {
    if let Some(name) = quoted_after(script, "params[") {
        return params.get(&name);
    }
    params_ref(script, params, "params.")
}

/// The params entry a `params.<name>` reference names, given its lead-in text.
fn params_ref<'a>(script: &str, params: &'a Map<String, Value>, prefix: &str) -> Option<&'a Value> {
    let start = script.find(prefix)? + prefix.len();
    let tail = &script[start..];
    let end = tail
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(tail.len());
    params.get(&tail[..end])
}

/// The dotted `ctx.` path that immediately precedes `marker`.
pub(crate) fn ctx_path_before(script: &str, marker: &str) -> Option<String> {
    let end = script.find(marker)?;
    let head = &script[..end];
    let start = head.rfind("ctx.")? + "ctx.".len();
    Some(clean_path(&head[start..]))
}

/// The `ctx.` path a script assigns to, from the LAST `ctx.<path> = ` in it.
///
/// A `def x = ctx.a.b` binding reads rather than writes, so the search is for
/// a path that IS the left-hand side, not merely one before an `=`.
fn ctx_assignment_target(script: &str) -> Option<String> {
    let mut found = None;
    for segment in script.split("ctx.").skip(1) {
        let end = segment
            .find(|c: char| !c.is_alphanumeric() && !".?_".contains(c))
            .unwrap_or(segment.len());
        let rest = segment[end..].trim_start();
        if rest.starts_with('=') && !rest.starts_with("==") {
            found = Some(clean_path(&segment[..end]));
        }
    }
    found
}

/// The dotted `ctx.` path written between two markers.
fn ctx_path_between(script: &str, open: &str, close: &str) -> Option<String> {
    let start = script.find(open)? + open.len();
    let tail = &script[start..];
    let end = tail.find(close)?;
    Some(clean_path(&tail[..end]))
}

/// The argument of the LAST call to `name(`, balanced across nested parens.
///
/// The table lookup is often written twice -- once to null-check, once to use
/// -- and it is the second that feeds the merge.
fn last_call_argument(script: &str, name: &str) -> Option<String> {
    let start = script.rfind(name)? + name.len();
    let mut depth = 1usize;
    for (i, c) in script[start..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(script[start..start + i].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

/// Strip Painless null-safe navigation from a field path.
pub(crate) fn clean_path(path: &str) -> String {
    path.trim().replace("?.", ".").replace('?', "")
}

/// A mutable reference to the value at a dotted path.
pub(crate) fn pointer_mut<'a>(event: &'a mut Event, path: &str) -> Option<&'a mut Value> {
    let mut pointer = String::with_capacity(path.len() + 1);
    for segment in path.split('.') {
        pointer.push('/');
        // JSON Pointer's own escapes, so a key holding one still resolves.
        for c in segment.chars() {
            match c {
                '~' => pointer.push_str("~0"),
                '/' => pointer.push_str("~1"),
                _ => pointer.push(c),
            }
        }
    }
    event.as_value_mut().pointer_mut(&pointer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from `pipelines/crowdstrike/default.yml`, so a change upstream
    /// shows up here as a miss.
    const SENTINEL: &str = "ctx.crowdstrike.event.entrySet().removeIf(entry -> \
                            params.values.contains(entry.getValue()));";

    /// Verbatim from `pipelines/okta/ecs_category_type.yml`. Elastic's `gen`
    /// emits this same script for every package that maps a vendor event name
    /// onto ECS categorisation.
    const ADD_UNIQUE: &str = "def addUnique(List dst, List src) {\n  src = src ?: [];\n  \
                              if (src.length == 0) {\n    return dst ?: [];\n  }\n  \
                              HashSet s = new HashSet(dst ?: []);\n  s.addAll(src);\n  \
                              return new ArrayList(s);\n}\n\
                              def p = params[ctx.okta.event_type];\n\
                              ctx.event.type = addUnique(ctx.event.type, p.type);\n\
                              ctx.event.category = addUnique(ctx.event.category, p.category);\n\
                              ctx.tags = addUnique(ctx.tags, p.tags);";

    fn add_unique_params() -> Value {
        json!({
            "user.session.start": {
                "category": ["authentication"],
                "type": ["start"],
                "tags": ["identity"],
            },
        })
    }

    /// The row's columns must be UNIONED into what the pipeline already put
    /// there -- the append processors ahead of this one have usually written
    /// `event.type` already, and replacing it drops their work.
    #[test]
    fn add_unique_unions_the_row_into_the_existing_arrays() {
        let mut event = Event::new(json!({
            "okta": { "event_type": "user.session.start" },
            "event": { "type": ["info"], "category": ["session"] },
            "tags": ["forwarded"],
        }));

        assert!(try_params_painless(
            &mut event,
            ADD_UNIQUE,
            &add_unique_params()
        ));

        assert_eq!(event.get("event.type"), Some(&json!(["info", "start"])));
        assert_eq!(
            event.get("event.category"),
            Some(&json!(["session", "authentication"]))
        );
        assert_eq!(event.get("tags"), Some(&json!(["forwarded", "identity"])));
    }

    /// A member the row repeats must not appear twice -- the script is a set
    /// union, not an append.
    #[test]
    fn add_unique_does_not_duplicate_what_is_already_there() {
        let mut event = Event::new(json!({
            "okta": { "event_type": "user.session.start" },
            "event": { "type": ["start"] },
        }));

        assert!(try_params_painless(
            &mut event,
            ADD_UNIQUE,
            &add_unique_params()
        ));
        assert_eq!(event.get("event.type"), Some(&json!(["start"])));
    }

    /// An arrays-absent event still gets the row, since `addUnique` treats a
    /// null destination as empty.
    #[test]
    fn add_unique_creates_the_arrays_it_finds_missing() {
        let mut event = Event::new(json!({ "okta": { "event_type": "user.session.start" } }));

        assert!(try_params_painless(
            &mut event,
            ADD_UNIQUE,
            &add_unique_params()
        ));
        assert_eq!(
            event.get("event.category"),
            Some(&json!(["authentication"]))
        );
    }

    /// An event type the table does not carry throws in Painless and the
    /// sub-pipeline's `on_failure` catches it. Nothing here models the throw,
    /// so the event must at least come through untouched rather than mangled.
    #[test]
    fn add_unique_leaves_an_unknown_event_type_alone() {
        let mut event = Event::new(json!({
            "okta": { "event_type": "user.session.nosuchthing" },
            "event": { "type": ["info"] },
        }));

        assert!(try_params_painless(
            &mut event,
            ADD_UNIQUE,
            &add_unique_params()
        ));
        assert_eq!(event.get("event.type"), Some(&json!(["info"])));
    }

    fn sentinel_params() -> Value {
        json!({ "values": [null, "", "-", "N/A", "NA", 0] })
    }

    #[test]
    fn strips_every_sentinel_the_params_name() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": {
                "keep": "value", "empty": "", "dash": "-", "na": "NA", "zero": 0, "nul": null,
            }},
        }));

        assert!(try_params_painless(
            &mut event,
            SENTINEL,
            &sentinel_params()
        ));

        let obj = event.get_object("crowdstrike.event").unwrap();
        assert_eq!(obj.len(), 1, "only `keep` survives: {obj:?}");
        assert_eq!(obj.get("keep"), Some(&json!("value")));
    }

    /// The metadata variant omits `0`, and a numeric zero must then survive.
    #[test]
    fn a_sentinel_absent_from_params_is_kept() {
        let script = SENTINEL.replace("crowdstrike.event", "crowdstrike.metadata");
        let mut event = Event::new(json!({
            "crowdstrike": { "metadata": { "zero": 0, "dash": "-" } },
        }));

        assert!(try_params_painless(
            &mut event,
            &script,
            &json!({ "values": [null, "", "-", "N/A", "NA"] }),
        ));

        let obj = event.get_object("crowdstrike.metadata").unwrap();
        assert_eq!(obj.get("zero"), Some(&json!(0)));
        assert!(!obj.contains_key("dash"));
    }

    #[test]
    fn a_missing_map_is_not_a_failure() {
        let mut event = Event::new(json!({ "other": 1 }));
        assert!(try_params_painless(
            &mut event,
            SENTINEL,
            &sentinel_params()
        ));
    }

    const FILETIME: &str = "def convertToUnix(def longValue) {\n\
                            if (longValue > 0x0100000000000000L) {\n\
                            return (longValue / 10000) - 11644473600000L;\n}\nreturn longValue;\n}\n\
                            for (def field : params.values) {\n\
                            def fieldValue = ctx.crowdstrike.event[field];\n\
                            ctx.crowdstrike.event[field] = convertToUnix(fieldValue);\n}";

    #[test]
    fn converts_every_filetime_field_the_params_name() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": {
                // 2020-01-01T00:00:00Z as a FILETIME, as a number and as a string.
                "StartTime": 132_223_104_000_000_000_i64,
                "EndTime": "132223104000000000",
                // Already UNIX seconds -- below the threshold, so untouched.
                "ContextTimeStamp": 1_577_836_800_i64,
                "Untouched": 132_223_104_000_000_000_i64,
            }},
        }));

        assert!(try_params_painless(
            &mut event,
            FILETIME,
            &json!({ "values": ["StartTime", "EndTime", "ContextTimeStamp"] }),
        ));

        assert_eq!(
            event.get_i64("crowdstrike.event.StartTime"),
            Some(1_577_836_800_000)
        );
        assert_eq!(
            event.get_i64("crowdstrike.event.EndTime"),
            Some(1_577_836_800_000)
        );
        assert_eq!(
            event.get_i64("crowdstrike.event.ContextTimeStamp"),
            Some(1_577_836_800)
        );
        assert_eq!(
            event.get_i64("crowdstrike.event.Untouched"),
            Some(132_223_104_000_000_000)
        );
    }

    /// Verbatim from `pipelines/azure/activitylogs/default.yml`.
    const LOOKUP: &str = "if (ctx?.azure?.activitylogs?.category == null) { return; } \
                          def category = ctx.azure.activitylogs.category.toLowerCase(); \
                          if (params.get(category) == null) { return; } \
                          def hm = new HashMap(params.get(category)); \
                          hm.forEach((k, v) -> ctx.event[k] = v);";

    fn lookup_params() -> Value {
        json!({
            "write": { "type": ["change"] },
            "read": { "type": ["access"] },
            "delete": { "type": ["deletion"] },
        })
    }

    #[test]
    fn merges_the_row_the_keyed_field_selects() {
        let mut event = Event::new(json!({
            "azure": { "activitylogs": { "category": "Write" } },
            "event": {},
        }));

        assert!(try_params_painless(&mut event, LOOKUP, &lookup_params()));
        assert_eq!(event.get("event.type"), Some(&json!(["change"])));
    }

    #[test]
    fn a_key_absent_from_the_table_changes_nothing() {
        let mut event = Event::new(json!({
            "azure": { "activitylogs": { "category": "Unmapped" } },
            "event": {},
        }));

        assert!(try_params_painless(&mut event, LOOKUP, &lookup_params()));
        assert_eq!(event.get_object("event").unwrap().len(), 0);
    }

    /// panw's decryption-log flags, as the generator emits them.
    const BIT_FLAGS: &str = r"def labels = ctx.labels; if (labels == null) {
  labels = new HashMap();
  ctx['labels'] = labels;
} long value = ctx._temp_.labels; for (entry in params.entrySet()) {
  def flag = entry.getValue();
  if (flag instanceof String) {
      flag = Long.decode(flag);
  }
  if ((value & flag) != 0) {
      labels[entry.getKey()] = true;
  }
}
";

    fn flag_params() -> Value {
        json!({
            "nat_translated": 0x0040_0000,
            "captive_portal": 0x0020_0000,
            "ssl_decrypted": 0x0100_0000,
        })
    }

    #[test]
    fn sets_a_label_for_every_bit_the_field_has_set() {
        let mut event = Event::new(json!({
            "_temp_": { "labels": 0x0060_0000 },
        }));

        assert!(try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
        assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
        assert_eq!(event.get("labels.captive_portal"), Some(&json!(true)));
    }

    /// The script only ever writes `true`, so a clear bit must leave the label
    /// ABSENT -- writing `false` would be an extra field Elastic never emits.
    #[test]
    fn a_clear_bit_leaves_its_label_absent() {
        let mut event = Event::new(json!({
            "_temp_": { "labels": 0x0040_0000 },
        }));

        assert!(try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
        assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
        assert_eq!(event.get("labels.captive_portal"), None);
        assert_eq!(event.get("labels.ssl_decrypted"), None);
    }

    /// `Long.decode` is in the vendor script because the flags have been known
    /// to arrive stringified, and a hex string must decode as hex.
    #[test]
    fn a_stringified_hex_flag_decodes() {
        let mut event = Event::new(json!({ "_temp_": { "labels": 0x0040_0000 } }));
        let params = json!({ "nat_translated": "0x00400000" });

        assert!(try_params_painless(&mut event, BIT_FLAGS, &params));
        assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
    }

    #[test]
    fn a_bitfield_carried_as_a_string_still_decodes() {
        let mut event = Event::new(json!({ "_temp_": { "labels": "4194304" } }));

        assert!(try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
        assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
    }

    #[test]
    fn an_absent_bitfield_writes_no_labels() {
        let mut event = Event::new(json!({ "event": {} }));

        assert!(!try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
        assert_eq!(event.get("labels"), None);
    }

    /// Cisco ASA's table is two levels deep, which this does not model -- it
    /// must fall through rather than merge the wrong row.
    #[test]
    fn a_two_level_table_falls_through() {
        let script = "params.get(ctx.event.code).get(ctx._temp_.outcome)\
                      .forEach((k, v) -> ctx.event[k] = v);";
        let mut event = Event::new(json!({ "event": { "code": "750002" } }));

        assert!(!try_params_painless(
            &mut event,
            script,
            &json!({ "750002": { "success": { "action": "started" } } }),
        ));
    }

    /// Verbatim from `pipelines/cisco/nexus/default.yml`.
    const INDEXED: &str = "def LogLevelValue = (int) ctx.event.severity;\n\
                           if (LogLevelValue >= 0 && LogLevelValue < params.LogLevel.length) {\n  \
                           ctx.log.put('level', params['LogLevel'][LogLevelValue]);\n}";

    #[test]
    fn indexes_the_params_array_by_the_numeric_field() {
        let mut event = Event::new(json!({ "event": { "severity": 3 }, "log": {} }));

        assert!(try_params_painless(
            &mut event,
            INDEXED,
            &json!({ "LogLevel": ["emergency", "alert", "critical", "error", "warning"] }),
        ));
        assert_eq!(event.get_str("log.level"), Some("error"));
    }

    /// The vendor's bounds check is the array's own length.
    #[test]
    fn an_index_past_the_end_sets_nothing() {
        let mut event = Event::new(json!({ "event": { "severity": 9 }, "log": {} }));

        assert!(try_params_painless(
            &mut event,
            INDEXED,
            &json!({ "LogLevel": ["emergency", "alert"] }),
        ));
        assert!(!event.has("log.level"));
    }

    /// Verbatim from `pipelines/azure/auditlogs/default.yml`.
    const SCALE: &str = "ctx.event.duration = ctx.event.duration * params.param_nano";

    #[test]
    fn scales_a_duration_by_the_params_constant() {
        let mut event = Event::new(json!({ "event": { "duration": 42 } }));

        assert!(try_params_painless(
            &mut event,
            SCALE,
            &json!({ "param_nano": 1_000_000_000_i64 }),
        ));
        assert_eq!(event.get_i64("event.duration"), Some(42_000_000_000));
    }

    /// Verbatim from `pipelines/azure/activitylogs/default.yml`.
    const REPLACE: &str = "ctx.message = ctx.message.replace(params.empty_field_name, '')";

    #[test]
    fn strips_the_marker_the_params_name() {
        let mut event = Event::new(json!({ "message": "a<EMPTY>b<EMPTY>" }));

        assert!(try_params_painless(
            &mut event,
            REPLACE,
            &json!({ "empty_field_name": "<EMPTY>" }),
        ));
        assert_eq!(event.get_str("message"), Some("ab"));
    }

    #[test]
    fn a_script_without_params_falls_through() {
        let mut event = Event::new(json!({}));
        assert!(!try_params_painless(&mut event, SENTINEL, &Value::Null));
    }
}
