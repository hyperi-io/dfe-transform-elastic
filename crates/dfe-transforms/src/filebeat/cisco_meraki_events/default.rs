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
                event.has_value("json.sharedSecret")
                    && event.get_str("json.sharedSecret") != Some("")
                    && event.get("_conf.secret").filter(|v| !v.is_null())
                        != event.get("json.sharedSecret").filter(|v| !v.is_null())
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("ecs.version", json!("8.11.0"))?;

            if let Some(v) = event.get("json.deviceSerial").cloned() {
                event.set("observer.serial_number", v)?;
            }

            gsub_field(
                event,
                "json.deviceMac",
                "_tmp.observer.mac",
                cached_regex!("[-:.]"),
                "-",
            )?;

            let _cond = { event.has_value("_tmp.observer.mac") };
            if _cond {
                event.append(
                    "observer.mac",
                    json!(
                        event
                            .get("_tmp.observer.mac")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event.get("json.deviceName").cloned() {
                event.set("observer.name", v)?;
            }

            event.set("observer.vendor", json!("Cisco"))?;

            if let Some(v) = event.get("json.deviceModel").cloned() {
                event.set("observer.product", v)?;
            }

            if let Some(v) = event.get("json.networkName").cloned() {
                event.set("network.name", v)?;
            }

            if let Some(date_str) = event.get_as_string("json.occurredAt") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.occurredAt".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if let Some(v) = event.get("json.organizationId").cloned() {
                event.set("organization.id", v)?;
            }

            if let Some(v) = event.get("json.organizationName").cloned() {
                event.set("organization.name", v)?;
            }

            if let Some(v) = event.get("json.alertLevel").cloned() {
                event.set("log.level", v)?;
            }

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("info"))?;

            let _cond = { event.has_value("json.alertTypeId") };
            if _cond {
                // Painless script
                // Source: def alertTypeId = ctx.json.alertTypeId;\ndef eventMap = params.get('eventmap');\ndef eventData = eventMap.get(alertTypeId);\nctx.event.action = ctx.json.alertType;\nif (eventData == null) {\n  // Unclassified events\n  // - geofencing_in, critical_temperature\n  // - gateway_to_repeater, mi_alert\n  // - motion_alert, usage_alert\n  // - new_splash_signup, rps_base_supply_up\n  // - rps_backup, vpn_connectivity_change\n  return;\n}\ndef eventCategory = eventData.get('category');\nif (eventCategory != null) {\n  for (def c : eventCategory) {\n    ctx.event.category.add(c);\n  }\n}\ndef eventType = eventData.get('type');\nif (eventType != null) {\n  for (def t : eventType) {\n    ctx.event.type.add(t);\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def alertTypeId = ctx.json.alertTypeId;\ndef eventMap = params.get('eventmap');\ndef eventData = eventMap.get(alertTypeId);\nctx.event.action = ctx.json.alertType;\nif (eventData == null) {\n  // Unclassified events\n  // - geofencing_in, critical_temperature\n  // - gateway_to_repeater, mi_alert\n  // - motion_alert, usage_alert\n  // - new_splash_signup, rps_base_supply_up\n  // - rps_backup, vpn_connectivity_change\n  return;\n}\ndef eventCategory = eventData.get('category');\nif (eventCategory != null) {\n  for (def c : eventCategory) {\n    ctx.event.category.add(c);\n  }\n}\ndef eventType = eventData.get('type');\nif (eventType != null) {\n  for (def t : eventType) {\n    ctx.event.type.add(t);\n  }\n}"#
                    ),
                    cached_params!(
                        "{\"eventmap\":{\"started_reporting\":{\"type\":[\"start\"]},\"stopped_reporting\":{\"type\":[\"end\"]},\"foreign_ap\":{\"category\":[\"intrusion_detection\",\"threat\"],\"type\":[\"indicator\"]},\"bluetooth_in\":{\"type\":[\"start\"]},\"bluetooth_out\":{\"type\":[\"end\"]},\"port_cable_error\":{\"type\":[\"connection\"]},\"node_hardware_failure\":{\"category\":[\"host\"],\"type\":[\"end\"]},\"cellular_up\":{\"type\":[\"start\"]},\"cellular_down\":{\"type\":[\"end\"]},\"umbrella_expiring\":{\"category\":[\"configuration\"]},\"ip_conflict\":{\"type\":[\"protocol\"]},\"rogue_ap_association\":{\"category\":[\"threat\"],\"type\":[\"indicator\"]},\"client_connectivity\":{\"category\":[\"session\"],\"type\":[\"connection\"]},\"pcc_security_compliance\":{\"category\":[\"configuration\"]},\"pcc_security_violation\":{\"category\":[\"configuration\",\"threat\"],\"type\":[\"change\",\"indicator\"]},\"pcc_outage_end\":{\"category\":[\"host\"],\"type\":[\"connection\"]},\"pcc_enrollment\":{\"category\":[\"session\"],\"type\":[\"connection\",\"start\"]},\"geofencing_out\":{\"type\":[\"connection\"]},\"pcc_outage_begin\":{\"category\":[\"host\"],\"type\":[\"connection\",\"end\"]},\"dhcp_no_leases\":{\"type\":[\"connection\",\"denied\",\"protocol\"]},\"vrrp\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"pcc_expired_apns_cert\":{\"category\":[\"authentication\"]},\"amp_malware_blocked\":{\"category\":[\"threat\",\"intrusion_detection\"],\"type\":[\"indicator\",\"denied\"]},\"amp_malware_detected\":{\"category\":[\"threat\",\"intrusion_detection\"],\"type\":[\"indicator\",\"allowed\"]},\"pcc_sw_found\":{\"category\":[\"host\",\"configuration\"],\"type\":[\"change\"]},\"pcc_unmanaged\":{\"category\":[\"configuration\",\"iam\"],\"type\":[\"change\",\"deletion\"]},\"dhcp_alerts\":{\"type\":[\"protocol\"]},\"power_supply_up\":{\"type\":[\"start\"]},\"power_supply_down\":{\"category\":[\"host\"],\"type\":[\"end\"]},\"unreachable_radius_server\":{\"category\":[\"authentication\"],\"type\":[\"end\",\"denied\"]},\"rogue_ap\":{\"category\":[\"threat\"],\"type\":[\"indicator\"]},\"rogue_dhcp\":{\"category\":[\"threat\"],\"type\":[\"indicator\"]},\"settings_changed\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"port_connected\":{\"type\":[\"connection\"]},\"port_disconnected\":{\"type\":[\"end\"]},\"port_speed_change\":{\"category\":[\"configuration\"],\"type\":[\"change\",\"protocol\"]},\"udld_error\":{\"type\":[\"connection\",\"end\"]},\"uplink_ip6_conflict\":{\"type\":[\"protocol\"]}}}"
                    ),
                )?;
            }

            event.rename("json", "cisco_meraki.event")?;

            event.remove("cisco_meraki.event.deviceSerial");
            event.remove("cisco_meraki.event.deviceMac");
            event.remove("cisco_meraki.event.deviceName");
            event.remove("cisco_meraki.event.deviceModel");
            event.remove("cisco_meraki.event.occurredAt");
            event.remove("cisco_meraki.event.networkName");
            event.remove("cisco_meraki.event.organizationId");
            event.remove("cisco_meraki.event.organizationName");
            event.remove("cisco_meraki.event.alertType");
            event.remove("cisco_meraki.event.alertLevel");
            event.remove("_tmp");
            event.remove("_conf");

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n"#
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
