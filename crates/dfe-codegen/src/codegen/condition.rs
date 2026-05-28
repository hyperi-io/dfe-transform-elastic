// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Transpile Elastic Painless conditional expressions to Rust code.
//!
//! This handles the `if:` field on Elastic ingest pipeline processors.
//! Patterns are matched at codegen time (not hot-path), so regex is appropriate.
//!
//! Supported patterns:
//! - `ctx?.field != null` / `ctx?.field == null` → `event.has("field")` / `!event.has("field")`
//! - `ctx?.field == "value"` / `ctx?.field != "value"` → string equality
//! - `ctx.field instanceof String` → `event.get("field").map(|v| v.is_string())`
//! - `["a","b"].contains(ctx?.field)` → list membership check
//! - `ctx?.tags.contains("x")` → field contains value
//! - `ctx?.field != null && ctx?.field != ""` → non-empty check
//! - Compound: `&&`, `||`, `!()` with recursive parsing

use regex::Regex;
use std::sync::OnceLock;

/// Attempt to transpile a Painless conditional expression to a Rust `if` condition.
///
/// Returns `Some(rust_expr)` if the expression can be transpiled,
/// `None` if it's too complex and should fall back to a TODO comment.
pub fn transpile_condition(expr: &str) -> Option<String> {
    let expr = expr.trim();

    // Try compound expressions first (&&, ||)
    if let Some(result) = try_compound(expr) {
        return Some(result);
    }

    // Try single expression
    transpile_single(expr)
}

/// Try to split on && or || and transpile each side.
fn try_compound(expr: &str) -> Option<String> {
    // Handle negation wrapping: !(expr)
    if let Some(inner) = strip_negation(expr) {
        let transpiled = transpile_condition(inner)?;
        return Some(format!("!({})", transpiled));
    }

    // Handle bare parentheses wrapping: (expr)
    if let Some(inner) = strip_parens(expr) {
        let transpiled = transpile_condition(inner)?;
        return Some(format!("({})", transpiled));
    }

    // Split on && (both sides must transpile)
    if let Some((left, right)) = split_logical(expr, "&&") {
        let l = transpile_condition(left)?;
        let r = transpile_condition(right)?;
        return Some(format!("{} && {}", l, r));
    }

    // Split on || (both sides must transpile)
    if let Some((left, right)) = split_logical(expr, "||") {
        let l = transpile_condition(left)?;
        let r = transpile_condition(right)?;
        return Some(format!("{} || {}", l, r));
    }

    None
}

/// Strip outer negation: `!(expr)` → `expr`
fn strip_negation(expr: &str) -> Option<&str> {
    let trimmed = expr.trim();
    if trimmed.starts_with("!(") && trimmed.ends_with(')') {
        Some(&trimmed[2..trimmed.len() - 1])
    } else {
        None
    }
}

/// Strip outer parentheses: `(expr)` → `expr`
/// Only strips if the closing paren matches the opening one (not a nested group).
fn strip_parens(expr: &str) -> Option<&str> {
    let trimmed = expr.trim();
    if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
        return None;
    }
    // Verify the closing paren matches the opening one
    let inner = &trimmed[1..trimmed.len() - 1];
    let mut depth = 0i32;
    for b in inner.bytes() {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth < 0 {
                    return None; // Closing paren doesn't match opening
                }
            }
            _ => {}
        }
    }
    if depth == 0 { Some(inner) } else { None }
}

/// Split an expression on a logical operator, respecting parentheses and brackets.
fn split_logical<'a>(expr: &'a str, op: &str) -> Option<(&'a str, &'a str)> {
    let mut depth_paren = 0i32;
    let mut depth_bracket = 0i32;
    let mut in_string = false;
    let bytes = expr.as_bytes();
    let op_bytes = op.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'"' && (i == 0 || bytes[i - 1] != b'\\') {
            in_string = !in_string;
        }
        if !in_string {
            if b == b'(' {
                depth_paren += 1;
            } else if b == b')' {
                depth_paren -= 1;
            } else if b == b'[' {
                depth_bracket += 1;
            } else if b == b']' {
                depth_bracket -= 1;
            }

            if depth_paren == 0 && depth_bracket == 0 && i + op_bytes.len() <= bytes.len() {
                if &bytes[i..i + op_bytes.len()] == op_bytes {
                    let left = expr[..i].trim();
                    let right = expr[i + op_bytes.len()..].trim();
                    if !left.is_empty() && !right.is_empty() {
                        return Some((left, right));
                    }
                }
            }
        }
        i += 1;
    }
    None
}

