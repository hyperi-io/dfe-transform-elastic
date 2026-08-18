// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `springcloudlogs_inner_pipeline` pipeline.
pub struct SpringcloudlogsInnerPipeline;

impl Transform for SpringcloudlogsInnerPipeline {
    fn name(&self) -> &str {
        "springcloudlogs_inner_pipeline"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            // Painless script
            // Source: ctx['_index'] = ctx['_index'].replace('platformlogs', 'springcloudlogs')
            painless_exec(
                event,
                r#"ctx['_index'] = ctx['_index'].replace('platformlogs', 'springcloudlogs')"#,
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

        // ctx.azure.springcloudlogs.category != 'SystemLogs'
        //   && ctx.azure.springcloudlogs.category != 'ApplicationConsole'
        let cond = event
            .get_as_string("azure.springcloudlogs.category")
            .is_none_or(|c| c != "SystemLogs" && c != "ApplicationConsole");
        if cond {
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

        if event.has("azure.springcloudlogs.operation_name") {
            if let Some(val) = event.get("azure.springcloudlogs.operation_name") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("event.action", converted)?;
            }
        }

        Ok(TransformResult::Continue)
    }
}
