// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Common Painless script patterns implemented in Rust.
//!
//! Instead of transpiling each Painless script individually, we identify
//! common patterns and implement them as shared runtime functions. The
//! `painless_exec` dispatcher matches known scripts and calls these.

use serde_json::{Map, Value, json};

use crate::error::Result;
use crate::event::Event;

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

/// Convert a Painless keys_to_snake_case operation.
///
/// Converts camelCase JSON object keys to snake_case recursively.
/// Common in Okta and other pipelines for normalising field names.
pub fn keys_to_snake_case(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let entries: Vec<(String, Value)> = map
                .iter()
                .map(|(k, v)| {
                    let mut snake = String::with_capacity(k.len() + 4);
                    for (i, c) in k.chars().enumerate() {
                        if c.is_uppercase() && i > 0 {
                            snake.push('_');
                        }
                        snake.push(c.to_lowercase().next().unwrap_or(c));
                    }
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
/// Sets: process.command_line, process.args, process.executable
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

/// Convert an epoch timestamp (seconds) to ISO8601 string and set on event.
pub fn epoch_to_timestamp(event: &mut Event, source_field: &str, target_field: &str) -> Result<()> {
    let epoch = match event.get(source_field) {
        Some(Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(Value::String(s)) => s.parse::<f64>().unwrap_or(0.0),
        _ => return Ok(()),
    };

    if epoch <= 0.0 {
        return Ok(());
    }

    let secs = epoch as i64;
    if let Some(dt) = chrono::DateTime::from_timestamp(secs, 0) {
        event.set(target_field, json!(dt.to_rfc3339()))?;
    }

    Ok(())
}

/// Check if a Painless script source matches a known pattern.
///
/// Returns true if the script was handled, false if it should fall through
/// to the generic painless_exec stub.
pub fn try_known_painless(event: &mut Event, script: &str) -> bool {
    let normalised = script.replace("\\n", "\n").replace("\\\"", "\"");

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

    false
}

/// Try to extract a target field from a Painless script like `ctx.field_name`.
fn extract_target_field(script: &str) -> Option<String> {
    // Look for patterns like ctx.okta.request or ctx.field
    for line in script.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("ctx.") && !trimmed.contains("(") {
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
        let mut event = Event::new(json!({"ts": 1536846339}));
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
}
