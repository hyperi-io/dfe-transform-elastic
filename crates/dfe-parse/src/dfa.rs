// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Pre-compiled DFA fallback for patterns that genuinely require regex.
//!
//! Layer 3 parsers use `regex_automata` to build DFAs for patterns that are
//! too complex for Layer 1/Layer 2 (e.g., alternations, lookahead, complex
//! escapes).
//!
//! DFAs can be pre-compiled at build time and serialised, then loaded at
//! runtime with zero compilation cost via `include_bytes!()`.
//!
//! # Performance
//!
//! DFA matching is 2-5x faster than interpreted regex but slower than native
//! Layer 1 parsers. Use only for irreducibly complex patterns.

use crate::error::{ParseError, ParseResult};
use regex_automata::PatternID;
use regex_automata::meta::Regex;
use std::collections::HashMap;

/// A DFA-backed parser for complex patterns with named capture groups.
///
/// Wraps `regex_automata::meta::Regex` and extracts named capture groups
/// into a `HashMap`.
pub struct DfaParser {
    regex: Regex,
    group_names: Vec<String>,
}

impl DfaParser {
    /// Compile a regex pattern into a DFA parser.
    ///
    /// The pattern should use named capture groups: `(?P<field>...)`.
    pub fn new(pattern: &str) -> Result<Self, ParseError> {
        let regex = Regex::new(pattern)
            .map_err(|e| ParseError::invalid(format!("failed to compile regex: {e}")))?;

        let group_names: Vec<String> = regex
            .group_info()
            .all_names()
            .filter_map(|(_, _, name)| name.map(|n| n.to_string()))
            .collect();

        Ok(Self { regex, group_names })
    }

    /// Parse the input, returning captured fields as string slices.
    ///
    /// Only the matched portion of the input is consumed. Use
    /// [`parse_with_remainder`] to also get the remaining input.
    pub fn parse<'a>(&self, input: &'a str) -> Result<HashMap<String, &'a str>, ParseError> {
        let mut caps = self.regex.create_captures();
        self.regex.captures(input, &mut caps);

        if caps.is_match() {
            let mut fields = HashMap::new();
            for name in &self.group_names {
                if let Some(group_index) = self
                    .regex
                    .group_info()
                    .to_index(PatternID::ZERO, name.as_str())
                    && let Some(span) = caps.get_group(group_index)
                {
                    fields.insert(name.clone(), &input[span.start..span.end]);
                }
            }
            Ok(fields)
        } else {
            Err(ParseError::invalid("DFA pattern did not match"))
        }
    }

    /// Parse the input, returning `(remaining, fields)`.
    ///
    /// The remaining input is everything after the overall match.
    pub fn parse_with_remainder<'a>(
        &self,
        input: &'a str,
    ) -> ParseResult<'a, HashMap<String, &'a str>> {
        let mut caps = self.regex.create_captures();
        self.regex.captures(input, &mut caps);

        if let Some(overall) = caps.get_match() {
            let mut fields = HashMap::new();
            for name in &self.group_names {
                if let Some(group_index) = self
                    .regex
                    .group_info()
                    .to_index(PatternID::ZERO, name.as_str())
                    && let Some(span) = caps.get_group(group_index)
                {
                    fields.insert(name.clone(), &input[span.start..span.end]);
                }
            }
            Ok((&input[overall.end()..], fields))
        } else {
            Err(ParseError::invalid("DFA pattern did not match"))
        }
    }

    /// Check if the pattern matches the input without extracting captures.
    pub fn is_match(&self, input: &str) -> bool {
        self.regex.is_match(input)
    }

    /// Return the names of all capture groups in this parser.
    pub fn group_names(&self) -> &[String] {
        &self.group_names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_named_capture() {
        let parser = DfaParser::new(r"(?P<user>\w+)@(?P<host>[\w.]+)").unwrap();
        let fields = parser.parse("admin@example.com rest").unwrap();
        assert_eq!(fields["user"], "admin");
        assert_eq!(fields["host"], "example.com");
    }

    #[test]
    fn with_remainder() {
        let parser = DfaParser::new(r"(?P<word>\w+)").unwrap();
        let (rest, fields) = parser.parse_with_remainder("hello world").unwrap();
        assert_eq!(fields["word"], "hello");
        assert_eq!(rest, " world");
    }

    #[test]
    fn no_match_returns_error() {
        let parser = DfaParser::new(r"(?P<digits>\d+)").unwrap();
        assert!(parser.parse("no-digits-here").is_err());
    }

    #[test]
    fn is_match_check() {
        let parser = DfaParser::new(r"\d{4}-\d{2}-\d{2}").unwrap();
        assert!(parser.is_match("2024-01-15"));
        assert!(!parser.is_match("not-a-date"));
    }

    #[test]
    fn group_names_listed() {
        let parser = DfaParser::new(r"(?P<src>\S+):(?P<port>\d+)").unwrap();
        let names = parser.group_names();
        assert!(names.contains(&"src".to_string()));
        assert!(names.contains(&"port".to_string()));
    }

    #[test]
    fn complex_syslog_pattern() {
        // Simplified syslog header: <priority>timestamp hostname
        let parser = DfaParser::new(
            r"<(?P<priority>\d+)>(?P<timestamp>\w{3}\s+\d+\s\d{2}:\d{2}:\d{2})\s(?P<hostname>\S+)",
        )
        .unwrap();

        let fields = parser.parse("<134>Jan 15 10:30:00 webserver").unwrap();
        assert_eq!(fields["priority"], "134");
        assert_eq!(fields["timestamp"], "Jan 15 10:30:00");
        assert_eq!(fields["hostname"], "webserver");
    }

    #[test]
    fn optional_groups() {
        let parser = DfaParser::new(r"(?P<ip>\d+\.\d+\.\d+\.\d+)(?::(?P<port>\d+))?").unwrap();

        // With port.
        let fields = parser.parse("10.0.0.1:8080").unwrap();
        assert_eq!(fields["ip"], "10.0.0.1");
        assert_eq!(fields["port"], "8080");

        // Without port.
        let fields = parser.parse("10.0.0.1").unwrap();
        assert_eq!(fields["ip"], "10.0.0.1");
        assert!(!fields.contains_key("port"));
    }
}
