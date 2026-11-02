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

/// Remove `event.original` unless the event is tagged [`PRESERVE_ORIGINAL_EVENT`].
///
/// Most generated transforms copy `message` to `event.original` and never
/// remove it, so every row would carry the vendor line twice. Elastic keeps the
/// copy, but the suite's default output is the extracted JSON only. A single
/// tags lookup and a single remove, with no allocation.
pub fn drop_original_unless_preserved(event: &mut Event) {
    let preserved = match event.get("tags") {
        Some(Value::Array(tags)) => tags
            .iter()
            .any(|tag| tag.as_str() == Some(PRESERVE_ORIGINAL_EVENT)),
        Some(Value::String(tag)) => tag == PRESERVE_ORIGINAL_EVENT,
        _ => false,
    };
    if !preserved {
        event.remove("event.original");
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
    fn original_is_dropped_unless_the_tag_keeps_it() {
        let cases = [
            ("untagged", json!({}), false),
            (
                "other tags",
                json!({"tags": ["forwarded", "cisco-ios"]}),
                false,
            ),
            (
                "tag in a list",
                json!({"tags": ["forwarded", PRESERVE_ORIGINAL_EVENT]}),
                true,
            ),
            (
                "tag as a scalar",
                json!({"tags": PRESERVE_ORIGINAL_EVENT}),
                true,
            ),
            ("tags of the wrong type", json!({"tags": 7}), false),
        ];
        for (name, extra, kept) in cases {
            let mut doc = extra;
            doc["event"] = json!({"original": "vendor line", "kind": "event"});
            let mut event = Event::new(doc);
            drop_original_unless_preserved(&mut event);
            assert_eq!(event.has("event.original"), kept, "{name}");
            assert_eq!(event.get_str("event.kind"), Some("event"), "{name}");
        }
    }

    #[test]
    fn dropping_an_absent_original_is_harmless() {
        let mut event = Event::new(json!({"message": "x"}));
        drop_original_unless_preserved(&mut event);
        assert_eq!(event.get_str("message"), Some("x"));
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
