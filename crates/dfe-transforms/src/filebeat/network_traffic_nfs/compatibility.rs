// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `compatibility` pipeline.
pub struct Compatibility;

impl Transform for Compatibility {
    fn name(&self) -> &str {
        "compatibility"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("nfs") {
                    event.rename("nfs", "network_traffic.nfs")?;
                }

                if event.has_value("method") {
                    event.rename("method", "network_traffic.nfs.method")?;
                }

                if event.has_value("query") {
                    event.rename("query", "network_traffic.nfs.query")?;
                }

                if event.has_value("rpc") {
                    event.rename("rpc", "network_traffic.nfs.rpc")?;
                }

                dot_expand(event, "", "group.id")?;

                dot_expand(event, "", "user.id")?;

                // Painless script
                // Source: if (ctx['host.hostname'] == null) {\n  return;\n}\nif (ctx.network_traffic == null) {\n  ctx.network_traffic = new HashMap();\n}\nif (ctx.network_traffic.nfs == null) {\n  ctx.network_traffic.nfs = new HashMap();\n}\nif (ctx.network_traffic.nfs.host == null) {\n  ctx.network_traffic.nfs.host = new HashMap();\n}\nctx.network_traffic.nfs.host['hostname'] = ctx['host.hostname'];\nctx.remove('host.hostname');\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"if (ctx['host.hostname'] == null) {\n  return;\n}\nif (ctx.network_traffic == null) {\n  ctx.network_traffic = new HashMap();\n}\nif (ctx.network_traffic.nfs == null) {\n  ctx.network_traffic.nfs = new HashMap();\n}\nif (ctx.network_traffic.nfs.host == null) {\n  ctx.network_traffic.nfs.host = new HashMap();\n}\nctx.network_traffic.nfs.host['hostname'] = ctx['host.hostname'];\nctx.remove('host.hostname');\n"#))?;

            let _cond = { event.has_value("network_traffic.nfs.host.hostname") && event.get_str("network_traffic.nfs.host.hostname") != Some("") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("network_traffic.nfs.host.hostname").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("status") {
                    event.rename("status", "network_traffic.status")?;
                }

                if event.has_value("process.ppid") {
                    event.rename("process.ppid", "process.parent.pid")?;
                }

                event.remove("type");

                event.remove("event.dataset");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
