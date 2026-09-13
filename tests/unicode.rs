// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Unicode resilience of the batch pipeline.
//!
//! This service eats arbitrary log data -- Windows event logs, syslog, cloud
//! audit trails -- so non-English text and outright invalid bytes are the
//! normal case, not the edge case. A panic here is a pod restart and a stalled
//! partition, and silent mangling is worse: it corrupts the normalised output
//! everything downstream trusts.
//!
//! The rules under test:
//!
//! 1. Nothing panics, whatever the bytes.
//! 2. Valid non-ASCII text survives a round trip byte-for-byte.
//! 3. Invalid UTF-8 costs a replacement character, never a record.
//! 4. One bad line costs that line, never the rest of the payload.

// A test asserts by panicking; the workspace lints ban that in library code.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use dfe_transform_elastic::pipeline::{parse_batch, serialise_batch, transform_batch};
use dfe_transform_elastic::registry;

/// Scripts, combining marks, emoji, RTL, and the codepoints that break naive
/// case-folding and byte slicing.
const SAMPLES: &[(&str, &str)] = &[
    ("latin-1 supplement", "Ä Ö Ü ß é à ç"),
    ("cjk", "日本語のログメッセージ"),
    ("korean", "한국어 로그"),
    ("cyrillic", "Сообщение журнала"),
    ("greek", "Μήνυμα καταγραφής"),
    ("arabic rtl", "رسالة السجل"),
    ("hebrew rtl", "הודעת יומן"),
    ("thai no-spaces", "ข้อความบันทึก"),
    ("devanagari", "लॉग संदेश"),
    ("emoji with zwj", "alert \u{1F468}\u{200D}\u{1F4BB} fired"),
    ("combining marks", "e\u{0301}e\u{0301}e\u{0301} accented"),
    (
        "turkish dotted i",
        "\u{0130}stanbul \u{0131}\u{015F}\u{0131}k",
    ),
    ("german sharp s", "STRASSE stra\u{00DF}e"),
    ("astral plane", "\u{1D400}\u{1D401}\u{1D402} maths"),
    ("bom in the middle", "before\u{FEFF}after"),
    ("zero width", "a\u{200B}b\u{200C}c\u{200D}d"),
    ("nbsp", "a\u{00A0}b"),
];

fn ndjson(field: &str, value: &str) -> Vec<u8> {
    let line = serde_json::json!({ field: value });
    let mut out = serde_json::to_vec(&line).expect("sample serialises");
    out.push(b'\n');
    out
}

#[test]
fn every_sample_survives_a_round_trip_unchanged() {
    for (label, sample) in SAMPLES {
        let payload = ndjson("message", sample);

        let (events, outcome) = parse_batch(&payload);
        assert_eq!(events.len(), 1, "{label}: expected one event");
        assert!(
            !outcome.lossy,
            "{label}: valid UTF-8 must not decode lossily"
        );
        assert_eq!(outcome.bad_lines, 0, "{label}: valid JSON must parse");

        let (round_tripped, serialised) = serialise_batch(&events);
        assert_eq!(serialised.serialised, 1, "{label}: serialise dropped it");
        let (reparsed, _) = parse_batch(&round_tripped);

        let value = reparsed[0]
            .get_str("message")
            .unwrap_or_else(|| panic!("{label}: message field missing after round trip"));
        assert_eq!(value, *sample, "{label}: text changed in transit");
    }
}

/// Field NAMES arrive from upstream too -- a Windows event log can key on a
/// localised field name, and the path splitter must not choke on it.
#[test]
fn non_ascii_field_names_survive_a_round_trip() {
    for (label, sample) in SAMPLES {
        // Dots are path separators in this event model, so a sample containing
        // one would legitimately nest. None of these do.
        assert!(!sample.contains('.'), "{label}: sample would nest");

        let payload = ndjson(sample, "value");
        let (events, _) = parse_batch(&payload);
        let (reparsed, _) = parse_batch(&serialise_batch(&events).0);

        assert_eq!(
            reparsed[0].get_str(sample),
            Some("value"),
            "{label}: field name changed in transit"
        );
    }
}

