// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Elastic ingest pipeline struct and YAML parsing.

use std::collections::HashMap;

use super::Validate;
use super::processors::*;
use anyhow::Context;
use serde::Deserialize;
use tracing::instrument;

/// An Elastic ingest pipeline parsed from YAML.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Pipeline {
    pub description: Option<String>,
    pub processors: Vec<Processor>,
    pub on_failure: Option<Vec<Processor>>,
    #[serde(default)]
    pub pipelines: HashMap<String, Pipeline>,
}

/// All supported Elastic ingest processors.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Processor {
    Append(Append),
    CommunityId(CommunityId),
    Convert(Convert),
    Csv(Csv),
    Date(Date),
    Dissect(Dissect),
    Drop(Drop),
    Fingerprint(Fingerprint),
    Foreach(Foreach),
    Geoip(Geoip),
    Grok(Grok),
    Gsub(Gsub),
    Json(Json),
    Kv(KV),
    Lowercase(Lowercase),
    NetworkDirection(NetworkDirection),
    Pipeline(NestedPipeline),
    RegisteredDomain(RegisteredDomain),
    Remove(Remove),
    Rename(Rename),
    Script(Script),
    Set(Set),
    Split(Split),
    Trim(Trim),
    Uppercase(Uppercase),
    UriParts(UriParts),
    UserAgent(UserAgent),
}

impl Validate for Processor {
    fn validate(&self) -> anyhow::Result<()> {
        match self {
            Processor::Append(inner) => inner.validate(),
            Processor::Csv(inner) => inner.validate(),
            Processor::CommunityId(inner) => inner.validate(),
            Processor::Convert(inner) => inner.validate(),
            Processor::Date(inner) => inner.validate(),
            Processor::Dissect(inner) => inner.validate(),
            Processor::Drop(inner) => inner.validate(),
            Processor::Fingerprint(inner) => inner.validate(),
            Processor::Foreach(inner) => inner.validate(),
            Processor::Geoip(inner) => inner.validate(),
            Processor::Grok(inner) => inner.validate(),
            Processor::Gsub(inner) => inner.validate(),
            Processor::Json(inner) => inner.validate(),
            Processor::Kv(inner) => inner.validate(),
            Processor::Lowercase(inner) => inner.validate(),
            Processor::NetworkDirection(inner) => inner.validate(),
            Processor::Pipeline(inner) => inner.validate(),
            Processor::RegisteredDomain(inner) => inner.validate(),
            Processor::Remove(inner) => inner.validate(),
            Processor::Rename(inner) => inner.validate(),
            Processor::Script(inner) => inner.validate(),
            Processor::Set(inner) => inner.validate(),
            Processor::Split(inner) => inner.validate(),
            Processor::Trim(inner) => inner.validate(),
            Processor::Uppercase(inner) => inner.validate(),
            Processor::UriParts(inner) => inner.validate(),
            Processor::UserAgent(inner) => inner.validate(),
        }
    }
}

impl Processor {
    /// Human-readable name of this processor type.
    pub fn name(&self) -> &'static str {
        match self {
            Processor::Append(_) => "append",
            Processor::CommunityId(_) => "community_id",
            Processor::Convert(_) => "convert",
            Processor::Csv(_) => "csv",
            Processor::Date(_) => "date",
            Processor::Dissect(_) => "dissect",
            Processor::Drop(_) => "drop",
            Processor::Fingerprint(_) => "fingerprint",
            Processor::Foreach(_) => "foreach",
            Processor::Geoip(_) => "geoip",
            Processor::Grok(_) => "grok",
            Processor::Gsub(_) => "gsub",
            Processor::Json(_) => "json",
            Processor::Kv(_) => "kv",
            Processor::Lowercase(_) => "lowercase",
            Processor::NetworkDirection(_) => "network_direction",
            Processor::Pipeline(_) => "pipeline",
            Processor::RegisteredDomain(_) => "registered_domain",
            Processor::Remove(_) => "remove",
            Processor::Rename(_) => "rename",
            Processor::Script(_) => "script",
            Processor::Set(_) => "set",
            Processor::Split(_) => "split",
            Processor::Trim(_) => "trim",
            Processor::Uppercase(_) => "uppercase",
            Processor::UriParts(_) => "uri_parts",
            Processor::UserAgent(_) => "user_agent",
        }
    }
}

impl Pipeline {
    /// Deserialize pipeline YAML without validation.
    ///
    /// Used for building a context map of all pipelines before
    /// resolving nested pipeline references with `parse_with_context`.
    pub fn deserialize(str: &str) -> anyhow::Result<Self> {
        let deserializer = serde_yaml_ng::Deserializer::from_str(str);
        serde_yaml_ng::with::singleton_map_recursive::deserialize::<Pipeline, _>(deserializer)
            .context("failed to deserialize pipeline")
    }

    /// Parse pipeline YAML into a validated Pipeline struct.
    #[instrument(name = "Pipeline::parse", skip_all)]
    pub fn parse(str: &str) -> anyhow::Result<Self> {
        Self::parse_with_context(str, HashMap::new())
    }

    /// Parse pipeline YAML with nested pipeline context.
    #[instrument(name = "Pipeline::parse_with_context", skip_all)]
    pub fn parse_with_context(
        str: &str,
        pipelines: HashMap<String, Pipeline>,
    ) -> anyhow::Result<Self> {
        let deserializer = serde_yaml_ng::Deserializer::from_str(str);
        let mut pipeline =
            serde_yaml_ng::with::singleton_map_recursive::deserialize::<Pipeline, _>(deserializer)
                .context("failed to parse pipeline")?;

        pipeline.processors.iter_mut().enumerate().try_for_each(
            |(index, processor)| -> anyhow::Result<()> {
                let span = tracing::info_span!("validating_processor", index);
                let _guard = span.enter();

                if let Processor::Pipeline(nested) = processor {
                    if let Some(other) = pipelines.get(&nested.name.0) {
                        nested.inner_pipeline = Some(other.clone());
                    }
                }

                processor.validate()
            },
        )?;

        Ok(pipeline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_pipeline() {
        let yaml = r#"
description: "Test pipeline"
processors:
  - set:
      field: event.kind
      value: event
  - remove:
      field: _temp
      ignore_missing: true
"#;
        let pipeline = Pipeline::parse(yaml).unwrap();
        assert_eq!(pipeline.processors.len(), 2);
        assert_eq!(pipeline.processors[0].name(), "set");
        assert_eq!(pipeline.processors[1].name(), "remove");
    }

    #[test]
    fn parse_pipeline_with_on_failure() {
        let yaml = r#"
description: "Pipeline with error handling"
processors:
  - set:
      field: event.kind
      value: event
on_failure:
  - set:
      field: error.message
      value: "pipeline failed"
"#;
        let pipeline = Pipeline::parse(yaml).unwrap();
        assert!(pipeline.on_failure.is_some());
        assert_eq!(pipeline.on_failure.unwrap().len(), 1);
    }
}
