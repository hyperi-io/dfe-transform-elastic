use anyhow::Result;
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{Validate, conditional::Conditional, on_failure::OnFailure};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Gsub {
    pub field: String,
    pub pattern: String,
    pub replacement: String,
    pub ignore_missing: Option<bool>,
    pub target_field: Option<String>,
    pub ignore_failure: Option<bool>,
    pub on_failure: Option<OnFailure>,
    pub tag: Option<String>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
}

impl Validate for Gsub {
    #[instrument(name = "Gsub::validate", skip_all, err)]
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::{
    // pipeline::{pipeline::TranspileCtx, Pipeline},
    // test_utils,
    // };
    // use pretty_assertions::assert_eq;
    //
    // #[test]
    // fn happy_path_scalar() {
    // let configuration = r#"
    // processors:
    // - gsub:
    // field: message
    // pattern: "\\."
    // replacement: "_"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ message: "i_am_sam" }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: "i.am.sam" }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn happy_path_array() {
    // let configuration = r#"
    // processors:
    // - gsub:
    // field: message
    // pattern: "\\."
    // replacement: "_"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ message: ["i_am_sam", "you_are_sam"] }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: ["i.am.sam", "you.are.sam"] }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_missing() {
    // let configuration = r#"
    // processors:
    // - gsub:
    // field: other
    // pattern: "\\."
    // replacement: "_"
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ message: ["i.am.sam", "you.are.sam"] }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: ["i.am.sam", "you.are.sam"] }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_failure_true() {
    // let configuration = r#"
    // processors:
    // - gsub:
    // field: message
    // pattern: "\\."
    // replacement: "_"
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ message: 42 }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: 42 }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_failure_false() {
    // let configuration = r#"
    // processors:
    // - gsub:
    // field: message
    // pattern: "\\."
    // replacement: "_"
    // ignore_failure: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: 42 }))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains("expected string, got integer"));
    // }
    //
    // #[test]
    // fn on_failure() {
    // let configuration = r#"
    // processors:
    // - gsub:
    // field: message
    // pattern: "\\."
    // replacement: "_"
    // tag: test
    // on_failure:
    // - append:
    // field: error.message
    // value: 'Processor {{{_ingest.on_failure_processor_type}}} with tag {{{_ingest.on_failure_processor_tag}}} in pipeline {{{_ingest.pipeline}}} failed with message: {{{_ingest.on_failure_message}}}'
    // "#;
    //
    // assert!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile_with_context(&TranspileCtx::builder().with_name("on_failure_test").build())
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: 42 }))
    // .unwrap()
    // .target
    // .get("error.message[0]")
    // .unwrap()
    // .as_str()
    // .unwrap()
    // .to_string()
    // .contains("Processor gsub with tag test in pipeline on_failure_test failed with message: function call error for \"replace\"")
    // );
    // }
    //
    // #[test]
    // fn target_field() {
    // let configuration = r#"
    // processors:
    // - gsub:
    // field: message
    // pattern: "\\."
    // replacement: "_"
    // target_field: new_message
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ message: "i.am.sam", new_message: "i_am_sam" }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: "i.am.sam" }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn conditional_true() {
    // let configuration = r#"
    // processors:
    // - gsub:
    // if: ctx?.should_gsub == true
    // field: message
    // pattern: "\\."
    // replacement: "_"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ message: "i_am_sam",  should_gsub: true }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: "i.am.sam", should_gsub: true }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn conditional_false() {
    // let configuration = r#"
    // processors:
    // - gsub:
    // if: ctx?.should_gsub == true
    // field: message
    // pattern: "\\."
    // replacement: "_"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ message: "i.am.not.sam",  should_gsub: false }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: "i.am.not.sam", should_gsub: false }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn unicode_control_characters() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - gsub:
    // field: message
    // pattern: '[\u0000-\u001F\u007F]'
    // replacement: ""
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ message: "clean" }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: "c\u{0000}l\u{001F}e\u{007F}an" }))
    // .unwrap()
    // .target
    // );
    // }
    // }
}
