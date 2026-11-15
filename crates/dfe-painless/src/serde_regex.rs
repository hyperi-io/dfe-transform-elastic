// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A compiled [`Regex`] carried through a serialised plan as its pattern text.

use regex::Regex;
use serde::{Deserialize, Deserializer, Serializer};

/// Write the regex as the pattern it was built from.
///
/// # Errors
///
/// Whatever the serializer returns.
pub fn serialize<S: Serializer>(regex: &Regex, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(regex.as_str())
}

/// Rebuild the regex from its pattern.
///
/// # Errors
///
/// A pattern the regex engine refuses, which a plan the generator wrote never holds.
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Regex, D::Error> {
    let pattern = String::deserialize(deserializer)?;
    Regex::new(&pattern).map_err(serde::de::Error::custom)
}
