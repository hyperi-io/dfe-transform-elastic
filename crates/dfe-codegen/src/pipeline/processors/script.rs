use crate::pipeline::unsupported_fields;
use crate::pipeline::{Validate, conditional::Conditional, on_failure::OnFailure};
use anyhow::{Result, ensure};
use serde::Deserialize;
use tracing::instrument;

/// A YAML value used as a script parameter.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Param(pub serde_yaml_ng::Value);

#[derive(Debug, Clone, Deserialize, Default, PartialEq)]
pub struct Script {
    pub lang: Option<String>,
    pub source: Option<String>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub tag: Option<String>,
    pub params: Option<Param>,
    pub on_failure: Option<OnFailure>,
    pub ignore_failure: Option<bool>,

    // Unsupported options
    pub id: Option<String>,
}

impl Validate for Script {
    #[instrument(skip_all, name = "Script::validate", err)]
    fn validate(&self) -> Result<()> {
        if let Some(lang) = &self.lang {
            ensure!(
                lang == "painless",
                "language '{lang}' not currently supported in scripts"
            );
        }

        ensure!(
            self.id.is_some() || self.source.is_some(),
            "'id' or 'source' must be specified"
        );

        if self.ignore_failure.unwrap_or_default() {
            tracing::warn!("Ignore Failure is not properly supported by 'script' processor")
        }

        if self.on_failure.is_some() {
            tracing::warn!("On Failure is not properly supported by 'script' processor")
        }

        unsupported_fields!("script", self, id);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::{Pipeline, Processor, unsupported_fields_tests};
        use pretty_assertions::assert_eq;

        #[test]
        fn happy_path() {
            let configuration = r#"
                    processors:
                        - script:
                            source: 'true'
                "#;

            match &Pipeline::parse(configuration).unwrap().processors[..] {
                [Processor::Script(script)] => {
                    assert_eq!(script.source.as_ref().unwrap(), "true");
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        #[test]
        fn conditional() {
            let configuration = r#"
                    processors:
                        - script:
                            conditional: 'true && false'
                            source: 'true'
                "#;

            match &Pipeline::parse(configuration).unwrap().processors[..] {
                [Processor::Script(script)] => {
                    assert_eq!(script.source.as_ref().unwrap(), "true");
                    assert_eq!(script.conditional.as_ref().unwrap().0, "true && false");
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        #[test]
        fn tag() {
            let configuration = r#"
                    processors:
                        - script:
                            tag: some tag
                            source: 'true'
                "#;

            match &Pipeline::parse(configuration).unwrap().processors[..] {
                [Processor::Script(script)] => {
                    assert_eq!(script.source.as_ref().unwrap(), "true");
                    assert_eq!(script.tag.as_ref().unwrap(), "some tag");
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        #[test]
        fn only_painless_supported() {
            for lang in ["painless", "expression", "mustache", "java"] {
                let configuration = format!(
                    r#"
                    processors:
                        - script:
                            lang: {lang}
                            source: 'true'
                "#
                );

                if lang == "painless" {
                    match &Pipeline::parse(&configuration).unwrap().processors[..] {
                        [Processor::Script(script)] => {
                            assert_eq!(script.lang.as_ref().unwrap(), "painless");
                        }
                        _ => panic!("unexpected pipeline structure"),
                    }
                } else {
                    assert_eq!(
                        Pipeline::parse(&configuration)
                            .unwrap_err()
                            .root_cause()
                            .to_string(),
                        format!("language '{lang}' not currently supported in scripts")
                    );
                }
            }
        }

        #[test]
        fn id_or_source_required() {
            let configuration = r#"
                    processors:
                        - script:
                "#;

            assert_eq!(
                Pipeline::parse(configuration)
                    .unwrap_err()
                    .root_cause()
                    .to_string(),
                format!("'id' or 'source' must be specified")
            );
        }

        unsupported_fields_tests!(
            "script",
            r#"
                processors:
                    - script:
                        source: 'true'
                        {}: {}
            "#,
            id => "some_id"
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::{pipeline::Pipeline, test_utils};
    // use pretty_assertions::assert_eq;
    //
    // #[test]
    // fn transpile_if_possible() {
    // let configuration = r#"
    // processors:
    // - script:
    // if: ctx?.event?.duration != null
    // source: ctx.event.duration = ctx.event.duration * 1000000
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ event: { duration: 2000000 }}),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ event: { duration: 2 }}))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn params() {
    // let configuration = r#"
    // processors:
    // - script:
    // if: ctx?.event?.duration != null
    // source: ctx.event.duration = ctx.event.duration * params.param_nano
    // params:
    // param_nano: 1000000
    // "#;
    //
    // assert_eq!(
    // vrl::value!({ event: { duration: 2000000 }}),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ event: { duration: 2 }}))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn script_4692373172329801823() {
    // let configuration = r#"
    // processors:
    // - uri_parts:
    // field: url.original
    // target_field: _temp_.url
    // ignore_failure: true
    // if: ctx?.url?.original != null
    // - script:
    // if: ctx?._temp_?.url != null
    // source: |
    // for (entry in ctx._temp_.url.entrySet()) {
    // if (entry != null && entry.getValue() != null) {
    // if(ctx.url[entry.getKey()] == null) {
    // ctx.url[entry.getKey()] = entry.getValue();
    // } else if (!ctx.url[entry.getKey()].contains(entry.getValue())) {
    // ctx.url[entry.getKey()] = [ctx.url[entry.getKey()]];
    // ctx.url[entry.getKey()].add(entry.getValue());
    // }
    // }
    // }
    // - remove:
    // field: _temp_
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ url: {
    // original: "https://example.com/cat.gif?w=1920&h=1080",
    // domain: "example.com",
    // extension: "jpeg"
    // }}))
    // .unwrap()
    // .target,
    // vrl::value!({
    // url: {
    // original: "https://example.com/cat.gif?w=1920&h=1080",
    // domain: "example.com",
    // extension: ["jpeg", "gif"],
    // path: "/cat.gif",
    // port: 443,
    // query: "h=1080&w=1920",
    // scheme: "https",
    // }
    // })
    // );
    // }
    //
    // #[test]
    // fn script_18143229625879012263() {
    // let configuration = r#"
    // processors:
    // - script:
    // source: |
    // Map keysToSnakeCase(Map m) {
    // def regex = /([a-z])([A-Z]+)/;
    // def out = [:];
    //
    // for (entry in m.entrySet()) {
    // def k = entry.getKey();
    // def v = entry.getValue();
    //
    // if (v instanceof Map) {
    // v = keysToSnakeCase(v);
    // } else if (v instanceof List) {
    // for (int i = 0; i < v.size(); i++) {
    // def item = v.get(i);
    // if (item instanceof Map) {
    // v.set(i, keysToSnakeCase(item));
    // }
    // }
    // }
    //
    // k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();
    // out.put(k, v);
    // }
    //
    // return out;
    // }
    //
    // ctx.azure['signinlogs'] = keysToSnakeCase(ctx.azure.signinlogs);
    //
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // "azure": {
    // "signinlogs": {
    // "camelCaseIsCool": "value",
    // "PascalCaseIsCooler": {
    // "nestedInsideObject": [{
    // "NestedInsideArray": true
    // }]
    // }
    // }
    // }
    // }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // "azure": {
    // "signinlogs": {
    // "camel_case_is_cool": "value",
    // "pascal_case_is_cooler": {
    // "nested_inside_object": [{
    // "nested_inside_array": true
    // }]
    // }
    // }
    // }
    // })
    // );
    // }
    //
    // #[test]
    // fn script_11866630383577126771() {
    // let configuration = r#"
    // processors:
    // - script:
    // lang: painless
    // ignore_failure: true
    // params:
    // "write":
    // type:
    // - change
    // "read":
    // type:
    // - access
    // "delete":
    // type:
    // - deletion
    // "action":
    // type:
    // - change
    // source: >-
    // if (ctx?.azure?.activitylogs?.category == null) {
    // return;
    // }
    // def hm = new HashMap(params.get(ctx.azure.activitylogs.category.toLowerCase()));
    // hm.forEach((k, v) -> ctx.event[k] = v);
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({ azure: { activitylogs: { category: "WRITE" } } }),
    // vrl::value!({ azure: { activitylogs: { category: "WRITE" } }, event: { "type": ["change"]} }),
    // ),
    // (
    // vrl::value!({ azure: { activitylogs: { category: "WRITE" } } }),
    // vrl::value!({ azure: { activitylogs: { category: "WRITE" } }, event: { "type": ["change"]} }),
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
    // pub fn script_3314299043616797117() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - script:
    // lang: painless
    // if: 'ctx?._temp_?.labels != null && ctx._temp_.labels != 0'
    // params:
    // pcap_included: 0x80000000
    // ipv6_session: 0x02000000
    // ssl_decrypted: 0x01000000
    // url_filter_denied: 0x00800000
    // nat_translated: 0x00400000
    // captive_portal: 0x00200000
    // x_forwarded_for: 0x00080000
    // http_proxy: 0x00040000
    // container_page: 0x00008000
    // temporary_match: 0x00002000
    // symmetric_return: 0x00000800
    // source: >
    // def labels = ctx?.labels;
    // if (labels == null) {
    // labels = new HashMap();
    // ctx['labels'] = labels;
    // }
    // long value = ctx._temp_.labels;
    // for (entry in params.entrySet()) {
    // if ((value & entry.getValue()) != 0) {
    // labels[entry.getKey()] = true;
    // }
    // }
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // _temp_: { labels: 50595840 },
    // labels: {
    // symmetric_return: true,
    // http_proxy: true,
    // ssl_decrypted: true,
    // ipv6_session: true
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // _temp_: { labels: 50595840 }
    // }))?
    // .target,
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn script_16890175434472863985() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - script:
    // lang: painless
    // if: 'ctx?.event?.duration != null'
    // params:
    // NANOS_IN_A_SECOND: 1000000000
    // source: >
    // long nanos = ctx['event']['duration'] * params.NANOS_IN_A_SECOND;
    // ctx['event']['duration'] = nanos;
    // def start = ctx.event?.start;
    // if (start != null) {
    // ctx.event['end'] = ZonedDateTime.parse(start).plusNanos(nanos);
    // }
    // "#;
    //
    // assert_eq!(
    // r#"{ "event": { "duration": 10000000000, "end": t'2024-09-25T15:00:10Z', "start": "2024-09-25T15:00:00Z" } }"#,
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // event: {
    // duration: 10,
    // start: "2024-09-25T15:00:00Z"
    // }
    // }))?
    // .target
    // .to_string(),
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // pub fn comparison() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - script:
    // lang: painless
    // description: This script will set log.level field in descriptive way from severity.
    // if: ctx.event?.severity != null
    // tag: 'script_to_set_log_level'
    // params:
    // LogLevel:
    // - emergency
    // - alert
    // - critical
    // - error
    // - warning
    // - notification
    // - informational
    // - debugging
    // source: |-
    // def LogLevelValue = (int) ctx.event.severity;
    // if (LogLevelValue >= 0 && LogLevelValue < params.LogLevel.length) {
    // ctx.log.put('level', params['LogLevel'][LogLevelValue]);
    // }
    // "#;
    //
    // for (input, output) in [
    // (
    // vrl::value!({ event: { severity: "0" }}),
    // vrl::value!({ event: { severity: "0" }, log: { level: "emergency" }}),
    // ),
    // (
    // vrl::value!({ event: { severity: "5" }}),
    // vrl::value!({ event: { severity: "5" }, log: { level: "notification" }}),
    // ),
    // (
    // vrl::value!({ event: { severity: "9" }}),
    // vrl::value!({ event: { severity: "9" }}),
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
    // pub fn weird_params() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - script:
    // lang: painless
    // params:
    // values:
    // - null
    // - ''
    // - '-'
    // - 'N/A'
    // - 'NA'
    // - 0
    // source: ctx.params = params.values;
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // params: [null, "", "-", "N/A", "NA", 0]
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({}))?
    // .target
    // );
    //
    // Ok(())
    // }
    // }
}
