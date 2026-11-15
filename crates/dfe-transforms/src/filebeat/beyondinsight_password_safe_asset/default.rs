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
            event.set("ecs.version", json!("9.1.0"))?;

            let _cond = { event.has_value("error.message") && !event.has_value("event.original") };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "event.original".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            parse_json_field(event, "event.original", "beyondinsight_password_safe.asset")?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "0dec42a426179d72b225632fd9bf5a7c0615a6c1c9fb3ae4ca92f0609af918e6",
                    r#"{"params":"SentinelRemoval","known":["SentinelRemovalLiteral"]}"#,
                    r#"ctx.beyondinsight_password_safe.asset.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);\n"#
                ),
            )?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "efe9461e087ab80eeea29cf043d9b6f9b527feadffea422b13f4fe5df1217964",
                    r#"{"params":{"StringifyNamedFields":{"container":"beyondinsight_password_safe.asset","table":"numeric_ids"}},"known":[]}"#
                ),
                cached_params!("{\"numeric_ids\":[\"AssetID\",\"WorkgroupID\"]}"),
            )?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "1dba6ddfd36e1995dae4e64d7e9c72758869cbcc00c0d2d7ab3e117035b2f1a0",
                    r#"{"params":{"SelectRenameKeys":{"subject":"beyondinsight_password_safe.asset","table":"field_mappings","keep_unmapped":true,"wrap_in_list":false}},"known":[]}"#
                ),
                cached_params!(
                    "{\"field_mappings\":{\"AssetID\":\"asset_id\",\"AssetName\":\"asset_name\",\"AssetType\":\"asset_type\",\"CreateDate\":\"create_date\",\"DnsName\":\"dns_name\",\"DomainName\":\"domain_name\",\"IPAddress\":\"ip_address\",\"LastUpdateDate\":\"last_update_date\",\"MacAddress\":\"mac_address\",\"OperatingSystem\":\"operating_system\",\"WorkgroupID\":\"workgroup_id\"}}"
                ),
            )?;

            let _cond = { event.has_value("beyondinsight_password_safe.asset.last_update_date") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("beyondinsight_password_safe.asset.last_update_date")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondinsight_password_safe.asset.last_update_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.set("event.kind", json!("asset"))?;

            event.append("event.category", json!("host"))?;

            event.append("event.type", json!("info"))?;

            if let Some(v) = event
                .get("beyondinsight_password_safe.asset.asset_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if event.has_value("beyondinsight_password_safe.asset.mac_address") {
                gsub_field(
                    event,
                    "beyondinsight_password_safe.asset.mac_address",
                    "beyondinsight_password_safe.asset.mac_address",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            if event.has_value("beyondinsight_password_safe.asset.mac_address") {
                map_strings(
                    event,
                    "beyondinsight_password_safe.asset.mac_address",
                    "beyondinsight_password_safe.asset.mac_address",
                    str::to_uppercase,
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.asset.mac_address") };
            if _cond {
                event.append(
                    "host.mac",
                    json!(
                        event
                            .get("beyondinsight_password_safe.asset.mac_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.asset.operating_system") };
            if _cond {
                if let Some(v) = event
                    .get("beyondinsight_password_safe.asset.operating_system")
                    .cloned()
                {
                    event.set("host.os.full", v)?;
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.asset.domain_name") };
            if _cond {
                if let Some(v) = event
                    .get("beyondinsight_password_safe.asset.domain_name")
                    .cloned()
                {
                    event.set("host.domain", v)?;
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.asset.ip_address") };
            if _cond {
                gsub_field(
                    event,
                    "beyondinsight_password_safe.asset.ip_address",
                    "beyondinsight_password_safe.asset.ip_address",
                    cached_regex!("\\b0+(\\d)"),
                    "$1",
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.asset.ip_address") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("beyondinsight_password_safe.asset.ip_address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondinsight_password_safe.asset.ip_address".into(),
                                message,
                            }
                        })?;
                        event.set("beyondinsight_password_safe.asset.ip_address", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    if event
                        .remove("beyondinsight_password_safe.asset.ip_address")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "beyondinsight_password_safe.asset.ip_address".into(),
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

            let _cond = { event.has_value("beyondinsight_password_safe.asset.ip_address") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("beyondinsight_password_safe.asset.ip_address")
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

            let _cond = { event.has_value("beyondinsight_password_safe.asset.ip_address") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("beyondinsight_password_safe.asset.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.asset.asset_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("beyondinsight_password_safe.asset.asset_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.asset.dns_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("beyondinsight_password_safe.asset.dns_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.asset.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("beyondinsight_password_safe.asset.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
