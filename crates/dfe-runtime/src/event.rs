// SPDX-License-Identifier: BUSL-1.1
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
#[derive(Debug, Clone, Default)]
pub struct Event {
    inner: Value,
    capacities: Vec<(String, usize)>,
}

impl Event {
    /// Construct from an owned `serde_json::Value`.
    pub fn new(value: Value) -> Self {
        Self {
            inner: value,
            capacities: Vec::new(),
        }
    }

    /// Parse from a JSON string using `serde_json`.
    pub fn from_json(json: &str) -> Result<Self> {
        let value: Value = serde_json::from_str(json)?;
        Ok(Self::new(value))
    }

    /// Parse from a byte buffer. The primary Kafka ingestion path.
    ///
    /// `serde_json`, not simd-json. Building a simd-json `OwnedValue` and then
    /// converting it with `serde_json::to_value` is two complete trees for one
    /// document, and it measures 1.7x slower on an okta payload and 2.7x on a
    /// small one (`benches/json_parse.rs`). simd-json's advantage is its own
    /// tape; it does not survive the conversion into the `IndexMap`-backed
    /// tree `preserve_order` requires.
    ///
    /// Takes `&mut [u8]` still, so a caller holding a scratch buffer needs no
    /// change.
    pub fn from_bytes(buf: &mut [u8]) -> Result<Self> {
        let value: Value = serde_json::from_slice(buf).map_err(|e| TransformError::ParseError {
            path: String::new(),
            message: e.to_string(),
        })?;
        Ok(Self::new(value))
    }

    /// Borrow the inner value.
    pub fn as_value(&self) -> &Value {
        &self.inner
    }

    /// Record the entry count a map held BEFORE a prune shrank it.
    ///
    /// A Java `HashMap` never shrinks its table on remove, so a pruned map's
    /// `toString` still renders through the buckets it had when it was built.
    /// Only the maps whose table the prune actually changed are recorded -- 5
    /// of the 1,007 in the cloudtrail corpus -- so the list is empty for
    /// nearly every event and costs one integer compare per pruned map.
    pub fn record_map_capacity(&mut self, path: String, entries: usize) {
        self.capacities.push((path, entries));
    }

    /// The recorded pre-prune entry count for the map at `path`.
    pub fn map_capacity(&self, path: &str) -> Option<usize> {
        self.capacities
            .iter()
            .find(|(recorded, _)| recorded == path)
            .map(|(_, entries)| *entries)
    }

    /// Whether any map's table survived a prune that shrank it.
    pub fn has_map_capacities(&self) -> bool {
        !self.capacities.is_empty()
    }

