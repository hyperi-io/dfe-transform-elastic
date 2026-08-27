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
            let _cond = { event.has_value("event.action") };
            if _cond {
                // Painless script
                // Source: def action = ctx.event.action;\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.kind = m.kind;\n  ctx.event.category = new ArrayList(m.category);\n} else {\n  ctx.event.kind = 'event';\n  ctx.event.category = ['configuration'];\n}\n\n// Add a security-domain category (malware/vulnerability) for specific\n// checks, keyed by check id; the base 'configuration' category is kept.\nif (ctx.rule?.id != null) {\n  def domain = params.check_category.get(ctx.rule.id);\n  if (domain != null && !ctx.event.category.contains(domain)) {\n    ctx.event.category.add(domain);\n  }\n}\n\n// blocked/resolved issues are a change; a newly detected issue is a creation.\n// blocked_device_at is a future deadline, not an immediate state: the device\n// is only actually blocked once that deadline has passed relative to this event.\nboolean pendingBlock = (ctx.kolide?.issues?.blocks_device_at != null);\nboolean blocked = false;\nif (pendingBlock && ctx['@timestamp'] != null) {\n  ZonedDateTime blockAt = ZonedDateTime.parse(ctx.kolide.issues.blocks_device_at);\n  ZonedDateTime eventTime = ZonedDateTime.parse(ctx['@timestamp']);\n  blocked = !blockAt.isAfter(eventTime);\n}\nboolean resolved = (ctx.kolide?.issues?.resolved_at != null) || (action == 'issues.resolved');\nctx.event.type = (pendingBlock || resolved) ? ['change'] : ['creation'];\n\n// Persist the computed blocked state for the fingerprint processor in\n// default.yml, so pending/blocked/resolved each fingerprint to a distinct _id.\nctx._tmp = ctx._tmp == null ? [:] : ctx._tmp;\nctx._tmp.blocked = blocked;\n\n// Precedence: a resolved issue takes priority over a still-blocking one;\n// a pending future block takes priority over a plain open issue.\nif (action == 'issue') {\n  if (resolved) {\n    ctx.event.action = 'resolved';\n  } else if (blocked) {\n    ctx.event.action = 'blocked';\n  } else if (pendingBlock) {\n    ctx.event.action = 'will_be_blocked';\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def action = ctx.event.action;\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.kind = m.kind;\n  ctx.event.category = new ArrayList(m.category);\n} else {\n  ctx.event.kind = 'event';\n  ctx.event.category = ['configuration'];\n}\n\n// Add a security-domain category (malware/vulnerability) for specific\n// checks, keyed by check id; the base 'configuration' category is kept.\nif (ctx.rule?.id != null) {\n  def domain = params.check_category.get(ctx.rule.id);\n  if (domain != null && !ctx.event.category.contains(domain)) {\n    ctx.event.category.add(domain);\n  }\n}\n\n// blocked/resolved issues are a change; a newly detected issue is a creation.\n// blocked_device_at is a future deadline, not an immediate state: the device\n// is only actually blocked once that deadline has passed relative to this event.\nboolean pendingBlock = (ctx.kolide?.issues?.blocks_device_at != null);\nboolean blocked = false;\nif (pendingBlock && ctx['@timestamp'] != null) {\n  ZonedDateTime blockAt = ZonedDateTime.parse(ctx.kolide.issues.blocks_device_at);\n  ZonedDateTime eventTime = ZonedDateTime.parse(ctx['@timestamp']);\n  blocked = !blockAt.isAfter(eventTime);\n}\nboolean resolved = (ctx.kolide?.issues?.resolved_at != null) || (action == 'issues.resolved');\nctx.event.type = (pendingBlock || resolved) ? ['change'] : ['creation'];\n\n// Persist the computed blocked state for the fingerprint processor in\n// default.yml, so pending/blocked/resolved each fingerprint to a distinct _id.\nctx._tmp = ctx._tmp == null ? [:] : ctx._tmp;\nctx._tmp.blocked = blocked;\n\n// Precedence: a resolved issue takes priority over a still-blocking one;\n// a pending future block takes priority over a plain open issue.\nif (action == 'issue') {\n  if (resolved) {\n    ctx.event.action = 'resolved';\n  } else if (blocked) {\n    ctx.event.action = 'blocked';\n  } else if (pendingBlock) {\n    ctx.event.action = 'will_be_blocked';\n  }\n}"#), cached_params!("{\"exact\":{\"issue\":{\"kind\":\"event\",\"category\":[\"configuration\"]},\"issues.new\":{\"kind\":\"event\",\"category\":[\"configuration\"]},\"issues.resolved\":{\"kind\":\"event\",\"category\":[\"configuration\"]}},\"check_category\":{\"17\":\"malware\",\"18\":\"malware\",\"20\":\"malware\",\"75639\":\"malware\",\"41\":\"vulnerability\"}}"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
