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
        let _cond = { event.get("tychon.file.attributes").is_some_and(|v| v.is_array()) };
        if _cond {
            // Painless script
            // Source: ctx.tychon.file.attributes.removeIf(e -> !(e instanceof String));
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"ctx.tychon.file.attributes.removeIf(e -> !(e instanceof String));"#))?;
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

            if event.has_value("tychon.process.owner") {
                event.rename("tychon.process.owner", "tychon.process.user.name")?;
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

            if event.has_value("tychon.process.ppid") {
                event.rename("tychon.process.ppid", "tychon.process.parent.pid")?;
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

            if event.has_value("tychon.file.code_signature.issuer") {
                event.rename("tychon.file.code_signature.issuer", "tychon.file.x509.issuer.distinguished_name")?;
            }

            if event.has_value("tychon.tls.server.supported_cipher.name") {
                event.rename("tychon.tls.server.supported_cipher.name", "tychon.tls.server.supported_ciphers")?;
            }

            if event.has_value("tychon.tls.server.supported_cipher.mac") {
                event.rename("tychon.tls.server.supported_cipher.mac", "tychon.tls.server.supported_cipher_mac")?;
            }

        if event.has_value("tychon.tls.client.supported_ciphers") {
            if let Some(s) = event.get_string("tychon.tls.client.supported_ciphers") {
                let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                while parts.last().and_then(Value::as_str) == Some("") {
                    parts.pop();
                }
                event.set("tychon.tls.client.supported_ciphers", Value::Array(parts))?;
            }
        }

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

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("tychon.server.ip") {
            if let Some(val) = event.get("tychon.server.ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.server.ip".into(),
                        message,
                    })?;
                event.set("tychon.server.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_tychon_server_ip")?;
                    if event.has_value("tychon.server.ip") {
                        event.rename("tychon.server.ip", "tychon.server.host")?;
                    }
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("tychon.server.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("tychon.server.ip").map_or_else(String::new, template_to_string)))?;
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

        if let Some(v) = event.get("tychon.server.host").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.domain", v)?;
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
