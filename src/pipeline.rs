// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Batch transform pipeline.
//!
//! One resolved transform is applied to every event in a batch. A transform
//! that drops an event removes it from the output; a transform that errors
//! leaves the event out and increments the error count rather than failing
//! the batch, so one malformed record cannot stall a partition.

use std::borrow::Cow;

use dfe_runtime::{Event, Transform, TransformError, TransformResult};

use crate::envelope::{Delivery, Resolution, Resolver};

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

/// What a batch's envelopes turned out to be.
///
/// Both flags are batch-wide rather than per event, which is the reading the
/// `envelope_contradicted` and `envelope_unaccepted` counters have always had:
/// one increment per batch that saw the condition at all.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EnvelopeOutcome {
    /// The first event's resolution, which is what the log line reports.
    pub first: Option<Resolution>,
    /// At least one event contradicted the pinned family.
    pub contradicted: bool,
    /// At least one event's detected family is not one this source accepts.
    pub unaccepted: bool,
}

/// How each event's envelope is decided.
enum Route<'a> {
    /// One delivery, decided by the caller and applied to every event. What an
    /// offline tool and the envelope tests take.
    Fixed(&'a Delivery),
    /// Decided per event, because one batch can span producers.
    PerEvent(&'a Resolver),
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
    transform_batch_with(transform, &Delivery::beats(), events)
}

/// Unwrap and stamp `delivery`, then apply `transform` to every event.
///
/// An event whose envelope will not unwrap is counted as errored, on the same
/// footing as one the transform rejects -- neither fails the batch.
pub fn transform_batch_with(
    transform: &dyn Transform,
    delivery: &Delivery,
    events: Vec<Event>,
) -> (Vec<Event>, BatchOutcome) {
    let (out, outcome, _) = run_batch(transform, &Route::Fixed(delivery), events);
    (out, outcome)
}

/// Detect each event's envelope, unwrap it, then apply `transform`.
///
/// The service path. A scalo `WorkBatch` spans partitions, so the family is
/// read per event rather than off the batch's first one.
pub fn transform_batch_resolved(
    transform: &dyn Transform,
    resolver: &Resolver,
    events: Vec<Event>,
) -> (Vec<Event>, BatchOutcome, EnvelopeOutcome) {
    run_batch(transform, &Route::PerEvent(resolver), events)
}

fn run_batch(
    transform: &dyn Transform,
    route: &Route<'_>,
    events: Vec<Event>,
) -> (Vec<Event>, BatchOutcome, EnvelopeOutcome) {
    let mut out = Vec::with_capacity(events.len());
    let mut outcome = BatchOutcome::default();
    let mut envelopes = EnvelopeOutcome::default();
    // One sample of each kind, reported once for the whole batch rather than
    // once per event -- see `report_failures`.
    let mut unwrap_failure: Option<String> = None;
    let mut transform_failure: Option<String> = None;

    for mut event in events {
        let unwrapped = match route {
            Route::Fixed(delivery) => delivery.apply(&mut event),
            Route::PerEvent(resolver) => {
                let resolution = resolver.resolve(&event);
                envelopes.contradicted |= resolution.contradicted;
                envelopes.unaccepted |= resolution.unaccepted;
                let applied = resolution.delivery.apply(&mut event);
                if envelopes.first.is_none() {
                    envelopes.first = Some(resolution);
                }
                applied
            }
        };
        if let Err(e) = unwrapped {
            outcome.errored += 1;
            if unwrap_failure.is_none() {
                unwrap_failure = Some(e.to_string());
            }
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
                if transform_failure.is_none() {
                    transform_failure = Some(for_the_log(&e).into_owned());
                }
            }
        }
    }

    if outcome.errored > 0 {
        report_failures(
            transform.name(),
            outcome.errored,
            unwrap_failure.as_deref(),
            transform_failure.as_deref(),
        );
    }

    (out, outcome, envelopes)
}

/// One line per batch for the events that did not make it.
///
/// A grok miss is normal on a device that mixes formats, so the per-event warn
/// this replaces put up to 20,000 lines in the log for one batch and drowned
/// everything else. The count is the signal; the samples make it actionable.
fn report_failures(
    transform: &str,
    errored: usize,
    unwrap_failure: Option<&str>,
    transform_failure: Option<&str>,
) {
    tracing::warn!(
        transform,
        errored,
        unwrap_sample = unwrap_failure.unwrap_or("-"),
        transform_sample = transform_failure.unwrap_or("-"),
        "events did not transform"
    );
}

/// Bytes of a grok's input that may reach a log line.
///
/// Enough to recognise which line it was, and short enough that 20,000 of them
/// could not export a batch of customer data into the service log.
const GROK_VALUE_LOG_MAX: usize = 120;

