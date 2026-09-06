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
                parse_json_field(event, "message", "jamf_protect.traffic")?;
            }

            let _cond = { event.has_value("json") };
            if _cond {
                event.rename("json", "jamf_protect.traffic")?;
            }

            event.set("observer.product", json!("Jamf Protect"))?;

            event.set("observer.vendor", json!("Jamf"))?;

            event.set("observer.type", json!("Endpoint Security"))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.provider", json!("Jamf Protect"))?;

            let _cond = { event.has_value("jamf_protect.traffic.event.timestamp") };
            if _cond {
                event.rename("jamf_protect.traffic.event.timestamp", "event.start")?;
            }

            event.append("event.category", json!("host"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("connection"))?;

            let _cond =
                { event.get_str("jamf_protect.traffic.event.threat.types") == Some("malware") };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.signatureId.name") };
            if _cond {
                event.set(
                    "event.action",
                    json!(
                        event
                            .get("jamf_protect.traffic.event.signatureId.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.threat.result") };
            if _cond {
                event.set(
                    "event.reason",
                    json!(
                        event
                            .get("jamf_protect.traffic.event.threat.result")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_bool("jamf_protect.traffic.event.blocked") == Some(true) };
            if _cond {
                event.append("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_bool("jamf_protect.traffic.event.blocked") == Some(false) };
            if _cond {
                event.append("event.outcome", json!("success"))?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.signatureId.name") };
            if _cond {
                event.set(
                    "rule.name",
                    json!(
                        event
                            .get("jamf_protect.traffic.event.signatureId.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.signatureId.id") };
            if _cond {
                if event.has_value("jamf_protect.traffic.event.signatureId.id.toString()") {
                    event.rename(
                        "jamf_protect.traffic.event.signatureId.id.toString()",
                        "rule.id",
                    )?;
                }
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.device.userDeviceName") };
            if _cond {
                if event.has_value("jamf_protect.traffic.event.device.userDeviceName") {
                    event.rename(
                        "jamf_protect.traffic.event.device.userDeviceName",
                        "host.hostname",
                    )?;
                }
            }

            let _cond = {
                !event.has_value("jamf_protect.traffic.event.device.deviceName")
                    && event.has_value("jamf_protect.traffic.event.device.userDeviceName")
            };
            if _cond {
                if event.has_value("jamf_protect.traffic.event.device.deviceName") {
                    event.rename(
                        "jamf_protect.traffic.event.device.deviceName",
                        "host.hostname",
                    )?;
                }
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.device.deviceId") };
            if _cond {
                if event.has_value("jamf_protect.traffic.event.device.deviceId") {
                    event.rename("jamf_protect.traffic.event.device.deviceId", "host.id")?;
                }
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.device.os") };
            if _cond {
                if event.has_value("jamf_protect.traffic.event.device.os") {
                    event.rename("jamf_protect.traffic.event.device.os", "host.os.full")?;
                }
            }

            let _cond =
                { event.get_str("jamf_protect.traffic.event.device.osType") == Some("IOS") };
            if _cond {
                event.append("host.os.type", json!("ios"))?;
            }

            let _cond =
                { event.get_str("jamf_protect.traffic.event.device.osType") == Some("MAC_OS") };
            if _cond {
                event.append("host.os.type", json!("macos"))?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.user.name") };
            if _cond {
                event.rename("jamf_protect.traffic.event.user.name", "user.name")?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.user.email") };
            if _cond {
                event.rename("jamf_protect.traffic.event.user.email", "user.email")?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.tld") };
            if _cond {
                event.rename(
                    "jamf_protect.traffic.event.tld",
                    "dns.question.top_level_domain",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.domain") };
            if _cond {
                event.rename(
                    "jamf_protect.traffic.event.domain",
                    "dns.question.registered_domain",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.hostName") };
            if _cond {
                event.rename("jamf_protect.traffic.event.hostName", "dns.question.name")?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.dns.responseStatus") };
            if _cond {
                event.rename(
                    "jamf_protect.traffic.event.dns.responseStatus",
                    "dns.response_code",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.dns.recordType") };
            if _cond {
                event.rename(
                    "jamf_protect.traffic.event.dns.recordType",
                    "dns.answers.type",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.dns.ttl") };
            if _cond {
                event.rename("jamf_protect.traffic.event.dns.ttl", "dns.answers.ttl")?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.destination.ip") };
            if _cond {
                event.set(
                    "dns.resolved_ip",
                    json!(
                        event
                            .get("jamf_protect.traffic.event.destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.location") };
            if _cond {
                event.rename(
                    "jamf_protect.traffic.event.location",
                    "host.geo.country_iso_code",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.destination.ip") };
            if _cond {
                event.rename(
                    "jamf_protect.traffic.event.destination.ip",
                    "destination.address",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.destination.name") };
            if _cond {
                event.rename(
                    "jamf_protect.traffic.event.destination.name",
                    "destination.domain",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.destination.port") };
            if _cond {
                event.rename(
                    "jamf_protect.traffic.event.destination.port",
                    "destination.port",
                )?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.destination.ip") };
            if _cond {
                event.rename("jamf_protect.traffic.event.source.ip", "source.address")?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.source.port") };
            if _cond {
                event.rename("jamf_protect.traffic.event.source.port", "source.port")?;
            }

            let _cond = { event.has_value("jamf_protect.traffic.event.account.customerId") };
            if _cond {
                if event.has_value("jamf_protect.traffic.event.account.customerId") {
                    event.rename(
                        "jamf_protect.traffic.event.account.customerId",
                        "organization.id",
                    )?;
                }
            }

            event.remove("jamf_protect.traffic");
            event.remove("jamf_protect");
            event.remove("message");

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\nif (object == null || object == '') {\n    return true;\n} else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n} else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n}\nreturn false;\n}\ndropEmptyFields(ctx);\n
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
