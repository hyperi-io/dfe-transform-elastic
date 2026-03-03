// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Per-processor Rust code generation.
//!
//! Each supported processor has a function that emits Rust statements
//! operating on an `Event`. The generated code uses the dfe-runtime API
//! (event.get(), event.set(), event.remove(), event.rename(), etc).

use anyhow::{bail, Result};

use crate::pipeline::conditional::Conditional;
use crate::pipeline::processors::*;
use crate::pipeline::template_string::{TemplateFragment, TemplateString};
use crate::pipeline::Processor;

/// Emit Rust code for a single processor.
///
/// `indent_level` is the number of 4-space indentation levels to prepend.
pub fn emit_processor(processor: &Processor, indent_level: usize) -> Result<String> {
    let pad = "    ".repeat(indent_level);

    match processor {
        Processor::Set(p) => emit_set(p, &pad),
        Processor::Remove(p) => emit_remove(p, &pad),
        Processor::Rename(p) => emit_rename(p, &pad),
        Processor::Append(p) => emit_append(p, &pad),
        Processor::Lowercase(p) => emit_lowercase(p, &pad),
        Processor::Uppercase(p) => emit_uppercase(p, &pad),
        Processor::Trim(p) => emit_trim(p, &pad),
        Processor::Convert(p) => emit_convert(p, &pad),
        Processor::Drop(p) => emit_drop(p, &pad),
        Processor::Split(p) => emit_split(p, &pad),
        Processor::UriParts(p) => emit_uri_parts(p, &pad),
        _ => bail!("codegen not yet implemented for '{}' processor", processor.name()),
    }
}

// -- Helpers ----------------------------------------------------------------

/// Format a dotted field path as a Rust string literal.
fn field_lit(field: &str) -> String {
    format!("\"{}\"", field)
}

/// Wrap generated statements in an `if` block when a condition is present.
fn wrap_conditional(cond: &Option<Conditional>, body: &str, pad: &str) -> String {
    match cond {
        Some(c) => {
            // Emit condition as a comment + the raw Painless expression.
            // Full Painless→Rust transpilation is future work (2.2.3).
            // For now we emit a TODO with the raw expression.
            format!(
                "{pad}// TODO: conditional: {expr}\n{pad}{{\n{body}{pad}}}\n",
                expr = c.0,
            )
        }
        None => body.to_string(),
    }
}

/// Wrap a block of statements in ignore_failure handling.
fn wrap_ignore_failure(ignore: Option<bool>, body: &str, pad: &str) -> String {
    if ignore == Some(true) {
        format!(
            "{pad}// ignore_failure: true\n{pad}let _ = (|| -> Result<()> {{\n{body}{pad}}})();\n"
        )
    } else {
        body.to_string()
    }
}

/// Wrap a field access in ignore_missing handling.
fn wrap_ignore_missing(
    ignore: Option<bool>,
    field: &str,
    body: &str,
    pad: &str,
) -> String {
    if ignore == Some(true) {
        format!(
            "{pad}if event.has({field}) {{\n{body}{pad}}}\n",
            field = field_lit(field),
        )
    } else {
        body.to_string()
    }
}

/// Generate a Rust expression for a template string value.
///
/// For plain strings, returns a `json!("value")` expression.
/// For template strings with `{{field}}` interpolation, returns code that
/// reads the field and builds the value.
fn emit_template_value(ts: &TemplateString) -> String {
    let fragments = ts.fragments();

    // Simple literal (no interpolation)
    if fragments.len() == 1 {
        if let TemplateFragment::Literal(lit) = &fragments[0] {
            return format!("json!(\"{}\")", escape_json_str(lit));
        }
    }

    // Single variable reference (copy_from equivalent)
    if fragments.len() == 1 {
        if let TemplateFragment::Variable(var) = &fragments[0] {
            return format!(
                "event.get({}).cloned().unwrap_or(Value::Null)",
                field_lit(var)
            );
        }
    }

    // Mixed template — build with format!()
    let mut fmt_str = String::new();
    let mut args = Vec::new();
    for frag in &fragments {
        match frag {
            TemplateFragment::Literal(lit) => {
                fmt_str.push_str(&escape_json_str(lit));
            }
            TemplateFragment::Variable(var) => {
                fmt_str.push_str("{}");
                args.push(format!(
                    "event.get_str({}).unwrap_or(\"\")",
                    field_lit(var)
                ));
            }
        }
    }

    if args.is_empty() {
        format!("json!(\"{}\")", fmt_str)
    } else {
        format!(
            "json!(format!(\"{}\", {}))",
            fmt_str,
            args.join(", ")
        )
    }
}