/// One transform error, rendered for an operator rather than for the document.
///
/// [`TransformError::GrokNoMatch`] carries the WHOLE field value, which on a
/// grok miss is the customer's raw log line. The document still gets the full
/// text -- the corpus compares Elasticsearch's wording verbatim -- and this is
/// the log-facing half.
fn for_the_log(error: &TransformError) -> Cow<'_, str> {
    match error {
        TransformError::GrokNoMatch { value } => Cow::Owned(format!(
            "Provided Grok expressions do not match field value: [{}]",
            elided(value)
        )),
        // The wrapper renders its source, so an unwrapped one would leak the
        // same value through a different variant.
        TransformError::ProcessorError { processor, source } => Cow::Owned(format!(
            "processor '{processor}' failed: {}",
            for_the_log(source)
        )),
        other => Cow::Owned(other.to_string()),
    }
}

/// The head of a value, with the full length named where it was cut.
fn elided(value: &str) -> Cow<'_, str> {
    let Some((head, _)) = value.char_indices().nth(GROK_VALUE_LOG_MAX) else {
        return Cow::Borrowed(value);
    };
    let kept = value.get(..head).unwrap_or_default();
    Cow::Owned(format!("{kept}... ({} bytes)", value.len()))
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

    // Sized from the payload's own newline count -- one SIMD pass against a
    // `Vec` that reallocated its way up to 20,000 events on every batch.
    let lines = memchr::memchr_iter(b'\n', payload).count();
    let mut events = Vec::with_capacity(lines.max(1));
    let mut first_bad: Option<String> = None;
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
                if first_bad.is_none() {
                    first_bad = Some(e.to_string());
                }
            }
        }
    }

    // One line per payload, not one per bad line: a payload of 20,000 lines
    // that a producer wrote in the wrong format is 20,000 warns otherwise.
    if let Some(sample) = first_bad {
        tracing::warn!(
            skipped = outcome.bad_lines,
            sample = %sample,
            "lines are not valid JSON, skipped"
        );
    }

    (events, outcome)
}

/// What serialising a batch produced.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SerialiseOutcome {
    /// Events that became an outbound message.
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
///
/// Public for the same reason as [`message_budget`]: the floor is part of what
/// that function promises, so a caller reading the promise can read the value.
pub const MIN_MESSAGE_BYTES: usize = 4096;

/// The per-message byte budget actually applied, floored at
/// [`MIN_MESSAGE_BYTES`].
///
/// Public because the service bounds an outbound BLOCK by the same number, and
/// a block judged against the raw value while the records in it were judged
/// against the floor would disagree about what fits.
#[must_use]
pub const fn message_budget(max_bytes: usize) -> usize {
    if max_bytes > MIN_MESSAGE_BYTES {
        max_bytes
    } else {
        MIN_MESSAGE_BYTES
    }
}

/// Serialise each event into a message payload of its own.
///
/// ONE JSON document per payload, never NDJSON. dfe-loader parses exactly one
/// document per message and has no NDJSON path, so a batch concatenated into a
/// single payload was dead-lettered whole instead of loaded
/// (hyperi-io/dfe-transform-elastic#67).
///
/// `max_bytes` bounds each payload on its own, because that is what a broker
/// bounds: librdkafka's producer `message.max.bytes` defaults to 1,000,000, so
/// an event above the budget can never be produced and is dropped rather than
/// blocking the partition behind a record that can only fail.
#[must_use]
pub fn serialise_events(events: &[Event], max_bytes: usize) -> (Vec<Vec<u8>>, SerialiseOutcome) {
    let budget = message_budget(max_bytes);
    let mut payloads = Vec::with_capacity(events.len());
    let mut outcome = SerialiseOutcome::default();
    // Seeded from the previous document: a batch is one source's events, so the
    // next one is close to the same size and the buffer does not regrow.
    let mut hint = 1024;

    for event in events {
        let mut payload: Vec<u8> = Vec::with_capacity(hint);
        if let Err(e) = serde_json::to_writer(&mut payload, event.as_value()) {
            outcome.failed += 1;
            tracing::warn!(error = %e, "event could not be serialised, dropped");
            continue;
        }
        if payload.len() > budget {
            outcome.oversize += 1;
            tracing::error!(
                bytes = payload.len(),
                budget,
                "event exceeds the per-message budget and cannot be produced, dropped"
            );
            continue;
        }

        // Seeded only from a payload that fits, so one oversize event does not
        // size the next allocation to a record the budget already refused.
        hint = payload.len().max(1024);
        outcome.serialised += 1;
        payloads.push(payload);
    }

    (payloads, outcome)
}

