// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_actor` pipeline.
pub struct PipelineObjectActor;

impl Transform for PipelineObjectActor {
    fn name(&self) -> &str {
        "pipeline_object_actor"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(v) = event.get("ocsf.actor.process.container.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.id", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.container.hash.value") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (params.get(ctx.ocsf.actor.process.container.hash.algorithm) == null) {\n  return;\n}\nif (ctx.container == null) {\n  ctx.container = new HashMap();\n}\nif (ctx.container.image == null) {\n  ctx.container.image = new HashMap();\n}\nif (ctx.container.image.hash == null) {\n  ctx.container.image.hash = new HashMap();\n}\ndef list = new ArrayList();\ndef value = params.get(ctx.ocsf.actor.process.container.hash.algorithm) + ':' + ctx.ocsf.actor.process.container.hash.value;\nlist.add(value);\nctx.container.image.hash.all = list;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (params.get(ctx.ocsf.actor.process.container.hash.algorithm) == null) {\n  return;\n}\nif (ctx.container == null) {\n  ctx.container = new HashMap();\n}\nif (ctx.container.image == null) {\n  ctx.container.image = new HashMap();\n}\nif (ctx.container.image.hash == null) {\n  ctx.container.image.hash = new HashMap();\n}\ndef list = new ArrayList();\ndef value = params.get(ctx.ocsf.actor.process.container.hash.algorithm) + ':' + ctx.ocsf.actor.process.container.hash.value;\nlist.add(value);\nctx.container.image.hash.all = list;"#), cached_params!("{\"MD5\":\"md5\",\"SHA-1\":\"sha1\",\"SHA-256\":\"sha256\",\"SHA-512\":\"sha512\",\"CTPH\":\"ssdeep\",\"TLSH\":\"tlsh\"}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_container_image_hash_all")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.container.hash.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.actor.process.container.hash.value").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.process.container.image.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.image.name", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.container.image.tag") };
            if _cond {
                event.append_unique("container.image.tag", json!(event.get("ocsf.actor.process.container.image.tag").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.process.container.image.labels").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.labels", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.container.orchestrator").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("orchestrator.type", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.container.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.name", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.container.runtime").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.runtime", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.accessed_time_dt") && event.get_str("ocsf.actor.process.file.accessed_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.accessed_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.accessed_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.accessed_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_accessed_time_dt")?;
                        event.remove("ocsf.actor.process.file.accessed_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.file.accessed_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.accessed", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.accessed_time") && event.get_str("ocsf.actor.process.file.accessed_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.accessed_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.accessed_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.accessed_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_accessed_time")?;
                        event.remove("ocsf.actor.process.file.accessed_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.file.accessed_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.accessed", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.created_time_dt") && event.get_str("ocsf.actor.process.file.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_created_time_dt")?;
                        event.remove("ocsf.actor.process.file.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.file.created_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.created", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.created_time") && event.get_str("ocsf.actor.process.file.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_created_time")?;
                        event.remove("ocsf.actor.process.file.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.file.created_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.created", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.file.parent_folder").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.directory", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.hashes") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.file == null) {\n  ctx.file = new HashMap();\n}\nif (ctx.file.hash == null) {\n  ctx.file.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.actor.process.file.get('hashes');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.file.hash = map;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"if (ctx.file == null) {\n  ctx.file = new HashMap();\n}\nif (ctx.file.hash == null) {\n  ctx.file.hash = new HashMap();\n}\nMap map = new HashMap();\ndef hashes = ctx.ocsf.actor.process.file.get('hashes');\nfor (def hash: hashes) {\n  def hashAlgorithm = params.get(hash.get('algorithm'));\n  if (hashAlgorithm == null) {\n    continue;\n  }\n  if (map.containsKey(hashAlgorithm)) {\n    map[hashAlgorithm].add(hash.get('value'));\n  } else {\n    Set set = new HashSet();\n    set.add(hash.get('value'));\n    map.put(hashAlgorithm, set);\n  }\n}\nctx.file.hash = map;"#), cached_params!("{\"MD5\":\"md5\",\"SHA-1\":\"sha1\",\"SHA-256\":\"sha256\",\"SHA-512\":\"sha512\",\"CTPH\":\"ssdeep\",\"TLSH\":\"tlsh\"}"))?;
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

            let _cond = { event.get("ocsf.actor.process.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.file.hashes", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.actor.process.file.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.inode", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.file.mime_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mime_type", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.modified_time_dt") && event.get_str("ocsf.actor.process.file.modified_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.modified_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.modified_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.modified_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_modified_time_dt")?;
                        event.remove("ocsf.actor.process.file.modified_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.file.modified_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mtime", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.modified_time") && event.get_str("ocsf.actor.process.file.modified_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.modified_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.modified_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.modified_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_modified_time")?;
                        event.remove("ocsf.actor.process.file.modified_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.file.modified_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mtime", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.file.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.name", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.file.owner.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.owner", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.owner.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.owner.name").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.process.file.path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.path", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.file.size") {
                if let Some(val) = event.get("ocsf.actor.process.file.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.size".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_file_size_to_long")?;
                        event.remove("ocsf.actor.process.file.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.actor.process.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.file.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.type", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.file.owner.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.uid", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.owner.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.owner.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.process.file.signature.certificate.issuer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.issuer.distinguished_name", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.signature.certificate.expiration_time_dt") && event.get_str("ocsf.actor.process.file.signature.certificate.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.signature.certificate.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.signature.certificate.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.signature.certificate.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_signature_certificate_expiration_time_dt")?;
                        event.remove("ocsf.actor.process.file.signature.certificate.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.file.signature.certificate.expiration_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.not_after", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.signature.certificate.expiration_time") && event.get_str("ocsf.actor.process.file.signature.certificate.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.signature.certificate.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.signature.certificate.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.signature.certificate.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_signature_certificate_expiration_time")?;
                        event.remove("ocsf.actor.process.file.signature.certificate.expiration_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.file.signature.certificate.expiration_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.not_after", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.file.signature.certificate.serial_number").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.serial_number", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.file.signature.certificate.subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.subject.distinguished_name", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.file.signature.certificate.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.version_number", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.cmd_line").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.command_line", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.terminated_time_dt") && event.get_str("ocsf.actor.process.terminated_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.terminated_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.terminated_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.terminated_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_terminated_time_dt")?;
                        event.remove("ocsf.actor.process.terminated_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.terminated_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.end", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.terminated_time") && event.get_str("ocsf.actor.process.terminated_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.terminated_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.terminated_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.terminated_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_terminated_time")?;
                        event.remove("ocsf.actor.process.terminated_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.terminated_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.end", v)?;
            }

            if event.has_value("ocsf.actor.process.egid") {
                if let Some(val) = event.get("ocsf.actor.process.egid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.egid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.egid", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.egid") };
            if _cond {
                event.append_unique("process.group.id", json!(event.get("ocsf.actor.process.egid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.group.uid") };
            if _cond {
                event.append_unique("process.group.id", json!(event.get("ocsf.actor.process.group.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.process.group.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.group.name", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.name", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.pid") {
                if let Some(val) = event.get("ocsf.actor.process.pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.pid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_pid_to_long")?;
                        event.remove("ocsf.actor.process.pid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.actor.process.pid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pid", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.created_time_dt") && event.get_str("ocsf.actor.process.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_created_time_dt")?;
                        event.remove("ocsf.actor.process.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.created_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.start", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.created_time") && event.get_str("ocsf.actor.process.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_created_time")?;
                        event.remove("ocsf.actor.process.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.created_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.start", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.tid") {
                if let Some(val) = event.get("ocsf.actor.process.tid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.tid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.tid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_tid_to_long")?;
                        event.remove("ocsf.actor.process.tid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.actor.process.tid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.thread.id", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.entity_id", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.user.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.user.domain", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.user.email_addr").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.user.email", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.user.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.user.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.process.user.full_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.user.full_name", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.user.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.user.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.actor.process.user.groups").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.user.groups", |event| {
                    event.append_unique("process.user.group.id", json!(event.get("_ingest._value.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.actor.process.user.groups").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.user.groups", |event| {
                    event.append_unique("process.user.group.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.actor.process.euid") {
                if let Some(val) = event.get("ocsf.actor.process.euid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.euid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.euid", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.euid") };
            if _cond {
                event.append_unique("process.user.id", json!(event.get("ocsf.actor.process.euid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.euid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.euid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.user.uid") };
            if _cond {
                event.append_unique("process.user.id", json!(event.get("ocsf.actor.process.user.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.user.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.user.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.process.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.user.name", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.user.name").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.user.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.user.email_addr").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.user.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.user.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.user.full_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.full_name", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.user.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.user.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.actor.user.groups").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.user.groups", |event| {
                    event.append_unique("user.group.id", json!(event.get("_ingest._value.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.actor.user.groups").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.user.groups", |event| {
                    event.append_unique("user.group.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.actor.user.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.user.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.user.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.user.name").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.actor.process.container.hash.algorithm_id") {
                if let Some(val) = event.get("ocsf.actor.process.container.hash.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.container.hash.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.container.hash.algorithm_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.auid") {
                if let Some(val) = event.get("ocsf.actor.process.auid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.auid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.auid", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.container.size") {
                if let Some(val) = event.get("ocsf.actor.process.container.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.container.size".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.container.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_container_size_to_long")?;
                        event.remove("ocsf.actor.process.container.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.actor.process.file.accessor.account.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.accessor.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.accessor.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.accessor.account.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.file.accessor.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.accessor.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.accessor.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.accessor.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.accessor.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.accessor.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.accessor.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.accessor.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.accessor.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.accessor.uid").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.actor.process.file.accessor.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.accessor.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.accessor.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.accessor.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.file.attributes") {
                if let Some(val) = event.get("ocsf.actor.process.file.attributes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.attributes".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.attributes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_file_attributes_to_long")?;
                        event.remove("ocsf.actor.process.file.attributes");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.actor.process.file.confidentiality_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.confidentiality_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.confidentiality_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.confidentiality_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.file.creator.account.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.creator.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.creator.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.creator.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.file.creator.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.creator.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.creator.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.creator.type_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.actor.process.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.file.signature.certificate.fingerprints", |event| {
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

            let _cond = { event.get("ocsf.actor.process.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.file.signature.certificate.fingerprints", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.actor.process.file.signature.algorithm_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.signature.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.signature.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.signature.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.file.signature.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.actor.process.file.signature.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.signature.certificate.created_time_dt") && event.get_str("ocsf.actor.process.file.signature.certificate.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.signature.certificate.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.signature.certificate.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.signature.certificate.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_signature_certificate_created_time_dt")?;
                        event.remove("ocsf.actor.process.file.signature.certificate.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.file.signature.certificate.created_time") && event.get_str("ocsf.actor.process.file.signature.certificate.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.signature.certificate.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.signature.certificate.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.signature.certificate.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_signature_certificate_created_time")?;
                        event.remove("ocsf.actor.process.file.signature.certificate.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.file.signature.created_time_dt") && event.get_str("ocsf.actor.process.file.signature.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.signature.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.signature.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.signature.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_signature_created_time_dt")?;
                        event.remove("ocsf.actor.process.file.signature.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.file.signature.created_time") && event.get_str("ocsf.actor.process.file.signature.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.file.signature.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.file.signature.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.file.signature.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_file_signature_created_time")?;
                        event.remove("ocsf.actor.process.file.signature.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.actor.process.file.signature.digest.algorithm_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.signature.digest.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.signature.digest.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.signature.digest.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.file.signature.digest.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.actor.process.file.signature.digest.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.actor.process.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.file.hashes", |event| {
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

            if event.has_value("ocsf.actor.process.file.modifier.account.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.modifier.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.modifier.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.modifier.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.file.modifier.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.modifier.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.modifier.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.modifier.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.file.owner.account.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.owner.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.owner.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.owner.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.file.owner.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.owner.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.owner.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.owner.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.file.is_system") {
                if let Some(val) = event.get("ocsf.actor.process.file.is_system") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.is_system".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.is_system", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_file_is_system_to_boolean")?;
                        event.remove("ocsf.actor.process.file.is_system");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.actor.process.file.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.file.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.file.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.file.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.integrity_id") {
                if let Some(val) = event.get("ocsf.actor.process.integrity_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.integrity_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.integrity_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.namespace_pid") {
                if let Some(val) = event.get("ocsf.actor.process.namespace_pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.namespace_pid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.namespace_pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_namespace_pid_to_long")?;
                        event.remove("ocsf.actor.process.namespace_pid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def process = ctx.ocsf.actor.process.parent_process;\ndef count = 0;\nwhile (true) {\n  if (process != null && process.parent_process != null) {\n    count += 1;\n    process = process.parent_process;\n  } else {\n    break;\n  }\n}\nif (count >= 15) {\n  ctx.ocsf.actor.process.parent_process.put(\"parent_process_keyword\", ctx.ocsf.actor.process.parent_process.parent_process.toString());\n  ctx.ocsf.actor.process.parent_process.remove(\"parent_process\");\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def process = ctx.ocsf.actor.process.parent_process;\ndef count = 0;\nwhile (true) {\n  if (process != null && process.parent_process != null) {\n    count += 1;\n    process = process.parent_process;\n  } else {\n    break;\n  }\n}\nif (count >= 15) {\n  ctx.ocsf.actor.process.parent_process.put(\"parent_process_keyword\", ctx.ocsf.actor.process.parent_process.parent_process.toString());\n  ctx.ocsf.actor.process.parent_process.remove(\"parent_process\");\n}"#))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_actor_process_parent_process_stringify")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.container.hash.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.actor.process.parent_process.container.hash.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.accessed_time_dt") && event.get_str("ocsf.actor.process.parent_process.file.accessed_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.accessed_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.accessed_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.accessed_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_accessed_time_dt")?;
                        event.remove("ocsf.actor.process.parent_process.file.accessed_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.accessed_time") && event.get_str("ocsf.actor.process.parent_process.file.accessed_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.accessed_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.accessed_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.accessed_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_accessed_time")?;
                        event.remove("ocsf.actor.process.parent_process.file.accessed_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.created_time_dt") && event.get_str("ocsf.actor.process.parent_process.file.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_created_time_dt")?;
                        event.remove("ocsf.actor.process.parent_process.file.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.created_time") && event.get_str("ocsf.actor.process.parent_process.file.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_created_time")?;
                        event.remove("ocsf.actor.process.parent_process.file.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("ocsf.actor.process.parent_process.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.parent_process.file.hashes", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.modified_time_dt") && event.get_str("ocsf.actor.process.parent_process.file.modified_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.modified_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.modified_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.modified_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_modified_time_dt")?;
                        event.remove("ocsf.actor.process.parent_process.file.modified_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.modified_time") && event.get_str("ocsf.actor.process.parent_process.file.modified_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.modified_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.modified_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.modified_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_modified_time")?;
                        event.remove("ocsf.actor.process.parent_process.file.modified_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.owner.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.owner.name").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.parent_process.file.size") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.size".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_parent_process_file_size_to_long")?;
                        event.remove("ocsf.actor.process.parent_process.file.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.owner.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.owner.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.signature.certificate.expiration_time_dt") && event.get_str("ocsf.actor.process.parent_process.file.signature.certificate.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.signature.certificate.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.signature.certificate.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.signature.certificate.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_signature_certificate_expiration_time_dt")?;
                        event.remove("ocsf.actor.process.parent_process.file.signature.certificate.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.signature.certificate.expiration_time") && event.get_str("ocsf.actor.process.parent_process.file.signature.certificate.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.signature.certificate.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.signature.certificate.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.signature.certificate.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_signature_certificate_expiration_time")?;
                        event.remove("ocsf.actor.process.parent_process.file.signature.certificate.expiration_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.cmd_line").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.command_line", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.terminated_time_dt") && event.get_str("ocsf.actor.process.parent_process.terminated_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.terminated_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.terminated_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.terminated_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_terminated_time_dt")?;
                        event.remove("ocsf.actor.process.parent_process.terminated_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.terminated_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.end", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.terminated_time") && event.get_str("ocsf.actor.process.parent_process.terminated_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.terminated_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.terminated_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.terminated_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_terminated_time")?;
                        event.remove("ocsf.actor.process.parent_process.terminated_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.terminated_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.end", v)?;
            }

            if event.has_value("ocsf.actor.process.parent_process.egid") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.egid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.egid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.egid", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.egid") };
            if _cond {
                event.append_unique("process.parent.group.id", json!(event.get("ocsf.actor.process.parent_process.egid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.group.uid") };
            if _cond {
                event.append_unique("process.parent.group.id", json!(event.get("ocsf.actor.process.parent_process.group.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.group.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.group.name", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.name", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.parent_process.pid") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.pid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_parent_process_pid_to_long")?;
                        event.remove("ocsf.actor.process.parent_process.pid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.pid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pid", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.created_time_dt") && event.get_str("ocsf.actor.process.parent_process.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_created_time_dt")?;
                        event.remove("ocsf.actor.process.parent_process.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.created_time_dt").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.start", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.created_time") && event.get_str("ocsf.actor.process.parent_process.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_created_time")?;
                        event.remove("ocsf.actor.process.parent_process.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.created_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.start", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.parent_process.tid") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.tid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.tid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.tid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_parent_process_tid_to_long")?;
                        event.remove("ocsf.actor.process.parent_process.tid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.tid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.thread.id", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.entity_id", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.user.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.user.domain", v)?;
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.user.email_addr").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.user.email", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.user.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.user.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.user.full_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.user.full_name", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.user.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.user.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.actor.process.parent_process.user.groups").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.parent_process.user.groups", |event| {
                    event.append_unique("process.parent.user.group.id", json!(event.get("_ingest._value.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.actor.process.parent_process.user.groups").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.parent_process.user.groups", |event| {
                    event.append_unique("process.parent.user.group.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.actor.process.parent_process.euid") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.euid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.euid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.euid", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.euid") };
            if _cond {
                event.append_unique("process.parent.user.id", json!(event.get("ocsf.actor.process.parent_process.euid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.euid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.euid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.user.uid") };
            if _cond {
                event.append_unique("process.parent.user.id", json!(event.get("ocsf.actor.process.parent_process.user.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.user.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.user.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.actor.process.parent_process.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.user.name", v)?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.user.name").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.actor.process.parent_process.container.hash.algorithm_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.container.hash.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.container.hash.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.container.hash.algorithm_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.auid") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.auid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.auid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.auid", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.parent_process.container.size") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.container.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.container.size".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.container.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_parent_process_container_size_to_long")?;
                        event.remove("ocsf.actor.process.parent_process.container.size");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.file.accessor.account.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.accessor.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.accessor.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.accessor.account.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.accessor.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.accessor.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.accessor.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.accessor.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.accessor.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.accessor.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.accessor.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.accessor.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.accessor.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.accessor.uid").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.actor.process.parent_process.file.accessor.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.accessor.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.accessor.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.accessor.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.parent_process.file.attributes") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.attributes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.attributes".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.attributes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_parent_process_file_attributes_to_long")?;
                        event.remove("ocsf.actor.process.parent_process.file.attributes");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.file.confidentiality_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.confidentiality_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.confidentiality_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.confidentiality_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.file.creator.account.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.creator.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.creator.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.creator.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.file.creator.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.creator.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.creator.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.creator.type_id", converted)?;
                }
            }

            let _cond = { event.get("ocsf.actor.process.parent_process.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.parent_process.file.signature.certificate.fingerprints", |event| {
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

            let _cond = { event.get("ocsf.actor.process.parent_process.file.signature.certificate.fingerprints").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.parent_process.file.signature.certificate.fingerprints", |event| {
                    event.append_unique("related.hash", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if event.has_value("ocsf.actor.process.parent_process.file.signature.algorithm_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.signature.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.signature.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.signature.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.signature.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.actor.process.parent_process.file.signature.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.signature.certificate.created_time_dt") && event.get_str("ocsf.actor.process.parent_process.file.signature.certificate.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.signature.certificate.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.signature.certificate.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.signature.certificate.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_signature_certificate_created_time_dt")?;
                        event.remove("ocsf.actor.process.parent_process.file.signature.certificate.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.signature.certificate.created_time") && event.get_str("ocsf.actor.process.parent_process.file.signature.certificate.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.signature.certificate.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.signature.certificate.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.signature.certificate.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_signature_certificate_created_time")?;
                        event.remove("ocsf.actor.process.parent_process.file.signature.certificate.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.signature.created_time_dt") && event.get_str("ocsf.actor.process.parent_process.file.signature.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.signature.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.signature.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.signature.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_signature_created_time_dt")?;
                        event.remove("ocsf.actor.process.parent_process.file.signature.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.signature.created_time") && event.get_str("ocsf.actor.process.parent_process.file.signature.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.file.signature.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.file.signature.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.file.signature.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_file_signature_created_time")?;
                        event.remove("ocsf.actor.process.parent_process.file.signature.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("ocsf.actor.process.parent_process.file.signature.digest.algorithm_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.signature.digest.algorithm_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.signature.digest.algorithm_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.signature.digest.algorithm_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.signature.digest.value") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("ocsf.actor.process.parent_process.file.signature.digest.value").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.actor.process.parent_process.file.hashes").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.actor.process.parent_process.file.hashes", |event| {
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

            if event.has_value("ocsf.actor.process.parent_process.file.modifier.account.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.modifier.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.modifier.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.modifier.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.file.modifier.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.modifier.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.modifier.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.modifier.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.file.owner.account.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.owner.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.owner.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.owner.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.file.owner.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.owner.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.owner.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.owner.type_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.parent_process.file.is_system") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.is_system") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.is_system".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.is_system", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_parent_process_file_is_system_to_boolean")?;
                        event.remove("ocsf.actor.process.parent_process.file.is_system");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.file.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.file.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.file.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.file.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.integrity_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.integrity_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.integrity_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.integrity_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.actor.process.parent_process.namespace_pid") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.namespace_pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.namespace_pid".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.namespace_pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_parent_process_namespace_pid_to_long")?;
                        event.remove("ocsf.actor.process.parent_process.namespace_pid");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.session.created_time_dt") && event.get_str("ocsf.actor.process.parent_process.session.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.session.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.session.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.session.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_session_created_time_dt")?;
                        event.remove("ocsf.actor.process.parent_process.session.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.session.created_time") && event.get_str("ocsf.actor.process.parent_process.session.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.session.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.session.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.session.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_session_created_time")?;
                        event.remove("ocsf.actor.process.parent_process.session.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.session.expiration_time_dt") && event.get_str("ocsf.actor.process.parent_process.session.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.session.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.session.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.session.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_session_expiration_time_dt")?;
                        event.remove("ocsf.actor.process.parent_process.session.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.session.expiration_time") && event.get_str("ocsf.actor.process.parent_process.session.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.parent_process.session.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.parent_process.session.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.parent_process.session.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_parent_process_session_expiration_time")?;
                        event.remove("ocsf.actor.process.parent_process.session.expiration_time");
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
            if event.has_value("ocsf.actor.process.parent_process.session.mfa") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.session.mfa") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.session.mfa".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.session.mfa", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_parent_process_session_mfa_to_boolean")?;
                        event.remove("ocsf.actor.process.parent_process.session.mfa");
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
            if event.has_value("ocsf.actor.process.parent_process.session.is_remote") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.session.is_remote") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.session.is_remote".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.session.is_remote", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_parent_process_session_is_remote_to_boolean")?;
                        event.remove("ocsf.actor.process.parent_process.session.is_remote");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.user.account.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.user.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.user.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.user.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.parent_process.user.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.parent_process.user.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.parent_process.user.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.parent_process.user.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.creator.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.creator.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.creator.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.creator.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.creator.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.creator.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.creator.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.creator.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.creator.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.creator.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.modifier.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.modifier.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.modifier.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.modifier.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.modifier.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.modifier.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.modifier.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.modifier.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.modifier.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.modifier.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.owner.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.owner.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.owner.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.owner.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.file.owner.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.file.owner.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.parent_process.user.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.parent_process.user.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.session.created_time_dt") && event.get_str("ocsf.actor.process.session.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.session.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.session.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.session.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_session_created_time_dt")?;
                        event.remove("ocsf.actor.process.session.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.session.created_time") && event.get_str("ocsf.actor.process.session.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.session.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.session.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.session.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_session_created_time")?;
                        event.remove("ocsf.actor.process.session.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.session.expiration_time_dt") && event.get_str("ocsf.actor.process.session.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.session.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.session.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.session.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_session_expiration_time_dt")?;
                        event.remove("ocsf.actor.process.session.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.process.session.expiration_time") && event.get_str("ocsf.actor.process.session.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.process.session.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.process.session.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.process.session.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_process_session_expiration_time")?;
                        event.remove("ocsf.actor.process.session.expiration_time");
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
            if event.has_value("ocsf.actor.process.session.mfa") {
                if let Some(val) = event.get("ocsf.actor.process.session.mfa") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.session.mfa".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.session.mfa", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_session_mfa_to_boolean")?;
                        event.remove("ocsf.actor.process.session.mfa");
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
            if event.has_value("ocsf.actor.process.session.is_remote") {
                if let Some(val) = event.get("ocsf.actor.process.session.is_remote") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.session.is_remote".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.session.is_remote", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_process_session_is_remote_to_boolean")?;
                        event.remove("ocsf.actor.process.session.is_remote");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.actor.process.user.account.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.user.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.user.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.user.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.process.user.type_id") {
                if let Some(val) = event.get("ocsf.actor.process.user.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.process.user.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.process.user.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.session.created_time_dt") && event.get_str("ocsf.actor.session.created_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.session.created_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.session.created_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.session.created_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_session_created_time_dt")?;
                        event.remove("ocsf.actor.session.created_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.session.created_time") && event.get_str("ocsf.actor.session.created_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.session.created_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.session.created_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.session.created_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_session_created_time")?;
                        event.remove("ocsf.actor.session.created_time");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.session.expiration_time_dt") && event.get_str("ocsf.actor.session.expiration_time_dt") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.session.expiration_time_dt") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.session.expiration_time_dt", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.session.expiration_time_dt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_session_expiration_time_dt")?;
                        event.remove("ocsf.actor.session.expiration_time_dt");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("ocsf.actor.session.expiration_time") && event.get_str("ocsf.actor.session.expiration_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("ocsf.actor.session.expiration_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("ocsf.actor.session.expiration_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "ocsf.actor.session.expiration_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_actor_session_expiration_time")?;
                        event.remove("ocsf.actor.session.expiration_time");
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
            if event.has_value("ocsf.actor.session.mfa") {
                if let Some(val) = event.get("ocsf.actor.session.mfa") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.session.mfa".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.session.mfa", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_session_mfa_to_boolean")?;
                        event.remove("ocsf.actor.session.mfa");
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
            if event.has_value("ocsf.actor.session.is_remote") {
                if let Some(val) = event.get("ocsf.actor.session.is_remote") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.session.is_remote".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.session.is_remote", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_actor_session_is_remote_to_boolean")?;
                        event.remove("ocsf.actor.session.is_remote");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("ocsf.actor.user.account.type_id") {
                if let Some(val) = event.get("ocsf.actor.user.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.user.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.user.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.actor.user.type_id") {
                if let Some(val) = event.get("ocsf.actor.user.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.actor.user.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.actor.user.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.actor.process.file.creator.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.creator.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.creator.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.creator.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.creator.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.creator.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.creator.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.creator.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.creator.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.creator.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.modifier.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.modifier.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.modifier.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.modifier.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.modifier.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.modifier.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.modifier.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.modifier.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.modifier.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.modifier.uid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.owner.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.owner.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.owner.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.owner.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.file.owner.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.file.owner.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.process.user.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.process.user.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.actor.user.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.actor.user.uid_alt").map_or_else(String::new, template_to_string)))?;
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
