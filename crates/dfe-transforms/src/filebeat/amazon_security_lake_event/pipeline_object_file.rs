// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_file` pipeline.
pub struct PipelineObjectFile;

impl Transform for PipelineObjectFile {
    fn name(&self) -> &str {
        "pipeline_object_file"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.remove("file.accessed");
                event.remove("file.created");
                event.remove("file.x509.serial_number");
                event.remove("file.x509.not_after");
                event.remove("file.x509.issuer.distinguished_name");
                event.remove("file.x509.subject.distinguished_name");
                event.remove("file.x509.version_number");
                event.remove("file.hash.*");
                event.remove("file.mime_type");
                event.remove("file.mtime");
                event.remove("file.name");
                event.remove("file.owner");
                event.remove("file.uid");
                event.remove("file.directory");
                event.remove("file.path");
                event.remove("file.size");
                event.remove("file.type");
                event.remove("file.inode");

            let _cond = { event.has_value("ocsf.file.accessed_time_dt") && event.get_str("ocsf.file.accessed_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.accessed_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file.accessed_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.accessed_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_accessed_time_dt")?;
                        event.remove("ocsf.file.accessed_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.file.accessed_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.accessed", v)?;
            }

            let _cond = { event.has_value("ocsf.file.accessed_time") && event.get_str("ocsf.file.accessed_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.accessed_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file.accessed_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.accessed_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_accessed_time")?;
                        event.remove("ocsf.file.accessed_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.file.accessed_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.accessed", v)?;
            }

            let _cond = { event.has_value("ocsf.file.created_time_dt") && event.get_str("ocsf.file.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_created_time_dt")?;
                        event.remove("ocsf.file.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.file.created_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.created", v)?;
            }

            let _cond = { event.has_value("ocsf.file.created_time") && event.get_str("ocsf.file.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_created_time")?;
                        event.remove("ocsf.file.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.file.created_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.created", v)?;
            }

            if let Some(v) = event.get("ocsf.file.parent_folder").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.directory", v)?;
            }

            let _cond = { event.has_value("ocsf.file.hashes") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.file == null) {\n  ctx.file = new HashMap();\n}\nif (ctx.file.hash == null) {\n  ctx.file.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.file.get('hashes');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.file.hash = map;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (ctx.file == null) {\n  ctx.file = new HashMap();\n}\nif (ctx.file.hash == null) {\n  ctx.file.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.file.get('hashes');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.file.hash = map;"#), cached_params!("{\"MD5\":\"md5\",\"SHA-1\":\"sha1\",\"SHA-256\":\"sha256\",\"SHA-512\":\"sha512\",\"CTPH\":\"ssdeep\",\"TLSH\":\"tlsh\"}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_file_hash_*")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ocsf.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.file.hashes", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.file.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.inode", v)?;
            }

