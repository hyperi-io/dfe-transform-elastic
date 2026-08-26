// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Elastic `date` processor formats, parsed in the order the pipeline lists.
//!
//! A pipeline names its formats as Java `DateTimeFormatter` patterns plus the
//! four reserved words. Both halves live here so a generated call site is one
//! function call over a literal slice, rather than a chain of parse attempts
//! in which the last one to succeed silently overwrites the first.

use std::borrow::Cow;

use chrono::{DateTime, Datelike, FixedOffset, NaiveDateTime, TimeZone, Utc};

/// The output shape: ISO 8601 with milliseconds, ending in `Z`.
///
/// Everything is converted to UTC before it is formatted, and Elastic writes
/// that zone as `Z` rather than `+00:00`.
const ISO_OUT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// The same shape in a zone that is not UTC, which prints its offset.
const OFFSET_OUT: &str = "%Y-%m-%dT%H:%M:%S%.3f%:z";

/// Zero offset, the fallback when no zone is named anywhere.
const UTC_OFFSET: FixedOffset = match FixedOffset::east_opt(0) {
    Some(offset) => offset,
    None => panic!("zero is a valid offset"),
};

/// Parse `input` against each format in turn, returning the first success.
///
/// `timezone` is the processor's own `timezone` setting, applied only when the
/// format itself carries no zone -- an offset in the text always wins.
#[must_use]
pub fn parse_date(input: &str, formats: &[&str], timezone: Option<&str>) -> Option<String> {
    parse_date_out(input, formats, timezone, None)
}

/// As [`parse_date`], with the processor's `output_format` applied.
///
/// A pipeline that reformats a timestamp and feeds it to the NEXT date
/// processor depends on this: without it the second parse sees the ISO form
/// its own format list does not describe, and the field stops being set.
#[must_use]
pub fn parse_date_out(
    input: &str,
    formats: &[&str],
    timezone: Option<&str>,
    output_format: Option<&str>,
) -> Option<String> {
    let input = input.trim();

    // `ZoneId.of` throws BEFORE any parsing, so a processor configured with a
    // zone Java does not know fails outright and runs its `on_failure` --
    // which for cisco/ftd removes `event.timezone` and re-parses as UTC.
    // Falling back to UTC here instead kept a field Elasticsearch had deleted.
    if let Some(zone) = timezone.filter(|zone| !zone.trim().is_empty())
        && processor_zone_offset(zone).is_none()
    {
        return None;
    }

    let parsed = formats
        .iter()
        .find_map(|format| parse_one(input, format, timezone))?;

    // The processor's OWN `timezone` is the output zone too, not just the
    // parsing one: Elasticsearch formats the instant in it, so zscaler's
    // `timezone: '{{{event.timezone}}}'` writes `+03:30` where rendering in
    // UTC writes `Z` an offset out. A zone the text named rather than the
    // processor still renders in UTC, which is what the corpus carries, and a
    // configured zone that IS UTC renders `Z` rather than `+00:00`.
    let configured = timezone.and_then(zone_offset).filter(|zone| {
        zone.local_minus_utc() != 0 && output_format.is_none() && !offset_in_text(input)
    });

    if let Some(zone) = configured {
        return Some(parsed.with_timezone(&zone).format(OFFSET_OUT).to_string());
    }
    let out = output_format.map_or(Cow::Borrowed(ISO_OUT), java_to_chrono);
    Some(parsed.with_timezone(&Utc).format(&out).to_string())
}

/// Whether the TEXT carried a zone of its own, which beats the processor's.
fn offset_in_text(input: &str) -> bool {
    // Only past the date, so `2023-10-16` is not read as carrying one.
    input.len() > 10
        && (input.ends_with('Z')
            || input[10..].contains('+')
            || input[10..].rfind('-').is_some_and(|at| at > 2))
}

