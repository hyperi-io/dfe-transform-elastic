// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_file` pipeline.
pub struct PipelineFile;

impl Transform for PipelineFile {
    fn name(&self) -> &str {
        "pipeline_file"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.category", Value::Array(vec![json!("file")]))?;

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.type") && event.get_str("sentinel_one_cloud_funnel.event.type").is_some_and(|s| s.to_lowercase().contains("creat")) };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("creation")]))?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.type") && event.get_str("sentinel_one_cloud_funnel.event.type").is_some_and(|s| s.to_lowercase().contains("delet")) };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("deletion")]))?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.type") && (event.get_str("sentinel_one_cloud_funnel.event.type").is_some_and(|s| s.to_lowercase().contains("change")) || event.get_str("sentinel_one_cloud_funnel.event.type").is_some_and(|s| s.to_lowercase().contains("modif")) || event.get_str("sentinel_one_cloud_funnel.event.type").is_some_and(|s| s.to_lowercase().contains("rename"))) };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("change")]))?;
            }

            if !event.has("event.type") {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.type") && event.get_str("sentinel_one_cloud_funnel.event.type").is_some_and(|s| s.to_lowercase().contains("creat")) };
            if _cond {
            event.set("event.action", Value::Array(vec![json!("creation")]))?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.type") && event.get_str("sentinel_one_cloud_funnel.event.type").is_some_and(|s| s.to_lowercase().contains("delet")) };
            if _cond {
            event.set("event.action", Value::Array(vec![json!("deletion")]))?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.type") && (event.get_str("sentinel_one_cloud_funnel.event.type").is_some_and(|s| s.to_lowercase().contains("change")) || event.get_str("sentinel_one_cloud_funnel.event.type").is_some_and(|s| s.to_lowercase().contains("modif"))) };
            if _cond {
            event.set("event.action", Value::Array(vec![json!("change")]))?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.type") && event.get_str("sentinel_one_cloud_funnel.event.type").is_some_and(|s| s.to_lowercase().contains("rename")) };
            if _cond {
            event.set("event.action", Value::Array(vec![json!("rename")]))?;
            }

                if event.has_value("json.k8sCluster.containerId") {
                    event.rename("json.k8sCluster.containerId", "sentinel_one_cloud_funnel.event.k8s_cluster.container.id")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.id", v)?;
            }

                if event.has_value("json.k8sCluster.containerImage.sha256") {
                    event.rename("json.k8sCluster.containerImage.sha256", "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256")?;
                }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256") };
            if _cond {
                event.append_unique("container.image.hash.all", json!(event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.k8sCluster.containerImage.value") {
                    event.rename("json.k8sCluster.containerImage.value", "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.value")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.value").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.image.name", v)?;
            }

                if event.has_value("json.k8sCluster.containerLabels") {
                    event.rename("json.k8sCluster.containerLabels", "sentinel_one_cloud_funnel.event.k8s_cluster.container.labels")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.labels").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.labels", v)?;
            }

                if event.has_value("json.k8sCluster.containerName") {
                    event.rename("json.k8sCluster.containerName", "sentinel_one_cloud_funnel.event.k8s_cluster.container.name")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.k8s_cluster.container.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("container.name", v)?;
            }

            let _cond = { event.has_value("json.tgt.file.creationTime") && event.get_str("json.tgt.file.creationTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.tgt.file.creationTime") {
                    match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                        Some(parsed) => event.set("sentinel_one_cloud_funnel.event.tgt.file.creation_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.tgt.file.creationTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_json_tgt_file_creationTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.tgt.file.creation_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.created", v)?;
            }

                if event.has_value("json.tgt.file.extension") {
                    event.rename("json.tgt.file.extension", "sentinel_one_cloud_funnel.event.tgt.file.extension")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.tgt.file.extension").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.extension", v)?;
            }

                if event.has_value("json.tgt.file.md5") {
                    event.rename("json.tgt.file.md5", "sentinel_one_cloud_funnel.event.tgt.file.md5")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.tgt.file.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.md5", v)?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("file.hash.md5").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.tgt.file.sha1") {
                    event.rename("json.tgt.file.sha1", "sentinel_one_cloud_funnel.event.tgt.file.sha1")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.tgt.file.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.sha1", v)?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("file.hash.sha1").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.tgt.file.sha256") {
                    event.rename("json.tgt.file.sha256", "sentinel_one_cloud_funnel.event.tgt.file.sha256")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.tgt.file.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("file.hash.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("json.tgt.file.modificationTime") && event.get_str("json.tgt.file.modificationTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.tgt.file.modificationTime") {
                    match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                        Some(parsed) => event.set("sentinel_one_cloud_funnel.event.tgt.file.modification_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.tgt.file.modificationTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_json_tgt_file_modificationTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.tgt.file.modification_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mtime", v)?;
            }

                if event.has_value("json.tgt.file.path") {
                    event.rename("json.tgt.file.path", "sentinel_one_cloud_funnel.event.tgt.file.path")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.tgt.file.path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.path", v)?;
            }

                if event.has_value("json.tgt.file.type") {
                    event.rename("json.tgt.file.type", "sentinel_one_cloud_funnel.event.tgt.file.type")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.tgt.file.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.type", v)?;
            }

            let _cond = { event.get("sentinel_one_cloud_funnel.event.tgt.file.path").is_some_and(|v| v.is_string()) && event.get_as_string("sentinel_one_cloud_funnel.event.tgt.file.path").is_some_and(|s| s.len() > 1) };
            if _cond {
                // Painless script
                // Source: def path = ctx.sentinel_one_cloud_funnel.event.tgt.file.path;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx == -1) {\n  idx = path.lastIndexOf(\"/\");\n}\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = ctx.file.name.lastIndexOf(\".\");\n    if (extIdx > -1 && ctx.file.type == \"file\") {\n        ctx.file.extension = ctx.file.name.substring(extIdx+1);\n    }\n}\nif (path.indexOf(':') == 1) {\n  ctx.file.drive_letter = path.substring(0, 1).toUpperCase();\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def path = ctx.sentinel_one_cloud_funnel.event.tgt.file.path;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx == -1) {\n  idx = path.lastIndexOf(\"/\");\n}\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = ctx.file.name.lastIndexOf(\".\");\n    if (extIdx > -1 && ctx.file.type == \"file\") {\n        ctx.file.extension = ctx.file.name.substring(extIdx+1);\n    }\n}\nif (path.indexOf(':') == 1) {\n  ctx.file.drive_letter = path.substring(0, 1).toUpperCase();\n}"#))?;
            }

            let _cond = { event.get_str("json.tgt.file.size") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.tgt.file.size") {
                if let Some(val) = event.get("json.tgt.file.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.tgt.file.size".into(),
                            message,
                        })?;
                    event.set("sentinel_one_cloud_funnel.event.tgt.file.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_tgt_file_size")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.tgt.file.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }

            let _cond = { event.get_str("json.src.process.tid") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.src.process.tid") {
                if let Some(val) = event.get("json.src.process.tid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.src.process.tid".into(),
                            message,
                        })?;
                    event.set("sentinel_one_cloud_funnel.event.src.process.tid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_src_process_tid")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.src.process.tid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.thread.id", v)?;
            }

                if event.has_value("json.tgt.file.oldMd5") {
                    event.rename("json.tgt.file.oldMd5", "sentinel_one_cloud_funnel.event.tgt.file.old.md5")?;
                }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.tgt.file.old.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("sentinel_one_cloud_funnel.event.tgt.file.old.md5").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.tgt.file.oldPath") {
                    event.rename("json.tgt.file.oldPath", "sentinel_one_cloud_funnel.event.tgt.file.old.path")?;
                }

                if event.has_value("json.tgt.file.oldSha1") {
                    event.rename("json.tgt.file.oldSha1", "sentinel_one_cloud_funnel.event.tgt.file.old.sha1")?;
                }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.tgt.file.old.sha1") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("sentinel_one_cloud_funnel.event.tgt.file.old.sha1").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.tgt.file.oldSha256") {
                    event.rename("json.tgt.file.oldSha256", "sentinel_one_cloud_funnel.event.tgt.file.old.sha256")?;
                }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.tgt.file.old.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("sentinel_one_cloud_funnel.event.tgt.file.old.sha256").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.k8sCluster.controllerLabels") {
                    event.rename("json.k8sCluster.controllerLabels", "sentinel_one_cloud_funnel.event.k8s_cluster.controller.labels")?;
                }

                if event.has_value("json.k8sCluster.controllerName") {
                    event.rename("json.k8sCluster.controllerName", "sentinel_one_cloud_funnel.event.k8s_cluster.controller.name")?;
                }

                if event.has_value("json.k8sCluster.controllerType") {
                    event.rename("json.k8sCluster.controllerType", "sentinel_one_cloud_funnel.event.k8s_cluster.controller.type")?;
                }

                if event.has_value("json.k8sCluster.name") {
                    event.rename("json.k8sCluster.name", "sentinel_one_cloud_funnel.event.k8s_cluster.name")?;
                }

                if event.has_value("json.k8sCluster.namespace") {
                    event.rename("json.k8sCluster.namespace", "sentinel_one_cloud_funnel.event.k8s_cluster.namespace.value")?;
                }

                if event.has_value("json.k8sCluster.namespaceLabels") {
                    event.rename("json.k8sCluster.namespaceLabels", "sentinel_one_cloud_funnel.event.k8s_cluster.namespace.labels")?;
                }

                if event.has_value("json.k8sCluster.nodeName") {
                    event.rename("json.k8sCluster.nodeName", "sentinel_one_cloud_funnel.event.k8s_cluster.node_name")?;
                }

                if event.has_value("json.k8sCluster.podLabels") {
                    event.rename("json.k8sCluster.podLabels", "sentinel_one_cloud_funnel.event.k8s_cluster.pod.labels")?;
                }

                if event.has_value("json.k8sCluster.podName") {
                    event.rename("json.k8sCluster.podName", "sentinel_one_cloud_funnel.event.k8s_cluster.pod.name")?;
                }

                if event.has_value("json.src.process.reasonSignatureInvalid") {
                    event.rename("json.src.process.reasonSignatureInvalid", "sentinel_one_cloud_funnel.event.src.process.reason_signature_invalid")?;
                }

            let _cond = { event.get_str("json.src.process.rpid") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.src.process.rpid") {
                if let Some(val) = event.get("json.src.process.rpid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.src.process.rpid".into(),
                            message,
                        })?;
                    event.set("sentinel_one_cloud_funnel.event.src.process.rpid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.task.path") {
                    event.rename("json.task.path", "sentinel_one_cloud_funnel.event.task.path")?;
                }

                if event.has_value("json.tgt.file.convictedBy") {
                    event.rename("json.tgt.file.convictedBy", "sentinel_one_cloud_funnel.event.tgt.file.convicted_by")?;
                }

                if event.has_value("json.tgt.file.description") {
                    event.rename("json.tgt.file.description", "sentinel_one_cloud_funnel.event.tgt.file.description")?;
                }

                if event.has_value("json.tgt.file.id") {
                    event.rename("json.tgt.file.id", "sentinel_one_cloud_funnel.event.tgt.file.id")?;
                }

                if event.has_value("json.tgt.file.internalName") {
                    event.rename("json.tgt.file.internalName", "sentinel_one_cloud_funnel.event.tgt.file.internal_name")?;
                }

            let _cond = { event.get_str("json.tgt.file.isExecutable") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.tgt.file.isExecutable") {
                if let Some(val) = event.get("json.tgt.file.isExecutable") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.tgt.file.isExecutable".into(),
                            message,
                        })?;
                    event.set("sentinel_one_cloud_funnel.event.tgt.file.is_executable", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_tgt_file_isExecutable")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.tgt.file.location") {
                    event.rename("json.tgt.file.location", "sentinel_one_cloud_funnel.event.tgt.file.location")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
