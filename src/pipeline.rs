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
    transform_batch_with(transform, crate::envelope::Envelope::Beats, None, events)
}

/// Unwrap `envelope`, then apply `transform` to every event.
///
/// An event whose envelope will not unwrap is counted as errored, on the same
/// footing as one the transform rejects -- neither fails the batch.
pub fn transform_batch_with(
    transform: &dyn Transform,
    envelope: crate::envelope::Envelope,
    framing: Option<crate::registry::Framing>,
    events: Vec<Event>,
) -> (Vec<Event>, BatchOutcome) {
    let mut out = Vec::with_capacity(events.len());
    let mut outcome = BatchOutcome::default();

    for mut event in events {
        if let Err(e) = envelope.unwrap_into_beats(&mut event, framing) {
            outcome.errored += 1;
            tracing::warn!(error = %e, "envelope unwrap failed");
            continue;
        }
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

/// What a payload cost to decode.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ParseOutcome {
    /// Lines that produced an event.
    pub parsed: usize,
    /// Lines that were not valid JSON and were skipped.
    pub bad_lines: usize,
    /// The payload was not valid UTF-8 and was decoded with replacements.
    pub lossy: bool,
}

/// Parse one NDJSON payload into events.
///
/// Each line must be Beats-shaped: the raw vendor payload as a STRING in
/// `message`. A bare vendor object parses fine and transforms almost not at
/// all, because every processor after the first reads fields that only exist
/// once `message` has been unpacked.
///
/// Nothing about a payload is fatal. Bytes that are not valid UTF-8 are
/// replaced with U+FFFD, matching what Beats itself substitutes for a file it
/// cannot decode; a line that is not valid JSON is skipped. Both are counted,
/// so the damage is visible rather than silent.
///
/// Rejecting either would discard every event already parsed from the same
/// payload -- up to a full batch for one bad byte.
pub fn parse_batch(payload: &[u8]) -> (Vec<Event>, ParseOutcome) {
    // Borrowed when the payload is already valid UTF-8, which is the common
    // case, so the hot path still reads straight out of the Kafka buffer.
    let text = String::from_utf8_lossy(payload);
    let mut outcome = ParseOutcome {
        lossy: matches!(text, std::borrow::Cow::Owned(_)),
        ..ParseOutcome::default()
    };

    let mut events = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        match Event::from_json(line) {
            Ok(event) => {
                outcome.parsed += 1;
                events.push(event);
            }
            Err(e) => {
                outcome.bad_lines += 1;
                tracing::warn!(error = %e, "line is not valid JSON, skipped");
            }
        }
    }

    (events, outcome)
}

/// What serialising a batch produced.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SerialiseOutcome {
    /// Events written into a chunk.
    pub serialised: usize,
    /// Events that on their own exceed the whole per-message budget. No
    /// broker will ever accept one, so it is dropped rather than blocking the
    /// partition behind a record that can only fail.
    pub oversize: usize,
    /// Events serde refused to serialise. Dropped, on the same footing as an
    /// event the transform rejected.
    pub failed: usize,
}

/// The smallest per-message budget worth honouring.
///
/// Below this a single ordinary event is oversize and the batch is dropped
/// wholesale, so a misconfigured value is clamped up rather than obeyed.
const MIN_MESSAGE_BYTES: usize = 4096;

/// Serialise events to NDJSON, split into messages of at most `max_bytes`.
///
/// A 20,000-event batch is 2 MB of 100-byte events and tens of MB of real ECS
/// ones, while librdkafka's producer `message.max.bytes` defaults to 1,000,000
/// -- so the outbound size is bounded by BYTES here, never by the event count
/// the batch happened to arrive with.
///
/// Every chunk is a complete NDJSON payload: no event is split across two.
#[must_use]
pub fn serialise_chunks(events: &[Event], max_bytes: usize) -> (Vec<Vec<u8>>, SerialiseOutcome) {
    let budget = max_bytes.max(MIN_MESSAGE_BYTES);
    // One allocation per chunk, sized to whichever is smaller: the budget, or
    // what the whole batch is likely to need.
    let reserve = budget.min(events.len().saturating_mul(512).max(MIN_MESSAGE_BYTES));

    let mut chunks = Vec::new();
    let mut current: Vec<u8> = Vec::with_capacity(reserve);
    let mut line: Vec<u8> = Vec::with_capacity(1024);
    let mut outcome = SerialiseOutcome::default();

    for event in events {
        line.clear();
        if let Err(e) = serde_json::to_writer(&mut line, event.as_value()) {
            outcome.failed += 1;
            tracing::warn!(error = %e, "event could not be serialised, dropped");
            continue;
        }
        line.push(b'\n');

        if line.len() > budget {
            outcome.oversize += 1;
            tracing::error!(
                bytes = line.len(),
                budget,
                "event exceeds the per-message budget and cannot be produced, dropped"
            );
            continue;
        }

        if !current.is_empty() && current.len() + line.len() > budget {
            chunks.push(std::mem::replace(&mut current, Vec::with_capacity(reserve)));
        }
        current.extend_from_slice(&line);
        outcome.serialised += 1;
    }

    if !current.is_empty() {
        chunks.push(current);
    }
    (chunks, outcome)
}

