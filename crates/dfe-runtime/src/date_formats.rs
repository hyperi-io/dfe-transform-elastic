// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Elastic `date` processor formats, parsed in the order the pipeline lists.
//!
//! A pipeline names its formats as Java `DateTimeFormatter` patterns plus the
//! four reserved words. Both halves live here so a generated call site is one
//! function call over a literal slice, rather than a chain of parse attempts
//! in which the last one to succeed silently overwrites the first.

use std::borrow::Cow;

use chrono::{
    DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, Offset, TimeDelta, TimeZone, Utc,
};
use chrono_tz::Tz;

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
        && resolve_zone(zone).is_none()
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
    let configured = timezone
        .and_then(resolve_zone)
        .map(|zone| zone.offset_at(parsed.to_utc()))
        .filter(|offset| {
            offset.local_minus_utc() != 0 && output_format.is_none() && !offset_in_text(input)
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
        // Zero is a TIMESTAMP, not a missing value: Elastic renders it
        // 1970-01-01T00:00:00.000Z, and ti_misp ships events whose
        // `publish_timestamp` is exactly that. Rejecting it sent 28 of its 59
        // events down the on_failure path with the epoch left as the string
        // "0". A negative is pre-1970 and equally real.
        // `epoch_second` and `epoch_millis` are the same two formats under
        // Elasticsearch's own names. A pipeline written against the ingest
        // node spells them that way and never says UNIX, so recognising only
        // the Beats spelling sent every such date down its on_failure path --
        // sysdig's cspm stream failed all 10 of its events on one
        // `epoch_second` field. 73 uses across 19 pipelines, splunk's alert
        // stream alone holding 30.
        "UNIX" | "epoch_second" => {
            let seconds = input.parse::<f64>().ok()?;
            #[allow(clippy::cast_possible_truncation)]
            DateTime::from_timestamp_millis((seconds * 1000.0) as i64).map(Into::into)
        }
        "UNIX_MS" | "epoch_millis" => {
            let millis = input.parse::<i64>().ok()?;
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
        // "optional_time": the time is optional, so a bare date is still valid
        // ISO 8601 and Elasticsearch reads it as midnight UTC.
        .or_else(|| {
            NaiveDate::parse_from_str(input, "%Y-%m-%d")
                .ok()
                .and_then(|date| date.and_hms_opt(0, 0, 0))
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
    named
        .and_then(|zone| zone_offset(&zone))
        .map(ProcessorZone::Fixed)
        .or_else(|| timezone.and_then(resolve_zone))
        .unwrap_or(ProcessorZone::Fixed(UTC_OFFSET))
        .read_local(&naive)
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
    if let Ok(naive) = NaiveDateTime::parse_from_str(input, chrono) {
        return Some((naive, None));
    }

    // A DATE-ONLY pattern. `NaiveDateTime` demands a time component, so
    // `yyyy-MM-dd` against `2024-12-15` failed here and took the whole
    // processor down its `on_failure` path -- which then removed the field the
    // following `rename` wanted, so one unparsed date cost two processors.
    // Java resolves that pattern to a `LocalDate` and the date processor reads
    // it at midnight, which is what Elasticsearch stores.
    NaiveDate::parse_from_str(input, chrono)
        .ok()
        .map(|date| (date.into(), None))
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

/// A zone a PROCESSOR's `timezone` setting names, resolved as far as it can be
/// without an instant.
///
/// An IANA region zone has no single offset -- `America/Denver` is -07:00 in
/// January and -06:00 in July -- so it stays a zone until an instant picks the
/// offset. Everything else reduces to a fixed one at resolve time.
#[derive(Clone, Copy)]
enum ProcessorZone {
    Fixed(FixedOffset),
    Named(Tz),
}

impl ProcessorZone {
    /// The offset this zone is at `instant`.
    fn offset_at(self, instant: DateTime<Utc>) -> FixedOffset {
        match self {
            Self::Fixed(offset) => offset,
            Self::Named(tz) => instant.with_timezone(&tz).offset().fix(),
        }
    }

    /// Read `naive` as a local time IN this zone.
    fn read_local(self, naive: &NaiveDateTime) -> Option<DateTime<FixedOffset>> {
        match self {
            Self::Fixed(offset) => offset.from_local_datetime(naive).earliest(),
            Self::Named(tz) => tz.from_local_datetime(naive).earliest().map(|dt| {
                // A local time inside a spring-forward gap does not exist and
                // `earliest` is None there, which is Java's own behaviour too.
                dt.fixed_offset()
            }),
        }
    }
}

/// The zone a PROCESSOR's `timezone` setting names, or `None` if `ZoneId.of`
/// would throw on it.
///
/// Stricter than [`zone_offset`] for the bare abbreviations, because this is
/// `ZoneId.of` rather than a format's zone-name parse. Wider for region ids,
/// because `ZoneId.of` accepts every IANA name and reading one as unknown
/// failed the date and ran the processor's `on_failure` -- which for arista
/// removes `event.timezone` and re-parses as UTC, losing the field
/// Elasticsearch keeps.
fn resolve_zone(zone: &str) -> Option<ProcessorZone> {
    let trimmed = zone.trim();
    if trimmed.contains('/') {
        return trimmed.parse::<Tz>().ok().map(ProcessorZone::Named);
    }
    let alphabetic = trimmed.chars().all(|c| c.is_ascii_alphabetic());
    if alphabetic
        && !matches!(trimmed, "UTC" | "GMT" | "Z" | "UT" | "Zulu")
        && !JAVA_SHORT_ZONE_IDS.contains(&trimmed)
    {
        return None;
    }
    zone_offset(trimmed).map(ProcessorZone::Fixed)
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

/// Add `count` of `unit` to an ISO 8601 timestamp, then take `back` seconds off.
///
/// `unit` is the vendor's own single character -- `d`, `h` or `m` -- because
/// that is how the threat-intel packages spell an expiry duration: one string
/// whose LAST CHARACTER selects the adder. `back` is the settling window those
/// same scripts subtract so an indicator expires just before its next
/// interval. The result is rendered the way the `date` processor that follows
/// them renders it, so the value is already right for a pipeline that omits
/// one.
///
/// Returns `None` for a unit this does not know or a timestamp it cannot read.
#[must_use]
pub fn iso8601_plus(input: &str, unit: char, count: i64, back: i64) -> Option<String> {
    let delta = match unit {
        'd' => TimeDelta::try_days(count),
        'h' => TimeDelta::try_hours(count),
        'm' => TimeDelta::try_minutes(count),
        _ => None,
    }?;
    Some(
        parse_iso8601(input.trim())?
            .checked_add_signed(delta)?
            .checked_sub_signed(TimeDelta::try_seconds(back)?)?
            .with_timezone(&Utc)
            .format(ISO_OUT)
            .to_string(),
    )
}

/// Add NANOSECONDS to an ISO 8601 instant, keeping the offset it arrived with.
///
/// [`iso8601_plus`] converts to UTC because its callers want UTC. A duration
/// window does not: Elasticsearch renders `event.end` in the same zone as
/// `event.start`, so sophos's captured `+05:30` survives into the output and a
/// `Z` here would name the right instant in the wrong words.
///
/// Returns `None` for a timestamp this cannot read.
#[must_use]
pub fn iso8601_plus_nanos(input: &str, nanos: i64) -> Option<String> {
    let at = parse_iso8601(input.trim())?.checked_add_signed(TimeDelta::nanoseconds(nanos))?;
    Some(if at.offset().local_minus_utc() == 0 {
        at.format(ISO_OUT).to_string()
    } else {
        at.format(OFFSET_OUT).to_string()
    })
}

/// Render epoch SECONDS the way the `date` processor renders a timestamp.
///
/// `ti_misp` holds `misp.attribute.timestamp` in seconds and its decay script
/// multiplies by 1000 to hand `Instant.ofEpochMilli` what it wants, so the
/// base of the expiry window is a number where every other package's is
/// already a string.
#[must_use]
pub fn epoch_seconds_to_iso8601(seconds: i64) -> Option<String> {
    DateTime::from_timestamp(seconds, 0).map(|at| at.format(ISO_OUT).to_string())
}

/// Whether one ISO 8601 timestamp is strictly before another.
///
/// `None` where either side does not read as a timestamp, which is a
/// different answer from `false` -- the caller writes nothing rather than
/// claiming an ordering it could not establish.
#[must_use]
pub fn iso8601_is_before(one: &str, two: &str) -> Option<bool> {
    Some(parse_iso8601(one.trim())? < parse_iso8601(two.trim())?)
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

        // chrono's fixed-width fraction exists at 3, 6 and 9 digits only, so a
        // shorter run has to take the literal dot back and use the
        // variable-width directive instead.
        if c == 'S' && !matches!(run, 3 | 6 | 9) && out.ends_with('.') {
            out.pop();
            out.push_str("%.f");
            continue;
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

    /// Verbatim from `o365_metrics`, whose every stream carries a
    /// `Report Refresh Date`. Java resolves `yyyy-MM-dd` to a `LocalDate` and
    /// the date processor reads it at midnight; `NaiveDateTime` demands a time
    /// component, so this returned None and the processor took its
    /// `on_failure` -- which removed the field the following `rename` wanted,
    /// costing two processors for one unparsed date.
    #[test]
    fn a_date_with_no_time_parses_at_midnight() {
        assert_eq!(
            parse_date_out("2024-12-15", &["yyyy-MM-dd"], Some("UTC"), None).as_deref(),
            Some("2024-12-15T00:00:00.000Z")
        );

        // The zone still applies: midnight in Sydney is the day before in UTC.
        assert_eq!(
            parse_date_out(
                "2024-12-15",
                &["yyyy-MM-dd"],
                Some("Australia/Sydney"),
                None
            )
            .as_deref(),
            Some("2024-12-15T00:00:00.000+11:00")
        );

        // A pattern WITH a time is unaffected, and a value that does not match
        // the pattern still declines.
        assert_eq!(
            parse_date_out("2024-12-15 09:30:00", &["yyyy-MM-dd HH:mm:ss"], None, None).as_deref(),
            Some("2024-12-15T09:30:00.000Z")
        );
        assert!(parse_date_out("not a date", &["yyyy-MM-dd"], None, None).is_none());
    }

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

    /// `ZoneId.of` accepts every IANA region id, so a processor configured
    /// with one must parse rather than run its `on_failure`. Verbatim from
    /// `pipelines/arista_ngfw/log/default.yml`, whose agent config sets
    /// `tz_offset: America/Denver`.
    #[test]
    fn an_iana_region_zone_resolves() {
        const FORMATS: [&str; 2] = ["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"];

        let out = parse_date_out(
            "2023-05-22 16:32:28.771",
            &FORMATS,
            Some("America/Denver"),
            None,
        )
        .unwrap();
        assert_eq!(out, "2023-05-22T16:32:28.771-06:00");
    }

    /// The offset is read AT THE EVENT, which is the whole reason the zone
    /// cannot be a table of fixed offsets: Denver is -06:00 on daylight time
    /// in May and -07:00 on standard time in January, and both dates go
    /// through the same configuration.
    #[test]
    fn a_region_zone_follows_daylight_saving() {
        const FORMATS: [&str; 1] = ["yyyy-MM-dd HH:mm:ss.SSS"];

        let summer = parse_date_out(
            "2023-07-04 12:00:00.000",
            &FORMATS,
            Some("America/Denver"),
            None,
        )
        .unwrap();
        let winter = parse_date_out(
            "2023-01-04 12:00:00.000",
            &FORMATS,
            Some("America/Denver"),
            None,
        )
        .unwrap();

        assert_eq!(summer, "2023-07-04T12:00:00.000-06:00");
        assert_eq!(winter, "2023-01-04T12:00:00.000-07:00");
    }

    /// A pipeline lists `.SSS`, `.SS` and `.S` because the device writes any
    /// of them, and each has to parse the width it names.
    #[test]
    fn a_short_fraction_parses_through_its_own_format() {
        const FORMATS: [&str; 3] = [
            "yyyy-MM-dd HH:mm:ss.SSS",
            "yyyy-MM-dd HH:mm:ss.SS",
            "yyyy-MM-dd HH:mm:ss.S",
        ];

        let two = parse_date_out("2023-05-21 09:58:40.25", &FORMATS, Some("UTC"), None).unwrap();
        assert_eq!(two, "2023-05-21T09:58:40.250Z");

        let three = parse_date_out("2023-05-21 09:58:40.477", &FORMATS, Some("UTC"), None).unwrap();
        assert_eq!(three, "2023-05-21T09:58:40.477Z");
    }

    /// A region id Java does not know still fails, so the processor's
    /// `on_failure` runs exactly where Elasticsearch runs it.
    #[test]
    fn an_unknown_region_zone_still_fails() {
        const FORMATS: [&str; 1] = ["yyyy-MM-dd HH:mm:ss.SSS"];

        assert!(
            parse_date_out(
                "2023-05-22 16:32:28.771",
                &FORMATS,
                Some("Middle/Earth"),
                None
            )
            .is_none()
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

    /// Elasticsearch's own names for the two epoch formats.
    ///
    /// A pipeline written against the ingest node spells them this way and
    /// never says UNIX. Recognising only the Beats spelling failed all 10 of
    /// sysdig's cspm events on one `epoch_second` field.
    #[test]
    fn the_elasticsearch_epoch_names_parse_as_their_beats_twins() {
        assert_eq!(
            parse_date("1736935526", &["epoch_second"], None).unwrap(),
            "2025-01-15T10:05:26.000Z"
        );
        assert_eq!(
            parse_date("1736935526000", &["epoch_millis"], None).unwrap(),
            "2025-01-15T10:05:26.000Z"
        );
        // Same value, same answer, whichever spelling the pipeline used.
        assert_eq!(
            parse_date("1587230269", &["epoch_second"], None).unwrap(),
            parse_date("1587230269", &["UNIX"], None).unwrap()
        );
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

    /// Epoch zero is a TIMESTAMP, not a missing value.
    ///
    /// Elastic renders it, and rejecting it sent 28 of `ti_misp`'s 59 events
    /// down the `on_failure` path with the epoch left as the string "0". Six
    /// sources moved when this was allowed. A negative is pre-1970 and just
    /// as real.
    #[test]
    fn epoch_zero_is_a_timestamp() {
        assert_eq!(
            parse_date("0", &["UNIX"], None).unwrap(),
            "1970-01-01T00:00:00.000Z"
        );
        assert_eq!(
            parse_date("0", &["UNIX_MS"], None).unwrap(),
            "1970-01-01T00:00:00.000Z"
        );
        assert_eq!(
            parse_date("-86400", &["UNIX"], None).unwrap(),
            "1969-12-31T00:00:00.000Z"
        );
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

    /// A duration window's end is rendered in the zone its start arrived in.
    ///
    /// The instant is the same either way; the TEXT is not, and the text is
    /// what the corpus compares. sophos's captured `event.end` reads
    /// `2017-01-31T14:16:49.000+05:30`.
    #[test]
    fn adding_nanoseconds_keeps_the_offset_it_arrived_with() {
        assert_eq!(
            iso8601_plus_nanos("2017-01-31T14:16:19.000+05:30", 30_000_000_000).unwrap(),
            "2017-01-31T14:16:49.000+05:30"
        );
        // A zero offset still prints `Z`, the same as every other renderer here.
        assert_eq!(
            iso8601_plus_nanos("2017-01-31T14:16:19.000Z", 30_000_000_000).unwrap(),
            "2017-01-31T14:16:49.000Z"
        );
        assert_eq!(iso8601_plus_nanos("not a timestamp", 1), None);
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

    /// Verbatim from `crowdstrike/vulnerability/default.yml`, tagged
    /// `date_cve_cisa_info_due_date`: "`optional_time`" means the time is
    /// optional, and a bare date is valid ISO 8601 read as midnight UTC.
    #[test]
    fn a_bare_date_parses_as_midnight_utc() {
        let out = parse_date("2025-03-01", &["ISO8601"], None);
        assert_eq!(out.as_deref(), Some("2025-03-01T00:00:00.000Z"));
    }
}
