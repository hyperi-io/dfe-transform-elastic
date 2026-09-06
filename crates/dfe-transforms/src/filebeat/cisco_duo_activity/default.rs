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
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            event.set("event.kind", json!("event"))?;

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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.ts") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.ts".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_ts")?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.activity_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.akey") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.ts") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.access_device") {
                event.rename("json.access_device", "cisco_duo.activity.access_device")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_duo.activity.access_device.ip.address") {
                    if let Some(val) = event.get("cisco_duo.activity.access_device.ip.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_duo.activity.access_device.ip.address".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_duo.activity.access_device.ip.address", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_ip_address")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("cisco_duo.activity.access_device.ip.address") {
                if let Some(ip_str) =
                    event.get_string("cisco_duo.activity.access_device.ip.address")
                {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set(
                                "cisco_duo.activity.access_device.geo.country_iso_code",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set(
                                "cisco_duo.activity.access_device.geo.country_name",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set(
                                "cisco_duo.activity.access_device.geo.continent_name",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set(
                                "cisco_duo.activity.access_device.geo.region_iso_code",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set(
                                "cisco_duo.activity.access_device.geo.region_name",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event
                                .set("cisco_duo.activity.access_device.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event
                                .set("cisco_duo.activity.access_device.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event
                                .set("cisco_duo.activity.access_device.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("cisco_duo.activity.access_device.ip.address") {
                if let Some(ip_str) =
                    event.get_string("cisco_duo.activity.access_device.ip.address")
                {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("cisco_duo.activity.access_device.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set(
                                "cisco_duo.activity.access_device.as.organization_name",
                                v.clone(),
                            )?;
                        }
                    }
                }
            }

            if event.has_value("cisco_duo.activity.access_device.as.asn") {
                event.rename(
                    "cisco_duo.activity.access_device.as.asn",
                    "cisco_duo.activity.access_device.as.number",
                )?;
            }

            if event.has_value("cisco_duo.activity.access_device.as.organization_name") {
                event.rename(
                    "cisco_duo.activity.access_device.as.organization_name",
                    "cisco_duo.activity.access_device.as.organization.name",
                )?;
            }

            if event.has_value("json.action") {
                event.rename("json.action", "cisco_duo.activity.action")?;
            }

            if event.has_value("json.activity_id") {
                event.rename("json.activity_id", "cisco_duo.activity.id")?;
            }

            if event.has_value("json.actor") {
                event.rename("json.actor", "cisco_duo.activity.actor")?;
            }

            let _cond = {
                event
                    .get("cisco_duo.activity.actor.details")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                parse_json_field(
                    event,
                    "cisco_duo.activity.actor.details",
                    "cisco_duo.activity.actor.details",
                )?;
            }

            if event.has_value("json.akey") {
                event.rename("json.akey", "cisco_duo.activity.akey")?;
            }

            if event.has_value("json.application") {
                event.rename("json.application", "cisco_duo.activity.application")?;
            }

            if event.has_value("json.old_target") {
                event.rename("json.old_target", "cisco_duo.activity.old_target")?;
            }

            let _cond = {
                event
                    .get("cisco_duo.activity.old_target.details")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                parse_json_field(
                    event,
                    "cisco_duo.activity.old_target.details",
                    "cisco_duo.activity.old_target.details",
                )?;
            }

            let _cond = { event.has_value("json.outcome.result") };
            if _cond {
                if event.has_value("json.outcome.result") {
                    event.rename("json.outcome.result", "cisco_duo.activity.outcome")?;
                }
            }

            if event.has_value("json.target") {
                event.rename("json.target", "cisco_duo.activity.target")?;
            }

            let _cond = {
                event
                    .get("cisco_duo.activity.target.details")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                parse_json_field(
                    event,
                    "cisco_duo.activity.target.details",
                    "cisco_duo.activity.target.details",
                )?;
            }

            if let Some(v) = event
                .get("cisco_duo.activity.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("cisco_duo.activity.action.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            let _cond = { event.get_str("cisco_duo.activity.outcome") == Some("SUCCESS") };
            if _cond {
                let v = json!("success");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
            }

            let _cond = { event.get_str("cisco_duo.activity.outcome") == Some("FAILURE") };
            if _cond {
                let v = json!("failure");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
            }

            if let Some(v) = event
                .get("cisco_duo.activity.access_device.ip.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event
                .get("cisco_duo.activity.access_device.geo")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo", v)?;
            }

            if let Some(v) = event
                .get("cisco_duo.activity.access_device.as")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.as", v)?;
            }

            let _cond = { event.get_str("cisco_duo.activity.actor.type") == Some("user") };
            if _cond {
                if let Some(v) = event
                    .get("cisco_duo.activity.actor.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            if let Some(v) = event
                .get("cisco_duo.activity.access_device.browser")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.name", v)?;
            }

            if let Some(v) = event
                .get("cisco_duo.activity.access_device.browser_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.version", v)?;
            }

            if let Some(v) = event
                .get("cisco_duo.activity.access_device.os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.os.name", v)?;
            }

            if let Some(v) = event
                .get("cisco_duo.activity.access_device.os_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.os.version", v)?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script, resolved to its runners at generation time
                // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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
                Ok(())
            })();

            event.remove("json");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
