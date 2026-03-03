// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Efficient string parsers replacing %{WORD}, %{QUOTEDSTRING}, %{GREEDYDATA},
//! %{NOTSPACE}, %{DATA}, %{LOGLEVEL}.
//!
//! All parsers are zero-copy and return `&str` slices into the original input.

use crate::error::{ParseError, ParseResult};
use memchr::memchr;

/// Consume one or more word characters (`[a-zA-Z0-9_]`).
/// Replaces `%{WORD}`.
pub fn take_word(input: &str) -> ParseResult<'_, &str> {
    let bytes = input.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() && is_word_char(bytes[pos]) {
        pos += 1;
    }
    if pos == 0 {
        return Err(ParseError::invalid("expected word character"));
    }
    Ok((&input[pos..], &input[..pos]))
}

/// Consume one or more non-whitespace characters.
/// Replaces `%{NOTSPACE}`.
pub fn take_notspace(input: &str) -> ParseResult<'_, &str> {
    let bytes = input.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    if pos == 0 {
        return Err(ParseError::invalid("expected non-whitespace"));
    }
    Ok((&input[pos..], &input[..pos]))
}

/// Return all remaining input. This is the zero-cost `%{GREEDYDATA}`.
pub fn take_greedy(input: &str) -> ParseResult<'_, &str> {
    Ok(("", input))
}

/// Non-greedy data: consume up to (but not including) the given delimiter byte.
/// Useful for `%{DATA}` when followed by a known separator.
pub fn take_until_byte(input: &str, delim: u8) -> ParseResult<'_, &str> {
    let bytes = input.as_bytes();
    match memchr(delim, bytes) {
        Some(pos) => Ok((&input[pos..], &input[..pos])),
        None => Ok(("", input)),
    }
}

/// Non-greedy data: consume up to (but not including) the given literal string.
pub fn take_until_str<'a>(input: &'a str, needle: &str) -> ParseResult<'a, &'a str> {
    match input.find(needle) {
        Some(pos) => Ok((&input[pos..], &input[..pos])),
        None => Ok(("", input)),
    }
}

/// Parse a quoted string (single or double quotes), handling backslash escapes.
///
/// Returns the content between quotes (without the quotes themselves).
/// Replaces `%{QUOTEDSTRING}`.
pub fn take_quoted(input: &str) -> ParseResult<'_, &str> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Err(ParseError::eof("quoted string"));
    }

    let quote = bytes[0];
    if quote != b'"' && quote != b'\'' {
        return Err(ParseError::UnexpectedByte {
            pos: 0,
            expected: "quote character",
            got: quote,
        });
    }

    let mut pos = 1;
    while pos < bytes.len() {
        if bytes[pos] == b'\\' && pos + 1 < bytes.len() {
            pos += 2; // skip escaped character
        } else if bytes[pos] == quote {
            // Found closing quote.
            let content = &input[1..pos];
            return Ok((&input[pos + 1..], content));
        } else {
            pos += 1;
        }
    }

    Err(ParseError::invalid("unterminated quoted string"))
}

/// Consume whitespace (spaces and tabs). At least one character required.
/// Replaces `%{SPACE}` when it must match.
pub fn expect_space(input: &str) -> ParseResult<'_, ()> {
    let bytes = input.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
        pos += 1;
    }
    if pos == 0 {
        return Err(ParseError::invalid("expected whitespace"));
    }
    Ok((&input[pos..], ()))
}

/// Skip optional whitespace (zero or more spaces/tabs).
pub fn skip_space(input: &str) -> &str {
    let bytes = input.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
        pos += 1;
    }
    &input[pos..]
}

/// Expect a specific literal byte.
pub fn expect_byte(input: &str, expected: u8) -> ParseResult<'_, ()> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Err(ParseError::eof("expected byte"));
    }
    if bytes[0] != expected {
        return Err(ParseError::UnexpectedByte {
            pos: 0,
            expected: "specific byte",
            got: bytes[0],
        });
    }
    Ok((&input[1..], ()))
}

/// Expect a specific literal string.
pub fn expect_str<'a>(input: &'a str, expected: &str) -> ParseResult<'a, ()> {
    if let Some(rest) = input.strip_prefix(expected) {
        Ok((rest, ()))
    } else {
        Err(ParseError::invalid("expected literal not found"))
    }
}

/// Parse a log level keyword (case-insensitive).
///
/// Matches: TRACE, DEBUG, INFO, WARN, WARNING, ERROR, CRITICAL, FATAL, NOTICE,
/// ALERT, EMERG, EMERGENCY, SEVERE.
/// Replaces `%{LOGLEVEL}`.
pub fn parse_loglevel(input: &str) -> ParseResult<'_, &str> {
    // Match word characters first, then check against known levels.
    let bytes = input.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() && bytes[pos].is_ascii_alphabetic() {
        pos += 1;
    }
    if pos == 0 {
        return Err(ParseError::invalid("expected log level"));
    }

    let word = &input[..pos];
    let upper = word.to_ascii_uppercase();
    match upper.as_str() {
        "TRACE" | "DEBUG" | "INFO" | "WARN" | "WARNING" | "ERROR" | "CRITICAL" | "FATAL"
        | "NOTICE" | "ALERT" | "EMERG" | "EMERGENCY" | "SEVERE" | "ERR" => {
            Ok((&input[pos..], word))
        }
        _ => Err(ParseError::invalid("unknown log level")),
    }
}

