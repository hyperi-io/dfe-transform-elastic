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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.document_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.severity") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.av_softwares") {
                event.rename("json.av_softwares", "spycloud.compass.av_softwares")?;
            }

            if event.has_value("json.backup_email_username") {
                event.rename(
                    "json.backup_email_username",
                    "spycloud.compass.backup.email.username",
                )?;
            }

            let _cond = { event.has_value("spycloud.compass.backup.email.username") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.compass.backup.email.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.backup_email") {
                event.rename("json.backup_email", "spycloud.compass.backup.email.value")?;
            }

            if event.has_value("json.bank_number") {
                event.rename("json.bank_number", "spycloud.compass.bank_number")?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.bank_number")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.bank_number", v)?;
                }
            }

            if event.has_value("json.cc_bin") {
                event.rename("json.cc_bin", "spycloud.compass.cc.bin")?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.cc.bin")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.cc.bin", v)?;
                }
            }

            if event.has_value("json.cc_expiration") {
                event.rename("json.cc_expiration", "spycloud.compass.cc.expiration")?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.cc.expiration")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.cc.expiration", v)?;
                }
            }

            if event.has_value("json.cc_last_four") {
                event.rename("json.cc_last_four", "spycloud.compass.cc.last_four")?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.cc.last_four")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.cc.last_four", v)?;
                }
            }

            if event.has_value("json.cc_number") {
                event.rename("json.cc_number", "spycloud.compass.cc.number")?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.cc.number")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.cc.number", v)?;
                }
            }

            if event.has_value("json.country_code") {
                event.rename("json.country_code", "spycloud.compass.country.code")?;
            }

            if let Some(v) = event
                .get("spycloud.compass.country.code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.geo.country_iso_code", v)?;
            }

            if event.has_value("json.country") {
                event.rename("json.country", "spycloud.compass.country.name")?;
            }

            if let Some(v) = event
                .get("spycloud.compass.country.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.geo.country_name", v)?;
            }

            if event.has_value("json.display_resolution") {
                event.rename(
                    "json.display_resolution",
                    "spycloud.compass.display_resolution",
                )?;
            }

            if event.has_value("json.document_id") {
                event.rename("json.document_id", "spycloud.compass.document_id")?;
            }

            if let Some(v) = event
                .get("spycloud.compass.document_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.domain") {
                event.rename("json.domain", "spycloud.compass.domain")?;
            }

            let _cond = { event.has_value("spycloud.compass.domain") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.compass.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("spycloud.compass.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            if event.has_value("json.drivers_license") {
                event.rename(
                    "json.drivers_license",
                    "spycloud.compass.drivers.license.number",
                )?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.license.number")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.license.number", v)?;
                }
            }

            if event.has_value("json.drivers_license_state_code") {
                event.rename(
                    "json.drivers_license_state_code",
                    "spycloud.compass.drivers.license.state_code",
                )?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.license.state_code")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.license.state_code", v)?;
                }
            }

            if event.has_value("json.email_domain") {
                event.rename("json.email_domain", "spycloud.compass.email.domain")?;
            }

            let _cond = { event.has_value("spycloud.compass.email.domain") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.compass.email.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.email_username") {
                event.rename("json.email_username", "spycloud.compass.email.username")?;
            }

            let _cond = { event.has_value("spycloud.compass.email.username") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.compass.email.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.email") {
                event.rename("json.email", "spycloud.compass.email.value")?;
            }

            if let Some(v) = event
                .get("spycloud.compass.email.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            if event.has_value("json.full_name") {
                event.rename("json.full_name", "spycloud.compass.full_name")?;
            }

            let _cond = { event.has_value("spycloud.compass.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.compass.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("spycloud.compass.full_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.full_name", v)?;
            }

            if event.has_value("json.homepage") {
                event.rename("json.homepage", "spycloud.compass.homepage")?;
            }

            if event.has_value("json.infected_machine_id") {
                event.rename(
                    "json.infected_machine_id",
                    "spycloud.compass.infected.machine_id",
                )?;
            }

            if event.has_value("json.infected_path") {
                event.rename("json.infected_path", "spycloud.compass.infected.path")?;
            }

            let _cond = {
                event.has_value("json.infected_time")
                    && event.get_str("json.infected_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.infected_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("spycloud.compass.infected.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.infected_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_infected_time")?;
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

            if event.has_value("json.ip_addresses") {
                event.rename("json.ip_addresses", "spycloud.compass.ip_addresses")?;
            }

            let _cond = {
                event
                    .get("spycloud.compass.ip_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "spycloud.compass.ip_addresses", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_ip_addresses_values_to_ip",
                        )?;
                        event.remove("_ingest._value");
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
                    Ok(())
                })?;
            }

            if let Some(v) = event
                .get("spycloud.compass.ip_addresses")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.ip", v)?;
            }

            if let Some(v) = event
                .get("spycloud.compass.ip_addresses")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("related.ip", v)?;
            }

            if event.has_value("json.keyboard_languages") {
                event.rename(
                    "json.keyboard_languages",
                    "spycloud.compass.keyboard_languages",
                )?;
            }

            if event.has_value("json.log_id") {
                event.rename("json.log_id", "spycloud.compass.log_id")?;
            }

            if event.has_value("json.national_id") {
                event.rename("json.national_id", "spycloud.compass.national_id")?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.national_id")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.national_id", v)?;
                }
            }

            if event.has_value("json.passport_number") {
                event.rename("json.passport_number", "spycloud.compass.passport_number")?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.passport_number")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.passport_number", v)?;
                }
            }

            if event.has_value("json.password_plaintext") {
                event.rename(
                    "json.password_plaintext",
                    "spycloud.compass.password.plaintext",
                )?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.password.plaintext")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.password.plaintext", v)?;
                }
            }

            if event.has_value("json.password_type") {
                event.rename("json.password_type", "spycloud.compass.password.type")?;
            }

            if event.has_value("json.password") {
                event.rename("json.password", "spycloud.compass.password.value")?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.password.value")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.password.value", v)?;
                }
            }

            if event.has_value("json.postal_code") {
                event.rename("json.postal_code", "spycloud.compass.postal_code")?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.postal_code")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.postal_code", v)?;
                }
            }

            if let Some(v) = event
                .get("spycloud.compass.postal_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.geo.postal_code", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.severity") {
                    if let Some(val) = event.get("json.severity") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severity".into(),
                                message,
                            }
                        })?;
                        event.set("spycloud.compass.severity", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_severity_to_long",
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

            if let Some(v) = event
                .get("spycloud.compass.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.severity", v)?;
            }

            if event.has_value("json.social_security_number") {
                event.rename(
                    "json.social_security_number",
                    "spycloud.compass.social_security_number",
                )?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.social_security_number")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.social_security_number", v)?;
                }
            }

            if event.has_value("json.source_id") {
                if let Some(val) = event.get("json.source_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.source_id".into(),
                            message,
                        }
                    })?;
                    event.set("spycloud.compass.source_id", converted)?;
                }
            }

            let _cond = {
                event.has_value("json.spycloud_publish_date")
                    && event.get_str("json.spycloud_publish_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.spycloud_publish_date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("spycloud.compass.spycloud_publish_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.spycloud_publish_date".into(),
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
                        "date_spycloud_publish_date",
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

            if let Some(v) = event
                .get("spycloud.compass.spycloud_publish_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.ssn_last_four") {
                event.rename("json.ssn_last_four", "spycloud.compass.ssn_last_four")?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("spycloud.compass.ssn_last_four")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.compass.ssn_last_four", v)?;
                }
            }

            if event.has_value("json.target_domain") {
                event.rename("json.target_domain", "spycloud.compass.target.domain")?;
            }

            if let Some(v) = event
                .get("spycloud.compass.target.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            if event.has_value("json.target_subdomain") {
                event.rename("json.target_subdomain", "spycloud.compass.target.subdomain")?;
            }

            if let Some(v) = event
                .get("spycloud.compass.target.subdomain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.subdomain", v)?;
            }

            if event.has_value("json.target_url") {
                event.rename("json.target_url", "spycloud.compass.target.url")?;
            }

            if let Some(v) = event
                .get("spycloud.compass.target.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.full", v)?;
            }

            let _cond = { event.has_value("url.full") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if !uri_parts(event, "url.full", "url", true, false)?
                        && event
                            .get_str("url.full")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "url.full".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                    Ok(())
                })();
            }

            if event.has_value("json.user_browser") {
                event.rename("json.user_browser", "spycloud.compass.user.browser")?;
            }

            if event.has_value("json.user_hostname") {
                event.rename("json.user_hostname", "spycloud.compass.user.hostname")?;
            }

            let _cond = { event.has_value("spycloud.compass.user.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("spycloud.compass.user.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("spycloud.compass.user.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if event.has_value("json.username") {
                event.rename("json.username", "spycloud.compass.user.name")?;
            }

            let _cond = { event.has_value("spycloud.compass.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.compass.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("spycloud.compass.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if event.has_value("json.user_os") {
                event.rename("json.user_os", "spycloud.compass.user.os")?;
            }

            let _cond = { event.has_value("spycloud.compass.user.os") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("spycloud.compass.user.os")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("spycloud.compass.user.os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            let _cond = { event.has_value("spycloud.compass.user.os") };
            if _cond {
                // Painless script
                // Source: if (ctx.spycloud.compass.user.os.toLowerCase().contains('windows')) {\n    ctx.host.os.type = 'windows';\n} else if (ctx.spycloud.compass.user.os.toLowerCase().contains('linux')) {\n    ctx.host.os.type = 'linux';\n} else if (ctx.spycloud.compass.user.os.toLowerCase().contains('mac')) {\n    ctx.host.os.type = 'macos';\n} else if (ctx.spycloud.compass.user.os.toLowerCase().contains('unix')) {\n    ctx.host.os.type = 'unix';\n} else if (ctx.spycloud.compass.user.os.toLowerCase().contains('ios')) {\n    ctx.host.os.type = 'ios';\n} else if (ctx.spycloud.compass.user.os.toLowerCase().contains('android')) {\n    ctx.host.os.type = 'android';\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.spycloud.compass.user.os.toLowerCase().contains('windows')) {\n    ctx.host.os.type = 'windows';\n} else if (ctx.spycloud.compass.user.os.toLowerCase().contains('linux')) {\n    ctx.host.os.type = 'linux';\n} else if (ctx.spycloud.compass.user.os.toLowerCase().contains('mac')) {\n    ctx.host.os.type = 'macos';\n} else if (ctx.spycloud.compass.user.os.toLowerCase().contains('unix')) {\n    ctx.host.os.type = 'unix';\n} else if (ctx.spycloud.compass.user.os.toLowerCase().contains('ios')) {\n    ctx.host.os.type = 'ios';\n} else if (ctx.spycloud.compass.user.os.toLowerCase().contains('android')) {\n    ctx.host.os.type = 'android';\n}\n"#
                    ),
                )?;
            }

            if event.has_value("json.user_sys_domain") {
                event.rename("json.user_sys_domain", "spycloud.compass.user.sys.domain")?;
            }

            let _cond = { event.has_value("spycloud.compass.user.sys.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("spycloud.compass.user.sys.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.user_sys_registered_owner") {
                event.rename(
                    "json.user_sys_registered_owner",
                    "spycloud.compass.user.sys.registered_owner",
                )?;
            }

            let _cond = { event.has_value("spycloud.compass.user.sys.registered_owner") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.compass.user.sys.registered_owner")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

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
                event.remove("spycloud.compass.country.code");
                event.remove("spycloud.compass.country.name");
                event.remove("spycloud.compass.document_id");
                event.remove("spycloud.compass.domain");
                event.remove("spycloud.compass.email.value");
                event.remove("spycloud.compass.full_name");
                event.remove("spycloud.compass.ip_addresses");
                event.remove("spycloud.compass.postal_code");
                event.remove("spycloud.compass.severity");
                event.remove("spycloud.compass.spycloud_publish_date");
                event.remove("spycloud.compass.target.domain");
                event.remove("spycloud.compass.target.subdomain");
                event.remove("spycloud.compass.target.url");
                event.remove("spycloud.compass.user.hostname");
                event.remove("spycloud.compass.user.os");
                event.remove("spycloud.compass.user.name");
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
                event.append_unique("event.kind", json!("pipeline_error"))?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
