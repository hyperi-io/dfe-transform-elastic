// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! End-to-end codegen integration tests.
//!
//! Parse pipeline YAML → generate Rust code → validate syntax with syn.

use dfe_codegen::codegen::PipelineCodegen;
use dfe_codegen::pipeline::Pipeline;

/// Helper: parse YAML, generate code, validate syntax with syn.
fn validate_codegen(yaml: &str, module_name: &str) -> String {
    let pipeline = Pipeline::parse(yaml).expect("failed to parse pipeline YAML");
    let gen = PipelineCodegen::new(&pipeline, module_name);
    let code = gen.generate().expect("failed to generate code");

    // Validate syntax with syn
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
    assert!(code.contains("json!(\"network\")"), "expected set event.category");
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
