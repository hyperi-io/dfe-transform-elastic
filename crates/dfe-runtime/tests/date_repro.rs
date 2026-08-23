// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The date-format behaviours the corpus pinned: a short format must not
//! swallow a longer text, a wrong day name does not fail the parse, and the
//! processor's numeric timezone shifts the instant.

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

#[test]
fn a_numeric_processor_timezone_shifts_the_instant() {
    let parsed = parse_date(
        "Tue Jan 14 16:22:01 2026",
        &["E MMM dd HH:mm:ss yyyy"],
        Some("+03:30"),
    );
    assert_eq!(parsed.as_deref(), Some("2026-01-14T12:52:01.000Z"));
}
