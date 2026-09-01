// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `cp_pipeline` pipeline.
pub struct CpPipeline;

impl Transform for CpPipeline {
    fn name(&self) -> &str {
        "cp_pipeline"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                // Painless script
                // Source: def actions = new ArrayList();\ndef exts = ctx.cef?.extensions;\nif (exts == null) return;\nfor (entry in params.extensions) {\n  def value = exts[entry.name];\n  if (value == null ||\n    (entry.convert != null &&\n      (value=entry.convert[value.toLowerCase()]) == null))\n    continue;\n  if (entry.to != null) {\n    actions.add([\n      \"value\": value,\n      \"to\": entry.to\n    ]);\n    continue;\n  }\n  def label = exts[entry.name + \"Label\"];\n  if (label == null) continue;\n  def dest = entry.labels[label.toLowerCase()];\n  if (dest == null) continue;\n  actions.add([\n    \"value\": value,\n    \"to\": dest\n  ]);\n}\nctx[\"_tmp_copy\"] = actions;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def actions = new ArrayList();\ndef exts = ctx.cef?.extensions;\nif (exts == null) return;\nfor (entry in params.extensions) {\n  def value = exts[entry.name];\n  if (value == null ||\n    (entry.convert != null &&\n      (value=entry.convert[value.toLowerCase()]) == null))\n    continue;\n  if (entry.to != null) {\n    actions.add([\n      \"value\": value,\n      \"to\": entry.to\n    ]);\n    continue;\n  }\n  def label = exts[entry.name + \"Label\"];\n  if (label == null) continue;\n  def dest = entry.labels[label.toLowerCase()];\n  if (dest == null) continue;\n  actions.add([\n    \"value\": value,\n    \"to\": dest\n  ]);\n}\nctx[\"_tmp_copy\"] = actions;\n"#), cached_params!("{\"extensions\":[{\"name\":\"cp_app_risk\",\"to\":\"checkpoint.app_risk\"},{\"name\":\"cp_app_risk\",\"to\":\"event.risk_score\",\"convert\":{\"unknown\":0,\"informational\":0,\"very-low\":1,\"low\":2,\"medium\":3,\"high\":4,\"very-high\":5,\"critical\":5}},{\"name\":\"cp_severity\",\"to\":\"checkpoint.severity\"},{\"name\":\"cp_severity\",\"to\":\"event.severity\",\"convert\":{\"unknown\":0,\"informational\":0,\"very-low\":1,\"low\":1,\"medium\":2,\"high\":3,\"very-high\":4,\"critical\":4}},{\"name\":\"baseEventCount\",\"to\":\"checkpoint.event_count\"},{\"name\":\"deviceExternalId\",\"to\":\"observer.type\"},{\"name\":\"deviceFacility\",\"to\":\"observer.type\",\"convert\":{\"0\":\"Network\",\"1\":\"Endpoint\",\"2\":\"Access\",\"3\":\"Threat\",\"4\":\"Mobile\"}},{\"name\":\"deviceInboundInterface\",\"to\":\"observer.ingress.interface.name\"},{\"name\":\"deviceOutboundInterface\",\"to\":\"observer.egress.interface.name\"},{\"name\":\"externalId\",\"to\":\"checkpoint.uuid\"},{\"name\":\"fileHash\",\"to\":\"checkpoint.file_hash\"},{\"name\":\"reason\",\"to\":\"checkpoint.termination_reason\"},{\"name\":\"requestCookies\",\"to\":\"checkpoint.cookie\"},{\"name\":\"checkrequestCookies\",\"to\":\"checkpoint.cookie\"},{\"name\":\"sourceNtDomain\",\"to\":\"dns.question.name\"},{\"name\":\"Signature\",\"to\":\"vulnerability.id\"},{\"name\":\"Recipient\",\"to\":\"destination.user.email\"},{\"name\":\"Sender\",\"to\":\"source.user.email\"},{\"name\":\"deviceCustomFloatingPoint1\",\"labels\":{\"update version\":\"observer.version\"}},{\"name\":\"deviceCustomIPv6Address2\",\"labels\":{\"source ipv6 address\":\"source.ip\"}},{\"name\":\"deviceCustomIPv6Address3\",\"labels\":{\"destination ipv6 address\":\"destination.ip\"}},{\"name\":\"deviceCustomNumber1\",\"labels\":{\"payload\":\"network.bytes\",\"elapsed time in seconds\":\"event.duration\",\"email recipients number\":\"checkpoint.email_recipients_num\"}},{\"name\":\"deviceCustomNumber2\",\"labels\":{\"duration in seconds\":\"event.duration\",\"icmp type\":\"checkpoint.icmp_type\"}},{\"name\":\"deviceCustomNumber3\",\"labels\":{\"icmp code\":\"checkpoint.icmp_code\"}},{\"name\":\"deviceCustomString1\",\"labels\":{\"application rule name\":\"rule.name\",\"dlp rule name\":\"rule.name\",\"threat prevention rule name\":\"rule.name\",\"connectivity state\":\"checkpoint.connectivity_state\",\"email id\":\"checkpoint.email_id\",\"voip log type\":\"checkpoint.voip_log_type\"}},{\"name\":\"deviceCustomString2\",\"labels\":{\"protection id\":\"checkpoint.protection_id\",\"update status\":\"checkpoint.update_status\",\"email subject\":\"checkpoint.email_subject\",\"sensor mode\":\"checkpoint.sensor_mode\",\"scan invoke type\":\"checkpoint.integrity_av_invoke_type\",\"category\":\"checkpoint.category\",\"categories\":\"rule.category\",\"peer gateway\":\"checkpoint.peer_gateway\"}},{\"name\":\"deviceCustomString6\",\"labels\":{\"application name\":\"network.application\",\"virus name\":\"checkpoint.virus_name\",\"malware name\":\"checkpoint.spyware_name\",\"malware family\":\"checkpoint.malware_family\"}},{\"name\":\"deviceCustomString3\",\"labels\":{\"user group\":\"group.name\",\"incident extension\":\"checkpoint.incident_extension\",\"identity type\":\"checkpoint.identity_type\",\"email spool id\":\"checkpoint.email_spool_id\",\"protection type\":\"checkpoint.protection_type\"}},{\"name\":\"deviceCustomString4\",\"labels\":{\"malware status\":\"checkpoint.spyware_status\",\"destination os\":\"os.name\",\"scan result\":\"checkpoint.scan_result\",\"frequency\":\"checkpoint.frequency\",\"protection name\":\"checkpoint.protection_name\",\"user response\":\"checkpoint.user_status\",\"email control\":\"checkpoint.email_control\",\"tcp flags\":\"checkpoint.tcp_flags\",\"threat prevention rule id\":\"rule.id\"}},{\"name\":\"deviceCustomString5\",\"labels\":{\"matched category\":\"rule.category\",\"authentication method\":\"checkpoint.auth_method\",\"email session id\":\"checkpoint.email_session_id\",\"vlan id\":\"network.vlan.id\"}},{\"name\":\"deviceCustomDate2\",\"labels\":{\"subscription expiration\":\"checkpoint.subs_exp\"}},{\"name\":\"deviceFlexNumber1\",\"labels\":{\"confidence\":\"checkpoint.confidence_level\"}},{\"name\":\"deviceFlexNumber2\",\"labels\":{\"destination phone number\":\"checkpoint.dst_phone_number\",\"performance impact\":\"checkpoint.performance_impact\"}},{\"name\":\"flexString1\",\"labels\":{\"application signature id\":\"checkpoint.app_sig_id\"}},{\"name\":\"flexString2\",\"labels\":{\"malware action\":\"rule.description\",\"attack information\":\"event.action\"}},{\"name\":\"rule_uid\",\"to\":\"rule.uuid\"},{\"name\":\"ifname\",\"to\":\"observer.ingress.interface.name\"},{\"name\":\"inzone\",\"to\":\"observer.ingress.zone\"},{\"name\":\"outzone\",\"to\":\"observer.egress.zone\"},{\"name\":\"product\",\"to\":\"observer.product\"}]}"))?;

                foreach_array(event, "_tmp_copy", |event| {
                    set_templated(event, "{{{_ingest._value.to}}}", json!(event.get("_ingest._value.value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;

                if event.remove("_tmp_copy").is_none() {
                    return Err(TransformError::FieldNotFound { path: "_tmp_copy".into() });
                }

            let _cond = { event.has_value("destination.user.email") };
            if _cond {
            event.set("email.to.address", Value::Array(vec![json!(event.get("destination.user.email").map_or_else(String::new, template_to_string))]))?;
            }

            let _cond = { event.has_value("source.user.email") };
            if _cond {
            event.set("email.from.address", Value::Array(vec![json!(event.get("source.user.email").map_or_else(String::new, template_to_string))]))?;
            }

            let _cond = { event.has_value("checkpoint.email_subject") };
            if _cond {
            if let Some(v) = event.get("checkpoint.email_subject").cloned() {
                event.set("email.subject", v)?;
            }
            }

            let _cond = { event.has_value("checkpoint.email_session_id") };
            if _cond {
            if let Some(v) = event.get("checkpoint.email_session_id").cloned() {
                event.set("email.message_id", v)?;
            }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("event.risk_score") {
                if let Some(val) = event.get("event.risk_score") {
                    let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                            path: "event.risk_score".into(),
                            message,
                        })?;
                    event.set("event.risk_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert risk score")?;
                        if event.remove("event.risk_score").is_none() {
                            return Err(TransformError::FieldNotFound { path: "event.risk_score".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("event.severity") {
                if let Some(val) = event.get("event.severity") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "event.severity".into(),
                            message,
                        })?;
                    event.set("event.severity", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert event.severity")?;
                        if event.remove("event.severity").is_none() {
                            return Err(TransformError::FieldNotFound { path: "event.severity".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def duration = ctx.event?.duration;\nif (duration == null) return;\nctx.event.duration = Long.parseLong(duration) * params.second_to_nanos;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def duration = ctx.event?.duration;\nif (duration == null) return;\nctx.event.duration = Long.parseLong(duration) * params.second_to_nanos;\n"#), cached_params!("{\"second_to_nanos\":1000000000}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "calculate duration")?;
                        event.remove("event.duration");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("checkpoint.file_hash") && event.get_as_string("checkpoint.file_hash").is_some_and(|s| s.len() == 32) };
            if _cond {
                event.rename("checkpoint.file_hash", "file.hash.md5")?;
            }

            let _cond = { event.has_value("checkpoint.file_hash") && event.get_as_string("checkpoint.file_hash").is_some_and(|s| s.len() == 40) };
            if _cond {
                event.rename("checkpoint.file_hash", "file.hash.sha1")?;
            }

            let _cond = { event.has_value("checkpoint.file_hash") && event.get_as_string("checkpoint.file_hash").is_some_and(|s| s.len() == 64) };
            if _cond {
                event.rename("checkpoint.file_hash", "file.hash.sha256")?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("cef.extensions.cp_app_risk") && event.has_value("rule") };
            if _cond {
            event.set("event.kind", json!("alert"))?;
            }

            let _cond = { event.has_value("source.ip") && event.has_value("destination.ip") };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.has_value("checkpoint.protection_id") || event.has_value("checkpoint.spyware_name") || event.has_value("checkpoint.malware_family") || event.has_value("checkpoint.spyware_status") };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = { event.has_value("event.category") && !(event.get("event.action").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("malware")), serde_json::Value::String(s) => s.contains("malware"), _ => false })) && (event.has_value("checkpoint.protection_type") || event.get_str("cef.extensions.flexString2Label") == Some("Attack Information")) };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

            if event.has_value("checkpoint.event_count") {
                if let Some(val) = event.get("checkpoint.event_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "checkpoint.event_count".into(),
                            message,
                        })?;
                    event.set("checkpoint.event_count", converted)?;
                }
            }

            if event.has_value("cef.extensions.baseEventCount") {
                if let Some(val) = event.get("cef.extensions.baseEventCount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cef.extensions.baseEventCount".into(),
                            message,
                        })?;
                    event.set("cef.extensions.baseEventCount", converted)?;
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
