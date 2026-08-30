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
        let v = json!("logs");
        if !painless_is_empty_value(&v) {
            event.set("data_stream.type", v)?;
        }

        let v = json!("osquery_manager.query_profile");
        if !painless_is_empty_value(&v) {
            event.set("data_stream.dataset", v)?;
        }

        if let Some(v) = event
            .get("agent.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("agent_id", v)?;
        }

        if let Some(v) = event
            .get("osquery_profile.query_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("query.name") {
                event.set("query.name", v)?;
            }
        }

        if let Some(v) = event
            .get("action_data.saved_query_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("query.name") {
                event.set("query.name", v)?;
            }
        }

        let _cond = {
            !event.has_value("query.name")
                && event
                    .get("action_data.query")
                    .is_some_and(|v| v.is_string())
                && event.get("action_data.query").is_some_and(|v| !match v {
                    serde_json::Value::String(s) => s.is_empty(),
                    serde_json::Value::Array(a) => a.is_empty(),
                    serde_json::Value::Object(o) => o.is_empty(),
                    serde_json::Value::Null => true,
                    _ => false,
                })
        };
        if _cond {
            // Painless script
            // Source: String sql = ctx.action_data.query;\nif (sql.length() > 1024) {\n  sql = sql.substring(0, 1023) + \"…\";\n}\nif (!(ctx.query instanceof Map)) {\n  ctx.query = new HashMap();\n}\nctx.query.name = sql;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"String sql = ctx.action_data.query;\nif (sql.length() > 1024) {\n  sql = sql.substring(0, 1023) + \"…\";\n}\nif (!(ctx.query instanceof Map)) {\n  ctx.query = new HashMap();\n}\nctx.query.name = sql;\n"#
                ),
            )?;
        }

        event.remove("type");

        Ok(TransformResult::Continue)
    }
}
