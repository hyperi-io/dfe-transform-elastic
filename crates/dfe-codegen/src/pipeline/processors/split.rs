
use anyhow::bail;
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    conditional::Conditional, on_failure::OnFailure, unsupported_fields,
    Validate,
};

// https://www.elastic.co/guide/en/elasticsearch/reference/8.13/split-processor.html
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Split {
    pub field: String,
    pub separator: String,
    pub ignore_missing: Option<bool>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub target_field: Option<String>,

    // Unsupported
    pub preserve_trailing: Option<bool>,
    pub ignore_failure: Option<bool>,
    pub on_failure: Option<OnFailure>,
}

impl Validate for Split {
    #[instrument(name = "Split::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!("split", self, ignore_failure, on_failure);

        if let Some(true) = self.preserve_trailing {
            bail!("codegen does not currently support 'preserve_trailing' in 'split' processor");
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "split",
            r#"
                processors:
                    - split:
                        field: message
                        separator: ",\\s+"
                        {}: {}
            "#,
            preserve_trailing => "true",
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
    //
    // #[test]
    // pub fn happy_path() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - split:
    // field: message
    // separator: ",\\s+"
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({ message: "Hello, World"}),
    // vrl::value!({ message: ["Hello", "World"]}),
    // ),
    // (
    // vrl::value!({ message: "Hello,   World"}),
    // vrl::value!({ message: ["Hello", "World"]}),
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
    //
    // #[test]
    // pub fn ignore_missing_true() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - split:
    // field: message
    // separator: ",\\s+"
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({}),
    // Pipeline::parse(configuration)
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
    // pub fn ignore_missing_false() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - split:
    // field: message
    // separator: ",\\s+"
    // ignore_missing: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({}))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains("expected string, got null"));
    // }
    //
    // #[test]
    // pub fn conditional() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - split:
    // if: ctx?.message == "Hello, World"
    // field: message
    // separator: ",\\s+"
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({ message: "Hello, World"}),
    // vrl::value!({ message: ["Hello", "World"]}),
    // ),
    // (
    // vrl::value!({ message: "Hello,   World"}),
    // vrl::value!({ message: "Hello,   World"}),
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
    //
    // #[test]
    // pub fn target_field() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - split:
    // field: message
    // separator: ",\\s+"
    // target_field: split_result
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({ message: "Hello, World"}),
    // vrl::value!({ message: "Hello, World", split_result: ["Hello", "World"]}),
    // ),
    // (
    // vrl::value!({ message: "Hello,   World"}),
    // vrl::value!({ message: "Hello,   World", split_result: ["Hello", "World"]}),
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
