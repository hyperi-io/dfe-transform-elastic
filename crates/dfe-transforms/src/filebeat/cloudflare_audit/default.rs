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

            parse_json_field(event, "event.original", "json")?;

            let _cond = {
                event.has_value("json.result")
                    && event.get("json.result").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("cloud.provider", json!("cloudflare"))?;

            if let Some(v) = event
                .get("_config.account_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            if let Some(date_str) = event.get_as_string("json.when") {
                match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.when".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.has_value("json.action.type") {
                event.rename("json.action.type", "event.action")?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_bool("json.action.result") == Some(true) };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { !(event.get_bool("json.action.result") == Some(true)) };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if event.has_value("json.actor.email") {
                event.rename("json.actor.email", "user.email")?;
            }

            if event.has_value("json.actor.id") {
                event.rename("json.actor.id", "user.id")?;
            }

            if event.has_value("json.actor.ip") {
                event.rename("json.actor.ip", "source.address")?;
            }

            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("json.actor.type") {
                event.rename("json.actor.type", "cloudflare.audit.actor.type")?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "event.id")?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.get_str("json.interface") != Some("") };
            if _cond {
                if event.has_value("json.interface") {
                    event.rename("json.interface", "event.provider")?;
                }
            }

            if event.has_value("json.metadata") {
                event.rename("json.metadata", "cloudflare.audit.metadata")?;
            }

            if event.has_value("json.newValueJson") {
                event.rename("json.newValueJson", "cloudflare.audit.new_value")?;
            }

            if event.has_value("json.oldValueJson") {
                event.rename("json.oldValueJson", "cloudflare.audit.old_value")?;
            }

            let _cond = { event.get_str("json.newValue") != Some("null") };
            if _cond {
                if event.has_value("json.newValue") {
                    event.rename("json.newValue", "cloudflare.audit.new_value.value")?;
                }
            }

            let _cond = { event.get_str("json.oldValue") != Some("null") };
            if _cond {
                if event.has_value("json.oldValue") {
                    event.rename("json.oldValue", "cloudflare.audit.old_value.value")?;
                }
            }

            if event.has_value("json.owner.id") {
                event.rename("json.owner.id", "cloudflare.audit.owner.id")?;
            }

            if event.has_value("json.resource") {
                event.rename("json.resource", "cloudflare.audit.resource")?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cloudflare.audit.resource.id")
                    && event.get_str("cloudflare.audit.resource.type") == Some("user")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare.audit.resource.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Painless script
            // Source: ctx.event.kind = 'event'; ctx.event.type = 'info'; if (ctx?.event?.action == null) {\n    return;\n} if (params.get(ctx.event.action) == null) {\n    return;\n} def hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"ctx.event.kind = 'event'; ctx.event.type = 'info'; if (ctx?.event?.action == null) {\n    return;\n} if (params.get(ctx.event.action) == null) {\n    return;\n} def hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"login\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"token_create\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"token_revoke\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"token_roll\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"api_key_view\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"rotate_api_key\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"api_key_created\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"purge\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"tls_settings_deployed\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"add\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"delete\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"rec_add\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"rec_del\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"pending\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"change_setting\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"add_enforce_twofactor\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"info\"]}}"
                ),
            )?;

            event.remove("json");
            event.remove("_config");

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n"#
                ),
            )?;

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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
