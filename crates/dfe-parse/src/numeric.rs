// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Efficient numeric parsers replacing %{INT}, %{NUMBER}, %{POSINT}, %{NONNEGINT}.
//!
//! All parsers are zero-copy where possible and avoid allocation.

use crate::error::{ParseError, ParseResult};

/// Parse a signed or unsigned integer, returning both the string slice and the
/// converted value.
///
/// Handles optional leading sign (`+` or `-`).
/// Replaces `%{INT}`.
pub fn parse_int(input: &str) -> ParseResult<'_, i64> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len == 0 {
        return Err(ParseError::eof("integer"));
    }

    let mut pos = 0;

    // Optional sign.
    let negative = if bytes[0] == b'-' {
        pos = 1;
        true
    } else if bytes[0] == b'+' {
        pos = 1;
        false
    } else {
        false
    };

    if pos >= len || !bytes[pos].is_ascii_digit() {
        return Err(ParseError::invalid("expected digit in integer"));
    }

    let digit_start = pos;
    while pos < len && bytes[pos].is_ascii_digit() {
        pos += 1;
    }

    // Convert digits to i64 inline (no allocation).
    let mut val: i64 = 0;
    for &b in &bytes[digit_start..pos] {
        val = val
            .checked_mul(10)
            .and_then(|v| v.checked_add((b - b'0') as i64))
            .ok_or_else(|| ParseError::invalid("integer overflow"))?;
    }

    if negative {
        val = val
            .checked_neg()
            .ok_or_else(|| ParseError::invalid("integer overflow on negation"))?;
    }

    Ok((&input[pos..], val))
}

/// Parse a non-negative integer (u64). Replaces `%{NONNEGINT}`.
pub fn parse_nonneg_int(input: &str) -> ParseResult<'_, u64> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len == 0 || !bytes[0].is_ascii_digit() {
        return Err(ParseError::invalid(
            "expected digit in non-negative integer",
        ));
    }

    let mut pos = 0;
    let mut val: u64 = 0;
    while pos < len && bytes[pos].is_ascii_digit() {
        val = val
            .checked_mul(10)
            .and_then(|v| v.checked_add((bytes[pos] - b'0') as u64))
            .ok_or_else(|| ParseError::invalid("integer overflow"))?;
        pos += 1;
    }

    Ok((&input[pos..], val))
}

/// Parse a positive integer (>= 1, u64). Replaces `%{POSINT}`.
pub fn parse_pos_int(input: &str) -> ParseResult<'_, u64> {
    let (rem, val) = parse_nonneg_int(input)?;
    if val == 0 {
        return Err(ParseError::OutOfRange {
            value: "0".into(),
            min: "1".into(),
            max: "u64::MAX".into(),
        });
    }
    Ok((rem, val))
}

/// Parse a port number (1-65535).
pub fn parse_port(input: &str) -> ParseResult<'_, u16> {
    let (rem, val) = parse_nonneg_int(input)?;
    if val == 0 || val > 65535 {
        return Err(ParseError::OutOfRange {
            value: val.to_string(),
            min: "1".into(),
            max: "65535".into(),
        });
    }
    Ok((rem, val as u16))
}

/// Parse a decimal number (integer or float, with optional exponent).
///
/// Supports: `42`, `-3.14`, `1.5e10`, `+0.001`, `.5`.
/// Replaces `%{NUMBER}` / `%{BASE10NUM}`.
pub fn parse_number(input: &str) -> ParseResult<'_, f64> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len == 0 {
        return Err(ParseError::eof("number"));
    }

    let mut pos = 0;

    // Optional sign.
    if pos < len && (bytes[pos] == b'-' || bytes[pos] == b'+') {
        pos += 1;
    }

    let digit_start = pos;

    // Integer part.
    while pos < len && bytes[pos].is_ascii_digit() {
        pos += 1;
    }

    let has_integer = pos > digit_start;

    // Fractional part.
    if pos < len && bytes[pos] == b'.' {
        pos += 1;
        let frac_start = pos;
        while pos < len && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
        if !has_integer && pos == frac_start {
            return Err(ParseError::invalid("no digits in number"));
        }
    } else if !has_integer {
        return Err(ParseError::invalid("no digits in number"));
    }

    // Exponent.
    if pos < len && (bytes[pos] == b'e' || bytes[pos] == b'E') {
        pos += 1;
        if pos < len && (bytes[pos] == b'-' || bytes[pos] == b'+') {
            pos += 1;
        }
        let exp_start = pos;
        while pos < len && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
        if pos == exp_start {
            return Err(ParseError::invalid("expected digits in exponent"));
        }
    }

    // Parse the float from the matched slice.
    let num_str = &input[..pos];
    let val: f64 = num_str
        .parse()
        .map_err(|_| ParseError::invalid("invalid number"))?;

    Ok((&input[pos..], val))
}