fn parse_one(input: &str, format: &str, timezone: Option<&str>) -> Option<DateTime<FixedOffset>> {
    match format {
        // Elastic scales the WHOLE double to milliseconds and truncates
        // (`Instant.ofEpochMilli((long) (Double.parseDouble(date) * 1000.0))`),
        // so its resolution stops at the millisecond. Taking the fraction off
        // first instead loses precision the integer seconds have already eaten:
        // `1742799479.852 - 1742799479.0` is 0.851999998, which renders .851
        // where Elastic renders .852.
        "UNIX" => {
            let seconds = input.parse::<f64>().ok().filter(|s| *s > 0.0)?;
            #[allow(clippy::cast_possible_truncation)]
            DateTime::from_timestamp_millis((seconds * 1000.0) as i64).map(Into::into)
        }
        "UNIX_MS" => {
            let millis = input.parse::<i64>().ok().filter(|ms| *ms > 0)?;
            DateTime::from_timestamp_millis(millis).map(Into::into)
        }
        // TAI64N labels are hex, and the leap-second table they need to become
        // UTC is not carried here. Falling through is honest; guessing is not.
        "TAI64N" => None,
        // Elasticsearch's own named formats, not Java patterns; every one of
        // them is ISO 8601 with a different optionality, and nanosecond
        // precision renders back at Elastic's milliseconds.
        "ISO8601"
        | "strict_date_optional_time_nanos"
        | "strict_date_optional_time"
        | "date_optional_time"
        | "date_time"
        | "date_time_no_millis"
        | "strict_date_time"
        | "strict_date_time_no_millis" => parse_iso8601(input),
        java => parse_java(input, java, timezone),
    }
}

fn parse_iso8601(input: &str) -> Option<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(input)
        .or_else(|_| DateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S%.f%:z"))
        .or_else(|_| DateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S%z"))
        .ok()
        .or_else(|| {
            NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S%.f")
                .ok()
                .and_then(|naive| Utc.from_utc_datetime(&naive).fixed_offset().into())
        })
}

/// Parse against a Java pattern, expanding its optional sections first.
///
/// `[EEE ]MMM [ ]d[ yyyy] HH:mm:ss` is one pattern to Java and sixteen to
/// chrono, which has no optional syntax -- so the brackets used to reach the
/// format string as literal characters and the pattern matched nothing at all.
/// A yearless Cisco date fell back to whatever `@timestamp` already held,
/// which the comparison skips, so only `event.start` and `event.end` showed it.
fn parse_java(input: &str, java: &str, timezone: Option<&str>) -> Option<DateTime<FixedOffset>> {
    expand_optional(java)
        .iter()
        .find_map(|candidate| parse_java_exact(input, candidate, timezone))
}

/// Every reading of a pattern's optional sections, the fullest one first.
///
/// Java takes an optional section when it can, so the order matters: `[ yyyy]`
/// present must be tried before `[ yyyy]` absent, or a date carrying a year
/// parses without it and the year is silently replaced by this one.
fn expand_optional(java: &str) -> Vec<Cow<'_, str>> {
    let Some(open) = java.find('[') else {
        return vec![Cow::Borrowed(java)];
    };
    let mut depth = 1usize;
    let mut close = None;
    for (index, c) in java[open + 1..].char_indices() {
        match c {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    close = Some(open + 1 + index);
                    break;
                }
            }
            _ => {}
        }
    }
    let Some(close) = close else {
        return vec![Cow::Borrowed(java)];
    };

    let (head, inner, tail) = (&java[..open], &java[open + 1..close], &java[close + 1..]);
    let mut out = Vec::new();
    for present in [true, false] {
        let body = if present { inner } else { "" };
        for rest in expand_optional(tail) {
            out.push(Cow::Owned(format!("{head}{body}{rest}")));
            // A pattern of many optional sections would otherwise expand
            // exponentially; the vendor ones have at most a handful.
            if out.len() >= 64 {
                return out;
            }
        }
    }
    out
}

