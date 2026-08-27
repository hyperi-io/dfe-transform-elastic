// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_tls` pipeline.
pub struct PipelineObjectTls;

impl Transform for PipelineObjectTls {
    fn name(&self) -> &str {
        "pipeline_object_tls"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(v) = event.get("ocsf.tls.cipher").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.cipher", v)?;
            }

            if let Some(v) = event.get("ocsf.tls.ja3_hash.value").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.client.ja3", v)?;
            }

            let _cond = { event.has_value("tls.client.ja3") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("tls.client.ja3").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.tls.sni").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.client.server_name", v)?;
            }

            let _cond = { event.has_value("tls.client.server_name") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("tls.client.server_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.tls.client_ciphers").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.tls.client_ciphers", |event| {
                    event.append_unique("tls.client.supported_ciphers", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.tls.sans").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.tls.sans", |event| {
                    event.append_unique("tls.client.x509.alternative_names", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("json.tls.sans").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: boolean dropDash(Object object) {\n  if (object == '-') {\n    // We do not need to handle null or ' ' since they were done in default.\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> dropDash(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> dropDash(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropDash(ctx.ocsf);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"boolean dropDash(Object object) {\n  if (object == '-') {\n    // We do not need to handle null or ' ' since they were done in default.\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> dropDash(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> dropDash(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropDash(ctx.ocsf);"#))?;
            }

            let _cond = { event.get("json.tls.sans").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.tls.sans", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.name") {
                    if let Some(val) = event.get("_ingest._value.name") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.name".into(),
                    message,
                    })?;
                    event.set("_ingest._value.name_ips", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append_unique("related.hosts", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("json.tls.sans").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.tls.sans", |event| {
                    event.append_unique("related.ip", json!(event.get("_ingest._value.name_ips").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("json.tls.sans").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "json.tls.sans", |event| {
                    event.remove("_ingest._value.name_ips");
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.tls.certificate.issuer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.client.x509.issuer.distinguished_name", v)?;
            }

            let _cond = { event.has_value("ocsf.tls.certificate.expiration_time_dt") && event.get_str("ocsf.tls.certificate.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.tls.certificate.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.tls.certificate.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.tls.certificate.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tls_certificate_expiration_time_dt")?;
                        event.remove("ocsf.tls.certificate.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ocsf.tls.extension_list").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.tls.extension_list", |event| {
                    if event.has_value("_ingest._value.type_id") {
                    if let Some(val) = event.get("_ingest._value.type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.type_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.tls.certificate.expiration_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.client.x509.not_after", v)?;
            }

            let _cond = { event.has_value("ocsf.tls.certificate.expiration_time") && event.get_str("ocsf.tls.certificate.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.tls.certificate.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.tls.certificate.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.tls.certificate.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tls_certificate_expiration_time")?;
                        event.remove("ocsf.tls.certificate.expiration_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.tls.certificate.expiration_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.client.x509.not_after", v)?;
            }

            if let Some(v) = event.get("ocsf.tls.certificate.serial_number").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.client.x509.serial_number", v)?;
            }

            if let Some(v) = event.get("ocsf.tls.certificate.subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.client.x509.subject.distinguished_name", v)?;
            }

            if let Some(v) = event.get("ocsf.tls.certificate.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.client.x509.version_number", v)?;
            }

            if let Some(v) = event.get("ocsf.tls.ja3s_hash.value").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.server.ja3s", v)?;
            }

            let _cond = { event.has_value("tls.server.ja3s") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("tls.server.ja3s").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.tls.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.version", v)?;
            }

            let _cond = { event.has_value("ocsf.tls.certificate.created_time_dt") && event.get_str("ocsf.tls.certificate.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.tls.certificate.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.tls.certificate.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.tls.certificate.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tls_certificate_created_time_dt")?;
                        event.remove("ocsf.tls.certificate.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.tls.certificate.created_time") && event.get_str("ocsf.tls.certificate.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.tls.certificate.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.tls.certificate.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.tls.certificate.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tls_certificate_created_time")?;
                        event.remove("ocsf.tls.certificate.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.tls.alert") {
                if let Some(val) = event.get("ocsf.tls.alert") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.tls.alert".into(),
                            message,
                        })?;
                    event.set("ocsf.tls.alert", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_tls_alert_to_long")?;
                        event.remove("ocsf.tls.alert");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.tls.handshake_dur") {
                if let Some(val) = event.get("ocsf.tls.handshake_dur") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.tls.handshake_dur".into(),
                            message,
                        })?;
                    event.set("ocsf.tls.handshake_dur", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_tls_handshake_dur_to_long")?;
                        event.remove("ocsf.tls.handshake_dur");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.tls.certificate.fingerprints") };
            if _cond {
                // Painless script
                // Source: if (ctx.tls == null) {\n  ctx.tls = new HashMap();\n}\nif (ctx.tls.client == null) {\n  ctx.tls.client = new HashMap();\n}\nif (ctx.tls.client.hash == null) {\n  ctx.tls.client.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.tls.certificate.get('fingerprints');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.tls.client.hash = map;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (ctx.tls == null) {\n  ctx.tls = new HashMap();\n}\nif (ctx.tls.client == null) {\n  ctx.tls.client = new HashMap();\n}\nif (ctx.tls.client.hash == null) {\n  ctx.tls.client.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.tls.certificate.get('fingerprints');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.tls.client.hash = map;"#), cached_params!("{\"MD5\":\"md5\",\"SHA-1\":\"sha1\",\"SHA-256\":\"sha256\"}"))?;
            }

            let _cond = { event.get("tls.client.hash").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "tls.client.hash", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.tls.key_length") {
                if let Some(val) = event.get("ocsf.tls.key_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.tls.key_length".into(),
                            message,
                        })?;
                    event.set("ocsf.tls.key_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_tls_key_length_to_long")?;
                        event.remove("ocsf.tls.key_length");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.tls.ja3_hash.algorithm_id") {
                if let Some(val) = event.get("ocsf.tls.ja3_hash.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.tls.ja3_hash.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.tls.ja3_hash.algorithm_id", converted)?;
                }
            }

            if event.has_value("ocsf.tls.ja3s_hash.algorithm_id") {
                if let Some(val) = event.get("ocsf.tls.ja3s_hash.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.tls.ja3s_hash.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.tls.ja3s_hash.algorithm_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.tls.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.tls.certificate.fingerprints", |event| {
                    if event.has_value("_ingest._value.algorithm_id") {
                    if let Some(val) = event.get("_ingest._value.algorithm_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.algorithm_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.algorithm_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
