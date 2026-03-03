// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Parser error types.

use thiserror::Error;

/// Result type for all parsers: `(remaining_input, parsed_value)` on success.
pub type ParseResult<'a, T> = Result<(&'a str, T), ParseError>;

/// Errors that occur during parsing.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Encountered an unexpected byte at the given position.
    #[error("unexpected byte at position {pos}: expected {expected}, got {got:#04x}")]
    UnexpectedByte {
        pos: usize,
        expected: &'static str,
        got: u8,
    },

    /// Input ended unexpectedly.
    #[error("unexpected end of input: {context}")]
    UnexpectedEof { context: &'static str },

    /// Input doesn't match the expected format.
    #[error("invalid format: {detail}")]
    InvalidFormat { detail: String },

    /// A parsed numeric value is outside the valid range.
    #[error("value out of range: {value} (expected {min}..={max})")]
    OutOfRange {
        value: String,
        min: String,
        max: String,
    },
}

impl ParseError {
    /// Shorthand for `InvalidFormat` with a static message.
    pub fn invalid(detail: impl Into<String>) -> Self {
        ParseError::InvalidFormat {
            detail: detail.into(),
        }
    }

    /// Shorthand for `UnexpectedEof`.
    pub fn eof(context: &'static str) -> Self {
        ParseError::UnexpectedEof { context }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unexpected_byte_display() {
        let err = ParseError::UnexpectedByte {
            pos: 5,
            expected: "digit",
            got: b'x',
        };
        let msg = err.to_string();
        assert!(msg.contains("position 5"));
        assert!(msg.contains("digit"));
        assert!(msg.contains("0x78"));
    }

    #[test]
    fn unexpected_eof_display() {
        let err = ParseError::eof("IPv4 octet");
        assert!(err.to_string().contains("IPv4 octet"));
    }

    #[test]
    fn invalid_format_display() {
        let err = ParseError::invalid("missing colon separator");
        assert!(err.to_string().contains("missing colon separator"));
    }

    #[test]
    fn out_of_range_display() {
        let err = ParseError::OutOfRange {
            value: "300".into(),
            min: "0".into(),
            max: "255".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("300"));
        assert!(msg.contains("0..=255"));
    }

    #[test]
    fn parse_error_is_clone_and_eq() {
        let err = ParseError::eof("test");
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }
}
