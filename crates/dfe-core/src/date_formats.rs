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
    DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Offset, TimeDelta,
    TimeZone, Utc,
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
    // The reserved words are reserved on the OUTPUT side too, and none of them
    // is a Java pattern.
    if let Some(named) = output_format.and_then(|name| named_output(name, &parsed)) {
        return Some(named);
    }
    let out = output_format.map_or(Cow::Borrowed(ISO_OUT), java_to_chrono);
    Some(parsed.with_timezone(&Utc).format(&out).to_string())
}

/// An `output_format` naming one of Elasticsearch's own formats rather than
/// spelling a Java pattern, rendered the way Elasticsearch renders it.
///
/// `None` for anything else, which leaves a real Java pattern to
/// [`java_to_chrono`]. That translator reads EVERY letter of its input as a
/// pattern letter, so a reserved word reaching it comes back as arithmetic on
/// the wrong fields: `epoch_millis` resolves to `%-I_%-M%-S`, which turned
/// endace's epoch into `10_4839` and threw in the `convert` after it, taking
/// the whole pipeline down its `on_failure` path.
fn named_output(name: &str, parsed: &DateTime<FixedOffset>) -> Option<String> {
    let utc = parsed.with_timezone(&Utc);
    Some(match name {
        "UNIX_MS" | "epoch_millis" => utc.timestamp_millis().to_string(),
        "UNIX" | "epoch_second" => {
            let seconds = utc.timestamp();
            let nanos = utc.timestamp_subsec_nanos();
            if nanos == 0 {
                seconds.to_string()
            } else if seconds < 0 {
                // A pre-1970 instant FLOORS its seconds, so half a second
                // before the epoch is second -1 plus 500 ms and printing the
                // pair as they stand gives -1.5 rather than -0.5.
                let whole = -(seconds + 1);
                let fraction = format!("{:09}", 1_000_000_000 - u64::from(nanos));
                format!("-{whole}.{}", fraction.trim_end_matches('0'))
            } else {
                let fraction = format!("{nanos:09}");
                format!("{seconds}.{}", fraction.trim_end_matches('0'))
            }
        }
        // Elasticsearch's nanosecond printer takes between three and nine
        // fraction digits, dropping trailing zeros above the third.
        "strict_date_optional_time_nanos" => {
            let nanos = utc.format("%9f").to_string();
            let trimmed = nanos.trim_end_matches('0');
            let fraction = if trimmed.len() < 3 {
                &nanos[..3]
            } else {
                trimmed
            };
            format!("{}.{fraction}Z", utc.format("%Y-%m-%dT%H:%M:%S"))
        }
        "date_time_no_millis" | "strict_date_time_no_millis" => {
            utc.format("%Y-%m-%dT%H:%M:%SZ").to_string()
        }
        "strict_date" | "date" => utc.format("%Y-%m-%d").to_string(),
        "basic_date" => utc.format("%Y%m%d").to_string(),
        "ISO8601"
        | "strict_date_optional_time"
        | "date_optional_time"
        | "date_time"
        | "strict_date_time" => utc.format(ISO_OUT).to_string(),
        // A Java pattern, or `TAI64N`, which the parse side declines too.
        _ => return None,
    })
}

/// Whether the TEXT carried a zone of its own, which beats the processor's.
fn offset_in_text(input: &str) -> bool {
    // Only past the date, so `2023-10-16` is not read as carrying one. `get`
    // rather than a slice, because byte 10 lands mid-character on a text whose
    // date is spelled with anything multi-byte and the slice panics there.
    let Some(past_date) = input.get(10..).filter(|rest| !rest.is_empty()) else {
        return false;
    };
    input.ends_with('Z') || past_date.contains('+') || past_date.rfind('-').is_some_and(|at| at > 2)
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
        // Two more of Elasticsearch's own names, and DATE-ONLY: `strict_date`
        // is `yyyy-MM-dd` and `basic_date` the same without separators, both
        // resolved to a `LocalDate` and stored at midnight UTC. Unnamed they
        // reach the Java translator, which reads every letter of `strict_date`
        // as a pattern letter and matches nothing -- which cost sysdig's
        // vulnerability stream its disclosure and solution dates and every
        // processor behind them.
        "strict_date" | "date" => parse_java(input, "yyyy-MM-dd", timezone),
        "basic_date" => parse_java(input, "yyyyMMdd", timezone),
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
    // Almost every vendor pattern carries no optional section at all, and
    // building a one-element `Vec` for it is an allocation per format per
    // event.
    if !java.contains('[') {
        return parse_java_exact(input, java, timezone);
    }
    let sections = optional_sections(java);
    if sections.is_empty() {
        return parse_java_exact(input, java, timezone);
    }
    let readings = 1u32 << sections.len().min(READING_BITS);
    for mask in 0..readings {
        let candidate = reading(java, &sections, mask);
        if let Some(parsed) = parse_java_exact(input, &candidate, timezone) {
            return Some(parsed);
        }
    }
    // The readings are capped, and the ALL-ABSENT one is the last of them --
    // so it is exactly what a truncation loses. Java reaches it by taking no
    // optional section at all, which is the reading a pattern of nine
    // fractional-second alternatives needs for a text carrying none.
    if sections.len() > READING_BITS {
        return parse_java_exact(input, &reading(java, &sections, u32::MAX), timezone);
    }
    None
}

/// How many optional sections are expanded exhaustively.
///
/// Ten is 1,024 readings, above the nine that `amazon_security_lake`'s
/// fractional-second pattern spells and far above every other pattern in the
/// tree. The bound is on the WORK a text that matches nothing can ask for.
const READING_BITS: usize = 10;

