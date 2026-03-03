
use lazy_static::lazy_static;
use regex::{Regex, Replacer};
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    conditional::Conditional,
    on_failure::OnFailure,
    unsupported_fields, Validate,
};

// https://www.elastic.co/guide/en/elasticsearch/reference/8.13/dissect-processor.html
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Dissect {
    pub field: String,
    pub pattern: String,
    #[serde(rename = "if")]
    pub conditional: Option<Conditional>,
    pub ignore_failure: Option<bool>,
    pub ignore_missing: Option<bool>,
    pub on_failure: Option<OnFailure>,
    pub tag: Option<String>,

    // Unsupported fields
    pub append_separator: Option<String>,
}

impl Validate for Dissect {
    #[instrument(name = "Dissect::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!("dissect", self, append_separator);

        Ok(())
    }
}

lazy_static! {
    static ref DISSECTION_PATTERN: Regex = Regex::new(
        r#"%\{(?P<append>\+)?(?P<reference_key>[&*])?(?P<path>.*?)(?:(?P<right_pad>->}(?P<right_pad_char>.)?)|})"#
    )
    .unwrap();
    static ref ESCAPE_CHARACTERS_PATTERN: Regex = Regex::new(r#"[\[\]\(\)\\']"#).unwrap();
}

pub struct DissectionPatternReplacer;

impl Replacer for DissectionPatternReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        // https://www.elastic.co/guide/en/elasticsearch/reference/current/dissect-processor.html
        // DISSECT doesn't use regex, but its behaviour most closely matches lazy regex matching
        // DATA comes from the grok pattern library and is lazy
        dst.push_str("%{DATA");

        // Need to temporarily replace path dots with the unicode • so we can escape other dots later
        // This is required because Rust regex doesn't support lookarounds negative or otherwise
        let path = caps.name("path").unwrap().as_str().to_string().replace('.', "•");
        if !path.is_empty() {
            dst.push(':');
            dst.push_str(&path);
        }

        dst.push('}');

        if caps.name("right_pad").is_some() && caps.name("right_pad_char").is_some() {
            dst.push_str(caps.name("right_pad_char").unwrap().as_str());
            dst.push('+');
        }
    }
}

pub struct EscapeCharactersReplacer;

