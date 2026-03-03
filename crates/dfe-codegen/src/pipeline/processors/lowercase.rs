
use anyhow::Result;
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    conditional::Conditional, on_failure::OnFailure, unsupported_fields,
    Validate,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Lowercase {
    pub field: String,
    pub ignore_missing: Option<bool>,
    pub ignore_failure: Option<bool>,
    pub tag: Option<String>,
    pub target_field: Option<String>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,

    // Unsupported Fields
    pub on_failure: Option<OnFailure>,
}

impl Validate for Lowercase {
    #[instrument(name = "Lowercase::validate", skip_all, err)]
    fn validate(&self) -> Result<()> {
        unsupported_fields!("lowercase", self, on_failure);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "lowercase",
            r#"
                processors:
                    - lowercase:
                        field: target
                        {}: {}
            "#,
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
    //
    // #[test]
    // fn happy_path() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - lowercase:
    // field: target
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // target: "why are we yelling"
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ target: "WHY ARE WE YELLING" }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn happy_path_array() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - lowercase:
    // field: target
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // target: ["why", "are", "we", "yelling"]
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ target: ["WHY", "ARE", "WE", "YELLING"] }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_missing_true() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - lowercase:
    // field: wrong_target
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // target: "WHY ARE WE YELLING"
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ target: "WHY ARE WE YELLING" }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_missing_false() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - lowercase:
    // field: wrong_target
    // ignore_missing: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ target: "WHY ARE WE YELLING" }))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains(".wrong_target must not be nullish"));
    // }
    //
    // #[test]
    // fn ignore_failure_true() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - lowercase:
    // field: target
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // target: 42
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ target: 42 }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_failure_false() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - lowercase:
    // field: target
    // ignore_failure: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ target: 42 }))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains("function call error for \"downcase\""));
    // }
    //
    // #[test]
    // fn target_field() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - lowercase:
    // field: source
    // target_field: target
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // source: "WHY ARE WE YELLING",
    // target: "why are we yelling"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ source: "WHY ARE WE YELLING" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - lowercase:
    // if: ctx?.message == "WHY ARE WE YELLING"
    // field: message
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({ message: "WHY ARE WE YELLING" }),
    // vrl::value!({ message: "why are we yelling" }),
    // ),
    // (
    // vrl::value!({ message: "WHY ARE YOU YELLING" }),
    // vrl::value!({ message: "WHY ARE YOU YELLING" }),
    // ),
    // ] {
    // assert_eq!(
    // output,
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(input)
    // .unwrap()
    // .target
    // );
    // }
    // }
    // }
}
