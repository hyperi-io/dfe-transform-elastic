// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Efficient IP address parsers replacing %{IP}, %{IPV4}, %{IPV6}, %{IPORHOST}.
//!
//! All parsers are zero-copy: they return `&str` slices into the original input.

use crate::error::{ParseError, ParseResult};

/// Parse an IPv4 address (e.g., `192.168.1.1`).
///
/// Validates four decimal octets (0-255) separated by dots.
/// Rejects leading zeros (e.g., `01.02.03.04`).
pub fn parse_ipv4(input: &str) -> ParseResult<'_, &str> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut pos = 0;

    for octet_idx in 0..4u8 {
        if pos >= len {
            return Err(ParseError::eof("IPv4 octet"));
        }

        // Parse 1-3 digit octet.
        let start = pos;
        while pos < len && bytes[pos].is_ascii_digit() {
            pos += 1;
            if pos - start > 3 {
                return Err(ParseError::invalid("IPv4 octet exceeds 3 digits"));
            }
        }

        let digit_count = pos - start;
        if digit_count == 0 {
            return Err(ParseError::UnexpectedByte {
                pos,
                expected: "digit",
                got: bytes[pos],
            });
        }

        // Reject leading zeros (except bare "0").
        if digit_count > 1 && bytes[start] == b'0' {
            return Err(ParseError::invalid("leading zero in IPv4 octet"));
        }

        // Validate range 0-255 without allocation.
        let val = fast_parse_u16(bytes, start, pos);
        if val > 255 {
            return Err(ParseError::OutOfRange {
                value: val.to_string(),
                min: "0".into(),
                max: "255".into(),
            });
        }

        // Expect dot separator between octets.
        if octet_idx < 3 {
            if pos >= len || bytes[pos] != b'.' {
                return Err(ParseError::invalid("expected '.' between IPv4 octets"));
            }
            pos += 1;
        }
    }

    Ok((&input[pos..], &input[..pos]))
}

/// Parse an IPv6 address.
///
/// Supports full form (`2001:0db8::1`), `::` shorthand, and
/// mixed IPv4-mapped form (`::ffff:192.168.1.1`).
pub fn parse_ipv6(input: &str) -> ParseResult<'_, &str> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut pos = 0;
    let mut groups: u8 = 0;
    let mut saw_double_colon = false;

    // Handle leading `::`.
    if len >= 2 && bytes[0] == b':' && bytes[1] == b':' {
        saw_double_colon = true;
        pos = 2;
        // `::` alone is valid (all zeros).
        if pos >= len || !is_hex(bytes[pos]) {
            return Ok((&input[pos..], &input[..pos]));
        }
    }

    loop {
        // Try IPv4-mapped suffix (e.g., `::ffff:192.168.1.1`).
        if (groups >= 2 || saw_double_colon)
            && let Ok((rem, _)) = parse_ipv4(&input[pos..])
        {
            // The IPv4 part counts as 2 groups.
            let ipv4_len = input[pos..].len() - rem.len();
            pos += ipv4_len;
            return Ok((&input[pos..], &input[..pos]));
        }

        // Parse hex group (1-4 hex digits).
        let group_start = pos;
        while pos < len && is_hex(bytes[pos]) && (pos - group_start) < 4 {
            pos += 1;
        }
        if pos == group_start {
            // No digits found — only valid if we saw `::` and have enough groups.
            if saw_double_colon && groups > 0 {
                return Ok((&input[pos..], &input[..pos]));
            }
            return Err(ParseError::invalid("expected hex digits in IPv6 group"));
        }
        groups += 1;

        // Check for separator or end.
        if pos >= len {
            break;
        }

        if bytes[pos] == b':' {
            if pos + 1 < len && bytes[pos + 1] == b':' {
                if saw_double_colon {
                    // Only one `::` allowed.
                    break;
                }
                saw_double_colon = true;
                pos += 2;
                // `::` at the end is valid.
                if pos >= len || (!is_hex(bytes[pos]) && bytes[pos] != b':') {
                    break;
                }
            } else {
                // Single colon — next group.
                pos += 1;
                if pos >= len || (!is_hex(bytes[pos])) {
                    // Trailing colon without digits — back up.
                    pos -= 1;
                    break;
                }
            }
        } else {
            // Non-colon character — end of address.
            break;
        }

        if groups >= 8 {
            break;
        }
    }

    // Validate group count.
    if !saw_double_colon && groups != 8 {
        return Err(ParseError::invalid(
            "IPv6 address must have 8 groups or use :: shorthand",
        ));
    }
    if saw_double_colon && groups > 7 {
        return Err(ParseError::invalid("too many groups with :: shorthand"));
    }

    if pos == 0 {
        return Err(ParseError::invalid("empty IPv6 address"));
    }

    Ok((&input[pos..], &input[..pos]))
}