/// Transpile a single (non-compound) expression.
fn transpile_single(expr: &str) -> Option<String> {
    // Prefix negation: !expr (without parens, e.g. !["a","b"].contains(...))
    if let Some(inner) = expr.strip_prefix('!') {
        let inner = inner.trim();
        if !inner.starts_with('(') {
            if let Some(r) = transpile_single(inner) {
                return Some(format!("!({r})"));
            }
        }
    }

    // List contains: ["a","b"].contains(ctx?.field)
    if let Some(r) = try_list_contains(expr) {
        return Some(r);
    }

    // Field contains: ctx?.tags.contains("value") or ctx.tags.contains('value')
    if let Some(r) = try_field_contains(expr) {
        return Some(r);
    }

    // isEmpty: ctx?.field.isEmpty() or !ctx?.field.isEmpty()
    if let Some(r) = try_is_empty(expr) {
        return Some(r);
    }

    // splitOnToken email check: ctx.field.splitOnToken('@').length == 2
    if let Some(r) = try_split_on_token(expr) {
        return Some(r);
    }

    // instanceof: ctx.field instanceof String
    if let Some(r) = try_instanceof(expr) {
        return Some(r);
    }

    // field != null && field != ""  (non-empty check)
    // Already handled by compound && splitting + individual != null / != ""

    // String length comparison: String.valueOf(ctx.field).length() >= 12
    if let Some(r) = try_string_length(expr) {
        return Some(r);
    }

    // field != null
    if let Some(r) = try_null_check(expr) {
        return Some(r);
    }

    // field == "value" / field != "value"
    if let Some(r) = try_equality(expr) {
        return Some(r);
    }

    // field == 123 / field != 0 (numeric equality)
    if let Some(r) = try_numeric_equality(expr) {
        return Some(r);
    }

    // field == true / field == false
    if let Some(r) = try_bool_check(expr) {
        return Some(r);
    }

    None
}

/// `String.valueOf(ctx.field).length() >= 12` → string length comparison
/// `ctx.field.length() >= 12` → string length comparison
///
/// Used by CrowdStrike epoch timestamp conditionals to distinguish
/// UNIX (<=11 digits) from UNIX_MS (>=12 digits).
fn try_string_length(expr: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(
            r#"^(?:String\.valueOf\()?ctx\??\.(.+?)\)?\s*\.length\(\)\s*(>=|<=|>|<|==)\s*(\d+)$"#,
        )
        .expect("string length regex")
    });

    let caps = re.captures(expr)?;
    let field = painless_field_to_ecs(caps.get(1)?.as_str());
    // Clean up trailing `)` from String.valueOf() wrapping
    let field = field.trim_end_matches(')');
    let op = caps.get(2)?.as_str();
    let threshold = caps.get(3)?.as_str();

    Some(format!(
        r#"event.get_as_string("{field}").is_some_and(|s| s.len() {op} {threshold})"#,
    ))
}

