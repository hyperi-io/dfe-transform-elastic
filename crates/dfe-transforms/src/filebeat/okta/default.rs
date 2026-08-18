// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("ecs.version", json!("8.11.0"))?;

        let cond = { event.has("event.original") };
        if cond {
            event.append(
                "error.message",
                json!("event.original is set before start of ingest pipeline"),
            )?;
        }

        let cond = { !event.has("event.original") };
        if cond {
            if event.has("message") {
                event.rename("message", "event.original")?;
            }
        }

        if let Some(s) = event.get_string("event.original") {
            let parsed: Value =
                serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                    path: "event.original".into(),
                    message: format!("failed to parse JSON: {}", e),
                })?;
            event.set("json", parsed)?;
        }

        // Painless script
        // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
        // TODO: Transpile Painless to Rust (2.2.3)
        painless_exec(
            event,
            r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#,
        )?;

        let cond =
            { event.has("json.uuid") && event.get_str("json.uuid").is_some_and(|s| !s.is_empty()) };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("json.uuid") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("_id", converted)?;
                }
                Ok(())
            })();
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.published") {
                // Try ISO8601 format
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                    })
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
                    })
                {
                    event.set(
                        "@timestamp",
                        dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                    )?;
                }
            }
            Ok(())
        })();

        event.set("event.kind", json!("event"))?;

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.displayMessage") {
                event.rename("json.displayMessage", "okta.display_message")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.eventType") {
                event.rename("json.eventType", "okta.event_type")?;
            }
            Ok(())
        })();

        let cond = {
            [
                "group.user_membership.add",
                "group.user_membership.remove",
                "user.lifecycle.activate",
                "user.lifecycle.create",
                "user.lifecycle.deactivate",
                "user.lifecycle.suspend",
                "user.lifecycle.unsuspend",
            ]
            .contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.category", json!("iam"))?;
        }

        let cond = {
            [
                "policy.lifecycle.activate",
                "policy.lifecycle.create",
                "policy.lifecycle.deactivate",
                "policy.lifecycle.delete",
                "policy.lifecycle.update",
                "policy.rule.activate",
                "policy.rule.add",
                "policy.rule.deactivate",
                "policy.rule.delete",
                "application.lifecycle.create",
                "application.lifecycle.delete",
                "policy.rule.update",
                "application.lifecycle.activate",
                "application.lifecycle.deactivate",
                "application.lifecycle.update",
            ]
            .contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.category", json!("configuration"))?;
        }

        let cond = {
            [
                "user.session.start",
                "user.session.end",
                "user.authentication.sso",
                "policy.evaluate_sign_on",
            ]
            .contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.category", json!("authentication"))?;
        }

        let cond = {
            ["user.session.start", "user.session.end"]
                .contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.category", json!("session"))?;
        }

        let cond = {
            [
                "system.org.rate_limit.warning",
                "system.org.rate_limit.violation",
                "core.concurrency.org.limit.violation",
            ]
            .contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.type", json!("info"))?;
        }

        let cond = {
            ["security.request.blocked"].contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.type", json!("network"))?;
        }

        let cond = {
            [
                "system.org.rate_limit.warning",
                "system.org.rate_limit.violation",
                "core.concurrency.org.limit.violation",
                "security.request.blocked",
            ]
            .contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.type", json!("network"))?;
        }

        let cond =
            { ["user.session.start"].contains(&event.get_str("okta.event_type").unwrap_or("")) };
        if cond {
            event.append("event.type", json!("start"))?;
        }

        let cond =
            { ["user.session.end"].contains(&event.get_str("okta.event_type").unwrap_or("")) };
        if cond {
            event.append("event.type", json!("end"))?;
        }

        let cond = {
            ["group.user_membership.add", "group.user_membership.remove"]
                .contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.type", json!("group"))?;
        }

        let cond = {
            [
                "user.lifecycle.activate",
                "user.lifecycle.create",
                "user.lifecycle.deactivate",
                "user.lifecycle.suspend",
                "user.lifecycle.unsuspend",
                "user.authentication.sso",
                "user.session.start",
                "user.session.end",
                "application.user_membership.add",
                "application.user_membership.remove",
                "application.user_membership.change_username",
            ]
            .contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.type", json!("info"))?;
        }

        let cond = {
            [
                "user.lifecycle.activate",
                "user.lifecycle.deactivate",
                "user.lifecycle.suspend",
                "user.lifecycle.unsuspend",
                "group.user_membership.add",
                "group.user_membership.remove",
                "policy.lifecycle.activate",
                "policy.lifecycle.deactivate",
                "policy.lifecycle.update",
                "policy.rule.activate",
                "policy.rule.add",
                "policy.rule.deactivate",
                "policy.rule.update",
                "application.user_membership.add",
                "application.user_membership.remove",
                "application.user_membership.change_username",
            ]
            .contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.type", json!("change"))?;
        }

        let cond = {
            [
                "user.lifecycle.create",
                "policy.lifecycle.create",
                "application.lifecycle.create",
            ]
            .contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.type", json!("creation"))?;
        }

        let cond = {
            ["policy.lifecycle.delete", "application.lifecycle.delete"]
                .contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.type", json!("deletion"))?;
        }

        let cond = {
            ["policy.evaluate_sign_on"].contains(&event.get_str("okta.event_type").unwrap_or(""))
        };
        if cond {
            event.append("event.type", json!("info"))?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.uuid") {
                event.rename("json.uuid", "okta.uuid")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.actor.alternateId") {
                event.rename("json.actor.alternateId", "okta.actor.alternate_id")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("okta.actor.alternate_id") {
                if let Some(input) = event.get_string("okta.actor.alternate_id") {
                    // Grok pattern: %{USER:user.name}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let (grok_pattern, grok_field_map) =
                        grok_to_regex_with_map("%{USER:user.name}");
                    let grok_re = regex::Regex::new(&grok_pattern).unwrap();
                    if let Some(caps) = grok_re.captures(&input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                let field_path =
                                    grok_field_map.get(name).map(|s| s.as_str()).unwrap_or(name);
                                event.set(field_path, m.as_str())?;
                            }
                        }
                    }
                }
            }
            Ok(())
        })();

        let cond = { event.has("user.name") };
        if cond {
            event.set(
                "source.user.name",
                event.get("user.name").cloned().unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("user.name") };
        if cond {
            event.set(
                "client.user.name",
                event.get("user.name").cloned().unwrap_or(Value::Null),
            )?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.actor.displayName") {
                event.rename("json.actor.displayName", "okta.actor.display_name")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.actor.id") {
                event.rename("json.actor.id", "okta.actor.id")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.actor.type") {
                event.rename("json.actor.type", "okta.actor.type")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.client.device") {
                event.rename("json.client.device", "okta.client.device")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.device") {
                event.rename("json.device", "okta.device")?;
            }
            Ok(())
        })();

        let cond = { event.has("okta.device.device_integrator") };
        if cond {
            if let Some(s) = event.get_string("okta.device.device_integrator") {
                let parsed: Value =
                    serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                        path: "okta.device.device_integrator".into(),
                        message: format!("failed to parse JSON: {}", e),
                    })?;
                event.set("okta.device.device_integrator", parsed)?;
            }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.client.geographicalContext.geolocation") {
                event.rename(
                    "json.client.geographicalContext.geolocation",
                    "client.geo.location",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.client.geographicalContext.city") {
                event.rename(
                    "json.client.geographicalContext.city",
                    "client.geo.city_name",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.client.geographicalContext.state") {
                event.rename(
                    "json.client.geographicalContext.state",
                    "client.geo.region_name",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.client.geographicalContext.country") {
                event.rename(
                    "json.client.geographicalContext.country",
                    "client.geo.country_name",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.client.id") {
                event.rename("json.client.id", "okta.client.id")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.client.ipAddress") {
                if let Some(s) = event.get_string("json.client.ipAddress") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "json.client.ipAddress".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("okta.client.ip", s)?;
                }
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.client.userAgent.browser") {
                event.rename(
                    "json.client.userAgent.browser",
                    "okta.client.user_agent.browser",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.client.userAgent.os") {
                event.rename("json.client.userAgent.os", "okta.client.user_agent.os")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.client.userAgent.rawUserAgent") {
                event.rename(
                    "json.client.userAgent.rawUserAgent",
                    "okta.client.user_agent.raw_user_agent",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.client.zone") {
                event.rename("json.client.zone", "okta.client.zone")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.outcome.reason") {
                event.rename("json.outcome.reason", "okta.outcome.reason")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.outcome.result") {
                event.rename("json.outcome.result", "okta.outcome.result")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.target") {
                event.rename("json.target", "okta.target")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.transaction.id") {
                event.rename("json.transaction.id", "okta.transaction.id")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.transaction.type") {
                event.rename("json.transaction.type", "okta.transaction.type")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.transaction.detail.requestApiTokenId") {
                event.rename(
                    "json.transaction.detail.requestApiTokenId",
                    "okta.transaction.detail.request_api_token_id",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.set(
                "okta.debug_context.debug_data.flattened",
                event
                    .get("json.debugContext.debugData")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(s) =
                event.get_string("okta.debug_context.debug_data.flattened.logOnlySecurityData")
            {
                let parsed: Value =
                    serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                        path: "okta.debug_context.debug_data.flattened.logOnlySecurityData".into(),
                        message: format!("failed to parse JSON: {}", e),
                    })?;
                event.set(
                    "okta.debug_context.debug_data.flattened.logOnlySecurityData",
                    parsed,
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("okta.debug_context.debug_data.flattened.behaviors") {
                if let Some(input) =
                    event.get_string("okta.debug_context.debug_data.flattened.behaviors")
                {
                    let mut remaining: &str = &input;
                    if let Some(rest) = remaining.strip_prefix("{") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("}") {
                        event.set(
                            "okta.debug_context.debug_data.flattened.behaviors",
                            &remaining[..pos],
                        )?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("}") {
                        remaining = rest;
                    }
                }
            }
            Ok(())
        })();

        let cond = { event.has("okta.debug_context.debug_data.flattened.behaviors") };
        if cond {
            if let Some(kv_str) =
                event.get_string("okta.debug_context.debug_data.flattened.behaviors")
            {
                for pair in kv_str.split(", ") {
                    if let Some((key, value)) = pair.split_once("=") {
                        if !key.is_empty() {
                            event.set(&format!("_behaviors_object.{}", key), value)?;
                        }
                    }
                }
            }
        }

        let cond = { event.has("_behaviors_object") };
        if cond {
            if event
                .remove("okta.debug_context.debug_data.flattened.behaviors")
                .is_none()
            {
                return Err(TransformError::FieldNotFound {
                    path: "okta.debug_context.debug_data.flattened.behaviors".into(),
                });
            }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("_behaviors_object") {
                event.rename(
                    "_behaviors_object",
                    "okta.debug_context.debug_data.flattened.behaviors",
                )?;
            }
            Ok(())
        })();

        let cond = { event.has("okta.debug_context.debug_data.flattened.risk") };
        if cond {
            event.set(
                "okta.debug_context.debug_data.flattened.risk_object",
                event
                    .get("okta.debug_context.debug_data.flattened.risk")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("okta.debug_context.debug_data.flattened.risk") {
                if let Some(input) =
                    event.get_string("okta.debug_context.debug_data.flattened.risk")
                {
                    let mut remaining: &str = &input;
                    if let Some(rest) = remaining.strip_prefix("{") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("}") {
                        event.set(
                            "okta.debug_context.debug_data.flattened.risk",
                            &remaining[..pos],
                        )?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("}") {
                        remaining = rest;
                    }
                }
            }
            Ok(())
        })();

        let cond = { event.has("okta.debug_context.debug_data.flattened.risk") };
        if cond {
            if let Some(kv_str) = event.get_string("okta.debug_context.debug_data.flattened.risk") {
                for pair in kv_str.split(", ") {
                    if let Some((key, value)) = pair.split_once("=") {
                        if !key.is_empty() {
                            event.set(&format!("_risk_object.{}", key), value)?;
                        }
                    }
                }
            }
        }

        let cond = { event.has("_risk_object") };
        if cond {
            if event
                .remove("okta.debug_context.debug_data.flattened.risk_object")
                .is_none()
            {
                return Err(TransformError::FieldNotFound {
                    path: "okta.debug_context.debug_data.flattened.risk_object".into(),
                });
            }
        }

        let cond = {
            event.has("okta.debug_context.debug_data.flattened.risk_object")
                && event.has("okta.debug_context.debug_data.flattened.risk")
        };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) =
                    event.get_string("okta.debug_context.debug_data.flattened.risk")
                {
                    // Grok pattern: level=%{NOTSPACE:_risk_object.level}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let (grok_pattern, grok_field_map) =
                        grok_to_regex_with_map("level=%{NOTSPACE:_risk_object.level}");
                    let grok_re = regex::Regex::new(&grok_pattern).unwrap();
                    if let Some(caps) = grok_re.captures(&input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                let field_path =
                                    grok_field_map.get(name).map(|s| s.as_str()).unwrap_or(name);
                                event.set(field_path, m.as_str())?;
                            }
                        }
                    }
                }
                Ok(())
            })();
        }

        // Hand-tuned: extract reasons from risk KV string
        // The codegen KV parser truncates at commas inside values.
        // This re-extracts the full reasons value from the original risk string.
        if let Some(risk_str) = event.get_string("okta.debug_context.debug_data.flattened.risk") {
            if let Some(reasons_start) = risk_str.find("reasons=") {
                let after = &risk_str[reasons_start + 8..];
                let reasons_end = regex::Regex::new(r", \w+=")
                    .ok()
                    .and_then(|re| re.find(after))
                    .map(|m| m.start())
                    .unwrap_or(after.len());
                let reasons = after[..reasons_end].trim();
                if !reasons.is_empty() {
                    let _ = event.set("_risk_object.reasons", reasons);
                }
            }
        }

        let cond = { event.has("_risk_object") };
        if cond {
            if event
                .remove("okta.debug_context.debug_data.flattened.risk")
                .is_none()
            {
                return Err(TransformError::FieldNotFound {
                    path: "okta.debug_context.debug_data.flattened.risk".into(),
                });
            }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("_risk_object") {
                event.rename(
                    "_risk_object",
                    "okta.debug_context.debug_data.flattened.risk",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.debugContext.debugData.deviceFingerprint") {
                event.rename(
                    "json.debugContext.debugData.deviceFingerprint",
                    "okta.debug_context.debug_data.device_fingerprint",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.debugContext.debugData.requestId") {
                event.rename(
                    "json.debugContext.debugData.requestId",
                    "okta.debug_context.debug_data.request_id",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.debugContext.debugData.requestUri") {
                event.rename(
                    "json.debugContext.debugData.requestUri",
                    "okta.debug_context.debug_data.request_uri",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.debugContext.debugData.threatSuspected") {
                event.rename(
                    "json.debugContext.debugData.threatSuspected",
                    "okta.debug_context.debug_data.threat_suspected",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.debugContext.debugData.url") {
                event.rename(
                    "json.debugContext.debugData.url",
                    "okta.debug_context.debug_data.url",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.debugContext.debugData.dtHash") {
                event.rename(
                    "json.debugContext.debugData.dtHash",
                    "okta.debug_context.debug_data.dt_hash",
                )?;
            }
            Ok(())
        })();

        let cond = {
            event.has("okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.level")
                && event
                    .get_str(
                        "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.level",
                    )
                    .is_some_and(|s| !s.is_empty())
        };
        if cond {
            event.set(
                "okta.debug_context.debug_data.risk_level",
                event
                    .get("okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.level")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = {
            event.has("okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.reasons")
                && event
                    .get_str(
                        "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.reasons",
                    )
                    .is_some_and(|s| !s.is_empty())
        };
        if cond {
            if let Some(s) = event.get_string(
                "okta.debug_context.debug_data.flattened.logOnlySecurityData.risk.reasons",
            ) {
                let parts: Vec<Value> = s.split(", ").map(|p| json!(p.trim())).collect();
                event.set(
                    "okta.debug_context.debug_data.risk_reasons",
                    Value::Array(parts),
                )?;
            }
        }

        let cond = {
            !event.has("okta.debug_context.debug_data.risk_level")
                && event.has("okta.debug_context.debug_data.flattened.risk.level")
                && event
                    .get_str("okta.debug_context.debug_data.flattened.risk.level")
                    .is_some_and(|s| !s.is_empty())
        };
        if cond {
            event.set(
                "okta.debug_context.debug_data.risk_level",
                event
                    .get("okta.debug_context.debug_data.flattened.risk.level")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = {
            !event.has("okta.debug_context.debug_data.factor")
                && event.has("okta.debug_context.debug_data.flattened.factor")
                && event
                    .get_str("okta.debug_context.debug_data.flattened.factor")
                    .is_some_and(|s| !s.is_empty())
        };
        if cond {
            event.set(
                "okta.debug_context.debug_data.factor",
                event
                    .get("okta.debug_context.debug_data.flattened.factor")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = {
            !event.has("okta.debug_context.debug_data.risk_reasons")
                && event.has("okta.debug_context.debug_data.flattened.risk.reasons")
                && event
                    .get_str("okta.debug_context.debug_data.flattened.risk.reasons")
                    .is_some_and(|s| !s.is_empty())
        };
        if cond {
            if let Some(s) =
                event.get_string("okta.debug_context.debug_data.flattened.risk.reasons")
            {
                let parts: Vec<Value> = s.split(", ").map(|p| json!(p.trim())).collect();
                event.set(
                    "okta.debug_context.debug_data.risk_reasons",
                    Value::Array(parts),
                )?;
            }
        }

        // Painless script
        // Source: def src = ctx.okta?.debug_context?.debug_data?.flattened?.behaviors;\nif (src == null) {\n  return;\n}\ndef dst = new ArrayList();\nfor (e in src.entrySet()) {\n  if (e != null && e.getValue() == \"POSITIVE\") {\n    dst.add(e.getKey());\n  }\n}\nif (dst.length != 0) {\n  ctx.okta.debug_context.debug_data['risk_behaviors'] = dst;\n}\n
        // TODO: Transpile Painless to Rust (2.2.3)
        painless_exec(
            event,
            r#"def src = ctx.okta?.debug_context?.debug_data?.flattened?.behaviors;\nif (src == null) {\n  return;\n}\ndef dst = new ArrayList();\nfor (e in src.entrySet()) {\n  if (e != null && e.getValue() == \"POSITIVE\") {\n    dst.add(e.getKey());\n  }\n}\nif (dst.length != 0) {\n  ctx.okta.debug_context.debug_data['risk_behaviors'] = dst;\n}\n"#,
        )?;

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.authenticationContext.authenticationProvider") {
                event.rename(
                    "json.authenticationContext.authenticationProvider",
                    "okta.authentication_context.authentication_provider",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.authenticationContext.authenticationStep") {
                event.rename(
                    "json.authenticationContext.authenticationStep",
                    "okta.authentication_context.authentication_step",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.authenticationContext.credentialProvider") {
                event.rename(
                    "json.authenticationContext.credentialProvider",
                    "okta.authentication_context.credential_provider",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.authenticationContext.credentialType") {
                event.rename(
                    "json.authenticationContext.credentialType",
                    "okta.authentication_context.credential_type",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.authenticationContext.externalSessionId") {
                event.rename(
                    "json.authenticationContext.externalSessionId",
                    "okta.authentication_context.external_session_id",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.authenticationContext.interface") {
                event.rename(
                    "json.authenticationContext.interface",
                    "okta.authentication_context.authentication_provider",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.authenticationContext.issuer") {
                event.rename(
                    "json.authenticationContext.issuer",
                    "okta.authentication_context.issuer",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.securityContext.asNumber") {
                event.rename(
                    "json.securityContext.asNumber",
                    "okta.security_context.as.number",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.securityContext.asOrg") {
                event.rename(
                    "json.securityContext.asOrg",
                    "okta.security_context.as.organization.name",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.securityContext.domain") {
                event.rename(
                    "json.securityContext.domain",
                    "okta.security_context.domain",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.securityContext.isProxy") {
                event.rename(
                    "json.securityContext.isProxy",
                    "okta.security_context.is_proxy",
                )?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.securityContext.isp") {
                event.rename("json.securityContext.isp", "okta.security_context.isp")?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("json.request.ipChain") {
                event.rename("json.request.ipChain", "okta.request.ip_chain")?;
            }
            Ok(())
        })();

        // Hand-tuned: convert camelCase keys in ip_chain array elements to snake_case
        // Replaces the foreach processor which uses _ingest._value (not supported)
        if let Some(val) = event.get("okta.request.ip_chain").cloned() {
            let converted = painless_keys_to_snake_case(&val);
            event.set("okta.request.ip_chain", converted)?;
        }

        // foreach processors for ip_chain key renames replaced by keys_to_snake_case above

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(val) = event.get("okta.client.user_agent.raw_user_agent") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("user_agent.original", converted)?;
            }
            Ok(())
        })();

        let cond = { event.has("okta.client.ip") };
        if cond {
            event.set(
                "client.ip",
                event.get("okta.client.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("okta.client.ip") };
        if cond {
            event.set(
                "source.ip",
                event.get("okta.client.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(val) = event.get("okta.event_type") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("event.action", converted)?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(val) = event.get("okta.security_context.as.organization.name") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("client.as.organization.name", converted)?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(val) = event.get("okta.security_context.domain") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("client.domain", converted)?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(val) = event.get("okta.security_context.domain") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("source.domain", converted)?;
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(val) = event.get("okta.uuid") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("event.id", converted)?;
            }
            Ok(())
        })();

        if event.has("okta.outcome.result") {
            if let Some(s) = event.get_string("okta.outcome.result") {
                let lowered = s.to_lowercase();
                event.set("okta.outcome.result_lower", lowered)?;
            }
        }

        let cond = {
            event.has("okta.outcome.result_lower")
                && (event.get_str("okta.outcome.result_lower") == Some("success")
                    || event.get_str("okta.outcome.result_lower") == Some("allow"))
        };
        if cond {
            event.set("event.outcome", json!("success"))?;
        }

        let cond = {
            event.has("okta.outcome.result_lower")
                && (event.get_str("okta.outcome.result_lower") == Some("failure")
                    || event.get_str("okta.outcome.result_lower") == Some("deny"))
        };
        if cond {
            event.set("event.outcome", json!("failure"))?;
        }

        let cond = { !event.has("event.outcome") };
        if cond {
            event.set("event.outcome", json!("unknown"))?;
        }

        event.remove("okta.outcome.result_lower");

        // Painless script
        // Source: def arr = ctx.okta?.target;\nif (arr != null) {\n  for (def i = 0; i < arr.length; i++) {\n    arr[i][\"alternate_id\"] = arr[i][\"alternateId\"];\n    arr[i].remove(\"alternateId\");\n    arr[i][\"display_name\"] = arr[i][\"displayName\"];\n    arr[i].remove(\"displayName\");\n    def de = arr[i].get(\"detailEntry\");\n    if (de != null) {\n      de.entrySet().removeIf(entry -> \n        entry.getKey() != \"methodTypeUsed\" && \n        entry.getKey() != \"methodUsedVerifiedProperties\");\n      if (de.size() == 0) {\n        arr[i].remove(\"detailEntry\");\n      }\n    }\n  }\n\n  for (def i = 0; i < arr.length; i++) {\n    if (arr[i][\"type\"].toLowerCase() == \"user\") {\n      ctx[\"okta_target_user\"] = arr[i];\n      break;\n    }\n  }\n\n  for (def i = 0; i < arr.length; i++) {\n    if (arr[i][\"type\"].toLowerCase() == \"usergroup\") {\n      ctx[\"okta_target_group\"] = arr[i];\n      break;\n    }\n  }\n}\n
        // TODO: Transpile Painless to Rust (2.2.3)
        painless_exec(
            event,
            r#"def arr = ctx.okta?.target;\nif (arr != null) {\n  for (def i = 0; i < arr.length; i++) {\n    arr[i][\"alternate_id\"] = arr[i][\"alternateId\"];\n    arr[i].remove(\"alternateId\");\n    arr[i][\"display_name\"] = arr[i][\"displayName\"];\n    arr[i].remove(\"displayName\");\n    def de = arr[i].get(\"detailEntry\");\n    if (de != null) {\n      de.entrySet().removeIf(entry -> \n        entry.getKey() != \"methodTypeUsed\" && \n        entry.getKey() != \"methodUsedVerifiedProperties\");\n      if (de.size() == 0) {\n        arr[i].remove(\"detailEntry\");\n      }\n    }\n  }\n\n  for (def i = 0; i < arr.length; i++) {\n    if (arr[i][\"type\"].toLowerCase() == \"user\") {\n      ctx[\"okta_target_user\"] = arr[i];\n      break;\n    }\n  }\n\n  for (def i = 0; i < arr.length; i++) {\n    if (arr[i][\"type\"].toLowerCase() == \"usergroup\") {\n      ctx[\"okta_target_group\"] = arr[i];\n      break;\n    }\n  }\n}\n"#,
        )?;

        let cond = { event.has("okta_target_user.display_name") };
        if cond {
            event.set(
                "user.target.full_name",
                event
                    .get("okta_target_user.display_name")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("okta_target_user.id") };
        if cond {
            event.set(
                "user.target.id",
                event
                    .get("okta_target_user.id")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("okta_target_user.login") };
        if cond {
            event.set(
                "user.target.email",
                event
                    .get("okta_target_user.login")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("okta_target_group.display_name") };
        if cond {
            event.set(
                "user.target.group.name",
                event
                    .get("okta_target_group.display_name")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("okta_target_group.id") };
        if cond {
            event.set(
                "user.target.group.id",
                event
                    .get("okta_target_group.id")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        event.remove("okta_target_user");
        event.remove("okta_target_group");

        let cond = { event.has("okta.actor.id") };
        if cond {
            event.set(
                "client.user.id",
                event.get("okta.actor.id").cloned().unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("okta.actor.id") };
        if cond {
            event.set(
                "source.user.id",
                event.get("okta.actor.id").cloned().unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("okta.actor.display_name") };
        if cond {
            event.set(
                "client.user.full_name",
                event
                    .get("okta.actor.display_name")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("okta.actor.display_name") };
        if cond {
            event.set(
                "source.user.full_name",
                event
                    .get("okta.actor.display_name")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("okta.actor.display_name") };
        if cond {
            event.set(
                "user.full_name",
                event
                    .get("okta.actor.display_name")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("okta.actor.display_name") };
        if cond {
            event.append(
                "related.user",
                event
                    .get("okta.actor.display_name")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("user.target.full_name") };
        if cond {
            event.append(
                "related.user",
                event
                    .get("user.target.full_name")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("user.name") };
        if cond {
            event.append(
                "related.user",
                event.get("user.name").cloned().unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("source.ip") };
        if cond {
            event.append(
                "related.ip",
                event.get("source.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("destination.ip") };
        if cond {
            event.append(
                "related.ip",
                event.get("destination.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        // Dedup related arrays (same user/IP can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }

        event.remove("json");

        if event.has("user_agent.original") {
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

        if event.has("source.ip") {
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

        if event.has("destination.ip") {
            if let Some(ip_str) = event.get_string("destination.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-City.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                    if let Some(v) = geo.get("country_iso_code") {
                        event.set("destination.geo.country_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("country_name") {
                        event.set("destination.geo.country_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("continent_name") {
                        event.set("destination.geo.continent_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_iso_code") {
                        event.set("destination.geo.region_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_name") {
                        event.set("destination.geo.region_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("city_name") {
                        event.set("destination.geo.city_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("timezone") {
                        event.set("destination.geo.timezone", v.clone())?;
                    }
                    if let Some(v) = geo.get("location") {
                        event.set("destination.geo.location", v.clone())?;
                    }
                }
            }
        }

        if event.has("source.ip") {
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

        if event.has("destination.ip") {
            if let Some(ip_str) = event.get_string("destination.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-ASN.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                    if let Some(v) = geo.get("asn") {
                        event.set("destination.as.asn", v.clone())?;
                    }
                    if let Some(v) = geo.get("organization_name") {
                        event.set("destination.as.organization_name", v.clone())?;
                    }
                }
            }
        }

        if event.has("source.as.asn") {
            event.rename("source.as.asn", "source.as.number")?;
        }

        if event.has("source.as.organization_name") {
            event.rename("source.as.organization_name", "source.as.organization.name")?;
        }

        if event.has("destination.as.asn") {
            event.rename("destination.as.asn", "destination.as.number")?;
        }

        if event.has("destination.as.organization_name") {
            event.rename(
                "destination.as.organization_name",
                "destination.as.organization.name",
            )?;
        }

        let cond = {
            !event.has("tags")
                || !(event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("preserve_original_event")),
                    serde_json::Value::String(s) => s.contains("preserve_original_event"),
                    _ => false,
                }))
        };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("event.original");
                Ok(())
            })();
        }

        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
