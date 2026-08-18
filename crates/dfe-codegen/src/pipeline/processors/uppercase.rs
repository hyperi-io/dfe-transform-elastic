use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    Validate, conditional::Conditional, on_failure::OnFailure, unsupported_fields,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Uppercase {
    pub field: String,
    pub ignore_missing: Option<bool>,
    pub tag: Option<String>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub target_field: Option<String>,
    pub ignore_failure: Option<bool>,

    // Unsupported fields
    pub on_failure: Option<OnFailure>,
}

impl Validate for Uppercase {
    #[instrument(name = "Uppercase::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!("uppercase", self, on_failure);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod validate {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "uppercase",
            r"
                processors:
                    - uppercase:
                        field: message
                        {}: {}
            ",
            on_failure => "[]"
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use anyhow::anyhow;
    //
    // use crate::{
    // pipeline::Pipeline,
    // test_utils::{self, assert_eq},
    // };
    //
    // #[test]
    // pub fn happy_path() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - uppercase:
    // field: message
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ "message": "HELLO" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ "message": "hello" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn ignore_missing_true() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - uppercase:
    // field: message
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ "message": "HELLO" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ "message": "hello" }))?
    // .target
    // );
    //
    // assert_eq!(
    // vrl::value!({ "other_field": "value" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ "other_field": "value" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn ignore_missing_false() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - uppercase:
    // field: message
    // ignore_missing: false
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ "message": "HELLO" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ "message": "hello" }))?
    // .target
    // );
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ "other_field": "value" }))
    // .err()
    // .ok_or_else(|| anyhow!("failed to fail"))?
    // .root_cause()
    // .to_string()
    // .contains(".message must not be nullish"));
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn conditional() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - uppercase:
    // if: ctx?.message == "hello"
    // field: message
    // "#;
    //
    // for (input, output) in [
    // (vrl::value!({ "message": "hello" }), vrl::value!({ "message": "HELLO" })),
    // (vrl::value!({ "message": "world" }), vrl::value!({ "message": "world" })),
    // ] {
    // assert_eq!(
    // output,
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(input)?
    // .target
    // );
    // }
    //
    // Ok(())
    // }
    // }
}
