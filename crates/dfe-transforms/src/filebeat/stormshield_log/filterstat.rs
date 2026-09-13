// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `filterstat` pipeline.
pub struct Filterstat;

impl Transform for Filterstat {
    fn name(&self) -> &str {
        "filterstat"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                // Painless script
                // Source: Pattern compositeKeyPattern = /^(\\w+)\\((\\w+)\\/(\\w+)\\)$/; Pattern compositeValuePattern = /^(\\w+)\\/(\\w+)$/;\ndef replacePairs = [:]; def move = []; ctx.stormshield.forEach((k, v) -> {\n  if (k.contains('(')) {\n    replacePairs[k] = v;\n  } else if (Character.isUpperCase(k.charAt(0))) {\n    move.add(k);\n  }\n\n  return true;\n});\nif (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} move.forEach(k -> {\n  ctx.stormshield.metadata[k] = ctx.stormshield[k];\n  ctx.stormshield.remove(k);\n  return true;\n}); replacePairs.forEach((k, v) -> {\n  def keyMatcher = compositeKeyPattern.matcher(k);\n  if (!keyMatcher.matches()) {\n    return false;\n  }\n  def valueMatcher = compositeValuePattern.matcher(v);\n  if (!valueMatcher.matches()) {\n    return false;\n  }\n  def newSubkeys = [:];\n  def newSubkey0 = keyMatcher.group(2);\n  def newSubkey1 = keyMatcher.group(3);\n  newSubkey0 = params.containsKey(newSubkey0) ? params[newSubkey0] : newSubkey0;\n  newSubkey1 = params.containsKey(newSubkey1) ? params[newSubkey1] : newSubkey1;\n  newSubkeys[newSubkey0] = Long.parseLong(valueMatcher.group(1));\n  newSubkeys[newSubkey1] = Long.parseLong(valueMatcher.group(2));\n  ctx.stormshield.metadata[keyMatcher.group(1)] = newSubkeys;\n  ctx.stormshield.remove(k);\n});
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"Pattern compositeKeyPattern = /^(\\w+)\\((\\w+)\\/(\\w+)\\)$/; Pattern compositeValuePattern = /^(\\w+)\\/(\\w+)$/;\ndef replacePairs = [:]; def move = []; ctx.stormshield.forEach((k, v) -> {\n  if (k.contains('(')) {\n    replacePairs[k] = v;\n  } else if (Character.isUpperCase(k.charAt(0))) {\n    move.add(k);\n  }\n\n  return true;\n});\nif (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} move.forEach(k -> {\n  ctx.stormshield.metadata[k] = ctx.stormshield[k];\n  ctx.stormshield.remove(k);\n  return true;\n}); replacePairs.forEach((k, v) -> {\n  def keyMatcher = compositeKeyPattern.matcher(k);\n  if (!keyMatcher.matches()) {\n    return false;\n  }\n  def valueMatcher = compositeValuePattern.matcher(v);\n  if (!valueMatcher.matches()) {\n    return false;\n  }\n  def newSubkeys = [:];\n  def newSubkey0 = keyMatcher.group(2);\n  def newSubkey1 = keyMatcher.group(3);\n  newSubkey0 = params.containsKey(newSubkey0) ? params[newSubkey0] : newSubkey0;\n  newSubkey1 = params.containsKey(newSubkey1) ? params[newSubkey1] : newSubkey1;\n  newSubkeys[newSubkey0] = Long.parseLong(valueMatcher.group(1));\n  newSubkeys[newSubkey1] = Long.parseLong(valueMatcher.group(2));\n  ctx.stormshield.metadata[keyMatcher.group(1)] = newSubkeys;\n  ctx.stormshield.remove(k);\n});"#), cached_params!("{\"i\":\"in_count\",\"o\":\"out_count\"}"))?;

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
