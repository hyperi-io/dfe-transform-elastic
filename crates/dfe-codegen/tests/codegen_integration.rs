// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! End-to-end codegen integration tests.
//!
//! Parse pipeline YAML → generate Rust code → validate syntax with syn.

use std::collections::HashMap;

use dfe_codegen::codegen::PipelineCodegen;
use dfe_codegen::pipeline::Pipeline;

/// Helper: parse YAML, generate code, validate syntax with syn.
fn validate_codegen(yaml: &str, module_name: &str) -> String {
    let pipeline = Pipeline::parse(yaml).expect("failed to parse pipeline YAML");
    let cg = PipelineCodegen::new(&pipeline, module_name);
    let code = cg.generate().expect("failed to generate code");

    // Validate syntax with syn
    syn::parse_file(&code).unwrap_or_else(|e| {
        panic!(
            "generated code is not valid Rust:\n{}\n\nError: {}",
            code, e
        )
    });

    code
}

/// Helper: parse YAML with nested pipeline context, generate code, validate syntax.
fn validate_codegen_with_context(
    yaml: &str,
    module_name: &str,
    pipelines: HashMap<String, Pipeline>,
) -> String {
    let pipeline =
        Pipeline::parse_with_context(yaml, pipelines).expect("failed to parse pipeline YAML");
    let cg = PipelineCodegen::new(&pipeline, module_name);
    let code = cg.generate().expect("failed to generate code");

    syn::parse_file(&code).unwrap_or_else(|e| {
        panic!(
            "generated code is not valid Rust:\n{}\n\nError: {}",
            code, e
        )
    });

    code
}

#[test]
fn simple_set_remove_pipeline() {
    let yaml = r#"
description: "Basic set and remove"
processors:
  - set:
      field: event.kind
      value: event
  - set:
      field: event.category
      value: network
  - remove:
      field: _temp
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "simple_pipeline");
    assert!(code.contains("pub struct SimplePipeline;"));
    assert!(code.contains("impl Transform for SimplePipeline"));
}

#[test]
fn rename_and_lowercase() {
    let yaml = r#"
processors:
  - rename:
      field: source.address
      target_field: source.ip
      ignore_missing: true
  - lowercase:
      field: host.name
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "rename_lower");
    assert!(code.contains("event.rename("));
    assert!(code.contains("to_lowercase()"));
}

#[test]
fn convert_integer() {
    let yaml = r#"
processors:
  - convert:
      field: source.port
      type: integer
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "convert_int");
    assert!(code.contains("parse::<i64>()"));
}

#[test]
fn convert_string() {
    let yaml = r#"
processors:
  - convert:
      field: count
      type: string
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "convert_str");
    assert!(code.contains("to_string()"));
}

#[test]
fn convert_float() {
    let yaml = r#"
processors:
  - convert:
      field: score
      type: float
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "convert_float");
    assert!(code.contains("parse::<f64>()"));
}

#[test]
fn multi_processor_pipeline() {
    let yaml = r#"
description: "Realistic pipeline with multiple processors"
processors:
  - set:
      field: event.kind
      value: event
  - set:
      field: event.category
      value: network
  - rename:
      field: source.address
      target_field: source.ip
      ignore_missing: true
  - rename:
      field: destination.address
      target_field: destination.ip
      ignore_missing: true
  - convert:
      field: source.port
      type: integer
      ignore_missing: true
  - convert:
      field: destination.port
      type: integer
      ignore_missing: true
  - lowercase:
      field: event.action
      ignore_missing: true
  - remove:
      field:
        - _temp
        - _conf
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "multi_proc");
    assert!(code.contains("pub struct MultiProc;"));

    // Verify key operations are present
    let rename_count = code.matches("event.rename(").count();
    let convert_count = code.matches("parse::<i64>()").count();
    let remove_count = code.matches("event.remove(").count();

    assert!(code.contains("json!(\"event\")"), "expected set event.kind");
    assert!(
        code.contains("json!(\"network\")"),
        "expected set event.category"
    );
    assert_eq!(rename_count, 2, "expected 2 rename calls");
    assert_eq!(convert_count, 2, "expected 2 integer converts");
    assert_eq!(remove_count, 2, "expected 2 remove calls");
}

#[test]
fn template_string_interpolation() {
    let yaml = r#"
processors:
  - set:
      field: error.message
      value: 'Processor {{{_ingest.on_failure_processor_type}}} failed'
"#;

    let code = validate_codegen(yaml, "template_interp");
    assert!(code.contains("format!"));
    assert!(code.contains("_ingest.on_failure_processor_type"));
}

#[test]
fn set_copy_from() {
    let yaml = r#"
processors:
  - set:
      field: destination.ip
      copy_from: source.ip
"#;

    let code = validate_codegen(yaml, "copy_from");
    assert!(code.contains("event.get(\"source.ip\")"));
}

