// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `rest` pipeline.
pub struct Rest;

impl Transform for Rest {
    fn name(&self) -> &str {
        "rest"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        let _cond = { event.get_str("tychon.script.name").is_some_and(|s| s.to_lowercase().contains("host")) };
        if _cond {
        event.set("labels.source", json!("host"))?;
        }

        let _cond = { event.get_str("tychon.script.name").is_some_and(|s| s.to_lowercase().contains("listening")) };
        if _cond {
        event.set("labels.source", json!("listening"))?;
        }

        let _cond = { event.get("tychon.file.attributes").is_some_and(|v| v.is_array()) };
        if _cond {
            // Painless script
            // Source: def result = [];\nfor (def v : ctx.tychon.file.attributes) {\n  if (v instanceof String) {\n    result.add(v);\n  }\n}\nctx.tychon.file.attributes = result;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"def result = [];\nfor (def v : ctx.tychon.file.attributes) {\n  if (v instanceof String) {\n    result.add(v);\n  }\n}\nctx.tychon.file.attributes = result;\n"#))?;
        }

        if event.has_value("tychon.file.size") {
            if let Some(val) = event.get("tychon.file.size") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.file.size".into(),
                        message,
                    })?;
                event.set("tychon.file.size", converted)?;
            }
        }

            if event.has_value("tychon.event.windows_certificate_store_path") {
                event.rename("tychon.event.windows_certificate_store_path", "tychon.windows_certificate_store_path")?;
            }

        if event.has_value("tychon.x509.public_key_size") {
            if let Some(val) = event.get("tychon.x509.public_key_size") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.x509.public_key_size".into(),
                        message,
                    })?;
                event.set("tychon.x509.public_key_size", converted)?;
            }
        }

        if event.has_value("tychon.x509.enhanced_key_usage") {
            if let Some(s) = event.get_string("tychon.x509.enhanced_key_usage") {
                let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                while parts.last().and_then(Value::as_str) == Some("") {
                    parts.pop();
                }
                event.set("tychon.x509.enhanced_key_usage", Value::Array(parts))?;
            }
        }

        if event.has_value("tychon.x509.key_usage") {
            if let Some(s) = event.get_string("tychon.x509.key_usage") {
                let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                while parts.last().and_then(Value::as_str) == Some("") {
                    parts.pop();
                }
                event.set("tychon.x509.key_usage", Value::Array(parts))?;
            }
        }

            if event.has_value("tychon.process.owner") {
                event.rename("tychon.process.owner", "tychon.process.user.name")?;
            }

            if event.has_value("tychon.process.ppid") {
                event.rename("tychon.process.ppid", "tychon.process.parent.pid")?;
            }

        if event.has_value("tychon.process.pid") {
            if let Some(val) = event.get("tychon.process.pid") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.process.pid".into(),
                        message,
                    })?;
                event.set("tychon.process.pid", converted)?;
            }
        }

        if event.has_value("tychon.process.parent.pid") {
            if let Some(val) = event.get("tychon.process.parent.pid") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.process.parent.pid".into(),
                        message,
                    })?;
                event.set("tychon.process.parent.pid", converted)?;
            }
        }

        if event.has_value("tychon.server.port") {
            if let Some(val) = event.get("tychon.server.port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.server.port".into(),
                        message,
                    })?;
                event.set("tychon.server.port", converted)?;
            }
        }

            if event.has_value("tychon.file.code_signature.issuer") {
                event.rename("tychon.file.code_signature.issuer", "tychon.file.x509.issuer.distinguished_name")?;
            }

            // Painless script
            // Source: def fieldset = ctx.tychon.x509;\nif (fieldset != null) {\n  for (String partyName : params.party_names) {\n    def party = fieldset[partyName];\n    if (party != null) {\n      for (String fieldName : params.field_names) {\n        def value = party[fieldName];\n        if (value == \"\") {\n          party.remove(fieldName);\n        } else if (value instanceof String) {\n          party[fieldName] = [value];\n        }\n      }\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(event, cached_painless!(r#"def fieldset = ctx.tychon.x509;\nif (fieldset != null) {\n  for (String partyName : params.party_names) {\n    def party = fieldset[partyName];\n    if (party != null) {\n      for (String fieldName : params.field_names) {\n        def value = party[fieldName];\n        if (value == \"\") {\n          party.remove(fieldName);\n        } else if (value instanceof String) {\n          party[fieldName] = [value];\n        }\n      }\n    }\n  }\n}\n"#), cached_params!("{\"party_names\":[\"issuer\",\"subject\"],\"field_names\":[\"common_name\",\"country\",\"distinguished_name\",\"locality\",\"organizational_unit\",\"organization\",\"state_or_province\"]}"))?;

        event.set("event.category", Value::Array(vec![json!("configuration")]))?;

        let _cond = { event.has_value("tychon.file.hash.md5") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("tychon.file.hash.md5").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("tychon.file.hash.sha1") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("tychon.file.hash.sha1").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("tychon.file.hash.sha256") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("tychon.file.hash.sha256").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("tychon.x509.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("tychon.x509.hash").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("tychon.file.accessed").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.accessed", v)?;
        }

        if let Some(v) = event.get("tychon.file.attributes").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.attributes", v)?;
        }

        if let Some(v) = event.get("tychon.file.code_signature.subject_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.code_signature.subject_name", v)?;
        }

        if let Some(v) = event.get("tychon.file.created").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.created", v)?;
        }

        if let Some(v) = event.get("tychon.file.extension").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.extension", v)?;
        }

        if let Some(v) = event.get("tychon.file.hash.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.hash.md5", v)?;
        }

        if let Some(v) = event.get("tychon.file.hash.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.hash.sha1", v)?;
        }

        if let Some(v) = event.get("tychon.file.hash.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.hash.sha256", v)?;
        }

        if let Some(v) = event.get("tychon.file.mtime").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.mtime", v)?;
        }

        if let Some(v) = event.get("tychon.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.name", v)?;
        }

        if let Some(v) = event.get("tychon.file.path").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.path", v)?;
        }

        if let Some(v) = event.get("tychon.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.size", v)?;
        }

        if let Some(v) = event.get("tychon.file.x509.issuer.distinguished_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("file.x509.issuer.distinguished_name", v)?;
        }

        if let Some(v) = event.get("tychon.process.command_line").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.command_line", v)?;
        }

        if let Some(v) = event.get("tychon.process.executable").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.executable", v)?;
        }

        if let Some(v) = event.get("tychon.process.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.name", v)?;
        }

        if let Some(v) = event.get("tychon.process.parent.pid").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.parent.pid", v)?;
        }

        if let Some(v) = event.get("tychon.process.pid").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.pid", v)?;
        }

        if let Some(v) = event.get("tychon.process.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.user.name", v)?;
        }

        if let Some(v) = event.get("tychon.server.address").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.address", v)?;
        }

        if let Some(v) = event.get("tychon.server.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.ip", v)?;
        }

        if let Some(v) = event.get("tychon.server.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.port", v)?;
        }

        if let Some(v) = event.get("tychon.service.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.name", v)?;
        }

        if let Some(v) = event.get("tychon.service.state").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.state", v)?;
        }

        if let Some(v) = event.get("tychon.url.full").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.full", v)?;
        }

        Ok(TransformResult::Continue)
    }
}
