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

            event.rename("json", "jamf_pro.events")?;

            let _cond = {
                event.has_value("jamf_pro.events.event.management_id")
                    && !(event
                        .get("jamf_pro.events.event.management_id")
                        .is_some_and(|v| v.is_string()))
            };
            if _cond {
                if let Some(val) = event.get("jamf_pro.events.event.management_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "jamf_pro.events.event.management_id".into(),
                            message,
                        }
                    })?;
                    event.set("jamf_pro.events.event.management_id", converted)?;
                }
            }

            let _cond = {
                event
                    .get("jamf_pro.events.event.computer")
                    .is_some_and(|v| v.is_boolean())
            };
            if _cond {
                event.rename(
                    "jamf_pro.events.event.computer",
                    "jamf_pro.events.event.is_computer",
                )?;
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "31d03dd006bf0331b26c03794258d93a945f53002a144691fa398fc71dbce3db",
                    r#"{"params":null,"known":[{"KeysToSnakeCase":["jamf_pro.events","CamelBreak"]}]}"#
                ),
            )?;

            if let Some(date_str) = event.get_as_string("jamf_pro.events.webhook.event_timestamp") {
                match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                    Some(parsed) => event.set("jamf_pro.events.webhook.event_timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "jamf_pro.events.webhook.event_timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            let _cond = { event.has_value("jamf_pro.events.event.last_update") };
            if _cond {
                if let Some(date_str) = event.get_as_string("jamf_pro.events.event.last_update") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("jamf_pro.events.event.last_update", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "jamf_pro.events.event.last_update".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event
                .get("jamf_pro.events.webhook.event_timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.entity.lifecycle.last_activity", v)?;
            }

            let _cond = {
                event.has_value("jamf_pro.events.event.ip_address")
                    && event.get_str("jamf_pro.events.event.ip_address") != Some("")
            };
            if _cond {
                let v = Value::Array(vec![json!(
                    event
                        .get("jamf_pro.events.event.ip_address")
                        .map_or_else(String::new, template_to_string)
                )]);
                if !painless_is_empty_value(&v) {
                    event.set("host.ip", v)?;
                }
            }

            let _cond = {
                event.has_value("jamf_pro.events.event.computer.reported_ip_address")
                    && event.get_str("jamf_pro.events.event.computer.reported_ip_address")
                        != Some("")
            };
            if _cond {
                let v = Value::Array(vec![json!(
                    event
                        .get("jamf_pro.events.event.computer.reported_ip_address")
                        .map_or_else(String::new, template_to_string)
                )]);
                if !painless_is_empty_value(&v) {
                    event.set("host.ip", v)?;
                }
            }

            if let Some(v) = event
                .get("jamf_pro.events.event.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("jamf_pro.events.event.computer.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("jamf_pro.events.event.computer.email_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            if let Some(v) = event
                .get("jamf_pro.events.event.email_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("jamf_pro.events.event.os_version") };
            if _cond {
                if let Some(v) = event.get("jamf_pro.events.event.os_version").cloned() {
                    event.set("host.os.version", v)?;
                }
            }

            let _cond = { event.has_value("jamf_pro.events.event.computer.os_version") };
            if _cond {
                if let Some(v) = event
                    .get("jamf_pro.events.event.computer.os_version")
                    .cloned()
                {
                    event.set("host.os.version", v)?;
                }
            }

            if let Some(v) = event
                .get("jamf_pro.events.event.computer.device_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if let Some(v) = event
                .get("jamf_pro.events.event.computer.udid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("host.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.address", v)?;
            }

            let _cond = { event.has_value("host.ip") && event.get_str("host.ip") != Some("") };
            if _cond {
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
            }

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("user.email") && event.get_str("user.email") != Some("") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.set("event.kind", json!("event"))?;

            if let Some(v) = event
                .get("jamf_pro.events.webhook.webhook_event")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "f7fcd815dd563e4cdcd59cc603b974577a96cc42b8030cef3011cd26d01a64b9",
                    r#"{"params":{"RowOrDefaults":{"subject":"event.action","table":"actions","prelude":[],"columns":[{"target":"event.category","member":"category","only_if_unset":false,"empty":"Skip"},{"target":"event.type","member":"type","only_if_unset":false,"empty":{"Fill":["info"]}}],"defaults":[]}},"known":[]}"#
                ),
                cached_params!(
                    "{\"actions\":{\"ComputerAdded\":{\"category\":[\"host\"],\"type\":[\"change\"]},\"ComputerCheckIn\":{\"category\":[\"host\"],\"type\":[]},\"ComputerInventoryCompleted\":{\"category\":[\"host\"],\"type\":[]},\"ComputerPushCapabilityChanged\":{\"category\":[\"host\"],\"type\":[\"change\"]},\"MobileDeviceEnrolled\":{\"category\":[\"host\"],\"type\":[\"change\",\"start\"]},\"MobileDeviceUnEnrolled\":{\"category\":[\"host\"],\"type\":[\"change\",\"end\"]},\"MobileDeviceCheckIn\":{\"category\":[\"host\"],\"type\":[]},\"MobileDeviceInventoryCompleted\":{\"category\":[\"host\"],\"type\":[]},\"MobileDeviceCommandCompleted\":{\"category\":[\"host\"],\"type\":[\"change\",\"end\"]},\"MobileDevicePushSent\":{\"category\":[\"host\"],\"type\":[]},\"DeviceAddedToDEP\":{\"category\":[\"host\"],\"type\":[\"change\"]},\"PushSent\":{\"category\":[\"host\"],\"type\":[]},\"SCEPChallenge\":{\"category\":[\"host\",\"authentication\"],\"type\":[\"start\"]},\"SmartGroupComputerMembershipChange\":{\"category\":[\"host\",\"iam\"],\"type\":[\"change\",\"group\"]},\"SmartGroupMobileDeviceMembershipChange\":{\"category\":[\"host\",\"iam\"],\"type\":[\"change\",\"group\"]},\"JSSStartup\":{\"category\":[\"host\"],\"type\":[\"start\"]},\"JSSShutdown\":{\"category\":[\"host\"],\"type\":[\"end\"]},\"ComputerPolicyFinished\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"ComputerPatchPolicyCompleted\":{\"category\":[\"configuration\",\"package\"],\"type\":[\"change\"]},\"PatchSoftwareTitleUpdated\":{\"category\":[\"configuration\",\"package\"],\"type\":[\"change\"]},\"RestAPIOperation\":{\"category\":[\"api\"],\"type\":[\"change\",\"admin\"]},\"SmartGroupUserMembershipChange\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\"]}}}"
                ),
            )?;

            let _cond = {
                event.get_bool("jamf_pro.events.event.successful") == Some(true)
                    || event.get_bool("jamf_pro.events.event.operation_successful") == Some(true)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_bool("jamf_pro.events.event.successful") == Some(false)
                    || event.get_bool("jamf_pro.events.event.operation_successful") == Some(false)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("jamf_pro.events.event.computer.ip_address")
                    && event.get_str("jamf_pro.events.event.computer.ip_address") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("jamf_pro.events.event.computer.ip_address") {
                        if let Some(val) = event.get("jamf_pro.events.event.computer.ip_address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "jamf_pro.events.event.computer.ip_address".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
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
            }
        }

        Ok(TransformResult::Continue)
    }
}
