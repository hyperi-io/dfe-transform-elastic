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
            event.rename("message", "jamf_pro.inventory")?;

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
            // Source: Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif (ctx.jamf_pro.inventory != null) {\n  ctx.jamf_pro.inventory = keysToSnakeCase(ctx.jamf_pro.inventory);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif (ctx.jamf_pro.inventory != null) {\n  ctx.jamf_pro.inventory = keysToSnakeCase(ctx.jamf_pro.inventory);\n}\n"#
                ),
            )?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("jamf_pro.inventory.general.report_date") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "jamf_pro.inventory.general.report_date".into(),
                    });
                }
                if let Some(v) = event.get("jamf_pro.inventory.udid") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "jamf_pro.inventory.udid".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("jamf_pro.inventory.general.last_ip_address") {
                if let Some(ip_str) = event.get_string("jamf_pro.inventory.general.last_ip_address")
                {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set(
                                "jamf_pro.inventory.general.last_ip_address_geo.country_iso_code",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set(
                                "jamf_pro.inventory.general.last_ip_address_geo.country_name",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set(
                                "jamf_pro.inventory.general.last_ip_address_geo.continent_name",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set(
                                "jamf_pro.inventory.general.last_ip_address_geo.region_iso_code",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set(
                                "jamf_pro.inventory.general.last_ip_address_geo.region_name",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set(
                                "jamf_pro.inventory.general.last_ip_address_geo.city_name",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set(
                                "jamf_pro.inventory.general.last_ip_address_geo.timezone",
                                v.clone(),
                            )?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set(
                                "jamf_pro.inventory.general.last_ip_address_geo.location",
                                v.clone(),
                            )?;
                        }
                    }
                }
            }

            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = {
                event.has_value("jamf_pro.inventory.general")
                    && event.get_str("jamf_pro.inventory.general.last_ip_address") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append(
                        "host.ip",
                        json!(
                            event
                                .get("jamf_pro.inventory.general.last_ip_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if let Some(v) = event
                .get("jamf_pro.inventory.udid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("jamf_pro.inventory.general.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if let Some(v) = event
                .get("host.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.address", v)?;
            }

            let _cond = {
                event.has_value("jamf_pro.inventory.hardware")
                    && event.get_str("jamf_pro.inventory.hardware.mac_address") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append(
                        "host.mac",
                        json!(
                            event
                                .get("jamf_pro.inventory.hardware.mac_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("host.mac") {
                    gsub_field(event, "host.mac", "host.mac", cached_regex!("[:.]"), "-")?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_host_mac")?;
                event.remove("host.mac");
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

            let _cond = { event.get_str("host.mac") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("host.mac") {
                        map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uppercase")?;
                    event.set("_ingest.on_failure_processor_tag", "uppercase_host_mac")?;
                    event.remove("host.mac");
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
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("jamf_pro.inventory.general.last_ip_address_geo")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.geo", v)?;
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("jamf_pro.inventory.general.remote_management.managed")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.entity.attributes.managed", v)?;
            }

            if let Some(v) = event
                .get("jamf_pro.inventory.general.last_contact_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.entity.lifecycle.last_activity", v)?;
            }

            if let Some(v) = event
                .get("jamf_pro.inventory.operating_system.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            let _cond = {
                event.has_value("jamf_pro.inventory.operating_system.version")
                    && event.get_str("jamf_pro.inventory.operating_system.version") != Some("")
            };
            if _cond {
                // Painless script
                // Source: String normalize(String s) {\n  int n = 0;\n  for (int i = 0; i < s.length(); i++){\n      char c = s.charAt(i);\n      if (c == (char)'.') {\n        n++;\n        continue;\n      }\n      if (c < (char)'0' || (char)'9' < c) {\n        // If we have non-numeric parts, bail.\n        return s;\n      }\n  }\n  if (n >= 2) {\n      return s;\n  }\n  if (n == 1) {\n      return s + \".0\";\n  }\n  return s + \".0.0\";\n}\nString full_name(String s) {\n  if (s.startsWith('15.')) {\n    return 'sequoia';\n  }\n  if (s.startsWith('14.')) {\n    return 'sonoma';\n  }\n  if (s.startsWith('13.')) {\n    return 'ventura';\n  }\n  if (s.startsWith('12.')) {\n    return 'monterey';\n  }\n  if (s.startsWith('11.')) {\n    return 'big sur';\n  }\n  if (s.startsWith('10.15.')) {\n    return 'catalina';\n  }\n  if (s.startsWith('10.14.')) {\n    return 'mojave';\n  }\n  if (s.startsWith('10.13.')) {\n    return 'high sierra';\n  }\n  if (s.startsWith('10.12.')) {\n    return 'sierra';\n  }\n  if (s.startsWith('10.11.')) {\n    return 'el capitan';\n  }\n  if (s.startsWith('10.10.')) {\n    return 'yosemite';\n  }\n  if (s.startsWith('10.9.')) {\n    return 'mavericks';\n  }\n  return '';\n}\nctx.jamf_pro.inventory.operating_system.version = normalize(ctx.jamf_pro.inventory.operating_system.version);\nString name = full_name(ctx.jamf_pro.inventory.operating_system.version);\nif (name != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.os = ctx.host.os ?: [:];\n  ctx.host.os.full = name;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String normalize(String s) {\n  int n = 0;\n  for (int i = 0; i < s.length(); i++){\n      char c = s.charAt(i);\n      if (c == (char)'.') {\n        n++;\n        continue;\n      }\n      if (c < (char)'0' || (char)'9' < c) {\n        // If we have non-numeric parts, bail.\n        return s;\n      }\n  }\n  if (n >= 2) {\n      return s;\n  }\n  if (n == 1) {\n      return s + \".0\";\n  }\n  return s + \".0.0\";\n}\nString full_name(String s) {\n  if (s.startsWith('15.')) {\n    return 'sequoia';\n  }\n  if (s.startsWith('14.')) {\n    return 'sonoma';\n  }\n  if (s.startsWith('13.')) {\n    return 'ventura';\n  }\n  if (s.startsWith('12.')) {\n    return 'monterey';\n  }\n  if (s.startsWith('11.')) {\n    return 'big sur';\n  }\n  if (s.startsWith('10.15.')) {\n    return 'catalina';\n  }\n  if (s.startsWith('10.14.')) {\n    return 'mojave';\n  }\n  if (s.startsWith('10.13.')) {\n    return 'high sierra';\n  }\n  if (s.startsWith('10.12.')) {\n    return 'sierra';\n  }\n  if (s.startsWith('10.11.')) {\n    return 'el capitan';\n  }\n  if (s.startsWith('10.10.')) {\n    return 'yosemite';\n  }\n  if (s.startsWith('10.9.')) {\n    return 'mavericks';\n  }\n  return '';\n}\nctx.jamf_pro.inventory.operating_system.version = normalize(ctx.jamf_pro.inventory.operating_system.version);\nString name = full_name(ctx.jamf_pro.inventory.operating_system.version);\nif (name != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.os = ctx.host.os ?: [:];\n  ctx.host.os.full = name;\n}"#
                    ),
                )?;
            }

            if let Some(v) = event
                .get("jamf_pro.inventory.operating_system.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if let Some(v) = event
                .get("jamf_pro.inventory.general.platform")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.platform", v)?;
            }

            if let Some(v) = event
                .get("jamf_pro.inventory.user_and_location.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("jamf_pro.inventory.user_and_location.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            if let Some(v) = event
                .get("jamf_pro.inventory.user_and_location.realname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.full_name", v)?;
            }

            let _cond = { event.has_value("jamf_pro.inventory.group_memberships") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("jamf_pro.inventory.group_memberships") {
                        foreach_array(event, "jamf_pro.inventory.group_memberships", |event| {
                            event.append_unique(
                                "user.group.name",
                                json!(
                                    event
                                        .get("_ingest._value.group_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("jamf_pro.inventory.group_memberships") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("jamf_pro.inventory.group_memberships") {
                        foreach_array(event, "jamf_pro.inventory.group_memberships", |event| {
                            event.append_unique(
                                "user.group.id",
                                json!(
                                    event
                                        .get("_ingest._value.group_id")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("user.email") && event.get_str("user.email") != Some("") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("host.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("related.ip", v)?;
            }

            event.set("event.kind", json!("asset"))?;

            let _cond = { event.has_value("jamf_pro.inventory.user_and_location") };
            if _cond {
                event.append_unique("event.type", json!("user"))?;
            }

            let _cond = { event.has_value("jamf_pro.inventory.hardware") };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
            }
        }

        Ok(TransformResult::Continue)
    }
}