/// Parse either an IPv4 or IPv6 address. Tries IPv4 first.
pub fn parse_ip(input: &str) -> ParseResult<'_, &str> {
    if let Ok(r) = parse_ipv4(input) {
        return Ok(r);
    }
    parse_ipv6(input)
}

/// Parse an IP address or hostname. Tries IP first, falls back to hostname.
pub fn parse_ip_or_host(input: &str) -> ParseResult<'_, &str> {
    if let Ok(r) = parse_ip(input) {
        return Ok(r);
    }
    parse_hostname(input)
}

/// Parse an RFC 1123 hostname (e.g., `web-01.example.com`).
///
/// Labels are `[a-zA-Z0-9]` optionally followed by `[a-zA-Z0-9-]` (max 63 chars
/// per label), separated by dots. Must start with alphanumeric.
pub fn parse_hostname(input: &str) -> ParseResult<'_, &str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len == 0 {
        return Err(ParseError::eof("hostname"));
    }

    // Must start with alphanumeric.
    if !bytes[0].is_ascii_alphanumeric() {
        return Err(ParseError::UnexpectedByte {
            pos: 0,
            expected: "alphanumeric",
            got: bytes[0],
        });
    }

    let mut pos = 0;

    loop {
        // Parse one label.
        let label_start = pos;
        while pos < len && is_hostname_char(bytes[pos]) {
            pos += 1;
            if pos - label_start > 63 {
                return Err(ParseError::invalid("hostname label exceeds 63 chars"));
            }
        }

        // Label must not end with a hyphen.
        if pos > label_start && bytes[pos - 1] == b'-' {
            pos -= 1;
            // Back up past trailing hyphens.
            while pos > label_start && bytes[pos - 1] == b'-' {
                pos -= 1;
            }
        }

        if pos == label_start {
            break;
        }

        // Check for dot separator.
        if pos < len && bytes[pos] == b'.' {
            // Peek ahead: next byte must be alphanumeric for a valid label.
            if pos + 1 < len && bytes[pos + 1].is_ascii_alphanumeric() {
                pos += 1; // consume dot
            } else {
                // Trailing dot or dot followed by non-alphanumeric.
                break;
            }
        } else {
            break;
        }
    }

    if pos == 0 {
        return Err(ParseError::invalid("empty hostname"));
    }

    Ok((&input[pos..], &input[..pos]))
}

// ── Helpers ──────────────────────────────────────────────────────────

/// Parse up to 3 ASCII digits into a u16 without allocating.
#[inline(always)]
fn fast_parse_u16(bytes: &[u8], start: usize, end: usize) -> u16 {
    let mut val: u16 = 0;
    for &b in &bytes[start..end] {
        val = val * 10 + u16::from(b - b'0');
    }
    val
}

#[inline(always)]
fn is_hex(b: u8) -> bool {
    b.is_ascii_hexdigit()
}