/// Serialise every event into ONE NDJSON payload, whatever its size.
///
/// For tests and offline tools. The service uses [`serialise_chunks`], because
/// a default-sized batch does not fit in one Kafka record.
#[must_use]
pub fn serialise_batch(events: &[Event]) -> (Vec<u8>, SerialiseOutcome) {
    let (mut chunks, outcome) = serialise_chunks(events, usize::MAX);
    (chunks.pop().unwrap_or_default(), outcome)
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
        parse_batch(b"{\"a\":1}\n{\"a\":2}\n{\"a\":3}\n").0
    }

    #[test]
    fn parses_ndjson_and_skips_blank_lines() {
        let (events, outcome) = parse_batch(b"{\"a\":1}\n\n{\"a\":2}\n");
        assert_eq!(events.len(), 2);
        assert_eq!(outcome.parsed, 2);
        assert_eq!(outcome.bad_lines, 0);
        assert!(!outcome.lossy);
    }

    #[test]
    fn counts_malformed_json_without_failing() {
        let (events, outcome) = parse_batch(b"{not json}\n");
        assert!(events.is_empty());
        assert_eq!(outcome.bad_lines, 1);
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
        let (bytes, outcome) = serialise_batch(&out);
        assert_eq!(outcome.serialised, 3);
        assert_eq!(parse_batch(&bytes).0.len(), 3);
    }

    /// Events roughly the size of a real ECS document, so the split is
    /// exercised at a realistic ratio rather than at one event per chunk.
    fn padded(count: usize, bytes: usize) -> Vec<Event> {
        use std::fmt::Write as _;

        let filler = "x".repeat(bytes);
        let mut lines = String::with_capacity(count * (bytes + 32));
        for i in 0..count {
            let _ = writeln!(lines, "{{\"i\":{i},\"pad\":\"{filler}\"}}");
        }
        parse_batch(lines.as_bytes()).0
    }

    #[test]
    fn every_chunk_stays_within_the_budget() {
        let events = padded(200, 400);
        let (chunks, outcome) = serialise_chunks(&events, 8192);

        assert!(chunks.len() > 1, "200 events of 400 bytes must split");
        assert_eq!(outcome.serialised, 200);
        for chunk in &chunks {
            assert!(chunk.len() <= 8192, "chunk of {} bytes", chunk.len());
        }
    }

    #[test]
    fn splitting_loses_no_events() {
        let events = padded(200, 400);
        let (chunks, _) = serialise_chunks(&events, 8192);

        let reassembled: usize = chunks.iter().map(|c| parse_batch(c).0.len()).sum();
        assert_eq!(reassembled, 200);
    }

    /// The defect this split exists to fix: at the shipped `batch_size` of
    /// 20,000, one concatenated payload is tens of MB against librdkafka's
    /// 1,000,000-byte producer default, so every batch fails to produce.
    #[test]
    fn a_default_sized_batch_produces_messages_the_broker_accepts() {
        let events = padded(20_000, 400);
        let budget = crate::config::default_max_message_bytes();
        let (chunks, outcome) = serialise_chunks(&events, budget);

        assert_eq!(outcome.serialised, 20_000);
        for chunk in &chunks {
            assert!(
                chunk.len() <= 1_000_000,
                "chunk of {} bytes exceeds librdkafka's message.max.bytes default",
                chunk.len()
            );
        }
        let reassembled: usize = chunks.iter().map(|c| parse_batch(c).0.len()).sum();
        assert_eq!(reassembled, 20_000);
    }

    /// A record no broker can accept is dropped and counted, rather than
    /// retried forever behind the rest of the partition.
    #[test]
    fn an_event_larger_than_the_budget_is_dropped_and_counted() {
        let mut events = padded(2, 100);
        events.extend(padded(1, 20_000));

        let (chunks, outcome) = serialise_chunks(&events, MIN_MESSAGE_BYTES);
        assert_eq!(outcome.oversize, 1);
        assert_eq!(outcome.serialised, 2);
        let reassembled: usize = chunks.iter().map(|c| parse_batch(c).0.len()).sum();
        assert_eq!(reassembled, 2);
    }

    /// A budget below one ordinary event would drop the entire batch as
    /// oversize, so it is clamped rather than obeyed.
    #[test]
    fn an_absurd_budget_is_clamped_not_obeyed() {
        let events = padded(4, 100);
        let (chunks, outcome) = serialise_chunks(&events, 1);
        assert_eq!(outcome.oversize, 0);
        assert_eq!(outcome.serialised, 4);
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn an_empty_batch_produces_no_messages() {
        let (chunks, outcome) = serialise_chunks(&[], 8192);
        assert!(chunks.is_empty());
        assert_eq!(outcome.serialised, 0);
    }
}
