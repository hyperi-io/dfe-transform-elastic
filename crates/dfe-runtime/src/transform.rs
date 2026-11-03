// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Transform trait, `TransformResult`, and `TransformChain`.
//!
//! Every transform module implements the `Transform` trait. Multiple
//! transforms compose into a `TransformChain` that runs sequentially,
//! stopping on `Drop` or error.

use serde_json::Value;

use crate::error::Result;
use crate::event::Event;

/// Outcome of a transform: continue processing or drop the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformResult {
    /// Pass the event to the next transform in the chain.
    Continue,
    /// Drop the event entirely (e.g., `drop` processor).
    Drop,
}

/// A single transform step applied to an event.
///
/// Object-safe for use as `Box<dyn Transform>` in heterogeneous chains.
pub trait Transform: Send + Sync {
    /// Human-readable name for error context (e.g., "set", "grok").
    fn name(&self) -> &str;

    /// Apply this transform to an event.
    fn transform(&self, event: &mut Event) -> Result<TransformResult>;
}

/// The `tags` value that keeps `event.original` on an event.
///
/// Elastic's own opt-in: a user sets it in the integration, and the pipelines
/// add it themselves when they fail.
pub const PRESERVE_ORIGINAL_EVENT: &str = "preserve_original_event";

/// What [`settle_original`] did with an event's `event.original`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Original {
    /// Left as it was: the event is tagged [`PRESERVE_ORIGINAL_EVENT`], or
    /// there was no `event.original` to settle.
    Kept,
    /// Removed, because `message` still holds the line.
    Removed,
    /// Moved back to `message`, because the transform had taken `message` out
    /// and `event.original` held the only copy.
    Restored,
}

/// A field a transform writes whatever the line said, or the shipper wrote
/// before the transform ran.
enum Envelope {
    /// This field, and everything under it.
    Field,
    /// Only the named fields under this one.
    Within(&'static [(&'static str, Envelope)]),
}

