// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Dynamic path parsing for Elastic template-based field paths.
//!
//! Handles paths like `cisco_meraki.{{{cisco_meraki.event_subtype}}}` where
//! parts of the path are resolved from event fields at runtime.

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use tracing::instrument;

lazy_static! {
    static ref DYNAMIC_PATH_PATTERN: Regex =
        Regex::new(r#"([{]{2,3}(?<dynamic_path>[^}]+)[}]{2,3}|(?<static_path>[^.]+))"#).unwrap();
}

/// A field path that may contain template expressions.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DynamicPath(String);

impl From<String> for DynamicPath {
    fn from(value: String) -> Self {
        DynamicPath(value)
    }
}

impl<'a> From<&'a str> for DynamicPath {
    fn from(value: &'a str) -> Self {
        DynamicPath(value.to_string())
    }
}

/// A single component of a parsed dynamic path.
#[derive(Debug, Clone, PartialEq)]
pub enum PathComponent {
    /// A static (literal) path segment.
    Static(String),
    /// A dynamic (template) path segment resolved at runtime.
    Dynamic(String),
}

/// A fully parsed dynamic path.
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedDynamicPath {
    /// All segments are static (e.g. `foo.bar.baz`).
    StaticPath(String),
    /// Contains at least one dynamic segment.
    DynamicPath(Vec<PathComponent>),
}

impl DynamicPath {
    /// Return the raw path string.
    pub fn raw(&self) -> &str {
        &self.0
    }

    /// Parse the path into static/dynamic components.
    #[instrument(err)]
    pub fn parse(&self) -> anyhow::Result<ParsedDynamicPath> {
        if DYNAMIC_PATH_PATTERN
            .captures_iter(&self.0)
            .all(|capture| capture.name("static_path").is_some())
        {
            return Ok(ParsedDynamicPath::StaticPath(self.0.clone()));
        }

        Ok(ParsedDynamicPath::DynamicPath(
            DYNAMIC_PATH_PATTERN
                .captures_iter(&self.0)
                .fold(Vec::new(), |mut acc, it| {
                    if let Some(dynamic_path) = it.name("dynamic_path") {
                        acc.push(PathComponent::Dynamic(dynamic_path.as_str().to_string()));
                    }

                    if let Some(static_path) = it.name("static_path") {
                        acc.push(PathComponent::Static(static_path.as_str().to_string()));
                    }

                    acc
                }),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn static_path() {
        let parsed = DynamicPath::from("cisco_meraki.event_subtype")
            .parse()
            .unwrap();
        assert_eq!(
            parsed,
            ParsedDynamicPath::StaticPath("cisco_meraki.event_subtype".into())
        );
    }

    #[test]
    fn dynamic_path() {
        let parsed = DynamicPath::from("{{{cisco_meraki.event_subtype}}}")
            .parse()
            .unwrap();
        match parsed {
            ParsedDynamicPath::DynamicPath(components) => {
                assert_eq!(
                    components,
                    vec![PathComponent::Dynamic("cisco_meraki.event_subtype".into())]
                );
            }
            _ => panic!("expected DynamicPath"),
        }
    }

    #[test]
    fn mixed_path() {
        let parsed = DynamicPath::from("cisco_meraki.{{{cisco_meraki.event_subtype}}}")
            .parse()
            .unwrap();
        match parsed {
            ParsedDynamicPath::DynamicPath(components) => {
                assert_eq!(
                    components,
                    vec![
                        PathComponent::Static("cisco_meraki".into()),
                        PathComponent::Dynamic("cisco_meraki.event_subtype".into()),
                    ]
                );
            }
            _ => panic!("expected DynamicPath"),
        }
    }
}
