// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `appfw_feature` pipeline.
pub struct AppfwFeature;

impl Transform for AppfwFeature {
    fn name(&self) -> &str {
        "appfw_feature"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^%{IP:source.ip:ip} %{NUMBER:citrix_adc.log.transaction_id}-%{DATA:citrix_adc.log.ppe} - %{NOTSPACE:citrix_adc.log.profile}(?: %{NOTSPACE:url.original})?(?: %{GREEDYDATA:citrix_adc.log.message})?$
                    // Grok pattern: ^XML%{SPACE}Mismatched%{SPACE}content-type%{SPACE}in%{SPACE}HTTP%{SPACE}header%{SPACE}detected%{SPACE}=%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.content_type_mismatch}\\\"\\.$
                    // Grok pattern: ^Disallow%{SPACE}Deny%{SPACE}URL%{SPACE}for%{SPACE}rule%{SPACE}pattern%{SPACE}=%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.url}\\\"\\.$
                    // Grok pattern: ^Unknown%{SPACE}content-type%{SPACE}header%{SPACE}value%{SPACE}=%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.unknown_content_type}\\\"\\.$
                    // Grok pattern: ^parsing%{SPACE}referer%{SPACE}header%{SPACE}\\'%{GREEDYDATA:citrix_adc.log.referer_header}\\'%{SPACE}failed$
                    // Grok pattern: ^URL%{SPACE}length\\(%{NUMBER:citrix_adc.log.url_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.url_length:int}\\)\\.$
                    // Grok pattern: ^Cookie%{SPACE}header%{SPACE}length\\(%{NUMBER:citrix_adc.log.cookie_header_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.cookie_header_length:int}\\)\\.$
                    // Grok pattern: ^Header\\(Referer\\)%{SPACE}length\\(%{NUMBER:citrix_adc.log.header_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.header_length:int}\\)\\.$
                    // Grok pattern: ^Query%{SPACE}string%{SPACE}length\\(%{NUMBER:citrix_adc.log.query_string_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.query_string_length:int}\\)\\.$
                    // Grok pattern: ^Total%{SPACE}HTTP%{SPACE}header%{SPACE}length\\(%{NUMBER:citrix_adc.log.total_http_header_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.total_http_header_length:int}\\)\\.$
                    // Grok pattern: ^Profile%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.profile}$
                    // Grok pattern: ^Field%{SPACE}Type%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.field_type}$
                    // Grok pattern: ^Field%{SPACE}Name%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.field_name}$
                    // Grok pattern: ^Content%{SPACE}length%{SPACE}is%{SPACE}too%{SPACE}large\\(%{NUMBER:citrix_adc.log.content_length_bytes:long}%{SPACE}Bytes\\).%{SPACE}Memory%{SPACE}Allocation%{SPACE}failed.$
                    // Grok pattern: ^Signature%{SPACE}id%{SPACE}%{NUMBER:citrix_adc.log.signature_id:int}%{SPACE}contains%{SPACE}no%{SPACE}fast%{SPACE}match%{SPACE}pattern$
                    // Grok pattern: ^Appfw%{SPACE}maximum%{SPACE}session%{SPACE}Limit%{SPACE}reached%{SPACE}for%{SPACE}PEID%{SPACE}%{NUMBER:citrix_adc.log.peid:int}$
                    // Grok pattern: ^APPFW%{SPACE}RFC%{SPACE}Profile:%{SPACE}%{GREEDYDATA:citrix_adc.log.appfw_rfc_profile}$
                    // Grok pattern: ^New%{SPACE}signature%{SPACE}available%{SPACE}:%{SPACE}RuleID%{SPACE}=%{SPACE}%{NUMBER:citrix_adc.log.rule_id:int}$
                    // Grok pattern: ^Learned%{SPACE}rule%{SPACE}will%{SPACE}be%{SPACE}auto-deployed%{SPACE}after%{SPACE}%{NUMBER:citrix_adc.log.auto_deploy_mins:int}mins.%{SPACE}ViolType%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.violation_type}.%{SPACE}Profile%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.profile}$
                    // Grok pattern: ^Rest%{SPACE}Validation%{SPACE}relaxation%{SPACE}rule%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.rule}%{SPACE}hit%{SPACE}at%{SPACE}url%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.url}$
                    // Grok pattern: ^gRPC%{SPACE}Validation%{SPACE}relaxation%{SPACE}rule%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.rule}%{SPACE}hit%{SPACE}at%{SPACE}url%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.url}$
                    // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{IP:source.ip:ip} %{NUMBER:citrix_adc.log.transaction_id}-%{DATA:citrix_adc.log.ppe} - %{NOTSPACE:citrix_adc.log.profile}(?: %{NOTSPACE:url.original})?(?: %{GREEDYDATA:citrix_adc.log.message})?$"),
                            cached_grok!("^XML%{SPACE}Mismatched%{SPACE}content-type%{SPACE}in%{SPACE}HTTP%{SPACE}header%{SPACE}detected%{SPACE}=%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.content_type_mismatch}\\\"\\.$"),
                            cached_grok!("^Disallow%{SPACE}Deny%{SPACE}URL%{SPACE}for%{SPACE}rule%{SPACE}pattern%{SPACE}=%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.url}\\\"\\.$"),
                            cached_grok!("^Unknown%{SPACE}content-type%{SPACE}header%{SPACE}value%{SPACE}=%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.unknown_content_type}\\\"\\.$"),
                            cached_grok!("^parsing%{SPACE}referer%{SPACE}header%{SPACE}\\'%{GREEDYDATA:citrix_adc.log.referer_header}\\'%{SPACE}failed$"),
                            cached_grok!("^URL%{SPACE}length\\(%{NUMBER:citrix_adc.log.url_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.url_length:int}\\)\\.$"),
                            cached_grok!("^Cookie%{SPACE}header%{SPACE}length\\(%{NUMBER:citrix_adc.log.cookie_header_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.cookie_header_length:int}\\)\\.$"),
                            cached_grok!("^Header\\(Referer\\)%{SPACE}length\\(%{NUMBER:citrix_adc.log.header_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.header_length:int}\\)\\.$"),
                            cached_grok!("^Query%{SPACE}string%{SPACE}length\\(%{NUMBER:citrix_adc.log.query_string_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.query_string_length:int}\\)\\.$"),
                            cached_grok!("^Total%{SPACE}HTTP%{SPACE}header%{SPACE}length\\(%{NUMBER:citrix_adc.log.total_http_header_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.total_http_header_length:int}\\)\\.$"),
                            cached_grok!("^Profile%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.profile}$"),
                            cached_grok!("^Field%{SPACE}Type%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.field_type}$"),
                            cached_grok!("^Field%{SPACE}Name%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.field_name}$"),
                            cached_grok!("^Content%{SPACE}length%{SPACE}is%{SPACE}too%{SPACE}large\\(%{NUMBER:citrix_adc.log.content_length_bytes:long}%{SPACE}Bytes\\).%{SPACE}Memory%{SPACE}Allocation%{SPACE}failed.$"),
                            cached_grok!("^Signature%{SPACE}id%{SPACE}%{NUMBER:citrix_adc.log.signature_id:int}%{SPACE}contains%{SPACE}no%{SPACE}fast%{SPACE}match%{SPACE}pattern$"),
                            cached_grok!("^Appfw%{SPACE}maximum%{SPACE}session%{SPACE}Limit%{SPACE}reached%{SPACE}for%{SPACE}PEID%{SPACE}%{NUMBER:citrix_adc.log.peid:int}$"),
                            cached_grok!("^APPFW%{SPACE}RFC%{SPACE}Profile:%{SPACE}%{GREEDYDATA:citrix_adc.log.appfw_rfc_profile}$"),
                            cached_grok!("^New%{SPACE}signature%{SPACE}available%{SPACE}:%{SPACE}RuleID%{SPACE}=%{SPACE}%{NUMBER:citrix_adc.log.rule_id:int}$"),
                            cached_grok!("^Learned%{SPACE}rule%{SPACE}will%{SPACE}be%{SPACE}auto-deployed%{SPACE}after%{SPACE}%{NUMBER:citrix_adc.log.auto_deploy_mins:int}mins.%{SPACE}ViolType%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.violation_type}.%{SPACE}Profile%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.profile}$"),
                            cached_grok!("^Rest%{SPACE}Validation%{SPACE}relaxation%{SPACE}rule%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.rule}%{SPACE}hit%{SPACE}at%{SPACE}url%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.url}$"),
                            cached_grok!("^gRPC%{SPACE}Validation%{SPACE}relaxation%{SPACE}rule%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.rule}%{SPACE}hit%{SPACE}at%{SPACE}url%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.url}$"),
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("citrix_adc.log.referer_header").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.referrer", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.rule_id") {
                if let Some(val) = event.get("citrix_adc.log.rule_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.rule_id".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.rule_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_rule_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.rule_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.id", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.original", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.peid") {
                if let Some(val) = event.get("citrix_adc.log.peid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.peid".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.peid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_peid_to_string")?;
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
            if event.has_value("citrix_adc.log.signature_id") {
                if let Some(val) = event.get("citrix_adc.log.signature_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.signature_id".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.signature_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_signature_id_to_string")?;
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
            if event.has_value("citrix_adc.log.url_length") {
                if let Some(val) = event.get("citrix_adc.log.url_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.url_length".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.url_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_url_length_to_long")?;
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
            if event.has_value("citrix_adc.log.max_allowed.url_length") {
                if let Some(val) = event.get("citrix_adc.log.max_allowed.url_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.max_allowed.url_length".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.max_allowed.url_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_max_allowed_url_length_to_long")?;
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
            if event.has_value("citrix_adc.log.cookie_header_length") {
                if let Some(val) = event.get("citrix_adc.log.cookie_header_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.cookie_header_length".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.cookie_header_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cookie_header_length_to_long")?;
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
            if event.has_value("citrix_adc.log.max_allowed.cookie_header_length") {
                if let Some(val) = event.get("citrix_adc.log.max_allowed.cookie_header_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.max_allowed.cookie_header_length".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.max_allowed.cookie_header_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_max_allowed_cookie_header_length_to_long")?;
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
            if event.has_value("citrix_adc.log.header_length") {
                if let Some(val) = event.get("citrix_adc.log.header_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.header_length".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.header_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_header_length_to_long")?;
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
            if event.has_value("citrix_adc.log.max_allowed.header_length") {
                if let Some(val) = event.get("citrix_adc.log.max_allowed.header_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.max_allowed.header_length".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.max_allowed.header_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_max_allowed_header_length_to_long")?;
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
            if event.has_value("citrix_adc.log.query_string_length") {
                if let Some(val) = event.get("citrix_adc.log.query_string_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.query_string_length".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.query_string_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_query_string_length_to_long")?;
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
            if event.has_value("citrix_adc.log.max_allowed.query_string_length") {
                if let Some(val) = event.get("citrix_adc.log.max_allowed.query_string_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.max_allowed.query_string_length".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.max_allowed.query_string_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_max_allowed_query_string_length_to_long")?;
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
            if event.has_value("citrix_adc.log.total_http_header_length") {
                if let Some(val) = event.get("citrix_adc.log.total_http_header_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_http_header_length".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_http_header_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_http_header_length_to_long")?;
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
            if event.has_value("citrix_adc.log.max_allowed.total_http_header_length") {
                if let Some(val) = event.get("citrix_adc.log.max_allowed.total_http_header_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.max_allowed.total_http_header_length".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.max_allowed.total_http_header_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_max_allowed_total_http_header_length_to_long")?;
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
            if event.has_value("citrix_adc.log.content_length_bytes") {
                if let Some(val) = event.get("citrix_adc.log.content_length_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.content_length_bytes".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.content_length_bytes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_content_length_bytes_to_long")?;
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
