// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Event wrapper with ECS-aware dotted-path field access.
//!
//! Wraps `serde_json::Value` with methods for navigating nested fields
//! using dotted paths (e.g., `source.geo.city_name`). The `set()` method
//! auto-creates intermediate objects when they don't exist.

use serde_json::{Map, Value};

use crate::error::{Result, TransformError};

/// A JSON event with dotted-path field access.
///
/// All field access uses ECS-style dotted paths. For example,
/// `event.get_str("source.geo.city_name")` navigates into
/// `{"source": {"geo": {"city_name": "Sydney"}}}`.
#[derive(Debug, Clone)]
pub struct Event {
    inner: Value,
}

impl Event {
    /// Construct from an owned `serde_json::Value`.
    pub fn new(value: Value) -> Self {
        Self { inner: value }
    }

    /// Parse from a JSON string using `serde_json`.
    pub fn from_json(json: &str) -> Result<Self> {
        let value: Value = serde_json::from_str(json)?;
        Ok(Self { inner: value })
    }

    /// Parse from a mutable byte buffer using `simd_json`.
    ///
    /// This is the primary Kafka ingestion path — 2-3x faster than serde_json.
    pub fn from_bytes(buf: &mut [u8]) -> Result<Self> {
        let owned = simd_json::to_owned_value(buf).map_err(|e| TransformError::ParseError {
            path: String::new(),
            message: e.to_string(),
        })?;
        // Convert simd_json OwnedValue → serde_json Value via serde
        let value: Value =
            serde_json::to_value(&owned).map_err(|e| TransformError::ParseError {
                path: String::new(),
                message: e.to_string(),
            })?;
        Ok(Self { inner: value })
    }

    /// Borrow the inner value.
    pub fn as_value(&self) -> &Value {
        &self.inner
    }

    /// Consume the event and return the inner value.
    pub fn into_value(self) -> Value {
        self.inner
    }

    // -- Getters --------------------------------------------------------

    /// Get a reference to the value at a dotted path.
    pub fn get(&self, path: &str) -> Option<&Value> {
        resolve_path(&self.inner, path)
    }

    /// Get a string value at a dotted path.
    pub fn get_str(&self, path: &str) -> Option<&str> {
        self.get(path).and_then(Value::as_str)
    }

    /// Get an i64 value at a dotted path.
    pub fn get_i64(&self, path: &str) -> Option<i64> {
        self.get(path).and_then(Value::as_i64)
    }

    /// Get an f64 value at a dotted path.
    pub fn get_f64(&self, path: &str) -> Option<f64> {
        self.get(path).and_then(Value::as_f64)
    }

    /// Get a bool value at a dotted path.
    pub fn get_bool(&self, path: &str) -> Option<bool> {
        self.get(path).and_then(Value::as_bool)
    }

    /// Get an array value at a dotted path.
    pub fn get_array(&self, path: &str) -> Option<&Vec<Value>> {
        self.get(path).and_then(Value::as_array)
    }

    /// Get an object value at a dotted path.
    pub fn get_object(&self, path: &str) -> Option<&Map<String, Value>> {
        self.get(path).and_then(Value::as_object)
    }

    /// Check whether a field exists at the given path.
    pub fn has(&self, path: &str) -> bool {
        self.get(path).is_some()
    }

    // -- Setters --------------------------------------------------------