/// Parse against one fully-resolved pattern, zoned first, then local.
fn parse_java_exact(
    input: &str,
    java: &str,
    timezone: Option<&str>,
) -> Option<DateTime<FixedOffset>> {
    let mut chrono = java_to_chrono(java);

    // The date processor resolves the date from its numeric fields and
    // IGNORES a day name that disagrees -- zscaler's fixtures carry a "Tue"
    // on a Wednesday and Elasticsearch parses them. chrono cross-checks and
    // rejects, so a LEADING day name is dropped from both sides; the
    // separators that follow it sit in both and keep the parse aligned.
    let input = if let Some(rest) = chrono
        .strip_prefix("%a")
        .or_else(|| chrono.strip_prefix("%A"))
    {
        let rest = rest.to_string();
        chrono = Cow::Owned(rest);
        Cow::Owned(
            input
                .trim_start_matches(|c: char| c.is_ascii_alphabetic())
                .to_string(),
        )
    } else {
        Cow::Borrowed(input)
    };

    // A BSD syslog date carries no year. Java fills in the ingesting node's
    // current one, so the text is prefixed rather than the parse failing.
    let input = if chrono.contains("%Y") || chrono.contains("%y") {
        input
    } else {
        chrono = Cow::Owned(format!("%Y {chrono}"));
        Cow::Owned(format!("{} {input}", Utc::now().year()))
    };

    if let Ok(dt) = DateTime::parse_from_str(&input, &chrono) {
        return Some(dt);
    }

    // A zone NAME cannot be resolved to an offset by chrono, so the text is
    // re-parsed without it and the trailing abbreviation is read separately.
    let (naive, named) = parse_naive(&input, &chrono)?;
    let offset = named
        .and_then(|zone| zone_offset(&zone))
        .or_else(|| timezone.and_then(processor_zone_offset))
        .unwrap_or(UTC_OFFSET);
    offset
        .from_local_datetime(&naive)
        .earliest()
        .map(|dt| dt.fixed_offset())
}

/// A naive datetime plus the zone name the pattern asked for, if any.
///
/// chrono's `%Z` parse consumes far more than a zone name -- digits and
/// punctuation included -- so `HH:mm` + `%Z` swallowed `:26.653Z` whole and
/// a minute-precision format beat the full one, truncating every mimecast
/// indicator timestamp. A pattern ending in `%Z` is split by hand instead:
/// the zone is the TRAILING ALPHABETIC run, abutting or space-separated, and
/// the rest must parse EXACTLY. A `%Z` pattern with no zone tail does not
/// match at all, which is Java's own reading of a mandatory `z`.
fn parse_naive(input: &str, chrono: &str) -> Option<(NaiveDateTime, Option<String>)> {
    if let Some(stripped) = chrono
        .strip_suffix(" %Z")
        .or_else(|| chrono.strip_suffix("%Z"))
    {
        let trimmed = input.trim_end();
        let zone_start = trimmed
            .rfind(|c: char| !c.is_ascii_alphabetic())
            .map_or(0, |at| at + 1);
        let (head, zone) = trimmed.split_at(zone_start);
        if !zone.is_empty()
            && let Ok(naive) = NaiveDateTime::parse_from_str(head.trim_end(), stripped.trim_end())
        {
            return Some((naive, Some(zone.to_string())));
        }
        return None;
    }
    NaiveDateTime::parse_from_str(input, chrono)
        .ok()
        .map(|naive| (naive, None))
}

/// Zone abbreviations with one unambiguous offset, in minutes east.
///
/// Deliberately partial. `CST` and `IST` each name several zones, so an
/// entry for them would silently pick one; they are left out and fall back
/// to the processor's own `timezone` setting, which is where a deployment
/// says which one it means.
const ZONE_ABBREVIATIONS: &[(&str, i32)] = &[
    ("ACDT", 630),
    ("ACST", 570),
    ("AEDT", 660),
    ("AEST", 600),
    ("AKDT", -480),
    ("AKST", -540),
    ("AWST", 480),
    ("BST", 60),
    ("CDT", -300),
    ("CEST", 120),
    ("CET", 60),
    ("EDT", -240),
    ("EEST", 180),
    ("EET", 120),
    ("EST", -300),
    ("HKT", 480),
    ("HST", -600),
    ("JST", 540),
    ("KST", 540),
    ("MDT", -360),
    ("MSK", 180),
    ("MST", -420),
    ("NZDT", 780),
    ("NZST", 720),
    ("PDT", -420),
    ("PST", -480),
    ("SGT", 480),
    ("WEST", 60),
    ("WET", 0),
];

