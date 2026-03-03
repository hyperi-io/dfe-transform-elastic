use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::pipeline::{
    conditional::Conditional, unsupported_fields, Processor, Validate,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Field {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Remove {
    pub field: Field,
    pub ignore_missing: Option<bool>,
    pub ignore_failure: Option<bool>,

    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,

    // Unsupported fields
    pub keep: Option<Vec<String>>,
    pub on_failure: Option<Vec<Processor>>,
}

impl Validate for Remove {
    #[instrument(name = "Remove::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!("remove", self, keep, on_failure);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::{processors::remove::Field, unsupported_fields_tests, Pipeline, Processor};

        #[test]
        pub fn parse() {
            let configuration = r#"
              processors:
                - remove:
                    field: foo
            "#;

            let pipeline = Pipeline::parse(configuration).unwrap();
            match &pipeline.processors[..] {
                [Processor::Remove(remove)] => {
                    assert_eq!(Field::One("foo".into()), remove.field)
                }
                _ => panic!("Unexpected pipeline structure {:#?}", pipeline),
            }
        }

        unsupported_fields_tests!(
          "remove",
          r#"
            processors:
              - remove:
                  field: foo
                  {}: {}
          "#,
          keep => "['foo', 'bar']",
          on_failure => r#"[ { "remove": { "field": "bar" } } ]"#
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use anyhow::anyhow;
    // use pretty_assertions::assert_eq;
    //
    // use crate::pipeline::Pipeline;
    //
    // #[test]
    // fn remove_field() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - remove:
    // field: foo
    // "#;
    //
    // assert_eq!(
    // value!({ bar: "bar"}),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ foo: "foo", bar: "bar"}))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // // Elastic will error if the field doesn't exist without `ignore_missing: true`
    // #[test]
    // fn remove_field_twice_not_idempotent() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - remove:
    // field: foo
    // - remove:
    // field: foo
    // "#;
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ foo: "foo", bar: "bar"}))
    // .unwrap_err()
    // .to_string()
    // .contains("tried to remove field '.foo' that doesn't exist"));
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn remove_ignore_missing() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - remove:
    // field: foo
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ bar: "bar"}))?
    // .target,
    // vrl::value!({ bar: "bar"})
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - remove:
    // if: ctx?.azure?.signinlogs?.properties?.user_id == null
    // field: azure.signinlogs.properties.user_id
    // "#;
    //
    // for (input, output) in [
    // (
    // value!({ azure: { signinlogs: { properties: { user_id: null } } } }),
    // value!({ azure: { signinlogs: { properties: {} } } }),
    // ),
    // (
    // value!({ azure: { signinlogs: { properties: { user_id: "foobar" } } } }),
    // value!({ azure: { signinlogs: { properties: { user_id: "foobar" } } } }),
    // ),
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
    //
    // #[test]
    // fn ignore_failure_true() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - remove:
    // ignore_failure: true
    // field: baz
    // "#;
    //
    // assert_eq!(
    // value!({ foo: "foo", bar: "bar"}),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ foo: "foo", bar: "bar"}))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_failure_false() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - remove:
    // ignore_failure: false
    // field: baz
    // "#;
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ foo: "foo", bar: "bar" }))
    // .err()
    // .ok_or_else(|| anyhow!("failed to fail"))?
    // .root_cause()
    // .to_string()
    // .contains("tried to remove field '.baz' that doesn't exist"));
    //
    // Ok(())
    // }
    // }
}
