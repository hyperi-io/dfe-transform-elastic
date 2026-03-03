use serde::Deserialize;
use tracing::instrument;
use crate::pipeline::{unsupported_fields};

use crate::pipeline::{
    conditional::Conditional,
    on_failure::OnFailure, Validate,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Json {
    pub field: String,
    #[serde(rename = "if")]
    pub conditional: Option<Conditional>,
    pub on_failure: Option<OnFailure>,
    pub tag: Option<String>,
    pub ignore_failure: Option<bool>,

    // Unsupported options
    pub target_field: Option<String>,
    pub add_to_root: Option<bool>,
    pub add_to_root_conflict_strategy: Option<String>,
    pub allow_duplicate_keys: Option<bool>,
    pub strict_json_parsing: Option<bool>,
}

impl Validate for Json {
    #[instrument(name = "Json::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!(
            "json",
            self,
            add_to_root,
            add_to_root_conflict_strategy,
            allow_duplicate_keys,
            strict_json_parsing
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::conditional::Conditional;
        use crate::pipeline::on_failure::OnFailure;
        use crate::pipeline::processors::set::Set;
        use crate::pipeline::{unsupported_fields_tests, Pipeline, Processor};

        #[test]
        pub fn parse() {
            let configuration = r#"
              processors:
                - json:
                    field: message
            "#;

            let pipeline = Pipeline::parse(configuration).unwrap();
            match &pipeline.processors[..] {
                [Processor::Json(json)] => {
                    assert_eq!("message", json.field);
                }
                _ => panic!("unexpected pipeline"),
            };
        }

        #[test]
        pub fn parse_on_failure() {
            let configuration = r#"
              processors:
                - json:
                    field: message
                    on_failure:
                      - set:
                          field: error
                          value: failed to process json
            "#;

            let pipeline = Pipeline::parse(configuration).unwrap();
            match &pipeline.processors[..] {
                [Processor::Json(json)] => {
                    assert_eq!("message", json.field);
                    assert_eq!(
                        Some(OnFailure(vec![Processor::Set(Set {
                            field: "error".into(),
                            value: Some("failed to process json".into()),
                            ..Default::default()
                        })])),
                        json.on_failure
                    );
                }
                _ => panic!("unexpected pipeline"),
            };
        }

        #[test]
        pub fn parse_conditional() {
            let configuration = r#"
              processors:
                - json:
                    if: message instanceof String
                    field: message
            "#;

            let pipeline = Pipeline::parse(configuration).unwrap();
            match &pipeline.processors[..] {
                [Processor::Json(json)] => {
                    assert_eq!("message", json.field);
                    assert_eq!(Some(Conditional("message instanceof String".into())), json.conditional)
                }
                _ => panic!("unexpected pipeline"),
            };
        }

        #[test]
        pub fn parse_tag() {
            let configuration = r#"
              processors:
                - json:
                    tag: example
                    field: message
            "#;

            let pipeline = Pipeline::parse(configuration).unwrap();
            match &pipeline.processors[..] {
                [Processor::Json(json)] => {
                    assert_eq!("message", json.field);
                    assert_eq!(Some("example".to_owned()), json.tag)
                }
                _ => panic!("unexpected pipeline"),
            };
        }

        unsupported_fields_tests!(
            "json",
            r#"
                processors:
                    - json:
                        field: message
                        {}: {}
            "#,
            add_to_root => "true",
            add_to_root_conflict_strategy => "merge",
            allow_duplicate_keys => "true",
            strict_json_parsing => "false"
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    //
    // use crate::pipeline::Pipeline;
    // use pretty_assertions::assert_eq;
    //
    // #[test]
    // fn unpack_json() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // field: message
    // "#;
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({
    // message: r#"{ "key": "value" }"#
    // }))?
    // .target;
    //
    // assert_eq!(
    // value!({
    // message: { key: "value" }
    // }),
    // target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn missing_key_failure() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // field: message
    // "#;
    //
    // if Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({
    // some_other_key: r#"{ "key: "value" }"#
    // }))
    // .is_ok()
    // {
    // panic!("expected processing to fail due to malformed json")
    // }
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn unpack_json_failure() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // field: message
    // "#;
    //
    // if Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({
    // message: r#"{ "key: "value" }"#
    // }))
    // .is_ok()
    // {
    // panic!("expected processing to fail due to malformed json")
    // }
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn on_failure() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // field: message
    // on_failure:
    // - set:
    // field: error
    // value: failed to process json
    // "#;
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({
    // message: r#"{ "key: "value" }"#
    // }))?
    // .target;
    //
    // assert_eq!(
    // value!({
    // message: r#"{ "key: "value" }"#,
    // error: "failed to process json"
    // }),
    // target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn on_failure_meta_variable() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // field: message
    // on_failure:
    // - set:
    // field: error
    // value: failed to process {{{ _ingest.on_failure_processor_type }}}
    // "#;
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({
    // message: r#"{ "key: "value" }"#
    // }))?
    // .target;
    //
    // assert_eq!(
    // value!({
    // message: r#"{ "key: "value" }"#,
    // error: "failed to process json"
    // }),
    // target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional_positive() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // if: ctx.message instanceof String
    // field: message
    // "#;
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({
    // message: r#"{ "key": "value" }"#
    // }))?
    // .target;
    //
    // assert_eq!(
    // value!({
    // message: { key: "value" }
    // }),
    // target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional_negative() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // if: ctx.message instanceof String
    // field: message
    // "#;
    //
    // let target = Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(value!({
    // message: 42
    // }))
    // .unwrap()
    // .target;
    //
    // assert_eq!(
    // value!({
    // message: 42
    // }),
    // target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn target_field() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // field: message
    // target_field: json
    // "#;
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({
    // message: r#"{ "key": "value" }"#
    // }))?
    // .target;
    //
    // assert_eq!(
    // value!({
    // message: r#"{ "key": "value" }"#,
    // json: { key: "value" }
    // }),
    // target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_failure_true() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - json:
    // field: message
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ message: r#"{ "key": "value }"# }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({
    // message: r#"{ "key": "value }"#
    // }))?
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
    // - json:
    // field: message
    // ignore_failure: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({
    // message: r#"{ "key": "value }"#
    // }))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains("unable to parse json"));
    //
    // Ok(())
    // }
    // }
}