/// The three-letter abbreviations `java.time.ZoneId.of` accepts.
///
/// Its `SHORT_IDS` map, and nothing else. A `z` in a FORMAT parses a zone's
/// display name, so `EDT` in the text is fine; `ZoneId.of("EDT")` throws, so a
/// processor configured with it FAILS its date and runs its `on_failure` --
/// which for cisco/ftd removes `event.timezone` and re-parses as UTC. Reading
/// it as an offset kept a field Elasticsearch had deleted.
const JAVA_SHORT_ZONE_IDS: [&str; 28] = [
    "ACT", "AET", "AGT", "ART", "AST", "BET", "BST", "CAT", "CNT", "CST", "CTT", "EAT", "ECT",
    "EST", "HST", "IET", "IST", "JST", "MIT", "MST", "NET", "NST", "PLT", "PNT", "PRT", "PST",
    "SST", "VST",
];

/// The offset a PROCESSOR's `timezone` setting names.
///
/// Stricter than [`zone_offset`], because this is `ZoneId.of` rather than a
/// format's zone-name parse, and it rejects everything Java rejects.
fn processor_zone_offset(zone: &str) -> Option<FixedOffset> {
    let trimmed = zone.trim();
    let alphabetic = trimmed.chars().all(|c| c.is_ascii_alphabetic());
    if alphabetic
        && !matches!(trimmed, "UTC" | "GMT" | "Z" | "UT" | "Zulu")
        && !JAVA_SHORT_ZONE_IDS.contains(&trimmed)
    {
        return None;
    }
    zone_offset(trimmed)
}

/// The offset a zone spelling names, for the spellings that carry one.
fn zone_offset(zone: &str) -> Option<FixedOffset> {
    let zone = zone.trim();
    if matches!(zone, "UTC" | "GMT" | "Z" | "UT" | "Zulu") {
        return FixedOffset::east_opt(0);
    }
    // `GMT+03:30` and `UTC-05:00` are Java zone ids, and zscaler's tunnel
    // events carry the first of them in `event.timezone`.
    for prefix in ["GMT", "UTC", "UT"] {
        if let Some(rest) = zone.strip_prefix(prefix)
            && rest.starts_with(['+', '-'])
        {
            return zone_offset(rest);
        }
    }
    if let Some((_, minutes)) = ZONE_ABBREVIATIONS.iter().find(|(name, _)| *name == zone) {
        return FixedOffset::east_opt(minutes * 60);
    }

    let digits = zone.strip_prefix(['+', '-'])?;
    let sign = if zone.starts_with('-') { -1 } else { 1 };
    let digits: String = digits.chars().filter(char::is_ascii_digit).collect();
    let (hours, minutes) = match digits.len() {
        2 => (digits.parse::<i32>().ok()?, 0),
        4 => (
            digits[..2].parse::<i32>().ok()?,
            digits[2..].parse::<i32>().ok()?,
        ),
        _ => return None,
    };
    FixedOffset::east_opt(sign * (hours * 3600 + minutes * 60))
}

/// Translate a Java `DateTimeFormatter` pattern into a chrono one.
///
/// Only the tokens the pipelines actually use are translated; an unknown
/// letter is passed through, which makes the parse fail rather than match
/// something it should not.
#[must_use]
pub fn java_to_chrono(java: &str) -> Cow<'_, str> {
    if !java.chars().any(|c| c.is_ascii_alphabetic()) {
        return Cow::Borrowed(java);
    }

    let mut out = String::with_capacity(java.len() * 2);
    let mut chars = java.chars().peekable();

    while let Some(c) = chars.next() {
        // A quoted run is a literal, and '' is a literal quote.
        if c == '\'' {
            if chars.peek() == Some(&'\'') {
                chars.next();
                out.push('\'');
                continue;
            }
            for lit in chars.by_ref() {
                if lit == '\'' {
                    break;
                }
                out.push(lit);
            }
            continue;
        }

        if !c.is_ascii_alphabetic() {
            if c == '%' {
                out.push('%');
            }
            out.push(c);
            continue;
        }

        let mut run = 1usize;
        while chars.peek() == Some(&c) {
            chars.next();
            run += 1;
        }
        out.push_str(&token(c, run));
    }

    Cow::Owned(out)
}