/// The top-level `[...]` sections of a pattern, as byte ranges.
///
/// A nested bracket stays inside its parent's body and reaches the translator
/// verbatim, which is how this has always read them.
fn optional_sections(java: &str) -> Vec<(usize, usize)> {
    let mut sections = Vec::new();
    let mut depth = 0usize;
    let mut open = 0usize;
    for (index, c) in java.char_indices() {
        match c {
            '[' => {
                if depth == 0 {
                    open = index;
                }
                depth += 1;
            }
            ']' if depth > 0 => {
                depth -= 1;
                if depth == 0 {
                    sections.push((open, index));
                }
            }
            _ => {}
        }
    }
    sections
}

/// One reading of a pattern, with each optional section kept or dropped.
///
/// Bit `n - 1 - k` of `mask` drops section `k`, so counting up from zero walks
/// the readings fullest-first: Java takes an optional section when it can, and
/// `[ yyyy]` present must be tried before `[ yyyy]` absent or a date carrying a
/// year parses without it and the year is silently replaced by this one.
///
/// Built one at a time rather than collected. A nine-section pattern has 512
/// readings, and materialising them allocated dozens of strings for a date that
/// matches on the first.
///
/// A section past [`READING_BITS`] is kept, because the counting has no bit
/// left for it; `u32::MAX` still drops every one, which is the all-absent
/// reading the caller falls back to.
fn reading(java: &str, sections: &[(usize, usize)], mask: u32) -> String {
    let counted = sections.len().min(READING_BITS);
    let mut out = String::with_capacity(java.len());
    let mut at = 0usize;
    for (index, (open, close)) in sections.iter().enumerate() {
        out.push_str(&java[at..*open]);
        let drop = match counted.checked_sub(index + 1) {
            Some(shift) => mask >> shift & 1 == 1,
            None => mask == u32::MAX,
        };
        if !drop {
            out.push_str(&java[*open + 1..*close]);
        }
        at = *close + 1;
    }
    out.push_str(&java[at..]);
    out
}

/// The DATE alone, from a pattern whose time Java refuses to resolve.
///
/// chrono cannot hold a fraction without a second either, so `NaiveDateTime`
/// is no use here: the fields go into a `Parsed` and only the date comes back
/// out of it. The trailing zone is split off by hand for the same reason
/// [`parse_naive`] does it -- chrono's `%Z` consumes far more than a name.
fn parse_date_only(
    input: &str,
    chrono: &str,
    timezone: Option<&str>,
) -> Option<DateTime<FixedOffset>> {
    use chrono::format::{Parsed, StrftimeItems, parse};

    let mut named = None;
    let (text, pattern) = match chrono.strip_suffix("%Z") {
        Some(stem) => {
            let trimmed = input.trim_end();
            // Counted from the END: the character before the abbreviation may
            // be multi-byte, and one past its first byte is not a boundary.
            let start = trimmed.len()
                - trimmed
                    .chars()
                    .rev()
                    .take_while(char::is_ascii_alphabetic)
                    .count();
            let (head, zone) = trimmed.split_at(start);
            if zone.is_empty() {
                return None;
            }
            named = Some(zone);
            (head.trim_end(), stem.trim_end())
        }
        None => (input, chrono),
    };

    let mut parsed = Parsed::new();
    parse(&mut parsed, text, StrftimeItems::new(pattern)).ok()?;
    let naive = parsed.to_naive_date().ok()?.and_time(NaiveTime::MIN);

    parsed
        .to_fixed_offset()
        .ok()
        .map(ProcessorZone::Fixed)
        .or_else(|| named.and_then(zone_offset).map(ProcessorZone::Fixed))
        .or_else(|| timezone.and_then(resolve_zone))
        .unwrap_or(ProcessorZone::Fixed(UTC_OFFSET))
        .read_local(&naive)
}

/// Whether a chrono pattern names a field under either of its two spellings.
///
/// chrono writes the zero-padded and the unpadded directive differently, and a
/// Java run of one letter translates to the second of them.
fn names_both(chrono: &str, padded: &str, bare: &str) -> bool {
    chrono.contains(padded) || chrono.contains(bare)
}

/// Which of hour, minute, second and fraction a chrono pattern names.
///
/// Walked rather than searched for substrings: the fraction is spelled `%.f`
/// and `%1f` through `%9f`, and a directive's modifiers sit between the `%`
/// and the letter that names the field.
fn time_parts(chrono: &str) -> (bool, bool, bool, bool) {
    let (mut hour, mut minute, mut second, mut fraction) = (false, false, false, false);
    let mut chars = chrono.chars();
    while let Some(c) = chars.next() {
        if c != '%' {
            continue;
        }
        let mut letter = chars.next();
        while letter.is_some_and(|c| matches!(c, '-' | '.' | ':' | '#' | '0'..='9')) {
            letter = chars.next();
        }
        match letter {
            Some('H' | 'I' | 'k' | 'l') => hour = true,
            Some('M') => minute = true,
            Some('S') => second = true,
            Some('f') => fraction = true,
            _ => {}
        }
    }
    (hour, minute, second, fraction)
}