    /// Set a value at a dotted path, creating intermediate objects as needed.
    pub fn set(&mut self, path: &str, value: impl Into<Value>) -> Result<()> {
        let value = value.into();
        let segments: Vec<&str> = path.split('.').collect();

        let mut current = &mut self.inner;
        for (i, segment) in segments.iter().enumerate() {
            if i == segments.len() - 1 {
                // Final segment — set the value
                if current.is_null() {
                    *current = Value::Object(Map::new());
                }
                match current {
                    Value::Object(map) => {
                        map.insert((*segment).to_string(), value);
                        return Ok(());
                    }
                    _ => {
                        return Err(TransformError::TypeMismatch {
                            path: segments[..i].join("."),
                            expected: "object",
                            actual: type_name(current),
                        });
                    }
                }
            }

            // Intermediate segment — navigate or create object
            // If current is null, promote it to an empty object (matches Elasticsearch behaviour)
            if current.is_null() {
                *current = Value::Object(Map::new());
            }
            match current {
                Value::Object(map) => {
                    current = map
                        .entry((*segment).to_string())
                        .or_insert_with(|| Value::Object(Map::new()));
                }
                _ => {
                    return Err(TransformError::TypeMismatch {
                        path: segments[..i].join("."),
                        expected: "object",
                        actual: type_name(current),
                    });
                }
            }
        }

        Ok(())
    }

    /// Remove a value at a dotted path, returning it if it existed.
    pub fn remove(&mut self, path: &str) -> Option<Value> {
        let segments: Vec<&str> = path.split('.').collect();
        if segments.is_empty() {
            return None;
        }

        let mut current = &mut self.inner;
        for segment in &segments[..segments.len() - 1] {
            match current {
                Value::Object(map) => {
                    current = map.get_mut(*segment)?;
                }
                _ => return None,
            }
        }

        let last = segments.last()?;
        match current {
            Value::Object(map) => map.remove(*last),
            _ => None,
        }
    }

    /// Rename a field from one path to another.
    ///
    /// Removes the value at `from` and sets it at `to`. Intermediate objects
    /// are created at the destination as needed.
    pub fn rename(&mut self, from: &str, to: &str) -> Result<()> {
        match self.remove(from) {
            Some(value) => self.set(to, value),
            None => Err(TransformError::FieldNotFound {
                path: from.to_string(),
            }),
        }
    }

    // -- Array operations -----------------------------------------------

    /// Append a value to an array at the given path.
    ///
    /// If the field doesn't exist, creates a new array. If it exists but isn't
    /// an array, returns an error.
    pub fn append(&mut self, path: &str, value: impl Into<Value>) -> Result<()> {
        let value = value.into();

        match self.get(path) {
            None => {
                // Create a new array with the value
                self.set(path, Value::Array(vec![value]))?;
            }
            Some(existing) => {
                if existing.is_array() {
                    // Navigate to the array and push
                    let target = resolve_path_mut(&mut self.inner, path).unwrap();
                    target.as_array_mut().unwrap().push(value);
                } else {
                    // Wrap existing scalar in an array, then append
                    let existing_clone = existing.clone();
                    self.set(path, Value::Array(vec![existing_clone, value]))?;
                }
            }
        }

        Ok(())
    }

    /// Merge another event into this one.
    ///
    /// With `deep = false`, top-level keys from `other` overwrite this event.
    /// With `deep = true`, nested objects are merged recursively.
    pub fn merge(&mut self, other: &Event, deep: bool) -> Result<()> {
        if deep {
            deep_merge(&mut self.inner, &other.inner);
        } else {
            shallow_merge(&mut self.inner, &other.inner);
        }
        Ok(())
    }
}

// -- Conversions ---------------------------------------------------------

impl From<Value> for Event {
    fn from(value: Value) -> Self {
        Self::new(value)
    }
}

impl From<Event> for Value {
    fn from(event: Event) -> Self {
        event.inner
    }
}

// -- Internal helpers ----------------------------------------------------

/// Walk a dotted path to find an immutable reference to the target value.
fn resolve_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for segment in path.split('.') {
        match current {
            Value::Object(map) => {
                current = map.get(segment)?;
            }
            _ => return None,
        }
    }
    Some(current)
}