impl Replacer for EscapeCharactersReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        dst.push('\\');
        dst.push_str(caps.get(0).unwrap().as_str());
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "dissect",
            r#"
                processors:
                    - dissect:
                        field: message
                        pattern: "${{network.transport}} connection"
                        {}: {}
            "#,
            append_separator => "\", \""
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::{
    // pipeline::{pipeline::TranspileCtx, Pipeline},
    // test_utils::{self, assert_eq},
    // };
    //
    // #[test]
    // fn simple() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "Hello, %{subject}"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Hello, World",
    // subject: "World"
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Hello, World"
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn simple_empty() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "Hello, %{}"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Hello, World",
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Hello, World"
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn complex() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "%{network.direction} %{network.transport} connection %{event.outcome} from %{source.address}/%{source.port} to %{destination.address}/%{destination.port} flags %{} on interface %{_temp_.cisco.source_interface}"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "OUTBOUND TCP connection SUCCESS from 192.168.0.1/22 to 127.0.0.1/22 flags DO NOT REPLY on interface eth0",
    // network: {
    // direction: "OUTBOUND",
    // transport: "TCP"
    // },
    // event: {
    // outcome: "SUCCESS"
    // },
    // source: {
    // address: "192.168.0.1",
    // port: "22"
    // },
    // destination: {
    // address: "127.0.0.1",
    // port: "22"
    // },
    // _temp_: {
    // cisco: {
    // source_interface: "eth0"
    // }
    // }
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "OUTBOUND TCP connection SUCCESS from 192.168.0.1/22 to 127.0.0.1/22 flags DO NOT REPLY on interface eth0"
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn conditional_true() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "Hello, %{subject}"
    // if: ctx?.message_id == '101'
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Hello, World",
    // subject: "World",
    // message_id: "101"
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Hello, World",
    // message_id: "101"
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn conditional_false() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "Hello, %{subject}"
    // if: ctx?.message_id == '101'
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Hello, World",
    // message_id: "102"
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Hello, World",
    // message_id: "102"
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_failure_true() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "Hello, %{subject}"
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Goodbye, World",
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Goodbye, World",
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_failure_false() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "Hello, %{subject}"
    // ignore_failure: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Goodbye, World",
    // }))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains("unable to parse grok: value does not match any rule"));
    // }
    //
    // #[test]
    // fn right_padding() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "%{greeting->} %{subject}"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Hello      World",
    // greeting: "Hello",
    // subject: "World",
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Hello      World",
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn right_padding_end() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "%{greeting}, %{subject->}"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Hello, World    ",
    // greeting: "Hello",
    // subject: "World    ",
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Hello, World    ",
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn append_modifier() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "%{greeting}, %{subject->} %{+subject}"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Hello, Outer     Worlds",
    // greeting: "Hello",
    // subject: "OuterWorlds",
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Hello, Outer     Worlds",
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn escape_parens() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "Offloaded %{network.transport} Flow for connection %{_temp_.cisco.connection_id} from %{_temp_.cisco.source_interface}:%{source.address}/%{source.port} (%{_temp_.natsrcip}/%{_temp_.cisco.mapped_source_port}) to %{_temp_.cisco.destination_interface}:%{destination.address}/%{destination.port} (%{_temp_.natdstip}/%{_temp_.cisco.mapped_destination_port})"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Offloaded TCP Flow for connection 111111111 from fw111:10.10.10.10/111 (175.16.199.1/111) to fw111:192.168.2.2/111 (8.8.5.4/111)",
    // _temp_: {
    // cisco: {
    // connection_id: "111111111",
    // destination_interface: "fw111",
    // mapped_destination_port: "111",
    // mapped_source_port: "111",
    // source_interface: "fw111",
    // },
    // natdstip: "8.8.5.4",
    // natsrcip: "175.16.199.1",
    // },
    // destination: {
    // address: "192.168.2.2",
    // port: "111",
    // },
    // network: {
    // transport: "TCP",
    // },
    // source: {
    // address: "10.10.10.10",
    // port: "111",
    // }
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Offloaded TCP Flow for connection 111111111 from fw111:10.10.10.10/111 (175.16.199.1/111) to fw111:192.168.2.2/111 (8.8.5.4/111)",
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn lazy_capture() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "Teardown stub %{network.transport} connection for %{_temp_.cisco.source_interface}:%{source.address}/%{source.port} to %{_temp_.cisco.destination_interface}:%{destination.address}/%{destination.port} duration %{_temp_.duration_hms} forwarded bytes %{network.bytes} %{event.reason}"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Teardown stub TCP connection for fw111:10.10.10.10/39210 to net:192.168.2.2/10051 duration 0:00:00 forwarded bytes 0 Cluster flow with CLU closed on owner",
    // event: {
    // reason: "Cluster flow with CLU closed on owner"
    // },
    // network: {
    // transport: "TCP",
    // bytes: "0"
    // },
    // _temp_: {
    // cisco: {
    // source_interface: "fw111",
    // destination_interface: "net"
    // },
    // duration_hms: "0:00:00"
    // },
    // source: {
    // address: "10.10.10.10",
    // port: "39210"
    // },
    // destination: {
    // address: "192.168.2.2",
    // port: "10051"
    // }
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Teardown stub TCP connection for fw111:10.10.10.10/39210 to net:192.168.2.2/10051 duration 0:00:00 forwarded bytes 0 Cluster flow with CLU closed on owner",
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn escape_full_stops() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "IP = %{source.address}, %{event.reason}. %{} packet."
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "IP = 1.128.3.4, Duplicate first packet detected. Ignoring packet.",
    // source: {
    // address: "1.128.3.4"
    // },
    // event: {
    // reason: "Duplicate first packet detected"
    // }
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "IP = 1.128.3.4, Duplicate first packet detected. Ignoring packet.",
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn right_padding_missing() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "DAP: User %{user.email}, Addr %{source.address}, Connection %{_temp_.cisco.connection_type}: The following DAP records were selected for this connection: %{_temp_.cisco.dap_records->}"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "DAP: User firstname.lastname@domain.net, Addr 81.2.69.143, Connection AnyConnect: The following DAP records were selected for this connection: dap_1, dap_2",
    // user: {
    // email: "firstname.lastname@domain.net",
    // },
    // source: {
    // address: "81.2.69.143"
    // },
    // _temp_: {
    // cisco: {
    // connection_type: "AnyConnect",
    // dap_records: "dap_1, dap_2"
    // }
    // }
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "DAP: User firstname.lastname@domain.net, Addr 81.2.69.143, Connection AnyConnect: The following DAP records were selected for this connection: dap_1, dap_2",
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_missing_true() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: other
    // pattern: "Hello, %{subject}"
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Goodbye, World",
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Goodbye, World",
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_missing_false() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: other
    // pattern: "Hello, %{subject}"
    // ignore_missing: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "Goodbye, World",
    // }))
    // .err()
    // .ok_or_else(|| anyhow::anyhow!("failed to fail"))?
    // .root_cause()
    // .to_string()
    // .contains(".other must not be nullish"));
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn escape_square_brackets() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // tag: dissect_login
    // pattern: "%{cisco.ios.action} %{_temp_.event.action} [user: %{source.user.name}] [Source: %{source.address}] [localport: %{destination.port}] at %{}"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Login Success [user: username] [Source: 192.168.0.1] [localport: 22] at 07:57:58 VIENNA Thu Jan 27 2022",
    // cisco: {
    // ios: {
    // action: "Login",
    // }
    // },
    // _temp_: {
    // event: {
    // action: "Success"
    // }
    // },
    // source: {
    // user: {
    // name: "username"
    // },
    // address: "192.168.0.1"
    // },
    // destination: {
    // port: "22"
    // }
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Login Success [user: username] [Source: 192.168.0.1] [localport: 22] at 07:57:58 VIENNA Thu Jan 27 2022"
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn partial_dissection_match() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // tag: dissect_login
    // pattern: "list %{cisco.ios.access_list} %{_temp_.event.action} %{network.iana_number} %{source.address} %{} %{destination.address}, %{source.packets} packet"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // "_temp_": {
    // "event": {
    // "action": "denied"
    // }
    // },
    // "destination": {
    // "address": "224.0.0.18"
    // },
    // "source": {
    // "packets": "295",
    // "address": "89.160.20.112"
    // },
    // "message": "list ACL_CE-SECURITY denied 112 89.160.20.112 -> 224.0.0.18, 295 packets",
    // "cisco": {
    // "ios": {
    // "access_list": "ACL_CE-SECURITY"
    // }
    // },
    // "network": {
    // "iana_number": "112"
    // }
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "list ACL_CE-SECURITY denied 112 89.160.20.112 -> 224.0.0.18, 295 packets"
    // }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn on_failure() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // field: message
    // pattern: "Hello, %{subject}"
    // tag: test_tag
    // ignore_failure: false
    // on_failure:
    // - append:
    // field: error.message
    // value: "Processor {{{_ingest.on_failure_processor_type}}} with tag {{{_ingest.on_failure_processor_tag}}} in pipeline {{{_ingest.pipeline}}} failed with message: {{{_ingest.on_failure_message}}}"
    // "#;
    //
    // assert!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile_with_context(&TranspileCtx::builder().with_name("test_pipeline").build())
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // message: "Goodbye, World",
    // }))
    // .unwrap()
    // .target
    // .get(".error.message[0]")
    // .unwrap()
    // .as_str()
    // .unwrap()
    // .to_string()
    // .contains("Processor grok with tag test_tag in pipeline test_pipeline failed with message: function call error for \"parse_groks\"")
    // );
    // }
    //
    // #[test]
    // fn escape_backslash() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - dissect:
    // if: ctx.user?.name != null && ctx.user.name.contains('\\')
    // tag: dissect_user_name
    // field: user.name
    // pattern: '%{user.domain}\%{user.name}'
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // user: {
    // domain: "mydomain",
    // name: "myuser",
    // }
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // user: {
    // name: "mydomain\\myuser",
    // }
    // }))
    // .unwrap()
    // .target
    // );
    // }
    // }
}