#[inline(always)]
fn is_hostname_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-'
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    // ── IPv4 ────────────────────────────────────────────────────────

    #[test]
    fn ipv4_basic() {
        let (rem, ip) = parse_ipv4("192.168.1.1").unwrap();
        assert_eq!(ip, "192.168.1.1");
        assert_eq!(rem, "");
    }

    #[test]
    fn ipv4_with_trailing() {
        let (rem, ip) = parse_ipv4("10.0.0.1:8080").unwrap();
        assert_eq!(ip, "10.0.0.1");
        assert_eq!(rem, ":8080");
    }

    #[test]
    fn ipv4_all_zeros() {
        let (_, ip) = parse_ipv4("0.0.0.0").unwrap();
        assert_eq!(ip, "0.0.0.0");
    }

    #[test]
    fn ipv4_max() {
        let (_, ip) = parse_ipv4("255.255.255.255").unwrap();
        assert_eq!(ip, "255.255.255.255");
    }

    #[test]
    fn ipv4_rejects_leading_zero() {
        assert!(parse_ipv4("01.02.03.04").is_err());
    }

    #[test]
    fn ipv4_rejects_octet_over_255() {
        assert!(parse_ipv4("256.0.0.1").is_err());
    }

    #[test]
    fn ipv4_rejects_incomplete() {
        assert!(parse_ipv4("192.168.1").is_err());
    }

    // ── IPv6 ────────────────────────────────────────────────────────

    #[test]
    fn ipv6_full() {
        let (rem, ip) = parse_ipv6("2001:0db8:85a3:0000:0000:8a2e:0370:7334").unwrap();
        assert_eq!(ip, "2001:0db8:85a3:0000:0000:8a2e:0370:7334");
        assert_eq!(rem, "");
    }

    #[test]
    fn ipv6_shorthand() {
        let (_, ip) = parse_ipv6("::1").unwrap();
        assert_eq!(ip, "::1");
    }

    #[test]
    fn ipv6_double_colon_only() {
        let (_, ip) = parse_ipv6("::").unwrap();
        assert_eq!(ip, "::");
    }

    #[test]
    fn ipv6_with_trailing() {
        let (rem, ip) = parse_ipv6("fe80::1%eth0").unwrap();
        assert_eq!(ip, "fe80::1");
        assert_eq!(rem, "%eth0");
    }

    #[test]
    fn ipv6_v4_mapped() {
        let (rem, ip) = parse_ipv6("::ffff:192.168.1.1").unwrap();
        assert_eq!(ip, "::ffff:192.168.1.1");
        assert_eq!(rem, "");
    }

    // ── parse_ip (IPv4 | IPv6) ──────────────────────────────────────

    #[test]
    fn ip_parses_ipv4() {
        let (_, ip) = parse_ip("10.0.0.1").unwrap();
        assert_eq!(ip, "10.0.0.1");
    }

    #[test]
    fn ip_parses_ipv6() {
        let (_, ip) = parse_ip("::1").unwrap();
        assert_eq!(ip, "::1");
    }

    // ── Hostname ────────────────────────────────────────────────────

    #[test]
    fn hostname_simple() {
        let (rem, host) = parse_hostname("example.com").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(rem, "");
    }

    #[test]
    fn hostname_with_hyphen() {
        let (_, host) = parse_hostname("web-01.example.com").unwrap();
        assert_eq!(host, "web-01.example.com");
    }

    #[test]
    fn hostname_single_label() {
        let (rem, host) = parse_hostname("localhost:8080").unwrap();
        assert_eq!(host, "localhost");
        assert_eq!(rem, ":8080");
    }

    #[test]
    fn hostname_rejects_leading_hyphen() {
        assert!(parse_hostname("-example.com").is_err());
    }

    #[test]
    fn hostname_trailing_hyphen_excluded() {
        // Trailing hyphen should NOT be included in the parsed hostname.
        let (rem, host) = parse_hostname("abc- rest").unwrap();
        assert_eq!(host, "abc");
        assert_eq!(rem, "- rest");
    }

    // ── parse_ip_or_host ────────────────────────────────────────────

    #[test]
    fn ip_or_host_picks_ip() {
        let (_, val) = parse_ip_or_host("192.168.1.1").unwrap();
        assert_eq!(val, "192.168.1.1");
    }

    #[test]
    fn ip_or_host_picks_hostname() {
        let (_, val) = parse_ip_or_host("example.com").unwrap();
        assert_eq!(val, "example.com");
    }
}
