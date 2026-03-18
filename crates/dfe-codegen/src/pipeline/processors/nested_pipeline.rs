use anyhow::{Result, ensure};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, de::Error};
use tracing::instrument;

use crate::pipeline::{
    Pipeline, Validate, conditional::Conditional, on_failure::OnFailure, unsupported_fields,
};

lazy_static! {
    static ref PIPELINE_NAME_PATTERN: Regex =
        Regex::new(r#"IngestPipeline "(?<name>[^"]+)""#).unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct PipelineName(pub String);

impl<'de> Deserialize<'de> for PipelineName {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;

        let name = PIPELINE_NAME_PATTERN
            .captures(&text)
            .ok_or_else(|| D::Error::custom("failed to match pipeline name"))?
            .name("name")
            .ok_or_else(|| D::Error::custom("failed to extract pipeline name capture group"))?
            .as_str()
            .to_string();

        Ok(PipelineName(name))
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct NestedPipeline {
    pub name: PipelineName,
    pub tag: Option<String>,
    #[serde(alias = "if")]
    pub condition: Option<Conditional>,

    #[serde(skip)]
    pub inner_pipeline: Option<Pipeline>,

    // Unsupported fields
    pub ignore_missing_pipeline: Option<bool>,
    pub ignore_failure: Option<bool>,
    pub on_failure: Option<OnFailure>,
}

impl Validate for NestedPipeline {
    #[instrument(name = "NestedPipeline::validate", skip_all, fields(name = ?self.name), err)]
    fn validate(&self) -> Result<()> {
        unsupported_fields!(
            "pipeline",
            self,
            ignore_missing_pipeline,
            ignore_failure,
            on_failure
        );

        ensure!(
            self.inner_pipeline.is_some(),
            "nested pipeline was not properly expanded"
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {

        use crate::pipeline::unsupported_fields_tests;
        use pretty_assertions::assert_eq;

        unsupported_fields_tests!(
            "pipeline",
            r#"
                processors:
                    - pipeline:
                        name: '{{< IngestPipeline "azure-shared-pipeline" >}}'
                        {}: {}
            "#,
            ignore_missing_pipeline => "true",
            ignore_failure => "true",
            on_failure => "[]"
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use std::collections::HashMap;
    //
    // use crate::pipeline::Pipeline;
    // use pretty_assertions::assert_eq;
    //
    // #[test]
    // fn happy_path() {
    // let nested_pipeline = r#"
    // processors:
    // - set:
    // field: target
    // value: Hello, World
    // "#;
    //
    // let nested_pipeline = Pipeline::parse(nested_pipeline).unwrap();
    //
    // let configuration = r#"
    // processors:
    // - set:
    // field: target
    // value: Goodbye, World
    // - pipeline:
    // name: '{{< IngestPipeline "azure-shared-pipeline" >}}'
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ target: "Hello, World" }),
    // Pipeline::parse_with_context(
    // configuration,
    // HashMap::from([("azure-shared-pipeline".into(), nested_pipeline)]),
    // )
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({}))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn condition() {
    // let nested_pipeline_a = {
    // let configuration = r#"
    // processors:
    // - set:
    // field: target
    // value: Hello, World
    // "#;
    //
    // Pipeline::parse(configuration).unwrap()
    // };
    //
    // let nested_pipeline_b = {
    // let configuration = r#"
    // processors:
    // - set:
    // field: target
    // value: Goodbye, World
    // "#;
    //
    // Pipeline::parse(configuration).unwrap()
    // };
    //
    // let configuration = r#"
    // processors:
    // - pipeline:
    // if: ctx?.event?.type == "HELLO"
    // name: '{{< IngestPipeline "pipeline_a" >}}'
    // - pipeline:
    // if: ctx?.event?.type == "GOODBYE"
    // name: '{{< IngestPipeline "pipeline_b" >}}'
    // "#;
    //
    // let pipeline = Pipeline::parse_with_context(
    // configuration,
    // HashMap::from([
    // ("pipeline_a".into(), nested_pipeline_a),
    // ("pipeline_b".into(), nested_pipeline_b),
    // ]),
    // )
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap();
    //
    // for (input, output) in [
    // (
    // vrl::value!({ event: { type: "HELLO" } }),
    // vrl::value!({ event: { type: "HELLO" }, target: "Hello, World" }),
    // ),
    // (
    // vrl::value!({ event: { type: "GOODBYE" } }),
    // vrl::value!({ event: { type: "GOODBYE" }, target: "Goodbye, World" }),
    // ),
    // ] {
    // assert_eq!(output, pipeline.run(input).unwrap().target);
    // }
    // }
    // }
}
