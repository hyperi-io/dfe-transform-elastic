// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Per-processor Rust code generation.
//!
//! Each supported processor has a function that emits Rust statements
//! operating on an `Event`. The generated code uses the dfe-runtime API
//! (event.get(), event.set(), event.remove(), event.rename(), etc).

use anyhow::{Result, bail};

use crate::pipeline::Processor;
use crate::pipeline::conditional::Conditional;
use crate::pipeline::processors::*;
use crate::pipeline::template_string::{TemplateFragment, TemplateString};

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
        Processor::Gsub(p) => emit_gsub(p, &pad),
        Processor::Json(p) => emit_json(p, &pad),
        Processor::Csv(p) => emit_csv(p, &pad),
        Processor::Kv(p) => emit_kv(p, &pad),
        Processor::Dissect(p) => emit_dissect(p, &pad),
        Processor::Grok(p) => emit_grok(p, &pad),
        Processor::Foreach(p) => emit_foreach(p, &pad),
        Processor::Date(p) => emit_date(p, &pad),
        Processor::RegisteredDomain(p) => emit_registered_domain(p, &pad),
        Processor::NetworkDirection(p) => emit_network_direction(p, &pad),
        Processor::Fingerprint(p) => emit_fingerprint(p, &pad),
        Processor::Pipeline(p) => emit_pipeline(p, &pad),
        Processor::Geoip(p) => emit_geoip(p, &pad),
        Processor::UserAgent(p) => emit_user_agent(p, &pad),
        Processor::CommunityId(p) => emit_community_id(p, &pad),
        Processor::Script(p) => emit_script(p, &pad),
    }
}

// -- Helpers ----------------------------------------------------------------

/// Format a dotted field path as a Rust string literal.
fn field_lit(field: &str) -> String {
    format!("\"{}\"", field)
}

/// Add one level of indentation (4 spaces) to a pad string.
fn indent(pad: &str) -> String {
    format!("{pad}    ")
}

/// Wrap generated statements in an `if` block when a condition is present.
///
/// Attempts to transpile the Painless conditional expression to Rust.
/// Falls back to a TODO comment if the expression is too complex.
///
/// Uses a `let` binding to evaluate the condition before the `if` block,
/// avoiding borrow conflicts between condition evaluation and body mutation.
fn wrap_conditional(cond: &Option<Conditional>, body: &str, pad: &str) -> String {
    match cond {
        Some(c) => {
            if let Some(rust_cond) = super::condition::transpile_condition(&c.0) {
                // Bind condition to a bool to drop any immutable borrows before the body
                format!("{pad}let _cond = {{ {rust_cond} }};\n{pad}if _cond {{\n{body}{pad}}}\n")
            } else {
                // Fallback: emit TODO comment and run unconditionally
                format!(
                    "{pad}// TODO: conditional not transpiled: {expr}\n{pad}{{\n{body}{pad}}}\n",
                    expr = c.0,
                )
            }
        }
        None => body.to_string(),
    }
}

/// Wrap a block of statements in ignore_failure handling.
fn wrap_ignore_failure(ignore: Option<bool>, body: &str, pad: &str) -> String {
    if ignore == Some(true) {
        let ip = indent(pad);
        format!(
            "{pad}// ignore_failure: true\n{pad}let _ = (|| -> Result<()> {{\n{body}{ip}Ok(())\n{pad}}})();\n"
        )
    } else {
        body.to_string()
    }
}

/// Wrap a field access in ignore_missing handling.
fn wrap_ignore_missing(ignore: Option<bool>, field: &str, body: &str, pad: &str) -> String {
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
                args.push(format!("event.get_str({}).unwrap_or(\"\")", field_lit(var)));
            }
        }
    }

    if args.is_empty() {
        format!("json!(\"{}\")", fmt_str)
    } else {
        format!("json!(format!(\"{}\", {}))", fmt_str, args.join(", "))
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
    let ip = indent(pad);
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
                let items: Vec<String> = arr
                    .iter()
                    .map(|s| format!("\"{}\"", escape_json_str(s)))
                    .collect();
                format!("json!([{}])", items.join(", "))
            }
        }
    } else {
        bail!("set processor has neither value nor copy_from");
    };

    // Handle override: false
    if p.override_values == Some(false) {
        body.push_str(&format!(
            "{pad}if !event.has({field_s}) {{\n{ip}event.set({field_s}, {value})?;\n{pad}}}\n",
            field_s = field_lit(field),
            value = value_expr,
        ));
    } else {
        body.push_str(&format!(
            "{pad}event.set({}, {})?;\n",
            field_lit(field),
            value_expr,
        ));
    }

    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.condition, &body, pad);
    Ok(body)
}

