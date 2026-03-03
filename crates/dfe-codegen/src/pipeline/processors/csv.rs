
use anyhow::Result;
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    conditional::Conditional, on_failure::OnFailure, unsupported_fields,
    Validate,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Csv {
    pub field: String,
    pub target_fields: Vec<String>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,

    pub separator: Option<String>,
    pub quote: Option<String>,
    pub ignore_missing: Option<bool>,
    pub ignore_failure: Option<bool>,
    pub on_failure: Option<OnFailure>,
    pub tag: Option<String>,
    pub description: Option<String>,

    // Unsupported fields
    pub trim: Option<bool>,
    pub empty_value: Option<String>,
}

impl Validate for Csv {
    #[instrument(name = "Csv::validate", skip_all, err)]
    fn validate(&self) -> Result<()> {
        unsupported_fields!(
            "csv",
            self,
            trim,
            empty_value
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod validate {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "csv",
            r#"
                processors:
                    - csv:
                        field: message
                        target_fields:
                            - foo
                            - bar
                        {}: {}
            "#,
            trim => "true",
            empty_value => "empty"
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
    // use anyhow::Result;
    //
    // #[test]
    // fn happy_path() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - csv:
    // field: message
    // target_fields: ["field1", "field2", "field3"]
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "foo,bar,baz",
    // field1: "foo",
    // field2: "bar",
    // field3: "baz"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo,bar,baz"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn correctly_handle_weird_spacing() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - csv:
    // field: message
    // target_fields:
    // - event.id
    // - cisco.umbrella._tmp.time
    // - user.email
    // - user.name
    // - cisco.umbrella.audit.type
    // - event.action
    // - source.address
    // - cisco.umbrella.audit.before
    // - cisco.umbrella.audit.after
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // "message": "\"1234567890\",\"2021-07-22 10:46:45\",\"user@domain.com\",\"user\", \"logexportconfigurations\", \"update\",\"81.2.69.144\",\"version: 4\",\"version: 5\"",
    // event: {
    // id: "1234567890",
    // action: "update",
    // },
    // user: {
    // email: "user@domain.com",
    // name: "user"
    // },
    // source: {
    // address: "81.2.69.144"
    // },
    // cisco: {
    // umbrella: {
    // _tmp: {
    // time: "2021-07-22 10:46:45"
    // },
    // audit: {
    // type: "logexportconfigurations",
    // before: "version: 4",
    // after: "version: 5"
    // }
    // }
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!(
    // { "message": "\"1234567890\",\"2021-07-22 10:46:45\",\"user@domain.com\",\"user\", \"logexportconfigurations\", \"update\",\"81.2.69.144\",\"version: 4\",\"version: 5\"" }
    // ))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn correctly_handle_weird_spacing_with_empty_values() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - csv:
    // field: message
    // target_fields:
    // - cisco.umbrella._tmp.time
    // - cisco.umbrella.origin_id
    // - cisco.umbrella.identities
    // - cisco.umbrella.identity_types
    // - cisco.umbrella.direction
    // - network.transport
    // - source.bytes
    // - source.address
    // - source.port
    // - destination.address
    // - destination.port
    // - cisco.umbrella.datacenter
    // - rule.id
    // - cisco.umbrella.action
    // - cisco.umbrella.fqdns
    // - cisco.umbrella.destination_lists_id
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // "message": "\"2019-01-14 18:03:46\",\"[211039844]\",\"Passive Monitor\", \"CDFW Tunnel Device\",\"OUTBOUND\",\"1\",\"84\",\"172.17.3.4\",\"\",\"67.43.156.12\", \"\",\"ams1.edc\",\"12\",\"ALLOW\",\"google.com,apple.com\",\"44,66\"",
    // cisco: {
    // umbrella: {
    // _tmp: {
    // time: "2019-01-14 18:03:46"
    // },
    // origin_id: "[211039844]",
    // identities: "Passive Monitor",
    // identity_types: "CDFW Tunnel Device",
    // direction: "OUTBOUND",
    // datacenter: "ams1.edc",
    // action: "ALLOW",
    // fqdns: "google.com,apple.com",
    // destination_lists_id: "44,66"
    // }
    // },
    // network: {
    // transport: "1"
    // },
    // source: {
    // bytes: "84",
    // address: "172.17.3.4",
    // },
    // destination: {
    // address: "67.43.156.12",
    // },
    // rule: {
    // id: "12"
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!(
    // { "message": "\"2019-01-14 18:03:46\",\"[211039844]\",\"Passive Monitor\", \"CDFW Tunnel Device\",\"OUTBOUND\",\"1\",\"84\",\"172.17.3.4\",\"\",\"67.43.156.12\", \"\",\"ams1.edc\",\"12\",\"ALLOW\",\"google.com,apple.com\",\"44,66\"" }
    // ))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn nested_commas_with_weird_spacing() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - csv:
    // field: message
    // target_fields:
    // - cisco.umbrella._tmp.time
    // - cisco.umbrella.identity
    // - cisco.umbrella.identities
    // - source.address
    // - source.nat.ip
    // - cisco.umbrella.action
    // - dns.question.type
    // - dns.response_code
    // - dns.question.name
    // - cisco.umbrella.categories
    // - cisco.umbrella.policy_identity_type
    // - cisco.umbrella.identity_types
    // - cisco.umbrella.blocked_categories
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // "message": "\"2015-01-16 17:48:41\",\"ActiveDirectoryUserName\", \"ActiveDirectoryUserName,ADSite,Network\", \"10.10.1.100\",\"81.2.69.144\",\"Allowed\",\"1 (A)\", \"NOERROR\",\"domain-visited.com.\", \"Chat,Photo Sharing,Social Networking,Allow List\"",
    // cisco: {
    // umbrella: {
    // _tmp: {
    // time: "2015-01-16 17:48:41"
    // },
    // identity: "ActiveDirectoryUserName",
    // identities: "ActiveDirectoryUserName,ADSite,Network",
    // action: "Allowed",
    // categories: "Chat,Photo Sharing,Social Networking,Allow List",
    // }
    // },
    // source: {
    // address: "10.10.1.100",
    // nat: {
    // ip: "81.2.69.144"
    // }
    // },
    // dns: {
    // question: {
    // type: "1 (A)",
    // name: "domain-visited.com."
    // },
    // response_code: "NOERROR"
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!(
    // { "message": "\"2015-01-16 17:48:41\",\"ActiveDirectoryUserName\", \"ActiveDirectoryUserName,ADSite,Network\", \"10.10.1.100\",\"81.2.69.144\",\"Allowed\",\"1 (A)\", \"NOERROR\",\"domain-visited.com.\", \"Chat,Photo Sharing,Social Networking,Allow List\"" }
    // ))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn empty_value_null() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - csv:
    // field: message
    // target_fields: ["field1", "field2", "field3"]
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "foo,\"\",baz",
    // field1: "foo",
    // field3: "baz"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo,\"\",baz"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional_true() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - csv:
    // if: 'ctx.message_type == "csv"'
    // field: message
    // target_fields: ["field1", "field2", "field3"]
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "foo,bar,baz",
    // message_type: "csv",
    // field1: "foo",
    // field2: "bar",
    // field3: "baz"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo,bar,baz",
    // message_type: "csv"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional_false() -> Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - csv:
    // if: 'ctx.message_type == "csv"'
    // field: message
    // target_fields: ["field1", "field2", "field3"]
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "foo,bar,baz",
    // message_type: "not_csv"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo,bar,baz",
    // message_type: "not_csv"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    // }
}
