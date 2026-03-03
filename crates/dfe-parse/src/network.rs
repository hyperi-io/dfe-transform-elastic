// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Efficient network parsers replacing %{MAC}, %{URI}, %{URIPATH}, %{EMAILADDRESS},
//! %{UUID}.
//!
//! All parsers are zero-copy where possible.

use crate::error::{ParseError, ParseResult};

/// Parse a MAC address in colon, hyphen, or dot notation.
///
/// Supports:
/// - `aa:bb:cc:dd:ee:ff` (colon-separated)
/// - `aa-bb-cc-dd-ee-ff` (hyphen-separated)
/// - `aabb.ccdd.eeff` (Cisco dot notation, 3 groups of 4)
///
/// Replaces `%{MAC}`.
pub fn parse_mac(input: &str) -> ParseResult<'_, &str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len < 12 {
        return Err(ParseError::eof("MAC address"));
    }

    // Try Cisco dot notation first: aabb.ccdd.eeff (14 chars).
    if len >= 14 && is_cisco_mac(bytes) {
        return Ok((&input[14..], &input[..14]));
    }

    // Colon or hyphen notation: xx:xx:xx:xx:xx:xx or xx-xx-xx-xx-xx-xx (17 chars).
    if len >= 17 {
        let sep = bytes[2];
        if (sep == b':' || sep == b'-') && is_separated_mac(bytes, sep) {
            return Ok((&input[17..], &input[..17]));
        }
    }

    Err(ParseError::invalid("invalid MAC address format"))
}

/// Parsed URI components.
#[derive(Debug, PartialEq, Eq)]
pub struct UriParts<'a> {
    pub scheme: &'a str,
    pub authority: &'a str,
    pub path: &'a str,
    pub query: Option<&'a str>,
    pub fragment: Option<&'a str>,
}

/// Parse a URI into its components.
///
/// Format: `scheme://authority/path?query#fragment`
/// Replaces `%{URI}`.
pub fn parse_uri(input: &str) -> ParseResult<'_, UriParts<'_>> {
    // Parse scheme.
    let (rest, scheme) = parse_uri_scheme(input)?;

    // Expect "://"
    if !rest.starts_with("://") {
        return Err(ParseError::invalid("expected :// after URI scheme"));
    }
    let rest = &rest[3..];

    // Authority: up to first '/', '?', '#', or end.
    let auth_end = rest
        .find(['/', '?', '#'])
        .unwrap_or(rest.len());
    let authority = &rest[..auth_end];
    let rest = &rest[auth_end..];

    // Path: up to '?', '#', or end.
    let path_end = rest
        .find(['?', '#'])
        .unwrap_or(rest.len());
    let path = &rest[..path_end];
    let rest = &rest[path_end..];

    // Query: after '?' up to '#' or end.
    let (query, rest) = if let Some(q_rest) = rest.strip_prefix('?') {
        let q_end = q_rest.find('#').unwrap_or(q_rest.len());
        (Some(&q_rest[..q_end]), &q_rest[q_end..])
    } else {
        (None, rest)
    };

    // Fragment: after '#'.
    let (fragment, rest) = if let Some(f_rest) = rest.strip_prefix('#') {
        // Fragment extends to whitespace or end.
        let f_end = f_rest
            .find(|c: char| c.is_ascii_whitespace())
            .unwrap_or(f_rest.len());
        (Some(&f_rest[..f_end]), &f_rest[f_end..])
    } else {
        (None, rest)
    };

    let parts = UriParts {
        scheme,
        authority,
        path,
        query,
        fragment,
    };

    Ok((rest, parts))
}

/// Parse a URI scheme (e.g., `http`, `https`, `ftp`).
/// Replaces `%{URIPROTO}`.
pub fn parse_uri_scheme(input: &str) -> ParseResult<'_, &str> {
    let bytes = input.as_bytes();
    if bytes.is_empty() || !bytes[0].is_ascii_alphabetic() {
        return Err(ParseError::invalid("URI scheme must start with letter"));
    }

    let mut pos = 1;
    while pos < bytes.len()
        && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'+' || bytes[pos] == b'-')
    {
        pos += 1;
    }

    Ok((&input[pos..], &input[..pos]))
}

