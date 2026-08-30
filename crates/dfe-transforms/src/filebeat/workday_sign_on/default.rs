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
            event.set("ecs.version", json!("9.4.0"))?;

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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

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

            let _cond = { event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "workday.sign_on")?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.get("workday.sign_on").is_some_and(|v| v.is_object()) };
            if _cond {
                // Painless script
                // Source: def signon = ctx.workday.sign_on;\nfor (def key : new ArrayList(signon.keySet())) {\n  if (key.contains('-')) {\n    signon.put(key.replace('-', '_'), signon.remove(key));\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def signon = ctx.workday.sign_on;\nfor (def key : new ArrayList(signon.keySet())) {\n  if (key.contains('-')) {\n    signon.put(key.replace('-', '_'), signon.remove(key));\n  }\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("workday.sign_on") };
            if _cond {
                // Painless script
                // Source: def flags = [\n  'Account_Locked__Disabled_or_Expired',\n  'Active_Session',\n  'Device_is_Trusted',\n  'Failed_Signon',\n  'Forgotten_Password_Reset_Request',\n  'Invalid_Credentials',\n  'Invalid_Password',\n  'Is_Device_Managed',\n  'Password_Changed',\n  'Signon',\n  'Successful'\n];\ndef signon = ctx.workday.sign_on;\nfor (def flag : flags) {\n  if (signon.containsKey(flag) && signon.get(flag) instanceof String) {\n    signon.put(flag, signon.get(flag) == '1');\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def flags = [\n  'Account_Locked__Disabled_or_Expired',\n  'Active_Session',\n  'Device_is_Trusted',\n  'Failed_Signon',\n  'Forgotten_Password_Reset_Request',\n  'Invalid_Credentials',\n  'Invalid_Password',\n  'Is_Device_Managed',\n  'Password_Changed',\n  'Signon',\n  'Successful'\n];\ndef signon = ctx.workday.sign_on;\nfor (def flag : flags) {\n  if (signon.containsKey(flag) && signon.get(flag) instanceof String) {\n    signon.put(flag, signon.get(flag) == '1');\n  }\n}"#
                    ),
                )?;
            }

            let _cond = { !event.has_value("workday.sign_on.Authentication_Type") };
            if _cond {
                if let Some(v) = event
                    .get("workday.sign_on.Authentication_Type_for_Signon")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("workday.sign_on.Authentication_Type") {
                        event.set("workday.sign_on.Authentication_Type", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("workday.sign_on.Sign_on_Time")
                    && event.get_str("workday.sign_on.Sign_on_Time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("workday.sign_on.Sign_on_Time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("workday.sign_on.Sign_on_Time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "workday.sign_on.Sign_on_Time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_signon_time")?;
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
                .get("workday.sign_on.Sign_on_Time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("workday.sign_on.Sign_on_Time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            let _cond = {
                event.has_value("workday.sign_on.Session_Start")
                    && event.get_str("workday.sign_on.Session_Start") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("workday.sign_on.Session_Start") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("workday.sign_on.Session_Start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "workday.sign_on.Session_Start".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_session_start")?;
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
                .get("workday.sign_on.Session_Start")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("@timestamp") {
                    event.set("@timestamp", v)?;
                }
            }

            if let Some(v) = event
                .get("workday.sign_on.Session_Start")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("event.start") {
                    event.set("event.start", v)?;
                }
            }

            let _cond = {
                event.has_value("workday.sign_on.Signon_DateTime")
                    && event.get_str("workday.sign_on.Signon_DateTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("workday.sign_on.Signon_DateTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("workday.sign_on.Signon_DateTime", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "workday.sign_on.Signon_DateTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_signon_datetime")?;
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
                .get("workday.sign_on.Signon_DateTime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("@timestamp") {
                    event.set("@timestamp", v)?;
                }
            }

            if let Some(v) = event
                .get("workday.sign_on.Signon_DateTime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("event.start") {
                    event.set("event.start", v)?;
                }
            }

            let _cond = {
                event.has_value("workday.sign_on.Session_End")
                    && event.get_str("workday.sign_on.Session_End") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("workday.sign_on.Session_End") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("workday.sign_on.Session_End", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "workday.sign_on.Session_End".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_session_end")?;
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
                .get("workday.sign_on.Session_End")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("event.end") {
                    event.set("event.end", v)?;
                }
            }

            let _cond = {
                event.has_value("workday.sign_on.Signoff_DateTime")
                    && event.get_str("workday.sign_on.Signoff_DateTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("workday.sign_on.Signoff_DateTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("workday.sign_on.Signoff_DateTime", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "workday.sign_on.Signoff_DateTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_signoff_datetime")?;
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
                .get("workday.sign_on.Signoff_DateTime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("event.end") {
                    event.set("event.end", v)?;
                }
            }

            let _cond = {
                event.has_value("workday.sign_on.Signoff_Time")
                    && event.get_str("workday.sign_on.Signoff_Time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("workday.sign_on.Signoff_Time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("workday.sign_on.Signoff_Time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "workday.sign_on.Signoff_Time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_signoff_time")?;
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
                .get("workday.sign_on.Signoff_Time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = {
                event.has_value("workday.sign_on.Created_Moment")
                    && event.get_str("workday.sign_on.Created_Moment") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("workday.sign_on.Created_Moment") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("workday.sign_on.Created_Moment", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "workday.sign_on.Created_Moment".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created_moment")?;
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
                .get("workday.sign_on.Created_Moment")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("workday.sign_on.userName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("workday.sign_on.Signon_Worker")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.full_name", v)?;
            }

            if let Some(v) = event
                .get("workday.sign_on.User_Agent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.original", v)?;
            }

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

            if let Some(v) = event
                .get("workday.sign_on.Operating_System")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            if let Some(v) = event
                .get("workday.sign_on.Device_Type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.type", v)?;
            }

            let _cond = { event.get_str("workday.sign_on.Signon_IP_Address") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("workday.sign_on.Signon_IP_Address") {
                        if let Some(val) = event.get("workday.sign_on.Signon_IP_Address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "workday.sign_on.Signon_IP_Address".into(),
                                    message,
                                }
                            })?;
                            event.set("workday.sign_on.Signon_IP_Address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_signon_ip_address_to_source_ip",
                    )?;
                    if event.has_value("workday.sign_on.Signon_IP_Address") {
                        event.rename(
                            "workday.sign_on.Signon_IP_Address",
                            "workday.sign_on.signon_ip_address_string",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("workday.sign_on.Signon_IP_Address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.get_str("workday.sign_on.Session_IP_Address") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("workday.sign_on.Session_IP_Address") {
                        if let Some(val) = event.get("workday.sign_on.Session_IP_Address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "workday.sign_on.Session_IP_Address".into(),
                                    message,
                                }
                            })?;
                            event.set("workday.sign_on.Session_IP_Address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_session_ip_address_to_source_ip",
                    )?;
                    if event.has_value("workday.sign_on.Session_IP_Address") {
                        event.rename(
                            "workday.sign_on.Session_IP_Address",
                            "workday.sign_on.session_ip_address_string",
                        )?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("workday.sign_on.Session_IP_Address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("source.ip") {
                    event.set("source.ip", v)?;
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
                .get("workday.sign_on.Authentication_Failure_Message")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            if let Some(v) = event
                .get("workday.sign_on.tenant_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.full_name")
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

            if let Some(v) = event
                .get("workday.sign_on.Browser_Type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("user_agent.name") {
                    event.set("user_agent.name", v)?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append_unique("event.category", json!("authentication"))?;

            event.append_unique("event.category", json!("session"))?;

            event.append_unique("event.type", json!("start"))?;

            let _cond = { event.has_value("event.end") };
            if _cond {
                event.append_unique("event.type", json!("end"))?;
            }

            event.set("event.action", json!("user-signon"))?;

            let _cond = {
                event.get_bool("workday.sign_on.Failed_Signon") == Some(true)
                    || event.has_value("event.reason")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event
                    .get("workday.sign_on.Is_Device_Managed")
                    .is_some_and(|v| v.is_boolean())
            };
            if _cond {
                if let Some(v) = event
                    .get("workday.sign_on.Is_Device_Managed")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.entity.attributes.managed", v)?;
                }
            }

            if let Some(v) = event
                .get("event.start")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.entity.lifecycle.last_activity", v)?;
            }

            if let Some(v) = event
                .get("event.end")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.entity.lifecycle.last_activity", v)?;
            }

            event.remove("workday.sign_on.Session_Start");
            event.remove("workday.sign_on.Session_End");
            event.remove("workday.sign_on.Sign_on_Time");
            event.remove("workday.sign_on.Created_Moment");
            event.remove("workday.sign_on.userName");
            event.remove("workday.sign_on.Signon_Worker");
            event.remove("workday.sign_on.User_Agent");
            event.remove("workday.sign_on.Operating_System");
            event.remove("workday.sign_on.Device_Type");
            event.remove("workday.sign_on.Signon_IP_Address");
            event.remove("workday.sign_on.Session_IP_Address");
            event.remove("workday.sign_on.Authentication_Failure_Message");
            event.remove("workday.sign_on.tenant_name");
            event.remove("workday.sign_on.Signon_DateTime");
            event.remove("workday.sign_on.Signoff_DateTime");
            event.remove("workday.sign_on.Signoff_Time");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
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
                    json!(format!(
                        "Processor '{}'\n{}failed with message '{}'",
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
                                "with tag '{}'\n",
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
