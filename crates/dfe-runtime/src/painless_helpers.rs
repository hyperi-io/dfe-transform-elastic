// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Runtime helpers for transpiled Painless code.
//!
//! These functions bridge Painless dynamic typing to Rust's `serde_json::Value`.
//! They are called by generated transform code that was transpiled from
//! Painless scripts. All functions are pure — no I/O, no side effects
//! beyond operating on the provided values.

use serde_json::{Map, Value, json};

/// Painless truthiness: `null`/`false`/`0`/`""` → false, everything else → true.
///
/// Matches Painless/Java boolean coercion semantics.
#[inline]
pub fn painless_truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i != 0
            } else if let Some(f) = n.as_f64() {
                f != 0.0
            } else {
                true
            }
        }
        Value::String(s) => !s.is_empty(),
        Value::Array(_) | Value::Object(_) => true,
    }
}

/// Painless addition: string concat if either operand is a string, else numeric.
pub fn painless_add(a: &Value, b: &Value) -> Value {
    // String concatenation takes priority (Painless/Java behaviour)
    if a.is_string() || b.is_string() {
        let a_str = painless_to_string(a);
        let b_str = painless_to_string(b);
        return json!(format!("{a_str}{b_str}"));
    }

    // Numeric addition
    let a_num = painless_to_f64(a);
    let b_num = painless_to_f64(b);
    let result = a_num + b_num;

    // Preserve integer type if both inputs are integers
    if a.is_i64() && b.is_i64() {
        json!(a.as_i64().unwrap_or(0) + b.as_i64().unwrap_or(0))
    } else if a.is_u64() && b.is_u64() {
        json!(a.as_u64().unwrap_or(0) + b.as_u64().unwrap_or(0))
    } else {
        json!(result)
    }
}

/// Painless subtraction.
pub fn painless_sub(a: &Value, b: &Value) -> Value {
    if a.is_i64() && b.is_i64() {
        json!(a.as_i64().unwrap_or(0) - b.as_i64().unwrap_or(0))
    } else {
        json!(painless_to_f64(a) - painless_to_f64(b))
    }
}

/// Painless multiplication.
pub fn painless_mul(a: &Value, b: &Value) -> Value {
    if a.is_i64() && b.is_i64() {
        json!(a.as_i64().unwrap_or(0) * b.as_i64().unwrap_or(0))
    } else {
        json!(painless_to_f64(a) * painless_to_f64(b))
    }
}

/// Painless division (integer division for integers, float otherwise).
pub fn painless_div(a: &Value, b: &Value) -> Value {
    let b_val = painless_to_f64(b);
    if b_val == 0.0 {
        return Value::Null;
    }
    if a.is_i64() && b.is_i64() {
        json!(a.as_i64().unwrap_or(0) / b.as_i64().unwrap_or(1))
    } else {
        json!(painless_to_f64(a) / b_val)
    }
}

/// Painless modulo.
pub fn painless_mod(a: &Value, b: &Value) -> Value {
    let b_val = painless_to_i64(b);
    if b_val == 0 {
        return Value::Null;
    }
    json!(painless_to_i64(a) % b_val)
}

/// Convert a `Value` to `i64`. Handles strings, floats, bools.
pub fn painless_to_i64(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n
            .as_i64()
            .unwrap_or_else(|| n.as_f64().unwrap_or(0.0) as i64),
        Value::String(s) => s.parse::<i64>().unwrap_or(0),
        Value::Bool(b) => i64::from(*b),
        _ => 0,
    }
}

