use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    Validate, conditional::Conditional, on_failure::OnFailure, unsupported_fields,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Trim {
    pub field: String,
    pub tag: Option<String>,
    pub ignore_missing: Option<bool>,

    // Unsupported
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub target_field: Option<String>,
    pub ignore_failure: Option<bool>,
    pub on_failure: Option<OnFailure>,
}

impl Validate for Trim {
    #[instrument(name = "Trim::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!(
            "trim",
            self,
            target_field,
            conditional,
            ignore_failure,
            on_failure
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod validate {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "trim",
            r#"
                processors:
                    - trim:
                        field: message,
                        {}: {}
            "#,
            conditional => "true && false",
            target_field => "target",
            ignore_failure => "true",
            on_failure => "[]"
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::{
    // pipeline::Pipeline,
    // test_utils::{self, assert_eq},
    // };
    // use anyhow::anyhow;
    //
    // #[test]
    // fn happy_path() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - trim:
    // field: message
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // "message": "foo"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // "message": " foo "
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_missing_true() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - trim:
    // field: message
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // "other_field": "value"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // "other_field": "value"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_missing_false() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - trim:
    // field: message
    // ignore_missing: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // "other_field": "value"
    // }))
    // .err()
    // .ok_or_else(|| anyhow!("failed to fail"))?
    // .root_cause()
    // .to_string()
    // .contains(".message must not be nullish"));
    //
    // Ok(())
    // }
    // }
}
