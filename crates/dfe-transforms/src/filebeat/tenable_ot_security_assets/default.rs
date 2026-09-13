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
            event.rename("message", "tenable_ot_security.assets")?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("tenable_ot_security.assets.firstSeen") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("tenable_ot_security.assets.id") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("tenable_ot_security.assets.lastHit") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("tenable_ot_security.assets.lastSeen") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("tenable_ot_security.assets.lastSnapshot") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("tenable_ot_security.assets.lastUpdate") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("tenable_ot_security.assets.runStatusTime") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "fingerprint")?;
                event.set("_ingest.on_failure_processor_tag", "fingerprinting")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).size() == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            // Painless script
            // Source: Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\nif (ctx.tenable_ot_security.assets != null) {\n  ctx.tenable_ot_security.assets = keysToSnakeCase(ctx.tenable_ot_security.assets);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\nif (ctx.tenable_ot_security.assets != null) {\n  ctx.tenable_ot_security.assets = keysToSnakeCase(ctx.tenable_ot_security.assets);\n}\n"#
                ),
            )?;

            // Painless script
            // Source: def normalize(def obj) {\n  if (obj instanceof Map) {\n        Map newObj = new HashMap();\n        for (entry in obj.entrySet()) {\n            def key = entry.getKey();\n            def value = entry.getValue();\n\n            // If the value is a map with a single key \"nodes\", replace it with its contents\n            if (value instanceof Map && value.containsKey(\"nodes\")) {\n                value = value.get(\"nodes\");\n            }\n\n            // Recursively process if value is a map or list\n            newObj.put(key, normalize(value));\n        }\n        return newObj;\n    } else if (obj instanceof List) {\n        List newList = new ArrayList();\n        for (item in obj) {\n            newList.add(normalize(item));\n        }\n        return newList;\n    } else {\n        return obj;\n    }\n   }\n// Apply normalization to every occurrence of \"nodes\" in assets.\nif (ctx.containsKey(\"tenable_ot_security\") && ctx.tenable_ot_security.containsKey(\"assets\")) {\n  ctx.tenable_ot_security.assets = normalize(ctx.tenable_ot_security.assets);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def normalize(def obj) {\n  if (obj instanceof Map) {\n        Map newObj = new HashMap();\n        for (entry in obj.entrySet()) {\n            def key = entry.getKey();\n            def value = entry.getValue();\n\n            // If the value is a map with a single key \"nodes\", replace it with its contents\n            if (value instanceof Map && value.containsKey(\"nodes\")) {\n                value = value.get(\"nodes\");\n            }\n\n            // Recursively process if value is a map or list\n            newObj.put(key, normalize(value));\n        }\n        return newObj;\n    } else if (obj instanceof List) {\n        List newList = new ArrayList();\n        for (item in obj) {\n            newList.add(normalize(item));\n        }\n        return newList;\n    } else {\n        return obj;\n    }\n   }\n// Apply normalization to every occurrence of \"nodes\" in assets.\nif (ctx.containsKey(\"tenable_ot_security\") && ctx.tenable_ot_security.containsKey(\"assets\")) {\n  ctx.tenable_ot_security.assets = normalize(ctx.tenable_ot_security.assets);\n}\n"#
                ),
            )?;

            let _cond = { event.has_value("tenable_ot_security.assets.risk.total_risk") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("tenable_ot_security.assets.risk.total_risk") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "tenable_ot_security.assets.risk.total_risk".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_ot_security.assets.risk.total_risk", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_total_risk_to_double",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    if event
                        .remove("tenable_ot_security.assets.risk.total_risk")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "tenable_ot_security.assets.risk.total_risk".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.set("event.kind", json!("event"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            event.append("event.category", json!("iam"))?;

            event.set("event.module", json!("tenable_ot_security"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.dataset", json!("tenable_ot_security.assets"))?;

            if let Some(v) = event
                .get("tenable_ot_security.assets.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.get("tenable_ot_security.assets.direct_ips").is_some_and(|v| v.is_array()) && event.get("tenable_ot_security.assets.direct_ips").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                foreach_array(event, "tenable_ot_security.assets.direct_ips", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.get("tenable_ot_security.assets.direct_macs").is_some_and(|v| v.is_array()) && event.get("tenable_ot_security.assets.direct_macs").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                foreach_array(event, "tenable_ot_security.assets.direct_macs", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.get("tenable_ot_security.assets.direct_ips").is_some_and(|v| v.is_array()) && event.get("tenable_ot_security.assets.direct_ips").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                foreach_array(event, "tenable_ot_security.assets.direct_ips", |event| {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event.get("tenable_ot_security.assets.direct_ips").is_some_and(|v| v.is_array()) && event.get("tenable_ot_security.assets.direct_ips").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                foreach_array(event, "tenable_ot_security.assets.direct_ips", |event| {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("host.ip") {
                if let Some(ip_str) = event.get_string("host.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("host.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("host.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("host.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("host.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("host.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("host.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("host.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("host.geo.location", v.clone())?;
                        }
                    }
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