/// One Java pattern letter, repeated `run` times, as chrono directives.
fn token(letter: char, run: usize) -> String {
    match (letter, run) {
        ('y' | 'u', 2) => "%y",
        ('y' | 'u', _) => "%Y",
        ('M' | 'L', 1) => "%-m",
        ('M' | 'L', 2) => "%m",
        ('M' | 'L', 3) => "%b",
        ('M' | 'L', _) => "%B",
        ('d', 1) => "%-d",
        ('d', _) => "%d",
        ('D', _) => "%j",
        ('E', 4) => "%A",
        ('E', _) => "%a",
        ('H', 1) => "%-H",
        ('H', _) => "%H",
        ('h', 1) => "%-I",
        ('h', _) => "%I",
        ('m', 1) => "%-M",
        ('m', _) => "%M",
        ('s', 1) => "%-S",
        ('s', _) => "%S",
        // Java writes the fraction WITHOUT a separator, so the dot in the
        // pattern is a literal the chrono directive must not repeat.
        ('S', n) => return format!("%{}f", n.min(9)),
        ('n' | 'N', _) => "%9f",
        ('a', _) => "%p",
        ('X', 1) | ('Z', 1..=3) => "%z",
        ('X' | 'x' | 'Z', _) => "%:z",
        ('z' | 'V', _) => "%Z",
        ('G', _) => "AD",
        _ => "",
    }
    .to_string()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn java_patterns_translate() {
        assert_eq!(
            java_to_chrono("yyyy MMM d HH:mm:ss zzz"),
            "%Y %b %-d %H:%M:%S %Z"
        );
        assert_eq!(
            java_to_chrono("yyyy-MM-dd'T'HH:mm:ss.SSSXXX"),
            "%Y-%m-%dT%H:%M:%S.%3f%:z"
        );
        assert_eq!(java_to_chrono("MMM  d HH:mm:ss"), "%b  %-d %H:%M:%S");
    }

    /// The fullest reading comes first, so a date carrying a year is never
    /// parsed by the yearless variant and quietly given this year instead.
    #[test]
    fn optional_sections_expand_fullest_first() {
        let expanded = expand_optional("[EEE ]MMM [ ]d[ yyyy] HH:mm:ss");
        assert_eq!(expanded.first().unwrap(), "EEE MMM  d yyyy HH:mm:ss");
        assert_eq!(expanded.last().unwrap(), "MMM d HH:mm:ss");
        assert_eq!(expanded.len(), 8);
    }

    /// Verbatim from `pipelines/cisco_asa/default.yml`. Cisco pads a
    /// single-digit day to two columns, which is what the `[ ]` is for.
    #[test]
    fn a_padded_cisco_day_parses_through_the_optional_space() {
        const FORMATS: [&str; 2] = ["ISO8601", "[EEE ]MMM [ ]d[ yyyy] HH:mm:ss[.SSS][ z]"];

        let padded = parse_date_out("May  5 17:51:17", &FORMATS, Some("UTC"), None).unwrap();
        assert!(
            padded.contains("-05-05T17:51:17.000"),
            "the day and time must survive the padding: {padded}"
        );

        // The year, where the text carries one, must be the text's own.
        let dated = parse_date_out("Oct 10 2018 12:34:56", &FORMATS, Some("UTC"), None).unwrap();
        assert!(
            dated.starts_with("2018-10-10T12:34:56.000"),
            "the year in the text must win: {dated}"
        );
    }

    /// Verbatim from `pipelines/cisco/nexus/default.yml`.
    #[test]
    fn a_nexus_timestamp_parses_with_its_zone_name() {
        let formats = ["yyyy MMM d HH:mm:ss zzz", "yyyy MMM d HH:mm:ss"];
        let out = parse_date("2023 May  2 12:55:19 UTC", &formats, None).unwrap();
        assert_eq!(out, "2023-05-02T12:55:19.000Z");
    }

    /// The single-space format has to cover the padded day too -- chrono
    /// treats a space in a pattern as "any run of whitespace".
    #[test]
    fn a_padded_day_matches_the_single_space_format() {
        let out = parse_date("2023 Apr  7 09:36:56", &["yyyy MMM d HH:mm:ss"], None).unwrap();
        assert_eq!(out, "2023-04-07T09:36:56.000Z");
    }

    /// The FIRST format that parses wins; Elastic stops there, and a later
    /// one that also parses must not overwrite it.
    #[test]
    fn the_first_matching_format_wins() {
        let out = parse_date("1587230269", &["UNIX", "UNIX_MS"], None).unwrap();
        assert_eq!(out, "2020-04-18T17:17:49.000Z");
    }

    /// A fractional UNIX second scales to milliseconds WHOLE, the way Elastic
    /// does it. Subtracting the integer seconds off first costs the precision
    /// they have already eaten, and every one of these lands a millisecond
    /// early that way.
    #[test]
    fn a_fractional_unix_second_keeps_its_millisecond() {
        for (input, expected) in [
            ("1742799479.852", "2025-03-24T06:57:59.852Z"),
            ("1742541951.883", "2025-03-21T07:25:51.883Z"),
            ("1636625755.218", "2021-11-11T10:15:55.218Z"),
            ("1742799480.061", "2025-03-24T06:58:00.061Z"),
        ] {
            assert_eq!(parse_date(input, &["UNIX"], None).unwrap(), expected);
        }
    }

    /// The processor's zone is the OUTPUT zone as well as the parsing one:
    /// Elasticsearch formats the instant in it rather than converting to UTC.
    /// zscaler's tunnel dates carry `+03:30` for exactly this reason.
    #[test]
    fn the_processor_timezone_applies_when_the_text_carries_none() {
        let out = parse_date(
            "2023 May 2 12:55:19",
            &["yyyy MMM d HH:mm:ss"],
            Some("+1000"),
        );
        assert_eq!(out.unwrap(), "2023-05-02T12:55:19.000+10:00");
    }

    /// A configured zone that IS UTC prints `Z`, not `+00:00`.
    #[test]
    fn a_configured_utc_still_prints_z() {
        let out = parse_date("2023 May 2 12:55:19", &["yyyy MMM d HH:mm:ss"], Some("UTC"));
        assert_eq!(out.unwrap(), "2023-05-02T12:55:19.000Z");
    }

    /// Elasticsearch's own named formats, not Java patterns. They were read as
    /// literal patterns and failed, and panw names
    /// `strict_date_optional_time_nanos` twenty times.
    #[test]
    fn elasticsearch_named_formats_parse_as_iso8601() {
        for format in [
            "strict_date_optional_time_nanos",
            "strict_date_optional_time",
            "date_optional_time",
            "date_time",
            "date_time_no_millis",
            "strict_date_time",
            "strict_date_time_no_millis",
        ] {
            let out = parse_date("2021-05-26T16:26:47.123456789Z", &[format], None);
            assert_eq!(out.as_deref(), Some("2021-05-26T16:26:47.123Z"), "{format}");
        }
    }

    /// The panw case exactly: a nanosecond instant rendered back at the
    /// millisecond precision Elastic emits.
    #[test]
    fn a_nanosecond_instant_renders_at_millisecond_precision() {
        let formats = ["yyyy/MM/dd HH:mm:ss", "strict_date_optional_time_nanos"];
        let out = parse_date("2021-05-26T16:26:47.000000000Z", &formats, None);
        assert_eq!(out.as_deref(), Some("2021-05-26T16:26:47.000Z"));
    }

    #[test]
    fn an_offset_in_the_text_beats_the_processor_setting() {
        let formats = ["yyyy-MM-dd HH:mm:ss Z"];
        let out = parse_date("2023-05-02 12:55:19 -0500", &formats, Some("+1000")).unwrap();
        assert_eq!(out, "2023-05-02T17:55:19.000Z");
    }

    /// Verbatim from `tests/fixtures/cisco/nexus`: AEST is +10:00, and
    /// treating an unrecognised abbreviation as UTC put the event ten hours
    /// out rather than failing visibly.
    #[test]
    fn a_named_zone_shifts_the_instant() {
        let out = parse_date(
            "2023 May 3 13:55:35.928 AEST",
            &["yyyy MMM d HH:mm:ss.SSS zzz"],
            None,
        );
        assert_eq!(out.unwrap(), "2023-05-03T03:55:35.928Z");
    }

    #[test]
    fn a_format_that_does_not_match_yields_nothing() {
        assert!(parse_date("not a date", &["yyyy MMM d HH:mm:ss"], None).is_none());
    }
}
