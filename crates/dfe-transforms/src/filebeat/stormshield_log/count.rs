// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `count` pipeline.
pub struct Count;

impl Transform for Count {
    fn name(&self) -> &str {
        "count"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                // Painless script
                // Source: boolean isNumeric(def s) {\n  if (!(s instanceof String) || s.isEmpty() || s.length() > 18) {\n    return false;\n  }\n  for (int i = 0; i < s.length(); i++) {\n    if (Character.isDigit(s.charAt(i)) == false) {\n      return false;\n    }\n  }\n  return true;\n} def ruleStats = []; def ruleKeys = []; def str = \"Rule\"; ctx.stormshield.forEach((k, v) -> {\n  if (k.startsWith(str) && k.indexOf(\":\") > str.length()) {\n    ruleKeys.add(k);\n    def parts = k.substring(str.length()).splitOnToken(\":\");\n    if (parts.length == 2 && isNumeric(v) && isNumeric(parts[1]) && parts[0].length() == 1 && isNumeric(parts[0])) {\n      int category = Integer.parseInt(parts[0]);\n      if (category < params.stats.size()) {\n        def item = [:];\n        item[\"original\"] = k;\n        item[\"byte_count\"] = Long.parseLong(v);\n        item[\"category\"] = params.stats[category];\n        item[\"rule_number\"] = Long.parseLong(parts[1]);\n        ruleStats.add(item);\n      }\n    }\n  }\n\n  return true;\n});\nfor (k in ruleKeys) {\n  ctx.stormshield.remove(k);\n}\nif (ruleStats.isEmpty()) {\n  return;\n}\nruleStats.sort((a, b) -> a[\"original\"].compareTo(b[\"original\"])); if (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} ctx.stormshield.metadata.rule_stats = ruleStats;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"boolean isNumeric(def s) {\n  if (!(s instanceof String) || s.isEmpty() || s.length() > 18) {\n    return false;\n  }\n  for (int i = 0; i < s.length(); i++) {\n    if (Character.isDigit(s.charAt(i)) == false) {\n      return false;\n    }\n  }\n  return true;\n} def ruleStats = []; def ruleKeys = []; def str = \"Rule\"; ctx.stormshield.forEach((k, v) -> {\n  if (k.startsWith(str) && k.indexOf(\":\") > str.length()) {\n    ruleKeys.add(k);\n    def parts = k.substring(str.length()).splitOnToken(\":\");\n    if (parts.length == 2 && isNumeric(v) && isNumeric(parts[1]) && parts[0].length() == 1 && isNumeric(parts[0])) {\n      int category = Integer.parseInt(parts[0]);\n      if (category < params.stats.size()) {\n        def item = [:];\n        item[\"original\"] = k;\n        item[\"byte_count\"] = Long.parseLong(v);\n        item[\"category\"] = params.stats[category];\n        item[\"rule_number\"] = Long.parseLong(parts[1]);\n        ruleStats.add(item);\n      }\n    }\n  }\n\n  return true;\n});\nfor (k in ruleKeys) {\n  ctx.stormshield.remove(k);\n}\nif (ruleStats.isEmpty()) {\n  return;\n}\nruleStats.sort((a, b) -> a[\"original\"].compareTo(b[\"original\"])); if (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} ctx.stormshield.metadata.rule_stats = ruleStats;"#), cached_params!("{\"devices\":[\"Rule\"],\"stats\":[\"Implicit Filter Rule\",\"Global Filter Rule\",\"Local Filter Rule\",\"Implicit NAT Rule\",\"Global NAT Rule\",\"Local NAT Rule\"]}"))?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