/// Walk a dotted path to find a mutable reference to the target value.
fn resolve_path_mut<'a>(value: &'a mut Value, path: &str) -> Option<&'a mut Value> {
    let mut current = value;
    for segment in path.split('.') {
        match current {
            Value::Object(map) => {
                current = map.get_mut(segment)?;
            }
            _ => return None,
        }
    }
    Some(current)
}

/// Return a human-readable type name for a JSON value.
fn type_name(value: &Value) -> String {
    match value {
        Value::Null => "null".into(),
        Value::Bool(_) => "bool".into(),
        Value::Number(_) => "number".into(),
        Value::String(_) => "string".into(),
        Value::Array(_) => "array".into(),
        Value::Object(_) => "object".into(),
    }
}

/// Deep merge: recursively merge objects, overwrite non-object values.
fn deep_merge(target: &mut Value, source: &Value) {
    match (target, source) {
        (Value::Object(target_map), Value::Object(source_map)) => {
            for (key, source_val) in source_map {
                let target_val = target_map.entry(key.clone()).or_insert(Value::Null);
                deep_merge(target_val, source_val);
            }
        }
        (target, source) => {
            *target = source.clone();
        }
    }
}

/// Shallow merge: overwrite top-level keys only.
fn shallow_merge(target: &mut Value, source: &Value) {
    if let (Value::Object(target_map), Value::Object(source_map)) = (target, source) {
        for (key, val) in source_map {
            target_map.insert(key.clone(), val.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // -- Constructor tests -----------------------------------------------

    #[test]
    fn new_from_value() {
        let event = Event::new(json!({"message": "hello"}));
        assert_eq!(event.get_str("message"), Some("hello"));
    }

    #[test]
    fn from_json_valid() {
        let event = Event::from_json(r#"{"a": {"b": 42}}"#).unwrap();
        assert_eq!(event.get_i64("a.b"), Some(42));
    }

    #[test]
    fn from_json_invalid() {
        assert!(Event::from_json("not json").is_err());
    }

    #[test]
    fn from_bytes_valid() {
        let mut buf = br#"{"key": "value"}"#.to_vec();
        let event = Event::from_bytes(&mut buf).unwrap();
        assert_eq!(event.get_str("key"), Some("value"));
    }

    #[test]
    fn into_value_roundtrip() {
        let original = json!({"x": 1});
        let event = Event::new(original.clone());
        assert_eq!(event.into_value(), original);
    }

    // -- Getter tests ----------------------------------------------------

    #[test]
    fn get_nested_path() {
        let event = Event::new(json!({"a": {"b": {"c": "deep"}}}));
        assert_eq!(event.get_str("a.b.c"), Some("deep"));
    }

    #[test]
    fn get_missing_intermediate() {
        let event = Event::new(json!({"a": 1}));
        assert!(event.get("a.b.c").is_none());
    }

    #[test]
    fn get_typed_values() {
        let event = Event::new(json!({
            "s": "hello",
            "i": 42,
            "f": 3.14,
            "b": true,
            "arr": [1, 2, 3],
            "obj": {"k": "v"}
        }));

        assert_eq!(event.get_str("s"), Some("hello"));
        assert_eq!(event.get_i64("i"), Some(42));
        assert_eq!(event.get_f64("f"), Some(3.14));
        assert_eq!(event.get_bool("b"), Some(true));
        assert_eq!(event.get_array("arr").map(|a| a.len()), Some(3));
        assert!(event.get_object("obj").is_some());
    }

    #[test]
    fn get_type_mismatch_returns_none() {
        let event = Event::new(json!({"s": "hello"}));
        assert_eq!(event.get_i64("s"), None);
        assert_eq!(event.get_bool("s"), None);
    }

    #[test]
    fn has_existing_and_missing() {
        let event = Event::new(json!({"a": {"b": 1}}));
        assert!(event.has("a.b"));
        assert!(event.has("a"));
        assert!(!event.has("a.c"));
        assert!(!event.has("x"));
    }

    // -- Setter tests ----------------------------------------------------

    #[test]
    fn set_creates_intermediate_objects() {
        let mut event = Event::new(json!({}));
        event.set("a.b.c", "deep").unwrap();
        assert_eq!(event.get_str("a.b.c"), Some("deep"));
    }

    #[test]
    fn set_overwrites_existing() {
        let mut event = Event::new(json!({"a": "old"}));
        event.set("a", "new").unwrap();
        assert_eq!(event.get_str("a"), Some("new"));
    }

    #[test]
    fn set_into_non_object_fails() {
        let mut event = Event::new(json!({"a": "string"}));
        let result = event.set("a.b", "value");
        assert!(result.is_err());
    }

    #[test]
    fn remove_existing() {
        let mut event = Event::new(json!({"a": {"b": 1, "c": 2}}));
        let removed = event.remove("a.b");
        assert_eq!(removed, Some(json!(1)));
        assert!(!event.has("a.b"));
        assert!(event.has("a.c"));
    }

    #[test]
    fn remove_missing_returns_none() {
        let mut event = Event::new(json!({"a": 1}));
        assert_eq!(event.remove("b"), None);
        assert_eq!(event.remove("a.b.c"), None);
    }

    #[test]
    fn rename_moves_value() {
        let mut event = Event::new(json!({"old": {"key": "value"}}));
        event.rename("old.key", "new.key").unwrap();
        assert!(!event.has("old.key"));
        assert_eq!(event.get_str("new.key"), Some("value"));
    }

    #[test]
    fn rename_missing_field_errors() {
        let mut event = Event::new(json!({}));
        assert!(event.rename("missing", "dest").is_err());
    }

    // -- Array + merge tests ---------------------------------------------

    #[test]
    fn append_to_existing_array() {
        let mut event = Event::new(json!({"tags": ["a", "b"]}));
        event.append("tags", "c").unwrap();
        assert_eq!(event.get_array("tags").map(|a| a.len()), Some(3));
    }

    #[test]
    fn append_creates_array_if_missing() {
        let mut event = Event::new(json!({}));
        event.append("tags", "first").unwrap();
        let arr = event.get_array("tags").unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], json!("first"));
    }

    #[test]
    fn append_to_non_array_wraps_in_array() {
        let mut event = Event::new(json!({"tags": "existing"}));
        event.append("tags", "new_value").unwrap();
        let arr = event.get_array("tags").unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0].as_str().unwrap(), "existing");
        assert_eq!(arr[1].as_str().unwrap(), "new_value");
    }

    #[test]
    fn merge_shallow() {
        let mut event = Event::new(json!({"a": 1, "b": {"x": 1}}));
        let other = Event::new(json!({"b": {"y": 2}, "c": 3}));
        event.merge(&other, false).unwrap();

        assert_eq!(event.get_i64("a"), Some(1));
        // Shallow merge: b is overwritten entirely
        assert!(!event.has("b.x"));
        assert_eq!(event.get_i64("b.y"), Some(2));
        assert_eq!(event.get_i64("c"), Some(3));
    }

    #[test]
    fn merge_deep() {
        let mut event = Event::new(json!({"a": 1, "b": {"x": 1}}));
        let other = Event::new(json!({"b": {"y": 2}, "c": 3}));
        event.merge(&other, true).unwrap();

        assert_eq!(event.get_i64("a"), Some(1));
        // Deep merge: b.x preserved, b.y added
        assert_eq!(event.get_i64("b.x"), Some(1));
        assert_eq!(event.get_i64("b.y"), Some(2));
        assert_eq!(event.get_i64("c"), Some(3));
    }

    #[test]
    fn merge_deep_overwrites_non_object() {
        let mut event = Event::new(json!({"a": {"b": "old"}}));
        let other = Event::new(json!({"a": {"b": "new"}}));
        event.merge(&other, true).unwrap();
        assert_eq!(event.get_str("a.b"), Some("new"));
    }
}