fn emit_remove(p: &remove::Remove, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let mut body = String::new();

    let fields = match &p.field {
        remove::Field::One(f) => vec![f.as_str()],
        remove::Field::Many(fs) => fs.iter().map(|s| s.as_str()).collect(),
    };

    let ignore_missing = p.ignore_missing == Some(true);

    for field in &fields {
        if ignore_missing {
            body.push_str(&format!("{ip}event.remove({});\n", field_lit(field),));
        } else {
            body.push_str(&format!(
                "{ip}if event.remove({f}).is_none() {{\n{ip}    return Err(TransformError::FieldNotFound {{ path: {f}.into() }}.into());\n{ip}}}\n",
                f = field_lit(field),
            ));
        }
    }

    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_rename(p: &rename::Rename, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let mut body = String::new();

    let from = p.field.raw();
    let to = p.target_field.raw();

    if p.ignore_missing == Some(true) {
        body.push_str(&format!(
            "{ip}if event.has({from_s}) {{\n{ip}    event.rename({from_s}, {to_s})?;\n{ip}}}\n",
            from_s = field_lit(from),
            to_s = field_lit(to),
        ));
    } else {
        body.push_str(&format!(
            "{ip}event.rename({}, {})?;\n",
            field_lit(from),
            field_lit(to),
        ));
    }

    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_append(p: &append::Append, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let mut body = String::new();
    let field = &p.field;

    match &p.value {
        append::AppendValue::String(ts) => {
            let value_expr = emit_template_value(ts);
            body.push_str(&format!(
                "{ip}event.append({}, {})?;\n",
                field_lit(field),
                value_expr,
            ));
        }
        append::AppendValue::Array(items) => {
            for item in items {
                let value_expr = emit_template_value(item);
                body.push_str(&format!(
                    "{ip}event.append({}, {})?;\n",
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
    let ip = indent(pad);
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(s) = event.get_string({field_s}) {{\n\
         {ip}    let lowered = s.to_lowercase();\n\
         {ip}    event.set({target_s}, lowered)?;\n\
         {ip}}}\n",
        field_s = field_lit(field),
        target_s = field_lit(target),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_uppercase(p: &uppercase::Uppercase, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(s) = event.get_string({field_s}) {{\n\
         {ip}    let uppered = s.to_uppercase();\n\
         {ip}    event.set({target_s}, uppered)?;\n\
         {ip}}}\n",
        field_s = field_lit(field),
        target_s = field_lit(target),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_trim(p: &trim::Trim, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(s) = event.get_string({field_s}) {{\n\
         {ip}    let trimmed = s.trim().to_string();\n\
         {ip}    event.set({target_s}, trimmed)?;\n\
         {ip}}}\n",
        field_s = field_lit(field),
        target_s = field_lit(target),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_convert(p: &convert::Convert, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);

    let conversion = match p.into_type {
        convert::IntoType::Integer | convert::IntoType::Long => format!(
            "{ip}if let Some(val) = event.get({field_s}) {{\n\
             {ip}    let converted = match val {{\n\
             {ip}        Value::String(s) => {{\n\
             {ip}            let s = s.trim();\n\
             {ip}            if let Some(hex) = s.strip_prefix(\"0x\") {{\n\
             {ip}                json!(i64::from_str_radix(hex, 16).map_err(|_| TransformError::ParseError {{ path: {field_s}.into(), message: format!(\"cannot convert '{{}}' to integer\", s) }})?)\n\
             {ip}            }} else {{\n\
             {ip}                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {{ path: {field_s}.into(), message: format!(\"cannot convert '{{}}' to integer\", s) }})?)\n\
             {ip}            }}\n\
             {ip}        }}\n\
             {ip}        Value::Number(n) => json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64)),\n\
             {ip}        Value::Bool(b) => json!(if *b {{ 1 }} else {{ 0 }}),\n\
             {ip}        _ => return Err(TransformError::ParseError {{ path: {field_s}.into(), message: \"cannot convert to integer\".into() }}.into()),\n\
             {ip}    }};\n\
             {ip}    event.set({target_s}, converted)?;\n\
             {ip}}}\n",
            field_s = field_lit(field),
            target_s = field_lit(target),
        ),
        convert::IntoType::Float => format!(
            "{ip}if let Some(val) = event.get({field_s}) {{\n\
             {ip}    let converted = match val {{\n\
             {ip}        Value::String(s) => json!(s.trim().parse::<f64>().map_err(|_| TransformError::ParseError {{ path: {field_s}.into(), message: format!(\"cannot convert '{{}}' to float\", s) }})?),\n\
             {ip}        Value::Number(n) => json!(n.as_f64().unwrap_or(0.0)),\n\
             {ip}        Value::Bool(b) => json!(if *b {{ 1.0 }} else {{ 0.0 }}),\n\
             {ip}        _ => return Err(TransformError::ParseError {{ path: {field_s}.into(), message: \"cannot convert to float\".into() }}.into()),\n\
             {ip}    }};\n\
             {ip}    event.set({target_s}, converted)?;\n\
             {ip}}}\n",
            field_s = field_lit(field),
            target_s = field_lit(target),
        ),
        convert::IntoType::String => format!(
            "{ip}if let Some(val) = event.get({field_s}) {{\n\
             {ip}    let converted = match val {{\n\
             {ip}        Value::String(_) => val.clone(),\n\
             {ip}        Value::Number(n) => json!(n.to_string()),\n\
             {ip}        Value::Bool(b) => json!(b.to_string()),\n\
             {ip}        Value::Null => json!(\"null\"),\n\
             {ip}        _ => json!(val.to_string()),\n\
             {ip}    }};\n\
             {ip}    event.set({target_s}, converted)?;\n\
             {ip}}}\n",
            field_s = field_lit(field),
            target_s = field_lit(target),
        ),
        convert::IntoType::IP => format!(
            "{ip}if let Some(s) = event.get_string({field_s}) {{\n\
             {ip}    // Validate IP format\n\
             {ip}    let s = s.trim();\n\
             {ip}    if s.parse::<std::net::IpAddr>().is_err() {{\n\
             {ip}        return Err(TransformError::ParseError {{ path: {field_s}.into(), message: format!(\"cannot convert '{{}}' to IP\", s) }}.into());\n\
             {ip}    }}\n\
             {ip}    event.set({target_s}, s)?;\n\
             {ip}}}\n",
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
    let ip = indent(pad);
    let mut body = String::new();

    // Drop always has a condition
    body.push_str(&format!("{ip}return Ok(TransformResult::Drop);\n"));

    // Always wrapped in conditional
    let body = wrap_conditional(&Some(p.condition.clone()), &body, pad);
    Ok(body)
}

fn emit_split(p: &split::Split, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);
    let separator = escape_json_str(&p.separator);

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(s) = event.get_string({field_s}) {{\n\
         {ip}    let parts: Vec<Value> = s.split(\"{sep}\").map(|p| json!(p)).collect();\n\
         {ip}    event.set({target_s}, Value::Array(parts))?;\n\
         {ip}}}\n",
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
    let ip = indent(pad);
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(uri_str) = event.get_string({field_s}) {{\n\
         {ip}    if let Ok(url) = url::Url::parse(uri_str) {{\n\
         {ip}        event.set(\"{target}.scheme\", url.scheme())?;\n\
         {ip}        if let Some(host) = url.host_str() {{\n\
         {ip}            event.set(\"{target}.domain\", host)?;\n\
         {ip}        }}\n\
         {ip}        if let Some(port) = url.port() {{\n\
         {ip}            event.set(\"{target}.port\", json!(port))?;\n\
         {ip}        }}\n\
         {ip}        event.set(\"{target}.path\", url.path())?;\n\
         {ip}        if let Some(query) = url.query() {{\n\
         {ip}            event.set(\"{target}.query\", query)?;\n\
         {ip}        }}\n\
         {ip}        if let Some(fragment) = url.fragment() {{\n\
         {ip}            event.set(\"{target}.fragment\", fragment)?;\n\
         {ip}        }}\n\
         {ip}        if let Some(userinfo) = url.password() {{\n\
         {ip}            event.set(\"{target}.user_info\", format!(\"{{}}:{{}}\", url.username(), userinfo))?;\n\
         {ip}        }} else if !url.username().is_empty() {{\n\
         {ip}            event.set(\"{target}.user_info\", url.username())?;\n\
         {ip}        }}\n\
         {ip}    }}\n\
         {ip}}}\n",
        field_s = field_lit(field),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_gsub(p: &gsub::Gsub, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);
    let pattern = escape_json_str(&p.pattern);
    let replacement = escape_json_str(&p.replacement);

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(s) = event.get_string({field_s}) {{\n\
         {ip}    let re = regex::Regex::new(\"{pattern}\").unwrap();\n\
         {ip}    let replaced = re.replace_all(&s, \"{replacement}\").into_owned();\n\
         {ip}    event.set({target_s}, replaced)?;\n\
         {ip}}}\n",
        field_s = field_lit(field),
        target_s = field_lit(target),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_json(p: &json::Json, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or(field);

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(s) = event.get_string({field_s}) {{\n\
         {ip}    let parsed: Value = serde_json::from_str(&s)\n\
         {ip}        .map_err(|e| TransformError::ParseError {{\n\
         {ip}            path: {field_s}.into(),\n\
         {ip}            message: format!(\"failed to parse JSON: {{}}\", e),\n\
         {ip}        }})?;\n\
         {ip}    event.set({target_s}, parsed)?;\n\
         {ip}}}\n",
        field_s = field_lit(field),
        target_s = field_lit(target),
    ));

    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_csv(p: &csv::Csv, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;
    let separator = p.separator.as_deref().unwrap_or(",");
    let quote_char = p.quote.as_deref().unwrap_or("\"");

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(csv_str) = event.get_string({field_s}) {{\n\
         {ip}    let mut rdr = csv::ReaderBuilder::new()\n\
         {ip}        .delimiter(b'{sep}')\n\
         {ip}        .quote(b'{quote}')\n\
         {ip}        .has_headers(false)\n\
         {ip}        .from_reader(csv_str.as_bytes());\n\
         {ip}    if let Some(Ok(record)) = rdr.records().next() {{\n",
        field_s = field_lit(field),
        sep = escape_json_str(separator),
        quote = escape_json_str(quote_char),
    ));

    for (i, target_field) in p.target_fields.iter().enumerate() {
        if target_field.is_empty() {
            continue;
        }
        body.push_str(&format!(
            "{ip}        if let Some(val) = record.get({i}) {{\n\
             {ip}            if !val.is_empty() {{\n\
             {ip}                event.set({target}, val)?;\n\
             {ip}            }}\n\
             {ip}        }}\n",
            target = field_lit(target_field),
        ));
    }

    body.push_str(&format!(
        "{ip}    }}\n\
         {ip}}}\n"
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_kv(p: &kv::KV, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;
    let field_split = escape_json_str(&p.field_split);
    let value_split = escape_json_str(&p.value_split);

    let target_prefix = p
        .target_field
        .as_ref()
        .map(|dp| format!("{}.", dp.raw()))
        .unwrap_or_default();

    let trim_enabled = p.trim_key == Some(" ".to_string()) || p.trim_value == Some(" ".to_string());

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(kv_str) = event.get_string({field_s}) {{\n\
         {ip}    for pair in kv_str.split(\"{field_split}\") {{\n\
         {ip}        if let Some((key, value)) = pair.split_once(\"{value_split}\") {{\n",
        field_s = field_lit(field),
    ));

    if trim_enabled {
        body.push_str(&format!(
            "{ip}            let key = key.trim();\n\
             {ip}            let value = value.trim();\n"
        ));
    }

    body.push_str(&format!(
        "{ip}            if !key.is_empty() {{\n\
         {ip}                event.set(&format!(\"{target_prefix}{{}}\", key), value)?;\n\
         {ip}            }}\n\
         {ip}        }}\n\
         {ip}    }}\n\
         {ip}}}\n"
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_dissect(p: &dissect::Dissect, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;

    // Parse the dissect pattern into literal and field segments
    let segments = parse_dissect_pattern(&p.pattern);

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(input) = event.get_string({field_s}) {{\n\
         {ip}    let mut remaining: &str = &input;\n",
        field_s = field_lit(field),
    ));

    for (i, seg) in segments.iter().enumerate() {
        match seg {
            DissectSegment::Literal(lit) => {
                let escaped = escape_json_str(lit);
                body.push_str(&format!(
                    "{ip}    if let Some(rest) = remaining.strip_prefix(\"{escaped}\") {{\n\
                     {ip}        remaining = rest;\n\
                     {ip}    }}\n"
                ));
            }
            DissectSegment::Field(name) => {
                // Look ahead for next literal to know where to split
                let next_lit = segments.get(i + 1).and_then(|s| match s {
                    DissectSegment::Literal(l) => Some(l.as_str()),
                    _ => None,
                });

                if let Some(delim) = next_lit {
                    if !name.is_empty() {
                        let escaped = escape_json_str(delim);
                        body.push_str(&format!(
                            "{ip}    if let Some(pos) = remaining.find(\"{escaped}\") {{\n\
                             {ip}        event.set({target}, &remaining[..pos])?;\n\
                             {ip}        remaining = &remaining[pos..];\n\
                             {ip}    }}\n",
                            target = field_lit(name),
                        ));
                    } else {
                        // Empty field name — skip/consume up to delimiter
                        let escaped = escape_json_str(delim);
                        body.push_str(&format!(
                            "{ip}    if let Some(pos) = remaining.find(\"{escaped}\") {{\n\
                             {ip}        remaining = &remaining[pos..];\n\
                             {ip}    }}\n"
                        ));
                    }
                } else if !name.is_empty() {
                    // Last field — take the rest
                    body.push_str(&format!(
                        "{ip}    event.set({target}, remaining)?;\n",
                        target = field_lit(name),
                    ));
                }
            }
        }
    }

    body.push_str(&format!("{ip}}}\n"));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

/// Segments of a dissect pattern.
#[derive(Debug)]
enum DissectSegment {
    Literal(String),
    Field(String),
}

/// Parse a dissect pattern like `"Hello, %{subject} from %{source}"` into segments.
fn parse_dissect_pattern(pattern: &str) -> Vec<DissectSegment> {
    let mut segments = Vec::new();
    let mut remaining = pattern;

    while !remaining.is_empty() {
        if let Some(start) = remaining.find("%{") {
            if start > 0 {
                segments.push(DissectSegment::Literal(remaining[..start].to_string()));
            }
            remaining = &remaining[start + 2..];

            // Handle right-padding modifier: %{field->}
            if let Some(end) = remaining.find('}') {
                let mut field_name = &remaining[..end];
                // Strip append modifier (+) and right-padding (->)
                if let Some(stripped) = field_name.strip_suffix("->") {
                    field_name = stripped;
                }
                if let Some(stripped) = field_name.strip_prefix('+') {
                    field_name = stripped;
                }
                // Strip reference modifier (&, *)
                if let Some(stripped) = field_name.strip_prefix('&') {
                    field_name = stripped;
                }
                if let Some(stripped) = field_name.strip_prefix('*') {
                    field_name = stripped;
                }
                segments.push(DissectSegment::Field(field_name.to_string()));
                remaining = &remaining[end + 1..];
            } else {
                break;
            }
        } else {
            segments.push(DissectSegment::Literal(remaining.to_string()));
            break;
        }
    }

    segments
}

fn emit_grok(p: &grok::Grok, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;

    let mut body = String::new();

    // Build pattern definitions map if present
    if let Some(pattern_defs) = &p.pattern_definitions {
        body.push_str(&format!("{ip}// Pattern definitions for grok\n"));
        for (name, pattern) in pattern_defs {
            body.push_str(&format!("{ip}// {name} = {pattern}\n",));
        }
    }

    // Try each grok pattern in order
    body.push_str(&format!(
        "{ip}if let Some(input) = event.get_string({field_s}) {{\n",
        field_s = field_lit(field),
    ));

    for (i, pattern) in p.patterns.iter().enumerate() {
        let escaped = escape_json_str(pattern);
        if i == 0 {
            body.push_str(&format!(
                "{ip}    // Grok pattern: {escaped}\n\
                 {ip}    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)\n\
                 {ip}    let (grok_pattern, grok_field_map) = grok_to_regex_with_map(\"{escaped}\");\n\
                 {ip}    let grok_re = regex::Regex::new(&grok_pattern).unwrap();\n\
                 {ip}    if let Some(caps) = grok_re.captures(&input) {{\n\
                 {ip}        for name in grok_re.capture_names().flatten() {{\n\
                 {ip}            if let Some(m) = caps.name(name) {{\n\
                 {ip}                let field_path = grok_field_map.get(name).map(|s| s.as_str()).unwrap_or(name);\n\
                 {ip}                event.set(field_path, m.as_str())?;\n\
                 {ip}            }}\n\
                 {ip}        }}\n\
                 {ip}    }}\n"
            ));
        } else {
            body.push_str(&format!(
                "{ip}    // Additional grok pattern {i}: {escaped}\n"
            ));
        }
    }

    body.push_str(&format!("{ip}}}\n"));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_foreach(p: &foreach::Foreach, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;

    // Generate the inner processor code
    let inner_code = emit_processor(&p.processor, 0)?;

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(arr) = event.get({field_s}).cloned() {{\n\
         {ip}    if let Value::Array(items) = arr {{\n\
         {ip}        for (idx, _item) in items.iter().enumerate() {{\n\
         {ip}            // Set _ingest._value for inner processor access\n\
         {ip}            let item_path = format!(\"{field}[{{}}]\", idx);\n\
         {ip}            // Inner processor operates on the element:\n",
        field_s = field_lit(field),
    ));

    // Emit the inner processor code, indented appropriately
    for line in inner_code.lines() {
        if !line.trim().is_empty() {
            body.push_str(&format!("{ip}            {}\n", line.trim()));
        }
    }

    body.push_str(&format!(
        "{ip}        }}\n\
         {ip}    }}\n\
         {ip}}}\n"
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_date(p: &date::Date, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or("@timestamp");

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(date_str) = event.get_as_string({field_s}) {{\n",
        field_s = field_lit(field),
    ));

    // Emit format-based parsing attempts
    let formats: Vec<String> = p
        .formats
        .iter()
        .map(|f| match f {
            date::TimeFormats::ISO8601 => "ISO8601".to_string(),
            date::TimeFormats::UNIX => "UNIX".to_string(),
            date::TimeFormats::UNIX_MS => "UNIX_MS".to_string(),
            date::TimeFormats::TAI64N => "TAI64N".to_string(),
            date::TimeFormats::Custom(c) => format!("{:?}", c),
        })
        .collect();

    for fmt in &formats {
        match fmt.as_str() {
            "ISO8601" => {
                body.push_str(&format!(
                    "{ip}    // Try ISO8601 format\n\
                     {ip}    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)\n\
                     {ip}        .or_else(|_| chrono::DateTime::parse_from_str(&date_str, \"%Y-%m-%dT%H:%M:%S%.f%:z\"))\n\
                     {ip}        .or_else(|_| chrono::DateTime::parse_from_str(&date_str, \"%Y-%m-%dT%H:%M:%S%:z\"))\n\
                     {ip}    {{\n\
                     {ip}        event.set({target_s}, dt.format(\"%Y-%m-%dT%H:%M:%S%.3fZ\").to_string())?;\n\
                     {ip}    }}\n",
                    target_s = field_lit(target),
                ));
            }
            "UNIX" => {
                body.push_str(&format!(
                    "{ip}    // Try UNIX timestamp (skip epoch 0)\n\
                     {ip}    if let Ok(ts) = date_str.parse::<f64>() {{\n\
                     {ip}        if ts > 0.0 {{\n\
                     {ip}            let secs = ts as i64;\n\
                     {ip}            let nsecs = ((ts - secs as f64) * 1_000_000_000.0) as u32;\n\
                     {ip}            if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {{\n\
                     {ip}                event.set({target_s}, dt.format(\"%Y-%m-%dT%H:%M:%S%.3fZ\").to_string())?;\n\
                     {ip}            }}\n\
                     {ip}        }}\n\
                     {ip}    }}\n",
                    target_s = field_lit(target),
                ));
            }
            "UNIX_MS" => {
                body.push_str(&format!(
                    "{ip}    // Try UNIX_MS timestamp (skip epoch 0)\n\
                     {ip}    if let Ok(ms) = date_str.parse::<i64>() {{\n\
                     {ip}        if ms > 0 {{\n\
                     {ip}            if let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {{\n\
                     {ip}                event.set({target_s}, dt.format(\"%Y-%m-%dT%H:%M:%S%.3fZ\").to_string())?;\n\
                     {ip}            }}\n\
                     {ip}        }}\n\
                     {ip}    }}\n",
                    target_s = field_lit(target),
                ));
            }
            java_fmt => {
                // Java datetime format → emit as comment with chrono parse attempt
                let escaped = escape_json_str(java_fmt);
                body.push_str(&format!(
                    "{ip}    // Try Java datetime format: {escaped}\n\
                     {ip}    // TODO: Convert Java format to chrono strftime (date processor 2.2.3)\n\
                     {ip}    // chrono::NaiveDateTime::parse_from_str(&date_str, \"{escaped}\")\n"
                ));
            }
        }
    }

    body.push_str(&format!("{ip}}}\n"));

    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_registered_domain(p: &registered_domain::RegisteredDomain, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;

    let prefix = match &p.target_field {
        Some(t) => format!("{}.", t),
        None => String::new(),
    };

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(domain_str) = event.get_string({field_s}) {{\n\
         {ip}    let domain = domain_str.to_string();\n\
         {ip}    event.set(\"{prefix}domain\", json!(domain.clone()))?;\n\
         {ip}    // Public suffix list lookup for registered domain extraction\n\
         {ip}    if let Some(rd) = registered_domain_lookup(&domain) {{\n\
         {ip}        event.set(\"{prefix}registered_domain\", json!(rd.registered_domain))?;\n\
         {ip}        event.set(\"{prefix}top_level_domain\", json!(rd.top_level_domain))?;\n\
         {ip}        if let Some(sub) = rd.subdomain {{\n\
         {ip}            event.set(\"{prefix}subdomain\", json!(sub))?;\n\
         {ip}        }}\n\
         {ip}    }}\n\
         {ip}}}\n",
        field_s = field_lit(field),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    Ok(body)
}

fn emit_network_direction(p: &network_direction::NetworkDirection, pad: &str) -> Result<String> {
    let ip = indent(pad);

    let networks_field = p
        .internal_networks_field
        .as_deref()
        .unwrap_or("internal_networks");

    let source_ip = "source.ip";
    let dest_ip = "destination.ip";

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}// Classify network direction based on internal network ranges\n\
         {ip}if let (Some(src), Some(dst)) = (event.get_string({src_s}), event.get_string({dst_s})) {{\n\
         {ip}    if let Some(networks) = event.get_array({net_s}) {{\n\
         {ip}        let networks: Vec<String> = networks.iter()\n\
         {ip}            .filter_map(|v| v.as_str().map(|s| s.to_string()))\n\
         {ip}            .collect();\n\
         {ip}        let src_internal = is_internal_ip(&src, &networks);\n\
         {ip}        let dst_internal = is_internal_ip(&dst, &networks);\n\
         {ip}        let direction = match (src_internal, dst_internal) {{\n\
         {ip}            (true, false) => \"outbound\",\n\
         {ip}            (false, true) => \"inbound\",\n\
         {ip}            (true, true) => \"internal\",\n\
         {ip}            (false, false) => \"external\",\n\
         {ip}        }};\n\
         {ip}        event.set(\"network.direction\", json!(direction))?;\n\
         {ip}    }}\n\
         {ip}}}\n",
        src_s = field_lit(source_ip),
        dst_s = field_lit(dest_ip),
        net_s = field_lit(networks_field),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, source_ip, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_fingerprint(p: &fingerprint::Fingerprint, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let target = p.target_field.as_deref().unwrap_or("_id");

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}{{\n\
         {ip}    use sha2::{{Sha256, Digest}};\n\
         {ip}    let mut hasher = Sha256::new();\n"
    ));

    for field in &p.fields {
        if p.ignore_missing == Some(true) {
            body.push_str(&format!(
                "{ip}    if let Some(v) = event.get({field_s}) {{\n\
                 {ip}        hasher.update(v.to_string().as_bytes());\n\
                 {ip}    }}\n",
                field_s = field_lit(field),
            ));
        } else {
            body.push_str(&format!(
                "{ip}    if let Some(v) = event.get({field_s}) {{\n\
                 {ip}        hasher.update(v.to_string().as_bytes());\n\
                 {ip}    }} else {{\n\
                 {ip}        return Err(TransformError::FieldNotFound(\"{field}\".into()).into());\n\
                 {ip}    }}\n",
                field_s = field_lit(field),
            ));
        }
    }

    body.push_str(&format!(
        "{ip}    let hash = format!(\"{{:x}}\", hasher.finalize());\n\
         {ip}    event.set({target_s}, json!(hash))?;\n\
         {ip}}}\n",
        target_s = field_lit(target),
    ));

    Ok(body)
}

fn emit_pipeline(p: &nested_pipeline::NestedPipeline, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let name = &p.name.0;

    let mut body = String::new();

    if let Some(ref inner) = p.inner_pipeline {
        body.push_str(&format!("{ip}// Begin nested pipeline: \"{name}\"\n"));

        for processor in &inner.processors {
            let proc_code = emit_processor(processor, 0)?;
            for line in proc_code.lines() {
                if !line.trim().is_empty() {
                    body.push_str(&format!("{ip}{}\n", line.trim()));
                }
            }
        }

        body.push_str(&format!("{ip}// End nested pipeline: \"{name}\"\n"));
    } else {
        body.push_str(&format!(
            "{ip}// Nested pipeline reference: \"{name}\"\n\
             {ip}// TODO: Resolve and inline pipeline \"{name}\" at build time\n\
             {ip}{name}_pipeline.transform(event)?;\n",
            name = name.replace('-', "_"),
        ));
    }

    let body = wrap_conditional(&p.condition, &body, pad);
    Ok(body)
}

fn emit_geoip(p: &geoip::Geoip, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;
    let db = p.database_file.unwrap_or_default();
    let target = p.target_field.as_deref().unwrap_or("geoip");

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(ip_str) = event.get_string({field_s}) {{\n\
         {ip}    let ip_str = ip_str.to_string();\n\
         {ip}    // GeoIP enrichment ({db})\n\
         {ip}    if let Ok(geo) = geoip_lookup(\"{table}\", &ip_str) {{\n",
        field_s = field_lit(field),
        table = db.table_name(),
    ));

    // Emit property assignments based on DB type
    let properties = match &p.properties {
        Some(props) => props.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        None => match db {
            geoip::GeoIPDB::CITY => vec![
                "country_iso_code",
                "country_name",
                "continent_name",
                "region_iso_code",
                "region_name",
                "city_name",
                "timezone",
                "location",
            ],
            geoip::GeoIPDB::COUNTRY => vec!["country_iso_code", "country_name", "continent_name"],
            geoip::GeoIPDB::ASN => vec!["asn", "organization_name", "network"],
        },
    };

    for prop in &properties {
        body.push_str(&format!(
            "{ip}        if let Some(v) = geo.get(\"{prop}\") {{\n\
             {ip}            event.set(\"{target}.{prop}\", v.clone())?;\n\
             {ip}        }}\n"
        ));
    }

    body.push_str(&format!(
        "{ip}    }}\n\
         {ip}}}\n"
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

fn emit_user_agent(p: &user_agent::UserAgent, pad: &str) -> Result<String> {
    let ip = indent(pad);
    let field = &p.field;
    let target = p.target_field.as_deref().unwrap_or("user_agent");

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}if let Some(ua_str) = event.get_string({field_s}) {{\n\
         {ip}    let ua_str = ua_str.to_string();\n\
         {ip}    // User agent parsing\n\
         {ip}    if let Ok(ua) = parse_user_agent(&ua_str) {{\n\
         {ip}        event.set(\"{target}.original\", json!(ua_str))?;\n\
         {ip}        if let Some(name) = ua.name {{ event.set(\"{target}.name\", json!(name))?; }}\n\
         {ip}        if let Some(version) = ua.version {{ event.set(\"{target}.version\", json!(version))?; }}\n\
         {ip}        if let Some(os_name) = ua.os_name {{\n\
         {ip}            event.set(\"{target}.os.name\", json!(os_name))?;\n\
         {ip}            if let Some(os_version) = ua.os_version {{\n\
         {ip}                event.set(\"{target}.os.version\", json!(os_version))?;\n\
         {ip}                event.set(\"{target}.os.full\", json!(format!(\"{{}} {{}}\", os_name, os_version)))?;\n\
         {ip}            }}\n\
         {ip}        }}\n\
         {ip}        if let Some(device) = ua.device {{ event.set(\"{target}.device.name\", json!(device))?; }}\n\
         {ip}    }}\n\
         {ip}}}\n",
        field_s = field_lit(field),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, field, &body, pad);
    Ok(body)
}

fn emit_community_id(p: &community_id::CommunityId, pad: &str) -> Result<String> {
    let ip = indent(pad);

    let src_ip = p.source_ip.as_deref().unwrap_or("source.ip");
    let src_port = p.source_port.as_deref().unwrap_or("source.port");
    let dst_ip = p.destination_ip.as_deref().unwrap_or("destination.ip");
    let dst_port = p.destination_port.as_deref().unwrap_or("destination.port");
    let target = p.target_field.as_deref().unwrap_or("network.community_id");

    let mut body = String::new();
    body.push_str(&format!(
        "{ip}// Community ID v1 hash\n\
         {ip}if let (Some(src_ip), Some(dst_ip)) = (\n\
         {ip}    event.get_string({src_ip_s}),\n\
         {ip}    event.get_string({dst_ip_s}),\n\
         {ip}) {{\n\
         {ip}    let src_port = event.get_i64({src_port_s}).unwrap_or(0) as u16;\n\
         {ip}    let dst_port = event.get_i64({dst_port_s}).unwrap_or(0) as u16;\n\
         {ip}    let protocol = event.get_string(\"network.transport\")\n\
         {ip}        .or_else(|| event.get_string(\"network.iana_number\"))\n\
         {ip}        .unwrap_or_else(|| \"tcp\".to_string());\n\
         {ip}    let cid = community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol);\n\
         {ip}    event.set({target_s}, json!(cid))?;\n\
         {ip}}}\n",
        src_ip_s = field_lit(src_ip),
        dst_ip_s = field_lit(dst_ip),
        src_port_s = field_lit(src_port),
        dst_port_s = field_lit(dst_port),
        target_s = field_lit(target),
    ));

    let body = wrap_ignore_missing(p.ignore_missing, src_ip, &body, pad);
    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    Ok(body)
}

fn emit_script(p: &script::Script, pad: &str) -> Result<String> {
    let ip = indent(pad);

    let mut body = String::new();

    if let Some(source) = &p.source {
        let escaped = escape_json_str(source);
        body.push_str(&format!(
            "{ip}// Painless script\n\
             {ip}// Source: {escaped}\n\
             {ip}// TODO: Transpile Painless to Rust (2.2.3)\n\
             {ip}painless_exec(event, r#\"{escaped}\"#)?;\n"
        ));
    } else if let Some(id) = &p.id {
        let escaped = escape_json_str(id);
        body.push_str(&format!(
            "{ip}// Painless script reference: {escaped}\n\
             {ip}// TODO: Resolve stored script and transpile (2.2.3)\n\
             {ip}painless_exec_id(event, \"{escaped}\")?;\n"
        ));
    }

    let body = wrap_ignore_failure(p.ignore_failure, &body, pad);
    let body = wrap_conditional(&p.conditional, &body, pad);
    Ok(body)
}

#[cfg(test)]
mod tests {
    use crate::pipeline::Pipeline;

    fn codegen_body(yaml: &str) -> String {
        let pipeline = Pipeline::parse(yaml).unwrap();
        let cg = super::super::emit::PipelineCodegen::new(&pipeline, "test");
        cg.generate_body().unwrap()
    }

    #[test]
    fn set_string_value() {
        let code = codegen_body(
            r#"
processors:
  - set:
      field: event.kind
      value: event
"#,
        );
        assert!(code.contains(r#"event.set("event.kind", json!("event"))?;"#));
    }

    #[test]
    fn set_number_value() {
        let code = codegen_body(
            r#"
processors:
  - set:
      field: event.severity
      value: 3
"#,
        );
        assert!(code.contains(r#"event.set("event.severity", json!(3))?;"#));
    }

    #[test]
    fn set_bool_value() {
        let code = codegen_body(
            r#"
processors:
  - set:
      field: event.enriched
      value: true
"#,
        );
        assert!(code.contains(r#"event.set("event.enriched", json!(true))?;"#));
    }

    #[test]
    fn set_copy_from() {
        let code = codegen_body(
            r#"
processors:
  - set:
      field: destination
      copy_from: source
"#,
        );
        assert!(code.contains(r#"event.get("source").cloned()"#));
        assert!(code.contains(r#"event.set("destination""#));
    }

    #[test]
    fn set_template_variable() {
        let code = codegen_body(
            r#"
processors:
  - set:
      field: message
      value: 'Hello {{name}}'
"#,
        );
        assert!(code.contains(r#"format!("Hello {}"#));
        assert!(code.contains(r#"event.get_str("name")"#));
    }

    #[test]
    fn remove_single() {
        let code = codegen_body(
            r#"
processors:
  - remove:
      field: _temp
      ignore_missing: true
"#,
        );
        assert!(code.contains(r#"event.remove("_temp")"#));
        // Should not check for errors since ignore_missing is true
        assert!(!code.contains("FieldNotFound"));
    }

    #[test]
    fn remove_multiple() {
        let code = codegen_body(
            r#"
processors:
  - remove:
      field:
        - _temp1
        - _temp2
      ignore_missing: true
"#,
        );
        assert!(code.contains(r#"event.remove("_temp1")"#));
        assert!(code.contains(r#"event.remove("_temp2")"#));
    }

    #[test]
    fn remove_required() {
        let code = codegen_body(
            r#"
processors:
  - remove:
      field: important
"#,
        );
        assert!(code.contains("FieldNotFound"));
    }

    #[test]
    fn rename_basic() {
        let code = codegen_body(
            r#"
processors:
  - rename:
      field: source
      target_field: destination
"#,
        );
        assert!(code.contains(r#"event.rename("source", "destination")?;"#));
    }

    #[test]
    fn rename_ignore_missing() {
        let code = codegen_body(
            r#"
processors:
  - rename:
      field: source
      target_field: destination
      ignore_missing: true
"#,
        );
        assert!(code.contains(r#"if event.has("source")"#));
        assert!(code.contains(r#"event.rename("source", "destination")?;"#));
    }

    #[test]
    fn lowercase_basic() {
        let code = codegen_body(
            r#"
processors:
  - lowercase:
      field: message
"#,
        );
        assert!(code.contains(r#"event.get_string("message")"#));
        assert!(code.contains("to_lowercase()"));
        assert!(code.contains(r#"event.set("message""#));
    }

    #[test]
    fn uppercase_with_target() {
        let code = codegen_body(
            r#"
processors:
  - uppercase:
      field: source
      target_field: destination
"#,
        );
        assert!(code.contains(r#"event.get_string("source")"#));
        assert!(code.contains("to_uppercase()"));
        assert!(code.contains(r#"event.set("destination""#));
    }

    #[test]
    fn trim_basic() {
        let code = codegen_body(
            r#"
processors:
  - trim:
      field: message
"#,
        );
        assert!(code.contains("trim()"));
        assert!(code.contains(r#"event.set("message""#));
    }

    #[test]
    fn convert_integer() {
        let code = codegen_body(
            r#"
processors:
  - convert:
      field: port
      type: integer
"#,
        );
        assert!(code.contains("parse::<i64>()"));
    }

    #[test]
    fn convert_string() {
        let code = codegen_body(
            r#"
processors:
  - convert:
      field: count
      type: string
"#,
        );
        assert!(code.contains("to_string()"));
    }

    #[test]
    fn drop_processor() {
        let code = codegen_body(
            r#"
processors:
  - drop:
      if: ctx?.severity > 7
"#,
        );
        assert!(code.contains("TransformResult::Drop"));
        assert!(code.contains("ctx?.severity > 7"));
    }

    #[test]
    fn split_basic() {
        let code = codegen_body(
            r#"
processors:
  - split:
      field: tags
      separator: ","
"#,
        );
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
        let cg = super::super::emit::PipelineCodegen::new(&pipeline, "test_module");
        let code = cg.generate().unwrap();

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

    // -- Medium processor unit tests --

    #[test]
    fn gsub_replace() {
        let code = codegen_body(
            r#"
processors:
  - gsub:
      field: message
      pattern: "\\."
      replacement: "_"
"#,
        );
        assert!(code.contains("regex::Regex::new("));
        assert!(code.contains("replace_all("));
        assert!(code.contains(r#"event.set("message""#));
    }

    #[test]
    fn gsub_target_field() {
        let code = codegen_body(
            r#"
processors:
  - gsub:
      field: message
      pattern: "-"
      replacement: "_"
      target_field: clean
"#,
        );
        assert!(code.contains(r#"event.set("clean""#));
    }

    #[test]
    fn json_parse() {
        let code = codegen_body(
            r#"
processors:
  - json:
      field: message
"#,
        );
        assert!(code.contains("serde_json::from_str("));
        assert!(code.contains(r#"event.set("message""#));
    }

    #[test]
    fn json_target_field() {
        let code = codegen_body(
            r#"
processors:
  - json:
      field: message
      target_field: parsed
"#,
        );
        assert!(code.contains(r#"event.set("parsed""#));
    }

    #[test]
    fn csv_parse() {
        let code = codegen_body(
            r#"
processors:
  - csv:
      field: message
      target_fields:
        - a
        - b
"#,
        );
        assert!(code.contains("csv::ReaderBuilder::new()"));
        assert!(code.contains(r#"event.set("a""#));
        assert!(code.contains(r#"event.set("b""#));
    }

    #[test]
    fn kv_parse() {
        let code = codegen_body(
            r#"
processors:
  - kv:
      field: message
      field_split: " "
      value_split: "="
"#,
        );
        assert!(code.contains("split(\" \")"));
        assert!(code.contains("split_once(\"=\")"));
    }

    #[test]
    fn dissect_parse() {
        let code = codegen_body(
            r#"
processors:
  - dissect:
      field: message
      pattern: "%{greeting}, %{subject}"
"#,
        );
        assert!(code.contains(r#"strip_prefix(", ")"#));
        assert!(code.contains(r#"event.set("greeting""#));
        assert!(code.contains(r#"event.set("subject""#));
    }

    #[test]
    fn grok_parse() {
        let code = codegen_body(
            r#"
processors:
  - grok:
      field: message
      patterns:
        - "%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level}"
      ignore_failure: true
"#,
        );
        assert!(code.contains("grok_to_regex_with_map("));
        assert!(code.contains("captures(&input)"));
        assert!(code.contains("ignore_failure: true"));
    }

    #[test]
    fn date_iso8601() {
        let code = codegen_body(
            r#"
processors:
  - date:
      field: timestamp
      formats:
        - ISO8601
"#,
        );
        assert!(code.contains("parse_from_rfc3339("));
        assert!(code.contains(r#"event.set("@timestamp""#));
    }

    #[test]
    fn date_unix_timestamp() {
        let code = codegen_body(
            r#"
processors:
  - date:
      field: ts
      formats:
        - UNIX
      target_field: event.created
"#,
        );
        assert!(code.contains("parse::<f64>()"));
        assert!(code.contains("from_timestamp("));
        assert!(code.contains(r#"event.set("event.created""#));
    }

    // -- Complex processor unit tests --

    #[test]
    fn registered_domain_basic() {
        let code = codegen_body(
            r#"
processors:
  - registered_domain:
      field: url.domain
      ignore_missing: true
"#,
        );
        assert!(code.contains(r#"event.get_string("url.domain")"#));
        assert!(code.contains("registered_domain_lookup("));
        assert!(code.contains(r#"event.set("registered_domain""#));
        assert!(code.contains(r#"event.set("top_level_domain""#));
    }

    #[test]
    fn registered_domain_target_field() {
        let code = codegen_body(
            r#"
processors:
  - registered_domain:
      field: source.domain
      target_field: source
"#,
        );
        assert!(code.contains(r#"event.set("source.domain""#));
        assert!(code.contains(r#"event.set("source.registered_domain""#));
        assert!(code.contains(r#"event.set("source.top_level_domain""#));
        assert!(code.contains(r#"event.set("source.subdomain""#));
    }

    #[test]
    fn network_direction_classify() {
        let code = codegen_body(
            r#"
processors:
  - network_direction:
      internal_networks_field: internal_networks
      ignore_missing: true
"#,
        );
        assert!(code.contains("is_internal_ip("));
        assert!(code.contains(r#"event.set("network.direction""#));
        assert!(code.contains(r#""outbound""#));
        assert!(code.contains(r#""inbound""#));
    }

    #[test]
    fn fingerprint_sha256() {
        let code = codegen_body(
            r#"
processors:
  - fingerprint:
      fields:
        - "@timestamp"
        - event.id
      target_field: _id
      ignore_missing: true
"#,
        );
        assert!(code.contains("Sha256"));
        assert!(code.contains("hasher.update("));
        assert!(code.contains(r#"event.set("_id""#));
    }

    #[test]
    fn fingerprint_required_fields() {
        let code = codegen_body(
            r#"
processors:
  - fingerprint:
      fields:
        - user.name
"#,
        );
        assert!(code.contains("FieldNotFound"));
        assert!(code.contains(r#"event.set("_id""#));
    }

    #[test]
    fn pipeline_nested_inline() {
        use std::collections::HashMap;

        let inner_yaml = r#"
processors:
  - set:
      field: target
      value: Hello
"#;
        let inner = Pipeline::parse(inner_yaml).unwrap();

        let yaml = r#"
processors:
  - pipeline:
      name: '{{< IngestPipeline "test-pipeline" >}}'
"#;
        let pipeline =
            Pipeline::parse_with_context(yaml, HashMap::from([("test-pipeline".into(), inner)]))
                .unwrap();
        let cg = super::super::emit::PipelineCodegen::new(&pipeline, "test");
        let code = cg.generate_body().unwrap();
        assert!(code.contains("test-pipeline"));
        assert!(code.contains(r#"event.set("target""#));
    }

    // -- Enrichment processor unit tests --

    #[test]
    fn geoip_city_default() {
        let code = codegen_body(
            r#"
processors:
  - geoip:
      field: source.ip
      ignore_missing: true
"#,
        );
        assert!(code.contains("geoip_lookup(\"geoip_city\""));
        assert!(code.contains(r#"event.set("geoip.country_iso_code""#));
        assert!(code.contains(r#"event.set("geoip.city_name""#));
    }

    #[test]
    fn geoip_asn() {
        let code = codegen_body(
            r#"
processors:
  - geoip:
      field: source.ip
      database_file: GeoLite2-ASN.mmdb
      target_field: source.as
"#,
        );
        assert!(code.contains("geoip_lookup(\"geoip_asn\""));
        assert!(code.contains(r#"event.set("source.as.asn""#));
        assert!(code.contains(r#"event.set("source.as.organization_name""#));
    }

    #[test]
    fn user_agent_parse() {
        let code = codegen_body(
            r#"
processors:
  - user_agent:
      field: agent
      target_field: user
"#,
        );
        assert!(code.contains("parse_user_agent("));
        assert!(code.contains(r#"event.set("user.original""#));
        assert!(code.contains(r#"event.set("user.name""#));
        assert!(code.contains(r#"event.set("user.os.name""#));
    }

    #[test]
    fn community_id_default() {
        let code = codegen_body(
            r#"
processors:
  - community_id:
      ignore_missing: true
"#,
        );
        assert!(code.contains("community_id_v1("));
        assert!(code.contains(r#"event.get_string("source.ip")"#));
        assert!(code.contains(r#"event.set("network.community_id""#));
    }

    #[test]
    fn script_painless() {
        let code = codegen_body(
            r#"
processors:
  - script:
      lang: painless
      source: "ctx.event.kind = 'event'"
      ignore_failure: true
"#,
        );
        assert!(code.contains("painless_exec("));
        assert!(code.contains("ctx.event.kind"));
    }
}
