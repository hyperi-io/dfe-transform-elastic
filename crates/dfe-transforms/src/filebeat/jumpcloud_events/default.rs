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

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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

            let _cond =
                { !event.has_value("json") || !(event.get("json").is_some_and(|v| v.is_object())) };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("missing json object in input document").to_string(),
                });
            }

            event.rename("json", "jumpcloud.event")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("jumpcloud.event.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("jumpcloud.event.timestamp") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "_id",
                        json!(
                            fingerprint_with(&values, "MurmurHash3", "").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_id".into(),
                                    message,
                                }
                            })?
                        ),
                    )?;
                }
            }

            let _cond = {
                event.has_value("jumpcloud.event.timestamp")
                    && event.get_str("jumpcloud.event.timestamp") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("jumpcloud.event.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "jumpcloud.event.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("jumpcloud.event.client_ip") {
                if let Some(val) = event.get("jumpcloud.event.client_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "jumpcloud.event.client_ip".into(),
                            message,
                        })?;
                    event.set("client.ip", converted)?;
                }
            }

            if event.has_value("jumpcloud.event.src_ip") {
                if let Some(val) = event.get("jumpcloud.event.src_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "jumpcloud.event.src_ip".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            let _cond = {
                event.has_value("jumpcloud.event.event_type")
                    && [
                        "ldap_bind",
                        "user_login_attempt",
                        "sso_auth",
                        "admin_login_attempt",
                    ]
                    .contains(&event.get_str("jumpcloud.event.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.has_value("jumpcloud.event.event_type")
                    && [
                        "ldap_bind",
                        "user_login_attempt",
                        "sso_auth",
                        "admin_login_attempt",
                    ]
                    .contains(&event.get_str("jumpcloud.event.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("jumpcloud.event.event_type")
                    && [
                        "user_password_warning_email",
                        "user_password_reset_request",
                        "user_activation_email",
                        "user_unlocked",
                    ]
                    .contains(&event.get_str("jumpcloud.event.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.category", json!("iam"))?;
            }

            let _cond = {
                event.has_value("jumpcloud.event.event_type")
                    && [
                        "user_password_warning_email",
                        "user_password_reset_request",
                        "user_activation_email",
                        "user_unlocked",
                    ]
                    .contains(&event.get_str("jumpcloud.event.event_type").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.message")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has_value("jumpcloud.event.version") {
                event.rename("jumpcloud.event.version", "jumpcloud.event.payload_version")?;
            }

            if event.has_value("jumpcloud.event.payload_version") {
                if let Some(val) = event.get("jumpcloud.event.payload_version") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "jumpcloud.event.payload_version".into(),
                            message,
                        }
                    })?;
                    event.set("jumpcloud.event.payload_version", converted)?;
                }
            }

            if event.has_value("jumpcloud.event.@version") {
                event.rename("jumpcloud.event.@version", "jumpcloud.event.version")?;
            }

            if event.has_value("jumpcloud.event.version") {
                if let Some(val) = event.get("jumpcloud.event.version") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "jumpcloud.event.version".into(),
                            message,
                        }
                    })?;
                    event.set("jumpcloud.event.version", converted)?;
                }
            }

            if let Some(v) = event
                .get("jumpcloud.event.system.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.system.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.initiated_by.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.id", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.initiated_by.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.email", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.initiated_by.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.name", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.useragent.device")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.device.name", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.useragent.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.name", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.useragent.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.version", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.useragent.os_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.os.name", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.useragent.os_full")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.os.full", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.useragent.os_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.os.version", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.event_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.service")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.module", v)?;
            }

            if let Some(v) = event
                .get("jumpcloud.event.process_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            event.set("event.outcome", json!("unknown"))?;

            let _cond = { event.get_bool("jumpcloud.event.sso_token_success") == Some(true) };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_bool("jumpcloud.event.sso_token_success") == Some(false) };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_bool("jumpcloud.event.success") == Some(true) };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_bool("jumpcloud.event.success") == Some(false) };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("source.geo") && event.has_value("source.ip") };
            if _cond {
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
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = { !event.has_value("client.geo") && event.has_value("client.ip") };
            if _cond {
                if event.has_value("client.ip") {
                    if let Some(ip_str) = event.get_string("client.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("client.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("client.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("client.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("client.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("client.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("client.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("client.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("client.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                if event.has_value("client.ip") {
                    if let Some(ip_str) = event.get_string("client.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("client.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("client.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            if event.has_value("client.as.asn") {
                event.rename("client.as.asn", "client.as.number")?;
            }

            if event.has_value("client.as.organization_name") {
                event.rename("client.as.organization_name", "client.as.organization.name")?;
            }

            let _cond = { event.has_value("jumpcloud.event.jumpcloud_protect_device.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("jumpcloud.event.jumpcloud_protect_device.ip")
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

            let _cond = { event.has_value("client.ip") };
            if _cond {
                event.append_unique(
                    "client.ip",
                    json!(
                        event
                            .get("client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("server.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("server.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("_tmp");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("jumpcloud.event.timestamp");
                event.remove("jumpcloud.event.client_ip");
                event.remove("jumpcloud.event.src_ip");
                event.remove("jumpcloud.event.id");
                event.remove("jumpcloud.event.message");
                event.remove("jumpcloud.event.system.id");
                event.remove("jumpcloud.event.system.hostname");
                event.remove("jumpcloud.event.initiated_by.id");
                event.remove("jumpcloud.event.initiated_by.email");
                event.remove("jumpcloud.event.initiated_by.username");
                event.remove("jumpcloud.event.username");
                event.remove("jumpcloud.event.useragent");
                event.remove("jumpcloud.event.event_type");
                event.remove("jumpcloud.event.service");
                event.remove("jumpcloud.event.process_name");
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
