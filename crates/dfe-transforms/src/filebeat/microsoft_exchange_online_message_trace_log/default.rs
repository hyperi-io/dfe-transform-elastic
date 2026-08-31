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

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            event.set("ecs.version", json!("8.11.0"))?;

            parse_json_field(event, "event.original", "microsoft.online_message_trace")?;

            if event.has_value("microsoft.online_message_trace.fromIP") {
                event.rename(
                    "microsoft.online_message_trace.fromIP",
                    "microsoft.online_message_trace.FromIP",
                )?;
            }

            if event.has_value("microsoft.online_message_trace.messageId") {
                event.rename(
                    "microsoft.online_message_trace.messageId",
                    "microsoft.online_message_trace.MessageId",
                )?;
            }

            if event.has_value("microsoft.online_message_trace.id") {
                event.rename(
                    "microsoft.online_message_trace.id",
                    "microsoft.online_message_trace.MessageTraceId",
                )?;
            }

            if event.has_value("microsoft.online_message_trace.receivedDateTime") {
                event.rename(
                    "microsoft.online_message_trace.receivedDateTime",
                    "microsoft.online_message_trace.Received",
                )?;
            }

            if event.has_value("microsoft.online_message_trace.recipientAddress") {
                event.rename(
                    "microsoft.online_message_trace.recipientAddress",
                    "microsoft.online_message_trace.RecipientAddress",
                )?;
            }

            if event.has_value("microsoft.online_message_trace.senderAddress") {
                event.rename(
                    "microsoft.online_message_trace.senderAddress",
                    "microsoft.online_message_trace.SenderAddress",
                )?;
            }

            if event.has_value("microsoft.online_message_trace.size") {
                event.rename(
                    "microsoft.online_message_trace.size",
                    "microsoft.online_message_trace.Size",
                )?;
            }

            if event.has_value("microsoft.online_message_trace.status") {
                event.rename(
                    "microsoft.online_message_trace.status",
                    "microsoft.online_message_trace.Status",
                )?;
            }

            if event.has_value("microsoft.online_message_trace.subject") {
                event.rename(
                    "microsoft.online_message_trace.subject",
                    "microsoft.online_message_trace.Subject",
                )?;
            }

            if event.has_value("microsoft.online_message_trace.toIP") {
                event.rename(
                    "microsoft.online_message_trace.toIP",
                    "microsoft.online_message_trace.ToIP",
                )?;
            }

            let _cond = {
                event.get("microsoft.online_message_trace.value").is_some_and(|v| v.is_array()) && event.get("microsoft.online_message_trace.value").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            // SKIPPED: condition not transpiled: ctx.microsoft?.online_message_trace?.Status?.equalsIgnoreCase(ctx._conf?.drop_status) == true
            #[allow(unreachable_code, unused_variables)]
            if false {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event
                    .get_str("microsoft.online_message_trace.Status")
                    .is_some_and(|s| s.eq_ignore_ascii_case("delivered"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event
                    .get_str("microsoft.online_message_trace.Status")
                    .is_some_and(|s| s.eq_ignore_ascii_case("failed"))
                    || event
                        .get_str("microsoft.online_message_trace.Status")
                        .is_some_and(|s| s.eq_ignore_ascii_case("quarantined"))
                    || event
                        .get_str("microsoft.online_message_trace.Status")
                        .is_some_and(|s| s.eq_ignore_ascii_case("filteredAsSpam"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            event.set("event.category", Value::Array(vec![json!("email")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond =
                { event.get_str("microsoft.online_message_trace.SenderAddress") != Some("<>") };
            if _cond {
                if let Some(v) = event
                    .get("microsoft.online_message_trace.SenderAddress")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("_temp_.email.from.address", v)?;
                }
            }

            let _cond = { event.has_value("_temp_.email.from.address") };
            if _cond {
                event.append(
                    "email.from.address",
                    json!(
                        event
                            .get("_temp_.email.from.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("_temp_.email.from.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.id", v)?;
            }

            if let Some(v) = event
                .get("_temp_.email.from.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.email", v)?;
            }

            if let Some(v) = event
                .get("microsoft.online_message_trace.MessageTraceId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.local_id", v)?;
            }

            if let Some(v) = event
                .get("microsoft.online_message_trace.ToIP")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event
                .get("microsoft.online_message_trace.Size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.attachments.file.size", v)?;
            }

            if let Some(v) = event
                .get("microsoft.online_message_trace.Received")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.delivery_timestamp", v)?;
            }

            if let Some(v) = event
                .get("microsoft.online_message_trace.Subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            if let Some(v) = event
                .get("microsoft.online_message_trace.RecipientAddress")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("_temp_.email.to.address", v)?;
            }

            let _cond = { event.has_value("_temp_.email.to.address") };
            if _cond {
                event.append(
                    "email.to.address",
                    json!(
                        event
                            .get("_temp_.email.to.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("microsoft.online_message_trace.RecipientAddress")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.id", v)?;
            }

            if let Some(v) = event
                .get("microsoft.online_message_trace.RecipientAddress")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.email", v)?;
            }

            if let Some(v) = event
                .get("microsoft.online_message_trace.FromIP")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event
                .get("microsoft.online_message_trace.MessageId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.message_id", v)?;
            }

            if let Some(date_str) = event.get_as_string("email.delivery_timestamp") {
                match parse_date_out(
                    &date_str,
                    &["yyyy-MM-dd'T'HH:mm:ss.SSSSSSSZ", "ISO8601"],
                    None,
                    None,
                ) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "email.delivery_timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
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

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
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

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            if event.has_value("_temp_.email.from.address") {
                if let Some(input) = event.get_string("_temp_.email.from.address") {
                    // Grok pattern: ^%{DATA}@%{DATA:source.domain}$
                    if !cached_grok!("^%{DATA}@%{DATA:source.domain}$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("_temp_.email.to.address") {
                if let Some(input) = event.get_string("_temp_.email.to.address") {
                    // Grok pattern: ^%{DATA}@%{DATA:destination.domain}$
                    if !cached_grok!("^%{DATA}@%{DATA:destination.domain}$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if let Some(domain_str) = event.get_string("destination.domain") {
                let domain = domain_str.to_string();
                event.set("destination.domain", json!(domain.clone()))?;
                // Public suffix list lookup for registered domain extraction
                if let Some(rd) = registered_domain_lookup(&domain) {
                    if let Some(registered) = rd.registered_domain {
                        event.set("destination.registered_domain", json!(registered))?;
                    }
                    event.set("destination.top_level_domain", json!(rd.top_level_domain))?;
                    if let Some(sub) = rd.subdomain {
                        event.set("destination.subdomain", json!(sub))?;
                    }
                }
            }

            if let Some(domain_str) = event.get_string("source.domain") {
                let domain = domain_str.to_string();
                event.set("source.domain", json!(domain.clone()))?;
                // Public suffix list lookup for registered domain extraction
                if let Some(rd) = registered_domain_lookup(&domain) {
                    if let Some(registered) = rd.registered_domain {
                        event.set("source.registered_domain", json!(registered))?;
                    }
                    event.set("source.top_level_domain", json!(rd.top_level_domain))?;
                    if let Some(sub) = rd.subdomain {
                        event.set("source.subdomain", json!(sub))?;
                    }
                }
            }

            let _cond = { event.has_value("_conf.local_domains") };
            if _cond {
                // Painless script
                // Source: if (!(ctx._conf.local_domains instanceof List)) {ctx._conf.local_domains = [ctx._conf.local_domains]}\ndef destination_internal = 0;\ndef source_internal = 0;\nfor (local_domain in ctx._conf.local_domains) {\n  if (ctx.destination?.domain instanceof String && ctx.destination.domain.equalsIgnoreCase(local_domain)){\n    destination_internal = 1;\n  }\n  if (ctx.source?.domain instanceof String && ctx.source.domain.equalsIgnoreCase(local_domain)){\n    source_internal = 1;\n  }\n}\nif (source_internal == 1 && destination_internal == 1){ctx.email.direction = \"internal\";}\nelse if (source_internal == 0 && destination_internal == 0){ctx.email.direction = \"external\";}\nelse if (source_internal == 1 && destination_internal == 0){ctx.email.direction = \"outbound\";}\nelse if (source_internal == 0 && destination_internal == 1){ctx.email.direction = \"inbound\";}\nelse {ctx.email.direction = \"unknown\";}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (!(ctx._conf.local_domains instanceof List)) {ctx._conf.local_domains = [ctx._conf.local_domains]}\ndef destination_internal = 0;\ndef source_internal = 0;\nfor (local_domain in ctx._conf.local_domains) {\n  if (ctx.destination?.domain instanceof String && ctx.destination.domain.equalsIgnoreCase(local_domain)){\n    destination_internal = 1;\n  }\n  if (ctx.source?.domain instanceof String && ctx.source.domain.equalsIgnoreCase(local_domain)){\n    source_internal = 1;\n  }\n}\nif (source_internal == 1 && destination_internal == 1){ctx.email.direction = \"internal\";}\nelse if (source_internal == 0 && destination_internal == 0){ctx.email.direction = \"external\";}\nelse if (source_internal == 1 && destination_internal == 0){ctx.email.direction = \"outbound\";}\nelse if (source_internal == 0 && destination_internal == 1){ctx.email.direction = \"inbound\";}\nelse {ctx.email.direction = \"unknown\";}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp_.email.to.address")
                    && event.get_str("_temp_.email.to.address") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("_temp_.email.to.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp_.email.from.address")
                    && event.get_str("_temp_.email.from.address") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("_temp_.email.from.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("_temp_.email.to.address") {
                if let Some(input) = event.get_string("_temp_.email.to.address") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("@") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.to_user_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("@") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.to_user_domain", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "_temp_.email.to.address".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                event.has_value("_temp_.to_user_name")
                    && event.get_str("_temp_.to_user_name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("_temp_.to_user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("_temp_.email.from.address") {
                if let Some(input) = event.get_string("_temp_.email.from.address") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("@") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.from_user_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("@") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp_.from_user_domain", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "_temp_.email.from.address".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                event.has_value("_temp_.from_user_name")
                    && event.get_str("_temp_.from_user_name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("_temp_.from_user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp_.to_user_name")
                    && event.get_str("_temp_.to_user_name") != Some("")
                    && event.get_str("email.direction") == Some("inbound")
                    || event.get_str("email.direction") == Some("internal")
            };
            if _cond {
                event.append_unique(
                    "user.name",
                    json!(
                        event
                            .get("_temp_.to_user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp_.from_user_name")
                    && event.get_str("_temp_.from_user_name") != Some("")
                    && event.get_str("email.direction") == Some("outbound")
                    || event.get_str("email.direction") == Some("internal")
            };
            if _cond {
                event.append_unique(
                    "user.name",
                    json!(
                        event
                            .get("_temp_.from_user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp_.email.to.address")
                    && event.get_str("_temp_.email.to.address") != Some("")
                    && event.get_str("email.direction") == Some("inbound")
                    || event.get_str("email.direction") == Some("internal")
            };
            if _cond {
                event.append_unique(
                    "user.email",
                    json!(
                        event
                            .get("_temp_.email.to.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp_.email.from.address")
                    && event.get_str("_temp_.email.from.address") != Some("")
                    && event.get_str("email.direction") == Some("outbound")
                    || event.get_str("email.direction") == Some("internal")
            };
            if _cond {
                event.append_unique(
                    "user.email",
                    json!(
                        event
                            .get("_temp_.email.from.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp_.email.to.address")
                    && event.get_str("_temp_.email.to.address") != Some("")
                    && event.get_str("email.direction") == Some("inbound")
                    || event.get_str("email.direction") == Some("internal")
            };
            if _cond {
                event.append_unique(
                    "user.id",
                    json!(
                        event
                            .get("_temp_.email.to.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_temp_.email.from.address")
                    && event.get_str("_temp_.email.from.address") != Some("")
                    && event.get_str("email.direction") == Some("outbound")
                    || event.get_str("email.direction") == Some("internal")
            };
            if _cond {
                event.append_unique(
                    "user.id",
                    json!(
                        event
                            .get("_temp_.email.from.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Painless script
            // Source: if (ctx.user?.name != null && ctx.user.name.length == 1) {\n  ctx.user.name = ctx.user.name[0];\n}\nif (ctx.user?.email != null && ctx.user.email.length == 1) {\n  ctx.user.email = ctx.user.email[0];\n}\nif (ctx.user?.id != null && ctx.user.id.length == 1) {\n  ctx.user.id = ctx.user.id[0];\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.user?.name != null && ctx.user.name.length == 1) {\n  ctx.user.name = ctx.user.name[0];\n}\nif (ctx.user?.email != null && ctx.user.email.length == 1) {\n  ctx.user.email = ctx.user.email[0];\n}\nif (ctx.user?.id != null && ctx.user.id.length == 1) {\n  ctx.user.id = ctx.user.id[0];\n}\n"#
                ),
            )?;

            let _cond = {
                event.has_value("_temp_.to_user_name")
                    && event.get_str("_temp_.to_user_name") != Some("")
            };
            if _cond {
                if let Some(v) = event.get("_temp_.to_user_name").cloned() {
                    event.set("destination.user.name", v)?;
                }
            }

            let _cond = {
                event.has_value("_temp_.to_user_domain")
                    && event.get_str("_temp_.to_user_domain") != Some("")
            };
            if _cond {
                if let Some(v) = event.get("_temp_.to_user_domain").cloned() {
                    event.set("destination.user.domain", v)?;
                }
            }

            let _cond = {
                event.has_value("_temp_.from_user_name")
                    && event.get_str("_temp_.from_user_name") != Some("")
            };
            if _cond {
                if let Some(v) = event.get("_temp_.from_user_name").cloned() {
                    event.set("source.user.name", v)?;
                }
            }

            let _cond = {
                event.has_value("_temp_.from_user_domain")
                    && event.get_str("_temp_.from_user_domain") != Some("")
            };
            if _cond {
                if let Some(v) = event.get("_temp_.from_user_domain").cloned() {
                    event.set("source.user.domain", v)?;
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("email.attachments.file.size") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("email.from.address") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("email.local_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("email.message_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("email.subject") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("email.to.address") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("microsoft.online_message_trace.Status") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_temp_");
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_conf");
                Ok(())
            })();

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
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
