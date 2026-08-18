use std::collections::HashMap;

use anyhow::{Result, anyhow};
use lazy_static::lazy_static;
use regex::{Regex, Replacer};
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    Validate, conditional::Conditional, on_failure::OnFailure, unsupported_fields,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ECSCompatibility {
    Disabled,
    V1,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Grok {
    pub field: String,
    pub patterns: Vec<String>,
    pub ignore_missing: Option<bool>,
    pub ignore_failure: Option<bool>,
    pub pattern_definitions: Option<HashMap<String, String>>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub on_failure: Option<OnFailure>,
    pub tag: Option<String>,
    pub ecs_compatibility: Option<ECSCompatibility>,

    // Unsupported fields
    pub trace_match: Option<bool>,
}

lazy_static! {
    static ref TYPE_COERCION_PATTERN: Regex =
        Regex::new("%\\{(?<alias>[^:}]+):(?<path>[^}:]+)(?::(?<type_coercion>[^}:]+))?}").unwrap();
    static ref UNGROUPED_UNION_PATTERN: Regex =
        Regex::new(r"^(?:[A-Za-z_]+\|)+[A-Za-z_]+$").unwrap();
    static ref UNGROUPED_OPTIONAL_GROK_PATTERN: Regex =
        Regex::new(r"(?<optional_grok>%\{[^}]+})\?").unwrap();
    static ref NAMED_PATH_CAPTURE_PATTERN: Regex =
        Regex::new(r"\?<(?<capture>[^>]+\.[^>]+)>").unwrap();
}

impl Validate for Grok {
    #[instrument(name = "Grok::validate", skip_all, err)]
    fn validate(&self) -> Result<()> {
        unsupported_fields!("grok", self, trace_match);

        if let Some(ECSCompatibility::V1) = self.ecs_compatibility {
            tracing::warn!("ECS v1 patterns may not be properly supported by codegen");
        }

        // Elastic's grok handles named paths in captures outside of pattern definitions
        // e.g. (?<user.email>%{DATA:user.name}@%{DATA:user.domain})
        // VRL ends up setting the path to "user.email" instead of "user"."email"
        self.all_patterns_iter()
            .flat_map(|pattern| NAMED_PATH_CAPTURE_PATTERN.captures_iter(pattern.as_str()))
            .filter_map(|capture| capture.name("capture"))
            .try_for_each(|path| {
                Err(anyhow!(
                    "grok codegen doesn't support named path captures outside of pattern definitions. Found: {}",
                    path.as_str()
                ))
            })?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TypeCoercion {
    Int,
    Long,
    String,
}

impl TryFrom<&str> for TypeCoercion {
    type Error = anyhow::Error;

    #[instrument(name = "TypeCoercion::try_from::<&str>", err)]
    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            "int" => Ok(TypeCoercion::Int),
            "long" => Ok(TypeCoercion::Long),
            "ip" => Ok(TypeCoercion::String),
            ty => Err(anyhow!("unsupported type coercion: {ty}")),
        }
    }
}

#[allow(dead_code)]
struct TypeCoercionReplacer;

impl Replacer for TypeCoercionReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        dst.push_str("%{");
        dst.push_str(&caps[1]);
        dst.push(':');
        dst.push_str(&caps[2]);
        dst.push('}');
    }
}

impl Grok {
    #[allow(dead_code)]
    fn all_patterns_iter_mut(&mut self) -> impl Iterator<Item = &mut String> {
        self.patterns.iter_mut().chain(
            self.pattern_definitions
                .as_mut()
                .into_iter()
                .flat_map(|inner| inner.values_mut()),
        )
    }