/// The fields that carry no parsed content, read down the tree.
///
/// An event holding nothing else is a shell: a transform ran, set its static
/// fields and parsed nothing from the line. The entries are the fields the
/// shells of `tests/original.rs` carry, plus the wrapper an Elastic Agent or
/// the receiver puts around a line. `host.name` is the shipper's agent name
/// and `event.created` and the `*_time` entries are timestamp defaults. The
/// rest of `host` and `log.syslog` are left out because transforms write parsed
/// values there. Any key starting with `_` is internal as well.
const ENVELOPE: &[(&str, Envelope)] = &[
    ("@timestamp", Envelope::Field),
    ("agent", Envelope::Field),
    ("data_stream", Envelope::Field),
    ("ecs", Envelope::Field),
    ("elastic_agent", Envelope::Field),
    ("input", Envelope::Field),
    ("tags", Envelope::Field),
    (
        "event",
        Envelope::Within(&[
            ("action", Envelope::Field),
            ("agent_id_status", Envelope::Field),
            ("category", Envelope::Field),
            ("created", Envelope::Field),
            ("dataset", Envelope::Field),
            ("ingested", Envelope::Field),
            ("kind", Envelope::Field),
            ("module", Envelope::Field),
            ("original", Envelope::Field),
            ("outcome", Envelope::Field),
            ("provider", Envelope::Field),
            ("reference", Envelope::Field),
            ("type", Envelope::Field),
        ]),
    ),
    (
        "cloud",
        Envelope::Within(&[
            ("provider", Envelope::Field),
            ("service", Envelope::Within(&[("name", Envelope::Field)])),
        ]),
    ),
    ("log", Envelope::Within(&[("source", Envelope::Field)])),
    ("host", Envelope::Within(&[("name", Envelope::Field)])),
    (
        "email",
        Envelope::Within(&[("delivery_timestamp", Envelope::Field)]),
    ),
    (
        "nagios_xi",
        Envelope::Within(&[
            (
                "event",
                Envelope::Within(&[("entry_time", Envelope::Field)]),
            ),
            (
                "host",
                Envelope::Within(&[("status_update_time", Envelope::Field)]),
            ),
            (
                "service",
                Envelope::Within(&[("status_update_time", Envelope::Field)]),
            ),
        ]),
    ),
    (
        "observer",
        Envelope::Within(&[
            ("product", Envelope::Field),
            ("type", Envelope::Field),
            ("vendor", Envelope::Field),
        ]),
    ),
    (
        "network",
        Envelope::Within(&[("protocol", Envelope::Field)]),
    ),
    (
        "result",
        Envelope::Within(&[("evaluation", Envelope::Field)]),
    ),
    (
        "threat",
        Envelope::Within(&[
            ("feed", Envelope::Within(&[("name", Envelope::Field)])),
            (
                "indicator",
                Envelope::Within(&[(
                    "marking",
                    Envelope::Within(&[("tlp_version", Envelope::Field)]),
                )]),
            ),
        ]),
    ),
    (
        "vulnerability",
        Envelope::Within(&[
            ("classification", Envelope::Field),
            ("enumeration", Envelope::Field),
            ("scanner", Envelope::Within(&[("vendor", Envelope::Field)])),
        ]),
    ),
    (
        "aws",
        Envelope::Within(&[(
            "bedrock_agentcore",
            Envelope::Within(&[
                ("operation_name", Envelope::Field),
                ("provider_name", Envelope::Field),
                (
                    "gateway",
                    Envelope::Within(&[
                        ("operation_name", Envelope::Field),
                        ("provider_name", Envelope::Field),
                    ]),
                ),
                (
                    "memory",
                    Envelope::Within(&[
                        ("operation_name", Envelope::Field),
                        ("provider_name", Envelope::Field),
                    ]),
                ),
            ]),
        )]),
    ),
    (
        "carbon_black_cloud",
        Envelope::Within(&[("alert", Envelope::Within(&[("category", Envelope::Field)]))]),
    ),
    (
        "tenable_sc",
        Envelope::Within(&[
            (
                "plugin",
                Envelope::Within(&[
                    ("is_patch_modified", Envelope::Field),
                    ("is_patch_published", Envelope::Field),
                    ("is_plugin_modified", Envelope::Field),
                    ("is_plugin_published", Envelope::Field),
                    ("is_vulnerability_published", Envelope::Field),
                ]),
            ),
            (
                "vulnerability",
                Envelope::Within(&[
                    ("is_vulnerability_published", Envelope::Field),
                    (
                        "patch",
                        Envelope::Within(&[("is_published", Envelope::Field)]),
                    ),
                    (
                        "plugin",
                        Envelope::Within(&[
                            ("is_modified", Envelope::Field),
                            ("is_published", Envelope::Field),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
];

/// A value that says nothing: null, an empty string, an empty list or object.
fn is_blank(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(text) => text.is_empty(),
        Value::Array(items) => items.is_empty(),
        Value::Object(fields) => fields.values().all(is_blank),
        _ => false,
    }
}

/// Whether the object `fields` holds anything the `names` list does not cover.
fn holds_content(fields: &serde_json::Map<String, Value>, names: &[(&str, Envelope)]) -> bool {
    fields.iter().any(|(key, child)| {
        if is_blank(child) {
            return false;
        }
        match names.iter().find(|(name, _)| name == key) {
            None => true,
            Some((_, Envelope::Field)) => false,
            Some((_, Envelope::Within(inner))) => match child {
                Value::Object(fields) => holds_content(fields, inner),
                _ => true,
            },
        }
    })
}

/// Whether the event holds a field outside [`ENVELOPE`], which means the
/// transform parsed something out of the line, or left the line in `message`.
///
/// One pass over the top-level keys that stops at the first such field.
/// Allocates nothing.
fn is_parsed(event: &Event) -> bool {
    let Value::Object(fields) = event.as_value() else {
        return true;
    };
    fields.iter().any(|(key, child)| {
        if key.starts_with('_') || is_blank(child) {
            return false;
        }
        match ENVELOPE.iter().find(|(name, _)| name == key) {
            None => true,
            Some((_, Envelope::Field)) => false,
            Some((_, Envelope::Within(inner))) => match child {
                Value::Object(fields) => holds_content(fields, inner),
                _ => true,
            },
        }
    })
}

/// Leave the event with one copy of the vendor line, never two and never none.
///
/// Most generated transforms copy `message` to `event.original` and keep both,
/// or move `message` there. Elastic keeps the copy, but the suite's default
/// output is the extracted JSON alone.
///
/// - Tagged [`PRESERVE_ORIGINAL_EVENT`]: `event.original` stays as it is.
/// - Otherwise, with any field outside the envelope set, `message` included:
///   the line was parsed or is still in `message`, so `event.original` is a
///   second copy and is removed.
/// - Otherwise the event is a shell and `event.original` is the only copy. It
///   moves back to `message`.
///
/// One tags lookup, one pass over the top-level keys that stops at the first
/// field of content, and one remove or rename. Allocates nothing.
pub fn settle_original(event: &mut Event) -> Original {
    let preserved = match event.get("tags") {
        Some(Value::Array(tags)) => tags
            .iter()
            .any(|tag| tag.as_str() == Some(PRESERVE_ORIGINAL_EVENT)),
        Some(Value::String(tag)) => tag == PRESERVE_ORIGINAL_EVENT,
        _ => false,
    };
    if preserved {
        return Original::Kept;
    }
    if is_parsed(event) {
        event.remove("event.original");
        return Original::Removed;
    }
    match event.rename_over("event.original", "message") {
        Ok(()) => Original::Restored,
        Err(_) => Original::Kept,
    }
}

/// Sequential composition of multiple transforms.
///
/// Executes transforms in order. Stops early on `TransformResult::Drop`
/// or on the first error.
pub struct TransformChain {
    transforms: Vec<Box<dyn Transform>>,
}

impl TransformChain {
    /// Create a chain from a list of transforms.
    pub fn new(transforms: Vec<Box<dyn Transform>>) -> Self {
        Self { transforms }
    }

    /// Run all transforms in sequence against the event.
    pub fn execute(&self, event: &mut Event) -> Result<TransformResult> {
        for t in &self.transforms {
            match t.transform(event)? {
                TransformResult::Continue => {}
                TransformResult::Drop => return Ok(TransformResult::Drop),
            }
        }
        Ok(TransformResult::Continue)
    }

    /// Number of transforms in the chain.
    pub fn len(&self) -> usize {
        self.transforms.len()
    }

    /// Whether the chain has no transforms.
    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::error::TransformError;
    use serde_json::json;

    struct SetField {
        path: &'static str,
        value: &'static str,
    }

    impl Transform for SetField {
        fn name(&self) -> &'static str {
            "set"
        }

        fn transform(&self, event: &mut Event) -> Result<TransformResult> {
            event.set(self.path, self.value)?;
            Ok(TransformResult::Continue)
        }
    }

    struct DropAll;

    impl Transform for DropAll {
        fn name(&self) -> &'static str {
            "drop"
        }

        fn transform(&self, _event: &mut Event) -> Result<TransformResult> {
            Ok(TransformResult::Drop)
        }
    }

    struct FailAlways;

    impl Transform for FailAlways {
        fn name(&self) -> &'static str {
            "fail"
        }

        fn transform(&self, _event: &mut Event) -> Result<TransformResult> {
            Err(TransformError::FieldNotFound {
                path: "missing".into(),
            })
        }
    }

    #[test]
    fn single_transform() {
        let mut event = Event::new(json!({}));
        let t = SetField {
            path: "event.kind",
            value: "event",
        };
        let result = t.transform(&mut event).unwrap();
        assert_eq!(result, TransformResult::Continue);
        assert_eq!(event.get_str("event.kind"), Some("event"));
    }

    #[test]
    fn chain_of_three() {
        let chain = TransformChain::new(vec![
            Box::new(SetField {
                path: "a",
                value: "1",
            }),
            Box::new(SetField {
                path: "b",
                value: "2",
            }),
            Box::new(SetField {
                path: "c",
                value: "3",
            }),
        ]);

        let mut event = Event::new(json!({}));
        let result = chain.execute(&mut event).unwrap();
        assert_eq!(result, TransformResult::Continue);
        assert_eq!(event.get_str("a"), Some("1"));
        assert_eq!(event.get_str("b"), Some("2"));
        assert_eq!(event.get_str("c"), Some("3"));
    }

    #[test]
    fn chain_drop_stops_early() {
        let chain = TransformChain::new(vec![
            Box::new(SetField {
                path: "a",
                value: "1",
            }),
            Box::new(DropAll),
            Box::new(SetField {
                path: "b",
                value: "2",
            }),
        ]);

        let mut event = Event::new(json!({}));
        let result = chain.execute(&mut event).unwrap();
        assert_eq!(result, TransformResult::Drop);
        assert_eq!(event.get_str("a"), Some("1"));
        assert!(!event.has("b"));
    }

    #[test]
    fn chain_error_stops_early() {
        let chain = TransformChain::new(vec![
            Box::new(SetField {
                path: "a",
                value: "1",
            }),
            Box::new(FailAlways),
            Box::new(SetField {
                path: "b",
                value: "2",
            }),
        ]);

        let mut event = Event::new(json!({}));
        let result = chain.execute(&mut event);
        assert!(result.is_err());
        assert_eq!(event.get_str("a"), Some("1"));
        assert!(!event.has("b"));
    }

    #[test]
    fn one_copy_of_the_line_never_two_and_never_none() {
        let line = "vendor line";
        let cases = [
            // (name, document, outcome, message after, event.original after)
            (
                "message present, untagged",
                json!({"message": "parsed rest", "event": {"original": line}}),
                Original::Removed,
                Some("parsed rest"),
                None,
            ),
            (
                "message gone, untagged",
                json!({"event": {"original": line, "kind": "event"}}),
                Original::Restored,
                Some(line),
                None,
            ),
            (
                "message empty, untagged",
                json!({"message": "", "event": {"original": line}}),
                Original::Restored,
                Some(line),
                None,
            ),
            (
                "message null, untagged",
                json!({"message": null, "event": {"original": line}}),
                Original::Restored,
                Some(line),
                None,
            ),
            (
                "tagged in a list, message present",
                json!({"tags": ["forwarded", PRESERVE_ORIGINAL_EVENT], "message": "m", "event": {"original": line}}),
                Original::Kept,
                Some("m"),
                Some(line),
            ),
            (
                "tagged as a scalar, message gone",
                json!({"tags": PRESERVE_ORIGINAL_EVENT, "event": {"original": line}}),
                Original::Kept,
                None,
                Some(line),
            ),
            (
                "other tags, message present",
                json!({"tags": ["forwarded"], "message": "m", "event": {"original": line}}),
                Original::Removed,
                Some("m"),
                None,
            ),
            (
                "tags of the wrong type, message gone",
                json!({"tags": 7, "event": {"original": line}}),
                Original::Restored,
                Some(line),
                None,
            ),
            (
                "parsed fields, message gone",
                json!({"source": {"ip": "192.0.2.1"}, "event": {"original": line}}),
                Original::Removed,
                None,
                None,
            ),
            (
                "content beside envelope fields under event",
                json!({"event": {"original": line, "kind": "event", "code": "5"}}),
                Original::Removed,
                None,
                None,
            ),
            (
                "parsed syslog header only",
                json!({"log": {"syslog": {"priority": 189}}, "event": {"original": line}}),
                Original::Removed,
                None,
                None,
            ),
            (
                "parsed host only",
                json!({"host": {"hostname": "fw1"}, "event": {"original": line}}),
                Original::Removed,
                None,
                None,
            ),
            (
                "shell inside the shipper wrapper",
                json!({
                    "@timestamp": "2026-02-19T00:00:00.000Z",
                    "agent": {"type": "filebeat"},
                    "data_stream": {"dataset": "x.log"},
                    "input": {"type": "tcp"},
                    "log": {"source": {"address": "192.0.2.10:46792"}},
                    "tags": ["forwarded"],
                    "_source": "x",
                    "ecs": {"version": "8.17.0"},
                    "observer": {"vendor": "Acme", "product": ""},
                    "event": {"original": line, "kind": "event", "agent_id_status": "verified"},
                }),
                Original::Restored,
                Some(line),
                None,
            ),
            (
                "nothing to settle",
                json!({"event": {"kind": "event"}}),
                Original::Kept,
                None,
                None,
            ),
        ];
        for (name, doc, outcome, message, original) in cases {
            let mut event = Event::new(doc);
            assert_eq!(settle_original(&mut event), outcome, "{name}");
            assert_eq!(event.get_str("message"), message, "{name}: message");
            assert_eq!(
                event.get_str("event.original"),
                original,
                "{name}: original"
            );
        }
    }

    #[test]
    fn empty_chain() {
        let chain = TransformChain::new(vec![]);
        assert!(chain.is_empty());
        let mut event = Event::new(json!({}));
        let result = chain.execute(&mut event).unwrap();
        assert_eq!(result, TransformResult::Continue);
    }
}
