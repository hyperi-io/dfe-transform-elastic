// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Efficient timestamp parsers replacing %{`TIMESTAMP_ISO8601`}, %{SYSLOGTIMESTAMP},
//! and related date/time patterns.
//!
//! Uses fixed-position byte extraction instead of regex for 10-12x speedup.

use crate::error::{ParseError, ParseResult};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

/// Parse an ISO 8601 timestamp.
///
/// Format: `YYYY-MM-DDTHH:MM:SS[.fractional][Z|±HH:MM|±HHMM|±HH]`
///
/// The `T` separator may also be a space. Replaces `%{TIMESTAMP_ISO8601}`.
pub fn parse_iso8601(input: &str) -> ParseResult<'_, DateTime<Utc>> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    // Minimum: YYYY-MM-DDTHH:MM:SS = 19 chars.
    if len < 19 {
        return Err(ParseError::eof("ISO 8601 timestamp"));
    }

    // Date: YYYY-MM-DD
    let year = parse_fixed_digits(bytes, 0, 4, "year")?;
    expect_at(bytes, 4, b'-', "date separator")?;
    let month = parse_fixed_digits(bytes, 5, 2, "month")?;
    expect_at(bytes, 7, b'-', "date separator")?;
    let day = parse_fixed_digits(bytes, 8, 2, "day")?;

    // Separator T or space.
    let sep = bytes[10];
    if sep != b'T' && sep != b't' && sep != b' ' {
        return Err(ParseError::UnexpectedByte {
            pos: 10,
            expected: "T or space",
            got: sep,
        });
    }

    // Time: HH:MM:SS
    let hour = parse_fixed_digits(bytes, 11, 2, "hour")?;
    expect_at(bytes, 13, b':', "time separator")?;
    let minute = parse_fixed_digits(bytes, 14, 2, "minute")?;
    expect_at(bytes, 16, b':', "time separator")?;
    let second = parse_fixed_digits(bytes, 17, 2, "second")?;

    let mut pos = 19;

    // Optional fractional seconds.
    let mut nanos: u32 = 0;
    if pos < len && bytes[pos] == b'.' {
        pos += 1;
        let frac_start = pos;
        while pos < len && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
        let frac_len = pos - frac_start;
        if frac_len > 0 {
            let mut frac_val: u64 = 0;
            for &b in &bytes[frac_start..pos] {
                frac_val = frac_val * 10 + u64::from(b - b'0');
            }
            // Normalise to nanoseconds (9 digits).
            if frac_len <= 9 {
                for _ in 0..(9 - frac_len) {
                    frac_val *= 10;
                }
            } else {
                for _ in 0..(frac_len - 9) {
                    frac_val /= 10;
                }
            }
            nanos = frac_val as u32;
        }
    }

    // Optional timezone.
    let offset_secs = if pos < len {
        parse_tz_offset(bytes, &mut pos)?
    } else {
        0 // Assume UTC if no timezone.
    };

    let date = NaiveDate::from_ymd_opt(year as i32, month, day)
        .ok_or_else(|| ParseError::invalid("invalid date"))?;
    let time = NaiveTime::from_hms_nano_opt(hour, minute, second, nanos)
        .ok_or_else(|| ParseError::invalid("invalid time"))?;
    let naive = NaiveDateTime::new(date, time);

    let offset = FixedOffset::east_opt(offset_secs)
        .ok_or_else(|| ParseError::invalid("invalid timezone offset"))?;
    let dt = offset
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| ParseError::invalid("ambiguous or invalid datetime"))?;

    Ok((&input[pos..], dt.with_timezone(&Utc)))
}

