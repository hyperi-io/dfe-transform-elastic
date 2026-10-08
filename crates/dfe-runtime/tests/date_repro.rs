// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The date-format behaviours the corpus pinned: a short format must not
//! swallow a longer text, a wrong day name does not fail the parse, and both
//! the processor's numeric timezone and a named zone in the text shift the
//! instant.

use dfe_runtime::date_formats::parse_date;

#[test]
fn mimecast_full_precision_survives_the_format_ladder() {
    let formats = [
        "yyyy-MM-dd'T'HH:mmz",
        "yyyy-MM-dd'T'HH:mmZ",
        "yyyy-MM-dd'T'HH:mm:ssz",
        "yyyy-MM-dd'T'HH:mm:ssZ",
        "yyyy-MM-dd'T'HH:mm:ss.Sz",
        "yyyy-MM-dd'T'HH:mm:ss.SZ",
        "yyyy-MM-dd'T'HH:mm:ss.SSz",
        "yyyy-MM-dd'T'HH:mm:ss.SSZ",
        "yyyy-MM-dd'T'HH:mm:ss.SSSz",
        "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
    ];
    let parsed = parse_date("2024-01-11T15:07:26.653Z", &formats, None);
    assert_eq!(parsed.as_deref(), Some("2024-01-11T15:07:26.653Z"));
}

#[test]
fn ctime_with_day_name_parses() {
    let formats = ["E MMM dd HH:mm:ss yyyy", "E MMM  d HH:mm:ss yyyy"];
    let parsed = parse_date("Tue Jan 14 16:22:01 2026", &formats, None);
    assert_eq!(parsed.as_deref(), Some("2026-01-14T16:22:01.000Z"));

    let padded = parse_date("Tue Jan  2 16:22:01 2026", &formats, None);
    assert_eq!(padded.as_deref(), Some("2026-01-02T16:22:01.000Z"));
}

/// The zone the processor names is where the text is read AND where the
/// result is written. `2026-01-14T16:22:01.000+03:30` is the same instant as
/// `2026-01-14T12:52:01.000Z`; Elasticsearch writes the first, which is what
/// zscaler's tunnel dates carry.
#[test]
fn a_numeric_processor_timezone_shifts_the_instant() {
    let parsed = parse_date(
        "Tue Jan 14 16:22:01 2026",
        &["E MMM dd HH:mm:ss yyyy"],
        Some("+03:30"),
    );
    assert_eq!(parsed.as_deref(), Some("2026-01-14T16:22:01.000+03:30"));
}

/// A named zone in the text shifts the instant; reading it as UTC puts the
/// event hours out rather than failing visibly. The text and the instant
/// Elasticsearch wrote for it are the first year-bearing device timestamp in
/// the Elastic data's `cisco/nexus/test-nexus.log-expected.json`.
#[cfg(feature = "testutil")]
#[test]
#[ignore = "reads Elastic-licensed test data kept outside this repository: set DFE_ELASTIC_FIXTURES to its fixtures/elastic directory and run with --ignored"]
#[allow(clippy::expect_used, clippy::panic)]
fn a_named_zone_shifts_the_instant() {
    let path =
        dfe_runtime::testutil::elastic_fixtures().join("cisco/nexus/test-nexus.log-expected.json");
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{} reads: {e}", path.display()));
    let expected: Vec<serde_json::Value> = serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("{}: not a JSON array: {e}", path.display()));

    // `<year> <month> <day> <time>.<millis> <zone>:`, the day space-padded.
    let stamp =
        regex::Regex::new(r"(\d{4} [A-Z][a-z]{2} +\d{1,2} \d{2}:\d{2}:\d{2}\.\d{3}) ([A-Z]{3,4}):")
            .expect("the pattern compiles");
    let (written, instant) = expected
        .iter()
        .find_map(|doc| {
            let found = stamp.captures(doc["event"]["original"].as_str()?)?;
            let zone = found.get(2)?.as_str();
            if matches!(zone, "UTC" | "GMT") {
                return None;
            }
            let wall: Vec<&str> = found.get(1)?.as_str().split_whitespace().collect();
            let instant = doc["cisco_nexus"]["log"]["time"].as_str()?;
            Some((format!("{} {zone}", wall.join(" ")), instant))
        })
        .unwrap_or_else(|| panic!("{} has no named-zone timestamp", path.display()));

    let parsed = parse_date(&written, &["yyyy MMM d HH:mm:ss.SSS zzz"], None)
        .unwrap_or_else(|| panic!("`{written}` did not parse"));
    let at = |text: &str| {
        chrono::DateTime::parse_from_rfc3339(text)
            .unwrap_or_else(|e| panic!("`{text}` is not RFC 3339: {e}"))
    };
    assert_eq!(at(&parsed), at(instant), "`{written}`");
}
