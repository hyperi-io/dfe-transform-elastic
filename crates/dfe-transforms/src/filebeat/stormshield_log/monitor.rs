// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `monitor` pipeline.
pub struct Monitor;

impl Transform for Monitor {
    fn name(&self) -> &str {
        "monitor"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                // Painless script
                // Source: def deviceStats = [:]; def specialCases = [:]; ctx.stormshield.forEach((k, v) -> {\n  params.devices.forEach(d -> {\n    if (k.startsWith(d)) {\n      deviceStats[k] = [:];\n      deviceStats[k][\"value\"] = v;\n      deviceStats[k][\"stem\"] = d;\n    }\n\n    return true;\n  });\n  params.special_cases.forEach(s -> {\n    def sp_k = s[\"key\"];\n    def sp_v = s[\"value\"];\n    if (k.startsWith(sp_k)) {\n      def _map = [:];\n      def values = v.splitOnToken(',', 3);\n      for (int i = 0; i < sp_v.length; ++i) {\n        _map[sp_v[i]] = Long.parseLong(values[i]);\n      }\n      specialCases[sp_k] = _map;\n    }\n\n    return true;\n  });\n});\nif (deviceStats.isEmpty() && specialCases.isEmpty()) {\n  return;\n}\ndef result = [:]; for (entry in deviceStats.entrySet()) {\n  def entryValue = entry.getValue();\n  def stem = entryValue[\"stem\"];\n  if (!result.containsKey(stem)) {\n    result[stem] = [];\n  }\n  def values = entryValue[\"value\"].splitOnToken(',', 7);\n  def item = [:];\n  item[\"original\"] = entry.getKey();\n  for (int i = 0; i < values.length; ++i) {\n    if (i == 0) {\n      item[params.stats[i]] = values[i];\n    } else {\n      item[params.stats[i]] = Long.parseLong(values[i]);        \n    }\n  }\n  result[stem].add(item);\n  ctx.stormshield.remove(entry.getKey());\n} for (entry in result.entrySet()) {\n  result[entry.getKey()].sort((a, b) -> a[\"original\"].compareTo(b[\"original\"]));\n} if (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield[\"metadata\"] = [:];\n} ctx.stormshield.metadata.device_stats = result; for (entry in specialCases.entrySet()) {\n  ctx.stormshield.metadata.device_stats[entry.getKey()] = entry.getValue();\n  ctx.stormshield.remove(entry.getKey());\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def deviceStats = [:]; def specialCases = [:]; ctx.stormshield.forEach((k, v) -> {\n  params.devices.forEach(d -> {\n    if (k.startsWith(d)) {\n      deviceStats[k] = [:];\n      deviceStats[k][\"value\"] = v;\n      deviceStats[k][\"stem\"] = d;\n    }\n\n    return true;\n  });\n  params.special_cases.forEach(s -> {\n    def sp_k = s[\"key\"];\n    def sp_v = s[\"value\"];\n    if (k.startsWith(sp_k)) {\n      def _map = [:];\n      def values = v.splitOnToken(',', 3);\n      for (int i = 0; i < sp_v.length; ++i) {\n        _map[sp_v[i]] = Long.parseLong(values[i]);\n      }\n      specialCases[sp_k] = _map;\n    }\n\n    return true;\n  });\n});\nif (deviceStats.isEmpty() && specialCases.isEmpty()) {\n  return;\n}\ndef result = [:]; for (entry in deviceStats.entrySet()) {\n  def entryValue = entry.getValue();\n  def stem = entryValue[\"stem\"];\n  if (!result.containsKey(stem)) {\n    result[stem] = [];\n  }\n  def values = entryValue[\"value\"].splitOnToken(',', 7);\n  def item = [:];\n  item[\"original\"] = entry.getKey();\n  for (int i = 0; i < values.length; ++i) {\n    if (i == 0) {\n      item[params.stats[i]] = values[i];\n    } else {\n      item[params.stats[i]] = Long.parseLong(values[i]);        \n    }\n  }\n  result[stem].add(item);\n  ctx.stormshield.remove(entry.getKey());\n} for (entry in result.entrySet()) {\n  result[entry.getKey()].sort((a, b) -> a[\"original\"].compareTo(b[\"original\"]));\n} if (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield[\"metadata\"] = [:];\n} ctx.stormshield.metadata.device_stats = result; for (entry in specialCases.entrySet()) {\n  ctx.stormshield.metadata.device_stats[entry.getKey()] = entry.getValue();\n  ctx.stormshield.remove(entry.getKey());\n}"#), cached_params!("{\"special_cases\":[{\"key\":\"CPU\",\"value\":[\"user_time\",\"kernel_time\",\"system_disruption\"]}],\"devices\":[\"agg\",\"ipsec\",\"sslvpn\",\"Wifi\",\"Qid\",\"Vlan\"],\"stats\":[\"name\",\"incoming_throughput\",\"maximum_incoming_throughput\",\"outgoing_throughput\",\"maximum_outgoing_throughput\",\"packets_accepted\",\"packets_blocked\"]}"))?;

                // Painless script
                // Source: def deviceStats = [:]; ctx.stormshield.forEach((k, v) -> {\n  params.devices.forEach(d -> {\n    if (k.startsWith(d)) {\n      deviceStats[k] = [:];\n      deviceStats[k][\"value\"] = v;\n      deviceStats[k][\"stem\"] = d;\n    }\n\n    return true;\n  });\n});\nif (deviceStats.isEmpty()) {\n  return;\n}\nif (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} def ports = []; for (entry in deviceStats.entrySet()) {\n  def entryKey = entry.getKey();\n  def entryValue = entry.getValue();\n  def values = entryValue[\"value\"].splitOnToken(',', 7);\n  def item = [:];\n  ports.add(entryKey);\n  item[\"original\"] = entryKey;\n  for (int i = 0; i < values.length; ++i) {\n    if (i == 0) {\n      item[params.stats[i]] = values[i];\n    } else {\n      item[params.stats[i]] = Long.parseLong(values[i]);        \n    }\n  }\n  ctx.stormshield.remove(entryKey);\n  ctx.stormshield.metadata[entryKey] = item;\n} ports.sort((a, b) -> a.compareTo(b)); ctx.stormshield.ports = ports;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def deviceStats = [:]; ctx.stormshield.forEach((k, v) -> {\n  params.devices.forEach(d -> {\n    if (k.startsWith(d)) {\n      deviceStats[k] = [:];\n      deviceStats[k][\"value\"] = v;\n      deviceStats[k][\"stem\"] = d;\n    }\n\n    return true;\n  });\n});\nif (deviceStats.isEmpty()) {\n  return;\n}\nif (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} def ports = []; for (entry in deviceStats.entrySet()) {\n  def entryKey = entry.getKey();\n  def entryValue = entry.getValue();\n  def values = entryValue[\"value\"].splitOnToken(',', 7);\n  def item = [:];\n  ports.add(entryKey);\n  item[\"original\"] = entryKey;\n  for (int i = 0; i < values.length; ++i) {\n    if (i == 0) {\n      item[params.stats[i]] = values[i];\n    } else {\n      item[params.stats[i]] = Long.parseLong(values[i]);        \n    }\n  }\n  ctx.stormshield.remove(entryKey);\n  ctx.stormshield.metadata[entryKey] = item;\n} ports.sort((a, b) -> a.compareTo(b)); ctx.stormshield.ports = ports;"#), cached_params!("{\"devices\":[\"Ethernet\"],\"stats\":[\"name\",\"incoming_throughput\",\"maximum_incoming_throughput\",\"outgoing_throughput\",\"maximum_outgoing_throughput\",\"packets_accepted\",\"packets_blocked\"]}"))?;

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
