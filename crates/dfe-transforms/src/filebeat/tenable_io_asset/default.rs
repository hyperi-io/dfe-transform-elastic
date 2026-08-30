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

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("state"))?;

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
                event.has_value("json.created_at") && event.get_str("json.created_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("tenable_io.asset.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.created_at".into(),
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

            if event.has_value("json.id") {
                event.rename("json.id", "tenable_io.asset.id")?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond = { event.get_str("json.has_agent") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.has_agent") {
                        if let Some(val) = event.get("json.has_agent") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.has_agent".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.asset.has_agent", converted)?;
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
            }

            let _cond = { event.get_str("json.has_plugin_results") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.has_plugin_results") {
                        if let Some(val) = event.get("json.has_plugin_results") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.has_plugin_results".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.asset.has_plugin_results", converted)?;
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
            }

            let _cond = {
                event.has_value("json.terminated_at")
                    && event.get_str("json.terminated_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.terminated_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("tenable_io.asset.terminated_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.terminated_at".into(),
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

            if event.has_value("json.terminated_by") {
                event.rename("json.terminated_by", "tenable_io.asset.terminated_by")?;
            }

            let _cond = {
                event.has_value("json.updated_at") && event.get_str("json.updated_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.updated_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("tenable_io.asset.updated_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.updated_at".into(),
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
                .get("tenable_io.asset.updated_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("json.deleted_at") && event.get_str("json.deleted_at") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.deleted_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("tenable_io.asset.deleted_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.deleted_at".into(),
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

            if event.has_value("json.deleted_by") {
                event.rename("json.deleted_by", "tenable_io.asset.deleted_by")?;
            }

            let _cond = {
                event.has_value("json.first_seen") && event.get_str("json.first_seen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.first_seen") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("tenable_io.asset.first_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.first_seen".into(),
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
                event.has_value("json.last_seen") && event.get_str("json.last_seen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_seen") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("tenable_io.asset.last_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_seen".into(),
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
                event.has_value("json.first_scan_time")
                    && event.get_str("json.first_scan_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.first_scan_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("tenable_io.asset.first_scan_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.first_scan_time".into(),
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
                event.has_value("json.last_scan_time")
                    && event.get_str("json.last_scan_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_scan_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("tenable_io.asset.last_scan_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_scan_time".into(),
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
                event.has_value("json.last_authenticated_scan_date")
                    && event.get_str("json.last_authenticated_scan_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_authenticated_scan_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("tenable_io.asset.last_authenticated_scan_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_authenticated_scan_date".into(),
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
                event.has_value("json.last_licensed_scan_date")
                    && event.get_str("json.last_licensed_scan_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_licensed_scan_date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("tenable_io.asset.last_licensed_scan_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_licensed_scan_date".into(),
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

            if event.has_value("json.last_scan_id") {
                event.rename("json.last_scan_id", "tenable_io.asset.last_scan_id")?;
            }

            if event.has_value("json.last_schedule_id") {
                event.rename("json.last_schedule_id", "tenable_io.asset.last_schedule_id")?;
            }

            if event.has_value("json.azure_resource_id") {
                event.rename(
                    "json.azure_resource_id",
                    "tenable_io.asset.azure.resource_id",
                )?;
            }

            if event.has_value("json.agent_uuid") {
                event.rename("json.agent_uuid", "tenable_io.asset.agent_uuid")?;
            }

            if event.has_value("json.bios_uuid") {
                event.rename("json.bios_uuid", "tenable_io.asset.bios_uuid")?;
            }

            if event.has_value("json.aws_owner_id") {
                event.rename("json.aws_owner_id", "tenable_io.asset.aws.owner_id")?;
            }

            if event.has_value("json.aws_vpc_id") {
                event.rename("json.aws_vpc_id", "tenable_io.asset.aws.vpc_id")?;
            }

            if event.has_value("json.aws_ec2_instance_group_name") {
                event.rename(
                    "json.aws_ec2_instance_group_name",
                    "tenable_io.asset.aws.ec2_instance.group_name",
                )?;
            }

            if event.has_value("json.aws_ec2_instance_state_name") {
                event.rename(
                    "json.aws_ec2_instance_state_name",
                    "tenable_io.asset.aws.ec2_instance.state_name",
                )?;
            }

            if event.has_value("json.aws_subnet_id") {
                event.rename("json.aws_subnet_id", "tenable_io.asset.aws.subnet_id")?;
            }

            if event.has_value("json.aws_ec2_product_code") {
                event.rename(
                    "json.aws_ec2_product_code",
                    "tenable_io.asset.aws.ec2_product_code",
                )?;
            }

            if event.has_value("json.aws_ec2_name") {
                event.rename("json.aws_ec2_name", "tenable_io.asset.aws.ec2_name")?;
            }

            if event.has_value("json.mcafee_epo_guid") {
                event.rename("json.mcafee_epo_guid", "tenable_io.asset.mcafee_epo.guid")?;
            }

            if event.has_value("json.mcafee_epo_agent_guid") {
                event.rename(
                    "json.mcafee_epo_agent_guid",
                    "tenable_io.asset.mcafee_epo.agent_guid",
                )?;
            }

            if event.has_value("json.servicenow_sysid") {
                event.rename("json.servicenow_sysid", "tenable_io.asset.servicenow_sysid")?;
            }

            if event.has_value("json.bigfix_asset_id") {
                event.rename("json.bigfix_asset_id", "tenable_io.asset.bigfix_asset_id")?;
            }

            if event.has_value("json.agent_names") {
                event.rename("json.agent_names", "tenable_io.asset.agent_names")?;
            }

            if event.has_value("json.installed_software") {
                event.rename(
                    "json.installed_software",
                    "tenable_io.asset.installed_software",
                )?;
            }

            let _cond = { event.get("json.ipv4s").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.ipv4s", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("_ingest._value");
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
                    Ok(())
                })?;
            }

            if event.has_value("json.ipv4s") {
                event.rename("json.ipv4s", "tenable_io.asset.ipv4s")?;
            }

            let _cond = {
                event
                    .get("tenable_io.asset.ipv4s")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "tenable_io.asset.ipv4s", |event| {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.ipv6s").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.ipv6s", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("_ingest._value");
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
                    Ok(())
                })?;
            }

            if event.has_value("json.ipv6s") {
                event.rename("json.ipv6s", "tenable_io.asset.ipv6s")?;
            }

            let _cond = {
                event
                    .get("tenable_io.asset.ipv6s")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "tenable_io.asset.ipv6s", |event| {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.fqdns") {
                event.rename("json.fqdns", "tenable_io.asset.fqdns")?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.fqdns")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.domain", v)?;
            }

            if event.has_value("json.mac_addresses") {
                gsub_field(
                    event,
                    "json.mac_addresses",
                    "json.mac_addresses",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            if event.has_value("json.mac_addresses") {
                map_strings(
                    event,
                    "json.mac_addresses",
                    "json.mac_addresses",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("json.mac_addresses") {
                event.rename("json.mac_addresses", "tenable_io.asset.mac_addresses")?;
            }

            let _cond = {
                event
                    .get("tenable_io.asset.mac_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "tenable_io.asset.mac_addresses", |event| {
                    event.append_unique(
                        "host.mac",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.netbios_names") {
                event.rename("json.netbios_names", "tenable_io.asset.netbios_names")?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.netbios_names")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if event.has_value("host.name") {
                map_strings(event, "host.name", "host.name", str::to_lowercase)?;
            }

            if event.has_value("json.operating_systems") {
                event.rename(
                    "json.operating_systems",
                    "tenable_io.asset.operating_systems",
                )?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.operating_systems")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            let _cond = { event.get_str("json.ratings.acr.score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ratings.acr.score") {
                        if let Some(val) = event.get("json.ratings.acr.score") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ratings.acr.score".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.asset.ratings.acr.score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_acr_score_to_long",
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

            let _cond = { event.get_str("json.ratings.aes.score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ratings.aes.score") {
                        if let Some(val) = event.get("json.ratings.aes.score") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ratings.aes.score".into(),
                                    message,
                                }
                            })?;
                            event.set("tenable_io.asset.ratings.aes.score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_aes_score_to_long",
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

            if event.has_value("json.system_types") {
                event.rename("json.system_types", "tenable_io.asset.system_types")?;
            }

            if event.has_value("json.hostnames") {
                event.rename("json.hostnames", "tenable_io.asset.hostnames")?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.hostnames")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if event.has_value("json.serial_number") {
                event.rename("json.serial_number", "tenable_io.asset.serial_number")?;
            }

            if event.has_value("json.ssh_fingerprints") {
                event.rename("json.ssh_fingerprints", "tenable_io.asset.ssh_fingerprints")?;
            }

            if event.has_value("json.qualys_asset_ids") {
                event.rename("json.qualys_asset_ids", "tenable_io.asset.qualys.asset_ids")?;
            }

            if event.has_value("json.qualys_host_ids") {
                event.rename("json.qualys_host_ids", "tenable_io.asset.qualys.host_ids")?;
            }

            if event.has_value("json.manufacturer_tpm_ids") {
                event.rename(
                    "json.manufacturer_tpm_ids",
                    "tenable_io.asset.manufacturer_tpm_ids",
                )?;
            }

            if event.has_value("json.symantec_ep_hardware_keys") {
                event.rename(
                    "json.symantec_ep_hardware_keys",
                    "tenable_io.asset.symantec_ep_hardware_keys",
                )?;
            }

            let _cond = { event.get("json.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.sources").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.first_seen")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.first_seen", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.first_seen".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.remove("_ingest._value.first_seen");
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
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "json.sources",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get("json.sources").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.sources").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.last_seen")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last_seen", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.last_seen".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.remove("_ingest._value.last_seen");
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
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "json.sources",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("json.sources") {
                event.rename("json.sources", "tenable_io.asset.sources")?;
            }

            let _cond = { event.get("json.tags").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.tags").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.added_at")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.added_at", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.added_at".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.remove("_ingest._value.added_at");
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
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "json.tags",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("json.tags") {
                event.rename("json.tags", "tenable_io.asset.tags")?;
            }

            let _cond = {
                event
                    .get("json.network_interfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.network_interfaces", |event| {
                    if event.has_value("_ingest._value.ipv4s") {
                        foreach_array(event, "_ingest._value.ipv4s", |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
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
                                event.remove("_ingest._value");
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
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.network_interfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.network_interfaces", |event| {
                    if event.has_value("_ingest._value.ipv4s") {
                        foreach_array(event, "_ingest._value.ipv4s", |event| {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.network_interfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.network_interfaces", |event| {
                    if event.has_value("_ingest._value.ipv6s") {
                        foreach_array(event, "_ingest._value.ipv6s", |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
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
                                event.remove("_ingest._value");
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
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.network_interfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.network_interfaces", |event| {
                    if event.has_value("_ingest._value.ipv6s") {
                        foreach_array(event, "_ingest._value.ipv6s", |event| {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.network_interfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.network_interfaces", |event| {
                    if event.has_value("_ingest._value.fqdns") {
                        foreach_array(event, "_ingest._value.fqdns", |event| {
                            event.append_unique(
                                "related.hosts",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.network_interfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.network_interfaces", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.aliased") {
                            if let Some(val) = event.get("_ingest._value.aliased") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.aliased".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.aliased", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("_ingest._value.aliased");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.network_interfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.network_interfaces", |event| {
                    if event.has_value("_ingest._value.mac_addresses") {
                        foreach_array(event, "_ingest._value.mac_addresses", |event| {
                            if event.has_value("_ingest._value") {
                                gsub_field(
                                    event,
                                    "_ingest._value",
                                    "_ingest._value",
                                    cached_regex!("[:.]"),
                                    "-",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.network_interfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.network_interfaces", |event| {
                    if event.has_value("_ingest._value.mac_addresses") {
                        foreach_array(event, "_ingest._value.mac_addresses", |event| {
                            if event.has_value("_ingest._value") {
                                map_strings(
                                    event,
                                    "_ingest._value",
                                    "_ingest._value",
                                    str::to_uppercase,
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.network_interfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.network_interfaces", |event| {
                    if event.has_value("_ingest._value.mac_addresses") {
                        foreach_array(event, "_ingest._value.mac_addresses", |event| {
                            event.append_unique(
                                "host.mac",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.network_interfaces") {
                event.rename(
                    "json.network_interfaces",
                    "tenable_io.asset.network_interfaces",
                )?;
            }

            let _cond = {
                event.has_value("json.acr_score") && event.get_str("json.acr_score") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.json.acr_score instanceof String) {\n  try {\n    long acr_score = Long.parseLong(ctx.json.acr_score);\n    ctx.tenable_io.asset.acr_score = acr_score;\n  } catch (NumberFormatException e) {\n    double acr_score = Double.parseDouble(ctx.json.acr_score);\n    ctx.tenable_io.asset.acr_score = (long) acr_score; \n  }\n  return;\n} if (ctx.json.acr_score instanceof int || ctx.json.acr_score instanceof long) {\n  ctx.tenable_io.asset.acr_score = (long) ctx.json.acr_score;\n  return;\n} if (ctx.json.acr_score instanceof double) {\n  ctx.tenable_io.asset.acr_score = (long) ctx.json.acr_score;\n  return;\n} \n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.json.acr_score instanceof String) {\n  try {\n    long acr_score = Long.parseLong(ctx.json.acr_score);\n    ctx.tenable_io.asset.acr_score = acr_score;\n  } catch (NumberFormatException e) {\n    double acr_score = Double.parseDouble(ctx.json.acr_score);\n    ctx.tenable_io.asset.acr_score = (long) acr_score; \n  }\n  return;\n} if (ctx.json.acr_score instanceof int || ctx.json.acr_score instanceof long) {\n  ctx.tenable_io.asset.acr_score = (long) ctx.json.acr_score;\n  return;\n} if (ctx.json.acr_score instanceof double) {\n  ctx.tenable_io.asset.acr_score = (long) ctx.json.acr_score;\n  return;\n} \n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "acr_score_is_long")?;
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
                event.has_value("json.exposure_score")
                    && event.get_str("json.exposure_score") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.json.exposure_score instanceof String) {\n  try {\n    long exposure_score = Long.parseLong(ctx.json.exposure_score);\n    ctx.tenable_io.asset.exposure_score = exposure_score;\n  } catch (Exception e) {\n    double exposure_score = Double.parseDouble(ctx.json.exposure_score);\n    ctx.tenable_io.asset.exposure_score = (long) exposure_score;\n  }\n  return;\n} if (ctx.json.exposure_score instanceof int || ctx.json.exposure_score instanceof long) {\n  ctx.tenable_io.asset.exposure_score = (long) ctx.json.exposure_score;\n  return;\n}  if (ctx.json.exposure_score instanceof double) {\n  ctx.tenable_io.asset.exposure_score = (long) ctx.json.exposure_score;\n  return;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.json.exposure_score instanceof String) {\n  try {\n    long exposure_score = Long.parseLong(ctx.json.exposure_score);\n    ctx.tenable_io.asset.exposure_score = exposure_score;\n  } catch (Exception e) {\n    double exposure_score = Double.parseDouble(ctx.json.exposure_score);\n    ctx.tenable_io.asset.exposure_score = (long) exposure_score;\n  }\n  return;\n} if (ctx.json.exposure_score instanceof int || ctx.json.exposure_score instanceof long) {\n  ctx.tenable_io.asset.exposure_score = (long) ctx.json.exposure_score;\n  return;\n}  if (ctx.json.exposure_score instanceof double) {\n  ctx.tenable_io.asset.exposure_score = (long) ctx.json.exposure_score;\n  return;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "exposure_score_is_long")?;
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

            if event.has_value("json.network_id") {
                event.rename("json.network_id", "tenable_io.asset.network.id")?;
            }

            if event.has_value("json.network_name") {
                event.rename("json.network_name", "tenable_io.asset.network.name")?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.network.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.name", v)?;
            }

            if event.has_value("json.gcp_zone") {
                event.rename("json.gcp_zone", "tenable_io.asset.gcp.zone")?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.gcp.zone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.availability_zone", v)?;
            }

            if event.has_value("json.aws_availability_zone") {
                event.rename(
                    "json.aws_availability_zone",
                    "tenable_io.asset.aws.availability_zone",
                )?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.aws.availability_zone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.availability_zone", v)?;
            }

            if event.has_value("json.azure_vm_id") {
                event.rename("json.azure_vm_id", "tenable_io.asset.azure.vm_id")?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.azure.vm_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.id", v)?;
            }

            if event.has_value("json.gcp_instance_id") {
                event.rename("json.gcp_instance_id", "tenable_io.asset.gcp.instance_id")?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.gcp.instance_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.id", v)?;
            }

            if event.has_value("json.aws_ec2_instance_ami_id") {
                event.rename(
                    "json.aws_ec2_instance_ami_id",
                    "tenable_io.asset.aws.ec2_instance.ami_id",
                )?;
            }

            if event.has_value("json.aws_ec2_instance_id") {
                event.rename(
                    "json.aws_ec2_instance_id",
                    "tenable_io.asset.aws.ec2_instance.id",
                )?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.aws.ec2_instance.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.id", v)?;
            }

            if event.has_value("json.aws_ec2_instance_type") {
                event.rename(
                    "json.aws_ec2_instance_type",
                    "tenable_io.asset.aws.ec2_instance.type",
                )?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.aws.ec2_instance.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.machine.type", v)?;
            }

            if event.has_value("json.gcp_project_id") {
                event.rename("json.gcp_project_id", "tenable_io.asset.gcp.project_id")?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.gcp.project_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.project.id", v)?;
            }

            if event.has_value("json.aws_region") {
                event.rename("json.aws_region", "tenable_io.asset.aws.region")?;
            }

            if let Some(v) = event
                .get("tenable_io.asset.aws.region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "host.ip", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("host.name").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "host.name", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("host.hostname").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "host.hostname", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("host.domain").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "host.domain", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
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
                event.remove("tenable_io.asset.updated_at");
                event.remove("tenable_io.asset.gcp.zone");
                event.remove("tenable_io.asset.aws.availability_zone");
                event.remove("tenable_io.asset.azure.vm_id");
                event.remove("tenable_io.asset.gcp.instance_id");
                event.remove("tenable_io.asset.aws.ec2_instance.id");
                event.remove("tenable_io.asset.aws.ec2_instance.type");
                event.remove("tenable_io.asset.gcp.project_id");
                event.remove("tenable_io.asset.aws.region");
                event.remove("tenable_io.asset.netbios_names");
                event.remove("tenable_io.asset.hostnames");
                event.remove("tenable_io.asset.id");
                event.remove("tenable_io.asset.ipv4s");
                event.remove("tenable_io.asset.ipv6s");
                event.remove("tenable_io.asset.mac_addresses");
                event.remove("tenable_io.asset.fqdns");
                event.remove("tenable_io.asset.operating_systems");
                event.remove("tenable_io.asset.network.name");
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