/// Parse a syslog-style timestamp.
///
/// Format: `MMM DD HH:MM:SS` or `MMM  D HH:MM:SS` (leading space for single-digit day).
///
/// Returns a `DateTime` in UTC for the current year (syslog timestamps lack year).
/// Replaces `%{SYSLOGTIMESTAMP}`.
pub fn parse_syslog_timestamp(input: &str) -> ParseResult<'_, DateTime<Utc>> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    // Minimum: "MMM DD HH:MM:SS" = 15, "MMM  D HH:MM:SS" = 15.
    if len < 15 {
        return Err(ParseError::eof("syslog timestamp"));
    }

    // Month abbreviation (3 bytes). Matched on bytes, not a `&str` slice:
    // a multibyte first character puts byte 3 inside a codepoint, and slicing
    // there panics.
    let month = month_from_abbrev(&bytes[..3])?;

    // Space + day (may have leading space for single-digit).
    if bytes[3] != b' ' {
        return Err(ParseError::UnexpectedByte {
            pos: 3,
            expected: "space after month",
            got: bytes[3],
        });
    }

    // Digits are CHECKED, not assumed: `bytes[4] - b'0'` on a non-digit
    // underflows, and this parser reads whatever arrived on the wire.
    let day = if bytes[4] == b' ' {
        // Single-digit day with leading space: "MMM  D HH:MM:SS".
        parse_fixed_digits(bytes, 5, 1, "day")?
    } else {
        // Two-digit day: "MMM DD HH:MM:SS".
        parse_fixed_digits(bytes, 4, 2, "day")?
    };
    if !(1..=31).contains(&day) {
        return Err(ParseError::OutOfRange {
            value: day.to_string(),
            min: "1".to_string(),
            max: "31".to_string(),
        });
    }
    let time_start = 7;

    if bytes[6] != b' ' {
        return Err(ParseError::UnexpectedByte {
            pos: 6,
            expected: "space after day",
            got: bytes[6],
        });
    }

    // Time: HH:MM:SS.
    let hour = parse_fixed_digits(bytes, time_start, 2, "hour")?;
    expect_at(bytes, time_start + 2, b':', "time separator")?;
    let minute = parse_fixed_digits(bytes, time_start + 3, 2, "minute")?;
    expect_at(bytes, time_start + 5, b':', "time separator")?;
    let second = parse_fixed_digits(bytes, time_start + 6, 2, "second")?;

    let pos = time_start + 8;

    // Use current year (syslog timestamps don't include year).
    let year = Utc::now().year();

    let date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| ParseError::invalid("invalid syslog date"))?;
    let time = NaiveTime::from_hms_opt(hour, minute, second)
        .ok_or_else(|| ParseError::invalid("invalid syslog time"))?;
    let naive = NaiveDateTime::new(date, time);
    let dt = Utc.from_utc_datetime(&naive);

    Ok((&input[pos..], dt))
}

/// Parse a 4-digit year. Replaces `%{YEAR}`.
pub fn parse_year(input: &str) -> ParseResult<'_, u32> {
    let bytes = input.as_bytes();
    if bytes.len() < 4 {
        return Err(ParseError::eof("year"));
    }
    let val = parse_fixed_digits(bytes, 0, 4, "year")?;
    Ok((&input[4..], val))
}

/// Parse a 2-digit month number (01-12). Replaces `%{MONTHNUM}`.
pub fn parse_monthnum(input: &str) -> ParseResult<'_, u32> {
    let bytes = input.as_bytes();
    if bytes.len() < 2 {
        return Err(ParseError::eof("month number"));
    }
    let val = parse_fixed_digits(bytes, 0, 2, "month number")?;
    if !(1..=12).contains(&val) {
        return Err(ParseError::OutOfRange {
            value: val.to_string(),
            min: "1".into(),
            max: "12".into(),
        });
    }
    Ok((&input[2..], val))
}

