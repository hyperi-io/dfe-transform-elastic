
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    conditional::Conditional, field::Field, on_failure::OnFailure, unsupported_fields, Validate,
};

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct Rename {
    pub field: Field,
    pub target_field: Field,
    pub ignore_missing: Option<bool>,
    pub ignore_failure: Option<bool>,
    pub tag: Option<String>,

    // Unimplemented options
    #[serde(alias = "override")]
    pub overwrite: Option<bool>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub on_failure: Option<OnFailure>,
}

impl Validate for Rename {
    #[instrument(name = "Rename::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!("rename", self, overwrite, on_failure);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::unsupported_fields_tests;
        use pretty_assertions::assert_eq;

        unsupported_fields_tests!(
            "rename",
            r#"
              processors:
                  - rename:
                      field: source
                      target_field: target
                      {}: {}
            "#,
            overwrite => "true",
            on_failure => "[]"
        );
    }
    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::pipeline::Pipeline;
    // use pretty_assertions::assert_eq;
    //
    // #[test]
    // fn happy_path() {
    // let configuration = r#"
    // processors:
    // - rename:
    // field: source
    // target_field: target
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ source: "foo" }))
    // .unwrap()
    // .target,
    // vrl::value!({ target: "foo"})
    // );
    // }
    //
    // #[test]
    // fn missing_field() {
    // let configuration = r#"
    // processors:
    // - rename:
    // field: source
    // target_field: target
    // "#;
    //
    // assert!(Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ target: "foo" }))
    // .unwrap_err()
    // .to_string()
    // .contains(".source must not be null"));
    // }
    //
    // #[test]
    // fn target_field_non_null() {
    // let configuration = r#"
    // processors:
    // - rename:
    // field: source
    // target_field: target
    // "#;
    //
    // assert!(Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ source: "foo", target: "bar" }))
    // .unwrap_err()
    // .to_string()
    // .contains(".target must be null or override: true"));
    // }
    //
    // #[test]
    // fn ignore_missing() {
    // let configuration = r#"
    // processors:
    // - rename:
    // field: source
    // target_field: target
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ foo: "bar" }))
    // .unwrap()
    // .target,
    // vrl::value!({ foo: "bar"})
    // );
    // }
    //
    // #[test]
    // fn names_with_dashes() {
    // let configuration = r#"
    // processors:
    // - rename:
    // field: azure
    // target_field: azure-eventhub
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ "azure-eventhub": "foo" }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ "azure": "foo" }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn conditional_true() {
    // let configuration = r#"
    // processors:
    // - rename:
    // field: azure
    // if: true
    // target_field: azure-eventhub
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ "azure-eventhub": "foo" }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ "azure": "foo" }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn conditional_false() {
    // let configuration = r#"
    // processors:
    // - rename:
    // field: azure
    // if: false
    // target_field: azure-eventhub
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ "azure": "foo" }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ "azure": "foo" }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_failure() {
    // let configuration = r#"
    // processors:
    // - rename:
    // field: source
    // ignore_failure: true
    // target_field: target
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({ source: "foo", target: "bar" }),
    // vrl::value!({ source: "foo", target: "bar" }),
    // ),
    // (vrl::value!({ target: "bar" }), vrl::value!({ target: "bar" })),
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