// ── Helpers ──────────────────────────────────────────────────────────

#[inline(always)]
fn is_word_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── take_word ───────────────────────────────────────────────────

    #[test]
    fn word_basic() {
        let (rem, w) = take_word("hello world").unwrap();
        assert_eq!(w, "hello");
        assert_eq!(rem, " world");
    }

    #[test]
    fn word_with_underscore() {
        let (_, w) = take_word("my_var=5").unwrap();
        assert_eq!(w, "my_var");
    }

    #[test]
    fn word_digits() {
        let (_, w) = take_word("abc123").unwrap();
        assert_eq!(w, "abc123");
    }

    #[test]
    fn word_rejects_leading_space() {
        assert!(take_word(" hello").is_err());
    }

    // ── take_notspace ───────────────────────────────────────────────

    #[test]
    fn notspace_basic() {
        let (rem, ns) = take_notspace("GET /path HTTP/1.1").unwrap();
        assert_eq!(ns, "GET");
        assert_eq!(rem, " /path HTTP/1.1");
    }

    // ── take_greedy ─────────────────────────────────────────────────

    #[test]
    fn greedy_returns_all() {
        let (rem, g) = take_greedy("everything here").unwrap();
        assert_eq!(g, "everything here");
        assert_eq!(rem, "");
    }

    // ── take_until_byte ─────────────────────────────────────────────

    #[test]
    fn until_byte_found() {
        let (rem, data) = take_until_byte("key=value", b'=').unwrap();
        assert_eq!(data, "key");
        assert_eq!(rem, "=value");
    }

    #[test]
    fn until_byte_not_found() {
        let (rem, data) = take_until_byte("nodelim", b'=').unwrap();
        assert_eq!(data, "nodelim");
        assert_eq!(rem, "");
    }

    // ── take_until_str ──────────────────────────────────────────────

    #[test]
    fn until_str_found() {
        let (rem, data) = take_until_str("hello -> world", " -> ").unwrap();
        assert_eq!(data, "hello");
        assert_eq!(rem, " -> world");
    }

    // ── take_quoted ─────────────────────────────────────────────────

    #[test]
    fn quoted_double() {
        let (rem, content) = take_quoted("\"hello world\" rest").unwrap();
        assert_eq!(content, "hello world");
        assert_eq!(rem, " rest");
    }

    #[test]
    fn quoted_single() {
        let (_, content) = take_quoted("'single'").unwrap();
        assert_eq!(content, "single");
    }

    #[test]
    fn quoted_escaped() {
        let (_, content) = take_quoted(r#""say \"hi\"""#).unwrap();
        assert_eq!(content, r#"say \"hi\""#);
    }

    #[test]
    fn quoted_unterminated() {
        assert!(take_quoted("\"unclosed").is_err());
    }

    // ── expect_space ────────────────────────────────────────────────

    #[test]
    fn space_basic() {
        let (rem, _) = expect_space("  hello").unwrap();
        assert_eq!(rem, "hello");
    }

    #[test]
    fn space_rejects_no_space() {
        assert!(expect_space("hello").is_err());
    }

    // ── expect_byte / expect_str ────────────────────────────────────

    #[test]
    fn byte_match() {
        let (rem, _) = expect_byte(":rest", b':').unwrap();
        assert_eq!(rem, "rest");
    }

    #[test]
    fn str_match() {
        let (rem, _) = expect_str(" -> dest", " -> ").unwrap();
        assert_eq!(rem, "dest");
    }

    // ── parse_loglevel ──────────────────────────────────────────────

    #[test]
    fn loglevel_upper() {
        let (rem, lvl) = parse_loglevel("ERROR rest").unwrap();
        assert_eq!(lvl, "ERROR");
        assert_eq!(rem, " rest");
    }

    #[test]
    fn loglevel_lower() {
        let (_, lvl) = parse_loglevel("info").unwrap();
        assert_eq!(lvl, "info");
    }

    #[test]
    fn loglevel_mixed() {
        let (_, lvl) = parse_loglevel("Warning").unwrap();
        assert_eq!(lvl, "Warning");
    }

    #[test]
    fn loglevel_rejects_unknown() {
        assert!(parse_loglevel("VERBOSE").is_err());
    }

    // ── skip_space ──────────────────────────────────────────────────

    #[test]
    fn skip_space_works() {
        assert_eq!(skip_space("  hello"), "hello");
        assert_eq!(skip_space("hello"), "hello");
    }
}
