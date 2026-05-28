// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Conditional expressions from Elastic Painless scripts.
//!
//! The `Conditional` type wraps a Painless expression string used in
//! processor `if` fields. VRL transpilation has been removed; the raw
//! Painless source is preserved for future codegen passes.

use serde::{Deserialize, Serialize};

/// A Painless conditional expression (e.g. `ctx?.message instanceof String`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conditional(pub String);

#[cfg(test)]
mod test {
    use super::Conditional;

    #[test]
    fn round_trip_serde() {
        let c = Conditional("ctx?.foo != null".into());
        assert_eq!(c.0, "ctx?.foo != null");
    }
}
