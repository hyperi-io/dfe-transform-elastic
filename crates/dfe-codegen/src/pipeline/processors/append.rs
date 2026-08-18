use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::pipeline::{
    Processor, Validate, conditional::Conditional, template_string::TemplateString,
    unsupported_fields,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AppendValue {
    String(TemplateString),
    Array(Vec<TemplateString>),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Append {
    pub field: String,
    pub value: AppendValue,
    pub allow_duplicates: Option<bool>,
    pub ignore_failure: Option<bool>,

    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,

    // Unsupported fields
    pub media_type: Option<String>,
    pub on_failure: Option<Vec<Processor>>,
}

impl Validate for Append {
    #[instrument(name = "Append::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!("append", self, media_type, on_failure);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::Pipeline;
        use crate::pipeline::Processor;
        use crate::pipeline::processors::append::AppendValue;
        use crate::pipeline::unsupported_fields_tests;

        #[test]
        fn parse() {
            let configuration = r"
              processors:
                - append:
                    field: warnings
                    value: this is a stern warning
            ";

            let pipeline = Pipeline::parse(configuration).unwrap();
            match &pipeline.processors[..] {
                [Processor::Append(append)] => {
                    assert_eq!(append.field, "warnings");
                    assert_eq!(
                        append.value,
                        AppendValue::String("this is a stern warning".into())
                    );
                }
                _ => panic!("unexpected pipeline structure {pipeline:#?}"),
            }
        }

        unsupported_fields_tests!(
          "append",
          r"
            processors:
              - append:
                  field: warnings
                  value: this is a stern warning
                  {}: {}
          ",
          media_type => "application/json",
          on_failure => r#"[ { "remove": { "field": "message" } } ]"#
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    //
    // use crate::pipeline::Pipeline;
    // use crate::test_utils::assert_eq;
    //
    // #[test]
    // fn append_scalar_to_array() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - append:
    // field: error.message
    // value: bar
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ error: { message: ["foo"] } }))?
    // .target,
    // value!({ error: { message: ["foo", "bar"] } })
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn append_array_to_array() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - append:
    // field: error.message
    // value: [ "bar", "baz" ]
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ error: { message: ["foo"] } }))?
    // .target,
    // value!({ error: { message: ["foo", "bar", "baz"] } })
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn append_scalar_to_scalar() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - append:
    // field: error.message
    // value: "bar"
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ error: { message: "foo" } }))?
    // .target,
    // value!({ error: { message: ["foo", "bar"] } })
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn append_array_to_scalar() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - append:
    // field: error.message
    // value: ["bar", "baz"]
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({ error: { message: "foo" } }))?
    // .target,
    // value!({ error: { message: ["foo", "bar", "baz"] } })
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn append_scalar_to_empty() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - append:
    // field: error.message
    // value: "foo"
    // "#;
    //
    // assert_eq!(
    // value!({ error: { message: ["foo"] } }),
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
    // fn append_array_to_empty() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - append:
    // field: error.message
    // value: ["foo", "bar"]
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({}))?
    // .target,
    // value!({ error: { message: ["foo", "bar"] } })
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn append_metadata_to_empty() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // tag: example_tag
    // field: message
    // on_failure:
    // - remove:
    // field: message
    // - append:
    // field: error.message
    // value: 'Processor {{_ingest.on_failure_processor_tag}} failed'
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(value!({ message: r#"{ "foo": "bar "# }))
    // .unwrap()
    // .target,
    // value!({ error: { message: ["Processor example_tag failed"] } })
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn append_metadata_to_scalar() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // tag: example_tag
    // field: message
    // on_failure:
    // - remove:
    // field: message
    // - append:
    // field: error.message
    // value: 'Processor {{_ingest.on_failure_processor_tag}} failed'
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(value!({ message: r#"{ "foo": "bar "#, error: { message: "existing error message" } }))
    // .unwrap()
    // .target,
    // value!({ error: { message: ["existing error message", "Processor example_tag failed"] } })
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn append_metadata_to_array() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // tag: example_tag
    // field: message
    // on_failure:
    // - remove:
    // field: message
    // - append:
    // field: error.message
    // value: 'Processor {{_ingest.on_failure_processor_tag}} failed'
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(value!({ message: r#"{ "foo": "bar "#, error: { message: ["error one", "error two"] } }))
    // .unwrap()
    // .target,
    // value!({ error: { message: ["error one", "error two", "Processor example_tag failed"] } })
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn allow_duplicates_positive() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - append:
    // field: target
    // value: ["bar", "baz"]
    // "#;
    //
    // for (input, output) in [
    // (value!({ target: ["foo"] }), value!({ target: ["foo", "bar", "baz"]})),
    // (
    // value!({ target: ["foo", "foo"] }),
    // value!({ target: ["foo", "foo", "bar", "baz"]}),
    // ),
    // (value!({ target: "foo" }), value!({ target: ["foo", "bar", "baz"]})),
    // (
    // value!({ target: ["foo", "bar"] }),
    // value!({ target: ["foo", "bar", "bar", "baz"]}),
    // ),
    // (value!({ target: "bar" }), value!({ target: ["bar", "bar", "baz"]})),
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
    // fn allow_duplicates_negative() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - append:
    // field: target
    // value: ["bar", "baz"]
    // allow_duplicates: false
    // "#;
    //
    // for (input, output) in [
    // (value!({ target: ["foo"] }), value!({ target: ["foo", "bar", "baz"]})),
    // (
    // value!({ target: ["foo", "foo", "bar"] }),
    // value!({ target: ["foo", "foo", "bar", "baz"]}),
    // ),
    // (value!({ target: "foo" }), value!({ target: ["foo", "bar", "baz"]})),
    // (
    // value!({ target: ["foo", "bar"] }),
    // value!({ target: ["foo", "bar", "baz"]}),
    // ),
    // (value!({ target: "bar" }), value!({ target: ["bar", "baz"]})),
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
    // fn allow_duplicates_negative_single_value() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - append:
    // field: target
    // value: "bar"
    // allow_duplicates: false
    // "#;
    //
    // for (input, output) in [
    // (value!({ target: "bar" }), value!({ target: "bar" })),
    // (value!({ target: "baz" }), value!({ target: ["baz", "bar"] })),
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
    // fn conditional() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - append:
    // if: ctx?.source.ip != null
    // field: related.ip
    // value: '{{{source.ip}}}'
    // "#;
    //
    // for (input, output) in [
    // (
    // value!({ source: { ip: "10.0.0.1" }}),
    // value!({ source: { ip: "10.0.0.1" }, related: { ip: [ "10.0.0.1" ] }}),
    // ),
    // (
    // value!({ target: { ip: "10.0.0.1" }}),
    // value!({ target: { ip: "10.0.0.1" }}),
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
    // // Can't figure out what would even make append fail
    // #[ignore]
    // #[test]
    // fn ignore_failure_true() -> anyhow::Result<()> {
    // todo!();
    // }
    //
    // #[ignore]
    // #[test]
    // fn ignore_failure_false() -> anyhow::Result<()> {
    // todo!();
    // }
    // }
    // }
    //
}
