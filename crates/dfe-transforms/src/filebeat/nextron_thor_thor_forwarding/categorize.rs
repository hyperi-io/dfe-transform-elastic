// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `categorize` pipeline.
pub struct Categorize;

impl Transform for Categorize {
    fn name(&self) -> &str {
        "categorize"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("thor.module") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:];\ndef m = params.get(ctx.thor.module);\nif (m != null) {\n  m.forEach((k, v) -> {\n    if (v instanceof List) {\n      ctx.event[k] = new ArrayList(v);\n    } else {\n      ctx.event[k] = v;\n    }\n  });\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"ctx.event = ctx.event ?: [:];\ndef m = params.get(ctx.thor.module);\nif (m != null) {\n  m.forEach((k, v) -> {\n    if (v instanceof List) {\n      ctx.event[k] = new ArrayList(v);\n    } else {\n      ctx.event[k] = v;\n    }\n  });\n}"#), cached_params!("{\"Amcache\":{\"category\":[\"registry\"],\"kind\":\"event\",\"type\":[\"access\"]},\"AtJobs\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Autoruns\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"DNSCache\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"info\"]},\"EnvCheck\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"EtwWatcher\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Eventlog\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Events\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Filescan\":{\"category\":[\"file\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Firewall\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Hosts\":{\"category\":[\"host\"],\"kind\":\"event\",\"type\":[\"info\"]},\"HotfixCheck\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Init\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"LSASessions\":{\"category\":[\"session\",\"iam\"],\"kind\":\"event\",\"type\":[\"info\"]},\"LogScan\":{\"category\":[\"file\"],\"kind\":\"event\",\"type\":[\"info\"]},\"LoggedIn\":{\"category\":[\"iam\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Mutex\":{\"category\":[\"process\"],\"kind\":\"event\",\"type\":[\"info\"]},\"NetworkSessions\":{\"category\":[\"network\",\"session\"],\"kind\":\"event\",\"type\":[\"info\"]},\"NetworkShares\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Pipes\":{\"category\":[\"process\"],\"kind\":\"event\",\"type\":[\"info\"]},\"ProcessCheck\":{\"category\":[\"process\"],\"kind\":\"event\",\"type\":[\"info\"]},\"ProcessConnections\":{\"category\":[\"process\",\"network\"],\"kind\":\"event\",\"type\":[\"info\"]},\"RegistryChecks\":{\"category\":[\"registry\"],\"kind\":\"event\",\"type\":[\"access\"]},\"RegistryHive\":{\"category\":[\"registry\"],\"kind\":\"event\",\"type\":[\"access\"]},\"Report\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Rootkit\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"SHIMCache\":{\"category\":[\"registry\"],\"kind\":\"event\",\"type\":[\"access\"]},\"ScheduledTasks\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"ServiceCheck\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Sigma\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Startup\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Timestomp\":{\"category\":[\"file\"],\"kind\":\"event\",\"type\":[\"info\"]},\"UserDir\":{\"category\":[\"iam\"],\"kind\":\"event\",\"type\":[\"info\",\"user\"]},\"Users\":{\"category\":[\"iam\"],\"kind\":\"event\",\"type\":[\"info\",\"user\"]},\"VulnerabilityCheck\":{\"category\":[\"vulnerability\"],\"kind\":\"event\",\"type\":[\"info\"]},\"WER\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"WMIPersistence\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"WMIStartup\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Yara\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]}}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_set_event_categorization")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("thor.level") == Some("Alert") };
            if _cond {
            event.set("event.kind", json!("alert"))?;
            }

            let _cond = { event.get_str("thor.message") == Some("Malware file found") };
            if _cond {
                event.append_unique("event.category", json!("malware"))?;
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
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
