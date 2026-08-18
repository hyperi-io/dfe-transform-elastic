// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Elastic template string handling.
//!
//! Elastic uses `{{field}}` and `{{{field}}}` syntax for string interpolation
//! in processor values. This module parses those templates and preserves the
//! structure for codegen.

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref METADATA_VARIABLES_PATTERN: Regex =
        Regex::new(r"(?:[{]{2,3} *(?<variable>[^{}]+) *[}]{2,3}|(?<string>[^{}]+))").unwrap();
}

/// An Elastic template string that may contain `{{field}}` or `{{{field}}}` interpolations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplateString(pub String);

impl From<&str> for TemplateString {
    fn from(value: &str) -> Self {
        TemplateString(value.into())
    }
}

impl TemplateString {
    /// Return the raw template string.
    pub fn raw(&self) -> &str {
        &self.0
    }

    /// Extract the template fragments (variables and string literals).
    pub fn fragments(&self) -> Vec<TemplateFragment> {
        METADATA_VARIABLES_PATTERN
            .captures_iter(&self.0)
            .map(|capture| {
                if let Some(variable) = capture.name("variable") {
                    return TemplateFragment::Variable(variable.as_str().to_string());
                }
                if let Some(string) = capture.name("string") {
                    return TemplateFragment::Literal(string.as_str().to_string());
                }
                unreachable!();
            })
            .collect()
    }
}

/// A fragment of a parsed template string.
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateFragment {
    /// A literal string portion.
    Literal(String),
    /// A variable reference (the content between `{{` and `}}`).
    Variable(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn plain_string() {
        let ts = TemplateString::from("Hello, World");
        let frags = ts.fragments();
        assert_eq!(
            frags,
            vec![TemplateFragment::Literal("Hello, World".into())]
        );
    }

    #[test]
    fn single_variable() {
        let ts = TemplateString::from("{{{_ingest.on_failure_processor_type}}}");
        let frags = ts.fragments();
        assert_eq!(
            frags,
            vec![TemplateFragment::Variable(
                "_ingest.on_failure_processor_type".into()
            )]
        );
    }

    #[test]
    fn mixed_template() {
        let ts = TemplateString::from("Processor {{{_ingest.on_failure_processor_type}}} failed");
        let frags = ts.fragments();
        assert_eq!(
            frags,
            vec![
                TemplateFragment::Literal("Processor ".into()),
                TemplateFragment::Variable("_ingest.on_failure_processor_type".into()),
                TemplateFragment::Literal(" failed".into()),
            ]
        );
    }

    #[test]
    fn double_brace_variable() {
        let ts = TemplateString::from("{{event.timezone}}");
        let frags = ts.fragments();
        assert_eq!(
            frags,
            vec![TemplateFragment::Variable("event.timezone".into())]
        );
    }
}