/// Parse an email address: `local@domain`.
/// Returns `(local_part, domain)`.
/// Replaces `%{EMAILADDRESS}`.
pub fn parse_email(input: &str) -> ParseResult<'_, (&str, &str)> {
    let bytes = input.as_bytes();

    // Find '@'.
    let at_pos = bytes
        .iter()
        .position(|&b| b == b'@')
        .ok_or_else(|| ParseError::invalid("no @ in email address"))?;

    if at_pos == 0 {
        return Err(ParseError::invalid("empty local part in email"));
    }

    // Validate local part (simplified: printable non-whitespace, not @).
    let local = &input[..at_pos];
    for &b in &bytes[..at_pos] {
        if b.is_ascii_whitespace() || b == b'@' {
            return Err(ParseError::invalid("invalid character in email local part"));
        }
    }

    // Parse domain (hostname-like chars + dots).
    let domain_start = at_pos + 1;
    let rest = &input[domain_start..];
    let domain_bytes = rest.as_bytes();

    if domain_bytes.is_empty() || !domain_bytes[0].is_ascii_alphanumeric() {
        return Err(ParseError::invalid("invalid email domain"));
    }

    let mut pos = 0;
    while pos < domain_bytes.len()
        && (domain_bytes[pos].is_ascii_alphanumeric()
            || domain_bytes[pos] == b'-'
            || domain_bytes[pos] == b'.')
    {
        pos += 1;
    }

    let domain = &rest[..pos];
    if !domain.contains('.') {
        return Err(ParseError::invalid("email domain must contain a dot"));
    }

    let consumed = domain_start + pos;
    Ok((&input[consumed..], (local, domain)))
}

/// Parse a UUID (`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`).
///
/// Fixed 8-4-4-4-12 hex pattern (36 chars). Replaces `%{UUID}`.
pub fn parse_uuid(input: &str) -> ParseResult<'_, &str> {
    let bytes = input.as_bytes();
    if bytes.len() < 36 {
        return Err(ParseError::eof("UUID"));
    }

    // Validate pattern: 8-4-4-4-12 hex with dashes at positions 8, 13, 18, 23.
    let groups = [(0, 8), (9, 13), (14, 18), (19, 23), (24, 36)];
    let dashes = [8, 13, 18, 23];

    for &dash_pos in &dashes {
        if bytes[dash_pos] != b'-' {
            return Err(ParseError::UnexpectedByte {
                pos: dash_pos,
                expected: "hyphen",
                got: bytes[dash_pos],
            });
        }
    }

    for &(start, end) in &groups {
        for (i, &b) in bytes.iter().enumerate().take(end).skip(start) {
            if !b.is_ascii_hexdigit() {
                return Err(ParseError::UnexpectedByte {
                    pos: i,
                    expected: "hex digit",
                    got: b,
                });
            }
        }
    }

    Ok((&input[36..], &input[..36]))
}

// ── Helpers ──────────────────────────────────────────────────────────

/// Check 6 groups of 2 hex digits separated by `sep`.
fn is_separated_mac(bytes: &[u8], sep: u8) -> bool {
    for g in 0..6 {
        let base = g * 3;
        if !bytes[base].is_ascii_hexdigit() || !bytes[base + 1].is_ascii_hexdigit() {
            return false;
        }
        if g < 5 && bytes[base + 2] != sep {
            return false;
        }
    }
    true
}

