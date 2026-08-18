// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Painless script params → inlined Rust literal conversion.
//!
//! Pipeline YAML scripts often include a `params:` block with static
//! configuration (enum mappings, field lists, timezone tables). Since
//! params are known at codegen time, we inline them directly as Rust
//! constants in the generated code.

use serde_json::Value;

/// Convert a `serde_yaml_ng::Value` params block to a `serde_json::Value`.
///
/// This is needed because pipeline YAML is parsed as `serde_yaml_ng::Value`
/// but our IR and emitter work with `serde_json::Value`.
pub fn yaml_to_json(yaml: &serde_yaml_ng::Value) -> Value {
    match yaml {
        serde_yaml_ng::Value::Null => Value::Null,
        serde_yaml_ng::Value::Bool(b) => Value::Bool(*b),
        serde_yaml_ng::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Number(i.into())
            } else if let Some(u) = n.as_u64() {
                Value::Number(u.into())
            } else if let Some(f) = n.as_f64() {
                serde_json::Number::from_f64(f).map_or(Value::Null, Value::Number)
            } else {
                Value::Null
            }
        }
        serde_yaml_ng::Value::String(s) => Value::String(s.clone()),
        serde_yaml_ng::Value::Sequence(seq) => Value::Array(seq.iter().map(yaml_to_json).collect()),
        serde_yaml_ng::Value::Mapping(map) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in map {
                let key = match k {
                    serde_yaml_ng::Value::String(s) => s.clone(),
                    serde_yaml_ng::Value::Number(n) => n.to_string(),
                    serde_yaml_ng::Value::Bool(b) => b.to_string(),
                    _ => continue,
                };
                obj.insert(key, yaml_to_json(v));
            }
            Value::Object(obj)
        }
        serde_yaml_ng::Value::Tagged(tagged) => yaml_to_json(&tagged.value),
    }
}

/// Emit a `serde_json::Value` as a Rust source literal using `json!()`.
///
/// Used to inline params values directly in generated code.
pub fn emit_json_literal(value: &Value) -> String {
    match value {
        Value::Null => "Value::Null".to_string(),
        Value::Bool(b) => format!("json!({b})"),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                // Handle hex-like values (e.g., 0x80000000) which are common in params
                format!("json!({i}_i64)")
            } else if let Some(f) = n.as_f64() {
                format!("json!({f}_f64)")
            } else {
                format!("json!({n})")
            }
        }
        Value::String(s) => {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
            format!("json!(\"{escaped}\")")
        }
        Value::Array(arr) => {
            let elems: Vec<String> = arr.iter().map(emit_json_literal).collect();
            format!("json!([{}])", elems.join(", "))
        }
        Value::Object(map) => {
            if map.is_empty() {
                return "Value::Object(serde_json::Map::new())".to_string();
            }
            let entries: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    let escaped_key = k.replace('\\', "\\\\").replace('"', "\\\"");
                    format!("\"{escaped_key}\": {}", emit_json_literal(v))
                })
                .collect();
            format!("json!({{{}}})", entries.join(", "))
        }
    }
}

/// Emit a params block as a `lazy_static!` Rust declaration.
///
/// For simple scalar params, callers may prefer to inline directly.
/// This function is for complex params (maps, arrays).
pub fn emit_params_lazy_static(params: &Value, var_name: &str) -> String {
    let literal = emit_json_literal(params);
    format!(
        "lazy_static::lazy_static! {{\n    \
         static ref {var_name}: Value = {literal};\n\
         }}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn yaml_null_to_json() {
        assert_eq!(yaml_to_json(&serde_yaml_ng::Value::Null), Value::Null);
    }

    #[test]
    fn yaml_scalar_to_json() {
        let yaml: serde_yaml_ng::Value = serde_yaml_ng::from_str("42").unwrap();
        assert_eq!(yaml_to_json(&yaml), json!(42));
    }

    #[test]
    fn yaml_map_to_json() {
        let yaml: serde_yaml_ng::Value =
            serde_yaml_ng::from_str("key: value\nnested:\n  a: 1").unwrap();
        let result = yaml_to_json(&yaml);
        assert_eq!(result, json!({"key": "value", "nested": {"a": 1}}));
    }

    #[test]
    fn emit_literal_string() {
        assert_eq!(emit_json_literal(&json!("hello")), "json!(\"hello\")");
    }

    #[test]
    fn emit_literal_number() {
        assert_eq!(emit_json_literal(&json!(42)), "json!(42_i64)");
    }

    #[test]
    fn emit_literal_array() {
        let result = emit_json_literal(&json!(["a", "b"]));
        assert!(result.contains("json!(["));
        assert!(result.contains("\"a\""));
    }

    #[test]
    fn emit_params_block() {
        let params = json!({"param_nano": 1000000000_i64});
        let result = emit_params_lazy_static(&params, "PARAMS");
        assert!(result.contains("lazy_static!"));
        assert!(result.contains("static ref PARAMS"));
    }
}
