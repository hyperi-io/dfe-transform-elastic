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

            event.set("ecs.version", json!("8.16.0"))?;

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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
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

            let _cond = { !event.has_value("json.events") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond =
                { event.has_value("json.id.time") && event.get_str("json.id.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.id.time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "UNIX",
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.id.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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
            }

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("google_workspace.id.time", v)?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.events") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.events.name") {
                event.rename("json.events.name", "event.action")?;
            }

            if let Some(v) = event
                .get("event.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("google_workspace.event.name", v)?;
            }

            if event.has_value("json.id.applicationName") {
                event.rename("json.id.applicationName", "event.provider")?;
            }

            if let Some(v) = event
                .get("event.provider")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("google_workspace.id.application_name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.id.uniqueQualifier") {
                    if let Some(val) = event.get("json.id.uniqueQualifier") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.id.uniqueQualifier".into(),
                                message,
                            }
                        })?;
                        event.set("event.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if let Some(v) = event
                .get("event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("google_workspace.id.unique_qualifier", v)?;
            }

            if event.has_value("json.actor.email") {
                event.rename("json.actor.email", "source.user.email")?;
            }

            if let Some(v) = event
                .get("source.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("google_workspace.actor.email", v)?;
            }

            if let Some(v) = event
                .get("source.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.actor.profileId") {
                    if let Some(val) = event.get("json.actor.profileId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.actor.profileId".into(),
                                message,
                            }
                        })?;
                        event.set("source.user.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if let Some(v) = event
                .get("source.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("google_workspace.actor.profile.id", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ipAddress") {
                    if let Some(val) = event.get("json.ipAddress") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ipAddress".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if let Some(v) = event
                .get("source.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("google_workspace.ip_address", v)?;
            }

            if event.has_value("json.kind") {
                event.rename("json.kind", "google_workspace.kind")?;
            }

            if event.has_value("json.etag") {
                event.rename("json.etag", "google_workspace.etag")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.id.customerId") {
                    if let Some(val) = event.get("json.id.customerId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.id.customerId".into(),
                                message,
                            }
                        })?;
                        event.set("organization.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if let Some(v) = event
                .get("organization.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("google_workspace.id.customer.id", v)?;
            }

            if event.has_value("json.actor.callerType") {
                event.rename("json.actor.callerType", "google_workspace.actor.type")?;
            }

            if event.has_value("json.actor.key") {
                event.rename("json.actor.key", "google_workspace.actor.key")?;
            }

            if event.has_value("json.ownerDomain") {
                event.rename("json.ownerDomain", "google_workspace.organization.domain")?;
            }

            if event.has_value("json.events.type") {
                event.rename("json.events.type", "google_workspace.event.type")?;
            }

            if let Some(v) = event
                .get("source.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = {
                event.has_value("source.user.email")
                    && event.get("source.user.email").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.source.user.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} ctx.user.name = splitmail[0]; ctx.source.user.name = splitmail[0]; ctx.user.domain = splitmail[1]; ctx.source.user.domain = splitmail[1];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String[] splitmail = ctx.source.user.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} ctx.user.name = splitmail[0]; ctx.source.user.name = splitmail[0]; ctx.user.domain = splitmail[1]; ctx.source.user.domain = splitmail[1];\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("user.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("user.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_workspace.organization.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_workspace.organization.domain")
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

            let _cond = { event.has_value("source.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.id")
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

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.events.parameters")
                    && event
                        .get("json.events.parameters")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def context_aware_access = ctx.google_workspace.context_aware_access; if (context_aware_access == null) {\n  context_aware_access = new HashMap();\n}\ndef fields = new String[] {\n  \"value\",\n  \"multiValue\",\n  \"messageValue\",\n  \"multiMessageValue\",\n  \"intValue\",\n  \"multiIntValue\",\n  \"boolValue\",\n  \"multiBoolValue\"\n};\ndef parameters = ctx.json.events.parameters; for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"messageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"messageValue\"][\"parameter\"].length; ++j ){\n      for (def f: fields) {\n        if (parameters[i][\"messageValue\"][\"parameter\"][j][f] != null) {\n          parameters[i].messageValue[parameters[i][\"messageValue\"][\"parameter\"][j][\"name\"]] = parameters[i][\"messageValue\"][\"parameter\"][j][f];\n        }\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"messageValue\"] != null) {\n    parameters[i][\"messageValue\"].remove('parameter');\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"multiMessageValue\"].length; ++j ){\n      if (parameters[i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        for (int k = 0;k < parameters[i][\"multiMessageValue\"][j][\"parameter\"].length; ++k ){\n          for (def f: fields) {\n            if (parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][f] != null) {\n              parameters[i].multiMessageValue[j][parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][f];\n            }\n          }\n        }\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"multiMessageValue\"].length; ++j ){\n      if (parameters[i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        parameters[i][\"multiMessageValue\"][j].remove('parameter');\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  for (def f: fields) {\n    if (parameters[i][f] != null) {\n      context_aware_access[parameters[i][\"name\"]] = parameters[i][f];\n    }\n  }\n  ctx.google_workspace.context_aware_access = context_aware_access;\n  ctx.json.events.parameters = parameters;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def context_aware_access = ctx.google_workspace.context_aware_access; if (context_aware_access == null) {\n  context_aware_access = new HashMap();\n}\ndef fields = new String[] {\n  \"value\",\n  \"multiValue\",\n  \"messageValue\",\n  \"multiMessageValue\",\n  \"intValue\",\n  \"multiIntValue\",\n  \"boolValue\",\n  \"multiBoolValue\"\n};\ndef parameters = ctx.json.events.parameters; for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"messageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"messageValue\"][\"parameter\"].length; ++j ){\n      for (def f: fields) {\n        if (parameters[i][\"messageValue\"][\"parameter\"][j][f] != null) {\n          parameters[i].messageValue[parameters[i][\"messageValue\"][\"parameter\"][j][\"name\"]] = parameters[i][\"messageValue\"][\"parameter\"][j][f];\n        }\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"messageValue\"] != null) {\n    parameters[i][\"messageValue\"].remove('parameter');\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"multiMessageValue\"].length; ++j ){\n      if (parameters[i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        for (int k = 0;k < parameters[i][\"multiMessageValue\"][j][\"parameter\"].length; ++k ){\n          for (def f: fields) {\n            if (parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][f] != null) {\n              parameters[i].multiMessageValue[j][parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][f];\n            }\n          }\n        }\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"multiMessageValue\"].length; ++j ){\n      if (parameters[i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        parameters[i][\"multiMessageValue\"][j].remove('parameter');\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  for (def f: fields) {\n    if (parameters[i][f] != null) {\n      context_aware_access[parameters[i][\"name\"]] = parameters[i][f];\n    }\n  }\n  ctx.google_workspace.context_aware_access = context_aware_access;\n  ctx.json.events.parameters = parameters;\n}\n"#
                    ),
                )?;
            }

            if event.has_value("google_workspace.context_aware_access.CAA_ACCESS_LEVEL_APPLIED") {
                event.rename(
                    "google_workspace.context_aware_access.CAA_ACCESS_LEVEL_APPLIED",
                    "google_workspace.context_aware_access.access_level.applied",
                )?;
            }

            if event.has_value("google_workspace.context_aware_access.CAA_ACCESS_LEVEL_SATISFIED") {
                event.rename(
                    "google_workspace.context_aware_access.CAA_ACCESS_LEVEL_SATISFIED",
                    "google_workspace.context_aware_access.access_level.satisfied",
                )?;
            }

            if event.has_value("google_workspace.context_aware_access.CAA_ACCESS_LEVEL_UNSATISFIED")
            {
                event.rename(
                    "google_workspace.context_aware_access.CAA_ACCESS_LEVEL_UNSATISFIED",
                    "google_workspace.context_aware_access.access_level.unsatisfied",
                )?;
            }

            if event.has_value("google_workspace.context_aware_access.CAA_APPLICATION") {
                event.rename(
                    "google_workspace.context_aware_access.CAA_APPLICATION",
                    "google_workspace.context_aware_access.application",
                )?;
            }

            if event.has_value("google_workspace.context_aware_access.CAA_DEVICE_ID") {
                event.rename(
                    "google_workspace.context_aware_access.CAA_DEVICE_ID",
                    "google_workspace.context_aware_access.device.id",
                )?;
            }

            if event.has_value("google_workspace.context_aware_access.CAA_DEVICE_STATE") {
                event.rename(
                    "google_workspace.context_aware_access.CAA_DEVICE_STATE",
                    "google_workspace.context_aware_access.device.state",
                )?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("event.action")
                    && [
                        "ACCESS_DENY_EVENT",
                        "ACCESS_DENY_INTERNAL_ERROR_EVENT",
                        "MONITOR_MODE_ACCESS_DENY_EVENT",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && [
                        "ACCESS_DENY_EVENT",
                        "ACCESS_DENY_INTERNAL_ERROR_EVENT",
                        "MONITOR_MODE_ACCESS_DENY_EVENT",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && [
                        "ACCESS_DENY_EVENT",
                        "ACCESS_DENY_INTERNAL_ERROR_EVENT",
                        "MONITOR_MODE_ACCESS_DENY_EVENT",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            event.remove("json");

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
                event.remove("google_workspace.ip_address");
                event.remove("google_workspace.event.name");
                event.remove("google_workspace.id.unique_qualifier");
                event.remove("google_workspace.id.application_name");
                event.remove("google_workspace.id.customer.id");
                event.remove("google_workspace.actor.profile.id");
                event.remove("google_workspace.actor.email");
                event.remove("google_workspace.id.time");
            }

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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