/// Check 3 groups of 4 hex digits separated by dots.
fn is_cisco_mac(bytes: &[u8]) -> bool {
    let positions = [0, 1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 13];
    let dots = [4, 9];

    for &p in &positions {
        if !bytes[p].is_ascii_hexdigit() {
            return false;
        }
    }
    for &d in &dots {
        if bytes[d] != b'.' {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── MAC ─────────────────────────────────────────────────────────

    #[test]
    fn mac_colon() {
        let (rem, mac) = parse_mac("aa:bb:cc:dd:ee:ff rest").unwrap();
        assert_eq!(mac, "aa:bb:cc:dd:ee:ff");
        assert_eq!(rem, " rest");
    }

    #[test]
    fn mac_hyphen() {
        let (_, mac) = parse_mac("AA-BB-CC-DD-EE-FF").unwrap();
        assert_eq!(mac, "AA-BB-CC-DD-EE-FF");
    }

    #[test]
    fn mac_cisco() {
        let (_, mac) = parse_mac("aabb.ccdd.eeff").unwrap();
        assert_eq!(mac, "aabb.ccdd.eeff");
    }

    #[test]
    fn mac_rejects_invalid() {
        assert!(parse_mac("not-a-mac").is_err());
    }

    // ── URI ─────────────────────────────────────────────────────────

    #[test]
    fn uri_full() {
        let (rem, parts) = parse_uri("https://example.com/path?q=1#frag rest").unwrap();
        assert_eq!(parts.scheme, "https");
        assert_eq!(parts.authority, "example.com");
        assert_eq!(parts.path, "/path");
        assert_eq!(parts.query, Some("q=1"));
        assert_eq!(parts.fragment, Some("frag"));
        assert_eq!(rem, " rest");
    }

    #[test]
    fn uri_no_path() {
        let (_, parts) = parse_uri("http://localhost").unwrap();
        assert_eq!(parts.scheme, "http");
        assert_eq!(parts.authority, "localhost");
        assert_eq!(parts.path, "");
    }

    #[test]
    fn uri_with_port() {
        let (_, parts) = parse_uri("http://host:8080/api").unwrap();
        assert_eq!(parts.authority, "host:8080");
        assert_eq!(parts.path, "/api");
    }

    // ── URI scheme ──────────────────────────────────────────────────

    #[test]
    fn scheme_basic() {
        let (rem, s) = parse_uri_scheme("https://").unwrap();
        assert_eq!(s, "https");
        assert_eq!(rem, "://");
    }

    // ── Email ───────────────────────────────────────────────────────

    #[test]
    fn email_basic() {
        let (rem, (local, domain)) = parse_email("user@example.com rest").unwrap();
        assert_eq!(local, "user");
        assert_eq!(domain, "example.com");
        assert_eq!(rem, " rest");
    }

    #[test]
    fn email_complex_local() {
        let (_, (local, domain)) = parse_email("user.name+tag@sub.example.com").unwrap();
        assert_eq!(local, "user.name+tag");
        assert_eq!(domain, "sub.example.com");
    }

    #[test]
    fn email_rejects_no_at() {
        assert!(parse_email("noemail").is_err());
    }

    #[test]
    fn email_rejects_no_dot_in_domain() {
        assert!(parse_email("user@localhost").is_err());
    }

    // ── UUID ────────────────────────────────────────────────────────

    #[test]
    fn uuid_basic() {
        let (rem, u) =
            parse_uuid("550e8400-e29b-41d4-a716-446655440000 rest").unwrap();
        assert_eq!(u, "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(rem, " rest");
    }

    #[test]
    fn uuid_uppercase() {
        let (_, u) = parse_uuid("550E8400-E29B-41D4-A716-446655440000").unwrap();
        assert_eq!(u, "550E8400-E29B-41D4-A716-446655440000");
    }

    #[test]
    fn uuid_rejects_short() {
        assert!(parse_uuid("550e8400-e29b-41d4").is_err());
    }

    #[test]
    fn uuid_rejects_missing_dash() {
        assert!(parse_uuid("550e8400xe29b-41d4-a716-446655440000").is_err());
    }
}
