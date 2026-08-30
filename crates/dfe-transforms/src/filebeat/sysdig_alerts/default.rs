// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            let _cond = { event.has_value("message") };
            if _cond {
                parse_json_field(event, "message", "json")?;
            }

            event.rename("json", "sysdig")?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "sysdig.content", "*")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "sysdig.labels", "*")?;
                Ok(())
            })();

            if let Some(v) = event
                .get("sysdig.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("sysdig.source")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if let Some(v) = event
                .get("sysdig.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.severity", v)?;
            }

            if let Some(v) = event
                .get("sysdig.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.syslog.severity.code", v)?;
            }

            let _cond = {
                event.has_value("sysdig.severity") && event.get_i64("sysdig.severity") == Some(0)
            };
            if _cond {
                event.set("log.syslog.severity.name", json!("emergency"))?;
            }

            let _cond = {
                event.has_value("sysdig.severity") && event.get_i64("sysdig.severity") == Some(1)
            };
            if _cond {
                event.set("log.syslog.severity.name", json!("alert"))?;
            }

            let _cond = {
                event.has_value("sysdig.severity") && event.get_i64("sysdig.severity") == Some(2)
            };
            if _cond {
                event.set("log.syslog.severity.name", json!("critical"))?;
            }

            let _cond = {
                event.has_value("sysdig.severity") && event.get_i64("sysdig.severity") == Some(3)
            };
            if _cond {
                event.set("log.syslog.severity.name", json!("error"))?;
            }

            let _cond = {
                event.has_value("sysdig.severity") && event.get_i64("sysdig.severity") == Some(4)
            };
            if _cond {
                event.set("log.syslog.severity.name", json!("warning"))?;
            }

            let _cond = {
                event.has_value("sysdig.severity") && event.get_i64("sysdig.severity") == Some(5)
            };
            if _cond {
                event.set("log.syslog.severity.name", json!("notice"))?;
            }

            let _cond = {
                event.has_value("sysdig.severity") && event.get_i64("sysdig.severity") == Some(6)
            };
            if _cond {
                event.set("log.syslog.severity.name", json!("informational"))?;
            }

            let _cond = {
                event.has_value("sysdig.severity") && event.get_i64("sysdig.severity") == Some(7)
            };
            if _cond {
                event.set("log.syslog.severity.name", json!("debug"))?;
            }

            let v = Value::Array(vec![json!(
                event
                    .get("sysdig.content.policyOrigin")
                    .map_or_else(String::new, template_to_string)
            )]);
            if !painless_is_empty_value(&v) {
                event.set("rule.author", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.ruleName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("sysdig.content.policyVersion") {
                if let Some(val) = event.get("sysdig.content.policyVersion") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.content.policyVersion".into(),
                            message,
                        }
                    })?;
                    event.set("rule.version", converted)?;
                }
            }

            if let Some(v) = event
                .get("sysdig.content.ruleType")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.category", v)?;
            }

            if let Some(v) = event
                .get("sysdig.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.ruleset", v)?;
            }

            if let Some(v) = event
                .get("sysdig.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("sysdig.event.description", v)?;
            }

            if let Some(v) = event
                .get("sysdig.category")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("sysdig.event.category", v)?;
            }

            if let Some(v) = event
                .get("sysdig.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("sysdig.event.type", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.ruleTags")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tags", v)?;
            }

            if event.has_value("sysdig.agentId") {
                if let Some(val) = event.get("sysdig.agentId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "sysdig.agentId".into(),
                            message,
                        }
                    })?;
                    event.set("agent.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("sysdig.machineId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond = { event.has_value("sysdig.hostMac") };
            if _cond {
                event.append(
                    "host.mac",
                    json!(
                        event
                            .get("sysdig.hostMac")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "-")?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

            if let Some(v) = event
                .get("sysdig.labels.host.hostName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if let Some(v) = event
                .get("sysdig.containerId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.id", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.container.image.tag")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.image.tag", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.container.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.image.name", v)?;
            }

            if let Some(v) = event
                .get("sysdig.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.ruleset", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.output")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.proc.exepath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.proc.cwd")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.working_directory", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.proc.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.proc.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.proc.cmdline")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.proc.ppid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.pid", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.proc.pname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.name", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.proc.pcmdline")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.command_line", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.user.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if let Some(v) = event
                .get("sysdig.content.fields.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("sysdig.content.ruleTags") };
            if _cond {
                // Painless script
                // Source: def mitreRegex = /T\\d{4}/;\nfor (int i = 0; i < ctx.sysdig.content.ruleTags.length; i++) {\n    def tag = ctx.sysdig.content.ruleTags[i];\n    def matcher = mitreRegex.matcher(tag);\n    if (matcher.find()) {\n        ctx['threat.technique.id'] = [matcher.group()];\n        break;\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def mitreRegex = /T\\d{4}/;\nfor (int i = 0; i < ctx.sysdig.content.ruleTags.length; i++) {\n    def tag = ctx.sysdig.content.ruleTags[i];\n    def matcher = mitreRegex.matcher(tag);\n    if (matcher.find()) {\n        ctx['threat.technique.id'] = [matcher.group()];\n        break;\n    }\n}\n"#
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sysdig.labels.cloudProvider.account.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            let _cond = {
                event.has_value("sysdig.labels.cloudProvider.name")
                    && event.get_str("sysdig.labels.cloudProvider.name") == Some("gcp")
            };
            if _cond {
                if let Some(v) = event
                    .get("sysdig.labels.gcp.availabilityZone")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.availability_zone", v)?;
                }
            }

            let _cond = { event.has_value("sysdig.labels.gcp.instanceId") };
            if _cond {
                if let Some(v) = event
                    .get("sysdig.labels.gcp.instanceId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            let _cond = { event.has_value("sysdig.labels.azure.instanceId") };
            if _cond {
                if let Some(v) = event
                    .get("sysdig.labels.azure.instanceId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            let _cond = { event.has_value("sysdig.labels.gcp.instanceName") };
            if _cond {
                if let Some(v) = event
                    .get("sysdig.labels.gcp.instanceName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.name", v)?;
                }
            }

            let _cond = { event.has_value("sysdig.labels.azure.instanceName") };
            if _cond {
                if let Some(v) = event
                    .get("sysdig.labels.gcp.instanceName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.name", v)?;
                }
            }

            let _cond = { event.has_value("sysdig.labels.gcp.machineType") };
            if _cond {
                if let Some(v) = event
                    .get("sysdig.labels.gcp.machineType")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.machine.type", v)?;
                }
            }

            let _cond = { event.has_value("sysdig.labels.azure.instanceSize") };
            if _cond {
                if let Some(v) = event
                    .get("sysdig.labels.azure.instanceSize")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.machine.type", v)?;
                }
            }

            if let Some(v) = event
                .get("sysdig.labels.gcp.projectId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.project.id", v)?;
            }

            if let Some(v) = event
                .get("sysdig.labels.gcp.projectName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.project.name", v)?;
            }

            if let Some(v) = event
                .get("sysdig.labels.cloudProvider.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.provider", v)?;
            }

            if let Some(v) = event
                .get("sysdig.labels.cloudProvider.region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            if let Some(v) = event
                .get("sysdig.labels.kubernetes.cluster.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.cluster.name", v)?;
            }

            if let Some(v) = event
                .get("sysdig.labels.kubernetes.namespace.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.namespace", v)?;
            }

            if let Some(v) = event
                .get("sysdig.labels.kubernetes.pod.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.resource.name", v)?;
            }

            if let Some(v) = event
                .get("sysdig.labels.kubernetes.workload.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.resource.parent.type", v)?;
            }

            let _cond = {
                event.has_value("sysdig.labels.kubernetes.cluster.name")
                    || event.has_value("sysdig.labels.kubernetes.namespace.name")
                    || event.has_value("sysdig.labels.kubernetes.pod.name")
                    || event.has_value("sysdig.labels.kubernetes.workload.type")
            };
            if _cond {
                event.set("orchestrator.type", json!("kubernetes"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
