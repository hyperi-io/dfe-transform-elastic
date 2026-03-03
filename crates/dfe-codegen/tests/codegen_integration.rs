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