#[test]
fn trim_uppercase_split() {
    let yaml = r#"
processors:
  - trim:
      field: message
  - uppercase:
      field: event.action
  - split:
      field: tags
      separator: ","
"#;

    let code = validate_codegen(yaml, "trim_upper_split");
    assert!(code.contains("trim()"));
    assert!(code.contains("to_uppercase()"));
    assert!(code.contains("split(\",\")"));
}

#[test]
fn drop_with_condition() {
    let yaml = r#"
processors:
  - drop:
      if: ctx?.event?.severity > 7
"#;

    let code = validate_codegen(yaml, "drop_cond");
    assert!(code.contains("TransformResult::Drop"));
}

#[test]
fn append_processor() {
    let yaml = r#"
processors:
  - append:
      field: related.ip
      value: "10.0.0.1"
"#;

    let code = validate_codegen(yaml, "append_proc");
    assert!(code.contains("event.append("));
}

#[test]
fn set_with_override_false() {
    let yaml = r#"
processors:
  - set:
      field: event.kind
      value: event
      override: false
"#;

    let code = validate_codegen(yaml, "set_no_override");
    assert!(code.contains("!event.has("));
}

#[test]
fn set_boolean_and_number() {
    let yaml = r#"
processors:
  - set:
      field: event.enriched
      value: true
  - set:
      field: event.severity
      value: 5
"#;

    let code = validate_codegen(yaml, "set_types");
    assert!(code.contains("json!(true)"));
    assert!(code.contains("json!(5)"));
}

// -- Medium processor integration tests --

#[test]
fn gsub_basic() {
    let yaml = r#"
processors:
  - gsub:
      field: message
      pattern: "\\."
      replacement: "_"
"#;

    let code = validate_codegen(yaml, "gsub_basic");
    assert!(code.contains("regex::Regex::new("));
    assert!(code.contains("replace_all("));
    assert!(code.contains("event.set(\"message\""));
}

#[test]
fn gsub_with_target_field() {
    let yaml = r#"
processors:
  - gsub:
      field: message
      pattern: "\\."
      replacement: "_"
      target_field: clean_message
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "gsub_target");
    assert!(code.contains("event.set(\"clean_message\""));
    assert!(code.contains("event.has(\"message\")"));
}

#[test]
fn json_basic() {
    let yaml = r#"
processors:
  - json:
      field: message
"#;

    let code = validate_codegen(yaml, "json_basic");
    assert!(code.contains("serde_json::from_str("));
    assert!(code.contains("event.set(\"message\""));
}

#[test]
fn json_with_target() {
    let yaml = r#"
processors:
  - json:
      field: message
      target_field: parsed
      ignore_failure: true
"#;

    let code = validate_codegen(yaml, "json_target");
    assert!(code.contains("event.set(\"parsed\""));
    assert!(code.contains("ignore_failure: true"));
}

#[test]
fn csv_basic() {
    let yaml = r#"
processors:
  - csv:
      field: message
      target_fields:
        - field1
        - field2
        - field3
"#;

    let code = validate_codegen(yaml, "csv_basic");
    assert!(code.contains("csv::ReaderBuilder::new()"));
    assert!(code.contains("event.set(\"field1\""));
    assert!(code.contains("event.set(\"field2\""));
    assert!(code.contains("event.set(\"field3\""));
}

#[test]
fn kv_basic() {
    let yaml = r#"
processors:
  - kv:
      field: message
      field_split: " "
      value_split: "="
      ignore_missing: true
      ignore_failure: true
"#;

    let code = validate_codegen(yaml, "kv_basic");
    assert!(code.contains("split(\" \")"));
    assert!(code.contains("split_once(\"=\")"));
    assert!(code.contains("event.set("));
}

#[test]
fn kv_with_target() {
    let yaml = r#"
processors:
  - kv:
      field: message
      field_split: ";"
      value_split: "="
      target_field: parsed
      trim_key: " "
      trim_value: " "
"#;

    let code = validate_codegen(yaml, "kv_target");
    assert!(code.contains("parsed."));
    assert!(code.contains("key.trim()"));
    assert!(code.contains("value.trim()"));
}

#[test]
fn dissect_simple() {
    let yaml = r#"
processors:
  - dissect:
      field: message
      pattern: "Hello, %{subject}"
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "dissect_simple");
    assert!(code.contains("event.set(\"subject\""));
    assert!(code.contains("strip_prefix(\"Hello, \")"));
}

#[test]
fn dissect_complex() {
    let yaml = r#"
processors:
  - dissect:
      field: message
      pattern: "%{network.direction} %{network.transport} connection %{event.outcome}"
"#;

    let code = validate_codegen(yaml, "dissect_complex");
    assert!(code.contains("event.set(\"network.direction\""));
    assert!(code.contains("event.set(\"network.transport\""));
    assert!(code.contains("event.set(\"event.outcome\""));
}