    fn all_patterns_iter(&self) -> impl Iterator<Item = &String> {
        self.patterns.iter().chain(
            self.pattern_definitions
                .as_ref()
                .into_iter()
                .flat_map(|inner| inner.values()),
        )
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::{Pipeline, unsupported_fields_tests};
        use anyhow::{Result, anyhow};

        #[test]
        fn nested_path_captures_outside_of_patterns() -> Result<()> {
            let configuration = r"
                processors:
                    - grok:
                        field: user.name
                        ignore_failure: true
                        patterns:
                            - ^%{GREEDYDATA:user.full_name} (\((?<user.email>%{DATA:user.name}@%{DATA:user.domain})\))?$
            ";

            assert_eq!(
                "grok codegen doesn't support named path captures outside of pattern definitions. Found: user.email",
                Pipeline::parse(configuration)
                    .err()
                    .unwrap_or_else(|| anyhow!("failed to fail"))
                    .root_cause()
                    .to_string()
            );

            Ok(())
        }

        unsupported_fields_tests!("grok",
            r#"
                processors:
                    - grok:
                        field: message
                        patterns: ["%{{TIMESTAMP_ISO8601:timestamp}} %{{LOGLEVEL:level}} %{{GREEDYDATA:message}}"]
                        {}: {}
            "#,
            trace_match => "false"
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::pipeline::{pipeline::TranspileCtx, Pipeline};
    // use anyhow::Result;
    // use pretty_assertions::assert_eq;
    //
    // #[test]
    // fn required_fields() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // patterns: ["%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"]
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ timestamp: "2021-01-01T00:00:00.000Z", level: "INFO", message: "Hello, world!" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "2021-01-01T00:00:00.000Z INFO Hello, world!"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_failure_true() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // patterns: ["%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"]
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ message: "Hello, World!" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "Hello, World!"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_failure_false() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // patterns: ["%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"]
    // ignore_failure: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "Hello, World!"
    // }))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains("value does not match any rule"));
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_missing_true() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // patterns: ["%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"]
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ another_key: "2021-01-01T00:00:00.000Z INFO Hello, world!" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ another_key: "2021-01-01T00:00:00.000Z INFO Hello, world!" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_missing_false() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // patterns: ["%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"]
    // ignore_missing: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({}))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains(".message must not be nullish"));
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn pattern_definitions() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: azure.resource_id
    // patterns:
    // - /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/NAMESPACES/%{NAMESPACE:azure.resource.namespace}/AUTHORIZATIONRULES/%{RULE:azure.resource.authorization_rule}
    // - /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}/providers/%{PROVIDERNAME:azure.resource.provider}/namespaces/%{NAMESPACE:azure.resource.namespace}/authorizationRules/%{RULE:azure.resource.authorization_rule}
    // pattern_definitions:
    // SUBID: (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}(\}){0,1}
    // GROUPID: .+
    // PROVIDERNAME: .+
    // NAMESPACE: .+
    // RULE: .+
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // azure: {
    // resource_id: "/SUBSCRIPTIONS/6c317256-1798-40e9-b6b7-5057eab729b8/RESOURCEGROUPS/group_id/PROVIDERS/provider/NAMESPACES/namespace/AUTHORIZATIONRULES/rule",
    // subscription_id: "6c317256-1798-40e9-b6b7-5057eab729b8",
    // resource: {
    // group: "group_id",
    // provider: "provider",
    // namespace: "namespace",
    // authorization_rule: "rule"
    // }
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(
    // vrl::value!({ azure: { resource_id: "/SUBSCRIPTIONS/6c317256-1798-40e9-b6b7-5057eab729b8/RESOURCEGROUPS/group_id/PROVIDERS/provider/NAMESPACES/namespace/AUTHORIZATIONRULES/rule" } })
    // )?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional_false() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // if: ctx.event?.message != null
    // field: event.message
    // patterns: ["%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"]
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ message: "hello" }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ message: "hello" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional_true() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // if: ctx.event?.message != null
    // field: event.message
    // patterns: ["%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"]
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // timestamp: "2021-01-01T00:00:00.000Z",
    // level: "INFO",
    // message: "Hello, world!",
    // event: {
    // message: "2021-01-01T00:00:00.000Z INFO Hello, world!",
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // event: {
    // message: "2021-01-01T00:00:00.000Z INFO Hello, world!",
    // }
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn substring_match() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: azure.resource_id
    // patterns:
    // - "/providers/%{PROVIDER:azure.resource.provider}"
    // pattern_definitions:
    // PROVIDER: .+
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // azure: {
    // resource_id: "/tenants/8a4de8b5-095c-47d0-a96f-a75130c61d53/providers/Microsoft.aadiam",
    // resource: {
    // provider: "Microsoft.aadiam"
    // }
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // azure: {
    // resource_id: "/tenants/8a4de8b5-095c-47d0-a96f-a75130c61d53/providers/Microsoft.aadiam",
    // }
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn grok_type_coercions() {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // patterns:
    // - \[%{IPORHOST:source.ip}\]:%{INT:source.port:int}
    // - "%{IPORHOST:source.ip}:%{INT:source.port:int}"
    // - "%{IPORHOST:source.ip}"
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({ message: "google.com:443" }),
    // vrl::value!({ message: "google.com:443", source: { ip: "google.com", port: 443 } }),
    // ),
    // (
    // vrl::value!({ message: "google.com" }),
    // vrl::value!({ message: "google.com", source: { ip: "google.com" } }),
    // ),
    // (
    // vrl::value!({ message: "192.168.0.1:443" }),
    // vrl::value!({ message: "192.168.0.1:443", source: { ip: "192.168.0.1", port: 443 } }),
    // ),
    // (
    // vrl::value!({ message: "192.168.0.1" }),
    // vrl::value!({ message: "192.168.0.1", source: { ip: "192.168.0.1" } }),
    // ),
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
    //
    // #[test]
    // fn grok_heterogeneous_type_coercions_failure() {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // patterns:
    // - \[%{IPORHOST:source.ip}\]:%{INT:source.port:int}
    // - "%{IPORHOST:source.ip}:%{INT:source.port}"
    // - "%{IPORHOST:source.ip}"
    // "#;
    //
    // assert_eq!(
    // "cannot support heterogeneous type coercions across grok patterns, found: Some(Int) & None",
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // );
    // }
    //
    // #[test]
    // fn grok_type_coercions_in_pattern_definitions() {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // patterns:
    // - \[%{IPORHOST:source.ip}\]:%{FOO_BAR_TEST_PATTERN}
    // - "%{IPORHOST:source.ip}:%{FOO_BAR_TEST_PATTERN}"
    // - "%{IPORHOST:source.ip}"
    // pattern_definitions:
    // FOO_BAR_TEST_PATTERN: "%{POSINT:source.port:long}"
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({ message: "google.com:443" }),
    // vrl::value!({ message: "google.com:443", source: { ip: "google.com", port: 443 } }),
    // ),
    // (
    // vrl::value!({ message: "google.com" }),
    // vrl::value!({ message: "google.com", source: { ip: "google.com" } }),
    // ),
    // (
    // vrl::value!({ message: "192.168.0.1:443" }),
    // vrl::value!({ message: "192.168.0.1:443", source: { ip: "192.168.0.1", port: 443 } }),
    // ),
    // (
    // vrl::value!({ message: "192.168.0.1" }),
    // vrl::value!({ message: "192.168.0.1", source: { ip: "192.168.0.1" } }),
    // ),
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
    //
    // #[test]
    // fn cisco() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: "message"
    // patterns:
    // - "%{NOTSPACE:event.outcome} %{NOTSPACE:network.direction} %{NOTSPACE:network.transport} src %{NOTSPACE:_temp_.cisco.source_interface}:%{NOTSPACE:source.address} (%{DATA})?dst %{NOTSPACE:_temp_.cisco.destination_interface}:%{DESTINATION_ADDRESS:destination.address}(%{GREEDYDATA})?"
    // pattern_definitions:
    // DESTINATION_ADDRESS: "[^ (]*"
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "Deny inbound icmp src fw111:10.10.10.10 dst fw111:10.10.10.10(type 8, code 0)",
    // event: {
    // outcome: "Deny",
    // },
    // network: {
    // direction: "inbound",
    // transport: "icmp",
    // },
    // _temp_: {
    // cisco: {
    // source_interface: "fw111",
    // destination_interface: "fw111",
    // }
    // },
    // source: {
    // address: "10.10.10.10",
    // },
    // destination: {
    // address: "10.10.10.10",
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "Deny inbound icmp src fw111:10.10.10.10 dst fw111:10.10.10.10(type 8, code 0)"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn on_failure() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // tag: test
    // patterns: ["%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"]
    // on_failure:
    // - append:
    // field: error.message
    // value: 'Processor {{{_ingest.on_failure_processor_type}}} with tag {{{_ingest.on_failure_processor_tag}}} in pipeline {{{_ingest.pipeline}}} failed with message: {{{_ingest.on_failure_message}}}'
    // "#;
    //
    // assert!(
    // Pipeline::parse(configuration)?
    // .transpile_with_context(&TranspileCtx::builder().with_name("grok_on_failure_test").build())?
    // .compile()?
    // .run(vrl::value!({
    // message: "Hello, World!"
    // }))?
    // .target
    // .get("error.message[0]")
    // .unwrap()
    // .as_str()
    // .unwrap()
    // .contains("Processor grok with tag test in pipeline grok_on_failure_test failed with message: function call error for \"parse_groks\" at")
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn cisco_nexus_trailing_newline() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // pattern_definitions:
    // NEXUS_TIMESTAMP_TIMEZONE: '%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}%{SPACE}%{WORD:cisco_nexus.log.timezone}'
    // NEXUS_TIMESTAMP: '%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}'
    // NEXUS_BODY: '(?s)(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description}'
    // patterns:
    // - '^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{NUMBER:cisco_nexus.log.sequence_number:long}:%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name}):%{SPACE}%{SYSLOGTIMESTAMP:temp.timestamp}:%{SPACE}%{NEXUS_BODY}$'
    // - '^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}%{NUMBER:cisco_nexus.log.sequence_number:long}:%{SPACE}%{SYSLOGTIMESTAMP:temp.syslog_timestamp}%{SPACE}%{WORD:cisco_nexus.log.timezone}:%{SPACE}%{NEXUS_BODY}$'
    // - '^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?::)?%{SPACE}%{NEXUS_TIMESTAMP_TIMEZONE:temp.timestamp}:%{SPACE}%{NEXUS_BODY}$'
    // - '^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:temp.syslog_timestamp}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?::)?%{SPACE}%{WORD:cisco_nexus.log.timezone}:%{SPACE}%{NEXUS_BODY}$'
    // - '^<%{NUMBER:cisco_nexus.log.priority_number:long}>(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name}):%{SPACE}%{NEXUS_TIMESTAMP_TIMEZONE:temp.timestamp}:%{SPACE}%{NEXUS_BODY}$'
    // - '^<%{NUMBER:cisco_nexus.log.priority_number:long}>:%{SPACE}%{NEXUS_TIMESTAMP_TIMEZONE:temp.timestamp}:%{SPACE}%{NEXUS_BODY}$'
    // - '^%{NEXUS_TIMESTAMP:temp.timestamp}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}%{NEXUS_BODY}$'
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "<186>switchname: 2023 Apr 26 09:08:48 UTC: %HMM-2-DUP_HOSTS:  hmm [23346]  (p0-i-mgmt-l3v534-vrf) [IPv4] Detected a potential duplicate host 10.208.112.147/32\n",
    // cisco_nexus: {
    // log: {
    // priority_number: 186,
    // switch_name: "switchname",
    // description: "  hmm [23346]  (p0-i-mgmt-l3v534-vrf) [IPv4] Detected a potential duplicate host 10.208.112.147/32",
    // facility: "HMM",
    // severity: 2,
    // timezone: "UTC",
    // "type": "DUP_HOSTS",
    // }
    // },
    // temp: {
    // timestamp: "2023 Apr 26 09:08:48 UTC",
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile_with_context(
    // &TranspileCtx::builder()
    // .with_name("grok_on_failure_test")
    // .build()
    // )?
    // .compile()?
    // .run(vrl::value!({
    // message: "<186>switchname: 2023 Apr 26 09:08:48 UTC: %HMM-2-DUP_HOSTS:  hmm [23346]  (p0-i-mgmt-l3v534-vrf) [IPv4] Detected a potential duplicate host 10.208.112.147/32\n"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ungrouped_union_pattern() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // patterns:
    // - "%{TYPE}( %{NOTSPACE:cisco_meraki.flows.op})? src=%{IP:source.ip:ip} dst=%{IP:destination.ip:ip}( mac=%{MAC:source.mac})? protocol=%{NOTSPACE:network.protocol}( type=%{NOTSPACE})?( sport=%{NONNEGINT:source.port:long})?( dport=%{NONNEGINT:destination.port:long})?( pattern: %{GREEDYDATA:cisco_meraki.firewall.pattern})?"
    // pattern_definitions:
    // TYPE: 'flows|firewall|vpn_firewall|cellular_firewall|bridge_anyconnect_client_vpn_firewall'
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // "destination": {
    // "port": 15600,
    // "ip": "10.0.0.34"
    // },
    // "source": {
    // "port": 54841,
    // "mac": "00:7C:2D:BD:76:F2",
    // "ip": "10.0.2.170"
    // },
    // "message": "flows allow src=10.0.2.170 dst=10.0.0.34 mac=00:7C:2D:BD:76:F2 protocol=udp sport=54841 dport=15600",
    // "cisco_meraki": {
    // "flows": {
    // "op": "allow"
    // }
    // },
    // "network": {
    // "protocol": "udp"
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile_with_context(
    // &TranspileCtx::builder()
    // .with_name("grok_on_failure_test")
    // .build()
    // )?
    // .compile()?
    // .run(vrl::value!({
    // message: "flows allow src=10.0.2.170 dst=10.0.0.34 mac=00:7C:2D:BD:76:F2 protocol=udp sport=54841 dport=15600"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ungrouped_optional_grok_patterns() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // tag: grok_header
    // patterns:
    // - '^%{CISCO_PRIORITY_MSGCOUNT}?%{SYSLOGTIMESTAMP} (?:%{IP}|%{CISCO_HOSTNAME:log.syslog.hostname}) %{NUMBER:cisco.ios.sequence}: (?:%{CISCO_UPTIME:cisco.ios.uptime}|%{CISCO_TIMESTAMP}): %{GREEDYDATA:_temp_.message}$'
    // pattern_definitions:
    // CISCO_PRIORITY_MSGCOUNT: '<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?'
    // CISCO_TIMESTAMP: '[*]?%{CISCOTIMESTAMP:_temp_.cisco_timestamp}(?: %{CISCO_TZ:_temp_.tz})?'
    // CISCO_UPTIME: '[0-9a-zA-Z]+'
    // CISCO_HOSTNAME: '[a-zA-Z][0-9a-zA-Z_-]{0,61}[0-9a-zA-Z]?'
    // CISCO_TZ: '[a-zA-Z]{1,4}'
    // "#;
    //
    // assert_eq!(
    // vrl::value!( {
    // "message": "Feb  8 04:00:48 192.168.100.2 585917: Feb  8 04:00:47.272: %SEC-6-IPACCESSLOGRP: list 177 denied igmp 192.168.100.197 -> 224.0.0.22, 1 packet",
    // "_temp_": {
    // "cisco_timestamp": "Feb  8 04:00:47.272",
    // "message": "%SEC-6-IPACCESSLOGRP: list 177 denied igmp 192.168.100.197 -> 224.0.0.22, 1 packet"
    // },
    // "cisco": {
    // "ios": {
    // "sequence": "585917"
    // }
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile_with_context(
    // &TranspileCtx::builder()
    // .with_name("grok_on_failure_test")
    // .build()
    // )?
    // .compile()?
    // .run(vrl::value!({
    // message: "Feb  8 04:00:48 192.168.100.2 585917: Feb  8 04:00:47.272: %SEC-6-IPACCESSLOGRP: list 177 denied igmp 192.168.100.197 -> 224.0.0.22, 1 packet"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn named_captures_outside_of_patterns() -> Result<()> {
    // let configuration = r#"
    // processors:
    // - grok:
    // field: message
    // ignore_failure: true
    // patterns:
    // - ^%{GREEDYDATA:user.full_name} (\((?<email>%{DATA:user.name}@%{DATA:user.domain})\))?$
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "John Doe (john.doe@example.com)",
    // user: {
    // full_name: "John Doe",
    // name: "john.doe",
    // domain: "example.com"
    // },
    // email: "john.doe@example.com"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "John Doe (john.doe@example.com)"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    // }
}
