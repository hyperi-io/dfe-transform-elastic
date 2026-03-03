use anyhow::bail;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::pipeline::unsupported_fields;

use crate::pipeline::{
    conditional::Conditional, template_string::TemplateString, Validate,
};

// We need this song and dance because Elastic supports arrays
// with a single element here for some cursed reason
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Value {
    String(TemplateString),
    Array(Vec<String>),
    Number(i64),
    Bool(bool),
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Value::String(TemplateString(value.to_string()))
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::String(TemplateString("".into()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Set {
    pub field: String,
    pub copy_from: Option<String>,
    pub value: Option<Value>,
    pub ignore_failure: Option<bool>,

    #[serde(rename = "if")]
    pub condition: Option<Conditional>,

    #[serde(rename = "override")]
    pub override_values: Option<bool>,

    // Unsupported configuration options
    pub ignore_empty_value: Option<bool>,
    pub media_type: Option<String>,
    pub on_failure: Option<serde_yaml_ng::Value>,
}

impl Validate for Set {
    #[instrument(name = "Set::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        if self.value.is_none() && self.copy_from.is_none() {
            bail!("value or copy_from must not both be undefined");
        }

        unsupported_fields!("set", self, media_type, on_failure);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::{processors::set::Value, Pipeline, Processor};

        #[test]
        pub fn parse() {
            let configuration = r#"
              processors:
                - set:
                    field: event.ingested
                    value: '{{_ingest.timestamp}}'
            "#;

            let pipeline = Pipeline::parse(configuration).unwrap();
            assert_eq!(pipeline.processors.len(), 1);

            let processor = &pipeline.processors[0];
            match processor {
                Processor::Set(set) => {
                    assert_eq!("event.ingested", set.field);
                    assert_eq!(
                        Value::String("{{_ingest.timestamp}}".into()),
                        *set.value.as_ref().unwrap()
                    );
                }
                _ => panic!("unexpected processor {:#?}", processor),
            }
        }

        #[test]
        pub fn unsupported_media_type() {
            let configuration = r#"
              processors:
                  - set:
                      field: foo
                      value: bar
                      media_type: application/json
            "#;

            if Pipeline::parse(configuration).is_ok() {
                panic!("expected error for unsupported configuration")
            }
        }

        #[test]
        pub fn unsupported_on_failure() {
            let configuration = r#"
              processors:
                  - set:
                      field: foo
                      value: bar
                      on_failure:
                        - foo:
                          bar: baz
            "#;

            if Pipeline::parse(configuration).is_ok() {
                panic!("expected error for unsupported configuration")
            }
        }
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    //
    // use crate::pipeline::Pipeline;
    // use crate::test_utils::{self, assert_eq};
    //
    // #[test]
    // fn string() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: bar
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ foo: "bar"}),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({}))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn number() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: 3
    // "#;
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({}))?
    // .target;
    //
    // match target {
    // value::Value::Object(object) => {
    // assert_eq!(&value!(3), object.get("foo").unwrap());
    // }
    // _ => panic!("unexpected result {:#?}", target),
    // };
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn boolean() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ foo: true }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({}))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn array() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value:
    // - bar
    // - baz
    // "#;
    //
    // assert_eq!(
    // value!({ foo: [ "bar", "baz" ]}),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({}))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ingest_metadata_timestamp() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: '{{_ingest.pipeline}}'
    // "#;
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({}))?
    // .target;
    //
    // match target {
    // value::Value::Object(object) => {
    // match object.get("foo") {
    // Some(value::Value::Bytes(_)) => {}
    // _ => panic!("expected timestamp"),
    // };
    // }
    // _ => panic!("unexpected result {:#?}", target),
    // };
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn set_overrides_set() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: foo
    // - set:
    // field: foo
    // value: bar
    // "#;
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({}))?
    // .target;
    //
    // match target {
    // value::Value::Object(object) => {
    // assert_eq!(&value!("bar"), object.get("foo").unwrap());
    // }
    // _ => panic!("unexpected result {:#?}", target),
    // };
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn set_distinct_keys() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: foo
    // - set:
    // field: bar
    // value: bar
    // "#;
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({}))?
    // .target;
    //
    // match target {
    // value::Value::Object(object) => {
    // assert_eq!(&value!("foo"), object.get("foo").unwrap());
    // assert_eq!(&value!("bar"), object.get("bar").unwrap());
    // }
    // _ => panic!("unexpected result {:#?}", target),
    // };
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn set_metadata_variable() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: '{{bar}}'
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ foo: "bar", bar: "bar"}),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ bar: "bar" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn set_template_variable() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: '{{{bar}}}'
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ foo: "bar", bar: "baz" }))?
    // .target,
    // value!({ foo: "baz", bar: "baz" })
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_empty_value() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: '{{{bar}}}'
    // ignore_empty_value: true
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ foo: "bar" }))?
    // .target,
    // value!({ foo: "bar" })
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_empty_value_false() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: '{{{bar}}}'
    // ignore_empty_value: false
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ foo: "bar" }))?
    // .target,
    // value!({ foo: null })
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional_positive() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: bar
    // if: ctx?.foo == null
    // "#;
    //
    // assert_eq!(
    // value!({ foo: "bar" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({}))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional_negative() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: bar
    // if: ctx?.foo == null
    // "#;
    //
    // assert_eq!(
    // value!({ foo: "foo" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ foo: "foo" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // // I can't figure out from the docs what would even cause set to fail
    // // https://www.elastic.co/guide/en/elasticsearch/reference/8.13/set-processor.html
    // #[ignore]
    // #[test]
    // fn ignore_failure_true() -> anyhow::Result<()> {
    // todo!()
    // }
    //
    // #[ignore]
    // #[test]
    // fn ignore_failure_false() -> anyhow::Result<()> {
    // todo!()
    // }
    //
    // #[test]
    // fn copy_from() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // copy_from: bar
    // "#;
    //
    // assert_eq!(
    // value!({ foo: "bar", bar: "bar" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ foo: "foo", bar: "bar" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn copy_from_array_path() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // copy_from: bar.0
    // "#;
    //
    // assert_eq!(
    // value!({ foo: "foo", bar: ["foo", "bar"] }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ bar: ["foo", "bar"] }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn override_true() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: bar
    // override: true
    // "#;
    //
    // assert_eq!(
    // value!({ foo: "bar" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ foo: "foo" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn override_false() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo
    // value: bar
    // override: false
    // "#;
    //
    // assert_eq!(
    // value!({ foo: "foo" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ foo: "foo" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn nested_shared_path_ignore_empty() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - set:
    // field: foo.bar.baz
    // value: "{{bar}}"
    // ignore_empty_value: true
    // - set:
    // field: foo.baz.qux
    // value: "{{bar}}"
    // ignore_empty_value: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ foo: { bar: { baz: "bar" }, baz: { qux: "bar" } }, bar: "bar" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ bar: "bar" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    // }
}
