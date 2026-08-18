// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! JSON flattening and unflattening.
//!
//! Converts between nested JSON objects and flat dot-notation maps.
//! The Elastic test fixtures use dot-notation keys in expected output,
//! so flattening is needed to compare transformed events.

use serde_json::{Map, Value};

/// Flatten a nested JSON value into dot-notation keys.
///
/// Arrays are preserved as values (not flattened into `key.0`, `key.1`).
/// Only objects are recursively flattened.
///
/// ```text
/// {"a": {"b": 1, "c": [2, 3]}} => {"a.b": 1, "a.c": [2, 3]}
/// ```
pub fn flatten_value(value: &Value) -> Map<String, Value> {
    let mut result = Map::new();
    if let Value::Object(map) = value {
        flatten_inner(map, "", &mut result);
    }
    result
}

fn flatten_inner(map: &Map<String, Value>, prefix: &str, result: &mut Map<String, Value>) {
    for (key, value) in map {
        let full_key = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}.{key}")
        };

        match value {
            Value::Object(inner) if !inner.is_empty() => {
                flatten_inner(inner, &full_key, result);
            }
            _ => {
                result.insert(full_key, value.clone());
            }
        }
    }
}

/// Unflatten dot-notation keys back into nested JSON objects.
///
/// ```text
/// {"a.b": 1, "a.c": [2, 3]} => {"a": {"b": 1, "c": [2, 3]}}
/// ```
pub fn unflatten_value(flat: &Map<String, Value>) -> Value {
    let mut root = Value::Object(Map::new());

    for (dotted_key, value) in flat {
        let segments: Vec<&str> = dotted_key.split('.').collect();
        let mut current = &mut root;

        for (i, segment) in segments.iter().enumerate() {
            if i == segments.len() - 1 {
                if let Value::Object(map) = current {
                    map.insert((*segment).to_string(), value.clone());
                }
            } else if let Value::Object(map) = current {
                current = map
                    .entry((*segment).to_string())
                    .or_insert_with(|| Value::Object(Map::new()));
            }
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn flatten_simple_nested() {
        let value = json!({"a": {"b": 1, "c": "hello"}});
        let flat = flatten_value(&value);
        assert_eq!(flat.get("a.b"), Some(&json!(1)));
        assert_eq!(flat.get("a.c"), Some(&json!("hello")));
        assert_eq!(flat.len(), 2);
    }

    #[test]
    fn flatten_deep_nested() {
        let value = json!({"a": {"b": {"c": {"d": true}}}});
        let flat = flatten_value(&value);
        assert_eq!(flat.get("a.b.c.d"), Some(&json!(true)));
        assert_eq!(flat.len(), 1);
    }

    #[test]
    fn flatten_preserves_arrays() {
        let value = json!({"tags": ["a", "b"], "nested": {"arr": [1, 2]}});
        let flat = flatten_value(&value);
        assert_eq!(flat.get("tags"), Some(&json!(["a", "b"])));
        assert_eq!(flat.get("nested.arr"), Some(&json!([1, 2])));
    }

    #[test]
    fn flatten_empty_object() {
        let value = json!({});
        let flat = flatten_value(&value);
        assert!(flat.is_empty());
    }

    #[test]
    fn flatten_preserves_empty_object_leaf() {
        let value = json!({"a": {"b": {}}});
        let flat = flatten_value(&value);
        assert_eq!(flat.get("a.b"), Some(&json!({})));
    }

    #[test]
    fn flatten_top_level_scalars() {
        let value = json!({"x": 1, "y": "two", "z": null});
        let flat = flatten_value(&value);
        assert_eq!(flat.get("x"), Some(&json!(1)));
        assert_eq!(flat.get("y"), Some(&json!("two")));
        assert_eq!(flat.get("z"), Some(&json!(null)));
    }

    #[test]
    fn unflatten_simple() {
        let mut flat = Map::new();
        flat.insert("a.b".to_string(), json!(1));
        flat.insert("a.c".to_string(), json!("hello"));
        let nested = unflatten_value(&flat);
        assert_eq!(nested, json!({"a": {"b": 1, "c": "hello"}}));
    }

    #[test]
    fn unflatten_deep() {
        let mut flat = Map::new();
        flat.insert("a.b.c.d".to_string(), json!(true));
        let nested = unflatten_value(&flat);
        assert_eq!(nested, json!({"a": {"b": {"c": {"d": true}}}}));
    }

    #[test]
    fn roundtrip_flatten_unflatten() {
        let original = json!({
            "event": {"kind": "event", "category": "web"},
            "source": {"ip": "1.2.3.4"},
            "tags": ["forwarded"]
        });
        let flat = flatten_value(&original);
        let restored = unflatten_value(&flat);
        assert_eq!(original, restored);
    }

    #[test]
    fn flatten_non_object_returns_empty() {
        assert!(flatten_value(&json!("string")).is_empty());
        assert!(flatten_value(&json!(42)).is_empty());
        assert!(flatten_value(&json!(null)).is_empty());
    }
}
