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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("message") {
                        event.rename("message", "event.original")?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "hashicorp_vault.audit")?;
                Ok(())
            })();

            if let Some(date_str) = event.get_as_string("hashicorp_vault.audit.time") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "hashicorp_vault.audit.time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            event.remove("hashicorp_vault.audit.time");

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("authentication"))?;

            let _cond =
                { event.get_str("hashicorp_vault.audit.request.operation") == Some("delete") };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
                event.append_unique("event.type", json!("end"))?;
            }

            let _cond =
                { event.get_str("hashicorp_vault.audit.request.operation") == Some("update") };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = {
                ["read", "list", "create"].contains(
                    &event
                        .get_str("hashicorp_vault.audit.request.operation")
                        .unwrap_or(""),
                )
            };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
                event.append_unique("event.type", json!("start"))?;
            }

            let _cond = { event.has_value("hashicorp_vault.audit.error") };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("hashicorp_vault.audit.error")
                    && event
                        .get("hashicorp_vault.audit.error")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("denied"))
                            }
                            serde_json::Value::String(s) => s.contains("denied"),
                            _ => false,
                        })
            };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            if let Some(v) = event
                .get("hashicorp_vault.audit.request.operation")
                .cloned()
            {
                event.set("event.action", v)?;
            }

            let _cond = { !event.has_value("hashicorp_vault.audit.error") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.has_value("hashicorp_vault.audit.error") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("hashicorp_vault.audit.request.id").cloned() {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("hashicorp_vault.audit.error").cloned() {
                    event.set("message", v)?;
                }
                Ok(())
            })();

            if event.has_value("hashicorp_vault.audit.request.remote_address") {
                if let Some(val) = event.get("hashicorp_vault.audit.request.remote_address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "hashicorp_vault.audit.request.remote_address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            if event.has_value("hashicorp_vault.audit.request.remote_port") {
                if let Some(val) = event.get("hashicorp_vault.audit.request.remote_port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "hashicorp_vault.audit.request.remote_port".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
            }

            // SKIPPED: condition not transpiled: ctx.hashicorp_vault?.audit?.request?.headers?.get('user-agent') != null && !ctx.hashicorp_vault.audit.request.headers['user-agent'].get(0).startsWith('hmac-')
            #[allow(unreachable_code, unused_variables)]
            if false {
                if let Some(v) = event
                    .get("hashicorp_vault.audit.request.headers.user-agent.0")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user_agent.original", v)?;
                }
            }

            let _cond = { event.has_value("user_agent.original") };
            if _cond {
                if event.has_value("user_agent.original") {
                    if let Some(ua_str) = event.get_string("user_agent.original") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.set("user_agent.original", json!(ua_str))?;
                            if let Some(name) = ua.name {
                                event.set("user_agent.name", json!(name))?;
                            }
                            if let Some(version) = ua.version {
                                event.set("user_agent.version", json!(version))?;
                            }
                            if let Some(os_name) = ua.os_name {
                                event.set("user_agent.os.name", json!(os_name))?;
                                if let Some(os_version) = ua.os_version {
                                    event.set("user_agent.os.version", json!(os_version))?;
                                    event.set(
                                        "user_agent.os.full",
                                        json!(format!("{} {}", os_name, os_version)),
                                    )?;
                                }
                            }
                            if let Some(device) = ua.device {
                                event.set("user_agent.device.name", json!(device))?;
                            }
                        }
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("hashicorp_vault.audit.auth.metadata.email")
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("hashicorp_vault.audit.auth.metadata.account_id")
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("hashicorp_vault.audit.auth.metadata.AllocationID")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.NodeID")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.Namespace")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.Task")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("hashicorp_vault.audit.auth.metadata.AllocationID")
                        .cloned()
                    {
                        event.set("nomad.allocation.id", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("hashicorp_vault.audit.auth.metadata.AllocationID")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.NodeID")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.Namespace")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.Task")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("hashicorp_vault.audit.auth.metadata.Namespace")
                        .cloned()
                    {
                        event.set("nomad.namespace", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("hashicorp_vault.audit.auth.metadata.AllocationID")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.NodeID")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.Namespace")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.Task")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("hashicorp_vault.audit.auth.metadata.NodeID")
                        .cloned()
                    {
                        event.set("nomad.node.id", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("hashicorp_vault.audit.auth.metadata.AllocationID")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.NodeID")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.Namespace")
                    && event.has_value("hashicorp_vault.audit.auth.metadata.Task")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("hashicorp_vault.audit.auth.metadata.Task")
                        .cloned()
                    {
                        event.set("nomad.task.name", v)?;
                    }
                    Ok(())
                })();
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
