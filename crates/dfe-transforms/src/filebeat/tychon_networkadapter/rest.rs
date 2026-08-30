// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `rest` pipeline.
pub struct Rest;

impl Transform for Rest {
    fn name(&self) -> &str {
        "rest"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            // Painless script
            // Source: if (ctx.tychon?.host?.adapter?.link_speed?.contains(' ') == true) {\n  String[] parts = ctx.tychon.host.adapter.link_speed.splitOnToken(' ');\n  ctx.tychon.host.adapter.link_speed = Double.parseDouble(parts[0]);\n  if (parts[1] == 'Kbps') {\n    ctx.tychon.host.adapter.link_speed *= 1e3\n  } else if (parts[1] == 'Mbps') {\n    ctx.tychon.host.adapter.link_speed *= 1e6\n  } else if (parts[1] == 'Gbps') {\n    ctx.tychon.host.adapter.link_speed *= 1e9\n  }\n} else {\n  if (ctx.tychon.host == null) {\n    ctx.tychon.host = [:];\n  }\n  if (ctx.tychon.host.adapter == null) {\n    ctx.tychon.host.adapter = [:];\n  }\n  ctx.tychon.host.adapter.link_speed = 0.0;\n}\nctx.tychon.host.adapter.link_speed = (long) ctx.tychon.host.adapter.link_speed;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx.tychon?.host?.adapter?.link_speed?.contains(' ') == true) {\n  String[] parts = ctx.tychon.host.adapter.link_speed.splitOnToken(' ');\n  ctx.tychon.host.adapter.link_speed = Double.parseDouble(parts[0]);\n  if (parts[1] == 'Kbps') {\n    ctx.tychon.host.adapter.link_speed *= 1e3\n  } else if (parts[1] == 'Mbps') {\n    ctx.tychon.host.adapter.link_speed *= 1e6\n  } else if (parts[1] == 'Gbps') {\n    ctx.tychon.host.adapter.link_speed *= 1e9\n  }\n} else {\n  if (ctx.tychon.host == null) {\n    ctx.tychon.host = [:];\n  }\n  if (ctx.tychon.host.adapter == null) {\n    ctx.tychon.host.adapter = [:];\n  }\n  ctx.tychon.host.adapter.link_speed = 0.0;\n}\nctx.tychon.host.adapter.link_speed = (long) ctx.tychon.host.adapter.link_speed;\n"#))?;

        let _cond = { event.has_value("tychon.host.adapter.domain") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("tychon.host.adapter.domain").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("tychon.host.adapter.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("tychon.host.adapter.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("tychon.host.adapter.gateway") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("tychon.host.adapter.gateway").map_or_else(String::new, template_to_string)))?;
        }

        event.set("event.category", Value::Array(vec![json!("configuration")]))?;

        Ok(TransformResult::Continue)
    }
}
