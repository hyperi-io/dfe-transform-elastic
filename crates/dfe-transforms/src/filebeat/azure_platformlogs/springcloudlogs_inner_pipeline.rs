// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `springcloudlogs_inner_pipeline` pipeline.
pub struct SpringcloudlogsInnerPipeline;

impl Transform for SpringcloudlogsInnerPipeline {
    fn name(&self) -> &str {
        "springcloudlogs_inner_pipeline"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: ctx['_index'] = ctx['_index'].replace('platformlogs', 'springcloudlogs')
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx['_index'] = ctx['_index'].replace('platformlogs', 'springcloudlogs')"#
                    ),
                )?;
                Ok(())
            })();

            if event.has("azure.platformlogs") {
                event.rename("azure.platformlogs", "azure.springcloudlogs")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("event.dataset", json!("azure.springcloudlogs"))?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("data_stream.dataset", json!("azure.springcloudlogs"))?;
                Ok(())
            })();

            let _cond = {
                event.get_str("azure.springcloudlogs.category") != Some("SystemLogs")
                    && event.get_str("azure.springcloudlogs.category") != Some("ApplicationConsole")
                    && event.get_str("azure.springcloudlogs.category") != Some("IngressLogs")
                    && event.get_str("azure.springcloudlogs.category") != Some("BuildLogs")
                    && event.get_str("azure.springcloudlogs.category") != Some("ContainerEventLogs")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if event.has("azure.springcloudlogs.LogFormat") {
                event.rename(
                    "azure.springcloudlogs.LogFormat",
                    "azure.springcloudlogs.log_format",
                )?;
            }

            if event.has("azure.springcloudlogs.properties.InstanceName") {
                event.rename(
                    "azure.springcloudlogs.properties.InstanceName",
                    "azure.springcloudlogs.properties.instance_name",
                )?;
            }

            if event.has("azure.springcloudlogs.properties.Log") {
                event.rename(
                    "azure.springcloudlogs.properties.Log",
                    "azure.springcloudlogs.properties.log",
                )?;
            }

            if event.has("azure.springcloudlogs.properties.ServiceName") {
                event.rename(
                    "azure.springcloudlogs.properties.ServiceName",
                    "azure.springcloudlogs.properties.service_name",
                )?;
            }

            if event.has("azure.springcloudlogs.properties.Stream") {
                event.rename(
                    "azure.springcloudlogs.properties.Stream",
                    "azure.springcloudlogs.properties.stream",
                )?;
            }

            if event.has("azure.springcloudlogs.properties.AppName") {
                event.rename(
                    "azure.springcloudlogs.properties.AppName",
                    "azure.springcloudlogs.properties.app_name",
                )?;
            }

            if event.has("azure.springcloudlogs.properties.ServiceId") {
                event.rename(
                    "azure.springcloudlogs.properties.ServiceId",
                    "azure.springcloudlogs.properties.service_id",
                )?;
            }

            if event.has("azure.springcloudlogs.properties.Type") {
                event.rename(
                    "azure.springcloudlogs.properties.Type",
                    "azure.springcloudlogs.properties.type",
                )?;
            }

            if event.has("azure.springcloudlogs.properties.Level") {
                event.rename(
                    "azure.springcloudlogs.properties.Level",
                    "azure.springcloudlogs.level",
                )?;
            }

            if event.has("azure.springcloudlogs.properties.Logger") {
                event.rename(
                    "azure.springcloudlogs.properties.Logger",
                    "azure.springcloudlogs.properties.logger",
                )?;
            }

            if event.has("azure.springcloudlogs.properties.Stack") {
                event.rename(
                    "azure.springcloudlogs.properties.Stack",
                    "azure.springcloudlogs.properties.stack",
                )?;
            }

            if event.has("azure.springcloudlogs.properties.Thread") {
                event.rename(
                    "azure.springcloudlogs.properties.Thread",
                    "azure.springcloudlogs.properties.thread",
                )?;
            }

            if event.has("azure.springcloudlogs.level") {
                event.rename("azure.springcloudlogs.level", "log.level")?;
            }

            if event.has("azure.springcloudlogs.operationName") {
                event.rename(
                    "azure.springcloudlogs.operationName",
                    "azure.springcloudlogs.operation_name",
                )?;
            }

            if event.has_value("azure.springcloudlogs.operation_name") {
                if let Some(val) = event.get("azure.springcloudlogs.operation_name") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "azure.springcloudlogs.operation_name".into(),
                            message,
                        }
                    })?;
                    event.set("event.action", converted)?;
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