/// `ctx?.field != null` → `event.has("field")`
/// `ctx?.field == null` → `!event.has("field")`
fn try_null_check(expr: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r#"^ctx\??\.(.+?)\s*(!=|==)\s*null$"#).expect("null check regex")
    });

    let caps = re.captures(expr)?;
    let field = painless_field_to_ecs(caps.get(1)?.as_str());
    let op = caps.get(2)?.as_str();

    Some(match op {
        "!=" => format!(r#"event.has("{field}")"#),
        "==" => format!(r#"!event.has("{field}")"#),
        _ => return None,
    })
}

/// `ctx?.field == "value"` → `event.get_str("field") == Some("value")`
/// `ctx?.field != "value"` → `event.get_str("field") != Some("value")`
/// `ctx?.field != ""` → `event.get_str("field").is_some_and(|s| !s.is_empty())`
fn try_equality(expr: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r#"^ctx\??\.(.+?)\s*(==|!=)\s*"([^"]*)"$"#).expect("equality regex")
    });

    let caps = re.captures(expr)?;
    let field = painless_field_to_ecs(caps.get(1)?.as_str());
    let op = caps.get(2)?.as_str();
    let value = caps.get(3)?.as_str();

    if value.is_empty() {
        // field != "" → non-empty check
        Some(match op {
            "!=" => format!(r#"event.get_str("{field}").is_some_and(|s| !s.is_empty())"#),
            "==" => format!(r#"event.get_str("{field}").is_none_or(|s| s.is_empty())"#),
            _ => return None,
        })
    } else {
        Some(match op {
            "==" => format!(r#"event.get_str("{field}") == Some("{value}")"#),
            "!=" => format!(r#"event.get_str("{field}") != Some("{value}")"#),
            _ => return None,
        })
    }
}

/// `ctx?.field == 0` / `ctx?.field != 123` (numeric equality)
fn try_numeric_equality(expr: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r#"^ctx\??\.(.+?)\s*(==|!=)\s*(-?\d+)$"#).expect("numeric equality regex")
    });

    let caps = re.captures(expr)?;
    let field = painless_field_to_ecs(caps.get(1)?.as_str());
    let op = caps.get(2)?.as_str();
    let value = caps.get(3)?.as_str();

    Some(format!(r#"event.get_i64("{field}") {op} Some({value})"#,))
}

/// `ctx?.field == true` / `ctx?.field == false`
fn try_bool_check(expr: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r#"^ctx\??\.(.+?)\s*(==|!=)\s*(true|false)$"#).expect("bool regex")
    });

    let caps = re.captures(expr)?;
    let field = painless_field_to_ecs(caps.get(1)?.as_str());
    let op = caps.get(2)?.as_str();
    let val = caps.get(3)?.as_str();

    Some(match (op, val) {
        ("==", "true") => format!(r#"event.get_bool("{field}") == Some(true)"#),
        ("==", "false") => format!(r#"event.get_bool("{field}") == Some(false)"#),
        ("!=", "true") => format!(r#"event.get_bool("{field}") != Some(true)"#),
        ("!=", "false") => format!(r#"event.get_bool("{field}") != Some(false)"#),
        _ => return None,
    })
}

/// `["a","b","c"].contains(ctx?.field)` → list membership
fn try_list_contains(expr: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r#"^\[([^\]]+)\]\.contains\(ctx\??\.(.+?)\)$"#).expect("list contains regex")
    });

    let caps = re.captures(expr)?;
    let list_str = caps.get(1)?.as_str();
    let field = painless_field_to_ecs(caps.get(2)?.as_str());

    // Parse the list items (they're quoted strings)
    let items: Vec<&str> = list_str
        .split(',')
        .map(|s| s.trim().trim_matches('"'))
        .collect();

    let items_str: Vec<String> = items.iter().map(|s| format!(r#""{s}""#)).collect();
    Some(format!(
        r#"[{items}].contains(&event.get_str("{field}").unwrap_or(""))"#,
        items = items_str.join(", "),
    ))
}

/// `ctx?.tags.contains("value")` or `ctx.tags.contains('value')`
fn try_field_contains(expr: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r#"^ctx\??\.(.+?)\.contains\(["']([^"']+)["']\)$"#)
            .expect("field contains regex")
    });

    let caps = re.captures(expr)?;
    let field = painless_field_to_ecs(caps.get(1)?.as_str());
    let value = caps.get(2)?.as_str();

    // For array fields (like tags), check if array contains value
    // For string fields, check if string contains substring
    Some(format!(
        r#"event.get("{field}").is_some_and(|v| match v {{ serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("{value}")), serde_json::Value::String(s) => s.contains("{value}"), _ => false }})"#,
    ))
}

/// `ctx.field instanceof String`
fn try_instanceof(expr: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r#"^ctx\??\.(.+?)\s+instanceof\s+(\w+)$"#).expect("instanceof regex")
    });

    let caps = re.captures(expr)?;
    let field = painless_field_to_ecs(caps.get(1)?.as_str());
    let java_type = caps.get(2)?.as_str();

    let check = match java_type {
        "String" => "is_string()",
        "Map" | "HashMap" => "is_object()",
        "List" | "ArrayList" => "is_array()",
        "Number" | "Integer" | "Long" | "Double" | "Float" => "is_number()",
        "Boolean" => "is_boolean()",
        _ => return None,
    };

    Some(format!(
        r#"event.get("{field}").is_some_and(|v| v.{check})"#
    ))
}

/// `ctx?.field.isEmpty()` → string empty check
/// `!ctx?.field.isEmpty()` handled by prefix negation + this
fn try_is_empty(expr: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re =
        RE.get_or_init(|| Regex::new(r#"^ctx\??\.(.+?)\.isEmpty\(\)$"#).expect("isEmpty regex"));

    let caps = re.captures(expr)?;
    let field = painless_field_to_ecs(caps.get(1)?.as_str());

    Some(format!(
        r#"event.get_str("{field}").is_none_or(|s| s.is_empty())"#,
    ))
}

/// `ctx.field.splitOnToken('@').length == 2` → email format validation
/// `ctx.field.splitOnToken('@')?.length == 2` (with optional chaining)
fn try_split_on_token(expr: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(
            r#"^ctx\??\.(.+?)\.splitOnToken\(['"](.+?)['"]\)\??\.length\s*(==|>=|<=|>|<)\s*(\d+)"#,
        )
        .expect("splitOnToken regex")
    });

    let caps = re.captures(expr)?;
    let field = painless_field_to_ecs(caps.get(1)?.as_str());
    let delimiter = caps.get(2)?.as_str();
    let op = caps.get(3)?.as_str();
    let count = caps.get(4)?.as_str();

    Some(format!(
        r#"event.get_str("{field}").is_some_and(|s| s.split('{delimiter}').count() {op} {count})"#,
    ))
}

/// Convert Painless field path to ECS dotted path.
///
/// Painless uses `ctx?.field?.sub` or `ctx.field.sub`.
/// We strip `ctx?.` / `ctx.` prefix and remove `?` optional chaining.
fn painless_field_to_ecs(field: &str) -> String {
    field
        .trim_start_matches("ctx?.")
        .trim_start_matches("ctx.")
        .replace("?.", ".")
        .replace('?', "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_check_not_null() {
        assert_eq!(
            transpile_condition("ctx?.source?.ip != null"),
            Some(r#"event.has("source.ip")"#.into())
        );
    }

    #[test]
    fn null_check_is_null() {
        assert_eq!(
            transpile_condition("ctx?.event?.outcome == null"),
            Some(r#"!event.has("event.outcome")"#.into())
        );
    }

    #[test]
    fn equality_string() {
        assert_eq!(
            transpile_condition(r#"ctx?.okta?.outcome?.result_lower == "success""#),
            Some(r#"event.get_str("okta.outcome.result_lower") == Some("success")"#.into())
        );
    }

    #[test]
    fn inequality_string() {
        assert_eq!(
            transpile_condition(r#"ctx?.okta?.outcome?.result_lower != "failure""#),
            Some(r#"event.get_str("okta.outcome.result_lower") != Some("failure")"#.into())
        );
    }

    #[test]
    fn non_empty_check() {
        assert_eq!(
            transpile_condition(r#"ctx?.json?.uuid != """#),
            Some(r#"event.get_str("json.uuid").is_some_and(|s| !s.is_empty())"#.into())
        );
    }

    #[test]
    fn list_contains() {
        let result = transpile_condition(
            r#"["user.session.start","user.session.end"].contains(ctx?.okta?.event_type)"#,
        );
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains(r#""user.session.start""#));
        assert!(r.contains(r#""user.session.end""#));
        assert!(r.contains(r#"event.get_str("okta.event_type")"#));
    }

    #[test]
    fn field_contains() {
        let result = transpile_condition(r#"ctx?.tags.contains("preserve_original_event")"#);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains(r#"event.get("tags")"#));
        assert!(r.contains(r#""preserve_original_event""#));
    }

    #[test]
    fn instanceof_string() {
        assert_eq!(
            transpile_condition("ctx.azure?.activitylogs?.identity instanceof String"),
            Some(
                r#"event.get("azure.activitylogs.identity").is_some_and(|v| v.is_string())"#.into()
            )
        );
    }

    #[test]
    fn compound_and() {
        let result = transpile_condition(r#"ctx?.json?.uuid != null && ctx?.json?.uuid != """#);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains(r#"event.has("json.uuid")"#));
        assert!(r.contains("&&"));
        assert!(r.contains("is_some_and"));
    }

    #[test]
    fn compound_or() {
        let result = transpile_condition(
            r#"ctx?.okta?.outcome?.result_lower == "success" || ctx?.okta?.outcome?.result_lower == "allow""#,
        );
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains("||"));
    }

    #[test]
    fn negation() {
        let result = transpile_condition(r#"!(ctx?.tags.contains("preserve_original_event"))"#);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.starts_with("!("));
    }

    #[test]
    fn bool_check() {
        assert_eq!(
            transpile_condition("ctx?.event?.enriched == true"),
            Some(r#"event.get_bool("event.enriched") == Some(true)"#.into())
        );
    }

    #[test]
    fn string_length_valueof() {
        let result = transpile_condition(
            "String.valueOf(ctx.crowdstrike.event.UTCTimestamp).length() >= 12",
        );
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains(r#"event.get_as_string("crowdstrike.event.UTCTimestamp")"#));
        assert!(r.contains("s.len() >= 12"));
    }

    #[test]
    fn string_length_direct() {
        let result = transpile_condition("ctx.crowdstrike.event.StartTime.length() <= 11");
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains(r#"event.get_as_string("crowdstrike.event.StartTime")"#));
        assert!(r.contains("s.len() <= 11"));
    }

    #[test]
    fn string_length_in_compound() {
        let result = transpile_condition(
            "ctx.crowdstrike?.event?.StartTime != null && ctx.crowdstrike.event.StartTime.length() >= 12",
        );
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains("&&"));
        assert!(r.contains("s.len() >= 12"));
    }

    #[test]
    fn is_empty() {
        let result = transpile_condition("ctx.client?._temp.isEmpty()");
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains("is_none_or"));
        assert!(r.contains("is_empty"));
    }

    #[test]
    fn not_is_empty_compound() {
        let result =
            transpile_condition("ctx.client?._temp != null && !ctx.client?._temp.isEmpty()");
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains("&&"));
        assert!(r.contains("!("));
    }

    #[test]
    fn single_quote_contains() {
        let result = transpile_condition("ctx.tags.contains('preserve_original_event')");
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains("preserve_original_event"));
    }

    #[test]
    fn split_on_token_email() {
        let result =
            transpile_condition("ctx.o365audit?.Data?.f3u?.splitOnToken('@')?.length == 2");
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.contains("split('@')"));
        assert!(r.contains("count() == 2"));
    }

    #[test]
    fn parenthesised_or_group() {
        let result = transpile_condition(
            r#"ctx?.okta?.outcome?.result_lower != null && (ctx?.okta?.outcome?.result_lower == "success" || ctx?.okta?.outcome?.result_lower == "allow")"#,
        );
        assert!(result.is_some(), "should transpile parenthesised OR group");
        let r = result.unwrap();
        assert!(r.contains("&&"));
        assert!(r.contains("||"));
        assert!(r.contains(r#""success""#));
        assert!(r.contains(r#""allow""#));
    }

    #[test]
    fn numeric_equality() {
        assert_eq!(
            transpile_condition("ctx.crowdstrike.event.ResourceCreateTime == 0"),
            Some(r#"event.get_i64("crowdstrike.event.ResourceCreateTime") == Some(0)"#.into())
        );
    }

    #[test]
    fn unsupported_returns_none() {
        // Complex Painless with method calls we can't handle
        assert!(transpile_condition("ctx.json.keySet().size() > 0").is_none());
    }

    #[test]
    fn field_path_cleanup() {
        assert_eq!(painless_field_to_ecs("event?.original"), "event.original");
        assert_eq!(painless_field_to_ecs("okta?.actor?.id"), "okta.actor.id");
        assert_eq!(painless_field_to_ecs("ctx?.foo"), "foo");
    }
}