#[test]
fn grok_basic() {
    let yaml = r#"
processors:
  - grok:
      field: message
      patterns:
        - "%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"
      ignore_missing: true
      ignore_failure: true
"#;

    let code = validate_codegen(yaml, "grok_basic");
    assert!(code.contains("grok_to_regex("));
    assert!(code.contains("regex::Regex::new("));
    assert!(code.contains("event.set(name, m.as_str())"));
}

#[test]
fn foreach_basic() {
    let yaml = r#"
processors:
  - foreach:
      field: items
      ignore_missing: true
      processor:
        uppercase:
          field: _ingest._value
"#;

    let code = validate_codegen(yaml, "foreach_basic");
    assert!(code.contains("Value::Array(items)"));
    assert!(code.contains("to_uppercase()"));
}

#[test]
fn date_iso8601() {
    let yaml = r#"
processors:
  - date:
      field: timestamp
      formats:
        - ISO8601
      target_field: "@timestamp"
"#;

    let code = validate_codegen(yaml, "date_iso");
    assert!(code.contains("parse_from_rfc3339("));
    assert!(code.contains("event.set(\"@timestamp\""));
}

#[test]
fn date_unix() {
    let yaml = r#"
processors:
  - date:
      field: timestamp
      formats:
        - UNIX
"#;

    let code = validate_codegen(yaml, "date_unix");
    assert!(code.contains("parse::<f64>()"));
    assert!(code.contains("from_timestamp("));
    assert!(code.contains("event.set(\"@timestamp\""));
}

#[test]
fn date_unix_ms() {
    let yaml = r#"
processors:
  - date:
      field: timestamp
      formats:
        - UNIX_MS
"#;

    let code = validate_codegen(yaml, "date_unix_ms");
    assert!(code.contains("parse::<i64>()"));
    assert!(code.contains("from_timestamp_millis("));
}

#[test]
fn mixed_medium_pipeline() {
    let yaml = r#"
description: "Pipeline with medium processors"
processors:
  - json:
      field: message
      ignore_failure: true
  - gsub:
      field: event.action
      pattern: "-"
      replacement: "_"
      ignore_missing: true
  - dissect:
      field: source.address
      pattern: "%{source.ip}:%{source.port}"
      ignore_failure: true
      ignore_missing: true
  - set:
      field: event.kind
      value: event
  - remove:
      field: _temp
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "mixed_medium");
    assert!(code.contains("pub struct MixedMedium;"));
    assert!(code.contains("serde_json::from_str("));
    assert!(code.contains("replace_all("));
    assert!(code.contains("strip_prefix("));
    assert!(code.contains("json!(\"event\")"));
    assert!(code.contains("event.remove("));
}

// -- Complex processor integration tests --

#[test]
fn registered_domain_basic() {
    let yaml = r#"
processors:
  - registered_domain:
      field: url.domain
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "reg_domain");
    assert!(code.contains("event.get_str(\"url.domain\")"));
    assert!(code.contains("registered_domain_lookup("));
    assert!(code.contains("event.set(\"domain\""));
    assert!(code.contains("event.set(\"registered_domain\""));
    assert!(code.contains("event.set(\"top_level_domain\""));
    assert!(code.contains("event.set(\"subdomain\""));
}

#[test]
fn registered_domain_with_target() {
    let yaml = r#"
processors:
  - registered_domain:
      field: url.domain
      target_field: url
"#;

    let code = validate_codegen(yaml, "reg_domain_target");
    assert!(code.contains("event.set(\"url.domain\""));
    assert!(code.contains("event.set(\"url.registered_domain\""));
    assert!(code.contains("event.set(\"url.top_level_domain\""));
    assert!(code.contains("event.set(\"url.subdomain\""));
}

#[test]
fn network_direction_basic() {
    let yaml = r#"
processors:
  - network_direction:
      internal_networks_field: internal_networks
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "net_dir");
    assert!(code.contains("event.get_str(\"source.ip\")"));
    assert!(code.contains("event.get_str(\"destination.ip\")"));
    assert!(code.contains("is_internal_ip("));
    assert!(code.contains("event.set(\"network.direction\""));
    assert!(code.contains("\"outbound\""));
    assert!(code.contains("\"inbound\""));
    assert!(code.contains("\"internal\""));
    assert!(code.contains("\"external\""));
}

#[test]
fn fingerprint_basic() {
    let yaml = r#"
processors:
  - fingerprint:
      fields:
        - "@timestamp"
        - event.id
      target_field: _id
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "fingerprint_basic");
    assert!(code.contains("Sha256"));
    assert!(code.contains("hasher.update("));
    assert!(code.contains("event.get(\"@timestamp\")"));
    assert!(code.contains("event.get(\"event.id\")"));
    assert!(code.contains("event.set(\"_id\""));
}

