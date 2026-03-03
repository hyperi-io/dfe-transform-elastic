
use anyhow::Result;
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    conditional::Conditional, on_failure::OnFailure, unsupported_fields,
    Validate,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CommunityId {
    pub source_ip: Option<String>,
    pub source_port: Option<String>,
    pub destination_ip: Option<String>,
    pub destination_port: Option<String>,
    pub iana_number: Option<String>,
    pub icmp_type: Option<String>,
    pub icmp_code: Option<String>,
    pub transport: Option<String>,
    pub target_field: Option<String>,
    pub seed: Option<u16>,
    pub ignore_missing: Option<bool>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub ignore_failure: Option<bool>,
    pub on_failure: Option<OnFailure>,
    pub tag: Option<String>,
}

impl Validate for CommunityId {
    #[instrument(name = "CommunityId::validate", skip_all, err)]
    fn validate(&self) -> Result<()> {
        unsupported_fields!(
            "community_id",
            self,
            iana_number,
            icmp_type,
            icmp_code,
            transport,
            seed,
            conditional,
            on_failure,
            tag
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod validate {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "community_id",
            r#"
                processors:
                    - community_id:
                        {}: {}
            "#,
            iana_number => "network.iana_number",
            icmp_type => "icmp.type",
            icmp_code => "icmp.code",
            transport => "network.transport",
            seed => "1",
            conditional => "true && false",
            on_failure => "[]",
            tag => "some_tag"
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use anyhow::Result;
    // use pretty_assertions::assert_eq;
    //
    // use crate::{pipeline::Pipeline, test_utils};
    //
    // #[test]
    // pub fn happy_path() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - community_id: {}
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // community_id: "1:9qr9Z1LViXcNwtLVOHZ3CL8MlyM=",
    // transport: "TCP"
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // transport: "TCP"
    // }
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn iana_number() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - community_id: {}
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // community_id: "1:9qr9Z1LViXcNwtLVOHZ3CL8MlyM=",
    // iana_number: 6
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // iana_number: 6
    // }
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn iana_number_in_transport_for_some_stupid_fucking_reason() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - community_id: {}
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // "source": {
    // "ip": "172.17.3.4"
    // },
    // "destination": {
    // "ip": "67.43.156.12"
    // },
    // "network": {
    // "community_id": "1:7y0Rtnc087ycVA+d/fCa/8i5fTo=",
    // "transport": "1"
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // "source": {
    // "ip": "172.17.3.4"
    // },
    // "destination": {
    // "ip": "67.43.156.12"
    // },
    // "network": {
    // "transport": "1"
    // }
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn ignore_missing_true() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - community_id:
    // ignore_missing: true
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({
    // source: {
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // transport: "TCP"
    // }
    // }),
    // vrl::value!({
    // source: {
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // transport: "TCP"
    // }
    // }),
    // ),
    // (
    // vrl::value!({
    // source: {
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // iana_number: 6
    // }
    // }),
    // vrl::value!({
    // source: {
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // iana_number: 6
    // }
    // }),
    // ),
    // (
    // vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // }),
    // vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // }),
    // ),
    // (
    // vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // iana_number: 6
    // }
    // }),
    // vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // community_id: "1:9qr9Z1LViXcNwtLVOHZ3CL8MlyM=",
    // iana_number: 6
    // }
    // }),
    // ),
    // (
    // vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // community_id: "1:9qr9Z1LViXcNwtLVOHZ3CL8MlyM=",
    // transport: "TCP"
    // }
    // }),
    // vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // community_id: "1:9qr9Z1LViXcNwtLVOHZ3CL8MlyM=",
    // transport: "TCP"
    // }
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
    // pub fn source_and_destination() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - community_id:
    // source_ip: source.nat.ip
    // source_port: source.nat.port
    // destination_ip: destination.nat.ip
    // destination_port: destination.nat.port
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // source: {
    // nat: {
    // ip: "123.124.125.126",
    // port: 12345
    // }
    // },
    // destination: {
    // nat: {
    // ip: "55.56.57.58",
    // port: 80
    // }
    // },
    // network: {
    // community_id: "1:9qr9Z1LViXcNwtLVOHZ3CL8MlyM=",
    // transport: "TCP"
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // source: {
    // nat: {
    // ip: "123.124.125.126",
    // port: 12345
    // }
    // },
    // destination: {
    // nat: {
    // ip: "55.56.57.58",
    // port: 80
    // }
    // },
    // network: {
    // transport: "TCP"
    // }
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn target_field() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - community_id:
    // target_field: community_id
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // community_id: "1:9qr9Z1LViXcNwtLVOHZ3CL8MlyM=",
    // network: {
    // transport: "TCP"
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // transport: "TCP"
    // }
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn ignore_failure() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - community_id:
    // ignore_failure: true
    // "#;
    //
    // for (input, output) in [
    // // Missing field
    // (
    // vrl::value!({
    // source: {
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // transport: "TCP"
    // }
    // }),
    // vrl::value!({
    // source: {
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // transport: "TCP"
    // }
    // }),
    // ),
    // // Unsupported transport
    // (
    // vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // transport: "FOO"
    // }
    // }),
    // vrl::value!({
    // source: {
    // ip: "123.124.125.126",
    // port: 12345
    // },
    // destination: {
    // ip: "55.56.57.58",
    // port: 80
    // },
    // network: {
    // transport: "FOO"
    // }
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
