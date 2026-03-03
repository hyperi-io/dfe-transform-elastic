
use anyhow::bail;
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    conditional::Conditional, on_failure::OnFailure, unsupported_fields,
    Validate,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct NetworkDirection {
    pub internal_networks_field: Option<String>,
    pub ignore_missing: Option<bool>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,

    // Unsupported fields
    pub destination_ip: Option<String>,
    pub ignore_failure: Option<bool>,
    pub internal_networks: Option<Vec<String>>,
    pub on_failure: Option<OnFailure>,
    pub source_ip: Option<String>,
    pub tag: Option<String>,
    pub target_field: Option<String>,
}

impl Validate for NetworkDirection {
    #[instrument(name = "NetworkDirection::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!(
            "network_direction",
            self,
            destination_ip,
            ignore_failure,
            internal_networks,
            on_failure,
            source_ip,
            tag,
            target_field
        );

        if self.internal_networks.is_none() && self.internal_networks_field.is_none() {
            bail!("Either internal_networks or internal_networks_field must be specified");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod validate {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "network_direction",
            r#"
                processors:
                    - network_direction:
                        internal_networks_fields: internal_networks
                        {}: {}
            "#,
            destination_ip => "destination.ip",
            ignore_failure => "true",
            internal_networks => "[private]",
            on_failure => "[]",
            source_ip => "source.ip",
            tag => "tag",
            target_field => "target"
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
    // fn internal_networks_field() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - network_direction:
    // internal_networks_field: internal_networks
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({
    // source: {
    // ip: "128.232.110.120"
    // },
    // destination: {
    // ip: "192.168.1.1"
    // },
    // internal_networks: ["private"]
    // }),
    // vrl::value!({
    // source: {
    // ip: "128.232.110.120"
    // },
    // destination: {
    // ip: "192.168.1.1"
    // },
    // network: {
    // direction: "inbound"
    // },
    // internal_networks: ["private"]
    // }),
    // ),
    // (
    // vrl::value!({
    // source: {
    // ip: "192.168.1.1"
    // },
    // destination: {
    // ip: "128.232.110.120"
    // },
    // internal_networks: ["private"]
    // }),
    // vrl::value!({
    // source: {
    // ip: "192.168.1.1"
    // },
    // destination: {
    // ip: "128.232.110.120"
    // },
    // network: {
    // direction: "outbound"
    // },
    // internal_networks: ["private"]
    // }),
    // ),
    // (
    // vrl::value!({
    // source: {
    // ip: "192.168.1.1"
    // },
    // destination: {
    // ip: "192.168.1.2"
    // },
    // internal_networks: ["private"]
    // }),
    // vrl::value!({
    // source: {
    // ip: "192.168.1.1"
    // },
    // destination: {
    // ip: "192.168.1.2"
    // },
    // network: {
    // direction: "internal"
    // },
    // internal_networks: ["private"]
    // }),
    // ),
    // (
    // vrl::value!({
    // source: {
    // ip: "128.232.110.121"
    // },
    // destination: {
    // ip: "128.232.110.120"
    // },
    // internal_networks: ["private"]
    // }),
    // vrl::value!({
    // source: {
    // ip: "128.232.110.121"
    // },
    // destination: {
    // ip: "128.232.110.120"
    // },
    // network: {
    // direction: "external"
    // },
    // internal_networks: ["private"]
    // }),
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
    // fn ignore_missing() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - network_direction:
    // ignore_missing: true
    // internal_networks_field: internal_networks
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({
    // source: {
    // ip: "128.232.110.120"
    // },
    // internal_networks: ["private"]
    // }),
    // vrl::value!({
    // source: {
    // ip: "128.232.110.120"
    // },
    // internal_networks: ["private"]
    // }),
    // ),
    // (
    // vrl::value!({
    // destination: {
    // ip: "128.232.110.120"
    // },
    // internal_networks: ["private"]
    // }),
    // vrl::value!({
    // destination: {
    // ip: "128.232.110.120"
    // },
    // internal_networks: ["private"]
    // }),
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
    // fn conditional() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - network_direction:
    // conditional: ctx?.network_direction == true
    // internal_networks_field: internal_networks
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({
    // destination: {
    // ip: "128.232.110.120"
    // },
    // source: {
    // ip: "128.232.110.120"
    // },
    // internal_networks: ["private"]
    // }),
    // vrl::value!({
    // destination: {
    // ip: "128.232.110.120"
    // },
    // source: {
    // ip: "128.232.110.120"
    // },
    // internal_networks: ["private"]
    // }),
    // ),
    // (
    // vrl::value!({
    // destination: {
    // ip: "128.232.110.120"
    // },
    // source: {
    // ip: "128.232.110.120"
    // },
    // network_direction: true,
    // internal_networks: ["private"]
    // }),
    // vrl::value!({
    // destination: {
    // ip: "128.232.110.120"
    // },
    // source: {
    // ip: "128.232.110.120"
    // },
    // network_direction: true,
    // network: {
    // direction: "external"
    // },
    // internal_networks: ["private"]
    // }),
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
    // }
}
