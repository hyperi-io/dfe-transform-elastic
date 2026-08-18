// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Batch transform pipeline.
//!
//! One resolved transform is applied to every event in a batch. A transform
//! that drops an event removes it from the output; a transform that errors
//! leaves the event out and increments the error count rather than failing
//! the batch, so one malformed record cannot stall a partition.

use dfe_runtime::{Event, Transform, TransformResult};

/// What a batch produced.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BatchOutcome {
    /// Events that passed through and are in the output.
    pub emitted: usize,
    /// Events the transform asked to drop.
    pub dropped: usize,
    /// Events whose transform returned an error.
    pub errored: usize,
}

impl BatchOutcome {
    /// Events seen, whatever became of them.
    pub const fn total(&self) -> usize {
        self.emitted + self.dropped + self.errored
    }
}

/// Apply `transform` to every event, returning the survivors.
///
/// The output vector is pre-sized to the input length: the common case is
/// that nothing is dropped, and growing on the hot path is the thing this
/// service exists to avoid.
pub fn transform_batch(
    transform: &dyn Transform,
    events: Vec<Event>,
) -> (Vec<Event>, BatchOutcome) {
    let mut out = Vec::with_capacity(events.len());
    let mut outcome = BatchOutcome::default();

    for mut event in events {
        match transform.transform(&mut event) {
            Ok(TransformResult::Continue) => {
                outcome.emitted += 1;
                out.push(event);
            }
            Ok(TransformResult::Drop) => outcome.dropped += 1,
            Err(e) => {
                outcome.errored += 1;
                tracing::warn!(
                    transform = transform.name(),
                    error = %e,
                    "event transform failed"
                );
            }
        }
    }

    (out, outcome)
}

/// Parse one NDJSON payload into events.
///
/// # Errors
///
/// Returns [`crate::Error::Parse`] if a line is not valid JSON.
pub fn parse_batch(payload: &[u8]) -> crate::Result<Vec<Event>> {
    let text = std::str::from_utf8(payload).map_err(|e| crate::Error::Parse(e.to_string()))?;

    let mut events = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        events.push(Event::from_json(line).map_err(|e| crate::Error::Parse(e.to_string()))?);
    }
    Ok(events)
}

/// Serialise events back to an NDJSON payload.
///
/// # Errors
///
/// Returns [`crate::Error::Parse`] if an event cannot be serialised.
pub fn serialise_batch(events: &[Event]) -> crate::Result<Vec<u8>> {
    let mut out = Vec::with_capacity(events.len() * 512);
    for event in events {
        serde_json::to_writer(&mut out, event.as_value())
            .map_err(|e| crate::Error::Parse(e.to_string()))?;
        out.push(b'\n');
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unnecessary_literal_bound)]
mod tests {
    use super::*;
    use dfe_runtime::Result as TransformOutcome;

    struct Passthrough;
    impl Transform for Passthrough {
        fn name(&self) -> &str {
            "passthrough"
        }
        fn transform(&self, _e: &mut Event) -> TransformOutcome<TransformResult> {
            Ok(TransformResult::Continue)
        }
    }

    struct DropAll;
    impl Transform for DropAll {
        fn name(&self) -> &str {
            "drop_all"
        }
        fn transform(&self, _e: &mut Event) -> TransformOutcome<TransformResult> {
            Ok(TransformResult::Drop)
        }
    }

    struct AlwaysErrors;
    impl Transform for AlwaysErrors {
        fn name(&self) -> &str {
            "always_errors"
        }
        fn transform(&self, _e: &mut Event) -> TransformOutcome<TransformResult> {
            Err(dfe_runtime::TransformError::FieldNotFound {
                path: "nope".into(),
            })
        }
    }

    fn three() -> Vec<Event> {
        parse_batch(b"{\"a\":1}\n{\"a\":2}\n{\"a\":3}\n").expect("fixture parses")
    }

    #[test]
    fn parses_ndjson_and_skips_blank_lines() {
        let events = parse_batch(b"{\"a\":1}\n\n{\"a\":2}\n").expect("parses");
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn rejects_malformed_json() {
        assert!(parse_batch(b"{not json}\n").is_err());
    }

    #[test]
    fn passthrough_emits_everything() {
        let (out, outcome) = transform_batch(&Passthrough, three());
        assert_eq!(out.len(), 3);
        assert_eq!(outcome.emitted, 3);
        assert_eq!(outcome.total(), 3);
    }

    #[test]
    fn drop_removes_from_output_without_erroring() {
        let (out, outcome) = transform_batch(&DropAll, three());
        assert!(out.is_empty());
        assert_eq!(outcome.dropped, 3);
        assert_eq!(outcome.errored, 0);
    }

    #[test]
    fn one_bad_event_does_not_fail_the_batch() {
        let (out, outcome) = transform_batch(&AlwaysErrors, three());
        assert!(out.is_empty());
        assert_eq!(outcome.errored, 3);
    }

    #[test]
    fn round_trips_through_serialise() {
        let (out, _) = transform_batch(&Passthrough, three());
        let bytes = serialise_batch(&out).expect("serialises");
        assert_eq!(parse_batch(&bytes).expect("reparses").len(), 3);
    }
}