/// A pattern spelling Java's week-based year `Y`, read the way Java reads it.
///
/// `Y` is `WeekFields.weekBasedYear()`, which is NOT `ChronoField.YEAR` -- so a
/// month and a day beside it have no year to build a date with and Java
/// resolves the date from the week fields alone. With no week and no day of
/// the week in the text that is the first day of the year's first week.
///
/// `symantec_endpoint`'s pipeline spells `YYYY-dd-MM HH:mm:ss` where it means
/// `yyyy-MM-dd HH:mm:ss`, so `2020-01-16 08:00:31` arrives as week-based year
/// 2020, day-of-month 1 and month 16. Elasticsearch reads it faithfully and
/// writes `2019-12-29T08:00:31.000Z`; the vendor's spelling is the bug, and
/// matching what the other engine does with it is the parity. Failing the
/// parse instead raised out of the date processor and cost the whole event.
///
/// Returns `None` for any pattern letter this does not read, and for one that
/// also names a real year -- both fall back to the ordinary reading.
fn parse_week_based_year(
    input: &str,
    java: &str,
    timezone: Option<&str>,
) -> Option<DateTime<FixedOffset>> {
    let mut rest = input;
    let mut week_year: Option<i32> = None;
    let (mut hour, mut minute, mut second, mut nanos) = (0u32, 0u32, 0u32, 0u32);
    let mut chars = java.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\'' {
            // Java quotes a literal run, and doubles the quote for one of its
            // own. An empty pair `''` is the quote character itself.
            if chars.peek() == Some(&'\'') {
                chars.next();
                rest = rest.strip_prefix('\'')?;
                continue;
            }
            for quoted in chars.by_ref() {
                if quoted == '\'' {
                    break;
                }
                rest = rest.strip_prefix(quoted)?;
            }
            continue;
        }
        if !c.is_ascii_alphabetic() {
            rest = rest.strip_prefix(c)?;
            continue;
        }
        let mut run = 1usize;
        while chars.peek() == Some(&c) {
            chars.next();
            run += 1;
        }
        let (value, tail) = take_digits(rest, run)?;
        rest = tail;
        match c {
            'Y' => week_year = Some(i32::try_from(value).ok()?),
            // A field whose digits are read only to consume them: without a
            // year Java builds no date out of them.
            'M' | 'L' | 'd' | 'D' | 'w' | 'W' | 'F' => {}
            'H' | 'k' => hour = value,
            'm' => minute = value,
            's' => second = value,
            'S' => nanos = value * 10u32.pow(u32::try_from(9 - run.min(9)).ok()?),
            // A real year, a zone, a half of the day, a name -- each of them
            // changes the reading, so the pattern goes back to the ordinary path.
            _ => return None,
        }
    }
    if !rest.is_empty() {
        return None;
    }

    let naive =
        first_day_of_week_year(week_year?)?.and_hms_nano_opt(hour, minute, second, nanos)?;
    timezone
        .and_then(resolve_zone)
        .unwrap_or(ProcessorZone::Fixed(UTC_OFFSET))
        .read_local(&naive)
}

/// Digits off the front of `text`: exactly `width` of them, or the whole run
/// where the pattern letter stood alone and Java parses greedily.
fn take_digits(text: &str, width: usize) -> Option<(u32, &str)> {
    let digits = text
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(text.len())
        .min(if width > 1 { width } else { text.len() });
    if digits == 0 || (width > 1 && digits < width) {
        return None;
    }
    Some((text[..digits].parse().ok()?, &text[digits..]))
}

/// The first day of the first week of a week-based year.
///
/// `WeekFields.of(Locale.ROOT)` starts its week on SUNDAY and needs one day in
/// the first week, so week one is the week holding January 1 and its first day
/// is the Sunday on or before it.
fn first_day_of_week_year(year: i32) -> Option<NaiveDate> {
    let first = NaiveDate::from_ymd_opt(year, 1, 1)?;
    first.checked_sub_days(chrono::Days::new(u64::from(
        first.weekday().num_days_from_sunday(),
    )))
}

/// Parse against one fully-resolved pattern, zoned first, then local.
fn parse_java_exact(
    input: &str,
    java: &str,
    timezone: Option<&str>,
) -> Option<DateTime<FixedOffset>> {
    // A WEEK-BASED year names no year, so the month and day beside it build no
    // date and Java resolves one from the week fields alone. Declines to the
    // ordinary reading below for a pattern it cannot walk.
    if java.contains('Y')
        && let Some(parsed) = parse_week_based_year(input, java, timezone)
    {
        return Some(parsed);
    }
    // `h` with no `a` names no half of the day, so Java resolves no hour and
    // drops the whole time group, keeping the date and the offset. The hour is
    // read as 24-hour so the text is still consumed, and the time discarded.
    let chrono = java_to_chrono(java);
    if chrono.contains("%I") && !chrono.contains("%p") {
        let readable = Cow::Owned(chrono.replace("%I", "%H"));
        return parse_java_exact_chrono(input, readable, timezone)?
            .with_time(NaiveTime::MIN)
            .single();
    }
    // Java's `resolveTimeLenient` will not invent a SECOND between a minute it
    // was given and a fraction it was given, nor a minute between an hour and a
    // second, so it resolves no time at all and the date processor stores the
    // date at midnight. f5_bigip's `yyyy-MM-dd:HH:mm.SSSz` is exactly that:
    // Elasticsearch reads `2019-01-01:01:01.000Z` as `2019-01-01T00:00:00.000Z`.
    //
    // Only a pattern MISSING one of the two can have such a gap, and a
    // two-byte search answers that far more cheaply than the walk does -- which
    // matters because this runs per format per event.
    if !names_both(&chrono, "%M", "%-M") || !names_both(&chrono, "%S", "%-S") {
        let (hour, minute, second, fraction) = time_parts(&chrono);
        let no_minute_below_one = !minute && (second || fraction);
        let no_second_below_a_fraction = minute && !second && fraction;
        if hour && (no_minute_below_one || no_second_below_a_fraction) {
            return parse_date_only(input, &chrono, timezone);
        }
    }
    parse_java_exact_chrono(input, chrono, timezone)
}

