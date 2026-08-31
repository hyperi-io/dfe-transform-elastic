// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `alg_feature` pipeline.
pub struct AlgFeature;

impl Transform for AlgFeature {
    fn name(&self) -> &str {
        "alg_feature"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^Errmsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.errmsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Call_ID%{SPACE}:%{SPACE}%{NOTSPACE:citrix_adc.log.call_id} - Transport%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.transport} - Source_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.source.ip} - Source_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.source.port} - Destination_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.destination.ip} - Destination_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.destination.port} - Natted_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.natted.ip} - Natted_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.natted.port} - Method%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.method} - Sequence_Number%{SPACE}:%{SPACE}%{INT:citrix_adc.log.sequence_number} - Register%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.register} - Content_Type%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.content_type} - Error%{SPACE}:%{SPACE}\\\"%{DATA:citrix_adc.log.error}\\\" - Error_line%{SPACE}:%{SPACE}\\\"%{DATA:citrix_adc.log.error_line}\\\" - Caller_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.caller.user_name} - Callee_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.callee.user_name} - Caller_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.caller.domain_name} - Callee_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.callee.domain_name} -$
                    // Grok pattern: ^Errmsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.errmsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Call_ID%{SPACE}:%{SPACE}%{NOTSPACE:citrix_adc.log.call_id} - Transport%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.transport} - Source_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.source.ip} - Source_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.source.port} - Destination_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.destination.ip} - Destination_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.destination.port} - Natted_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.natted.ip} - Natted_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.natted.port} - Method%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.method} - Sequence_Number%{SPACE}:%{SPACE}%{INT:citrix_adc.log.sequence_number} - Register%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.register} - Content_Type%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.content_type} - Caller_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.caller.user_name} - Callee_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.callee.user_name} - Caller_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.caller.domain_name} - Callee_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.callee.domain_name}$
                    // Grok pattern: ^Errmsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.errmsg}\\\" - Error_Code%{SPACE}:%{SPACE}%{INT:citrix_adc.log.error_code} - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group}$
                    // Grok pattern: ^Infomsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.infomsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Session_ID%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.session_id} -$
                    // Grok pattern: ^Infomsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.infomsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Call_ID%{SPACE}:%{SPACE}%{NOTSPACE:citrix_adc.log.call_id} - Transport%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.transport} - Source_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.source.ip} - Source_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.source.port} - Destination_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.destination.ip} - Destination_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.destination.port} - Natted_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.natted.ip} - Natted_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.natted.port} - Method%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.method} - Sequence_Number%{SPACE}:%{SPACE}%{INT:citrix_adc.log.sequence_number} - Register%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.register} - Content_Type%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.content_type} - Caller_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.caller.user_name} - Callee_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.callee.user_name} - Caller_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.caller.domain_name} - Callee_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.callee.domain_name} -$
                    // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                    if !extract_first_match(
                        &[
                            cached_grok!("^Errmsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.errmsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Call_ID%{SPACE}:%{SPACE}%{NOTSPACE:citrix_adc.log.call_id} - Transport%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.transport} - Source_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.source.ip} - Source_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.source.port} - Destination_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.destination.ip} - Destination_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.destination.port} - Natted_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.natted.ip} - Natted_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.natted.port} - Method%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.method} - Sequence_Number%{SPACE}:%{SPACE}%{INT:citrix_adc.log.sequence_number} - Register%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.register} - Content_Type%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.content_type} - Error%{SPACE}:%{SPACE}\\\"%{DATA:citrix_adc.log.error}\\\" - Error_line%{SPACE}:%{SPACE}\\\"%{DATA:citrix_adc.log.error_line}\\\" - Caller_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.caller.user_name} - Callee_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.callee.user_name} - Caller_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.caller.domain_name} - Callee_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.callee.domain_name} -$"),
                            cached_grok!("^Errmsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.errmsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Call_ID%{SPACE}:%{SPACE}%{NOTSPACE:citrix_adc.log.call_id} - Transport%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.transport} - Source_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.source.ip} - Source_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.source.port} - Destination_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.destination.ip} - Destination_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.destination.port} - Natted_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.natted.ip} - Natted_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.natted.port} - Method%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.method} - Sequence_Number%{SPACE}:%{SPACE}%{INT:citrix_adc.log.sequence_number} - Register%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.register} - Content_Type%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.content_type} - Caller_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.caller.user_name} - Callee_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.callee.user_name} - Caller_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.caller.domain_name} - Callee_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.callee.domain_name}$"),
                            cached_grok!("^Errmsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.errmsg}\\\" - Error_Code%{SPACE}:%{SPACE}%{INT:citrix_adc.log.error_code} - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group}$"),
                            cached_grok!("^Infomsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.infomsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Session_ID%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.session_id} -$"),
                            cached_grok!("^Infomsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.infomsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Call_ID%{SPACE}:%{SPACE}%{NOTSPACE:citrix_adc.log.call_id} - Transport%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.transport} - Source_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.source.ip} - Source_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.source.port} - Destination_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.destination.ip} - Destination_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.destination.port} - Natted_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.natted.ip} - Natted_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.natted.port} - Method%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.method} - Sequence_Number%{SPACE}:%{SPACE}%{INT:citrix_adc.log.sequence_number} - Register%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.register} - Content_Type%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.content_type} - Caller_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.caller.user_name} - Callee_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.callee.user_name} - Caller_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.caller.domain_name} - Callee_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.callee.domain_name} -$"),
                            cached_grok!("%{GREEDYDATA:citrix_adc.log.message}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("citrix_adc.log.destination.ip") && event.get_str("citrix_adc.log.destination.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.destination.ip") {
                if let Some(val) = event.get("citrix_adc.log.destination.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.destination.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.destination.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_destination_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.destination.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.natted.ip") && event.get_str("citrix_adc.log.natted.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.natted.ip") {
                if let Some(val) = event.get("citrix_adc.log.natted.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.natted.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.natted.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_natted_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.natted.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.nat.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.natted.port") {
                if let Some(val) = event.get("citrix_adc.log.natted.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.natted.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.natted.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_natted_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.natted.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.nat.port", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.destination.port") {
                if let Some(val) = event.get("citrix_adc.log.destination.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.destination.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.destination.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_destination_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.destination.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.callee.domain_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.user.domain", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.callee.user_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.user.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.error_code") {
                if let Some(val) = event.get("citrix_adc.log.error_code") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.error_code".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.error_code", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_error_code_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.error_code").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("error.code", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.errmsg").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("error.message", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.group").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("group.name", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.method").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.method", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.infomsg").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("message", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.transport").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.transport", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.source.ip") && event.get_str("citrix_adc.log.source.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.source.ip") {
                if let Some(val) = event.get("citrix_adc.log.source.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.source.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.source.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.source.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.source.port") {
                if let Some(val) = event.get("citrix_adc.log.source.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.source.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.source.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.source.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.caller.domain_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.user.domain", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.caller.user_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.user.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.sequence_number") {
                if let Some(val) = event.get("citrix_adc.log.sequence_number") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.sequence_number".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.sequence_number", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sequence_number_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.session_id") {
                if let Some(val) = event.get("citrix_adc.log.session_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.session_id".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.session_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