/// Convert a `Value` to `f64`.
pub fn painless_to_f64(v: &Value) -> f64 {
    match v {
        Value::Number(n) => n.as_f64().unwrap_or(0.0),
        Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
        Value::Bool(b) => {
            if *b {
                1.0
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

/// Convert a `Value` to its string representation.
///
/// Matches Painless `toString()` semantics.
pub fn painless_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Array(_) | Value::Object(_) => v.to_string(),
    }
}

/// Painless equality — null-safe, with type coercion for numbers.
pub fn painless_eq(a: &Value, b: &Value) -> bool {
    if a == b {
        return true;
    }
    // Coerce numeric types for cross-type comparison
    if a.is_number() && b.is_number() {
        return painless_to_f64(a) == painless_to_f64(b);
    }
    // Compare string to number
    if a.is_string() && b.is_number() {
        if let Ok(n) = a.as_str().unwrap_or("").parse::<f64>() {
            return n == painless_to_f64(b);
        }
    }
    if a.is_number() && b.is_string() {
        if let Ok(n) = b.as_str().unwrap_or("").parse::<f64>() {
            return painless_to_f64(a) == n;
        }
    }
    false
}

/// Painless comparison — returns ordering for `<`, `<=`, `>`, `>=`.
pub fn painless_cmp(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
    let a_f = painless_to_f64(a);
    let b_f = painless_to_f64(b);
    a_f.partial_cmp(&b_f)
}

/// Recursive removal of null and empty values from a `Value` tree.
///
/// Used by okta, cisco_nexus, and fortinet `drop` scripts.
/// Returns `true` if the value itself should be removed.
pub fn painless_drop_empty(v: &mut Value) -> bool {
    match v {
        Value::Null => true,
        Value::String(s) if s.is_empty() => true,
        Value::Object(map) => {
            let keys_to_remove: Vec<String> = map
                .iter_mut()
                .filter_map(|(k, v)| {
                    if painless_drop_empty(v) {
                        Some(k.clone())
                    } else {
                        None
                    }
                })
                .collect();
            for k in keys_to_remove {
                map.remove(&k);
            }
            map.is_empty()
        }
        Value::Array(arr) => {
            arr.retain_mut(|item| !painless_drop_empty(item));
            arr.is_empty()
        }
        _ => false,
    }
}

/// Remove entries from a JSON object whose values match sentinel values.
///
/// Used by CrowdStrike and other pipelines that use Painless scripts like:
/// ```painless
/// ctx.crowdstrike.event.entrySet().removeIf(
///     entry -> params.values.contains(entry.getValue())
/// );
/// ```
///
/// The `sentinels` parameter contains values to remove (e.g., `null`, `""`, `"-"`, `"NA"`, `0`).
pub fn remove_sentinel_values(obj: &mut Map<String, Value>, sentinels: &[Value]) {
    obj.retain(|_, v| !sentinels.contains(v));
}

/// Convert a Windows FILETIME / LDAP timestamp to UNIX epoch milliseconds.
///
/// Windows FILETIME uses 100-nanosecond intervals since 1601-01-01.
/// Values above `0x0100000000000000` (72057594037927936) are FILETIME;
/// smaller values are already UNIX timestamps (seconds or milliseconds).
///
/// Used by CrowdStrike for StartTime, EndTime, ContextTimeStamp, etc.
/// Reference: <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>
#[inline]
pub fn filetime_to_unix_ms(value: i64) -> i64 {
    const FILETIME_THRESHOLD: i64 = 0x0100_0000_0000_0000; // 72057594037927936
    const FILETIME_TO_UNIX_OFFSET_MS: i64 = 11_644_473_600_000; // ms between 1601 and 1970

    if value > FILETIME_THRESHOLD {
        (value / 10_000) - FILETIME_TO_UNIX_OFFSET_MS
    } else {
        value
    }
}

/// Deduplicate a JSON array in-place, preserving order.
///
/// Used after multiple `append` calls that may produce duplicates
/// (e.g., related.ip being appended from both source.ip and destination.ip
/// when they're the same address).
pub fn dedup_array(arr: &mut Vec<Value>) {
    let mut seen = Vec::with_capacity(arr.len());
    arr.retain(|v| {
        if seen.contains(v) {
            false
        } else {
            seen.push(v.clone());
            true
        }
    });
}

/// Recursive camelCase-to-snake_case key renaming on a `Value` tree.
///
/// Used by azure_signinlogs `keysToSnakeCase` script.
pub fn painless_keys_to_snake_case(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut out = Map::new();
            for (k, v) in map {
                let snake_key = camel_to_snake(k);
                let converted_v = if v.is_object() {
                    painless_keys_to_snake_case(v)
                } else if let Some(arr) = v.as_array() {
                    Value::Array(
                        arr.iter()
                            .map(|item| {
                                if item.is_object() {
                                    painless_keys_to_snake_case(item)
                                } else {
                                    item.clone()
                                }
                            })
                            .collect(),
                    )
                } else {
                    v.clone()
                };
                out.insert(snake_key, converted_v);
            }
            Value::Object(out)
        }
        Value::Array(arr) => Value::Array(
            arr.iter()
                .map(|item| painless_keys_to_snake_case(item))
                .collect(),
        ),
        _ => v.clone(),
    }
}

