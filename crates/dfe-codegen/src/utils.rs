// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Utility traits.

#![allow(dead_code)]

use std::ops::Deref;

pub trait TraceError {
    fn trace_error(self) -> Self;
}

pub trait TraceAnyhow {
    fn trace_error(self) -> Self;
}

impl<T, E: std::error::Error> TraceError for Result<T, E> {
    fn trace_error(self) -> Self {
        if let Err(err) = &self {
            tracing::error!("{}", err as &dyn std::error::Error)
        }

        self
    }
}

impl<T> TraceAnyhow for Result<T, anyhow::Error> {
    fn trace_error(self) -> Self {
        if let Err(err) = &self {
            tracing::error!("{}", err.deref());
        }

        self
    }
}
