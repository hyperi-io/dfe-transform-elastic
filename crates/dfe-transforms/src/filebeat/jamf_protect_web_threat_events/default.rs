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

            let _cond = { event.has_value("message") };
            if _cond {
                parse_json_field(event, "message", "jamf_protect.threat")?;
            }

            let _cond = { event.has_value("json") };
            if _cond {
                event.rename("json", "jamf_protect.threat")?;
            }

            event.set("observer.product", json!("Jamf Protect"))?;

            event.set("observer.vendor", json!("Jamf"))?;

            event.set("observer.type", json!("Endpoint Security"))?;

            event.set("event.kind", json!("alert"))?;

            event.set("event.provider", json!("Jamf Protect"))?;

            let _cond = { event.has_value("jamf_protect.threat.event.timestamp") };
            if _cond {
                event.rename("jamf_protect.threat.event.timestamp", "event.start")?;
            }

            event.append("event.category", json!("host"))?;

            let _cond =
                { event.get_str("jamf_protect.threat.event.eventType.name") == Some("MALWARE") };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.action") };
            if _cond {
                if event.has_value("jamf_protect.threat.event.action") {
                    event.rename("jamf_protect.threat.event.action", "event.action")?;
                }
            }

            let _cond = { event.has_value("jamf_protect.threat.event.eventType.description") };
            if _cond {
                event.set(
                    "event.reason",
                    json!(
                        event
                            .get("jamf_protect.threat.event.eventType.description")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.alertId") };
            if _cond {
                if event.has_value("jamf_protect.threat.event.alertId") {
                    event.rename("jamf_protect.threat.event.alertId", "event.id")?;
                }
            }

            let _cond = { event.has_value("jamf_protect.threat.event.severity") };
            if _cond {
                if event.has_value("jamf_protect.threat.event.severity") {
                    event.rename("jamf_protect.threat.event.severity", "event.severity")?;
                }
            }

            let _cond = { event.has_value("jamf_protect.threat.event.eventUrl") };
            if _cond {
                if event.has_value("jamf_protect.threat.event.eventUrl") {
                    event.rename("jamf_protect.threat.event.eventUrl", "event.url")?;
                }
            }

            let _cond = { event.has_value("jamf_protect.threat.event.eventType.name") };
            if _cond {
                if event.has_value("jamf_protect.threat.event.eventType.name") {
                    event.rename("jamf_protect.threat.event.eventType.name", "rule.name")?;
                }
            }

            let _cond = { event.has_value("jamf_protect.threat.event.eventType.description") };
            if _cond {
                event.set(
                    "rule.description",
                    json!(
                        event
                            .get("jamf_protect.threat.event.eventType.description")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.eventType.id") };
            if _cond {
                if event.has_value("jamf_protect.threat.event.eventType.id.toString()") {
                    event.rename(
                        "jamf_protect.threat.event.eventType.id.toString()",
                        "rule.id",
                    )?;
                }
            }

            let _cond = { event.has_value("jamf_protect.threat.event.device.userDeviceName") };
            if _cond {
                if event.has_value("jamf_protect.threat.event.device.userDeviceName") {
                    event.rename(
                        "jamf_protect.threat.event.device.userDeviceName",
                        "host.hostname",
                    )?;
                }
            }

            let _cond = {
                !event.has_value("jamf_protect.threat.event.device.deviceName")
                    && event.has_value("jamf_protect.threat.event.device.userDeviceName")
            };
            if _cond {
                if event.has_value("jamf_protect.threat.event.device.deviceName") {
                    event.rename(
                        "jamf_protect.threat.event.device.deviceName",
                        "host.hostname",
                    )?;
                }
            }

            let _cond = { event.has_value("jamf_protect.threat.event.device.deviceId") };
            if _cond {
                if event.has_value("jamf_protect.threat.event.device.deviceId") {
                    event.rename("jamf_protect.threat.event.device.deviceId", "host.id")?;
                }
            }

            let _cond = { event.has_value("jamf_protect.threat.event.device.os") };
            if _cond {
                if event.has_value("jamf_protect.threat.event.device.os") {
                    event.rename("jamf_protect.threat.event.device.os", "host.os.full")?;
                }
            }

            let _cond = { event.has_value("jamf_protect.threat.event.user.name") };
            if _cond {
                event.rename("jamf_protect.threat.event.user.name", "user.name")?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.user.email") };
            if _cond {
                event.rename("jamf_protect.threat.event.user.email", "user.email")?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.location") };
            if _cond {
                event.rename(
                    "jamf_protect.threat.event.location",
                    "host.geo.country_iso_code",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.destination.ip") };
            if _cond {
                event.rename(
                    "jamf_protect.threat.event.destination.ip",
                    "destination.address",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.destination.name") };
            if _cond {
                event.rename(
                    "jamf_protect.threat.event.destination.name",
                    "destination.domain",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.destination.port") };
            if _cond {
                event.rename(
                    "jamf_protect.threat.event.destination.port",
                    "destination.port",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.app.name") };
            if _cond {
                event.rename("jamf_protect.threat.event.app.name", "file.name")?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.app.sha1") };
            if _cond {
                event.rename("jamf_protect.threat.event.app.sha1", "file.hash.sha1")?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.app.sha256") };
            if _cond {
                event.rename("jamf_protect.threat.event.app.sha256", "file.hash.sha256")?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.destination.ip") };
            if _cond {
                event.rename("jamf_protect.threat.event.source.ip", "source.address")?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.source.port") };
            if _cond {
                event.rename("jamf_protect.threat.event.source.port", "source.port")?;
            }

            let _cond = { event.has_value("jamf_protect.threat.event.account.customerId") };
            if _cond {
                if event.has_value("jamf_protect.threat.event.account.customerId") {
                    event.rename(
                        "jamf_protect.threat.event.account.customerId",
                        "organization.id",
                    )?;
                }
            }

            event.remove("jamf_protect.threat");
            event.remove("jamf_protect");
            event.remove("message");

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\nif (object == null || object == '') {\n    return true;\n} else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n} else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n}\nreturn false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\nif (object == null || object == '') {\n    return true;\n} else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n} else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n}\nreturn false;\n}\ndropEmptyFields(ctx);\n"#
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
