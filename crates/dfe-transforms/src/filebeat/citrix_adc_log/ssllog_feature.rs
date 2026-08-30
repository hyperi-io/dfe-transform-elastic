// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ssllog_feature` pipeline.
pub struct SsllogFeature;

impl Transform for SsllogFeature {
    fn name(&self) -> &str {
        "ssllog_feature"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^Backend%{SPACE}SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - Server%{SPACE}IP %{IP:citrix_adc.log.server.ip} - Server%{SPACE}Port %{NUMBER:citrix_adc.log.server.port:int} - Protocol%{SPACE}Version %{DATA:citrix_adc.log.protocol_version} - Cipher%{SPACE}Suite \\\"%{DATA:citrix_adc.log.cipher_suite}\\\" - Session %{DATA:citrix_adc.log.session}(%{SPACE}- %{WORD:citrix_adc.log.server_authentication} -%{SPACE}SerialNumber \\\"%{DATA:citrix_adc.log.serial_number}\\\" - SignatureAlgorithm \\\"%{DATA:citrix_adc.log.signature_algorithm}\\\" - ValidFrom \\\"%{DATA:citrix_adc.log.valid_from}\\\" - ValidTo \\\"%{DATA:citrix_adc.log.valid_to}\\\" - HandshakeTime %{INT:citrix_adc.log.handshake_time} ms)?$
                    // Grok pattern: ^Certificate%{SPACE}Key%{SPACE}Pair %{DATA:citrix_adc.log.certificate_key_pair} - Days%{SPACE}To%{SPACE}Expire %{NUMBER:citrix_adc.log.days_to_expire:int}$
                    // Grok pattern: ^SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - Issuer%{SPACE}Name \\\"%{GREEDYDATA:citrix_adc.log.issuer_name}\\\"$
                    // Grok pattern: ^SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - Subject%{SPACE}Name \\\"%{GREEDYDATA:citrix_adc.log.subject_name}\\\"$
                    // Grok pattern: ^crl_name %{DATA:citrix_adc.log.crl_name} - server_ip %{IP:citrix_adc.log.server.ip} - server_port %{NUMBER:citrix_adc.log.server.port:int} - method %{WORD:citrix_adc.log.method} - ldapscope %{WORD:citrix_adc.log.ldap_scope}$
                    // Grok pattern: ^Domainname %{DATA:citrix_adc.log.domain_name} Ipaddress %{IP:citrix_adc.log.ip_address}$
                    // Grok pattern: ^SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - ClientIP %{IP:citrix_adc.log.client_ip} - ClientPort %{NUMBER:citrix_adc.log.client_port} - VserverServiceIP %{IP:citrix_adc.log.vserver.ip} - VserverServicePort %{NUMBER:citrix_adc.log.vserver.port} - ClientVersion %{DATA:citrix_adc.log.client_version} - CipherSuite \\\"%{GREEDYDATA:citrix_adc.log.cipher_suite}\\\"( - )?Session %{WORD:citrix_adc.log.session}(%{SPACE}- HandshakeTime %{INT:citrix_adc.log.handshake_time} ms)?( - Reason \\\"%{GREEDYDATA:citrix_adc.log.reason}\\\")?$
                    // Grok pattern: ^%{GREEDYDATA:citrix_adc.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^Backend%{SPACE}SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - Server%{SPACE}IP %{IP:citrix_adc.log.server.ip} - Server%{SPACE}Port %{NUMBER:citrix_adc.log.server.port:int} - Protocol%{SPACE}Version %{DATA:citrix_adc.log.protocol_version} - Cipher%{SPACE}Suite \\\"%{DATA:citrix_adc.log.cipher_suite}\\\" - Session %{DATA:citrix_adc.log.session}(%{SPACE}- %{WORD:citrix_adc.log.server_authentication} -%{SPACE}SerialNumber \\\"%{DATA:citrix_adc.log.serial_number}\\\" - SignatureAlgorithm \\\"%{DATA:citrix_adc.log.signature_algorithm}\\\" - ValidFrom \\\"%{DATA:citrix_adc.log.valid_from}\\\" - ValidTo \\\"%{DATA:citrix_adc.log.valid_to}\\\" - HandshakeTime %{INT:citrix_adc.log.handshake_time} ms)?$"),
                            cached_grok!("^Certificate%{SPACE}Key%{SPACE}Pair %{DATA:citrix_adc.log.certificate_key_pair} - Days%{SPACE}To%{SPACE}Expire %{NUMBER:citrix_adc.log.days_to_expire:int}$"),
                            cached_grok!("^SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - Issuer%{SPACE}Name \\\"%{GREEDYDATA:citrix_adc.log.issuer_name}\\\"$"),
                            cached_grok!("^SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - Subject%{SPACE}Name \\\"%{GREEDYDATA:citrix_adc.log.subject_name}\\\"$"),
                            cached_grok!("^crl_name %{DATA:citrix_adc.log.crl_name} - server_ip %{IP:citrix_adc.log.server.ip} - server_port %{NUMBER:citrix_adc.log.server.port:int} - method %{WORD:citrix_adc.log.method} - ldapscope %{WORD:citrix_adc.log.ldap_scope}$"),
                            cached_grok!("^Domainname %{DATA:citrix_adc.log.domain_name} Ipaddress %{IP:citrix_adc.log.ip_address}$"),
                            cached_grok!("^SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - ClientIP %{IP:citrix_adc.log.client_ip} - ClientPort %{NUMBER:citrix_adc.log.client_port} - VserverServiceIP %{IP:citrix_adc.log.vserver.ip} - VserverServicePort %{NUMBER:citrix_adc.log.vserver.port} - ClientVersion %{DATA:citrix_adc.log.client_version} - CipherSuite \\\"%{GREEDYDATA:citrix_adc.log.cipher_suite}\\\"( - )?Session %{WORD:citrix_adc.log.session}(%{SPACE}- HandshakeTime %{INT:citrix_adc.log.handshake_time} ms)?( - Reason \\\"%{GREEDYDATA:citrix_adc.log.reason}\\\")?$"),
                            cached_grok!("^%{GREEDYDATA:citrix_adc.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("citrix_adc.log.valid_from") && event.get_str("citrix_adc.log.valid_from") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("citrix_adc.log.valid_from") {
                    match parse_date_out(&date_str, &["MMM dd HH:mm:ss yyyy z", "MMM  d HH:mm:ss yyyy z"], None, None) {
                        Some(parsed) => event.set("citrix_adc.log.valid_from", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "citrix_adc.log.valid_from".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_valid_from")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("citrix_adc.log.valid_to") && event.get_str("citrix_adc.log.valid_to") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("citrix_adc.log.valid_to") {
                    match parse_date_out(&date_str, &["MMM dd HH:mm:ss yyyy z", "MMM  d HH:mm:ss yyyy z"], None, None) {
                        Some(parsed) => event.set("citrix_adc.log.valid_to", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "citrix_adc.log.valid_to".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_valid_to")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.method").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.method", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.server.ip") && event.get_str("citrix_adc.log.server.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.server.ip") {
                if let Some(val) = event.get("citrix_adc.log.server.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.server.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.server.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_server_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.server.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.server.port") {
                if let Some(val) = event.get("citrix_adc.log.server.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.server.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.server.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_server_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.server.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.domain_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.spcb_id") {
                if let Some(val) = event.get("citrix_adc.log.spcb_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.spcb_id".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.spcb_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_spcb_id_to_string")?;
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
            if event.has_value("citrix_adc.log.days_to_expire") {
                if let Some(val) = event.get("citrix_adc.log.days_to_expire") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.days_to_expire".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.days_to_expire", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_days_to_expire_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("citrix_adc.log.ip_address") && event.get_str("citrix_adc.log.ip_address") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.ip_address") {
                if let Some(val) = event.get("citrix_adc.log.ip_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.ip_address".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_ip_address_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("citrix_adc.log.client_ip") && event.get_str("citrix_adc.log.client_ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.client_ip") {
                if let Some(val) = event.get("citrix_adc.log.client_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.client_ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.client_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_client_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.client_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.client_port") {
                if let Some(val) = event.get("citrix_adc.log.client_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.client_port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.client_port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_client_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.client_port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client.port", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.vserver.ip") && event.get_str("citrix_adc.log.vserver.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.vserver.ip") {
                if let Some(val) = event.get("citrix_adc.log.vserver.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.vserver.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.vserver.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_vserver_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.vserver.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.vserver.port") {
                if let Some(val) = event.get("citrix_adc.log.vserver.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.vserver.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.vserver.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_vserver_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.vserver.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.port", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.handshake_time") {
                if let Some(val) = event.get("citrix_adc.log.handshake_time") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.handshake_time".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.handshake_time", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_handshake_time_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.cipher_suite").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.cipher", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.issuer_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.server.issuer", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.subject_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.server.subject", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.protocol_version") && event.get_str("citrix_adc.log.protocol_version") != Some("") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("citrix_adc.log.protocol_version") {
                    // Grok pattern: ^%{DATA:tls.version_protocol}v%{DATA:tls.version}$
                    let _ = cached_grok!("^%{DATA:tls.version_protocol}v%{DATA:tls.version}$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("citrix_adc.log.client_version") && event.get_str("citrix_adc.log.client_version") != Some("") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("citrix_adc.log.client_version") {
                    // Grok pattern: ^%{DATA:tls.version_protocol}v%{DATA:tls.version}$
                    let _ = cached_grok!("^%{DATA:tls.version_protocol}v%{DATA:tls.version}$").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            if let Some(v) = event.get("citrix_adc.log.reason").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reason", v)?;
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
