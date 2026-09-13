// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Composite parser builder for chaining Layer 1 parsers into full grok-equivalent
//! functions.
//!
//! Layer 2 parsers are assembled from a sequence of steps:
//! - **Literal**: expect a fixed string (e.g., ` -> `, `:`)
//! - **Capture**: run a Layer 1 parser and store the result under a field name
//! - **Skip**: consume characters matching a predicate (e.g., whitespace)
//!
//! The builder executes steps sequentially with no backtracking — if any step
//! fails, the entire composite parse fails. This gives predictable O(n)
//! performance for a grok-equivalent pattern expressed as literals and
//! Layer 1 parser captures.
//!
//! # Example
//!
//! ```ignore
//! // Grok: %{IPORHOST:source.ip}:%{INT:source.port:int} -> %{IPORHOST:dest.ip}:%{INT:dest.port:int}
//! let parser = CompositeParser::builder()
//!     .capture("source.ip", parse_ip_or_host_str)
//!     .literal(":")
//!     .capture("source.port", take_int_str)
//!     .literal(" -> ")
//!     .capture("dest.ip", parse_ip_or_host_str)
//!     .literal(":")
//!     .capture("dest.port", take_int_str)
//!     .build();
//!
//! let fields = parser.parse("10.0.0.1:8080 -> 192.168.1.1:443")?;
//! assert_eq!(fields["source.ip"], "10.0.0.1");
//! ```

use crate::error::ParseError;
use std::collections::HashMap;

/// A parser function that takes input and returns `(remaining, captured_str)`.
///
/// All Layer 1 parsers that return `&str` match this signature.
type ParserFn = fn(&str) -> Result<(&str, &str), ParseError>;

/// A step in the composite parser chain.
enum Step {
    /// Expect and consume a literal string.
    Literal(String),
    /// Run a parser and capture the result under the given field name.
    Capture { field: String, parser: ParserFn },
    /// Consume optional whitespace (zero or more spaces/tabs).
    SkipSpace,
}

/// A composite parser assembled from a chain of steps.
///
/// Built via [`CompositeParserBuilder`]. Executes steps in sequence with no
/// backtracking.
pub struct CompositeParser {
    steps: Vec<Step>,
}

impl CompositeParser {
    /// Start building a composite parser.
    pub fn builder() -> CompositeParserBuilder {
        CompositeParserBuilder { steps: Vec::new() }
    }

    /// Parse the input, returning a map of field names to captured string values.
    ///
    /// All captured values are `&str` slices into the original input (zero-copy).
    /// The caller can convert to typed values after parsing.
    pub fn parse<'a>(&self, input: &'a str) -> Result<HashMap<String, &'a str>, ParseError> {
        let mut remaining = input;
        let mut fields = HashMap::new();

        for step in &self.steps {
            match step {
                Step::Literal(lit) => {
                    if let Some(rest) = remaining.strip_prefix(lit.as_str()) {
                        remaining = rest;
                    } else {
                        return Err(ParseError::invalid(format!(
                            "expected literal '{}' at position {}",
                            lit,
                            input.len() - remaining.len()
                        )));
                    }
                }
                Step::Capture { field, parser } => {
                    let (rest, value) = parser(remaining)?;
                    fields.insert(field.clone(), value);
                    remaining = rest;
                }
                Step::SkipSpace => {
                    remaining = crate::string::skip_space(remaining);
                }
            }
        }

        Ok(fields)
    }

    /// Parse the input and also return the remaining unconsumed input.
    pub fn parse_with_remainder<'a>(
        &self,
        input: &'a str,
    ) -> Result<(&'a str, HashMap<String, &'a str>), ParseError> {
        let mut remaining = input;
        let mut fields = HashMap::new();

        for step in &self.steps {
            match step {
                Step::Literal(lit) => {
                    if let Some(rest) = remaining.strip_prefix(lit.as_str()) {
                        remaining = rest;
                    } else {
                        return Err(ParseError::invalid(format!(
                            "expected literal '{}' at position {}",
                            lit,
                            input.len() - remaining.len()
                        )));
                    }
                }
                Step::Capture { field, parser } => {
                    let (rest, value) = parser(remaining)?;
                    fields.insert(field.clone(), value);
                    remaining = rest;
                }
                Step::SkipSpace => {
                    remaining = crate::string::skip_space(remaining);
                }
            }
        }

        Ok((remaining, fields))
    }

    /// Number of steps in this composite parser.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Whether this composite parser has no steps.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// Builder for constructing a [`CompositeParser`].
pub struct CompositeParserBuilder {
    steps: Vec<Step>,
}

impl CompositeParserBuilder {
    /// Add a literal string step. The input must match this exactly.
    #[must_use]
    pub fn literal(mut self, s: &str) -> Self {
        self.steps.push(Step::Literal(s.to_string()));
        self
    }

