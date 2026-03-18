use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::pipeline::{Validate, conditional::Conditional, on_failure::OnFailure};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IntoType {
    Integer,
    Long,
    Float,
    String,
    Boolean,
    IP,
    Auto,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Convert {
    pub field: String,
    #[serde(alias = "type")]
    pub into_type: IntoType,
    pub target_field: Option<String>,
    pub ignore_missing: Option<bool>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub ignore_failure: Option<bool>,
    pub tag: Option<String>,
    pub on_failure: Option<OnFailure>,
}

impl Validate for Convert {
    #[instrument(name = "Convert::validate", skip_all, err)]
    fn validate(&self) -> Result<()> {
        ensure!(
            matches!(
                self.into_type,
                IntoType::IP
                    | IntoType::String
                    | IntoType::Integer
                    | IntoType::Long
                    | IntoType::Float
            ),
            "{:?} not supported, expected (ip | string | integer | float)",
            self.into_type
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::{Pipeline, Processor, processors::convert::IntoType};

        use pretty_assertions::assert_eq;

        #[test]
        fn ip() {
            let config = r#"
                processors:
                    - convert:
                        field: source.address
                        type: ip
            "#;

            match &Pipeline::parse(config).unwrap().processors[..] {
                [Processor::Convert(convert)] => {
                    assert_eq!(convert.field, "source.address");
                    assert_eq!(convert.into_type, IntoType::IP);
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        #[test]
        fn string() {
            let config = r#"
                processors:
                    - convert:
                        field: source.address
                        type: string
            "#;

            match &Pipeline::parse(config).unwrap().processors[..] {
                [Processor::Convert(convert)] => {
                    assert_eq!(convert.field, "source.address");
                    assert_eq!(convert.into_type, IntoType::String);
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        #[test]
        fn into_type_unsupported() {
            let config = r#"
                processors:
                    - convert:
                        field: source.address
                        type: boolean
            "#;

            assert_eq!(
                "Boolean not supported, expected (ip | string | integer | float)",
                Pipeline::parse(config)
                    .unwrap_err()
                    .root_cause()
                    .to_string()
            );
        }
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::{pipeline::Pipeline, test_utils};
    // use pretty_assertions::assert_eq;
    //
    // #[test]
    // fn ip() {
    // let config = r#"
    // processors:
    // - convert:
    // field: source.address
    // type: ip
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ source: { address: "192.168.0.1" }}),
    // Pipeline::parse(config)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ source: { address: "192.168.0.1" }}))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ip_failure() {
    // let config = r#"
    // processors:
    // - convert:
    // field: source.address
    // type: ip
    // "#;
    //
    // assert!(Pipeline::parse(config)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ source: { address: "192.168.1" }}))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains("Failed converting .source.address to an IP address"));
    // }
    //
    // #[test]
    // fn string() {
    // let config = r#"
    // processors:
    // - convert:
    // field: source.port
    // type: string
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ source: { port: "80" }}),
    // Pipeline::parse(config)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ source: { port: 80 }}))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn target_field() {
    // let config = r#"
    // processors:
    // - convert:
    // field: source.port
    // type: string
    // target_field: port
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ source: { port: 80 }, port: "80" }),
    // Pipeline::parse(config)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ source: { port: 80 }}))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_missing_false() {
    // let config = r#"
    // processors:
    // - convert:
    // field: source.port
    // type: string
    // ignore_missing: false
    // "#;
    //
    // assert!(Pipeline::parse(config)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ source: { address: "192.168.0.1" }}))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains("source.port must be present"));
    // }
    //
    // #[test]
    // fn ignore_missing_true() {
    // let config = r#"
    // processors:
    // - convert:
    // field: source.port
    // type: string
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ source: { address: "192.168.0.1" } }),
    // Pipeline::parse(config)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ source: { address: "192.168.0.1" }}))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_failure_missing() {
    // let config = r#"
    // processors:
    // - convert:
    // field: source.port
    // type: string
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ source: { address: "192.168.0.1" } }),
    // Pipeline::parse(config)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ source: { address: "192.168.0.1" }}))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_failure_false() {
    // let config = r#"
    // processors:
    // - convert:
    // field: source.address
    // type: ip
    // ignore_failure: false
    // "#;
    //
    // assert!(Pipeline::parse(config)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ source: { address: "192.168.1" }}))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains("Failed converting .source.address to an IP address"));
    // }
    //
    // #[test]
    // fn ignore_failure_true() {
    // let config = r#"
    // processors:
    // - convert:
    // field: source.address
    // type: string
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ source: { address: "192.168.1" } }),
    // Pipeline::parse(config)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ source: { address: "192.168.1" }}))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn conditional() {
    // let config = r#"
    // processors:
    // - convert:
    // field: azure.activitylogs.properties.result
    // target_field: event.outcome
    // type: string
    // if: "ctx?.event?.outcome == null && ctx?.azure?.activitylogs?.properties?.result != null && ctx?.azure?.activitylogs?.properties?.result instanceof String && ['success', 'failure', 'unknown'].contains(ctx.azure?.activitylogs?.properties?.result)"
    // "#;
    //
    // assert_eq!(
    // vrl::value!(vrl::value!({
    // event: { outcome: "success" },
    // azure: {
    // activitylogs: {
    // properties: {
    // result: "success"
    // }
    // }
    // }
    // })),
    // Pipeline::parse(config)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // azure: {
    // activitylogs: {
    // properties: {
    // result: "success"
    // }
    // }
    // }
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn long() {
    // let config = r#"
    // processors:
    // - convert:
    // field: value
    // type: long
    // "#;
    //
    // for (input, output) in [
    // (vrl::value!({ value: "1234567890" }), vrl::value!({ value: 1234567890 })),
    // (vrl::value!({ value: "0123456789" }), vrl::value!({ value: 123456789 })),
    // (vrl::value!({ value: "0x20" }), vrl::value!({ value: 32 })),
    // ] {
    // assert_eq!(
    // output,
    // Pipeline::parse(config)
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
    // fn float() {
    // let config = r#"
    // processors:
    // - convert:
    // field: value
    // type: float
    // "#;
    //
    // for (input, output) in [
    // (vrl::value!({ value: "123.45" }), vrl::value!({ value: 123.45 })),
    // (vrl::value!({ value: "0.5" }), vrl::value!({ value: 0.5 })),
    // (vrl::value!({ value: "1e-3" }), vrl::value!({ value: 0.001 })),
    // ] {
    // assert_eq!(
    // output,
    // Pipeline::parse(config)
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
    // fn on_failure() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - convert:
    // tag: example_tag
    // field: source.address
    // type: integer
    // on_failure:
    // - remove:
    // field: message
    // - append:
    // field: error.message
    // value: 'Processor {{_ingest.on_failure_processor_tag}} failed'
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ error: { message: ["Processor example_tag failed"] } }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message:  "192.168.0.1" }))
    // .unwrap()
    // .target,
    // );
    //
    // Ok(())
    // }
    // }
}