/// [`parse_java_exact`] once its pattern is settled, so the clock-hour case can
/// rewrite the pattern and still reach the whole parse ladder.
fn parse_java_exact_chrono(
    input: &str,
    mut chrono: Cow<'_, str>,
    timezone: Option<&str>,
) -> Option<DateTime<FixedOffset>> {
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

    // Java resolves a partial date against `Instant.EPOCH` with the ingesting
    // node's year written over it, so a missing year is the current one and a
    // missing month and day are January 1 rather than today's date. The text is
    // prefixed rather than the parse failing.
    let (has_year, has_month_or_day) = date_parts(&chrono);
    let input = match (has_year, has_month_or_day) {
        // Complete, or a year with no month and day -- which no vendor pattern
        // spells, so it is left to parse as it stands.
        (true, _) => input,
        // A BSD syslog date: the month and day are in the text, the year is not.
        (false, true) => {
            chrono = Cow::Owned(format!("%Y {chrono}"));
            Cow::Owned(format!("{} {input}", Utc::now().year()))
        }
        // No date at all, which is sentinel_one's `HH:mm:ss.SSS`.
        (false, false) => {
            chrono = Cow::Owned(format!("%Y-%m-%d {chrono}"));
            Cow::Owned(format!("{}-01-01 {input}", Utc::now().year()))
        }
    };

    if let Ok(dt) = DateTime::parse_from_str(&input, &chrono) {
        return Some(dt);
    }

    // Java's `X` spells a zero offset as the single letter `Z`, and chrono's
    // `%z` family takes only the numeric forms.
    if let Some(dt) = parse_zulu_offset(&input, &chrono) {
        return Some(dt);
    }

    // Java's `z` takes a zone NAME or a numeric OFFSET, and chrono's `%Z` takes
    // only the name.
    if let Some(dt) = parse_offset_for_zone_name(&input, &chrono) {
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

/// The same text read with Java's `Z` spelling of a zero offset.
///
/// `OffsetIdPrinterParser` prints and parses UTC as the single letter `Z` for
/// every width of `X`, and chrono's `%z`, `%:z` and `%#z` all demand digits --
/// so an ISO-8601 date ending in `Z` matched no pattern that spells its zone
/// with `X`. gitlab's production stream reads `yyyy-MM-dd'T'HH:mm:ss.SSSX`
/// against `2024-04-03T21:02:19.168Z` and threw on all 18 of its events,
/// aborting before the eleven processors that build the ECS fields.
///
/// Only a TRAILING offset directive is rewritten, and only against a text that
/// ends in `Z`, so this can turn a failed parse into a match and never a
/// matched one into something else.
fn parse_zulu_offset(input: &str, chrono: &str) -> Option<DateTime<FixedOffset>> {
    let head = input.strip_suffix('Z')?;
    let stem = ["%:z", "%z", "%#z"]
        .into_iter()
        .find_map(|offset| chrono.strip_suffix(offset))?;
    [("%:z", "+00:00"), ("%z", "+0000"), ("%#z", "+00")]
        .iter()
        .find_map(|(offset, zero)| {
            DateTime::parse_from_str(&format!("{head}{zero}"), &format!("{stem}{offset}")).ok()
        })
}

/// Whether a chrono pattern names a year, and whether it names a month or day.
///
/// `%j` is the day of the year, which resolves both at once.
fn date_parts(chrono: &str) -> (bool, bool) {
    const MONTH_OR_DAY: [&str; 7] = ["%m", "%-m", "%b", "%B", "%d", "%-d", "%j"];

    let year = chrono.contains("%Y") || chrono.contains("%y");
    let month_or_day = MONTH_OR_DAY.iter().any(|part| chrono.contains(part));
    (year, month_or_day)
}

/// The same text read with the pattern's trailing `%Z` taken as an offset.
///
/// `ZoneTextPrinterParser` falls back to a zone id, so `+00:00`, `+0000` and
/// `Z` all satisfy a Java `z`. Only the TRAILING `%Z` is rewritten, because a
/// zone in the middle of a pattern is followed by text that decides where it
/// ends.
fn parse_offset_for_zone_name(input: &str, chrono: &str) -> Option<DateTime<FixedOffset>> {
    let stem = chrono.strip_suffix("%Z")?;
    ["%:z", "%z", "%#z"]
        .iter()
        .find_map(|offset| DateTime::parse_from_str(input, &format!("{stem}{offset}")).ok())
}

/// Where a trailing `UTC+01:00` zone id begins, for the texts that carry one.
///
/// `ZoneIdPrinterParser` reads `UTC`, `GMT` and `UT` followed by an offset as a
/// zone, and a Java `z` falls back to it -- so `eset_protect`'s
/// `2/20/25, 4:27:59 PM UTC+01:00` names a zone the trailing-alphabetic split
/// cannot see, because the offset digits come after the letters.
fn prefixed_zone_start(text: &str) -> Option<usize> {
    ["UTC", "GMT", "UT"].into_iter().find_map(|prefix| {
        let at = text.rfind(prefix)?;
        let rest = &text[at + prefix.len()..];
        (rest.starts_with(['+', '-']) && zone_offset(rest).is_some()).then_some(at)
    })
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
        // Counted from the END rather than found from the front: the character
        // before the abbreviation may be multi-byte -- an accented letter, a
        // no-break space -- and one past its FIRST byte is not a character
        // boundary, which is a panic on the per-event path.
        let zone_start = prefixed_zone_start(trimmed).unwrap_or_else(|| {
            trimmed.len()
                - trimmed
                    .chars()
                    .rev()
                    .take_while(char::is_ascii_alphabetic)
                    .count()
        });
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
        // `ZoneId.of` takes every zone tzdb ships, and CET, EET, MET and WET
        // are zones in their own right rather than `SHORT_IDS` aliases -- so
        // `ZoneId.of("CET")` succeeds where reading it as unknown failed the
        // date and ran sophos's `on_failure`. Each carries summer-time rules,
        // so it stays a zone until an instant picks the offset.
        return trimmed.parse::<Tz>().ok().map(ProcessorZone::Named);
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
                // A quoted `%` is text, so it is doubled the same way a bare
                // one is -- passing it through made chrono read the character
                // after it as a directive.
                if lit == '%' {
                    out.push('%');
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
        // chrono has only `%3f`, `%6f` and `%9f`, so a run of any other width
        // with no dot ahead of it emits a directive chrono rejects and the
        // whole format declines; no vendor pattern in the tree spells one, and
        // it is what declines Java's `ISO_INSTANT` constant.
        ('S', n) => return format!("%{}f", n.min(9)),
        ('n' | 'N', _) => "%9f",
        ('a', _) => "%p",
        ('X', 1) | ('Z', 1..=3) => "%z",
        ('X' | 'x' | 'Z', _) => "%:z",
        // `v` is the GENERIC zone name, printed and parsed by the same
        // `ZoneTextPrinterParser` as `z` and falling back to a zone id the same
        // way -- so `+00:00` satisfies it, which is what tanium's
        // `yyyy-MM-dd' 'HH:mm:ss' 'v` reads.
        ('z' | 'V' | 'v', _) => "%Z",
        ('G', _) => "AD",
        _ => "",
    }
    .to_string()
}

#[cfg(test)]
// A format string is quoted verbatim from the pipeline that spells it.
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
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

    /// Verbatim from `sentinel_one_cloud_funnel`, whose events carry a
    /// time-only `timestamp`. Java resolves the missing date against
    /// `Instant.EPOCH` with the current year written over it, so a capture
    /// taken in August 2026 is stored as `2026-01-01T18:32:29.495Z`; this
    /// returned None instead and the date processor's `on_failure` appended an
    /// `error.message` Elasticsearch does not have.
    #[test]
    fn a_time_with_no_date_parses_at_january_first() {
        // The year is the ingesting node's, so the expectation is derived
        // rather than written down -- a literal would pass only this year.
        let expected = format!("{}-01-01T18:32:29.495Z", Utc::now().year());
        assert_eq!(
            parse_date_out("18:32:29.495", &["HH:mm:ss.SSS"], None, None).as_deref(),
            Some(expected.as_str())
        );

        // splunk_alert and splunk_search spell the same form without the
        // fraction.
        let expected = format!("{}-01-01T09:30:00.000Z", Utc::now().year());
        assert_eq!(
            parse_date_out("09:30:00", &["HH:mm:ss"], None, None).as_deref(),
            Some(expected.as_str())
        );

        // The zone still applies, and a value that does not match the pattern
        // still declines.
        let expected = format!("{}-01-01T09:30:00.000+11:00", Utc::now().year());
        assert_eq!(
            parse_date_out("09:30:00", &["HH:mm:ss"], Some("Australia/Sydney"), None).as_deref(),
            Some(expected.as_str())
        );
        assert!(parse_date_out("not a time", &["HH:mm:ss"], None, None).is_none());
    }

    /// `zoom_webhook` names Java's `ISO_INSTANT` constant as a date format, and
    /// DECLINING it is correct. Elasticsearch has no such named format, so it
    /// reads the word as a Java pattern, rejects `I` as an unknown pattern
    /// letter and leaves the field unset under the processor's
    /// `ignore_failure`. Parsing it wrote an `@timestamp` Elasticsearch does
    /// not have and cost zoom an event, 97/100 to 96/100.
    #[test]
    fn a_java_formatter_constant_is_not_an_elasticsearch_format() {
        assert!(parse_date_out("2024-04-03T21:02:19.168Z", &["ISO_INSTANT"], None, None).is_none());
    }

    /// Verbatim from `box_events`, whose three file dates all spell `hh` with
    /// no `a`. Java resolves no hour-of-day from a clock-hour that names no half
    /// of the day, so it keeps the date and the offset and drops the time:
    /// Elasticsearch stored `2022-05-30T04:12:12-07:00` as
    /// `2022-05-30T07:00:00.000Z`, which is midnight at -07:00.
    #[test]
    fn a_clock_hour_with_no_meridiem_loses_its_time() {
        const FORMAT: [&str; 1] = ["yyyy-MM-dd'T'hh:mm:ssXXX"];

        assert_eq!(
            parse_date_out("2022-05-30T04:12:12-07:00", &FORMAT, None, None).as_deref(),
            Some("2022-05-30T07:00:00.000Z")
        );
        assert_eq!(
            parse_date_out("2022-07-19T07:18:04-07:00", &FORMAT, None, None).as_deref(),
            Some("2022-07-19T07:00:00.000Z")
        );

        // An `a` in the pattern resolves the half of the day, so the time stays.
        assert_eq!(
            parse_date_out(
                "2022-05-30T04:12:12 PM-07:00",
                &["yyyy-MM-dd'T'hh:mm:ss aXXX"],
                None,
                None
            )
            .as_deref(),
            Some("2022-05-30T23:12:12.000Z")
        );
    }

    /// A BSD syslog date carries a month and day but no year, which is filled
    /// from the clock while the month and day stay the text's own.
    #[test]
    fn a_date_with_no_year_keeps_its_month_and_day() {
        let expected = format!("{}-06-11T09:30:00.000Z", Utc::now().year());
        assert_eq!(
            parse_date_out("Jun 11 09:30:00", &["MMM d HH:mm:ss"], None, None).as_deref(),
            Some(expected.as_str())
        );
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
        const PATTERN: &str = "[EEE ]MMM [ ]d[ yyyy] HH:mm:ss";
        let sections = optional_sections(PATTERN);
        assert_eq!(sections.len(), 3);
        let readings: Vec<String> = (0..8)
            .map(|mask| reading(PATTERN, &sections, mask))
            .collect();
        assert_eq!(readings.first().unwrap(), "EEE MMM  d yyyy HH:mm:ss");
        assert_eq!(readings.last().unwrap(), "MMM d HH:mm:ss");
    }

    /// Nine optional sections is 512 readings and the all-absent one is the
    /// last of them, which a cap of 64 never reached -- so a text carrying no
    /// fraction at all failed the pattern that describes it.
    /// Verbatim from `pipelines/amazon_security_lake/event/default.yml`.
    #[test]
    fn a_text_with_no_fractional_second_reaches_the_all_absent_reading() {
        const FORMAT: &str = "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X";

        assert_eq!(
            parse_date_out("2016-02-29 10:42:23-0700", &[FORMAT], None, None).as_deref(),
            Some("2016-02-29T17:42:23.000Z")
        );
        // A fraction the pattern DOES describe still reads through its own
        // section rather than being dropped with the rest.
        assert_eq!(
            parse_date_out("2016-02-29 10:42:23.125-0700", &[FORMAT], None, None).as_deref(),
            Some("2016-02-29T17:42:23.125Z")
        );
    }

    /// Java's `v` is the generic zone name and falls back to a zone id, so the
    /// offset `+00:00` satisfies it. Verbatim from `pipelines/tanium`.
    #[test]
    fn a_generic_zone_name_takes_an_offset() {
        assert_eq!(
            parse_date_out(
                "2022-11-18 10:10:57 +00:00",
                &["yyyy-MM-dd' 'HH:mm:ss' 'v"],
                None,
                None
            )
            .as_deref(),
            Some("2022-11-18T10:10:57.000Z")
        );
    }

    /// `Y` is the WEEK-BASED year, which names no year for a month and a day
    /// to build a date with -- so Java resolves the date from the week fields
    /// alone. Verbatim from `pipelines/symantec_endpoint/log/default.yml`,
    /// which spells `YYYY-dd-MM` where it means `yyyy-MM-dd`: Elasticsearch
    /// reads it faithfully and writes the first day of week-based year 2020.
    #[test]
    fn a_week_based_year_resolves_without_its_month_and_day() {
        assert_eq!(
            parse_date_out(
                "2020-01-16 08:00:31",
                &["YYYY-dd-MM HH:mm:ss"],
                Some("UTC"),
                None
            )
            .as_deref(),
            Some("2019-12-29T08:00:31.000Z")
        );
    }

    /// A Java `z` falls back to a zone ID, and `UTC+01:00` is one. Verbatim
    /// from `pipelines/eset_protect/event/default.yml`, whose gsub pads the
    /// vendor's `UTC+1` out to the full offset before the date processor runs.
    #[test]
    fn a_zone_name_carrying_an_offset_is_read_whole() {
        assert_eq!(
            parse_date_out(
                "2/20/25, 4:27:59 PM UTC+01:00",
                &["dd-MMM-yyyy HH:mm:ss", "M/d/yy, h:m:s a z"],
                None,
                None
            )
            .as_deref(),
            Some("2025-02-20T15:27:59.000Z")
        );
    }

    /// `ZoneId.of("CET")` succeeds: CET, EET, MET and WET are tzdb zones in
    /// their own right rather than `SHORT_IDS` aliases. Reading one as unknown
    /// failed the date outright and ran the processor's `on_failure`.
    #[test]
    fn a_tzdb_zone_that_is_not_a_short_id_resolves() {
        assert_eq!(
            parse_date_out(
                "2023-06-15 12:00:00",
                &["yyyy-MM-dd HH:mm:ss"],
                Some("CET"),
                None
            )
            .as_deref(),
            Some("2023-06-15T12:00:00.000+02:00")
        );
        // Still nothing for an abbreviation `ZoneId.of` throws on.
        assert_eq!(
            parse_date_out(
                "2023-06-15 12:00:00",
                &["yyyy-MM-dd HH:mm:ss"],
                Some("EDT"),
                None
            ),
            None
        );
    }

    /// Java will not invent a SECOND between a minute it was given and a
    /// fraction it was given, so it resolves no time and the date processor
    /// stores the date at midnight. Verbatim from
    /// `pipelines/f5_bigip/log/pipeline_bigipltm.yml`, and the value is the
    /// one Elasticsearch captured.
    #[test]
    fn a_fraction_with_no_seconds_drops_the_whole_time() {
        assert_eq!(
            parse_date_out(
                "2019-01-01:01:01.000Z",
                &["yyyy-MM-dd:HH:mm.SSSz"],
                None,
                None
            )
            .as_deref(),
            Some("2019-01-01T00:00:00.000Z")
        );
        // The same pattern WITH seconds resolves the time as written.
        assert_eq!(
            parse_date_out(
                "2019-01-01:01:01:02.000Z",
                &["yyyy-MM-dd:HH:mm:ss.SSSz"],
                None,
                None
            )
            .as_deref(),
            Some("2019-01-01T01:01:02.000Z")
        );
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

    /// `strict_date` and `basic_date` are Elasticsearch's names, not Java
    /// patterns. Verbatim from `pipelines/sysdig/vulnerability/default.yml`,
    /// whose disclosure and solution dates carry no time at all: read as a
    /// pattern, `strict_date` matched nothing and the removes in its
    /// `on_failure` deleted the fields the processors behind it copy.
    #[test]
    fn a_date_only_named_format_is_not_a_java_pattern() {
        assert_eq!(
            parse_date_out("1999-01-01", &["strict_date"], None, None).as_deref(),
            Some("1999-01-01T00:00:00.000Z")
        );
        assert_eq!(
            parse_date_out("20250116", &["basic_date"], None, None).as_deref(),
            Some("2025-01-16T00:00:00.000Z")
        );
        // Reserved on the OUTPUT side too, where the same translator turned
        // endace's `epoch_millis` into arithmetic on the wrong fields.
        assert_eq!(
            parse_date_out(
                "2025-01-16T04:05:06Z",
                &["ISO8601"],
                None,
                Some("strict_date")
            )
            .as_deref(),
            Some("2025-01-16")
        );
    }

    /// Java's `X` reads a zero offset as the letter `Z`, and chrono's `%z`
    /// family reads only digits. Verbatim from
    /// `pipelines/gitlab/production/default.yml`, whose every date is
    /// ISO-8601 with a `Z`: the date processor threw, the pipeline's own
    /// handler stamped `pipeline_error`, and the eleven processors behind it
    /// never ran -- all 18 of its scored events, on this one reading.
    #[test]
    fn a_zulu_offset_parses_where_the_pattern_spells_x() {
        assert_eq!(
            parse_date_out(
                "2024-04-03T21:02:19.168Z",
                &["yyyy-MM-dd'T'HH:mm:ss.SSSX"],
                None,
                None
            )
            .as_deref(),
            Some("2024-04-03T21:02:19.168Z")
        );

        // Every width of `X` prints UTC the same way, and a real offset still
        // parses through the directive itself.
        for format in ["yyyy-MM-dd'T'HH:mm:ssX", "yyyy-MM-dd'T'HH:mm:ssXXX"] {
            assert_eq!(
                parse_date_out("2024-04-03T21:02:19Z", &[format], None, None).as_deref(),
                Some("2024-04-03T21:02:19.000Z"),
                "{format}"
            );
        }
        assert_eq!(
            parse_date_out(
                "2024-04-03T21:02:19+05:30",
                &["yyyy-MM-dd'T'HH:mm:ssXXX"],
                None,
                None
            )
            .as_deref(),
            Some("2024-04-03T15:32:19.000Z")
        );

        // A `Z` the pattern does not ask for is still no match: the rewrite
        // fires only where the pattern's last directive is an offset.
        assert_eq!(
            parse_date_out(
                "2024-04-03T21:02:19Z",
                &["yyyy-MM-dd'T'HH:mm:ss"],
                None,
                None
            )
            .as_deref(),
            None
        );
    }

    /// Java's `z` reads a zone NAME or a numeric OFFSET, and chrono's `%Z`
    /// reads only the name. Verbatim from
    /// `pipelines/ti_eclecticiq/threat/default.yml`, whose every date carries
    /// `+00:00`: the first date processor took its `on_failure`, the pipeline's
    /// own handler stamped `pipeline_error`, and the removes behind it never
    /// ran -- all 27 of its scored events, on this one reading.
    #[test]
    fn a_numeric_offset_parses_where_the_pattern_spells_a_zone_name() {
        const FORMATS: [&str; 4] = [
            "yyyy-MM-dd HH:mm:ss.SSSSSSz",
            "yyyy-MM-dd HH:mm:ssz",
            "yyyy-MM-dd'T'HH:mm:ss.SSSSSSz",
            "yyyy-MM-dd'T'HH:mm:ssz",
        ];

        assert_eq!(
            parse_date_out("2023-06-20 18:06:08.725000+00:00", &FORMATS, None, None).as_deref(),
            Some("2023-06-20T18:06:08.725Z")
        );
        assert_eq!(
            parse_date_out("2023-06-20 18:06:08+00:00", &FORMATS, None, None).as_deref(),
            Some("2023-06-20T18:06:08.000Z")
        );
        assert_eq!(
            parse_date_out("2023-06-08T12:00:29.962000+05:30", &FORMATS, None, None).as_deref(),
            Some("2023-06-08T06:30:29.962Z")
        );

        // A zone NAME still resolves through the same `z`.
        assert_eq!(
            parse_date_out("2023-06-20 18:06:08 AEST", &FORMATS, None, None).as_deref(),
            Some("2023-06-20T08:06:08.000Z")
        );
        // And a `z` with nothing to read still declines, as Java's mandatory
        // zone does.
        assert!(parse_date_out("2023-06-20 18:06:08", &FORMATS, None, None).is_none());
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

    /// A reserved word as the OUTPUT format, which is not a Java pattern.
    ///
    /// endace names `epoch_millis` on four call sites, and `java_to_chrono`
    /// read every letter of it as a pattern letter: `h`, `m` and `s` survived
    /// as `%-I_%-M%-S` and `2024-07-01T10:48:39.852Z` came back as `10_4839`.
    /// The `convert` processor after it could not read that as a long, threw,
    /// and took the whole pipeline with it -- so the `_conf` scratch was never
    /// pruned and `event.reference` never built.
    #[test]
    fn an_epoch_output_format_writes_the_epoch() {
        let out = parse_date_out(
            "2024-07-01T10:48:39.852Z",
            &["ISO8601"],
            None,
            Some("epoch_millis"),
        );
        assert_eq!(out.as_deref(), Some("1719830919852"));

        // The Beats spelling of the same format, and the seconds pair.
        assert_eq!(
            parse_date_out(
                "2024-07-01T10:48:39.852Z",
                &["ISO8601"],
                None,
                Some("UNIX_MS")
            )
            .as_deref(),
            Some("1719830919852")
        );
        assert_eq!(
            parse_date_out(
                "2024-07-01T10:48:39.000Z",
                &["ISO8601"],
                None,
                Some("epoch_second")
            )
            .as_deref(),
            Some("1719830919")
        );
    }

    /// The nanosecond printer keeps between three and nine fraction digits.
    ///
    /// sysdig and `google_workspace_meet` name this one, and read as a Java
    /// pattern it resolved to `%-S_%-d%p_%9f%p_%-M_%9f%p%9f%-S`. The digit
    /// counts here are Elasticsearch's own output, captured in the corpus.
    #[test]
    fn the_nanosecond_output_format_trims_to_three_digits() {
        for (input, expected) in [
            (
                "2025-04-16T03:01:01.1951498Z",
                "2025-04-16T03:01:01.1951498Z",
            ),
            ("2025-04-15T07:13:45.52835Z", "2025-04-15T07:13:45.52835Z"),
            ("2026-05-28T11:35:32.639354Z", "2026-05-28T11:35:32.639354Z"),
            // Nothing below the millisecond leaves the minimum three.
            ("2025-04-16T03:01:01Z", "2025-04-16T03:01:01.000Z"),
        ] {
            let out = parse_date_out(
                input,
                &["ISO8601"],
                None,
                Some("strict_date_optional_time_nanos"),
            );
            assert_eq!(out.as_deref(), Some(expected), "{input}");
        }
    }

    /// A real Java pattern still goes through the translator.
    #[test]
    fn a_java_output_pattern_is_unaffected() {
        let out = parse_date_out(
            "2024-07-01T10:48:39.852Z",
            &["ISO8601"],
            None,
            Some("yyyy-MM-dd HH:mm:ss"),
        );
        assert_eq!(out.as_deref(), Some("2024-07-01 10:48:39"));
    }

    /// A multi-byte character abutting the trailing zone abbreviation used to
    /// land the split one byte into it, which panics on the per-event path.
    #[test]
    fn a_multi_byte_character_before_the_zone_does_not_split_mid_character() {
        const FORMATS: [&str; 1] = ["yyyy-MM-dd HH:mm:ss z"];

        // A no-break space is whitespace, so the date still reads and AEST
        // still shifts it.
        assert_eq!(
            parse_date_out("2023-06-20 18:06:08\u{a0}AEST", &FORMATS, None, None).as_deref(),
            Some("2023-06-20T08:06:08.000Z")
        );

        // An accented letter is not whitespace, so the parse declines -- which
        // is an answer rather than a panic.
        assert!(parse_date_out("2023-06-20 18:06:08 \u{e9}AEST", &FORMATS, None, None).is_none());
    }

    /// A multi-byte character straddling byte ten used to slice the text
    /// mid-character while deciding whether it carried an offset.
    #[test]
    fn a_multi_byte_character_at_byte_ten_does_not_slice_mid_character() {
        assert_eq!(
            parse_date_out(
                "abcdefghi\u{3a9}2024-12-15 09:30:00",
                &["'abcdefghi\u{3a9}'yyyy-MM-dd HH:mm:ss"],
                Some("+1000"),
                None
            )
            .as_deref(),
            Some("2024-12-15T09:30:00.000+10:00")
        );
    }

    /// A `%` inside a Java quoted literal is TEXT, so it is doubled the same
    /// way a bare one is rather than turning the character after it into a
    /// directive.
    #[test]
    fn a_percent_in_a_quoted_literal_stays_text() {
        assert_eq!(java_to_chrono("yyyy'%d'"), "%Y%%d");
        assert_eq!(
            parse_date_out(
                "2024-12-15%H09:30:00",
                &["yyyy-MM-dd'%H'HH:mm:ss"],
                None,
                None
            )
            .as_deref(),
            Some("2024-12-15T09:30:00.000Z")
        );
    }

    /// A pre-1970 instant floors its seconds, so the fraction has to come off
    /// the whole rather than be appended to the floor.
    #[test]
    fn a_pre_1970_epoch_second_prints_its_own_sign_and_fraction() {
        for (input, expected) in [
            ("1969-12-31T23:59:59.500Z", "-0.5"),
            ("1969-12-31T23:59:58.250Z", "-1.75"),
            ("1969-12-31T23:59:59Z", "-1"),
            // The post-epoch side is unchanged.
            ("1970-01-01T00:00:00.500Z", "0.5"),
            ("1970-01-01T00:00:01Z", "1"),
        ] {
            assert_eq!(
                parse_date_out(input, &["ISO8601"], None, Some("epoch_second")).as_deref(),
                Some(expected),
                "{input}"
            );
        }
    }

    /// Every pattern letter the translator claims, at the widths that select a
    /// different directive.
    #[test]
    fn each_pattern_letter_translates_at_its_own_width() {
        for (java, chrono) in [
            ("G", "AD"),
            ("D", "%j"),
            ("uu", "%y"),
            ("uuuu", "%Y"),
            ("L", "%-m"),
            ("LL", "%m"),
            ("LLL", "%b"),
            ("LLLL", "%B"),
            ("N", "%9f"),
            ("VV", "%Z"),
            ("x", "%:z"),
            ("xxx", "%:z"),
            ("EEE", "%a"),
            ("EEEE", "%A"),
            ("H:m:s", "%-H:%-M:%-S"),
            ("h a", "%-I %p"),
        ] {
            assert_eq!(java_to_chrono(java), chrono, "{java}");
        }
    }

    /// `ZoneId.of` takes its own `SHORT_IDS` map and nothing else, so a
    /// processor configured with an abbreviation outside it FAILS its date
    /// rather than falling back to UTC.
    #[test]
    fn a_configured_abbreviation_resolves_only_where_zone_id_of_would() {
        const FORMATS: [&str; 1] = ["yyyy-MM-dd HH:mm:ss"];

        assert!(resolve_zone("EST").is_some());
        assert!(resolve_zone("EDT").is_none());

        assert_eq!(
            parse_date_out("2023-06-20 18:06:08", &FORMATS, Some("EST"), None).as_deref(),
            Some("2023-06-20T18:06:08.000-05:00")
        );
        assert!(parse_date_out("2023-06-20 18:06:08", &FORMATS, Some("EDT"), None).is_none());
    }
}