/// Parse an integer as a string slice (zero-copy). Replaces `%{INT}` when you
/// only need the text, not the numeric value.
pub fn take_int(input: &str) -> ParseResult<'_, &str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len == 0 {
        return Err(ParseError::eof("integer"));
    }

    let mut pos = 0;

    // Optional sign.
    if bytes[0] == b'-' || bytes[0] == b'+' {
        pos = 1;
    }

    if pos >= len || !bytes[pos].is_ascii_digit() {
        return Err(ParseError::invalid("expected digit in integer"));
    }

    while pos < len && bytes[pos].is_ascii_digit() {
        pos += 1;
    }

    Ok((&input[pos..], &input[..pos]))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── parse_int ───────────────────────────────────────────────────

    #[test]
    fn int_positive() {
        let (rem, val) = parse_int("42 rest").unwrap();
        assert_eq!(val, 42);
        assert_eq!(rem, " rest");
    }

    #[test]
    fn int_negative() {
        let (_, val) = parse_int("-100").unwrap();
        assert_eq!(val, -100);
    }

    #[test]
    fn int_with_plus() {
        let (_, val) = parse_int("+7").unwrap();
        assert_eq!(val, 7);
    }

    #[test]
    fn int_zero() {
        let (_, val) = parse_int("0").unwrap();
        assert_eq!(val, 0);
    }

    #[test]
    fn int_rejects_empty() {
        assert!(parse_int("").is_err());
    }

    #[test]
    fn int_rejects_no_digits() {
        assert!(parse_int("abc").is_err());
    }

    // ── parse_nonneg_int ────────────────────────────────────────────

    #[test]
    fn nonneg_basic() {
        let (_, val) = parse_nonneg_int("123").unwrap();
        assert_eq!(val, 123);
    }

    #[test]
    fn nonneg_zero() {
        let (_, val) = parse_nonneg_int("0").unwrap();
        assert_eq!(val, 0);
    }

    // ── parse_pos_int ───────────────────────────────────────────────

    #[test]
    fn pos_int_basic() {
        let (_, val) = parse_pos_int("42").unwrap();
        assert_eq!(val, 42);
    }

    #[test]
    fn pos_int_rejects_zero() {
        assert!(parse_pos_int("0").is_err());
    }

    // ── parse_port ──────────────────────────────────────────────────

    #[test]
    fn port_valid() {
        let (rem, val) = parse_port("8080/tcp").unwrap();
        assert_eq!(val, 8080);
        assert_eq!(rem, "/tcp");
    }

    #[test]
    fn port_max() {
        let (_, val) = parse_port("65535").unwrap();
        assert_eq!(val, 65535);
    }

    #[test]
    fn port_rejects_zero() {
        assert!(parse_port("0").is_err());
    }

    #[test]
    fn port_rejects_over_65535() {
        assert!(parse_port("65536").is_err());
    }

    // ── parse_number ────────────────────────────────────────────────

    #[test]
    fn number_integer() {
        let (_, val) = parse_number("42").unwrap();
        assert!((val - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn number_float() {
        let (_, val) = parse_number("3.14").unwrap();
        assert!((val - 3.14).abs() < 1e-10);
    }

    #[test]
    fn number_negative_float() {
        let (_, val) = parse_number("-0.5").unwrap();
        assert!((val - -0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn number_exponent() {
        let (_, val) = parse_number("1.5e3").unwrap();
        assert!((val - 1500.0).abs() < f64::EPSILON);
    }

    #[test]
    fn number_leading_dot() {
        let (_, val) = parse_number(".5").unwrap();
        assert!((val - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn number_with_trailing() {
        let (rem, val) = parse_number("99.9 foo").unwrap();
        assert!((val - 99.9).abs() < 1e-10);
        assert_eq!(rem, " foo");
    }

    // ── take_int ────────────────────────────────────────────────────

    #[test]
    fn take_int_basic() {
        let (rem, s) = take_int("-42abc").unwrap();
        assert_eq!(s, "-42");
        assert_eq!(rem, "abc");
    }
}
