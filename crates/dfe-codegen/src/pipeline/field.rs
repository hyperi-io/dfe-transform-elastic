// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Field path handling with special character escaping.
//!
//! Elastic field names can contain characters like `-` that need special
//! handling. This module provides quoting for such path segments.

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// A field path that handles quoting of segments with special characters.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Field(pub String);

impl<'a> From<&'a str> for Field {
    fn from(value: &'a str) -> Self {
        Field(value.into())
    }
}

lazy_static! {
    static ref PATH_SEGMENT_PATTERN: Regex = Regex::new(r"([^.]+)").unwrap();
    static ref ALLOWED_PATH_NAME: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
}

impl Field {
    /// Return the raw field path string.
    pub fn raw(&self) -> &str {
        &self.0
    }

    /// Format the field path, quoting segments that contain special characters.
    #[instrument(name = "Field::format_path", skip_all)]
    pub fn format_path(&self) -> String {
        let mut path = PATH_SEGMENT_PATTERN
            .captures_iter(&self.0)
            .flat_map(|variable| variable.extract::<1>().1.into_iter())
            .map(|item| {
                if ALLOWED_PATH_NAME.is_match(item) {
                    item.to_string()
                } else {
                    format!("\"{item}\"")
                }
            })
            .collect::<Vec<_>>()
            .join(".");

        if !path.starts_with("_ingest._value") {
            path = format!(".{path}");
        }

        path
    }
}

#[cfg(test)]
mod test {
    use super::Field;
    use pretty_assertions::assert_eq;

    #[test]
    fn single() {
        assert_eq!(".foo", Field::from("foo").format_path());
    }

    #[test]
    fn single_illegal() {
        assert_eq!(".\"foo-bar\"", Field::from("foo-bar").format_path());
    }

    #[test]
    fn path() {
        assert_eq!(".foo.bar.baz", Field::from("foo.bar.baz").format_path());
    }

    #[test]
    fn path_illegal() {
        assert_eq!(
            ".\"foo-bar\".\"baz-qux\"",
            Field::from("foo-bar.baz-qux").format_path()
        );
    }

    #[test]
    fn mixed() {
        assert_eq!(
            ".\"foo-bar\".baz.\"qux-qiz\"",
            Field::from("foo-bar.baz.qux-qiz").format_path()
        );
    }

    #[test]
    fn ingest_value() {
        assert_eq!(
            "_ingest._value.geographicalContext",
            Field::from("_ingest._value.geographicalContext").format_path()
        );
    }
}