#[test]
fn fingerprint_default_target() {
    let yaml = r#"
processors:
  - fingerprint:
      fields:
        - user.name
"#;

    let code = validate_codegen(yaml, "fingerprint_default");
    // Default target is _id
    assert!(code.contains("event.set(\"_id\""));
    // Without ignore_missing, should have FieldNotFound error path
    assert!(code.contains("FieldNotFound"));
}

#[test]
fn pipeline_nested() {
    let inner_yaml = r#"
processors:
  - set:
      field: target
      value: Hello
"#;
    let inner = Pipeline::parse(inner_yaml).expect("inner pipeline");

    let yaml = r#"
processors:
  - pipeline:
      name: '{{< IngestPipeline "shared-pipeline" >}}'
"#;

    let code = validate_codegen_with_context(
        yaml,
        "pipeline_ref",
        HashMap::from([("shared-pipeline".into(), inner)]),
    );
    assert!(code.contains("shared-pipeline"));
    assert!(code.contains("event.set(\"target\""));
}

#[test]
fn mixed_complex_pipeline() {
    let yaml = r#"
description: "Pipeline with complex processors"
processors:
  - set:
      field: event.kind
      value: event
  - registered_domain:
      field: url.domain
      target_field: url
      ignore_missing: true
  - fingerprint:
      fields:
        - "@timestamp"
        - event.id
      target_field: _id
      ignore_missing: true
  - remove:
      field: _temp
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "mixed_complex");
    assert!(code.contains("pub struct MixedComplex;"));
    assert!(code.contains("json!(\"event\")"));
    assert!(code.contains("registered_domain_lookup("));
    assert!(code.contains("Sha256"));
    assert!(code.contains("event.remove("));
}

// -- Enrichment processor integration tests --

#[test]
fn geoip_basic() {
    let yaml = r#"
processors:
  - geoip:
      field: source.ip
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "geoip_basic");
    assert!(code.contains("event.get_str(\"source.ip\")"));
    assert!(code.contains("geoip_lookup("));
    assert!(code.contains("geoip.country_iso_code"));
    assert!(code.contains("geoip.city_name"));
}

#[test]
fn geoip_with_target() {
    let yaml = r#"
processors:
  - geoip:
      field: source.ip
      target_field: source.geo
      database_file: GeoLite2-ASN.mmdb
"#;

    let code = validate_codegen(yaml, "geoip_target");
    assert!(code.contains("geoip_lookup(\"geoip_asn\""));
    assert!(code.contains("source.geo.asn"));
    assert!(code.contains("source.geo.organization_name"));
}

#[test]
fn user_agent_basic() {
    let yaml = r#"
processors:
  - user_agent:
      field: user_agent.original
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "ua_basic");
    assert!(code.contains("event.get_str(\"user_agent.original\")"));
    assert!(code.contains("parse_user_agent("));
    assert!(code.contains("user_agent.name"));
    assert!(code.contains("user_agent.version"));
    assert!(code.contains("user_agent.os.name"));
}

#[test]
fn user_agent_target() {
    let yaml = r#"
processors:
  - user_agent:
      field: agent
      target_field: user
"#;

    let code = validate_codegen(yaml, "ua_target");
    assert!(code.contains("event.get_str(\"agent\")"));
    assert!(code.contains("user.name"));
    assert!(code.contains("user.version"));
}

#[test]
fn community_id_basic() {
    let yaml = r#"
processors:
  - community_id:
      ignore_missing: true
"#;

    let code = validate_codegen(yaml, "cid_basic");
    assert!(code.contains("event.get_str(\"source.ip\")"));
    assert!(code.contains("event.get_str(\"destination.ip\")"));
    assert!(code.contains("community_id_v1("));
    assert!(code.contains("event.set(\"network.community_id\""));
}

#[test]
fn community_id_custom_fields() {
    let yaml = r#"
processors:
  - community_id:
      source_ip: source.nat.ip
      source_port: source.nat.port
      destination_ip: destination.nat.ip
      destination_port: destination.nat.port
      target_field: community_id
"#;

    let code = validate_codegen(yaml, "cid_custom");
    assert!(code.contains("event.get_str(\"source.nat.ip\")"));
    assert!(code.contains("event.get_str(\"destination.nat.ip\")"));
    assert!(code.contains("event.set(\"community_id\""));
}

#[test]
fn script_basic() {
    let yaml = r#"
processors:
  - script:
      lang: painless
      source: "ctx.event_severity = ctx.event.severity"
      ignore_failure: true
"#;

    let code = validate_codegen(yaml, "script_basic");
    assert!(code.contains("painless_exec("));
    assert!(code.contains("ctx.event_severity"));
    assert!(code.contains("ignore_failure: true"));
}