/// Parse a 1-2 digit day of month (1-31). Replaces `%{MONTHDAY}`.
pub fn parse_monthday(input: &str) -> ParseResult<'_, u32> {
    let bytes = input.as_bytes();
    if bytes.is_empty() || !bytes[0].is_ascii_digit() {
        return Err(ParseError::invalid("expected digit for day"));
    }

    let mut pos = 0;
    while pos < bytes.len() && pos < 2 && bytes[pos].is_ascii_digit() {
        pos += 1;
    }

    let val = parse_fixed_digits(bytes, 0, pos, "day")?;
    if !(1..=31).contains(&val) {
        return Err(ParseError::OutOfRange {
            value: val.to_string(),
            min: "1".into(),
            max: "31".into(),
        });
    }
    Ok((&input[pos..], val))
}

/// Parse a time component `HH:MM:SS`. Replaces `%{TIME}`.
pub fn parse_time(input: &str) -> ParseResult<'_, NaiveTime> {
    let bytes = input.as_bytes();
    if bytes.len() < 8 {
        return Err(ParseError::eof("time"));
    }

    let hour = parse_fixed_digits(bytes, 0, 2, "hour")?;
    expect_at(bytes, 2, b':', "time separator")?;
    let minute = parse_fixed_digits(bytes, 3, 2, "minute")?;
    expect_at(bytes, 5, b':', "time separator")?;
    let second = parse_fixed_digits(bytes, 6, 2, "second")?;

    let time = NaiveTime::from_hms_opt(hour, minute, second)
        .ok_or_else(|| ParseError::invalid("invalid time"))?;

    Ok((&input[8..], time))
}

/// Parse an ISO 8601 timezone offset: `Z`, `±HH:MM`, `±HHMM`, or `±HH`.
/// Replaces `%{ISO8601_TIMEZONE}`.
pub fn parse_iso8601_tz(input: &str) -> ParseResult<'_, i32> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Err(ParseError::eof("timezone"));
    }
    let mut pos = 0;
    let offset = parse_tz_offset(bytes, &mut pos)?;
    Ok((&input[pos..], offset))
}

// ── Helpers ──────────────────────────────────────────────────────────

/// Parse `count` ASCII digits starting at `offset`, return as u32.
fn parse_fixed_digits(
    bytes: &[u8],
    offset: usize,
    count: usize,
    context: &'static str,
) -> Result<u32, ParseError> {
    if offset + count > bytes.len() {
        return Err(ParseError::eof(context));
    }
    let mut val: u32 = 0;
    for (i, &b) in bytes.iter().enumerate().skip(offset).take(count) {
        if !b.is_ascii_digit() {
            return Err(ParseError::UnexpectedByte {
                pos: i,
                expected: "digit",
                got: b,
            });
        }
        val = val * 10 + u32::from(b - b'0');
    }
    Ok(val)
}

/// Expect a specific byte at a specific position.
fn expect_at(
    bytes: &[u8],
    pos: usize,
    expected: u8,
    context: &'static str,
) -> Result<(), ParseError> {
    if pos >= bytes.len() {
        return Err(ParseError::eof(context));
    }
    if bytes[pos] != expected {
        return Err(ParseError::UnexpectedByte {
            pos,
            expected: context,
            got: bytes[pos],
        });
    }
    Ok(())
}

/// Parse timezone offset from current position, advancing pos.
fn parse_tz_offset(bytes: &[u8], pos: &mut usize) -> Result<i32, ParseError> {
    if *pos >= bytes.len() {
        return Ok(0); // No timezone = assume UTC.
    }

    if bytes[*pos] == b'Z' || bytes[*pos] == b'z' {
        *pos += 1;
        return Ok(0);
    }

    let sign = match bytes[*pos] {
        b'+' => 1i32,
        b'-' => -1i32,
        _ => return Ok(0), // No recognisable timezone.
    };
    *pos += 1;

    // HH
    if *pos + 2 > bytes.len() {
        return Err(ParseError::eof("timezone hours"));
    }
    let hours = parse_fixed_digits(bytes, *pos, 2, "timezone hours")? as i32;
    *pos += 2;

    // Optional ':' or HHMM.
    let minutes = if *pos < bytes.len() && bytes[*pos] == b':' {
        *pos += 1;
        if *pos + 2 > bytes.len() {
            return Err(ParseError::eof("timezone minutes"));
        }
        let m = parse_fixed_digits(bytes, *pos, 2, "timezone minutes")? as i32;
        *pos += 2;
        m
    } else if *pos + 2 <= bytes.len() && bytes[*pos].is_ascii_digit() {
        let m = parse_fixed_digits(bytes, *pos, 2, "timezone minutes")? as i32;
        *pos += 2;
        m
    } else {
        0
    };

    Ok(sign * (hours * 3600 + minutes * 60))
}