/// Escape a string for use inside a Rust string literal.
fn escape_json_str(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

// -- Processor emitters -----------------------------------------------------

fn emit_set(p: &set::Set, pad: &str) -> Result<String> {
    let inner_pad = format!("{pad}    ");
    let field = &p.field;

    let mut body = String::new();

    // Determine the value expression
    let value_expr = if let Some(copy_from) = &p.copy_from {
        format!(
            "event.get({}).cloned().unwrap_or(Value::Null)",
            field_lit(copy_from)
        )
    } else if let Some(value) = &p.value {
        match value {
            set::Value::String(ts) => emit_template_value(ts),
            set::Value::Number(n) => format!("json!({})", n),
            set::Value::Bool(b) => format!("json!({})", b),
            set::Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|s| format!("\"{}\"", escape_json_str(s))).collect();
                format!("json!([{}])", items.join(", "))
            }
        }
    } else {
        bail!("set processor has neither value nor copy_from");
    };

    // Handle override: false
    if p.override_values == Some(false) {
        body.push_str(&format!(
            "{inner_pad}if !event.has({field_s}) {{\n{inner_pad}    event.set({field_s}, {value})?;\n{inner_pad}}}\n",
            field_s = field_lit(field),
            value = value_expr,
        ));
    } else {
        body.push_str(&format!(
            "{inner_pad}event.set({}, {})?;\n",
            field_lit(field),
            value_expr,
        ));
    }

    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.condition, &body, pad);
    Ok(body)
}

