
use anyhow::Result;
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    conditional::Conditional, on_failure::OnFailure, unsupported_fields, Processor, Validate,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Foreach {
    pub field: String,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub processor: Box<Processor>,
    pub ignore_missing: Option<bool>,

    // Unsupported fields
    pub description: Option<String>,
    pub ignore_failure: Option<bool>,
    pub on_failure: Option<OnFailure>,
    pub tag: Option<String>,
}

impl Validate for Foreach {
    #[instrument(name = "Foreach::validate", skip_all, err)]
    fn validate(&self) -> Result<()> {
        unsupported_fields!("foreach", self, description, ignore_failure, on_failure, tag);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod validate {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "foreach",
            r#"
                processors:
                    - foreach:
                        field: items
                        processor:
                            uppercase:
                                field: "_ingest._value"
                        {}: {}
            "#,
            description => "some_description",
            ignore_failure => "true",
            on_failure => "[]",
            tag => "some_tag"
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::{
    // pipeline::Pipeline,
    // test_utils::{self, assert_eq},
    // };
    // use anyhow::{anyhow, Result};
    //
    // #[test]
    // pub fn happy_path() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - foreach:
    // field: messages
    // processor:
    // append:
    // field: target
    // value: "{{_ingest._value}}"
    // allow_duplicates: false
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ "messages": ["hello", "world", "test", "test"], "target": ["hello", "world", "test"] }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ "messages": ["hello", "world", "test", "test"] }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn ignore_missing_true() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - foreach:
    // field: other
    // ignore_missing: true
    // processor:
    // append:
    // field: target
    // value: "{{_ingest._value}}"
    // allow_duplicates: false
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ "messages": ["hello", "world", "test", "test"] }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ "messages": ["hello", "world", "test", "test"] }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn ignore_missing_false() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - foreach:
    // field: other
    // ignore_missing: false
    // processor:
    // append:
    // field: target
    // value: "{{_ingest._value}}"
    // allow_duplicates: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ "messages": ["hello", "world", "test", "test"] }))
    // .err()
    // .unwrap_or_else(|| anyhow!("failed to fail"))
    // .root_cause()
    // .to_string()
    // .contains(".other must not be nullish"));
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn mutate_value() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - foreach:
    // field: okta.request.ip_chain
    // ignore_missing: true
    // processor:
    // rename:
    // field: _ingest._value.geographicalContext
    // target_field: _ingest._value.geographical_context
    // ignore_missing: true
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // okta: {
    // request: {
    // ip_chain: [
    // {
    // geographical_context: "San Francisco"
    // }
    // ],
    // },
    // },
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // okta: {
    // request: {
    // ip_chain: [
    // {
    // geographicalContext: "San Francisco"
    // }
    // ],
    // },
    // },
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    // }
}