    /// Mutably borrow the inner value.
    pub fn as_value_mut(&mut self) -> &mut Value {
        &mut self.inner
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

    /// Get a string value at a dotted path (borrowed).
    pub fn get_str(&self, path: &str) -> Option<&str> {
        self.get(path).and_then(Value::as_str)
    }

    /// Get a string value at a dotted path (owned).
    ///
    /// Returns an owned `String` to avoid borrow conflicts when
    /// the caller needs to mutate the event after reading.
    pub fn get_string(&self, path: &str) -> Option<String> {
        self.get_str(path).map(String::from)
    }

    /// Get any value at a dotted path as a string representation.
    ///
    /// Works for strings, numbers, and booleans. Useful for fields
    /// that may be stored as either string or number (e.g., epoch timestamps).
    pub fn get_as_string(&self, path: &str) -> Option<String> {
        self.get(path).and_then(|v| match v {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            _ => None,
        })
    }

    /// Get an i64 value at a dotted path.
    pub fn get_i64(&self, path: &str) -> Option<i64> {
        self.get(path).and_then(Value::as_i64)
    }

    /// Get any value at a dotted path as an `i64`.
    ///
    /// The mirror of [`Self::get_as_string`]: a vendor field that came out of
    /// a grok is a string even when it holds a number, and a `convert`
    /// processor may or may not have run over it yet.
    pub fn get_as_i64(&self, path: &str) -> Option<i64> {
        self.get(path).and_then(|v| match v {
            Value::Number(n) => n.as_i64(),
            Value::String(s) => s.trim().parse::<i64>().ok(),
            Value::Bool(b) => Some(i64::from(*b)),
            _ => None,
        })
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

    /// Check whether a KEY exists at the given path, null or not.
    ///
    /// This is Painless `containsKey`. For `!= null`, which an explicit null
    /// fails, use [`Event::has_value`].
    pub fn has(&self, path: &str) -> bool {
        self.get(path).is_some()
    }

    /// Check whether a field holds a non-null value at the given path.
    ///
    /// Painless `ctx.a?.b != null` is FALSE when `b` is present and null, and
    /// a pipeline that writes a null placeholder earlier -- which several do,
    /// via `set` with an absent `copy_from` -- would otherwise open every gate
    /// downstream of it.
    pub fn has_value(&self, path: &str) -> bool {
        !matches!(self.get(path), None | Some(Value::Null))
    }

    // -- Setters --------------------------------------------------------

    /// Set a value at a dotted path, creating intermediate objects as needed.
    ///
    /// On the hot path this allocates NOTHING for the path itself. The
    /// segments are walked as an iterator rather than collected, and a key is
    /// only turned into a `String` when it has to be inserted -- which, after
    /// the first event of a batch, it usually does not.
    pub fn set(&mut self, path: &str, value: impl Into<Value>) -> Result<()> {
        let value = value.into();
        let mut segments = path.split('.').peekable();
        let mut depth = 0;

        let mut current = &mut self.inner;
        while let Some(segment) = segments.next() {
            // A null is promoted to an object, matching Elasticsearch.
            if current.is_null() {
                *current = Value::Object(Map::new());
            }
            let Value::Object(map) = current else {
                return Err(TransformError::TypeMismatch {
                    path: prefix_of(path, depth),
                    expected: "object",
                    actual: type_name(current),
                });
            };

            if segments.peek().is_none() {
                // Final segment. Overwrite in place where the key exists, so
                // the common case does not allocate a duplicate key.
                if let Some(slot) = map.get_mut(segment) {
                    *slot = value;
                } else {
                    map.insert(segment.to_string(), value);
                }
                return Ok(());
            }

            if map.get(segment).is_none() {
                map.insert(segment.to_string(), Value::Object(Map::new()));
            }
            let Some(next) = map.get_mut(segment) else {
                // Unreachable: just inserted above. Reported rather than
                // panicked, because a panic here takes the pod.
                return Err(TransformError::TypeMismatch {
                    path: prefix_of(path, depth),
                    expected: "object",
                    actual: "missing".to_string(),
                });
            };
            current = next;
            depth += 1;
        }

        Ok(())
    }

    /// Remove a value at a dotted path, returning it if it existed.
    ///
    /// The walk is iterator-only. Collecting the segments into a `Vec` first
    /// put one heap allocation on every call, and a generated transform calls
    /// this constantly -- the sampled profile charges it 78 allocations an
    /// event on okta and 56 on fortinet, none of which bought anything.
    pub fn remove(&mut self, path: &str) -> Option<Value> {
        let (parents, last) = path
            .rsplit_once('.')
            .map_or((None, path), |(head, last)| (Some(head), last));

        let mut current = &mut self.inner;
        if let Some(parents) = parents {
            for segment in parents.split('.') {
                let Value::Object(map) = current else {
                    return None;
                };
                current = map.get_mut(segment)?;
            }
        }

        match current {
            // `shift_remove`, never `remove`: under `preserve_order` the plain
            // one is `swap_remove`, which moves the LAST key into the freed
            // slot. Insertion order is what parity rests on -- Elasticsearch's
            // maps are insertion-ordered and Painless renders them in that
            // order -- so a swap here reorders the document.
            Value::Object(map) => map.shift_remove(last),
            _ => None,
        }
    }

    /// Move the ARRAY at a dotted path out, leaving anything else alone.
    ///
    /// The zero-copy half of the foreach machinery: the elements MOVE out
    /// instead of being cloned, one array's worth of allocation saved per
    /// call. A value that is not an array is left exactly where it was, and
    /// the slot itself keeps an empty array until the caller writes the
    /// processed list back.
    ///
    /// Resolves the path the way [`Self::get`] does, flat keys included.
    pub fn take_array(&mut self, path: &str) -> Option<Vec<Value>> {
        let slot = resolve_path_mut(&mut self.inner, path)?;
        let Value::Array(items) = slot else {
            return None;
        };
        Some(std::mem::take(items))
    }

    /// Rename a field from one path to another, the way Elasticsearch does.
    ///
    /// An ABSENT source and an OCCUPIED target are both failures there --
    /// `field [x] doesn't exist` and `field [y] already exists` -- and only
    /// `ignore_missing`, `ignore_failure` or an `on_failure` handler swallows
    /// one. Overwriting instead let a rename clobber a value the vendor
    /// pipeline leaves alone: aws `ec2_metrics` ships six whose targets the
    /// agent has already filled, and each replaced a computed reading
    /// (`host.cpu.usage` 0.421) with the raw metric (42).
    ///
    /// [`Self::rename_over`] is the `override: true` spelling, which does
    /// replace the target.
    pub fn rename(&mut self, from: &str, to: &str) -> Result<()> {
        if self.has(to) {
            return Err(TransformError::FieldExists {
                path: to.to_string(),
            });
        }
        self.rename_over(from, to)
    }

    /// Rename a field, replacing whatever the target already held.
    ///
    /// The `override: true` reading. Three of the 4,105 renames the vendored
    /// pipelines ship ask for it.
    pub fn rename_over(&mut self, from: &str, to: &str) -> Result<()> {
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

        // One mutable walk: it answers whether the field is already an array and
        // yields the reference to push through, so the path is walked once.
        if let Some(existing) = resolve_path_mut(&mut self.inner, path) {
            if let Value::Array(array) = existing {
                array.push(value);
            } else {
                let first = std::mem::take(existing);
                *existing = Value::Array(vec![first, value]);
            }
            return Ok(());
        }

        self.set(path, Value::Array(vec![value]))
    }

    /// Append a value only if the array does not already hold it.
    ///
    /// Elastic's `append` with `allow_duplicates: false`. Azure's signinlogs
    /// pipeline appends six different fields into `related.entity` and several
    /// of them carry the same id, so appending regardless writes it twice.
    ///
    /// # Errors
    ///
    /// Returns [`TransformError`] if the value cannot be set.
    pub fn append_unique(&mut self, path: &str, value: impl Into<Value>) -> Result<()> {
        let value = value.into();
        match self.get(path) {
            Some(Value::Array(existing)) if existing.contains(&value) => Ok(()),
            Some(existing) if *existing == value => Ok(()),
            _ => self.append(path, value),
        }
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

/// The first `depth` segments of a dotted path, for an error message.
///
/// Only called on the error path, so it can afford to allocate.
fn prefix_of(path: &str, depth: usize) -> String {
    path.split('.').take(depth).collect::<Vec<_>>().join(".")
}

/// Walk a dotted path to find an immutable reference to the target value.
fn resolve_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    let mut rest = path;
    loop {
        // A numeric segment on an array is an index, the way Elastic's
        // mustache reads `{{_temp.user_parts.0}}`.
        if let Value::Array(items) = current {
            let (segment, tail) = rest.split_once('.').unwrap_or((rest, ""));
            let index: usize = segment.parse().ok()?;
            let next = items.get(index)?;
            if tail.is_empty() {
                return Some(next);
            }
            current = next;
            rest = tail;
            continue;
        }
        let Value::Object(map) = current else {
            return None;
        };
        let Some((segment, tail)) = rest.split_once('.') else {
            return map.get(rest);
        };
        let Some(next) = map.get(segment) else {
            // The segment is not a field, so the dot may sit INSIDE a key --
            // see flat_key.
            if let Some(found) = map.get(rest) {
                return Some(found);
            }
            let key = flat_key(map, rest)?;
            rest = &rest[key.len() + 1..];
            current = map.get(key)?;
            continue;
        };
        current = next;
        rest = tail;
    }
}

/// Walk a dotted path to find a mutable reference to the target value.
///
/// Must resolve exactly what [`resolve_path`] does, flat keys included, so that
/// a field readable through `get` is reachable for mutation.
fn resolve_path_mut<'a>(value: &'a mut Value, path: &str) -> Option<&'a mut Value> {
    let mut current = value;
    let mut rest = path;
    loop {
        if current.is_array() {
            let (segment, tail) = rest.split_once('.').unwrap_or((rest, ""));
            let index: usize = segment.parse().ok()?;
            let next = current.as_array_mut()?.get_mut(index)?;
            if tail.is_empty() {
                return Some(next);
            }
            current = next;
            rest = tail;
            continue;
        }
        let Value::Object(map) = current else {
            return None;
        };
        let Some((segment, tail)) = rest.split_once('.') else {
            return map.get_mut(rest);
        };
        // The borrow checker will not let the miss branch look in `map` again
        // after `get_mut`, so the decision is made before anything is taken.
        let key: String = if map.contains_key(segment) {
            rest = tail;
            segment.to_string()
        } else if map.contains_key(rest) {
            return map.get_mut(rest);
        } else {
            let key = flat_key(map, rest)?.to_string();
            rest = &rest[key.len() + 1..];
            key
        };
        current = map.get_mut(&key)?;
    }
}

/// The longest key in `map` that spells the head of `rest`, for a key holding
/// dots of its own.
///
/// A dotted path cannot say whether a dot separates two fields or sits inside
/// one name, and Azure's SAML claims are keyed by URI -- the whole of
/// `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname` is ONE key,
/// with four dots in it, so splitting on them found nothing. Nested wins where
/// both readings exist, which is what Painless does: this runs only after the
/// plain segment lookup has already missed, so an ordinary path never pays for
/// it.
fn flat_key<'m>(map: &'m serde_json::Map<String, Value>, rest: &str) -> Option<&'m str> {
    map.keys()
        .filter(|k| {
            k.len() < rest.len() && rest.as_bytes()[k.len()] == b'.' && rest.starts_with(k.as_str())
        })
        .max_by_key(|k| k.len())
        .map(String::as_str)
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
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
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

    /// A numeric segment indexes an array, which is what a `set` template
    /// spelled `{{_temp.user_parts.0}}` asks for.
    #[test]
    fn a_numeric_segment_indexes_an_array() {
        let event = Event::new(json!({
            "_temp": {"user_parts": ["VAGRANT", "vagrant"]},
            "rows": [{"name": "first"}, {"name": "second"}],
        }));

        assert_eq!(event.get_str("_temp.user_parts.0"), Some("VAGRANT"));
        assert_eq!(event.get_str("_temp.user_parts.1"), Some("vagrant"));
        assert_eq!(event.get("_temp.user_parts.2"), None);
        assert_eq!(event.get_str("rows.1.name"), Some("second"));
    }

    /// Reading through an index is not writing through one: `set` builds the
    /// path it is given and an array element is not a name it can create.
    #[test]
    fn writing_through_an_index_is_still_refused() {
        let mut event = Event::new(json!({"rows": [{"name": "first"}]}));
        assert!(event.set("rows.0.name", "renamed").is_err());
    }

    /// A non-numeric segment on an array is still nothing, rather than the
    /// first element or a panic.
    #[test]
    fn a_named_segment_on_an_array_finds_nothing() {
        let event = Event::new(json!({"tags": ["a", "b"]}));
        assert_eq!(event.get("tags.name"), None);
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

    // -- Flat-key tests --------------------------------------------------

    /// Verbatim from `testdata/compat/azure/activitylogs/supporttickets_write`.
    /// The SAML claim is keyed by URI, so the whole thing is ONE key with four
    /// dots in it and splitting on them reached nothing.
    #[test]
    fn a_key_holding_dots_resolves_whole() {
        let event = Event::new(json!({
            "azure": { "activitylogs": { "identity": { "claims": {
                "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname": "Smith",
                "ver": "1.0",
            } } } },
        }));

        assert_eq!(
            event.get_str(
                "azure.activitylogs.identity.claims.http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname"
            ),
            Some("Smith")
        );
        assert_eq!(
            event.get_str("azure.activitylogs.identity.claims.ver"),
            Some("1.0")
        );
    }

    /// Nested wins where a document spells the same path both ways, which is
    /// what Painless does -- the flat key is only consulted after the segment
    /// lookup has missed.
    #[test]
    fn a_nested_path_beats_a_flat_key_of_the_same_name() {
        let event = Event::new(json!({ "a": { "b": "nested" }, "a.b": "flat" }));
        assert_eq!(event.get_str("a.b"), Some("nested"));

        let only_flat = Event::new(json!({ "a.b": "flat" }));
        assert_eq!(only_flat.get_str("a.b"), Some("flat"));
    }

    /// `append` looks the path up with `get` and then unwraps the mutable
    /// walk, so the two must agree about a flat key or it panics.
    #[test]
    fn append_reaches_an_array_under_a_flat_key() {
        let mut event = Event::new(json!({ "a.b": { "list": ["one"] } }));
        event.append("a.b.list", "two").unwrap();

        assert_eq!(event.get("a.b.list"), Some(&json!(["one", "two"])));
    }

    /// A path that genuinely is not there still resolves to nothing, however
    /// many of its segments exist.
    #[test]
    fn a_missing_path_stays_missing() {
        let event = Event::new(json!({ "a": { "b": "value" }, "c.d": "flat" }));
        assert!(event.get("a.z").is_none());
        assert!(event.get("c.z").is_none());
        assert!(event.get("c.d.e").is_none());
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
            "f": 2.75,
            "b": true,
            "arr": [1, 2, 3],
            "obj": {"k": "v"}
        }));

        assert_eq!(event.get_str("s"), Some("hello"));
        assert_eq!(event.get_i64("i"), Some(42));
        assert_eq!(event.get_f64("f"), Some(2.75));
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

    /// Removing a key must not move any other key.
    ///
    /// `serde_json`'s own `Map::remove` is `swap_remove` under
    /// `preserve_order`, which drops the LAST key into the freed slot. This
    /// project runs `preserve_order` because Elasticsearch's maps are
    /// insertion-ordered and Painless renders them in that order, so a swap
    /// here reorders what a later `toString` writes.
    #[test]
    fn remove_keeps_the_order_of_what_it_leaves() {
        let mut event = Event::new(json!({
            "m": { "a": 1, "b": 2, "c": 3, "d": 4, "e": 5 }
        }));
        event.remove("m.b");
        let Some(Value::Object(map)) = event.get("m") else {
            panic!("m is not an object");
        };
        let order: Vec<&str> = map.keys().map(String::as_str).collect();
        assert_eq!(order, ["a", "c", "d", "e"]);
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

    /// An occupied target is a failure, not an overwrite -- Elasticsearch's own
    /// `field [y] already exists`. The source stays put so an `on_failure`
    /// handler still sees the document whole.
    #[test]
    fn rename_onto_an_occupied_target_fails_and_changes_nothing() {
        let mut event = Event::new(json!({"host": {"cpu": {"usage": 0.421}}, "raw": 42}));
        assert!(event.rename("raw", "host.cpu.usage").is_err());
        assert_eq!(event.get_f64("host.cpu.usage"), Some(0.421));
        assert_eq!(event.get_i64("raw"), Some(42));
    }

    /// `override: true` is the spelling that asks to replace it.
    #[test]
    fn rename_over_replaces_an_occupied_target() {
        let mut event = Event::new(json!({"host": {"cpu": {"usage": 0.421}}, "raw": 42}));
        event.rename_over("raw", "host.cpu.usage").unwrap();
        assert_eq!(event.get_i64("host.cpu.usage"), Some(42));
        assert!(!event.has("raw"));
    }

    /// A present-but-NULL target counts as occupied: Elasticsearch's
    /// `hasField` answers key presence, and a `set` with an absent
    /// `copy_from` writes exactly that null.
    #[test]
    fn rename_onto_an_explicit_null_still_fails() {
        let mut event = Event::new(json!({"dest": null, "raw": 42}));
        assert!(event.rename("raw", "dest").is_err());
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

    // --- Failure paths and boundary values ---

    #[test]
    fn get_nonexistent_deep_path_returns_none() {
        let event = Event::new(json!({"a": {"b": 1}}));
        assert!(event.get("a.b.c.d.e").is_none());
        assert!(event.get("x.y.z").is_none());
        assert!(event.get("").is_none());
    }

    #[test]
    fn get_str_on_non_string_returns_none() {
        let event = Event::new(json!({"num": 42, "bool": true, "arr": [1,2]}));
        assert!(event.get_str("num").is_none());
        assert!(event.get_str("bool").is_none());
        assert!(event.get_str("arr").is_none());
    }

    #[test]
    fn get_i64_on_non_number_returns_none() {
        let event = Event::new(json!({"s": "hello", "b": true}));
        assert!(event.get_i64("s").is_none());
        assert!(event.get_i64("nonexistent").is_none());
    }

    #[test]
    fn set_creates_deep_nested_path() {
        let mut event = Event::new(json!({}));
        event.set("a.b.c.d.e", json!("deep")).unwrap();
        assert_eq!(event.get_str("a.b.c.d.e"), Some("deep"));
    }

    #[test]
    fn set_rejects_object_path_through_primitive() {
        let mut event = Event::new(json!({"a": "string"}));
        // Setting a.b when a is a string returns TypeMismatch — it won't
        // silently overwrite a primitive with an object hierarchy.
        // This is correct: prevents accidental data loss.
        let result = event.set("a.b", json!("value"));
        assert!(result.is_err(), "should reject path through primitive");
    }

    #[test]
    fn from_json_invalid_returns_error() {
        let result = Event::from_json("not valid json {{{");
        assert!(result.is_err());
    }

    #[test]
    fn from_json_non_object_returns_error() {
        // JSON arrays and primitives are not valid events
        let result = Event::from_json("[1,2,3]");
        // Depends on implementation — may wrap or error
        // At minimum, should not panic
        let _ = result;
    }

    #[test]
    fn append_null_value_still_appends() {
        let mut event = Event::new(json!({"tags": ["a"]}));
        event.append("tags", Value::Null).unwrap();
        let arr = event.get_array("tags").unwrap();
        assert_eq!(arr.len(), 2); // null is a valid array element
    }

    #[test]
    fn has_returns_true_for_null_value() {
        let event = Event::new(json!({"a": null}));
        // has() is key existence, not truthiness.
        assert!(event.has("a"));
        assert!(event.get("a").is_some());
        assert!(!event.has("b"));
    }
}