fn emit_remove(p: &remove::Remove, pad: &str) -> Result<String> {
    let inner_pad = format!("{pad}    ");
    let mut body = String::new();

    let fields = match &p.field {
        remove::Field::One(f) => vec![f.as_str()],
        remove::Field::Many(fs) => fs.iter().map(|s| s.as_str()).collect(),
    };

    let ignore_missing = p.ignore_missing == Some(true);

    for field in &fields {
        if ignore_missing {
            body.push_str(&format!(
                "{inner_pad}event.remove({});\n",
                field_lit(field),
            ));
        } else {
            body.push_str(&format!(
                "{inner_pad}if event.remove({f}).is_none() {{\n{inner_pad}    return Err(TransformError::FieldNotFound {{ path: {f}.into() }}.into());\n{inner_pad}}}\n",
                f = field_lit(field),
            ));
        }
    }

    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_rename(p: &rename::Rename, pad: &str) -> Result<String> {
    let inner_pad = format!("{pad}    ");
    let mut body = String::new();

    let from = p.field.raw();
    let to = p.target_field.raw();

    if p.ignore_missing == Some(true) {
        body.push_str(&format!(
            "{inner_pad}if event.has({from_s}) {{\n{inner_pad}    event.rename({from_s}, {to_s})?;\n{inner_pad}}}\n",
            from_s = field_lit(from),
            to_s = field_lit(to),
        ));
    } else {
        body.push_str(&format!(
            "{inner_pad}event.rename({}, {})?;\n",
            field_lit(from),
            field_lit(to),
        ));
    }

    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_append(p: &append::Append, pad: &str) -> Result<String> {
    let inner_pad = format!("{pad}    ");
    let mut body = String::new();
    let field = &p.field;

    match &p.value {
        append::AppendValue::String(ts) => {
            let value_expr = emit_template_value(ts);
            body.push_str(&format!(
                "{inner_pad}event.append({}, {})?;\n",
                field_lit(field),
                value_expr,
            ));
        }
        append::AppendValue::Array(items) => {
            for item in items {
                let value_expr = emit_template_value(item);
                body.push_str(&format!(
                    "{inner_pad}event.append({}, {})?;\n",
                    field_lit(field),
                    value_expr,
                ));
            }
        }
    }

    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_lowercase(p: &lowercase::Lowercase, pad: &str) -> Result<String> {
    let inner_pad = format!("{pad}    ");
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);

    let mut body = String::new();
    body.push_str(&format!(
        "{inner_pad}if let Some(s) = event.get_str({field_s}) {{\n\
         {inner_pad}    let lowered = s.to_lowercase();\n\
         {inner_pad}    event.set({target_s}, lowered)?;\n\
         {inner_pad}}}\n",
        field_s = field_lit(field),
        target_s = field_lit(target),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_uppercase(p: &uppercase::Uppercase, pad: &str) -> Result<String> {
    let inner_pad = format!("{pad}    ");
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);

    let mut body = String::new();
    body.push_str(&format!(
        "{inner_pad}if let Some(s) = event.get_str({field_s}) {{\n\
         {inner_pad}    let uppered = s.to_uppercase();\n\
         {inner_pad}    event.set({target_s}, uppered)?;\n\
         {inner_pad}}}\n",
        field_s = field_lit(field),
        target_s = field_lit(target),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_trim(p: &trim::Trim, pad: &str) -> Result<String> {
    let inner_pad = format!("{pad}    ");
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);

    let mut body = String::new();
    body.push_str(&format!(
        "{inner_pad}if let Some(s) = event.get_str({field_s}) {{\n\
         {inner_pad}    let trimmed = s.trim().to_string();\n\
         {inner_pad}    event.set({target_s}, trimmed)?;\n\
         {inner_pad}}}\n",
        field_s = field_lit(field),
        target_s = field_lit(target),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_convert(p: &convert::Convert, pad: &str) -> Result<String> {
    let inner_pad = format!("{pad}    ");
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);

    let conversion = match p.into_type {
        convert::IntoType::Integer | convert::IntoType::Long => format!(
            "{inner_pad}if let Some(val) = event.get({field_s}) {{\n\
             {inner_pad}    let converted = match val {{\n\
             {inner_pad}        Value::String(s) => {{\n\
             {inner_pad}            let s = s.trim();\n\
             {inner_pad}            if let Some(hex) = s.strip_prefix(\"0x\") {{\n\
             {inner_pad}                json!(i64::from_str_radix(hex, 16).map_err(|_| TransformError::ParseError {{ path: {field_s}.into(), message: format!(\"cannot convert '{{}}' to integer\", s) }})?)\n\
             {inner_pad}            }} else {{\n\
             {inner_pad}                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {{ path: {field_s}.into(), message: format!(\"cannot convert '{{}}' to integer\", s) }})?)\n\
             {inner_pad}            }}\n\
             {inner_pad}        }}\n\
             {inner_pad}        Value::Number(n) => json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64)),\n\
             {inner_pad}        Value::Bool(b) => json!(if *b {{ 1 }} else {{ 0 }}),\n\
             {inner_pad}        _ => return Err(TransformError::ParseError {{ path: {field_s}.into(), message: \"cannot convert to integer\".into() }}.into()),\n\
             {inner_pad}    }};\n\
             {inner_pad}    event.set({target_s}, converted)?;\n\
             {inner_pad}}}\n",
            field_s = field_lit(field),
            target_s = field_lit(target),
        ),
        convert::IntoType::Float => format!(
            "{inner_pad}if let Some(val) = event.get({field_s}) {{\n\
             {inner_pad}    let converted = match val {{\n\
             {inner_pad}        Value::String(s) => json!(s.trim().parse::<f64>().map_err(|_| TransformError::ParseError {{ path: {field_s}.into(), message: format!(\"cannot convert '{{}}' to float\", s) }})?),\n\
             {inner_pad}        Value::Number(n) => json!(n.as_f64().unwrap_or(0.0)),\n\
             {inner_pad}        Value::Bool(b) => json!(if *b {{ 1.0 }} else {{ 0.0 }}),\n\
             {inner_pad}        _ => return Err(TransformError::ParseError {{ path: {field_s}.into(), message: \"cannot convert to float\".into() }}.into()),\n\
             {inner_pad}    }};\n\
             {inner_pad}    event.set({target_s}, converted)?;\n\
             {inner_pad}}}\n",
            field_s = field_lit(field),
            target_s = field_lit(target),
        ),
        convert::IntoType::String => format!(
            "{inner_pad}if let Some(val) = event.get({field_s}) {{\n\
             {inner_pad}    let converted = match val {{\n\
             {inner_pad}        Value::String(_) => val.clone(),\n\
             {inner_pad}        Value::Number(n) => json!(n.to_string()),\n\
             {inner_pad}        Value::Bool(b) => json!(b.to_string()),\n\
             {inner_pad}        Value::Null => json!(\"null\"),\n\
             {inner_pad}        _ => json!(val.to_string()),\n\
             {inner_pad}    }};\n\
             {inner_pad}    event.set({target_s}, converted)?;\n\
             {inner_pad}}}\n",
            field_s = field_lit(field),
            target_s = field_lit(target),
        ),
        convert::IntoType::IP => format!(
            "{inner_pad}if let Some(s) = event.get_str({field_s}) {{\n\
             {inner_pad}    // Validate IP format\n\
             {inner_pad}    let s = s.trim();\n\
             {inner_pad}    if s.parse::<std::net::IpAddr>().is_err() {{\n\
             {inner_pad}        return Err(TransformError::ParseError {{ path: {field_s}.into(), message: format!(\"cannot convert '{{}}' to IP\", s) }}.into());\n\
             {inner_pad}    }}\n\
             {inner_pad}    event.set({target_s}, s)?;\n\
             {inner_pad}}}\n",
            field_s = field_lit(field),
            target_s = field_lit(target),
        ),
        _ => bail!("convert type {:?} not supported in codegen", p.into_type),
    };

    let mut body = String::new();
    body.push_str(&conversion);

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_drop(p: &drop::Drop, pad: &str) -> Result<String> {
    let inner_pad = format!("{pad}    ");
    let mut body = String::new();

    // Drop always has a condition
    body.push_str(&format!(
        "{inner_pad}return Ok(TransformResult::Drop);\n"
    ));

    // Always wrapped in conditional
    let body = wrap_conditional(&Some(p.condition.clone()), &body, pad);
    Ok(body)
}

fn emit_split(p: &split::Split, pad: &str) -> Result<String> {
    let inner_pad = format!("{pad}    ");
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);
    let separator = escape_json_str(&p.separator);

    let mut body = String::new();
    body.push_str(&format!(
        "{inner_pad}if let Some(s) = event.get_str({field_s}) {{\n\
         {inner_pad}    let parts: Vec<Value> = s.split(\"{sep}\").map(|p| json!(p)).collect();\n\
         {inner_pad}    event.set({target_s}, Value::Array(parts))?;\n\
         {inner_pad}}}\n",
        field_s = field_lit(field),
        target_s = field_lit(target),
        sep = separator,
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_uri_parts(p: &uri_parts::UriParts, pad: &str) -> Result<String> {
    let inner_pad = format!("{pad}    ");
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);

    let mut body = String::new();
    body.push_str(&format!(
        "{inner_pad}if let Some(uri_str) = event.get_str({field_s}) {{\n\
         {inner_pad}    if let Ok(url) = url::Url::parse(uri_str) {{\n\
         {inner_pad}        event.set(\"{target}.scheme\", url.scheme())?;\n\
         {inner_pad}        if let Some(host) = url.host_str() {{\n\
         {inner_pad}            event.set(\"{target}.domain\", host)?;\n\
         {inner_pad}        }}\n\
         {inner_pad}        if let Some(port) = url.port() {{\n\
         {inner_pad}            event.set(\"{target}.port\", json!(port))?;\n\
         {inner_pad}        }}\n\
         {inner_pad}        event.set(\"{target}.path\", url.path())?;\n\
         {inner_pad}        if let Some(query) = url.query() {{\n\
         {inner_pad}            event.set(\"{target}.query\", query)?;\n\
         {inner_pad}        }}\n\
         {inner_pad}        if let Some(fragment) = url.fragment() {{\n\
         {inner_pad}            event.set(\"{target}.fragment\", fragment)?;\n\
         {inner_pad}        }}\n\
         {inner_pad}        if let Some(userinfo) = url.password() {{\n\
         {inner_pad}            event.set(\"{target}.user_info\", format!(\"{{}}:{{}}\", url.username(), userinfo))?;\n\
         {inner_pad}        }} else if !url.username().is_empty() {{\n\
         {inner_pad}            event.set(\"{target}.user_info\", url.username())?;\n\
         {inner_pad}        }}\n\
         {inner_pad}    }}\n\
         {inner_pad}}}\n",
        field_s = field_lit(field),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::Pipeline;

    fn codegen_body(yaml: &str) -> String {
        let pipeline = Pipeline::parse(yaml).unwrap();
        let gen = super::super::emit::PipelineCodegen::new(&pipeline, "test");
        gen.generate_body().unwrap()
    }

    #[test]
    fn set_string_value() {
        let code = codegen_body(r#"
processors:
  - set:
      field: event.kind
      value: event
"#);
        assert!(code.contains(r#"event.set("event.kind", json!("event"))?;"#));
    }

    #[test]
    fn set_number_value() {
        let code = codegen_body(r#"
processors:
  - set:
      field: event.severity
      value: 3
"#);
        assert!(code.contains(r#"event.set("event.severity", json!(3))?;"#));
    }

    #[test]
    fn set_bool_value() {
        let code = codegen_body(r#"
processors:
  - set:
      field: event.enriched
      value: true
"#);
        assert!(code.contains(r#"event.set("event.enriched", json!(true))?;"#));
    }

    #[test]
    fn set_copy_from() {
        let code = codegen_body(r#"
processors:
  - set:
      field: destination
      copy_from: source
"#);
        assert!(code.contains(r#"event.get("source").cloned()"#));
        assert!(code.contains(r#"event.set("destination""#));
    }

    #[test]
    fn set_template_variable() {
        let code = codegen_body(r#"
processors:
  - set:
      field: message
      value: 'Hello {{name}}'
"#);
        assert!(code.contains(r#"format!("Hello {}"#));
        assert!(code.contains(r#"event.get_str("name")"#));
    }

    #[test]
    fn remove_single() {
        let code = codegen_body(r#"
processors:
  - remove:
      field: _temp
      ignore_missing: true
"#);
        assert!(code.contains(r#"event.remove("_temp")"#));
        // Should not check for errors since ignore_missing is true
        assert!(!code.contains("FieldNotFound"));
    }

    #[test]
    fn remove_multiple() {
        let code = codegen_body(r#"
processors:
  - remove:
      field:
        - _temp1
        - _temp2
      ignore_missing: true
"#);
        assert!(code.contains(r#"event.remove("_temp1")"#));
        assert!(code.contains(r#"event.remove("_temp2")"#));
    }

    #[test]
    fn remove_required() {
        let code = codegen_body(r#"
processors:
  - remove:
      field: important
"#);
        assert!(code.contains("FieldNotFound"));
    }

    #[test]
    fn rename_basic() {
        let code = codegen_body(r#"
processors:
  - rename:
      field: source
      target_field: destination
"#);
        assert!(code.contains(r#"event.rename("source", "destination")?;"#));
    }

    #[test]
    fn rename_ignore_missing() {
        let code = codegen_body(r#"
processors:
  - rename:
      field: source
      target_field: destination
      ignore_missing: true
"#);
        assert!(code.contains(r#"if event.has("source")"#));
        assert!(code.contains(r#"event.rename("source", "destination")?;"#));
    }

    #[test]
    fn lowercase_basic() {
        let code = codegen_body(r#"
processors:
  - lowercase:
      field: message
"#);
        assert!(code.contains(r#"event.get_str("message")"#));
        assert!(code.contains("to_lowercase()"));
        assert!(code.contains(r#"event.set("message""#));
    }

    #[test]
    fn uppercase_with_target() {
        let code = codegen_body(r#"
processors:
  - uppercase:
      field: source
      target_field: destination
"#);
        assert!(code.contains(r#"event.get_str("source")"#));
        assert!(code.contains("to_uppercase()"));
        assert!(code.contains(r#"event.set("destination""#));
    }

    #[test]
    fn trim_basic() {
        let code = codegen_body(r#"
processors:
  - trim:
      field: message
"#);
        assert!(code.contains("trim()"));
        assert!(code.contains(r#"event.set("message""#));
    }

    #[test]
    fn convert_integer() {
        let code = codegen_body(r#"
processors:
  - convert:
      field: port
      type: integer
"#);
        assert!(code.contains("parse::<i64>()"));
    }

    #[test]
    fn convert_string() {
        let code = codegen_body(r#"
processors:
  - convert:
      field: count
      type: string
"#);
        assert!(code.contains("to_string()"));
    }

    #[test]
    fn drop_processor() {
        let code = codegen_body(r#"
processors:
  - drop:
      if: ctx?.severity > 7
"#);
        assert!(code.contains("TransformResult::Drop"));
        assert!(code.contains("ctx?.severity > 7"));
    }

    #[test]
    fn split_basic() {
        let code = codegen_body(r#"
processors:
  - split:
      field: tags
      separator: ","
"#);
        assert!(code.contains(r#"split(",")"#));
        assert!(code.contains(r#"event.set("tags""#));
    }

    #[test]
    fn full_module_generation() {
        let yaml = r#"
description: "Test pipeline"
processors:
  - set:
      field: event.kind
      value: event
  - remove:
      field: _temp
      ignore_missing: true
  - lowercase:
      field: host.name
"#;
        let pipeline = Pipeline::parse(yaml).unwrap();
        let gen = super::super::emit::PipelineCodegen::new(&pipeline, "test_module");
        let code = gen.generate().unwrap();

        // Verify structure
        assert!(code.contains("use dfe_runtime::prelude::*;"));
        assert!(code.contains("pub struct TestModule;"));
        assert!(code.contains("impl Transform for TestModule {"));
        assert!(code.contains("fn name(&self) -> &str"));
        assert!(code.contains("fn transform(&self, event: &mut Event) -> Result<TransformResult>"));
        assert!(code.contains("Ok(TransformResult::Continue)"));

        // Verify all three processors are present
        assert!(code.contains(r#"event.set("event.kind""#));
        assert!(code.contains(r#"event.remove("_temp")"#));
        assert!(code.contains("to_lowercase()"));
    }
}