/// Convert a camelCase or PascalCase string to snake_case.
fn camel_to_snake(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                // Only insert underscore if previous char is lowercase
                if let Some(prev) = s.chars().nth(i - 1) {
                    if prev.is_lowercase() {
                        result.push('_');
                    }
                }
            }
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        } else {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn truthiness() {
        assert!(!painless_truthy(&Value::Null));
        assert!(!painless_truthy(&json!(false)));
        assert!(!painless_truthy(&json!(0)));
        assert!(!painless_truthy(&json!("")));
        assert!(painless_truthy(&json!(true)));
        assert!(painless_truthy(&json!(1)));
        assert!(painless_truthy(&json!("hello")));
        assert!(painless_truthy(&json!([])));
        assert!(painless_truthy(&json!({})));
    }

    #[test]
    fn addition_numeric() {
        assert_eq!(painless_add(&json!(2), &json!(3)), json!(5));
        assert_eq!(painless_add(&json!(2.5), &json!(1.5)), json!(4.0));
    }

    #[test]
    fn addition_string_concat() {
        assert_eq!(
            painless_add(&json!("hello"), &json!(" world")),
            json!("hello world")
        );
        assert_eq!(
            painless_add(&json!("count: "), &json!(42)),
            json!("count: 42")
        );
    }

    #[test]
    fn multiplication() {
        assert_eq!(painless_mul(&json!(3), &json!(4)), json!(12));
        assert_eq!(
            painless_mul(&json!(1_000_000), &json!(1_000_000_000_i64)),
            json!(1_000_000_000_000_000_i64)
        );
    }

    #[test]
    fn division_integer() {
        assert_eq!(painless_div(&json!(10), &json!(3)), json!(3));
        assert_eq!(painless_div(&json!(10), &json!(0)), Value::Null);
    }

    #[test]
    fn to_i64_conversions() {
        assert_eq!(painless_to_i64(&json!(42)), 42);
        assert_eq!(painless_to_i64(&json!("123")), 123);
        assert_eq!(painless_to_i64(&json!(3.7)), 3);
        assert_eq!(painless_to_i64(&json!(true)), 1);
        assert_eq!(painless_to_i64(&Value::Null), 0);
    }

    #[test]
    fn to_string_conversions() {
        assert_eq!(painless_to_string(&json!("hello")), "hello");
        assert_eq!(painless_to_string(&json!(42)), "42");
        assert_eq!(painless_to_string(&json!(true)), "true");
        assert_eq!(painless_to_string(&Value::Null), "null");
    }

    #[test]
    fn equality() {
        assert!(painless_eq(&json!(1), &json!(1)));
        assert!(painless_eq(&json!(1), &json!(1.0)));
        assert!(painless_eq(&json!("hello"), &json!("hello")));
        assert!(painless_eq(&Value::Null, &Value::Null));
        assert!(!painless_eq(&json!(1), &json!(2)));
        assert!(!painless_eq(&json!("a"), &json!("b")));
    }

    #[test]
    fn drop_empty_recursive() {
        let mut val = json!({
            "a": null,
            "b": "",
            "c": "keep",
            "d": {
                "e": null,
                "f": ""
            },
            "g": [null, "", "keep"]
        });
        painless_drop_empty(&mut val);
        assert_eq!(
            val,
            json!({
                "c": "keep",
                "g": ["keep"]
            })
        );
    }

    #[test]
    fn filetime_to_unix_conversion() {
        // Windows FILETIME for 2023-11-02T10:36:00.000Z
        let ft = 133_433_949_600_000_000_i64;
        let unix_ms = filetime_to_unix_ms(ft);
        let dt = chrono::DateTime::from_timestamp_millis(unix_ms).unwrap();
        assert_eq!(
            dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            "2023-11-02T10:36:00.000Z"
        );
    }

    #[test]
    fn filetime_passthrough_unix() {
        // Already UNIX milliseconds (should pass through)
        let unix_ms = 1_698_918_960_000_i64;
        assert_eq!(filetime_to_unix_ms(unix_ms), unix_ms);
    }

    #[test]
    fn remove_sentinels() {
        let mut map = serde_json::from_value::<Map<String, Value>>(json!({
            "keep": "valid",
            "zero": 0,
            "empty": "",
            "na": "NA",
            "dash": "-",
            "null_val": null,
            "also_keep": 42
        }))
        .unwrap();
        let sentinels = vec![
            Value::Null,
            json!(""),
            json!("-"),
            json!("N/A"),
            json!("NA"),
            json!(0),
        ];
        remove_sentinel_values(&mut map, &sentinels);
        assert_eq!(map.len(), 2);
        assert!(map.contains_key("keep"));
        assert!(map.contains_key("also_keep"));
    }

    #[test]
    fn dedup_array_values() {
        let mut arr = vec![json!("a"), json!("b"), json!("a"), json!("c"), json!("b")];
        dedup_array(&mut arr);
        assert_eq!(arr, vec![json!("a"), json!("b"), json!("c")]);
    }

    #[test]
    fn dedup_array_preserves_order() {
        let mut arr = vec![json!(3), json!(1), json!(2), json!(1), json!(3)];
        dedup_array(&mut arr);
        assert_eq!(arr, vec![json!(3), json!(1), json!(2)]);
    }

    #[test]
    fn keys_to_snake_case_conversion() {
        let input = json!({
            "camelCaseIsCool": "value",
            "PascalCaseIsCooler": {
                "nestedInsideObject": [{"NestedInsideArray": true}]
            }
        });
        let result = painless_keys_to_snake_case(&input);
        assert_eq!(
            result,
            json!({
                "camel_case_is_cool": "value",
                "pascal_case_is_cooler": {
                    "nested_inside_object": [{"nested_inside_array": true}]
                }
            })
        );
    }

    #[test]
    fn camel_to_snake_examples() {
        assert_eq!(camel_to_snake("camelCase"), "camel_case");
        assert_eq!(camel_to_snake("PascalCase"), "pascal_case");
        assert_eq!(camel_to_snake("already_snake"), "already_snake");
        assert_eq!(camel_to_snake("HTTPResponse"), "httpresponse");
        assert_eq!(camel_to_snake("simple"), "simple");
    }
}
