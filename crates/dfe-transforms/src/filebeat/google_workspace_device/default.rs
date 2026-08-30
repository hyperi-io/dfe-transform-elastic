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

            let _cond = { event.has_value("source.user.email") };
            if _cond {
                event.append_unique(
                    "user.email",
                    json!(
                        event
                            .get("source.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            let _cond = {
                event.has_value("json.events.parameters")
                    && event
                        .get("json.events.parameters")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def device = ctx.google_workspace.device; if (device == null) {\n  device = new HashMap();\n}\ndef fields = new String[] {\n  \"value\",\n  \"multiValue\",\n  \"messageValue\",\n  \"multiMessageValue\",\n  \"intValue\",\n  \"multiIntValue\",\n  \"boolValue\",\n  \"multiBoolValue\"\n};\ndef parameters = ctx.json.events.parameters; for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"messageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"messageValue\"][\"parameter\"].length; ++j ){\n      for (def f: fields) {\n        if (parameters[i][\"messageValue\"][\"parameter\"][j][f] != null) {\n          parameters[i].messageValue[parameters[i][\"messageValue\"][\"parameter\"][j][\"name\"]] = parameters[i][\"messageValue\"][\"parameter\"][j][f];\n        }\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"messageValue\"] != null) {\n    parameters[i][\"messageValue\"].remove('parameter');\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"multiMessageValue\"].length; ++j ){\n      if (parameters[i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        for (int k = 0;k < parameters[i][\"multiMessageValue\"][j][\"parameter\"].length; ++k ){\n          for (def f: fields) {\n            if (parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][f] != null) {\n              parameters[i].multiMessageValue[j][parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][f];\n            }\n          }\n        }\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"multiMessageValue\"].length; ++j ){\n      if (parameters[i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        parameters[i][\"multiMessageValue\"][j].remove('parameter');\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  for (def f: fields) {\n    if (parameters[i][f] != null) {\n      device[parameters[i][\"name\"]] = parameters[i][f];\n    }\n  }\n  ctx.google_workspace.device = device;\n  ctx.json.events.parameters = parameters;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def device = ctx.google_workspace.device; if (device == null) {\n  device = new HashMap();\n}\ndef fields = new String[] {\n  \"value\",\n  \"multiValue\",\n  \"messageValue\",\n  \"multiMessageValue\",\n  \"intValue\",\n  \"multiIntValue\",\n  \"boolValue\",\n  \"multiBoolValue\"\n};\ndef parameters = ctx.json.events.parameters; for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"messageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"messageValue\"][\"parameter\"].length; ++j ){\n      for (def f: fields) {\n        if (parameters[i][\"messageValue\"][\"parameter\"][j][f] != null) {\n          parameters[i].messageValue[parameters[i][\"messageValue\"][\"parameter\"][j][\"name\"]] = parameters[i][\"messageValue\"][\"parameter\"][j][f];\n        }\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"messageValue\"] != null) {\n    parameters[i][\"messageValue\"].remove('parameter');\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"multiMessageValue\"].length; ++j ){\n      if (parameters[i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        for (int k = 0;k < parameters[i][\"multiMessageValue\"][j][\"parameter\"].length; ++k ){\n          for (def f: fields) {\n            if (parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][f] != null) {\n              parameters[i].multiMessageValue[j][parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = parameters[i][\"multiMessageValue\"][j][\"parameter\"][k][f];\n            }\n          }\n        }\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  if (parameters[i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < parameters[i][\"multiMessageValue\"].length; ++j ){\n      if (parameters[i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        parameters[i][\"multiMessageValue\"][j].remove('parameter');\n      }\n    }\n  }\n} for (int i = 0; i < parameters.length; ++i) {\n  for (def f: fields) {\n    if (parameters[i][f] != null) {\n      device[parameters[i][\"name\"]] = parameters[i][f];\n    }\n  }\n  ctx.google_workspace.device = device;\n  ctx.json.events.parameters = parameters;\n}\n"#
                    ),
                )?;
            }

            if event.has_value("google_workspace.device.ACCOUNT_STATE") {
                event.rename(
                    "google_workspace.device.ACCOUNT_STATE",
                    "google_workspace.device.account_state",
                )?;
            }

            if event.has_value("google_workspace.device.ACTION_EXECUTION_STATUS") {
                event.rename(
                    "google_workspace.device.ACTION_EXECUTION_STATUS",
                    "google_workspace.device.action.execution_status",
                )?;
            }

            if event.has_value("google_workspace.device.ACTION_ID") {
                event.rename(
                    "google_workspace.device.ACTION_ID",
                    "google_workspace.device.action.id",
                )?;
            }

            if event.has_value("google_workspace.device.ACTION_TYPE") {
                event.rename(
                    "google_workspace.device.ACTION_TYPE",
                    "google_workspace.device.action.type",
                )?;
            }

            if event.has_value("google_workspace.device.APK_SHA256_HASH") {
                event.rename(
                    "google_workspace.device.APK_SHA256_HASH",
                    "google_workspace.device.apk_sha256_hash",
                )?;
            }

            let _cond = { event.has_value("google_workspace.device.apk_sha256_hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_workspace.device.apk_sha256_hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("google_workspace.device.APPLICATION_ID") {
                event.rename(
                    "google_workspace.device.APPLICATION_ID",
                    "google_workspace.device.application.id",
                )?;
            }

            if event.has_value("google_workspace.device.APPLICATION_MESSAGE") {
                event.rename(
                    "google_workspace.device.APPLICATION_MESSAGE",
                    "google_workspace.device.application.message",
                )?;
            }

            if event.has_value("google_workspace.device.APPLICATION_REPORT_KEY") {
                event.rename(
                    "google_workspace.device.APPLICATION_REPORT_KEY",
                    "google_workspace.device.application.report.key",
                )?;
            }

            let _cond = {
                event.has_value("google_workspace.device.APPLICATION_REPORT_TIMESTAMP")
                    && event.get_str("google_workspace.device.APPLICATION_REPORT_TIMESTAMP")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_workspace.device.APPLICATION_REPORT_TIMESTAMP")
                    {
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
                            Some(parsed) => event.set(
                                "google_workspace.device.application.report.timestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_workspace.device.APPLICATION_REPORT_TIMESTAMP"
                                        .into(),
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

            let _cond = {
                event.has_value("google_workspace.device.LAST_SYNC_AUDIT_DATE")
                    && event.get_str("google_workspace.device.LAST_SYNC_AUDIT_DATE") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_workspace.device.LAST_SYNC_AUDIT_DATE")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "UNIX_MS",
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
                            Some(parsed) => {
                                event.set("google_workspace.device.last_sync_audit_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_workspace.device.LAST_SYNC_AUDIT_DATE".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_last_sync_audit_date",
                    )?;
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

            if event.has_value("google_workspace.device.APPLICATION_REPORT_SEVERITY") {
                event.rename(
                    "google_workspace.device.APPLICATION_REPORT_SEVERITY",
                    "google_workspace.device.application.report.severity",
                )?;
            }

            if event.has_value("google_workspace.device.APPLICATION_STATE") {
                event.rename(
                    "google_workspace.device.APPLICATION_STATE",
                    "google_workspace.device.application.state",
                )?;
            }

            if event.has_value("google_workspace.device.BASIC_INTEGRITY") {
                event.rename(
                    "google_workspace.device.BASIC_INTEGRITY",
                    "google_workspace.device.basic_integrity",
                )?;
            }

            if event.has_value("google_workspace.device.CTS_PROFILE_MATCH") {
                event.rename(
                    "google_workspace.device.CTS_PROFILE_MATCH",
                    "google_workspace.device.cts_profile_match",
                )?;
            }

            if event.has_value("google_workspace.device.DEVICE_COMPLIANCE") {
                event.rename(
                    "google_workspace.device.DEVICE_COMPLIANCE",
                    "google_workspace.device.compliance",
                )?;
            }

            if event.has_value("google_workspace.device.DEVICE_COMPROMISED_STATE") {
                event.rename(
                    "google_workspace.device.DEVICE_COMPROMISED_STATE",
                    "google_workspace.device.compromised_state",
                )?;
            }

            if event.has_value("google_workspace.device.DEVICE_DEACTIVATION_REASON") {
                event.rename(
                    "google_workspace.device.DEVICE_DEACTIVATION_REASON",
                    "google_workspace.device.deactivation_reason",
                )?;
            }

            if event.has_value("google_workspace.device.DEVICE_ID") {
                event.rename(
                    "google_workspace.device.DEVICE_ID",
                    "google_workspace.device.id",
                )?;
            }

            if event.has_value("google_workspace.device.DEVICE_MODEL") {
                event.rename(
                    "google_workspace.device.DEVICE_MODEL",
                    "google_workspace.device.model",
                )?;
            }

            if event.has_value("google_workspace.device.DEVICE_OWNERSHIP") {
                event.rename(
                    "google_workspace.device.DEVICE_OWNERSHIP",
                    "google_workspace.device.ownership",
                )?;
            }

            if event.has_value("google_workspace.device.DEVICE_PROPERTY") {
                event.rename(
                    "google_workspace.device.DEVICE_PROPERTY",
                    "google_workspace.device.property",
                )?;
            }

            if event.has_value("google_workspace.device.DEVICE_SETTING") {
                event.rename(
                    "google_workspace.device.DEVICE_SETTING",
                    "google_workspace.device.setting",
                )?;
            }

            if event.has_value("google_workspace.device.DEVICE_STATUS_ON_APPLE_PORTAL") {
                event.rename(
                    "google_workspace.device.DEVICE_STATUS_ON_APPLE_PORTAL",
                    "google_workspace.device.status_on_apple_portal",
                )?;
            }

            if event.has_value("google_workspace.device.DEVICE_TYPE") {
                event.rename(
                    "google_workspace.device.DEVICE_TYPE",
                    "google_workspace.device.type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.device.FAILED_PASSWD_ATTEMPTS") {
                    if let Some(val) = event.get("google_workspace.device.FAILED_PASSWD_ATTEMPTS") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_workspace.device.FAILED_PASSWD_ATTEMPTS".into(),
                                message,
                            }
                        })?;
                        event.set("google_workspace.device.failed_passwd_attempts", converted)?;
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

            if event.has_value("google_workspace.device.IOS_VENDOR_ID") {
                event.rename(
                    "google_workspace.device.IOS_VENDOR_ID",
                    "google_workspace.device.ios_vendor_id",
                )?;
            }

            if event.has_value("google_workspace.device.NEW_DEVICE_ID") {
                event.rename(
                    "google_workspace.device.NEW_DEVICE_ID",
                    "google_workspace.device.new_device_id",
                )?;
            }

            if event.has_value("google_workspace.device.NEW_VALUE") {
                event.rename(
                    "google_workspace.device.NEW_VALUE",
                    "google_workspace.device.new_value",
                )?;
            }

            if event.has_value("google_workspace.device.OLD_VALUE") {
                event.rename(
                    "google_workspace.device.OLD_VALUE",
                    "google_workspace.device.old_value",
                )?;
            }

            if event.has_value("google_workspace.device.OS_EDITION") {
                event.rename(
                    "google_workspace.device.OS_EDITION",
                    "google_workspace.device.os.edition",
                )?;
            }

            if event.has_value("google_workspace.device.OS_PROPERTY") {
                event.rename(
                    "google_workspace.device.OS_PROPERTY",
                    "google_workspace.device.os.property",
                )?;
            }

            if event.has_value("google_workspace.device.OS_VERSION") {
                event.rename(
                    "google_workspace.device.OS_VERSION",
                    "google_workspace.device.os.version",
                )?;
            }

            if let Some(v) = event
                .get("google_workspace.device.os.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if event.has_value("google_workspace.device.PHA_CATEGORY") {
                event.rename(
                    "google_workspace.device.PHA_CATEGORY",
                    "google_workspace.device.pha_category",
                )?;
            }

            if event.has_value("google_workspace.device.POLICY_NAME") {
                event.rename(
                    "google_workspace.device.POLICY_NAME",
                    "google_workspace.device.policy.name",
                )?;
            }

            if event.has_value("google_workspace.device.POLICY_SYNC_RESULT") {
                event.rename(
                    "google_workspace.device.POLICY_SYNC_RESULT",
                    "google_workspace.device.policy.sync.result",
                )?;
            }

            if event.has_value("google_workspace.device.POLICY_SYNC_TYPE") {
                event.rename(
                    "google_workspace.device.POLICY_SYNC_TYPE",
                    "google_workspace.device.policy.sync.type",
                )?;
            }

            if event.has_value("google_workspace.device.REGISTER_PRIVILEGE") {
                event.rename(
                    "google_workspace.device.REGISTER_PRIVILEGE",
                    "google_workspace.device.register_privilege",
                )?;
            }

            if event.has_value("google_workspace.device.RESOURCE_ID") {
                event.rename(
                    "google_workspace.device.RESOURCE_ID",
                    "google_workspace.device.resource.id",
                )?;
            }

            if event.has_value("google_workspace.device.RISK_SIGNAL") {
                event.rename(
                    "google_workspace.device.RISK_SIGNAL",
                    "google_workspace.device.risk_signal",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.device.SECURITY_EVENT_ID") {
                    if let Some(val) = event.get("google_workspace.device.SECURITY_EVENT_ID") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_workspace.device.SECURITY_EVENT_ID".into(),
                                message,
                            }
                        })?;
                        event.set("google_workspace.device.security.event_id", converted)?;
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

            if event.has_value("google_workspace.device.SECURITY_PATCH_LEVEL") {
                event.rename(
                    "google_workspace.device.SECURITY_PATCH_LEVEL",
                    "google_workspace.device.security.patch_level",
                )?;
            }

            if event.has_value("google_workspace.device.SERIAL_NUMBER") {
                event.rename(
                    "google_workspace.device.SERIAL_NUMBER",
                    "google_workspace.device.serial_number",
                )?;
            }

            if event.has_value("google_workspace.device.USER_EMAIL") {
                event.rename(
                    "google_workspace.device.USER_EMAIL",
                    "google_workspace.device.user_email",
                )?;
            }

            let _cond = { event.has_value("google_workspace.device.user_email") };
            if _cond {
                event.append_unique(
                    "user.email",
                    json!(
                        event
                            .get("google_workspace.device.user_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("google_workspace.device.VALUE") {
                event.rename(
                    "google_workspace.device.VALUE",
                    "google_workspace.device.value",
                )?;
            }

            if event.has_value("google_workspace.device.WINDOWS_SYNCML_POLICY_STATUS_CODE") {
                event.rename(
                    "google_workspace.device.WINDOWS_SYNCML_POLICY_STATUS_CODE",
                    "google_workspace.device.windows_syncml_policy_status_code",
                )?;
            }

            event.append("event.category", json!("host"))?;

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("event.action")
                    && ["DEVICE_COMPROMISED_EVENT", "SUSPICIOUS_ACTIVITY_EVENT"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event.get_str("event.action") == Some("FAILED_PASSWORD_ATTEMPTS_EVENT")
            };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event.get_str("event.action") == Some("SUSPICIOUS_ACTIVITY_EVENT")
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && ["DEVICE_COMPROMISED_EVENT", "SUSPICIOUS_ACTIVITY_EVENT"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("threat"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event.get_str("event.action") == Some("APPLICATION_EVENT")
            };
            if _cond {
                event.append("event.category", json!("package"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && [
                        "APPLICATION_EVENT",
                        "DEVICE_ACTION_EVENT",
                        "DEVICE_COMPLIANCE_CHANGED_EVENT",
                        "OS_UPDATED_EVENT",
                        "DEVICE_OWNERSHIP_CHANGE_EVENT",
                        "DEVICE_SETTINGS_UPDATED_EVENT",
                        "ADVANCED_POLICY_SYNC_EVENT",
                        "RISK_SIGNAL_UPDATED_EVENT",
                        "ANDROID_WORK_PROFILE_SUPPORT_ENABLED_EVENT",
                        "APPLE_DEP_DEVICE_UPDATE_ON_APPLE_PORTAL_EVENT",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && [
                        "DEVICE_SYNC_EVENT",
                        "APPLICATION_REPORT_EVENT",
                        "SUSPICIOUS_ACTIVITY_EVENT",
                        "FAILED_PASSWORD_ATTEMPTS_EVENT",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && ["DEVICE_COMPROMISED_EVENT", "SUSPICIOUS_ACTIVITY_EVENT"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("indicator"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event.get_str("event.action") == Some("DEVICE_REGISTER_UNREGISTER_EVENT")
                    && event.has_value("google_workspace.device.account_state")
                    && event
                        .get_str("google_workspace.device.account_state")
                        .is_some_and(|s| s.eq_ignore_ascii_case("REGISTERED"))
            };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event.get_str("event.action") == Some("DEVICE_REGISTER_UNREGISTER_EVENT")
                    && event.has_value("google_workspace.device.account_state")
                    && !(event
                        .get_str("google_workspace.device.account_state")
                        .is_some_and(|s| s.eq_ignore_ascii_case("REGISTERED")))
            };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event.get_str("event.action") == Some("APPLICATION_EVENT")
                    && event.has_value("google_workspace.device.application.state")
                    && event
                        .get_str("google_workspace.device.application.state")
                        .is_some_and(|s| s.eq_ignore_ascii_case("INSTALLED"))
            };
            if _cond {
                event.append("event.type", json!("installation"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event.get_str("event.action") == Some("APPLICATION_EVENT")
                    && event.has_value("google_workspace.device.application.state")
                    && (event
                        .get_str("google_workspace.device.application.state")
                        .is_some_and(|s| s.eq_ignore_ascii_case("REMOVED"))
                        || event
                            .get_str("google_workspace.device.application.state")
                            .is_some_and(|s| s.eq_ignore_ascii_case("UNINSTALLED")))
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.has_value("google_workspace.device.action.execution_status")
                    && (event
                        .get_str("google_workspace.device.action.execution_status")
                        .is_some_and(|s| s.eq_ignore_ascii_case("SUCCESS"))
                        || event
                            .get_str("google_workspace.device.action.execution_status")
                            .is_some_and(|s| s.eq_ignore_ascii_case("ACTION_SUCCESS")))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("google_workspace.device.policy.sync.result")
                    && event
                        .get_str("google_workspace.device.policy.sync.result")
                        .is_some_and(|s| s.eq_ignore_ascii_case("POLICY_SYNC_SUCCEEDED"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("google_workspace.device.compliance")
                    && event
                        .get_str("google_workspace.device.compliance")
                        .is_some_and(|s| s.eq_ignore_ascii_case("COMPLIANT"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("google_workspace.device.action.execution_status")
                    && (event
                        .get("google_workspace.device.action.execution_status")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("REJECT"))
                            }
                            serde_json::Value::String(s) => s.contains("REJECT"),
                            _ => false,
                        })
                        || event
                            .get("google_workspace.device.action.execution_status")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("FAIL"))
                                }
                                serde_json::Value::String(s) => s.contains("FAIL"),
                                _ => false,
                            }))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("google_workspace.device.policy.sync.result")
                    && (event
                        .get_str("google_workspace.device.policy.sync.result")
                        .is_some_and(|s| s.eq_ignore_ascii_case("POLICY_SYNC_ABORTED"))
                        || event
                            .get_str("google_workspace.device.policy.sync.result")
                            .is_some_and(|s| s.eq_ignore_ascii_case("POLICY_SYNC_FAILED")))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("google_workspace.device.compliance")
                    && event
                        .get_str("google_workspace.device.compliance")
                        .is_some_and(|s| s.eq_ignore_ascii_case("NON_COMPLIANT"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.action") == Some("FAILED_PASSWORD_ATTEMPTS_EVENT")
                    && event.has_value("google_workspace.device.failed_passwd_attempts")
                    && event
                        .get_i64("google_workspace.device.failed_passwd_attempts")
                        .is_some_and(|n| n > 0)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get("user.email").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("user.email") {
                    foreach_array(event, "user.email", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            event.remove("json");
            event.remove("google_workspace.device.APPLICATION_REPORT_TIMESTAMP");
            event.remove("google_workspace.device.FAILED_PASSWD_ATTEMPTS");
            event.remove("google_workspace.device.SECURITY_EVENT_ID");
            event.remove("google_workspace.device.LAST_SYNC_AUDIT_DATE");

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
                event.remove("google_workspace.device.os.version");
                event.remove("google_workspace.device.user_email");
                event.remove("google_workspace.id.time");
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

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
