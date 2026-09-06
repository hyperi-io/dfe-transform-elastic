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
            event.rename("message", "tenable_ot_security.events")?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("tenable_ot_security.events") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "tenable_ot_security.events".into(),
                        });
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
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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
            // Source: Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\nif (ctx.tenable_ot_security.events != null) {\n  ctx.tenable_ot_security.events = keysToSnakeCase(ctx.tenable_ot_security.events);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\nif (ctx.tenable_ot_security.events != null) {\n  ctx.tenable_ot_security.events = keysToSnakeCase(ctx.tenable_ot_security.events);\n}\n"#
                ),
            )?;

            // Painless script
            // Source: def normalize(def obj) {\n  if (obj instanceof Map) {\n        Map newObj = new HashMap();\n        for (entry in obj.entrySet()) {\n            def key = entry.getKey();\n            def value = entry.getValue();\n\n            // If the value is a map with a single key \"nodes\", replace it with its contents\n            if (value instanceof Map && value.containsKey(\"nodes\")) {\n                value = value.get(\"nodes\");\n            }\n\n            // Recursively process if value is a map or list\n            newObj.put(key, normalize(value));\n        }\n        return newObj;\n    } else if (obj instanceof List) {\n        List newList = new ArrayList();\n        for (item in obj) {\n            newList.add(normalize(item));\n        }\n        return newList;\n    } else {\n        return obj;\n    }\n   }\n// Apply normalization to every occurrence of \"nodes\" in events.\nif (ctx.containsKey(\"tenable_ot_security\") && ctx.tenable_ot_security.containsKey(\"events\")) {\n  ctx.tenable_ot_security.events = normalize(ctx.tenable_ot_security.events);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def normalize(def obj) {\n  if (obj instanceof Map) {\n        Map newObj = new HashMap();\n        for (entry in obj.entrySet()) {\n            def key = entry.getKey();\n            def value = entry.getValue();\n\n            // If the value is a map with a single key \"nodes\", replace it with its contents\n            if (value instanceof Map && value.containsKey(\"nodes\")) {\n                value = value.get(\"nodes\");\n            }\n\n            // Recursively process if value is a map or list\n            newObj.put(key, normalize(value));\n        }\n        return newObj;\n    } else if (obj instanceof List) {\n        List newList = new ArrayList();\n        for (item in obj) {\n            newList.add(normalize(item));\n        }\n        return newList;\n    } else {\n        return obj;\n    }\n   }\n// Apply normalization to every occurrence of \"nodes\" in events.\nif (ctx.containsKey(\"tenable_ot_security\") && ctx.tenable_ot_security.containsKey(\"events\")) {\n  ctx.tenable_ot_security.events = normalize(ctx.tenable_ot_security.events);\n}\n"#
                ),
            )?;

            event.set("event.kind", json!("event"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            event.append("event.category", json!("iam"))?;

            event.set("event.module", json!("tenable_ot_security"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.dataset", json!("tenable_ot_security.events"))?;

            if let Some(v) = event
                .get("tenable_ot_security.events.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("tenable_ot_security.events.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("tenable_ot_security.events.src_ip")
                    && event.get_str("tenable_ot_security.events.src_ip") != Some("")
            };
            if _cond {
                if event.has_value("tenable_ot_security.events.src_ip") {
                    if let Some(val) = event.get("tenable_ot_security.events.src_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "tenable_ot_security.events.src_ip".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_ot_security.events.src_ip", converted)?;
                    }
                }
            }

            let _cond = {
                event.has_value("tenable_ot_security.events.src_ip")
                    || event.get_str("tenable_ot_security.events.src_ip") != Some("")
            };
            if _cond {
                event.append_unique(
                    "source.ip",
                    json!(
                        event
                            .get("tenable_ot_security.events.src_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("tenable_ot_security.events.dst_ip")
                    && event.get_str("tenable_ot_security.events.dst_ip") != Some("")
            };
            if _cond {
                if event.has_value("tenable_ot_security.events.dst_ip") {
                    if let Some(val) = event.get("tenable_ot_security.events.dst_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "tenable_ot_security.events.dst_ip".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_ot_security.events.dst_ip", converted)?;
                    }
                }
            }

            let _cond = {
                event.has_value("tenable_ot_security.events.dst_ip")
                    || event.get_str("tenable_ot_security.events.dst_ip") != Some("")
            };
            if _cond {
                event.append_unique(
                    "destination.ip",
                    json!(
                        event
                            .get("tenable_ot_security.events.dst_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Painless script
            // Source: def events = ctx?.tenable_ot_security?.events;\nif (events != null) {\n  if (events.dst_mac != null) {\n    events.dst_mac = events.dst_mac.replace(\":\", \"-\").toUpperCase();\n  }\n  if (events.src_mac != null) {\n    events.src_mac = events.src_mac.replace(\":\", \"-\").toUpperCase();\n  }\n  def srcInterface = events.src_interface;\n  if (srcInterface?.mac != null) {\n    srcInterface.mac = srcInterface.mac.replace(\":\", \"-\").toUpperCase();\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def events = ctx?.tenable_ot_security?.events;\nif (events != null) {\n  if (events.dst_mac != null) {\n    events.dst_mac = events.dst_mac.replace(\":\", \"-\").toUpperCase();\n  }\n  if (events.src_mac != null) {\n    events.src_mac = events.src_mac.replace(\":\", \"-\").toUpperCase();\n  }\n  def srcInterface = events.src_interface;\n  if (srcInterface?.mac != null) {\n    srcInterface.mac = srcInterface.mac.replace(\":\", \"-\").toUpperCase();\n  }\n}\n"#
                ),
            )?;

            let _cond = {
                event.has_value("tenable_ot_security.events.dst_mac")
                    && event.get_str("tenable_ot_security.events.dst_mac") != Some("")
            };
            if _cond {
                event.append_unique(
                    "destination.mac",
                    json!(
                        event
                            .get("tenable_ot_security.events.dst_mac")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("tenable_ot_security.events.src_mac")
                    && event.get_str("tenable_ot_security.events.src_mac") != Some("")
            };
            if _cond {
                event.append_unique(
                    "source.mac",
                    json!(
                        event
                            .get("tenable_ot_security.events.src_mac")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("tenable_ot_security.events.protocol_raw")
                    && event.get_str("tenable_ot_security.events.protocol_raw") != Some("")
            };
            if _cond {
                event.append_unique(
                    "network.protocol",
                    json!(
                        event
                            .get("tenable_ot_security.events.protocol_raw")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("tenable_ot_security.events.resolved_user")
                    && event.get_str("tenable_ot_security.events.resolved_user") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("tenable_ot_security.events.resolved_user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("tenable_ot_security.events.src_ip")
                    && event.get_str("tenable_ot_security.events.src_ip") != Some("")
            };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("tenable_ot_security.events.src_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
