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
            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "sailpoint_identity_sc.events")?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("sailpoint_identity_sc.events.id") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "sailpoint_identity_sc.events.id".into(),
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

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            // Painless script
            // Source: Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\nif (ctx.sailpoint_identity_sc.events != null) {\n  ctx.sailpoint_identity_sc.events = keysToSnakeCase(ctx.sailpoint_identity_sc.events);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\nif (ctx.sailpoint_identity_sc.events != null) {\n  ctx.sailpoint_identity_sc.events = keysToSnakeCase(ctx.sailpoint_identity_sc.events);\n}\n"#
                ),
            )?;

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("iam"))?;

            event.set("event.dataset", json!("sailpoint_identity_sc.events"))?;

            event.set("event.module", json!("sailpoint_identity_sc"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.dataset", json!("sailpoint_identity_sc.events"))?;

            event.set("event.module", json!("sailpoint_identity_sc"))?;

            if let Some(v) = event
                .get("sailpoint_identity_sc.events.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("ctx.sailpoint_identity_sc.events.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("sailpoint_identity_sc.events.ip_address")
                    && event.get_str("sailpoint_identity_sc.events.ip_address") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("sailpoint_identity_sc.events.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("sailpoint_identity_sc.events.actor.name")
                    && event.get_str("sailpoint_identity_sc.events.actor.name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("sailpoint_identity_sc.events.actor.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("sailpoint_identity_sc.events.host_name")
                    && event.get_str("sailpoint_identity_sc.events.host_name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("sailpoint_identity_sc.events.attributes.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("sailpoint_identity_sc.events.ip_address")
                    && event.get_str("sailpoint_identity_sc.events.ip_address") != Some("")
            };
            if _cond {
                if event.has_value("sailpoint_identity_sc.events.ip_address") {
                    if let Some(val) = event.get("sailpoint_identity_sc.events.ip_address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "sailpoint_identity_sc.events.ip_address".into(),
                                message,
                            }
                        })?;
                        event.set("sailpoint_identity_sc.events.ip_address", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("sailpoint_identity_sc.events.ip_address") == Some("") };
            if _cond {
                event.remove("sailpoint_identity_sc.events.ip_address");
            }

            let _cond = {
                event.has_value("sailpoint_identity_sc.events.ip_address")
                    && event.get_str("sailpoint_identity_sc.events.ip_address") != Some("")
            };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("sailpoint_identity_sc.events.ip_address")
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

            let _cond = { event.has_value("sailpoint_identity_sc.events.actor.id") };
            if _cond {
                if let Some(v) = event.get("sailpoint_identity_sc.events.actor.id").cloned() {
                    event.set("user.entity.id", v)?;
                }
            }

            let _cond = { event.has_value("sailpoint_identity_sc.events.actor.display_name") };
            if _cond {
                if let Some(v) = event
                    .get("sailpoint_identity_sc.events.actor.display_name")
                    .cloned()
                {
                    event.set("user.entity.name", v)?;
                }
            }

            let _cond = {
                !event.has_value("sailpoint_identity_sc.events.actor.display_name")
                    && event.has_value("sailpoint_identity_sc.events.actor.name")
            };
            if _cond {
                if let Some(v) = event
                    .get("sailpoint_identity_sc.events.actor.name")
                    .cloned()
                {
                    event.set("user.entity.name", v)?;
                }
            }

            let _cond = { event.has_value("sailpoint_identity_sc.events.actor.name") };
            if _cond {
                if let Some(v) = event
                    .get("sailpoint_identity_sc.events.actor.name")
                    .cloned()
                {
                    event.set("user.entity.name", v)?;
                }
            }

            let _cond = {
                event.has_value("sailpoint_identity_sc.events.actor.id")
                    || event.has_value("sailpoint_identity_sc.events.actor.name")
            };
            if _cond {
                event.append_unique("user.entity.type", json!("user"))?;
            }

            let _cond = {
                event.has_value("sailpoint_identity_sc.events.actor.id")
                    || event.has_value("sailpoint_identity_sc.events.actor.name")
            };
            if _cond {
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("user.entity.lifecycle.last_activity", v)?;
                }
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
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
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
