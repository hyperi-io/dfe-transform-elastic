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

            event.set("event.kind", json!("alert"))?;

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

            if event.has_value("json.account_image_url") {
                event.rename(
                    "json.account_image_url",
                    "spycloud.breach_record.account.image_url",
                )?;
            }

            let _cond = {
                event.has_value("json.account_login_time")
                    && event.get_str("json.account_login_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.account_login_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("spycloud.breach_record.account.login_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.account_login_time".into(),
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
                        "date_account_login_time",
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

            let _cond = {
                event.has_value("json.account_modification_time")
                    && event.get_str("json.account_modification_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.account_modification_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("spycloud.breach_record.account.modification_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.account_modification_time".into(),
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
                        "date_account_modification_time",
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

            let _cond = {
                event.has_value("json.account_signup_time")
                    && event.get_str("json.account_signup_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.account_signup_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("spycloud.breach_record.account.signup_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.account_signup_time".into(),
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
                        "date_account_signup_time",
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

            if event.has_value("json.av_softwares") {
                event.rename("json.av_softwares", "spycloud.breach_record.av_softwares")?;
            }

            if event.has_value("json.cc_bin") {
                event.rename("json.cc_bin", "spycloud.breach_record.cc.bin")?;
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
                    && event.has_value("spycloud.breach_record.cc.bin")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.breach_record.cc.bin", v)?;
                }
            }

            if event.has_value("json.cc_expiration") {
                event.rename("json.cc_expiration", "spycloud.breach_record.cc.expiration")?;
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
                    && event.has_value("spycloud.breach_record.cc.expiration")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.breach_record.cc.expiration", v)?;
                }
            }

            if event.has_value("json.cc_last_four") {
                event.rename("json.cc_last_four", "spycloud.breach_record.cc.last_four")?;
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
                    && event.has_value("spycloud.breach_record.cc.last_four")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.breach_record.cc.last_four", v)?;
                }
            }

            if event.has_value("json.cc_number") {
                event.rename("json.cc_number", "spycloud.breach_record.cc.number")?;
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
                    && event.has_value("spycloud.breach_record.cc.number")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.breach_record.cc.number", v)?;
                }
            }

            if event.has_value("json.company_name") {
                event.rename("json.company_name", "spycloud.breach_record.company_name")?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.company_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            if event.has_value("json.country_code") {
                event.rename("json.country_code", "spycloud.breach_record.country.code")?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.country.code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.geo.country_iso_code", v)?;
            }

            if event.has_value("json.country") {
                event.rename("json.country", "spycloud.breach_record.country.name")?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.country.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.geo.country_name", v)?;
            }

            if event.has_value("json.display_resolution") {
                event.rename(
                    "json.display_resolution",
                    "spycloud.breach_record.display_resolution",
                )?;
            }

            if event.has_value("json.document_id") {
                event.rename("json.document_id", "spycloud.breach_record.document_id")?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.document_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.domain") {
                event.rename("json.domain", "spycloud.breach_record.domain")?;
            }

            let _cond = { event.has_value("spycloud.breach_record.domain") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.breach_record.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            if event.has_value("json.email_domain") {
                event.rename("json.email_domain", "spycloud.breach_record.email.domain")?;
            }

            let _cond = { event.has_value("spycloud.breach_record.email.domain") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.breach_record.email.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.email_username") {
                event.rename(
                    "json.email_username",
                    "spycloud.breach_record.email.username",
                )?;
            }

            let _cond = { event.has_value("spycloud.breach_record.email.username") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.breach_record.email.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.email") {
                event.rename("json.email", "spycloud.breach_record.email.value")?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.email.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            if event.has_value("json.first_name") {
                event.rename("json.first_name", "spycloud.breach_record.first_name")?;
            }

            if event.has_value("json.full_name") {
                event.rename("json.full_name", "spycloud.breach_record.full_name")?;
            }

            let _cond = { event.has_value("spycloud.breach_record.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.breach_record.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.full_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.full_name", v)?;
            }

            if event.has_value("json.homepage") {
                event.rename("json.homepage", "spycloud.breach_record.homepage")?;
            }

            if event.has_value("json.industry") {
                event.rename("json.industry", "spycloud.breach_record.industry")?;
            }

            if event.has_value("json.infected_machine_id") {
                event.rename(
                    "json.infected_machine_id",
                    "spycloud.breach_record.infected.machine_id",
                )?;
            }

            if event.has_value("json.infected_path") {
                event.rename("json.infected_path", "spycloud.breach_record.infected.path")?;
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
                            Some(parsed) => {
                                event.set("spycloud.breach_record.infected.time", parsed)?
                            }
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
                event.rename("json.ip_addresses", "spycloud.breach_record.ip_addresses")?;
            }

            let _cond = {
                event
                    .get("spycloud.breach_record.ip_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "spycloud.breach_record.ip_addresses", |event| {
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
                .get("spycloud.breach_record.ip_addresses")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.ip", v)?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.ip_addresses")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("related.ip", v)?;
            }

            if event.has_value("json.job_title") {
                event.rename("json.job_title", "spycloud.breach_record.job_title")?;
            }

            if event.has_value("json.keyboard_languages") {
                event.rename(
                    "json.keyboard_languages",
                    "spycloud.breach_record.keyboard_languages",
                )?;
            }

            if event.has_value("json.last_name") {
                event.rename("json.last_name", "spycloud.breach_record.last_name")?;
            }

            if event.has_value("json.log_id") {
                event.rename("json.log_id", "spycloud.breach_record.log_id")?;
            }

            if event.has_value("json.password_plaintext") {
                event.rename(
                    "json.password_plaintext",
                    "spycloud.breach_record.password.plaintext",
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
                    && event.has_value("spycloud.breach_record.password.plaintext")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.breach_record.password.plaintext", v)?;
                }
            }

            if event.has_value("json.password_type") {
                event.rename("json.password_type", "spycloud.breach_record.password.type")?;
            }

            if event.has_value("json.password") {
                event.rename("json.password", "spycloud.breach_record.password.value")?;
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
                    && event.has_value("spycloud.breach_record.password.value")
            };
            if _cond {
                let v = json!("REDACTED");
                if !painless_is_empty_value(&v) {
                    event.set("spycloud.breach_record.password.value", v)?;
                }
            }

            let _cond = {
                event.has_value("json.record_addition_date")
                    && event.get_str("json.record_addition_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.record_addition_date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("spycloud.breach_record.record.addition_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.record_addition_date".into(),
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
                        "date_record_addition_date",
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

            let _cond = {
                event.has_value("json.record_cracked_date")
                    && event.get_str("json.record_cracked_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.record_cracked_date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("spycloud.breach_record.record.cracked_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.record_cracked_date".into(),
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
                        "date_record_cracked_date",
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

            let _cond = {
                event.has_value("json.record_modification_date")
                    && event.get_str("json.record_modification_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.record_modification_date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("spycloud.breach_record.record.modification_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.record_modification_date".into(),
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
                        "date_record_modification_date",
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

            if event.has_value("json.salt") {
                event.rename("json.salt", "spycloud.breach_record.salt")?;
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
                        event.set("spycloud.breach_record.severity", converted)?;
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
                .get("spycloud.breach_record.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.severity", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.sighting") {
                    if let Some(val) = event.get("json.sighting") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.sighting".into(),
                                message,
                            }
                        })?;
                        event.set("spycloud.breach_record.sighting", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_sighting_to_long",
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

            if event.has_value("json.social_linkedin") {
                event.rename(
                    "json.social_linkedin",
                    "spycloud.breach_record.social_linkedin",
                )?;
            }

            if event.has_value("json.source_id") {
                if let Some(val) = event.get("json.source_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.source_id".into(),
                            message,
                        }
                    })?;
                    event.set("spycloud.breach_record.source_id", converted)?;
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
                                event.set("spycloud.breach_record.spycloud_publish_date", parsed)?
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

            let _cond = { event.has_value("json.record_modification_date") };
            if _cond {
                if let Some(v) = event
                    .get("spycloud.breach_record.record.modification_date")
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { !event.has_value("json.record_modification_date") };
            if _cond {
                if let Some(v) = event
                    .get("spycloud.breach_record.spycloud_publish_date")
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
            }

            if event.has_value("json.target_domain") {
                event.rename("json.target_domain", "spycloud.breach_record.target.domain")?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.target.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            if event.has_value("json.target_subdomain") {
                event.rename(
                    "json.target_subdomain",
                    "spycloud.breach_record.target.subdomain",
                )?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.target.subdomain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.subdomain", v)?;
            }

            if event.has_value("json.target_url") {
                event.rename("json.target_url", "spycloud.breach_record.target.url")?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.target.url")
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
                event.rename("json.user_browser", "spycloud.breach_record.user.browser")?;
            }

            if event.has_value("json.user_hostname") {
                event.rename("json.user_hostname", "spycloud.breach_record.user.hostname")?;
            }

            let _cond = { event.has_value("spycloud.breach_record.user.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("spycloud.breach_record.user.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.user.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if event.has_value("json.username") {
                event.rename("json.username", "spycloud.breach_record.user.name")?;
            }

            let _cond = { event.has_value("spycloud.breach_record.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.breach_record.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if event.has_value("json.user_os") {
                event.rename("json.user_os", "spycloud.breach_record.user.os")?;
            }

            let _cond = { event.has_value("spycloud.breach_record.user.os") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("spycloud.breach_record.user.os")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("spycloud.breach_record.user.os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            let _cond = { event.has_value("spycloud.breach_record.user.os") };
            if _cond {
                // Painless script
                // Source: if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('windows')) {\n    ctx.host.os.type = 'windows';\n} else if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('linux')) {\n    ctx.host.os.type = 'linux';\n} else if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('mac')) {\n    ctx.host.os.type = 'macos';\n} else if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('unix')) {\n    ctx.host.os.type = 'unix';\n} else if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('ios')) {\n    ctx.host.os.type = 'ios';\n} else if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('android')) {\n    ctx.host.os.type = 'android';\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('windows')) {\n    ctx.host.os.type = 'windows';\n} else if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('linux')) {\n    ctx.host.os.type = 'linux';\n} else if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('mac')) {\n    ctx.host.os.type = 'macos';\n} else if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('unix')) {\n    ctx.host.os.type = 'unix';\n} else if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('ios')) {\n    ctx.host.os.type = 'ios';\n} else if (ctx.spycloud.breach_record.user.os.toLowerCase().contains('android')) {\n    ctx.host.os.type = 'android';\n}\n"#
                    ),
                )?;
            }

            if event.has_value("json.user_sys_domain") {
                event.rename(
                    "json.user_sys_domain",
                    "spycloud.breach_record.user.sys.domain",
                )?;
            }

            let _cond = { event.has_value("spycloud.breach_record.user.sys.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("spycloud.breach_record.user.sys.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.user_sys_registered_owner") {
                event.rename(
                    "json.user_sys_registered_owner",
                    "spycloud.breach_record.user.sys.registered_owner",
                )?;
            }

            let _cond = { event.has_value("spycloud.breach_record.user.sys.registered_owner") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("spycloud.breach_record.user.sys.registered_owner")
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
                event.remove("spycloud.breach_record.country.code");
                event.remove("spycloud.breach_record.country.name");
                event.remove("spycloud.breach_record.document_id");
                event.remove("spycloud.breach_record.domain");
                event.remove("spycloud.breach_record.email.value");
                event.remove("spycloud.breach_record.full_name");
                event.remove("spycloud.breach_record.ip_addresses");
                event.remove("spycloud.breach_record.severity");
                event.remove("spycloud.breach_record.target.domain");
                event.remove("spycloud.breach_record.target.subdomain");
                event.remove("spycloud.breach_record.target.url");
                event.remove("spycloud.breach_record.user.hostname");
                event.remove("spycloud.breach_record.user.os");
                event.remove("spycloud.breach_record.user.name");
                event.remove("spycloud.breach_record.company_name");
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