/// A lone 0x80..0xFF byte is what a latin-1 log line looks like once it
/// reaches Kafka. It must cost a replacement character, not the record.
#[test]
fn invalid_utf8_costs_a_replacement_not_the_record() {
    // {"message":"caf<0xE9>"} -- latin-1 e-acute, invalid as UTF-8.
    let mut payload = b"{\"message\":\"caf".to_vec();
    payload.push(0xE9);
    payload.extend_from_slice(b"\"}\n");

    let (events, outcome) = parse_batch(&payload);
    assert_eq!(events.len(), 1, "the record must survive one bad byte");
    assert!(
        outcome.lossy,
        "the substitution must be visible in the count"
    );

    let message = events[0].get_str("message").expect("message survives");
    assert!(
        message.starts_with("caf"),
        "the decodable prefix must survive: {message:?}"
    );
    assert!(
        message.contains('\u{FFFD}'),
        "the undecodable byte must show as U+FFFD: {message:?}"
    );
}

/// A truncated multibyte sequence at the end of a payload is what a
/// byte-sliced upstream buffer produces.
#[test]
fn truncated_multibyte_sequence_does_not_fail_the_payload() {
    let mut payload = b"{\"message\":\"\xe6\x97\xa5\xe6\x9c".to_vec();
    payload.extend_from_slice(b"\"}\n");

    let (events, outcome) = parse_batch(&payload);
    assert_eq!(
        events.len(),
        1,
        "a truncated codepoint must not cost the record"
    );
    assert!(outcome.lossy);
}

/// One malformed line must cost that line only. Failing the payload discards
/// every event already parsed from it -- up to a full batch.
#[test]
fn one_bad_line_does_not_discard_the_rest_of_the_payload() {
    let payload = concat!(
        "{\"message\":\"日本語\"}\n",
        "{not json at all}\n",
        "{\"message\":\"Ärger\"}\n",
    )
    .as_bytes();

    let (events, outcome) = parse_batch(payload);
    assert_eq!(events.len(), 2, "both good lines must survive");
    assert_eq!(outcome.parsed, 2);
    assert_eq!(outcome.bad_lines, 1);
    assert_eq!(events[0].get_str("message"), Some("日本語"));
    assert_eq!(events[1].get_str("message"), Some("Ärger"));
}

/// An all-bad payload is still not an error -- it yields nothing, and the
/// caller counts it.
#[test]
fn a_payload_of_only_bad_lines_yields_no_events() {
    let (events, outcome) = parse_batch(b"{bad}\n{also bad}\n");
    assert!(events.is_empty());
    assert_eq!(outcome.bad_lines, 2);
    assert_eq!(outcome.parsed, 0);
}

/// A line-splitter that assumed one byte per character would cut inside a
/// codepoint here: every sample is padded past any plausible fixed width.
#[test]
fn long_multibyte_lines_split_on_newlines_only() {
    let long: String = "日本語".repeat(500);
    let payload = ndjson("message", &long);

    let (events, _) = parse_batch(&payload);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].get_str("message"), Some(long.as_str()));
}

/// Every registered transform, fed non-ASCII text in the fields it is most
/// likely to slice: `message`, and the identity fields the dissect
/// code splits on a delimiter.
///
/// A transform may drop or error -- neither is a failure here. The only
/// failure is a panic, which in production takes the pod and stalls the
/// partition.
#[test]
fn no_transform_panics_on_non_ascii_input() {
    dfe_runtime::testutil::on_a_deep_stack(non_ascii_input_sweep);
}

fn non_ascii_input_sweep() {
    for source in registry::sources() {
        let transform = registry::lookup(source).expect("registered source resolves");

        for (label, sample) in SAMPLES {
            // Fields chosen because the transforms split, slice or case-fold
            // them: message text, a domain-prefixed user, an email-shaped
            // value, and a URL.
            let event = serde_json::json!({
                "message": sample,
                "event": { "original": sample, "action": sample, "code": sample },
                "user": { "name": format!("DOMAIN\\{sample}"), "email": format!("{sample}@{sample}.example") },
                "url": { "original": format!("https://{sample}.example/{sample}?q={sample}") },
                "source": { "ip": "10.0.0.1", "address": sample },
                "host": { "name": sample, "hostname": sample },
                "process": { "command_line": format!("C:\\{sample}\\{sample}.exe --flag {sample}") },
                "winlog": { "event_data": { sample.to_string(): sample } },
                "tags": [sample, sample],
            });

            let payload = format!("{event}\n");
            let (events, _) = parse_batch(payload.as_bytes());
            assert_eq!(events.len(), 1, "{source} / {label}: fixture did not parse");

            // The assertion is that this returns at all.
            let (out, outcome) = transform_batch(transform, events);
            assert_eq!(
                outcome.total(),
                1,
                "{source} / {label}: event vanished without being counted"
            );

            // Whatever came out must still serialise -- a transform that
            // produced a broken string would be counted as failed here.
            let (_, serialised) = serialise_batch(&out);
            assert_eq!(
                serialised.failed, 0,
                "{source} / {label}: output does not serialise"
            );
        }
    }
}

