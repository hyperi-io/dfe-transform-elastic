use anyhow::bail;
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    Validate, conditional::Conditional, dynamic_path::DynamicPath, on_failure::OnFailure,
    unsupported_fields,
};

// https://www.elastic.co/guide/en/elasticsearch/reference/8.13/kv-processor.html
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct KV {
    pub field: String,
    pub field_split: String,
    pub value_split: String,

    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub target_field: Option<DynamicPath>,
    pub ignore_failure: Option<bool>,
    pub ignore_missing: Option<bool>,
    pub on_failure: Option<OnFailure>,
    pub trim_key: Option<String>,
    pub trim_value: Option<String>,
    pub strip_brackets: Option<bool>,
    pub tag: Option<String>,

    // Unsupported
    pub include_keys: Option<Vec<String>>,
    pub exclude_keys: Option<Vec<String>>,
    pub prefix: Option<String>,
}

impl Validate for KV {
    #[instrument(name = "KV::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!("kv", self, include_keys, exclude_keys, prefix);

        if !matches!(self.trim_key.as_deref(), Some(" ") | None)
            || !matches!(self.trim_value.as_deref(), Some(" ") | None)
        {
            bail!(
                "only \" \" is currently supported as the values of trim_key and trim_value, found key: {:?} and value: {:?}",
                self.trim_key,
                self.trim_value
            );
        }

        if self.strip_brackets.is_some() {
            tracing::warn!("strip_brackets is not supported by kv processor");
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "kv",
            r#"
                processors:
                    - kv:
                        field: message
                        field_split: " "
                        value_split: "="
                        {}: {}
            "#,
            include_keys => "[]",
            exclude_keys => "[]",
            prefix => "prefix"
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
    // use anyhow::anyhow;
    //
    // #[test]
    // fn happy_path() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // field: message
    // field_split: " "
    // value_split: "="
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "foo=Foo bar=Bar",
    // foo: "Foo",
    // bar: "Bar"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo=Foo bar=Bar"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn target_field() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // field: message
    // field_split: " "
    // value_split: "="
    // target_field: map
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "foo=Foo bar=Bar",
    // map: {
    // foo: "Foo",
    // bar: "Bar"
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo=Foo bar=Bar"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional_true() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // if: '["101", "102", "103"].contains(ctx.message_id)'
    // field: message
    // field_split: " "
    // value_split: "="
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "foo=Foo bar=Bar",
    // message_id: "102",
    // foo: "Foo",
    // bar: "Bar"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo=Foo bar=Bar",
    // message_id: "102"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn conditional_false() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // if: '["101", "102", "103"].contains(ctx.message_id)'
    // field: message
    // field_split: " "
    // value_split: "="
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "foo=Foo bar=Bar",
    // message_id: "104"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo=Foo bar=Bar",
    // message_id: "104"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_failure_true() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // field: message
    // ignore_failure: true
    // field_split: " "
    // value_split: "="
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "foo Foo bar Bar",
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo Foo bar Bar",
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_failure_false() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // field: message
    // ignore_failure: false
    // field_split: " "
    // value_split: "="
    // "#;
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo Foo bar Bar",
    // }))
    // .err()
    // .ok_or_else(|| anyhow!("failed to fail"))?
    // .root_cause()
    // .to_string()
    // .contains("field .message does not contain value_split [=]"));
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_missing_true() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // field: other
    // ignore_missing: true
    // field_split: " "
    // value_split: "="
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "foo=bar",
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo=bar",
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_missing_false() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // field: other
    // ignore_missing: false
    // field_split: " "
    // value_split: "="
    // "#;
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo=bar",
    // }))
    // .err()
    // .ok_or_else(|| anyhow!("failed to fail"))?
    // .root_cause()
    // .to_string()
    // .contains(".other must not be nullish"));
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn cisco_nexus() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // field: message
    // target_field: temp
    // field_split: ';'
    // value_split: =
    // ignore_missing: true
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "TTY=pts/0 ; PWD=/var/sysmgr/vsh ; USER=root ; COMMAND=/isanboot/bin/imghdr Debug /var/sysmgr/ftp/img-sync/curr-isan.img - sudo",
    // temp: {
    // TTY: "pts/0",
    // PWD: "/var/sysmgr/vsh",
    // USER: "root",
    // COMMAND: "/isanboot/bin/imghdr Debug /var/sysmgr/ftp/img-sync/curr-isan.img - sudo",
    // },
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "TTY=pts/0 ; PWD=/var/sysmgr/vsh ; USER=root ; COMMAND=/isanboot/bin/imghdr Debug /var/sysmgr/ftp/img-sync/curr-isan.img - sudo",
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn cisco_strict_whitespace() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // field: message
    // target_field: temp
    // field_split: ' '
    // value_split: =
    // ignore_missing: true
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "logname= uid=0 euid=0 tty= ruser= rhost=  user=admin",
    // temp: {
    // "logname": "",
    // "uid": "0",
    // "euid": "0",
    // "tty": "",
    // "ruser": "",
    // "rhost": "",
    // "user": "admin",
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "logname= uid=0 euid=0 tty= ruser= rhost=  user=admin",
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn user_agent() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // field: message
    // target_field: temp
    // field_split: ','
    // value_split: ':'
    // ignore_missing: true
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "User: username, Agent: Mozilla/5.0 (KHTML, like Gecko) Chrome/103.0.0.0, Email: email@example.com",
    // temp: {
    // "User": "username",
    // "Agent": "Mozilla/5.0 (KHTML like Gecko) Chrome/103.0.0.0",
    // "Email": "email@example.com",
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "User: username, Agent: Mozilla/5.0 (KHTML, like Gecko) Chrome/103.0.0.0, Email: email@example.com",
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn template_target() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // field: message
    // target_field: cisco_meraki.{{{cisco_meraki.event_subtype}}}
    // field_split: ' '
    // value_split: =
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "uid=0 euid=0 user=admin",
    // cisco_meraki: {
    // event_subtype: "syslog",
    // syslog: {
    // "uid": "0",
    // "euid": "0",
    // "user": "admin",
    // }
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // message: "uid=0 euid=0 user=admin",
    // cisco_meraki: {
    // event_subtype: "syslog",
    // }
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn on_failure() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - kv:
    // field: message
    // field_split: " "
    // value_split: "="
    // tag: test_tag
    // on_failure:
    // - append:
    // field: error.message
    // value: 'Processor {{{_ingest.on_failure_processor_type}}} with tag {{{_ingest.on_failure_processor_tag}}} in pipeline {{{_ingest.on_failure_pipeline}}} failed with message: {{{_ingest.on_failure_message}}}'
    // "#;
    //
    // assert!(Pipeline::parse(configuration)?
    // .transpile_with_context(&TranspileCtx::builder().with_name("test_pipeline").build())?
    // .compile()?
    // .run(vrl::value!({
    // message: "foo Foo bar Bar",
    // }))?
    // .target
    // .get("error.message[0]")
    // .ok_or_else(|| anyhow!("failed to find error message"))?
    // .as_str()
    // .ok_or_else(|| anyhow!("error message was not a string"))?
    // .to_string()
    // .contains("Processor kv with tag test_tag in pipeline test_pipeline failed with message"));
    //
    // Ok(())
    // }
    // }
}
