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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.normal_check_interval") {
                    if let Some(val) = event.get("json.normal_check_interval") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.normal_check_interval".into(),
                                message,
                            }
                        })?;
                        event.set("json.normal_check_interval", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.retry_check_interval") {
                    if let Some(val) = event.get("json.retry_check_interval") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.retry_check_interval".into(),
                                message,
                            }
                        })?;
                        event.set("json.retry_check_interval", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.execution_time") {
                    if let Some(val) = event.get("json.execution_time") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.execution_time".into(),
                                message,
                            }
                        })?;
                        event.set("json.execution_time", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.latency") {
                    if let Some(val) = event.get("json.latency") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.latency".into(),
                                message,
                            }
                        })?;
                        event.set("json.latency", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.status_update_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.status_update_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!(
                    event
                        .get("@timestamp")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("json.status_update_time", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.status_update_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss.SSSXXX"], None, None) {
                        Some(parsed) => event.set("json.status_update_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.status_update_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_check") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("json.last_check", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_check".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.next_check") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("json.next_check", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.next_check".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_state_change") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.service.last_state_change", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_state_change".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_hard_state_change") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => {
                            event.set("nagios_xi.service.last_hard_state_change", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_hard_state_change".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_time_ok") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.service.last_time_ok", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_time_ok".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_time_warning") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.service.last_time_warning", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_time_warning".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_time_unknown") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.service.last_time_unknown", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_time_unknown".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_time_critical") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => {
                            event.set("nagios_xi.service.last_time_critical", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_time_critical".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.last_notification") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.service.last_notification", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_notification".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.next_notification") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("nagios_xi.service.next_notification", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.next_notification".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.display_name") {
                    event.rename("json.display_name", "service.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.host_name") {
                    event.rename("json.host_name", "nagios_xi.service.host_name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.status_update_time") {
                    event.rename(
                        "json.status_update_time",
                        "nagios_xi.service.status_update_time",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.perfdata") {
                    event.rename("json.perfdata", "nagios_xi.service.temp.performance_data")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.check_command") {
                    event.rename("json.check_command", "nagios_xi.service.check_command")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.normal_check_interval") {
                    event.rename(
                        "json.normal_check_interval",
                        "nagios_xi.service.normal_check_interval",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.retry_check_interval") {
                    event.rename(
                        "json.retry_check_interval",
                        "nagios_xi.service.retry_check_interval",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.last_check") {
                    event.rename("json.last_check", "nagios_xi.service.last_check")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.next_check") {
                    event.rename("json.next_check", "nagios_xi.service.next_check")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.latency") {
                    event.rename("json.latency", "nagios_xi.service.latency")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.execution_time") {
                    event.rename("json.execution_time", "nagios_xi.service.execution_time")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.service_description") {
                    event.rename(
                        "json.service_description",
                        "nagios_xi.service.service_description",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.host_object_id") {
                    event.rename("json.host_object_id", "nagios_xi.service.host_object_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.host_address") {
                    event.rename("json.host_address", "nagios_xi.service.host_address")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.host_alias") {
                    event.rename("json.host_alias", "nagios_xi.service.host_alias")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.icon_image") {
                    event.rename("json.icon_image", "nagios_xi.service.icon_image")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.icon_image_alt") {
                    event.rename("json.icon_image_alt", "nagios_xi.service.icon_image_alt")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.notes") {
                    event.rename("json.notes", "nagios_xi.service.notes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.notes_url") {
                    event.rename("json.notes_url", "nagios_xi.service.notes_url")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.action_url") {
                    event.rename("json.action_url", "nagios_xi.service.action_url")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.servicestatus_id") {
                    event.rename(
                        "json.servicestatus_id",
                        "nagios_xi.service.servicestatus_id",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.instance_id") {
                    event.rename("json.instance_id", "nagios_xi.service.instance_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.service_object_id") {
                    event.rename(
                        "json.service_object_id",
                        "nagios_xi.service.service_object_id",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.output") {
                    event.rename("json.output", "nagios_xi.service.output")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.long_output") {
                    event.rename("json.long_output", "nagios_xi.service.long_output")?;
                }
                Ok(())
            })();

            let _cond = {
                event.get("json.current_state").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("0")),
                    serde_json::Value::String(s) => s.contains("0"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("Up/Pending");
                    if !painless_is_empty_value(&v) {
                        event.set("nagios_xi.service.current_state", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("json.current_state").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("1")),
                    serde_json::Value::String(s) => s.contains("1"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("Warning");
                    if !painless_is_empty_value(&v) {
                        event.set("nagios_xi.service.current_state", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("json.current_state").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("2")),
                    serde_json::Value::String(s) => s.contains("2"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("Critical");
                    if !painless_is_empty_value(&v) {
                        event.set("nagios_xi.service.current_state", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("json.current_state").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("3")),
                    serde_json::Value::String(s) => s.contains("3"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("Unknown");
                    if !painless_is_empty_value(&v) {
                        event.set("nagios_xi.service.current_state", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.has_been_checked") {
                    event.rename(
                        "json.has_been_checked",
                        "nagios_xi.service.has_been_checked",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.should_be_scheduled") {
                    event.rename(
                        "json.should_be_scheduled",
                        "nagios_xi.service.should_be_scheduled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.current_check_attempt") {
                    event.rename(
                        "json.current_check_attempt",
                        "nagios_xi.service.current_check_attempt",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.max_check_attempts") {
                    event.rename(
                        "json.max_check_attempts",
                        "nagios_xi.service.max_check_attempts",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.check_type") {
                    event.rename("json.check_type", "nagios_xi.service.check_type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.check_options") {
                    event.rename("json.check_options", "nagios_xi.service.check_options")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.last_hard_state") {
                    event.rename("json.last_hard_state", "nagios_xi.service.last_hard_state")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.state_type") {
                    event.rename("json.state_type", "nagios_xi.service.state_type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.no_more_notifications") {
                    event.rename(
                        "json.no_more_notifications",
                        "nagios_xi.service.no_more_notifications",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.notifications_enabled") {
                    event.rename(
                        "json.notifications_enabled",
                        "nagios_xi.service.notifications_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.problem_has_been_acknowledged") {
                    event.rename(
                        "json.problem_has_been_acknowledged",
                        "nagios_xi.service.problem_has_been_acknowledged",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.acknowledgement_type") {
                    event.rename(
                        "json.acknowledgement_type",
                        "nagios_xi.service.acknowledgement_type",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.current_notification_number") {
                    event.rename(
                        "json.current_notification_number",
                        "nagios_xi.service.current_notification_number",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.passive_checks_enabled") {
                    event.rename(
                        "json.passive_checks_enabled",
                        "nagios_xi.service.passive_checks_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.active_checks_enabled") {
                    event.rename(
                        "json.active_checks_enabled",
                        "nagios_xi.service.active_checks_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.event_handler_enabled") {
                    event.rename(
                        "json.event_handler_enabled",
                        "nagios_xi.service.event_handler_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.flap_detection_enabled") {
                    event.rename(
                        "json.flap_detection_enabled",
                        "nagios_xi.service.flap_detection_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.is_flapping") {
                    event.rename("json.is_flapping", "nagios_xi.service.is_flapping")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.percent_state_change") {
                    event.rename(
                        "json.percent_state_change",
                        "nagios_xi.service.percent_state_change",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.scheduled_downtime_depth") {
                    event.rename(
                        "json.scheduled_downtime_depth",
                        "nagios_xi.service.scheduled_downtime_depth",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.failure_prediction_enabled") {
                    event.rename(
                        "json.failure_prediction_enabled",
                        "nagios_xi.service.failure_prediction_enabled",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.process_performance_data") {
                    event.rename(
                        "json.process_performance_data",
                        "nagios_xi.service.process_performance_data",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.obsess_over_service") {
                    event.rename(
                        "json.obsess_over_service",
                        "nagios_xi.service.obsess_over_service",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.modified_service_attributes") {
                    event.rename(
                        "json.modified_service_attributes",
                        "nagios_xi.service.modified_service_attributes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.event_handler") {
                    event.rename("json.event_handler", "nagios_xi.service.event_handler")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.check_timeperiod_object_id") {
                    event.rename(
                        "json.check_timeperiod_object_id",
                        "nagios_xi.service.check_timeperiod_object_id",
                    )?;
                }
                Ok(())
            })();

            let _cond = {
                event
                    .get("nagios_xi.service.check_command")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("check_local_users"))
                        }
                        serde_json::Value::String(s) => s.contains("check_local_users"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("nagios_xi.service.temp")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("nagios_xi.service.current_users", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has("nagios_xi.service.current_users") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("nagios_xi.service.current_users.performance_data") {
                        if let Some(input) =
                            event.get_string("nagios_xi.service.current_users.performance_data")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("users=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "nagios_xi.service.current_users.users",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.current_users.users") {
                    if let Some(val) = event.get("nagios_xi.service.current_users.users") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.current_users.users".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.current_users.users", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has("nagios_xi.service.current_users") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("nagios_xi.current_users");
                    if !painless_is_empty_value(&v) {
                        event.set("event.provider", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("nagios_xi.service.check_command")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("check_local_load"))
                        }
                        serde_json::Value::String(s) => s.contains("check_local_load"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("nagios_xi.service.temp")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("nagios_xi.service.current_load", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has("nagios_xi.service.current_load") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("nagios_xi.service.current_load.performance_data") {
                        if let Some(input) =
                            event.get_string("nagios_xi.service.current_load.performance_data")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("load1=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "nagios_xi.service.current_load.load1",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("load5=") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("load5=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "nagios_xi.service.current_load.load5",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("load15=") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("load15=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "nagios_xi.service.current_load.load15",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.current_load.load1") {
                    if let Some(val) = event.get("nagios_xi.service.current_load.load1") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.current_load.load1".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.current_load.load1", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.current_load.load5") {
                    if let Some(val) = event.get("nagios_xi.service.current_load.load5") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.current_load.load5".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.current_load.load5", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.current_load.load15") {
                    if let Some(val) = event.get("nagios_xi.service.current_load.load15") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.current_load.load15".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.current_load.load15", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has("nagios_xi.service.current_load") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("nagios_xi.current_load");
                    if !painless_is_empty_value(&v) {
                        event.set("event.provider", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("nagios_xi.service.check_command")
                    && (event
                        .get("nagios_xi.service.check_command")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("check_ssh"))
                            }
                            serde_json::Value::String(s) => s.contains("check_ssh"),
                            _ => false,
                        })
                        || event
                            .get("nagios_xi.service.check_command")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("check_xi_service_ssh"))
                                }
                                serde_json::Value::String(s) => s.contains("check_xi_service_ssh"),
                                _ => false,
                            }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("nagios_xi.service.temp")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("nagios_xi.service.ssh", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has("nagios_xi.service.ssh") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("nagios_xi.service.ssh.performance_data") {
                        if let Some(input) =
                            event.get_string("nagios_xi.service.ssh.performance_data")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("time=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("s") else {
                                    break 'dissect false;
                                };
                                captured.push(("nagios_xi.service.ssh.time", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("s") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.ssh.time") {
                    if let Some(val) = event.get("nagios_xi.service.ssh.time") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.ssh.time".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.ssh.time", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has("nagios_xi.service.ssh") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("nagios_xi.ssh");
                    if !painless_is_empty_value(&v) {
                        event.set("event.provider", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("nagios_xi.service.check_command")
                    && (event
                        .get("nagios_xi.service.check_command")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("check_ping"))
                            }
                            serde_json::Value::String(s) => s.contains("check_ping"),
                            _ => false,
                        })
                        || event
                            .get("nagios_xi.service.check_command")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => a
                                    .iter()
                                    .any(|x| x.as_str() == Some("check_xi_service_ping")),
                                serde_json::Value::String(s) => s.contains("check_xi_service_ping"),
                                _ => false,
                            }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("nagios_xi.service.temp")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("nagios_xi.service.ping", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has("nagios_xi.service.ping") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("nagios_xi.service.ping.performance_data") {
                        if let Some(input) =
                            event.get_string("nagios_xi.service.ping.performance_data")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("rta=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("ms;") else {
                                    break 'dissect false;
                                };
                                captured.push(("nagios_xi.service.ping.rta", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("ms;") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("pl=") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("pl=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("%;") else {
                                    break 'dissect false;
                                };
                                captured.push(("nagios_xi.service.ping.pl", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("%;") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.ping.rta") {
                    if let Some(val) = event.get("nagios_xi.service.ping.rta") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.ping.rta".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.ping.rta", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.ping.pl") {
                    if let Some(val) = event.get("nagios_xi.service.ping.pl") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.ping.pl".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.ping.pl", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has("nagios_xi.service.ping") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("nagios_xi.ping");
                    if !painless_is_empty_value(&v) {
                        event.set("event.provider", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("nagios_xi.service.check_command")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("check_local_swap"))
                        }
                        serde_json::Value::String(s) => s.contains("check_local_swap"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("nagios_xi.service.temp")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("nagios_xi.service.swap_usage", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has("nagios_xi.service.swap_usage") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("nagios_xi.service.swap_usage.performance_data") {
                        if let Some(input) =
                            event.get_string("nagios_xi.service.swap_usage.performance_data")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("swap=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("MB;") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "nagios_xi.service.swap_usage.free_swap",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("MB;") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured
                                    .push(("nagios_xi.service.swap_usage.total_swap", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.swap_usage.free_swap") {
                    if let Some(val) = event.get("nagios_xi.service.swap_usage.free_swap") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.swap_usage.free_swap".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.swap_usage.free_swap", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.swap_usage.total_swap") {
                    if let Some(val) = event.get("nagios_xi.service.swap_usage.total_swap") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.swap_usage.total_swap".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.swap_usage.total_swap", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if(ctx.nagios_xi?.service?.containsKey(\"swap_usage\") == true) {\n    ctx.nagios_xi.service.swap_usage.used_swap = ctx.nagios_xi.service.swap_usage.total_swap - ctx.nagios_xi.service.swap_usage.free_swap\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if(ctx.nagios_xi?.service?.containsKey(\"swap_usage\") == true) {\n    ctx.nagios_xi.service.swap_usage.used_swap = ctx.nagios_xi.service.swap_usage.total_swap - ctx.nagios_xi.service.swap_usage.free_swap\n}\n"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has("nagios_xi.service.swap_usage") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("nagios_xi.swap_usage");
                    if !painless_is_empty_value(&v) {
                        event.set("event.provider", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("nagios_xi.service.check_command")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("check_local_procs"))
                        }
                        serde_json::Value::String(s) => s.contains("check_local_procs"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("nagios_xi.service.temp")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("nagios_xi.service.process", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has("nagios_xi.service.process") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("nagios_xi.service.process.performance_data") {
                        if let Some(input) =
                            event.get_string("nagios_xi.service.process.performance_data")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("procs=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                captured
                                    .push(("nagios_xi.service.process.total", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.process.total") {
                    if let Some(val) = event.get("nagios_xi.service.process.total") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.process.total".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.process.total", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has("nagios_xi.service.process") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("nagios_xi.process");
                    if !painless_is_empty_value(&v) {
                        event.set("event.provider", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("nagios_xi.service.check_command")
                    && (event
                        .get("nagios_xi.service.check_command")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("check_http"))
                            }
                            serde_json::Value::String(s) => s.contains("check_http"),
                            _ => false,
                        })
                        || event
                            .get("nagios_xi.service.check_command")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => a
                                    .iter()
                                    .any(|x| x.as_str() == Some("check_xi_service_http")),
                                serde_json::Value::String(s) => s.contains("check_xi_service_http"),
                                _ => false,
                            }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("nagios_xi.service.temp")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("nagios_xi.service.http", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has("nagios_xi.service.http") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("nagios_xi.service.http.performance_data") {
                        if let Some(input) =
                            event.get_string("nagios_xi.service.http.performance_data")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("time=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("s;;;") else {
                                    break 'dissect false;
                                };
                                captured.push(("nagios_xi.service.http.time", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("s;;;") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("size=") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("size=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("B;") else {
                                    break 'dissect false;
                                };
                                captured.push(("nagios_xi.service.http.size", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("B;") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.http.time") {
                    if let Some(val) = event.get("nagios_xi.service.http.time") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.http.time".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.http.time", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.http.size") {
                    if let Some(val) = event.get("nagios_xi.service.http.size") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.http.size".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.http.size", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has("nagios_xi.service.http") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("nagios_xi.http");
                    if !painless_is_empty_value(&v) {
                        event.set("event.provider", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("nagios_xi.service.check_command")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("check_local_disk"))
                        }
                        serde_json::Value::String(s) => s.contains("check_local_disk"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("nagios_xi.service.temp")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("nagios_xi.service.root_partition", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has("nagios_xi.service.root_partition") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("nagios_xi.service.root_partition.performance_data") {
                        if let Some(input) =
                            event.get_string("nagios_xi.service.root_partition.performance_data")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("=") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("M") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "nagios_xi.service.root_partition.used_space",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("M") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(";") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(";") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push((
                                    "nagios_xi.service.root_partition.total_space",
                                    remaining,
                                ));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.root_partition.total_space") {
                    if let Some(val) = event.get("nagios_xi.service.root_partition.total_space") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.root_partition.total_space".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.root_partition.total_space", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("nagios_xi.service.root_partition.used_space") {
                    if let Some(val) = event.get("nagios_xi.service.root_partition.used_space") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "nagios_xi.service.root_partition.used_space".into(),
                                message,
                            }
                        })?;
                        event.set("nagios_xi.service.root_partition.used_space", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if(ctx.nagios_xi?.service?.containsKey(\"root_partition\") == true) {\n    ctx.nagios_xi.service.root_partition.free_space = ctx.nagios_xi.service.root_partition.total_space - ctx.nagios_xi.service.root_partition.used_space\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if(ctx.nagios_xi?.service?.containsKey(\"root_partition\") == true) {\n    ctx.nagios_xi.service.root_partition.free_space = ctx.nagios_xi.service.root_partition.total_space - ctx.nagios_xi.service.root_partition.used_space\n}\n"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has("nagios_xi.service.root_partition") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("nagios_xi.root_partition");
                    if !painless_is_empty_value(&v) {
                        event.set("event.provider", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                !event.has_value("nagios_xi.service")
                    || !(event.has("nagios_xi.service.current_users")
                        || event.has("nagios_xi.service.current_load")
                        || event.has("nagios_xi.service.ssh")
                        || event.has("nagios_xi.service.ping")
                        || event.has("nagios_xi.service.swap_usage")
                        || event.has("nagios_xi.service.process")
                        || event.has("nagios_xi.service.http")
                        || event.has("nagios_xi.service.root_partition"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("nagios_xi.custom");
                    if !painless_is_empty_value(&v) {
                        event.set("event.provider", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.dataset").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("nagios_xi.custom"))
                    }
                    serde_json::Value::String(s) => s.contains("nagios_xi.custom"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("nagios_xi.service.temp")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("nagios_xi.service.custom", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("metric");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("nagios_xi");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("info")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.type", v)?;
                }
                Ok(())
            })();

            // Painless script
            // Source: boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("json");
                event.remove("nagios_xi.service.temp");
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
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
