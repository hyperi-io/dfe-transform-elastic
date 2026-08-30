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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

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

            event.set("event.kind", json!("alert"))?;

            event.set("observer.vendor", json!("Claroty"))?;

            event.set("observer.product", json!("xDome"))?;

            if event.has_value("json.alert_info.category") {
                event.rename("json.alert_info.category", "claroty_xdome.alert.category")?;
            }

            if event.has_value("json.alert_info.alert_class") {
                event.rename("json.alert_info.alert_class", "claroty_xdome.alert.class")?;
            }

            if event.has_value("json.alert_info.description") {
                event.rename(
                    "json.alert_info.description",
                    "claroty_xdome.alert.description",
                )?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = {
                event.has_value("json.alert_info.detected_time")
                    && event.get_str("json.alert_info.detected_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.alert_info.detected_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("claroty_xdome.alert.detected_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.alert_info.detected_time".into(),
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
                        "date_alert_info_detected_time",
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

            if event.has_value("json.active_queries_seen_reported_from") {
                event.rename(
                    "json.active_queries_seen_reported_from",
                    "claroty_xdome.alert.device.active_queries_seen_reported_from",
                )?;
            }

            let _cond = { event.get_str("json.activity_rate") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.activity_rate") {
                        if let Some(val) = event.get("json.activity_rate") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.activity_rate".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.device.activity_rate", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_activity_rate_to_double",
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

            if event.has_value("json.ad_description") {
                event.rename(
                    "json.ad_description",
                    "claroty_xdome.alert.device.ad.description",
                )?;
            }

            if event.has_value("json.ad_distinguished_name") {
                event.rename(
                    "json.ad_distinguished_name",
                    "claroty_xdome.alert.device.ad.distinguished_name",
                )?;
            }

            if event.has_value("json.ae_titles") {
                event.rename("json.ae_titles", "claroty_xdome.alert.device.ae_titles")?;
            }

            if event.has_value("json.ap_location_list") {
                event.rename(
                    "json.ap_location_list",
                    "claroty_xdome.alert.device.ap.location_list",
                )?;
            }

            if event.has_value("json.ap_name_list") {
                event.rename(
                    "json.ap_name_list",
                    "claroty_xdome.alert.device.ap.name_list",
                )?;
            }

            if event.has_value("json.applied_acl_list") {
                event.rename(
                    "json.applied_acl_list",
                    "claroty_xdome.alert.device.applied_acl.list",
                )?;
            }

            if event.has_value("json.applied_acl_type_list") {
                event.rename(
                    "json.applied_acl_type_list",
                    "claroty_xdome.alert.device.applied_acl.type_list",
                )?;
            }

            if event.has_value("json.asset_id") {
                event.rename("json.asset_id", "claroty_xdome.alert.device.asset_id")?;
            }

            if event.has_value("json.assignees") {
                event.rename("json.assignees", "claroty_xdome.alert.device.assignees")?;
            }

            let _cond = {
                event
                    .get("claroty_xdome.alert.device.assignees")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_xdome.alert.device.assignees", |event| {
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

            let _cond = {
                event
                    .get("json.assignees_data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.assignees_data", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.id") {
                            if let Some(val) = event.get("_ingest._value.id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_assignees_data_to_string",
                        )?;
                        event.remove("_ingest._value.id");
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

            let _cond = {
                event
                    .get("json.assignees_data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.assignees_data", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.assignees_data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.assignees_data", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.display_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.assignees_data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.assignees_data", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.is_active") {
                            if let Some(val) = event.get("_ingest._value.is_active") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.is_active".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.is_active", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_assignees_data_is_Active_to_boolean",
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

            if event.has_value("json.assignees_data") {
                event.rename(
                    "json.assignees_data",
                    "claroty_xdome.alert.device.assignees_data",
                )?;
            }

            if event.has_value("json.authentication_user_list") {
                event.rename(
                    "json.authentication_user_list",
                    "claroty_xdome.alert.device.authentication_user_list",
                )?;
            }

            let _cond = { event.get_str("json.avg_examinations_per_day") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.avg_examinations_per_day") {
                        if let Some(val) = event.get("json.avg_examinations_per_day") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.avg_examinations_per_day".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "claroty_xdome.alert.device.avg.examinations_per_day",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_avg_examinations_per_day_to_double",
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

            let _cond = { event.get_str("json.avg_in_use_per_day") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.avg_in_use_per_day") {
                        if let Some(val) = event.get("json.avg_in_use_per_day") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.avg_in_use_per_day".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("claroty_xdome.alert.device.avg.in_use_per_day", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_avg_in_use_per_day_to_double",
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

            let _cond = { event.get_str("json.avg_online_per_day") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.avg_online_per_day") {
                        if let Some(val) = event.get("json.avg_online_per_day") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.avg_online_per_day".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("claroty_xdome.alert.device.avg.online_per_day", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_avg_online_per_day_to_double",
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

            if event.has_value("json.battery_level") {
                event.rename(
                    "json.battery_level",
                    "claroty_xdome.alert.device.battery_level",
                )?;
            }

            if event.has_value("json.bssid_list") {
                event.rename("json.bssid_list", "claroty_xdome.alert.device.bssid_list")?;
            }

            if event.has_value("json.device_category") {
                event.rename(
                    "json.device_category",
                    "claroty_xdome.alert.device.category",
                )?;
            }

            let _cond = { event.get_str("json.cmms_asset_purchase_cost") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.cmms_asset_purchase_cost") {
                        if let Some(val) = event.get("json.cmms_asset_purchase_cost") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.cmms_asset_purchase_cost".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "claroty_xdome.alert.device.cmms.asset.purchase_cost",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cmms_asset_purchase_cost_to_double",
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

            if event.has_value("json.cmms_asset_tag") {
                event.rename(
                    "json.cmms_asset_tag",
                    "claroty_xdome.alert.device.cmms.asset.tag",
                )?;
            }

            if event.has_value("json.cmms_building") {
                event.rename(
                    "json.cmms_building",
                    "claroty_xdome.alert.device.cmms.building",
                )?;
            }

            if event.has_value("json.cmms_campus") {
                event.rename("json.cmms_campus", "claroty_xdome.alert.device.cmms.campus")?;
            }

            if event.has_value("json.cmms_department") {
                event.rename(
                    "json.cmms_department",
                    "claroty_xdome.alert.device.cmms.department",
                )?;
            }

            let _cond = { event.get_str("json.cmms_financial_cost") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.cmms_financial_cost") {
                        if let Some(val) = event.get("json.cmms_financial_cost") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.cmms_financial_cost".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("claroty_xdome.alert.device.cmms.financial_cost", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cmms_financial_cost_to_double",
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

            if event.has_value("json.cmms_floor") {
                event.rename("json.cmms_floor", "claroty_xdome.alert.device.cmms.floor")?;
            }

            if event.has_value("json.cmms_last_pm") {
                event.rename(
                    "json.cmms_last_pm",
                    "claroty_xdome.alert.device.cmms.last_pm",
                )?;
            }

            if event.has_value("json.cmms_location") {
                event.rename(
                    "json.cmms_location",
                    "claroty_xdome.alert.device.cmms.location",
                )?;
            }

            if event.has_value("json.cmms_manufacturer") {
                event.rename(
                    "json.cmms_manufacturer",
                    "claroty_xdome.alert.device.cmms.manufacturer",
                )?;
            }

            if event.has_value("json.cmms_model") {
                event.rename("json.cmms_model", "claroty_xdome.alert.device.cmms.model")?;
            }

            if event.has_value("json.cmms_ownership") {
                event.rename(
                    "json.cmms_ownership",
                    "claroty_xdome.alert.device.cmms.ownership",
                )?;
            }

            if event.has_value("json.cmms_owning_cost_center") {
                event.rename(
                    "json.cmms_owning_cost_center",
                    "claroty_xdome.alert.device.cmms.owning_cost_center",
                )?;
            }

            if event.has_value("json.cmms_room") {
                event.rename("json.cmms_room", "claroty_xdome.alert.device.cmms.room")?;
            }

            if event.has_value("json.cmms_serial_number") {
                event.rename(
                    "json.cmms_serial_number",
                    "claroty_xdome.alert.device.cmms.serial_number",
                )?;
            }

            if event.has_value("json.cmms_state") {
                event.rename("json.cmms_state", "claroty_xdome.alert.device.cmms.state")?;
            }

            if event.has_value("json.cmms_technician") {
                event.rename(
                    "json.cmms_technician",
                    "claroty_xdome.alert.device.cmms.technician",
                )?;
            }

            if event.has_value("json.collection_interfaces_seen_reported_from") {
                event.rename(
                    "json.collection_interfaces_seen_reported_from",
                    "claroty_xdome.alert.device.collection.interfaces.seen_reported_from",
                )?;
            }

            if event.has_value("json.collection_interfaces") {
                event.rename(
                    "json.collection_interfaces",
                    "claroty_xdome.alert.device.collection.interfaces.value",
                )?;
            }

            if event.has_value("json.collection_servers_seen_reported_from") {
                event.rename(
                    "json.collection_servers_seen_reported_from",
                    "claroty_xdome.alert.device.collection.servers.seen_reported_from",
                )?;
            }

            if event.has_value("json.collection_servers") {
                event.rename(
                    "json.collection_servers",
                    "claroty_xdome.alert.device.collection.servers.value",
                )?;
            }

            if event.has_value("json.combined_os") {
                event.rename("json.combined_os", "claroty_xdome.alert.device.combined_os")?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.combined_os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.combined_os") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.combined_os")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.connection_paths") {
                event.rename(
                    "json.connection_paths",
                    "claroty_xdome.alert.device.connection.paths",
                )?;
            }

            if event.has_value("json.connection_type_list") {
                event.rename(
                    "json.connection_type_list",
                    "claroty_xdome.alert.device.connection.type_list",
                )?;
            }

            if event.has_value("json.consequence_of_failure") {
                event.rename(
                    "json.consequence_of_failure",
                    "claroty_xdome.alert.device.consequence_of_failure",
                )?;
            }

            if event.has_value("json.cppm_authentication_status_list") {
                event.rename(
                    "json.cppm_authentication_status_list",
                    "claroty_xdome.alert.device.cppm.authentication_status_list",
                )?;
            }

            if event.has_value("json.cppm_roles_list") {
                event.rename(
                    "json.cppm_roles_list",
                    "claroty_xdome.alert.device.cppm.roles_list",
                )?;
            }

            if event.has_value("json.cppm_service_list") {
                event.rename(
                    "json.cppm_service_list",
                    "claroty_xdome.alert.device.cppm.service_list",
                )?;
            }

            if event.has_value("json.data_sources_seen_reported_from") {
                event.rename(
                    "json.data_sources_seen_reported_from",
                    "claroty_xdome.alert.device.data_sources_seen_reported_from",
                )?;
            }

            if event.has_value("json.dhcp_fingerprint") {
                event.rename(
                    "json.dhcp_fingerprint",
                    "claroty_xdome.alert.device.dhcp.fingerprint",
                )?;
            }

            let _cond = {
                event
                    .get("json.dhcp_hostnames")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.dhcp_hostnames", |event| {
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

            if event.has_value("json.dhcp_hostnames") {
                event.rename(
                    "json.dhcp_hostnames",
                    "claroty_xdome.alert.device.dhcp.hostnames",
                )?;
            }

            if event.has_value("json.dhcp_last_seen_hostname") {
                event.rename(
                    "json.dhcp_last_seen_hostname",
                    "claroty_xdome.alert.device.dhcp.last_seen_hostname",
                )?;
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.dhcp.last_seen_hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.dhcp.last_seen_hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get("json.domains").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.domains", |event| {
                    event.append_unique(
                        "host.domain",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.domains").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.domains", |event| {
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

            if event.has_value("json.domains") {
                event.rename("json.domains", "claroty_xdome.alert.device.domains")?;
            }

            if event.has_value("json.edge_hosts_seen_reported_from") {
                event.rename(
                    "json.edge_hosts_seen_reported_from",
                    "claroty_xdome.alert.device.edge.hosts_seen_reported_from",
                )?;
            }

            if event.has_value("json.edge_locations") {
                event.rename(
                    "json.edge_locations",
                    "claroty_xdome.alert.device.edge.locations",
                )?;
            }

            if event.has_value("json.edge_locations_seen_reported_from") {
                event.rename(
                    "json.edge_locations_seen_reported_from",
                    "claroty_xdome.alert.device.edge.locations_seen_reported_from",
                )?;
            }

            if event.has_value("json.edr_is_up_to_date_text") {
                event.rename(
                    "json.edr_is_up_to_date_text",
                    "claroty_xdome.alert.device.edr.is_up_to_date_text",
                )?;
            }

            let _cond = {
                event.has_value("json.edr_last_scan_time")
                    && event.get_str("json.edr_last_scan_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.edr_last_scan_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("claroty_xdome.alert.device.edr.last_scan_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.edr_last_scan_time".into(),
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
                        "date_edr_last_scan_time",
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

            let _cond = { event.get_str("json.effective_likelihood_subscore_points") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.effective_likelihood_subscore_points") {
                        if let Some(val) = event.get("json.effective_likelihood_subscore_points") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.effective_likelihood_subscore_points".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "claroty_xdome.alert.device.effective_likelihood_subscore.points",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_effective_likelihood_subscore_points_to_double",
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

            if event.has_value("json.effective_likelihood_subscore") {
                event.rename(
                    "json.effective_likelihood_subscore",
                    "claroty_xdome.alert.device.effective_likelihood_subscore.value",
                )?;
            }

            let _cond = {
                event.has_value("json.end_of_life_date")
                    && event.get_str("json.end_of_life_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.end_of_life_date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("claroty_xdome.alert.device.end_of.life.date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.end_of_life_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_end_of_life_date")?;
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

            if event.has_value("json.end_of_life_state") {
                event.rename(
                    "json.end_of_life_state",
                    "claroty_xdome.alert.device.end_of.life.state",
                )?;
            }

            let _cond = {
                event.has_value("json.end_of_sale_date")
                    && event.get_str("json.end_of_sale_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.end_of_sale_date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("claroty_xdome.alert.device.end_of.sale_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.end_of_sale_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_end_of_sale_date")?;
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

            if event.has_value("json.endpoint_security_names") {
                event.rename(
                    "json.endpoint_security_names",
                    "claroty_xdome.alert.device.endpoint_security_names",
                )?;
            }

            if event.has_value("json.enforcement_or_authorization_profiles_list") {
                event.rename(
                    "json.enforcement_or_authorization_profiles_list",
                    "claroty_xdome.alert.device.enforcement_or_authorization_profiles_list",
                )?;
            }

            if event.has_value("json.equipment_class") {
                event.rename(
                    "json.equipment_class",
                    "claroty_xdome.alert.device.equipment_class",
                )?;
            }

            if event.has_value("json.fda_class") {
                event.rename("json.fda_class", "claroty_xdome.alert.device.fda_class")?;
            }

            if event.has_value("json.financial_cost") {
                event.rename(
                    "json.financial_cost",
                    "claroty_xdome.alert.device.financial_cost",
                )?;
            }

            let _cond = {
                event
                    .get("json.first_seen_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.first_seen_list").cloned();
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_ingest._value") {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => event.set("_ingest._value", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value".into(),
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
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_first_seen_list",
                                )?;
                                event.remove("_ingest._value");
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
                            "json.first_seen_list",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("json.first_seen_list") {
                event.rename(
                    "json.first_seen_list",
                    "claroty_xdome.alert.device.first_seen_list",
                )?;
            }

            if event.has_value("json.handles_pii") {
                event.rename("json.handles_pii", "claroty_xdome.alert.device.handles_pii")?;
            }

            if event.has_value("json.http_hostnames") {
                event.rename(
                    "json.http_hostnames",
                    "claroty_xdome.alert.device.http.hostnames",
                )?;
            }

            let _cond = {
                event
                    .get("claroty_xdome.alert.device.http.hostnames")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "claroty_xdome.alert.device.http.hostnames",
                    |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.http_last_seen_hostname") {
                event.rename(
                    "json.http_last_seen_hostname",
                    "claroty_xdome.alert.device.http.last_seen_hostname",
                )?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.http.last_seen_hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.http.last_seen_hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.http.last_seen_hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.hw_version") {
                event.rename("json.hw_version", "claroty_xdome.alert.device.hw_version")?;
            }

            let _cond = { event.get_str("json.impact_subscore_points") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.impact_subscore_points") {
                        if let Some(val) = event.get("json.impact_subscore_points") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.impact_subscore_points".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "claroty_xdome.alert.device.impact_subscore.points",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_impact_subscore_points_to_double",
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

            if event.has_value("json.impact_subscore") {
                event.rename(
                    "json.impact_subscore",
                    "claroty_xdome.alert.device.impact_subscore.value",
                )?;
            }

            let _cond = { event.get_str("json.insecure_protocols_points") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.insecure_protocols_points") {
                        if let Some(val) = event.get("json.insecure_protocols_points") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.insecure_protocols_points".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "claroty_xdome.alert.device.insecure_protocols.points",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_insecure_protocols_points_to_double",
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

            if event.has_value("json.insecure_protocols") {
                event.rename(
                    "json.insecure_protocols",
                    "claroty_xdome.alert.device.insecure_protocols.value",
                )?;
            }

            if event.has_value("json.integration_types_reported_from") {
                event.rename(
                    "json.integration_types_reported_from",
                    "claroty_xdome.alert.device.integration_types_reported_from",
                )?;
            }

            if event.has_value("json.integrations_reported_from") {
                event.rename(
                    "json.integrations_reported_from",
                    "claroty_xdome.alert.device.integrations_reported_from",
                )?;
            }

            if event.has_value("json.internet_communication") {
                if let Some(val) = event.get("json.internet_communication") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.internet_communication".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "claroty_xdome.alert.device.internet_communication",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.ip_assignment_list") {
                event.rename(
                    "json.ip_assignment_list",
                    "claroty_xdome.alert.device.ip.assignment_list",
                )?;
            }

            if event.has_value("json.ip_list") {
                event.rename("json.ip_list", "claroty_xdome.alert.device.ip.list")?;
            }

            let _cond = {
                event
                    .get("claroty_xdome.alert.device.ip.list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_xdome.alert.device.ip.list", |event| {
                    // on_failure: 1 handler(s)
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
                        event.set("_ingest.on_failure_processor_tag", "convert_ip_list_to_ip")?;
                        event.remove("_ingest._value");
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
                    .get("claroty_xdome.alert.device.ip.list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_xdome.alert.device.ip.list", |event| {
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

            let _cond = { event.get_str("json.device_name") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.device_name") {
                        if let Some(val) = event.get("json.device_name") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.device_name".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.device.ip.value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_device_name_to_ip",
                    )?;
                    if event.has_value("json.device_name") {
                        event.rename("json.device_name", "claroty_xdome.alert.device.name")?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.ip.value") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.ip.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.ip.value") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.ip.value")
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

            let _cond = { event.get_str("json.is_online") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.is_online") {
                        if let Some(val) = event.get("json.is_online") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.is_online".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.device.is_online", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_is_online_to_boolean",
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

            let _cond = { event.get_str("json.is_resolved") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.is_resolved") {
                        if let Some(val) = event.get("json.is_resolved") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.is_resolved".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.device.is_resolved", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_is_resolved_to_boolean",
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

            if event.has_value("json.ise_authentication_method_list") {
                event.rename(
                    "json.ise_authentication_method_list",
                    "claroty_xdome.alert.device.ise.authentication_method_list",
                )?;
            }

            if event.has_value("json.ise_endpoint_profile_list") {
                event.rename(
                    "json.ise_endpoint_profile_list",
                    "claroty_xdome.alert.device.ise.endpoint_profile_list",
                )?;
            }

            if event.has_value("json.ise_identity_group_list") {
                event.rename(
                    "json.ise_identity_group_list",
                    "claroty_xdome.alert.device.ise.identity_group_list",
                )?;
            }

            if event.has_value("json.ise_logical_profile_list") {
                event.rename(
                    "json.ise_logical_profile_list",
                    "claroty_xdome.alert.device.ise.logical_profile_list",
                )?;
            }

            if event.has_value("json.ise_security_group_description_list") {
                event.rename(
                    "json.ise_security_group_description_list",
                    "claroty_xdome.alert.device.ise.security_group.description_list",
                )?;
            }

            if event.has_value("json.ise_security_group_name_list") {
                event.rename(
                    "json.ise_security_group_name_list",
                    "claroty_xdome.alert.device.ise.security_group.name_list",
                )?;
            }

            let _cond = {
                event
                    .get("json.ise_security_group_tag_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.ise_security_group_tag_list", |event| {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.ise_security_group_tag_list") {
                event.rename(
                    "json.ise_security_group_tag_list",
                    "claroty_xdome.alert.device.ise.security_group.tag_list",
                )?;
            }

            let _cond = { event.get_str("json.known_vulnerabilities_points") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.known_vulnerabilities_points") {
                        if let Some(val) = event.get("json.known_vulnerabilities_points") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.known_vulnerabilities_points".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "claroty_xdome.alert.device.known_vulnerabilities.points",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_known_vulnerabilities_points_to_double",
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

            if event.has_value("json.known_vulnerabilities") {
                event.rename(
                    "json.known_vulnerabilities",
                    "claroty_xdome.alert.device.known_vulnerabilities.value",
                )?;
            }

            if event.has_value("json.labels") {
                event.rename("json.labels", "claroty_xdome.alert.device.labels")?;
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
                            Some(parsed) => {
                                event.set("claroty_xdome.alert.device.last.scan_time", parsed)?
                            }
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
                    event.set("_ingest.on_failure_processor_tag", "date_last_scan_time")?;
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
                event
                    .get("json.last_seen_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.last_seen_list").cloned();
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_ingest._value") {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => event.set("_ingest._value", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value".into(),
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
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_last_seen_list",
                                )?;
                                event.remove("_ingest._value");
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
                            "json.last_seen_list",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("json.last_seen_list") {
                event.rename(
                    "json.last_seen_list",
                    "claroty_xdome.alert.device.last.seen.list",
                )?;
            }

            let _cond = {
                event
                    .get("json.last_seen_on_switch_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.last_seen_on_switch_list").cloned();
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_ingest._value") {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => event.set("_ingest._value", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value".into(),
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
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_last_seen_on_switch_list",
                                )?;
                                event.remove("_ingest._value");
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
                            "json.last_seen_on_switch_list",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("json.last_seen_on_switch_list") {
                event.rename(
                    "json.last_seen_on_switch_list",
                    "claroty_xdome.alert.device.last.seen.on_switch_list",
                )?;
            }

            let _cond = {
                event.has_value("json.last_seen_reported")
                    && event.get_str("json.last_seen_reported") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_seen_reported") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("claroty_xdome.alert.device.last.seen.reported", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_seen_reported".into(),
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
                        "date_last_seen_reported",
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
                event.has_value("json.last_domain_user_activity")
                    && event.get_str("json.last_domain_user_activity") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_domain_user_activity") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "claroty_xdome.alert.device.last_domain_user.activity",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_domain_user_activity".into(),
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
                        "date_last_domain_user_activity",
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

            if event.has_value("json.last_domain_user") {
                event.rename(
                    "json.last_domain_user",
                    "claroty_xdome.alert.device.last_domain_user.name",
                )?;
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.last_domain_user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.last_domain_user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.likelihood_subscore_points") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.likelihood_subscore_points") {
                        if let Some(val) = event.get("json.likelihood_subscore_points") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.likelihood_subscore_points".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "claroty_xdome.alert.device.likelihood_subscore.points",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_likelihood_subscore_points_to_double",
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

            if event.has_value("json.likelihood_subscore") {
                event.rename(
                    "json.likelihood_subscore",
                    "claroty_xdome.alert.device.likelihood_subscore.value",
                )?;
            }

            if event.has_value("json.local_name") {
                event.rename("json.local_name", "claroty_xdome.alert.device.local_name")?;
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.local_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.local_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get("json.mac_list").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.mac_list", |event| {
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("host.mac") {
                    gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "-")?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_host_mac")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("host.mac") {
                    map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uppercase")?;
                event.set("_ingest.on_failure_processor_tag", "uppercase_host_mac")?;
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

            if event.has_value("json.mac_list") {
                event.rename("json.mac_list", "claroty_xdome.alert.device.mac.list")?;
            }

            if event.has_value("json.mac_oui_list") {
                event.rename(
                    "json.mac_oui_list",
                    "claroty_xdome.alert.device.mac.oui_list",
                )?;
            }

            if event.has_value("json.machine_type") {
                event.rename(
                    "json.machine_type",
                    "claroty_xdome.alert.device.machine_type",
                )?;
            }

            if event.has_value("json.managed_by") {
                event.rename("json.managed_by", "claroty_xdome.alert.device.managed_by")?;
            }

            if event.has_value("json.management_services") {
                event.rename(
                    "json.management_services",
                    "claroty_xdome.alert.device.management_services",
                )?;
            }

            if event.has_value("json.manufacturer") {
                event.rename(
                    "json.manufacturer",
                    "claroty_xdome.alert.device.manufacturer",
                )?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.manufacturer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.manufacturer", v)?;
            }

            if event.has_value("json.mdm_compliance_status") {
                event.rename(
                    "json.mdm_compliance_status",
                    "claroty_xdome.alert.device.mdm.compliance_status",
                )?;
            }

            if event.has_value("json.mdm_enrollment_status") {
                event.rename(
                    "json.mdm_enrollment_status",
                    "claroty_xdome.alert.device.mdm.enrollment_status",
                )?;
            }

            if event.has_value("json.mdm_ownership") {
                event.rename(
                    "json.mdm_ownership",
                    "claroty_xdome.alert.device.mdm.ownership",
                )?;
            }

            if event.has_value("json.mobility") {
                event.rename("json.mobility", "claroty_xdome.alert.device.mobility")?;
            }

            if event.has_value("json.model_family") {
                event.rename(
                    "json.model_family",
                    "claroty_xdome.alert.device.model.family",
                )?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.model.family")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.model.name", v)?;
            }

            if event.has_value("json.model") {
                event.rename("json.model", "claroty_xdome.alert.device.model.name")?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.model.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.model.identifier", v)?;
            }

            if event.has_value("json.network_scope_list") {
                event.rename(
                    "json.network_scope_list",
                    "claroty_xdome.alert.device.network.scope_list",
                )?;
            }

            if event.has_value("json.network_list") {
                event.rename(
                    "json.network_list",
                    "claroty_xdome.alert.device.network_list",
                )?;
            }

            if event.has_value("json.note") {
                event.rename("json.note", "claroty_xdome.alert.device.note")?;
            }

            let _cond = { event.get_str("json.number_of_nics") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.number_of_nics") {
                        if let Some(val) = event.get("json.number_of_nics") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.number_of_nics".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.device.number_of_nics", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_number_of_nics_to_long",
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

            if event.has_value("json.operating_hours_pattern_name") {
                event.rename(
                    "json.operating_hours_pattern_name",
                    "claroty_xdome.alert.device.operating_hours_pattern_name",
                )?;
            }

            if event.has_value("json.organization_firewall_group_name") {
                event.rename(
                    "json.organization_firewall_group_name",
                    "claroty_xdome.alert.device.organization.firewall_group_name",
                )?;
            }

            if event.has_value("json.organization_zone_name") {
                event.rename(
                    "json.organization_zone_name",
                    "claroty_xdome.alert.device.organization.zone_name",
                )?;
            }

            if event.has_value("json.os_category") {
                event.rename("json.os_category", "claroty_xdome.alert.device.os.category")?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.os.category")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.family", v)?;
            }

            let _cond = {
                event.has_value("json.os_eol_date") && event.get_str("json.os_eol_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.os_eol_date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("claroty_xdome.alert.device.os.eol_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.os_eol_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_os_eol_date")?;
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

            if event.has_value("json.os_name") {
                event.rename("json.os_name", "claroty_xdome.alert.device.os.name")?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.os.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.os.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.os.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.os_revision") {
                event.rename("json.os_revision", "claroty_xdome.alert.device.os.revision")?;
            }

            if event.has_value("json.os_subcategory") {
                event.rename(
                    "json.os_subcategory",
                    "claroty_xdome.alert.device.os.subcategory",
                )?;
            }

            if event.has_value("json.os_version") {
                event.rename("json.os_version", "claroty_xdome.alert.device.os.version")?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.os.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if event.has_value("json.other_hostnames") {
                event.rename(
                    "json.other_hostnames",
                    "claroty_xdome.alert.device.other_hostnames",
                )?;
            }

            let _cond = {
                event
                    .get("claroty_xdome.alert.device.other_hostnames")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "claroty_xdome.alert.device.other_hostnames",
                    |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.phi") {
                event.rename("json.phi", "claroty_xdome.alert.device.phi")?;
            }

            if event.has_value("json.product_code") {
                event.rename(
                    "json.product_code",
                    "claroty_xdome.alert.device.product.code",
                )?;
            }

            if event.has_value("json.protocol_location_list") {
                event.rename(
                    "json.protocol_location_list",
                    "claroty_xdome.alert.device.protocol.location_list",
                )?;
            }

            if event.has_value("json.purdue_level_source") {
                event.rename(
                    "json.purdue_level_source",
                    "claroty_xdome.alert.device.purdue_level.source",
                )?;
            }

            if event.has_value("json.purdue_level") {
                event.rename(
                    "json.purdue_level",
                    "claroty_xdome.alert.device.purdue_level.value",
                )?;
            }

            if event.has_value("json.recommended_firewall_group_name") {
                event.rename(
                    "json.recommended_firewall_group_name",
                    "claroty_xdome.alert.device.recommended.firewall_group_name",
                )?;
            }

            if event.has_value("json.recommended_zone_name") {
                event.rename(
                    "json.recommended_zone_name",
                    "claroty_xdome.alert.device.recommended.zone_name",
                )?;
            }

            let _cond = {
                event.has_value("json.retired_since")
                    && event.get_str("json.retired_since") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.retired_since") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("claroty_xdome.alert.device.retired.since", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.retired_since".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_retired_since")?;
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

            let _cond = { event.get_str("json.retired") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.retired") {
                        if let Some(val) = event.get("json.retired") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.retired".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.device.retired.value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_retired_to_boolean",
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

            let _cond = { event.get_str("json.risk_score_points") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.risk_score_points") {
                        if let Some(val) = event.get("json.risk_score_points") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.risk_score_points".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.device.risk_score.points", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_risk_score_points_to_float",
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
                .get("claroty_xdome.alert.device.risk_score.points")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.risk_score", v)?;
            }

            if event.has_value("json.risk_score") {
                event.rename(
                    "json.risk_score",
                    "claroty_xdome.alert.device.risk_score.value",
                )?;
            }

            let _cond = {
                event
                    .get("claroty_xdome.alert.device.risk_score.value")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.claroty_xdome.alert.device.risk_score.value;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"very low\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.claroty_xdome.alert.device.risk_score.value;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"very low\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
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

            if event.has_value("json.serial_number") {
                event.rename(
                    "json.serial_number",
                    "claroty_xdome.alert.device.serial_number",
                )?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.serial_number")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.serial_number", v)?;
            }

            if event.has_value("json.site_group_name") {
                event.rename(
                    "json.site_group_name",
                    "claroty_xdome.alert.device.site.group_name",
                )?;
            }

            if event.has_value("json.site_name") {
                event.rename("json.site_name", "claroty_xdome.alert.device.site.name")?;
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.site.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.site.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.slot_cards") {
                event.rename("json.slot_cards", "claroty_xdome.alert.device.slot_cards")?;
            }

            let _cond =
                { event.get_str("claroty_xdome.alert.device.slot_cards.cards_count") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("claroty_xdome.alert.device.slot_cards.cards_count") {
                        if let Some(val) =
                            event.get("claroty_xdome.alert.device.slot_cards.cards_count")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "claroty_xdome.alert.device.slot_cards.cards_count"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.device.slot_cards.count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_device_slot_cards_cards_count_to_long",
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
                event
                    .get("claroty_xdome.alert.device.slot_cards.racks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "claroty_xdome.alert.device.slot_cards.racks",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.number_of_slots") {
                                if let Some(val) = event.get("_ingest._value.number_of_slots") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.number_of_slots".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.number_of_slots", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_device_slot_cards_number_of_slots",
                            )?;
                            event.remove("_ingest._value.number_of_slots");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("claroty_xdome.alert.device.slot_cards.racks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "claroty_xdome.alert.device.slot_cards.racks",
                    |event| {
                        foreach_array(event, "_ingest._value.cards", |event| {
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.slot_number") {
                                    if let Some(val) = event.get("_ingest._value.slot_number") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.slot_number".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.slot_number", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_device_slot_cards_slot_number",
                                )?;
                                event.remove("_ingest._value");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        })?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("claroty_xdome.alert.device.slot_cards.racks")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "claroty_xdome.alert.device.slot_cards.racks",
                    |event| {
                        foreach_array(event, "_ingest._value.cards", |event| {
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.ip") {
                                    if let Some(val) = event.get("_ingest._value.ip") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.ip".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.ip", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_device_slot_cards_ip",
                                )?;
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        })?;
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.snmp_hostnames") {
                event.rename(
                    "json.snmp_hostnames",
                    "claroty_xdome.alert.device.snmp.hostnames",
                )?;
            }

            let _cond = {
                event
                    .get("claroty_xdome.alert.device.snmp.hostnames")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "claroty_xdome.alert.device.snmp.hostnames",
                    |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.snmp_last_seen_hostname") {
                event.rename(
                    "json.snmp_last_seen_hostname",
                    "claroty_xdome.alert.device.snmp.last_seen_hostname",
                )?;
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.snmp.last_seen_hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.snmp.last_seen_hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.software_or_firmware_version") {
                event.rename(
                    "json.software_or_firmware_version",
                    "claroty_xdome.alert.device.software_or_firmware_version",
                )?;
            }

            if event.has_value("json.ssid_list") {
                event.rename("json.ssid_list", "claroty_xdome.alert.device.ssid_list")?;
            }

            if event.has_value("json.device_subcategory") {
                event.rename(
                    "json.device_subcategory",
                    "claroty_xdome.alert.device.subcategory",
                )?;
            }

            if event.has_value("json.suspicious") {
                event.rename("json.suspicious", "claroty_xdome.alert.device.suspicious")?;
            }

            if event.has_value("json.switch_group_name_list") {
                event.rename(
                    "json.switch_group_name_list",
                    "claroty_xdome.alert.device.switch.group_name_list",
                )?;
            }

            if event.has_value("json.switch_ip_list") {
                event.rename(
                    "json.switch_ip_list",
                    "claroty_xdome.alert.device.switch.ip_list",
                )?;
            }

            let _cond = {
                event
                    .get("claroty_xdome.alert.device.switch.ip_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "claroty_xdome.alert.device.switch.ip_list",
                    |event| {
                        // on_failure: 1 handler(s)
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
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_device_switch_ip_list_to_ip",
                            )?;
                            event.remove("_ingest._value");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("claroty_xdome.alert.device.switch.ip_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "claroty_xdome.alert.device.switch.ip_list",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.switch_location_list") {
                event.rename(
                    "json.switch_location_list",
                    "claroty_xdome.alert.device.switch.location_list",
                )?;
            }

            if event.has_value("json.switch_mac_list") {
                event.rename(
                    "json.switch_mac_list",
                    "claroty_xdome.alert.device.switch.mac_list",
                )?;
            }

            if event.has_value("json.switch_name_list") {
                event.rename(
                    "json.switch_name_list",
                    "claroty_xdome.alert.device.switch.name_list",
                )?;
            }

            if event.has_value("json.switch_port_description_list") {
                event.rename(
                    "json.switch_port_description_list",
                    "claroty_xdome.alert.device.switch.port_description_list",
                )?;
            }

            if event.has_value("json.switch_port_list") {
                event.rename(
                    "json.switch_port_list",
                    "claroty_xdome.alert.device.switch.port_list",
                )?;
            }

            if event.has_value("json.device_type_family") {
                event.rename(
                    "json.device_type_family",
                    "claroty_xdome.alert.device.type.family",
                )?;
            }

            if event.has_value("json.device_type") {
                event.rename("json.device_type", "claroty_xdome.alert.device.type.value")?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.type.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.type", v)?;
            }

            if event.has_value("json.uid") {
                event.rename("json.uid", "claroty_xdome.alert.device.uid")?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.device.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond = { event.has_value("claroty_xdome.alert.device.uid") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.uid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.utilization_rate") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.utilization_rate") {
                        if let Some(val) = event.get("json.utilization_rate") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.utilization_rate".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.device.utilization_rate", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_utilization_rate_to_double",
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

            if event.has_value("json.visibility_score_level") {
                event.rename(
                    "json.visibility_score_level",
                    "claroty_xdome.alert.device.visibility_score.level",
                )?;
            }

            let _cond = { event.get_str("json.visibility_score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.visibility_score") {
                        if let Some(val) = event.get("json.visibility_score") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.visibility_score".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "claroty_xdome.alert.device.visibility_score.value",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_visibility_score_to_long",
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

            if event.has_value("json.vlan_description_list") {
                event.rename(
                    "json.vlan_description_list",
                    "claroty_xdome.alert.device.vlan.description_list",
                )?;
            }

            let _cond = { event.get("json.vlan_list").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.vlan_list", |event| {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.vlan_list").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.vlan_list", |event| {
                    event.append_unique(
                        "observer.ingress.vlan.id",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.vlan_list") {
                event.rename("json.vlan_list", "claroty_xdome.alert.device.vlan.list")?;
            }

            let _cond = {
                event
                    .get("json.vlan_name_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.vlan_name_list", |event| {
                    event.append_unique(
                        "observer.ingress.vlan.name",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.vlan_name_list") {
                event.rename(
                    "json.vlan_name_list",
                    "claroty_xdome.alert.device.vlan.name_list",
                )?;
            }

            let _cond = {
                event
                    .get("json.wifi_last_seen_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.wifi_last_seen_list").cloned();
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_ingest._value") {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => event.set("_ingest._value", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value".into(),
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
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_wifi_last_seen_list",
                                )?;
                                event.remove("_ingest._value");
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
                            "json.wifi_last_seen_list",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("json.wifi_last_seen_list") {
                event.rename(
                    "json.wifi_last_seen_list",
                    "claroty_xdome.alert.device.wifi_last_seen_list",
                )?;
            }

            if event.has_value("json.windows_hostnames") {
                event.rename(
                    "json.windows_hostnames",
                    "claroty_xdome.alert.device.windows.hostnames",
                )?;
            }

            let _cond = {
                event
                    .get("claroty_xdome.alert.device.windows.hostnames")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "claroty_xdome.alert.device.windows.hostnames",
                    |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.windows_last_seen_hostname") {
                event.rename(
                    "json.windows_last_seen_hostname",
                    "claroty_xdome.alert.device.windows.last_seen_hostname",
                )?;
            }

            let _cond =
                { event.has_value("claroty_xdome.alert.device.windows.last_seen_hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("claroty_xdome.alert.device.windows.last_seen_hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.wireless_encryption_type_list") {
                event.rename(
                    "json.wireless_encryption_type_list",
                    "claroty_xdome.alert.device.wireless_encryption_type_list",
                )?;
            }

            if event.has_value("json.wlc_location_list") {
                event.rename(
                    "json.wlc_location_list",
                    "claroty_xdome.alert.device.wlc.location_list",
                )?;
            }

            if event.has_value("json.wlc_name_list") {
                event.rename(
                    "json.wlc_name_list",
                    "claroty_xdome.alert.device.wlc.name_list",
                )?;
            }

            let _cond = { event.get_str("json.alert_info.devices_count") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.alert_info.devices_count") {
                        if let Some(val) = event.get("json.alert_info.devices_count") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.alert_info.devices_count".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.devices_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_alert_info_devices_count_to_long",
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

            if event.has_value("json.alert_info.id") {
                if let Some(val) = event.get("json.alert_info.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.alert_info.id".into(),
                            message,
                        }
                    })?;
                    event.set("claroty_xdome.alert.id", converted)?;
                }
            }

            let _cond = { event.get_str("json.alert_info.iot_devices_count") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.alert_info.iot_devices_count") {
                        if let Some(val) = event.get("json.alert_info.iot_devices_count") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.alert_info.iot_devices_count".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.iot_devices_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_alert_info_iot_devices_count_to_long",
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

            let _cond = { event.get_str("json.alert_info.it_devices_count") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.alert_info.it_devices_count") {
                        if let Some(val) = event.get("json.alert_info.it_devices_count") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.alert_info.it_devices_count".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.it_devices_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_alert_info_it_devices_count_to_long",
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

            if event.has_value("json.alert_info.malicious_ip_tags_list") {
                event.rename(
                    "json.alert_info.malicious_ip_tags_list",
                    "claroty_xdome.alert.malicious_ip_tags_list",
                )?;
            }

            let _cond = { event.get_str("json.alert_info.medical_devices_count") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.alert_info.medical_devices_count") {
                        if let Some(val) = event.get("json.alert_info.medical_devices_count") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.alert_info.medical_devices_count".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.medical_devices_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_alert_info_medical_devices_count_to_long",
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

            if event.has_value("json.alert_info.mitre_technique_enterprise_ids") {
                event.rename(
                    "json.alert_info.mitre_technique_enterprise_ids",
                    "claroty_xdome.alert.mitre_technique.enterprise.ids",
                )?;
            }

            if event.has_value("json.alert_info.mitre_technique_enterprise_names") {
                event.rename(
                    "json.alert_info.mitre_technique_enterprise_names",
                    "claroty_xdome.alert.mitre_technique.enterprise.names",
                )?;
            }

            if event.has_value("json.alert_info.mitre_technique_ics_ids") {
                event.rename(
                    "json.alert_info.mitre_technique_ics_ids",
                    "claroty_xdome.alert.mitre_technique.ics.ids",
                )?;
            }

            if event.has_value("json.alert_info.mitre_technique_ics_names") {
                event.rename(
                    "json.alert_info.mitre_technique_ics_names",
                    "claroty_xdome.alert.mitre_technique.ics.names",
                )?;
            }

            if event.has_value("json.alert_info.alert_name") {
                event.rename("json.alert_info.alert_name", "claroty_xdome.alert.name")?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("claroty_xdome.alert.friendly_name", v)?;
            }

            if let Some(v) = event
                .get("claroty_xdome.alert.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("event.action") {
                    gsub_field(
                        event,
                        "event.action",
                        "event.action",
                        cached_regex!("[/:-]"),
                        "",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_event_action")?;
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

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        if let Some(s) = event.get_string("event.action") {
                            let mut parts: Vec<Value> = cached_regex!("\\s+")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set("event.action", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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

            let _cond = { event.get("event.action").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                    if let Some(joined) = joined {
                        event.set("event.action", json!(joined))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "join")?;
                    event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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

            let _cond = { event.get_str("json.alert_info.ot_devices_count") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.alert_info.ot_devices_count") {
                        if let Some(val) = event.get("json.alert_info.ot_devices_count") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.alert_info.ot_devices_count".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.ot_devices_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_alert_info_ot_devices_count_to_long",
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

            if event.has_value("json.alert_info.status") {
                event.rename("json.alert_info.status", "claroty_xdome.alert.status")?;
            }

            if event.has_value("json.alert_info.alert_type_name") {
                event.rename(
                    "json.alert_info.alert_type_name",
                    "claroty_xdome.alert.type_name",
                )?;
            }

            let _cond = { event.get_str("json.alert_info.unresolved_devices_count") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.alert_info.unresolved_devices_count") {
                        if let Some(val) = event.get("json.alert_info.unresolved_devices_count") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.alert_info.unresolved_devices_count".into(),
                                    message,
                                }
                            })?;
                            event.set("claroty_xdome.alert.unresolved_devices_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_alert_info_unresolved_devices_count_to_long",
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
                event.has_value("json.alert_info.updated_time")
                    && event.get_str("json.alert_info.updated_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.alert_info.updated_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("claroty_xdome.alert.updated_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.alert_info.updated_time".into(),
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
                        "date_alert_info_updated_time",
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
                .get("claroty_xdome.alert.updated_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
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
                event.remove("claroty_xdome.alert.description");
                event.remove("claroty_xdome.alert.device.domains");
                event.remove("claroty_xdome.alert.device.combined_os");
                event.remove("claroty_xdome.alert.device.manufacturer");
                event.remove("claroty_xdome.alert.device.model.family");
                event.remove("claroty_xdome.alert.device.model.name");
                event.remove("claroty_xdome.alert.device.os.category");
                event.remove("claroty_xdome.alert.device.os.name");
                event.remove("claroty_xdome.alert.device.os.version");
                event.remove("claroty_xdome.alert.device.risk_score.points");
                event.remove("claroty_xdome.alert.device.serial_number");
                event.remove("claroty_xdome.alert.device.type.value");
                event.remove("claroty_xdome.alert.device.uid");
                event.remove("claroty_xdome.alert.updated_time");
                event.remove("claroty_xdome.alert.device.vlan.list");
                event.remove("claroty_xdome.alert.device.vlan.name_list");
                event.remove("claroty_xdome.alert.device.http.last_seen_hostname");
            }

            event.remove("json");
            event.remove("claroty_xdome.alert.device.slot_cards.cards_count");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