/// Convert 3-letter month abbreviation to 1-based month number.
fn month_from_abbrev(s: &[u8]) -> Result<u32, ParseError> {
    match s {
        b"Jan" => Ok(1),
        b"Feb" => Ok(2),
        b"Mar" => Ok(3),
        b"Apr" => Ok(4),
        b"May" => Ok(5),
        b"Jun" => Ok(6),
        b"Jul" => Ok(7),
        b"Aug" => Ok(8),
        b"Sep" => Ok(9),
        b"Oct" => Ok(10),
        b"Nov" => Ok(11),
        b"Dec" => Ok(12),
        _ => Err(ParseError::invalid("unknown month abbreviation")),
    }
}

use chrono::Datelike;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    // ── Unicode resilience ──────────────────────────────────────────

    /// A syslog line whose first characters are multibyte must be rejected,
    /// not panic. Slicing the month abbreviation by byte offset cuts inside a
    /// codepoint unless the bytes are checked first.
    #[test]
    fn syslog_timestamp_rejects_multibyte_input_without_panicking() {
        // Each of these is at least the 15 bytes the parser requires, with a
        // multibyte character straddling byte offset 3.
        for input in [
            "ÄÄÄ 17 12:00:00 rest",
            "日本語 17 12:00:00",
            "\u{1F600}\u{1F600} 17 12:00:00",
            "aÄb 17 12:00:00 rest",
            "\u{0130}st 17 12:00:00",
        ] {
            assert!(
                parse_syslog_timestamp(input).is_err(),
                "{input:?} must be rejected, not parsed"
            );
        }
    }

    /// Every byte prefix of a multibyte string is a potential slice point.
    #[test]
    fn syslog_timestamp_survives_every_multibyte_prefix() {
        let base = "日本語한국어Ää\u{1F600} 17 12:00:00 padding padding";
        for end in 0..base.len() {
            if base.is_char_boundary(end) {
                let _ = parse_syslog_timestamp(&base[end..]);
            }
        }
    }

    // ── ISO 8601 ────────────────────────────────────────────────────

    #[test]
    fn iso8601_basic() {
        let (rem, dt) = parse_iso8601("2024-01-15T10:30:00Z rest").unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 15);
        assert_eq!(dt.hour(), 10);
        assert_eq!(dt.minute(), 30);
        assert_eq!(rem, " rest");
    }

    #[test]
    fn iso8601_with_offset() {
        let (_, dt) = parse_iso8601("2024-06-15T10:30:00+10:00").unwrap();
        // 10:30 AEST = 00:30 UTC.
        assert_eq!(dt.hour(), 0);
        assert_eq!(dt.minute(), 30);
    }

    #[test]
    fn iso8601_with_millis() {
        let (_, dt) = parse_iso8601("2024-01-15T10:30:00.123Z").unwrap();
        assert_eq!(dt.nanosecond() / 1_000_000, 123);
    }

    #[test]
    fn iso8601_with_micros() {
        let (_, dt) = parse_iso8601("2024-01-15T10:30:00.123456Z").unwrap();
        assert_eq!(dt.nanosecond() / 1_000, 123_456);
    }

    #[test]
    fn iso8601_space_separator() {
        let (_, dt) = parse_iso8601("2024-01-15 10:30:00Z").unwrap();
        assert_eq!(dt.hour(), 10);
    }

    #[test]
    fn iso8601_negative_offset() {
        let (_, dt) = parse_iso8601("2024-01-15T10:30:00-05:00").unwrap();
        // 10:30 EST = 15:30 UTC.
        assert_eq!(dt.hour(), 15);
    }

    #[test]
    fn iso8601_no_timezone() {
        let (_, dt) = parse_iso8601("2024-01-15T10:30:00").unwrap();
        assert_eq!(dt.hour(), 10);
    }

    #[test]
    fn iso8601_rejects_short() {
        assert!(parse_iso8601("2024-01-15").is_err());
    }

    // ── Syslog timestamp ────────────────────────────────────────────

    #[test]
    fn syslog_two_digit_day() {
        let (rem, dt) = parse_syslog_timestamp("Jan 15 10:30:00 rest").unwrap();
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 15);
        assert_eq!(dt.hour(), 10);
        assert_eq!(rem, " rest");
    }

    #[test]
    fn syslog_single_digit_day() {
        let (_, dt) = parse_syslog_timestamp("Mar  5 08:00:00").unwrap();
        assert_eq!(dt.month(), 3);
        assert_eq!(dt.day(), 5);
    }

    /// The day was subtracted from `b'0'` without checking it was a digit, so
    /// a non-digit underflowed the byte. Input arrives from the wire.
    #[test]
    fn syslog_rejects_a_non_digit_day() {
        for input in [
            "Jan !! 00:00:00",
            "Jan  ! 00:00:00",
            "Jan -1 00:00:00",
            "Jan \u{00}0 00:00:00",
        ] {
            assert!(
                parse_syslog_timestamp(input).is_err(),
                "{input:?} must be rejected, not parsed"
            );
        }
    }

    #[test]
    fn syslog_rejects_a_day_out_of_range() {
        assert!(parse_syslog_timestamp("Jan 00 00:00:00").is_err());
        assert!(parse_syslog_timestamp("Jan 32 00:00:00").is_err());
        assert!(parse_syslog_timestamp("Jan 31 00:00:00").is_ok());
    }

    // ── parse_year ──────────────────────────────────────────────────

    #[test]
    fn year_basic() {
        let (rem, y) = parse_year("2024-rest").unwrap();
        assert_eq!(y, 2024);
        assert_eq!(rem, "-rest");
    }

    // ── parse_monthnum ──────────────────────────────────────────────

    #[test]
    fn monthnum_valid() {
        let (_, m) = parse_monthnum("12").unwrap();
        assert_eq!(m, 12);
    }

    #[test]
    fn monthnum_rejects_13() {
        assert!(parse_monthnum("13").is_err());
    }

    // ── parse_monthday ──────────────────────────────────────────────

    #[test]
    fn monthday_single() {
        let (rem, d) = parse_monthday("5 ").unwrap();
        assert_eq!(d, 5);
        assert_eq!(rem, " ");
    }

    #[test]
    fn monthday_double() {
        let (_, d) = parse_monthday("31").unwrap();
        assert_eq!(d, 31);
    }

    #[test]
    fn monthday_rejects_32() {
        assert!(parse_monthday("32").is_err());
    }

    // ── parse_time ──────────────────────────────────────────────────

    #[test]
    fn time_basic() {
        let (rem, t) = parse_time("14:30:00 rest").unwrap();
        assert_eq!(t.hour(), 14);
        assert_eq!(t.minute(), 30);
        assert_eq!(rem, " rest");
    }

    // ── parse_iso8601_tz ────────────────────────────────────────────

    #[test]
    fn tz_z() {
        let (_, offset) = parse_iso8601_tz("Z").unwrap();
        assert_eq!(offset, 0);
    }

    #[test]
    fn tz_positive() {
        let (_, offset) = parse_iso8601_tz("+10:00").unwrap();
        assert_eq!(offset, 36000);
    }

    #[test]
    fn tz_negative_compact() {
        let (_, offset) = parse_iso8601_tz("-0500").unwrap();
        assert_eq!(offset, -18000);
    }

    use chrono::Timelike;
}