/// The same sweep over degenerate DOCUMENTS rather than degenerate values.
///
/// The two sweeps above always hand a transform a fully-populated structure
/// and vary only the strings inside it, so a field being ABSENT, the document
/// being empty, or a field being explicitly null is never exercised -- and an
/// explicit null is a shape several pipelines write themselves, which is why
/// `Event::has_value` exists.
#[test]
fn no_transform_panics_on_a_degenerate_document() {
    dfe_runtime::testutil::on_a_deep_stack(degenerate_document_sweep);
}

fn degenerate_document_sweep() {
    let documents = [
        ("empty", serde_json::json!({})),
        ("minimal message", serde_json::json!({ "message": "{}" })),
        (
            "null fields",
            serde_json::json!({ "message": null, "event": null }),
        ),
    ];

    for source in registry::sources() {
        let transform = registry::lookup(source).expect("registered source resolves");

        for (label, document) in &documents {
            let payload = format!("{document}\n");
            let (events, _) = parse_batch(payload.as_bytes());
            let (out, outcome) = transform_batch(transform, events);
            assert_eq!(outcome.total(), 1, "{source} / {label}: event vanished");
            let (_, serialised) = serialise_batch(&out);
            assert_eq!(
                serialised.failed, 0,
                "{source} / {label}: output does not serialise"
            );
        }
    }
}

/// The same sweep with the fields EMPTY and with lone surrogual escapes and
/// control characters, which is what a truncated upstream buffer produces.
#[test]
fn no_transform_panics_on_degenerate_input() {
    dfe_runtime::testutil::on_a_deep_stack(degenerate_input_sweep);
}

fn degenerate_input_sweep() {
    let degenerate = [
        ("empty", ""),
        ("single space", " "),
        ("lone backslash", "\\"),
        ("trailing backslash", "domain\\"),
        ("leading backslash", "\\user"),
        ("only separators", "\\\\\\"),
        ("replacement char", "\u{FFFD}"),
        ("nul-ish escape", "a\u{0000}b"),
        ("del", "a\u{007F}b"),
        ("combining only", "\u{0301}\u{0301}"),
        ("bare at", "@"),
        ("dot only", "\u{FF0E}"),
    ];

    for source in registry::sources() {
        let transform = registry::lookup(source).expect("registered source resolves");

        for (label, sample) in degenerate {
            let event = serde_json::json!({
                "message": sample,
                "event": { "original": sample, "action": sample },
                "user": { "name": sample, "email": sample },
                "url": { "original": sample },
                "host": { "name": sample },
                "process": { "command_line": sample },
            });

            let payload = format!("{event}\n");
            let (events, _) = parse_batch(payload.as_bytes());
            let (out, outcome) = transform_batch(transform, events);
            assert_eq!(outcome.total(), 1, "{source} / {label}: event vanished");
            let (_, serialised) = serialise_batch(&out);
            assert_eq!(
                serialised.failed, 0,
                "{source} / {label}: output does not serialise"
            );
        }
    }
}

/// Escaped control characters and quotes inside a non-ASCII string must not
/// confuse the NDJSON line split -- a literal newline never appears inside a
/// JSON string, only its `\n` escape.
#[test]
fn escaped_newlines_inside_a_string_do_not_split_the_line() {
    let payload = "{\"message\":\"日本語\\nsecond half \\\"quoted\\\"\"}\n".as_bytes();

    let (events, _) = parse_batch(payload);
    assert_eq!(events.len(), 1, "the escape must not end the line");
    assert_eq!(
        events[0].get_str("message"),
        Some("日本語\nsecond half \"quoted\"")
    );
}