            if let Some(v) = event.get("ocsf.file.mime_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mime_type", v)?;
            }

            let _cond = { event.has_value("ocsf.file.modified_time_dt") && event.get_str("ocsf.file.modified_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.modified_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file.modified_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.modified_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_modified_time_dt")?;
                        event.remove("ocsf.file.modified_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.file.modified_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mtime", v)?;
            }

            let _cond = { event.has_value("ocsf.file.modified_time") && event.get_str("ocsf.file.modified_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.modified_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file.modified_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.modified_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_modified_time")?;
                        event.remove("ocsf.file.modified_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.file.modified_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mtime", v)?;
            }

            if let Some(v) = event.get("ocsf.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.name", v)?;
            }

            if let Some(v) = event.get("ocsf.file.owner.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.owner", v)?;
            }

            let _cond = { event.has_value("ocsf.file.owner.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.owner.name").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.file.path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.path", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.file.size") {
                if let Some(val) = event.get("ocsf.file.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.size".into(),
                            message,
                        })?;
                    event.set("ocsf.file.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_size_to_long")?;
                        event.remove("ocsf.file.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }

            if let Some(v) = event.get("ocsf.file.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.type", v)?;
            }

            if let Some(v) = event.get("ocsf.file.owner.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.uid", v)?;
            }

            let _cond = { event.has_value("ocsf.file.owner.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.owner.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.file.signature.certificate.issuer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.issuer.distinguished_name", v)?;
            }

            let _cond = { event.has_value("ocsf.file.signature.certificate.expiration_time_dt") && event.get_str("ocsf.file.signature.certificate.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.signature.certificate.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file.signature.certificate.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.signature.certificate.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_certificate_expiration_time_dt")?;
                        event.remove("ocsf.file.signature.certificate.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.file.signature.certificate.expiration_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.not_after", v)?;
            }

            let _cond = { event.has_value("ocsf.file.signature.certificate.expiration_time") && event.get_str("ocsf.file.signature.certificate.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.signature.certificate.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file.signature.certificate.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.signature.certificate.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_certificate_expiration_time")?;
                        event.remove("ocsf.file.signature.certificate.expiration_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.file.signature.certificate.expiration_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.not_after", v)?;
            }

            if let Some(v) = event.get("ocsf.file.signature.certificate.serial_number").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.serial_number", v)?;
            }

            if let Some(v) = event.get("ocsf.file.signature.certificate.subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.subject.distinguished_name", v)?;
            }

            if let Some(v) = event.get("ocsf.file.signature.certificate.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.version_number", v)?;
            }

            if event.has_value("ocsf.file.accessor.account.type_id") {
                if let Some(val) = event.get("ocsf.file.accessor.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.accessor.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.accessor.account.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.file.accessor.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.accessor.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.accessor.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.accessor.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.accessor.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.accessor.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.accessor.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.accessor.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.accessor.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.accessor.uid").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.file.accessor.type_id") {
                if let Some(val) = event.get("ocsf.file.accessor.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.accessor.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.accessor.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.file.attributes") {
                if let Some(val) = event.get("ocsf.file.attributes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.attributes".into(),
                            message,
                        })?;
                    event.set("ocsf.file.attributes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_attributes_to_long")?;
                        event.remove("ocsf.file.attributes");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.file.confidentiality_id") {
                if let Some(val) = event.get("ocsf.file.confidentiality_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.confidentiality_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.confidentiality_id", converted)?;
                }
            }

            if event.has_value("ocsf.file.creator.account.type_id") {
                if let Some(val) = event.get("ocsf.file.creator.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.creator.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.creator.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.file.creator.type_id") {
                if let Some(val) = event.get("ocsf.file.creator.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.creator.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.creator.type_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.file.signature.certificate.fingerprints", |event| {
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

            let _cond = { event.get("ocsf.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.file.signature.certificate.fingerprints", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.file.signature.algorithm_id") {
                if let Some(val) = event.get("ocsf.file.signature.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.signature.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.signature.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.file.signature.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.file.signature.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.signature.certificate.created_time_dt") && event.get_str("ocsf.file.signature.certificate.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.signature.certificate.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file.signature.certificate.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.signature.certificate.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_certificate_created_time_dt")?;
                        event.remove("ocsf.file.signature.certificate.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file.signature.certificate.created_time") && event.get_str("ocsf.file.signature.certificate.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.signature.certificate.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file.signature.certificate.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.signature.certificate.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_certificate_created_time")?;
                        event.remove("ocsf.file.signature.certificate.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file.signature.created_time_dt") && event.get_str("ocsf.file.signature.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.signature.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.file.signature.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.signature.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_created_time_dt")?;
                        event.remove("ocsf.file.signature.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.file.signature.created_time") && event.get_str("ocsf.file.signature.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.file.signature.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.file.signature.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.file.signature.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_signature_created_time")?;
                        event.remove("ocsf.file.signature.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.file.signature.digest.algorithm_id") {
                if let Some(val) = event.get("ocsf.file.signature.digest.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.signature.digest.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.signature.digest.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.file.signature.digest.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.file.signature.digest.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.file.hashes", |event| {
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

            if event.has_value("ocsf.file.modifier.account.type_id") {
                if let Some(val) = event.get("ocsf.file.modifier.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.modifier.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.modifier.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.file.modifier.type_id") {
                if let Some(val) = event.get("ocsf.file.modifier.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.modifier.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.modifier.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.file.owner.account.type_id") {
                if let Some(val) = event.get("ocsf.file.owner.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.owner.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.owner.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.file.owner.type_id") {
                if let Some(val) = event.get("ocsf.file.owner.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.owner.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.owner.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.file.is_system") {
                if let Some(val) = event.get("ocsf.file.is_system") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.is_system".into(),
                            message,
                        })?;
                    event.set("ocsf.file.is_system", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_is_system_to_boolean")?;
                        event.remove("ocsf.file.is_system");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.file.type_id") {
                if let Some(val) = event.get("ocsf.file.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.file.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.file.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.file.creator.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.creator.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.creator.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.creator.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.creator.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.creator.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.creator.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.creator.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.creator.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.creator.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.modifier.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.modifier.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.modifier.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.modifier.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.modifier.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.modifier.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.modifier.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.modifier.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.modifier.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.modifier.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.owner.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.owner.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.owner.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.owner.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.file.owner.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.file.owner.full_name").map_or_else(String::new, template_to_string)))?;
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
