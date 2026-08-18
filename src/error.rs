// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Service error type.

use thiserror::Error;

/// Errors raised by the transform service.
#[derive(Debug, Error)]
pub enum Error {
    /// The configured source has no registered transform.
    #[error("unknown source '{0}' -- no transform registered")]
    UnknownSource(String),

    /// A batch payload was not valid JSON.
    #[error("payload parse failed: {0}")]
    Parse(String),

    /// A transform returned an error for an event.
    #[error("transform '{transform}' failed: {message}")]
    Transform {
        /// Name of the transform that failed.
        transform: String,
        /// Underlying message.
        message: String,
    },

    /// The Kafka transport failed.
    #[error("transport: {0}")]
    Transport(String),

    /// Configuration was invalid.
    #[error("config: {0}")]
    Config(String),

    /// Unwrapping the input envelope failed.
    #[error("envelope: {0}")]
    Envelope(#[from] dfe_runtime::TransformError),
}

/// Service result alias.
pub type Result<T> = std::result::Result<T, Error>;
