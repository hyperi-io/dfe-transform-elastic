// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Error types for the transform runtime.

use thiserror::Error;

/// Alias for `Result<T, TransformError>`.
pub type Result<T> = std::result::Result<T, TransformError>;

/// Errors that occur during event transformation.
#[derive(Error, Debug)]
pub enum TransformError {
    /// A required field was not found at the given path.
    #[error("field not found: '{path}'")]
    FieldNotFound { path: String },

    /// A field exists but has an unexpected type.
    #[error("type mismatch at '{path}': expected {expected}, got {actual}")]
    TypeMismatch {
        path: String,
        expected: &'static str,
        actual: String,
    },

    /// A field value could not be parsed.
    #[error("parse error at '{path}': {message}")]
    ParseError { path: String, message: String },

    /// An enrichment module (geoip, user_agent, community_id) failed.
    #[error("enrichment '{enrichment}' failed: {message}")]
    EnrichmentError { enrichment: String, message: String },

    /// A named processor failed.
    #[error("processor '{processor}' failed: {source}")]
    ProcessorError {
        processor: String,
        #[source]
        source: Box<TransformError>,
    },

    /// JSON serialisation/deserialisation error.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// Wrapped I/O error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl TransformError {
    /// Wrap this error with processor context.
    pub fn with_processor(self, processor: impl Into<String>) -> Self {
        TransformError::ProcessorError {
            processor: processor.into(),
            source: Box::new(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_not_found_display() {
        let err = TransformError::FieldNotFound {
            path: "source.ip".into(),
        };
        assert_eq!(err.to_string(), "field not found: 'source.ip'");
    }

    #[test]
    fn type_mismatch_display() {
        let err = TransformError::TypeMismatch {
            path: "event.duration".into(),
            expected: "i64",
            actual: "string".into(),
        };
        assert!(err.to_string().contains("event.duration"));
        assert!(err.to_string().contains("expected i64"));
    }

    #[test]
    fn with_processor_wraps_error() {
        let inner = TransformError::FieldNotFound {
            path: "host.name".into(),
        };
        let wrapped = inner.with_processor("set");
        assert!(wrapped.to_string().contains("processor 'set' failed"));
    }

    #[test]
    fn question_mark_propagation() {
        fn fallible() -> Result<()> {
            let _: serde_json::Value = serde_json::from_str("invalid")?;
            Ok(())
        }
        assert!(fallible().is_err());
    }
}