    /// Add a capture step. The parser runs and the result is stored under `field`.
    #[must_use]
    pub fn capture(mut self, field: &str, parser: ParserFn) -> Self {
        self.steps.push(Step::Capture {
            field: field.to_string(),
            parser,
        });
        self
    }

    /// Add a step that skips optional whitespace.
    #[must_use]
    pub fn skip_space(mut self) -> Self {
        self.steps.push(Step::SkipSpace);
        self
    }

    /// Build the composite parser.
    pub fn build(self) -> CompositeParser {
        CompositeParser { steps: self.steps }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::ip::{parse_ip_or_host, parse_ipv4};
    use crate::numeric::take_int;
    use crate::string::take_word;

    /// Wrapper to adapt `parse_ipv4` (which returns `&str`) to our `ParserFn` signature.
    fn parse_ipv4_fn(input: &str) -> Result<(&str, &str), ParseError> {
        parse_ipv4(input)
    }

    fn parse_ip_or_host_fn(input: &str) -> Result<(&str, &str), ParseError> {
        parse_ip_or_host(input)
    }

    fn take_int_fn(input: &str) -> Result<(&str, &str), ParseError> {
        take_int(input)
    }

    fn take_word_fn(input: &str) -> Result<(&str, &str), ParseError> {
        take_word(input)
    }

    #[test]
    fn two_field_with_colon() {
        let parser = CompositeParser::builder()
            .capture("ip", parse_ipv4_fn)
            .literal(":")
            .capture("port", take_int_fn)
            .build();

        let fields = parser.parse("192.168.1.1:8080").unwrap();
        assert_eq!(fields["ip"], "192.168.1.1");
        assert_eq!(fields["port"], "8080");
    }

    #[test]
    fn connection_pattern() {
        // %{IPORHOST:src}:%{INT:sport} -> %{IPORHOST:dst}:%{INT:dport}
        let parser = CompositeParser::builder()
            .capture("source.ip", parse_ip_or_host_fn)
            .literal(":")
            .capture("source.port", take_int_fn)
            .literal(" -> ")
            .capture("dest.ip", parse_ip_or_host_fn)
            .literal(":")
            .capture("dest.port", take_int_fn)
            .build();

        let fields = parser.parse("10.0.0.1:8080 -> 192.168.1.1:443").unwrap();
        assert_eq!(fields["source.ip"], "10.0.0.1");
        assert_eq!(fields["source.port"], "8080");
        assert_eq!(fields["dest.ip"], "192.168.1.1");
        assert_eq!(fields["dest.port"], "443");
    }

    #[test]
    fn skip_space_between_fields() {
        let parser = CompositeParser::builder()
            .capture("first", take_word_fn)
            .skip_space()
            .capture("second", take_word_fn)
            .build();

        let fields = parser.parse("hello   world").unwrap();
        assert_eq!(fields["first"], "hello");
        assert_eq!(fields["second"], "world");
    }

    #[test]
    fn word_space_word() {
        let parser = CompositeParser::builder()
            .capture("first", take_word_fn)
            .literal(" ")
            .capture("second", take_word_fn)
            .build();

        let fields = parser.parse("hello world").unwrap();
        assert_eq!(fields["first"], "hello");
        assert_eq!(fields["second"], "world");
    }

    #[test]
    fn literal_mismatch_fails() {
        let parser = CompositeParser::builder()
            .capture("ip", parse_ipv4_fn)
            .literal(":")
            .capture("port", take_int_fn)
            .build();

        let result = parser.parse("192.168.1.1-8080");
        assert!(result.is_err());
    }

    #[test]
    fn parse_with_remainder() {
        let parser = CompositeParser::builder()
            .capture("ip", parse_ipv4_fn)
            .literal(":")
            .capture("port", take_int_fn)
            .build();

        let (rest, fields) = parser
            .parse_with_remainder("192.168.1.1:8080 trailing data")
            .unwrap();
        assert_eq!(fields["ip"], "192.168.1.1");
        assert_eq!(fields["port"], "8080");
        assert_eq!(rest, " trailing data");
    }

    #[test]
    fn empty_parser() {
        let parser = CompositeParser::builder().build();
        assert!(parser.is_empty());
        let fields = parser.parse("anything").unwrap();
        assert!(fields.is_empty());
    }

    #[test]
    fn hostname_port_pattern() {
        let parser = CompositeParser::builder()
            .capture("host", parse_ip_or_host_fn)
            .literal(":")
            .capture("port", take_int_fn)
            .build();

        let fields = parser.parse("example.com:443").unwrap();
        assert_eq!(fields["host"], "example.com");
        assert_eq!(fields["port"], "443");
    }
}