/// Serialise every event into ONE NDJSON payload, whatever its size.
///
/// For tests and offline tools, and what [`parse_batch`] reads back. The
/// service sends one record per event ([`serialise_events`]), because the
/// loader parses one document per message.
#[must_use]
pub fn serialise_batch(events: &[Event]) -> (Vec<u8>, SerialiseOutcome) {
    let (payloads, outcome) = serialise_events(events, usize::MAX);
    let mut ndjson = Vec::with_capacity(payloads.iter().map(|p| p.len() + 1).sum());
    for payload in payloads {
        ndjson.extend_from_slice(&payload);
        ndjson.push(b'\n');
    }
    (ndjson, outcome)
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

    /// The defect: dfe-loader parses ONE JSON document per message, so a batch
    /// concatenated into one NDJSON payload is dead-lettered whole. N events
    /// must produce N messages.
    #[test]
    fn every_event_gets_a_message_of_its_own() {
        let events = padded(200, 400);
        let (payloads, outcome) = serialise_events(&events, 8192);

        assert_eq!(payloads.len(), 200);
        assert_eq!(outcome.serialised, 200);
        for payload in &payloads {
            let parsed: serde_json::Value =
                serde_json::from_slice(payload).expect("one document per message");
            assert!(parsed.get("i").is_some(), "payload is not the event");
        }
    }

    /// A trailing newline would make a payload NDJSON of one line rather than a
    /// document, which is the framing the loader has no path for.
    #[test]
    fn a_message_carries_no_ndjson_framing() {
        let (payloads, _) = serialise_events(&padded(3, 40), 8192);
        for payload in &payloads {
            assert!(!payload.contains(&b'\n'), "payload carries a newline");
        }
    }

    /// At the shipped `batch_size` of 20,000, every message still has to be one
    /// librdkafka's producer default will accept.
    #[test]
    fn a_default_sized_batch_produces_messages_the_broker_accepts() {
        let events = padded(20_000, 400);
        let budget = crate::config::default_max_message_bytes();
        let (payloads, outcome) = serialise_events(&events, budget);

        assert_eq!(outcome.serialised, 20_000);
        assert_eq!(payloads.len(), 20_000);
        for payload in &payloads {
            assert!(
                payload.len() <= 1_000_000,
                "message of {} bytes exceeds librdkafka's message.max.bytes default",
                payload.len()
            );
        }
    }

    /// A record no broker can accept is dropped and counted, rather than
    /// retried forever behind the rest of the partition.
    #[test]
    fn an_event_larger_than_the_budget_is_dropped_and_counted() {
        let mut events = padded(2, 100);
        events.extend(padded(1, 20_000));

        let (payloads, outcome) = serialise_events(&events, MIN_MESSAGE_BYTES);
        assert_eq!(outcome.oversize, 1);
        assert_eq!(outcome.serialised, 2);
        assert_eq!(payloads.len(), 2);
    }

    /// A budget below one ordinary event would drop the entire batch as
    /// oversize, so it is clamped rather than obeyed.
    #[test]
    fn an_absurd_budget_is_clamped_not_obeyed() {
        let events = padded(4, 100);
        let (payloads, outcome) = serialise_events(&events, 1);
        assert_eq!(outcome.oversize, 0);
        assert_eq!(outcome.serialised, 4);
        assert_eq!(payloads.len(), 4);
    }

    /// The clamp raises the budget, and an event that sits BETWEEN the
    /// configured value and the floor is what tells the direction apart: the
    /// other way round it would be dropped as oversize rather than sent.
    #[test]
    fn the_clamp_raises_the_budget_rather_than_lowering_it() {
        let events = padded(1, 2_000);
        let (payloads, outcome) = serialise_events(&events, 512);

        assert_eq!(outcome.oversize, 0, "the event was judged against 512");
        assert_eq!(outcome.serialised, 1);
        assert!(
            payloads.first().is_some_and(|p| p.len() > 512),
            "a payload under 512 bytes means the budget was obeyed, not clamped"
        );
    }

    /// A budget ABOVE the floor is obeyed as given -- the clamp is a floor, not
    /// a replacement. An event between the two is what tells them apart.
    #[test]
    fn a_budget_above_the_floor_is_used_as_given() {
        let events = padded(1, 6_000);
        let (payloads, outcome) = serialise_events(&events, MIN_MESSAGE_BYTES * 2);

        assert_eq!(outcome.oversize, 0, "the floor was used instead of 8192");
        assert_eq!(payloads.len(), 1);
        assert_eq!(message_budget(MIN_MESSAGE_BYTES * 2), MIN_MESSAGE_BYTES * 2);
    }

    #[test]
    fn an_empty_batch_produces_no_messages() {
        let (payloads, outcome) = serialise_events(&[], 8192);
        assert!(payloads.is_empty());
        assert_eq!(outcome.serialised, 0);
    }
}
