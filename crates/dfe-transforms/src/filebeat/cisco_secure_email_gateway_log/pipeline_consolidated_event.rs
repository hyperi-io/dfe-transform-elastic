// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_consolidated_event` pipeline.
pub struct PipelineConsolidatedEvent;

impl Transform for PipelineConsolidatedEvent {
    fn name(&self) -> &str {
        "pipeline_consolidated_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^(?:%{DATA:_tmp.timestamp} )?CEF:%{NUMBER:cisco_secure_email_gateway.log.cef_format_version}\\|%{WORD:cisco_secure_email_gateway.log.appliance.vendor}\\|%{DATA:cisco_secure_email_gateway.log.appliance.product}\\|%{DATA:cisco_secure_email_gateway.log.appliance.version}\\|%{DATA:cisco_secure_email_gateway.log.event_class_id}\\|%{DATA:cisco_secure_email_gateway.log.event.name}\\|%{WORD:event.severity}\\|%{GREEDYDATA:_tmp.details}$
                    let _ = cached_grok!("^(?:%{DATA:_tmp.timestamp} )?CEF:%{NUMBER:cisco_secure_email_gateway.log.cef_format_version}\\|%{WORD:cisco_secure_email_gateway.log.appliance.vendor}\\|%{DATA:cisco_secure_email_gateway.log.appliance.product}\\|%{DATA:cisco_secure_email_gateway.log.appliance.version}\\|%{DATA:cisco_secure_email_gateway.log.event_class_id}\\|%{DATA:cisco_secure_email_gateway.log.event.name}\\|%{WORD:event.severity}\\|%{GREEDYDATA:_tmp.details}$").extract_into(&input, event)?;
                }

            let _cond = { event.has_value("_tmp.details") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.details") {
                if let Some(kv_str) = event.get_string("_tmp.details") {
                    for pair in cached_regex!("(?:((\\s+)?$|\\s+(?=\\w+=)))").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_tmp.details".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("_tmp.fields.{}", key), value)?;
                            }
                        }
                    }
                }
            }
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAAttachmentDetails") {
                    event.rename("_tmp.fields.ESAAttachmentDetails", "cisco_secure_email_gateway.log.esa.attachment_details")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.fields.ESADHASource") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.fields.ESADHASource") {
                if let Some(val) = event.get("_tmp.fields.ESADHASource") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_tmp.fields.ESADHASource".into(),
                            message,
                        })?;
                    event.set("cisco_secure_email_gateway.log.esa.dha_source", converted)?;
                }
            }
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESADKIMVerdict") {
                    event.rename("_tmp.fields.ESADKIMVerdict", "cisco_secure_email_gateway.log.esa.dkim_verdict")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESADLPVerdict") {
                    event.rename("_tmp.fields.ESADLPVerdict", "cisco_secure_email_gateway.log.esa.dlp_verdict")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESADMARCVerdict") {
                    event.rename("_tmp.fields.ESADMARCVerdict", "cisco_secure_email_gateway.log.esa.dmarc_verdict")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESADaneHost") {
                    event.rename("_tmp.fields.ESADaneHost", "cisco_secure_email_gateway.log.esa.dane.host")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESADaneStatus") {
                    event.rename("_tmp.fields.ESADaneStatus", "cisco_secure_email_gateway.log.esa.dane.status")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAFinalActionDetails") {
                    event.rename("_tmp.fields.ESAFinalActionDetails", "cisco_secure_email_gateway.log.esa.final_action_details")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAFriendlyFrom") {
                    event.rename("_tmp.fields.ESAFriendlyFrom", "cisco_secure_email_gateway.log.esa.friendly_from")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAGMVerdict") {
                    event.rename("_tmp.fields.ESAGMVerdict", "cisco_secure_email_gateway.log.esa.graymail_verdict")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.fields.ESAHeloDomain") {
                if let Some(s) = event.get_string("_tmp.fields.ESAHeloDomain") {
                    match url_decode(&s) {
                        Some(decoded) => event.set("cisco_secure_email_gateway.log.esa.helo.domain", json!(decoded))?,
                        None => return Err(TransformError::ParseError {
                            path: "_tmp.fields.ESAHeloDomain".into(),
                            message: format!("cannot url-decode '{s}'"),
                        }),
                    }
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.fields.ESAHeloIP") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.fields.ESAHeloIP") {
                if let Some(val) = event.get("_tmp.fields.ESAHeloIP") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_tmp.fields.ESAHeloIP".into(),
                            message,
                        })?;
                    event.set("cisco_secure_email_gateway.log.esa.helo.ip", converted)?;
                }
            }
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAMARAction") {
                    event.rename("_tmp.fields.ESAMARAction", "cisco_secure_email_gateway.log.esa.mail_auto_remediation_action")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAMFVerdict") {
                    event.rename("_tmp.fields.ESAMFVerdict", "cisco_secure_email_gateway.log.esa.mf_verdict")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAMailFlowPolicy") {
                    event.rename("_tmp.fields.ESAMailFlowPolicy", "cisco_secure_email_gateway.log.esa.mail_flow_policy")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.fields.ESAMsgSize") {
                if let Some(val) = event.get("_tmp.fields.ESAMsgSize") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_tmp.fields.ESAMsgSize".into(),
                            message,
                        })?;
                    event.set("cisco_secure_email_gateway.log.esa.msg_size", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAMsgTooBigFromSender") {
                    event.rename("_tmp.fields.ESAMsgTooBigFromSender", "cisco_secure_email_gateway.log.esa.msg_too_big_from_sender")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAOFVerdict") {
                    event.rename("_tmp.fields.ESAOFVerdict", "cisco_secure_email_gateway.log.esa.outbreak_filter_verdict")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESARateLimitedIP") {
                    event.rename("_tmp.fields.ESARateLimitedIP", "cisco_secure_email_gateway.log.esa.rate_limited_ip")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAReplyTo") {
                    event.rename("_tmp.fields.ESAReplyTo", "cisco_secure_email_gateway.log.esa.reply_to")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESASDRDomainAge") {
                    event.rename("_tmp.fields.ESASDRDomainAge", "cisco_secure_email_gateway.log.esa.sdr_consolidated_domain_age")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESASPFVerdict") {
                    event.rename("_tmp.fields.ESASPFVerdict", "cisco_secure_email_gateway.log.esa.spf_verdict")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESASenderGroup") {
                    event.rename("_tmp.fields.ESASenderGroup", "cisco_secure_email_gateway.log.esa.sender_group")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESATLSInCipher") {
                    event.rename("_tmp.fields.ESATLSInCipher", "cisco_secure_email_gateway.log.esa.tls.in.cipher")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESATLSInConnStatus") {
                    event.rename("_tmp.fields.ESATLSInConnStatus", "cisco_secure_email_gateway.log.esa.tls.in.connection_status")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESATLSInProtocol") {
                    event.rename("_tmp.fields.ESATLSInProtocol", "cisco_secure_email_gateway.log.esa.tls.in.protocol")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESATLSOutCipher") {
                    event.rename("_tmp.fields.ESATLSOutCipher", "cisco_secure_email_gateway.log.esa.tls.out.cipher")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESATLSOutConnStatus") {
                    event.rename("_tmp.fields.ESATLSOutConnStatus", "cisco_secure_email_gateway.log.esa.tls.out.connection_status")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESATLSOutProtocol") {
                    event.rename("_tmp.fields.ESATLSOutProtocol", "cisco_secure_email_gateway.log.esa.tls.out.protocol")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAURLDetails") {
                    event.rename("_tmp.fields.ESAURLDetails", "cisco_secure_email_gateway.log.esa.url_details")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.act") {
                    event.rename("_tmp.fields.act", "cisco_secure_email_gateway.log.act")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cfp1Label") {
                    event.rename("_tmp.fields.cfp1Label", "cisco_secure_email_gateway.log.cfp1_label")?;
                }
                Ok(())
            })();

            let _cond = { !(["None", "not enabled", "rfc1918"].contains(&event.get_str("_tmp.fields.cfp1").unwrap_or(""))) };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("_tmp.fields.cfp1") {
                if let Some(val) = event.get("_tmp.fields.cfp1") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "_tmp.fields.cfp1".into(),
                            message,
                        })?;
                    event.set("cisco_secure_email_gateway.log.cfp1", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert__tmp_fields_cfp1_to_cisco_secure_email_gateway_log_cfp1_736a7f37")?;
                        if event.remove("_tmp.fields.cfp1").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_tmp.fields.cfp1".into() });
                        }
                        event.append("error.message", json!(format!("Failed to convert cfp1 field: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cs1Label") {
                    event.rename("_tmp.fields.cs1Label", "cisco_secure_email_gateway.log.cs1_label")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cs1") {
                    event.rename("_tmp.fields.cs1", "cisco_secure_email_gateway.log.cs1")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cs2Label") {
                    event.rename("_tmp.fields.cs2Label", "cisco_secure_email_gateway.log.cs2_label")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cs2") {
                    event.rename("_tmp.fields.cs2", "cisco_secure_email_gateway.log.cs2")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cs3Label") {
                    event.rename("_tmp.fields.cs3Label", "cisco_secure_email_gateway.log.cs3_label")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cs3") {
                    event.rename("_tmp.fields.cs3", "cisco_secure_email_gateway.log.cs3")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cs4Label") {
                    event.rename("_tmp.fields.cs4Label", "cisco_secure_email_gateway.log.cs4_label")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.fields.cs4") && event.get_str("_tmp.fields.cs4") != Some("''") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.fields.cs4") {
                gsub_field(event, "_tmp.fields.cs4", "cisco_secure_email_gateway.log.cs4", cached_regex!("^'<|>'$"), "")?;
            }
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cs5Label") {
                    event.rename("_tmp.fields.cs5Label", "cisco_secure_email_gateway.log.cs5_label")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cs5") {
                    event.rename("_tmp.fields.cs5", "cisco_secure_email_gateway.log.cs5")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cs6Label") {
                    event.rename("_tmp.fields.cs6Label", "cisco_secure_email_gateway.log.cs6_label")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.cs6") {
                    event.rename("_tmp.fields.cs6", "cisco_secure_email_gateway.log.cs6")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.deviceDirection") {
                    event.rename("_tmp.fields.deviceDirection", "cisco_secure_email_gateway.log.device_direction")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.deviceInboundInterface") {
                    event.rename("_tmp.fields.deviceInboundInterface", "cisco_secure_email_gateway.log.listener.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.deviceOutboundInterface") {
                    event.rename("_tmp.fields.deviceOutboundInterface", "cisco_secure_email_gateway.log.listener.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.duser") {
                    event.rename("_tmp.fields.duser", "email.to.address")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.fields.dvc") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.fields.dvc") {
                if let Some(val) = event.get("_tmp.fields.dvc") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_tmp.fields.dvc".into(),
                            message,
                        })?;
                    event.set("cisco_secure_email_gateway.log.data.ip", converted)?;
                }
            }
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.endTime") {
                    event.rename("_tmp.fields.endTime", "event.end")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.end") {
                    event.rename("_tmp.fields.end", "event.end")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.fields.sourceHostName") {
                if let Some(s) = event.get_string("_tmp.fields.sourceHostName") {
                    match url_decode(&s) {
                        Some(decoded) => event.set("source.domain", json!(decoded))?,
                        None => return Err(TransformError::ParseError {
                            path: "_tmp.fields.sourceHostName".into(),
                            message: format!("cannot url-decode '{s}'"),
                        }),
                    }
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.fields.shost") {
                if let Some(s) = event.get_string("_tmp.fields.shost") {
                    match url_decode(&s) {
                        Some(decoded) => event.set("source.domain", json!(decoded))?,
                        None => return Err(TransformError::ParseError {
                            path: "_tmp.fields.shost".into(),
                            message: format!("cannot url-decode '{s}'"),
                        }),
                    }
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.fields.sourceAddress") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.fields.sourceAddress") {
                if let Some(val) = event.get("_tmp.fields.sourceAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_tmp.fields.sourceAddress".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("_tmp.fields.src") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.fields.src") {
                if let Some(val) = event.get("_tmp.fields.src") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_tmp.fields.src".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("_tmp.fields.msg") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_tmp.fields.msg") {
                gsub_field(event, "_tmp.fields.msg", "email.subject", cached_regex!("(?:^['\"]|['\"]$)"), "")?;
            }
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.suser") {
                    event.rename("_tmp.fields.suser", "email.from.address")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.startTime") {
                    event.rename("_tmp.fields.startTime", "event.start")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.start") {
                    event.rename("_tmp.fields.start", "event.start")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("cisco_secure_email_gateway.log.esa.helo.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("cisco_secure_email_gateway.log.esa.helo.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_secure_email_gateway.log.data.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("cisco_secure_email_gateway.log.data.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.listener.name") == Some("Incomingmail") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("email.direction", json!("inbound"))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.listener.name") == Some("Outcomingmail") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("email.direction", json!("outbound"))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.device_direction") == Some("0") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("cisco_secure_email_gateway.log.device_direction", json!("incoming"))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.device_direction") == Some("1") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("cisco_secure_email_gateway.log.device_direction", json!("outgoing"))?;
                Ok(())
            })();
            }

                if event.has_value("_tmp.fields.deviceExternalId") {
                    event.rename("_tmp.fields.deviceExternalId", "host.id")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.fields.ESAMID") {
                    event.rename("_tmp.fields.ESAMID", "email.message_id")?;
                }
                Ok(())
            })();

                if event.has_value("_tmp.fields.ESAICID") {
                    event.rename("_tmp.fields.ESAICID", "cisco_secure_email_gateway.log.esa.injection_connection_id")?;
                }

                if event.has_value("_tmp.fields.ESADCID") {
                    event.rename("_tmp.fields.ESADCID", "cisco_secure_email_gateway.log.esa.delivery_connection_id")?;
                }

                if event.has_value("_tmp.fields.ESAAMPVerdict") {
                    event.rename("_tmp.fields.ESAAMPVerdict", "cisco_secure_email_gateway.log.esa.amp_verdict")?;
                }

                if event.has_value("_tmp.fields.ESAASVerdict") {
                    event.rename("_tmp.fields.ESAASVerdict", "cisco_secure_email_gateway.log.esa.as_verdict")?;
                }

                if event.has_value("_tmp.fields.ESAAVVerdict") {
                    event.rename("_tmp.fields.ESAAVVerdict", "cisco_secure_email_gateway.log.esa.av_verdict")?;
                }

                if event.has_value("_tmp.fields.ESACFVerdict") {
                    event.rename("_tmp.fields.ESACFVerdict", "cisco_secure_email_gateway.log.esa.content_filter_verdict")?;
                }

            let _cond = { event.has_value("_tmp.timestamp") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["E MMM dd HH:mm:ss yyyy", "E MMM  d HH:mm:ss yyyy", "E MMM d HH:mm:ss yyyy", "MMM  d HH:mm:ss", "MMM dd HH:mm:ss", "MMM d HH:mm:ss"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date__tmp_timestamp_d7741cfc")?;
                        event.remove("event.timezone");
                    let _cond = { event.has_value("_tmp.timestamp") };
                    if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                            match parse_date_out(&date_str, &["E MMM dd HH:mm:ss yyyy", "E MMM  d HH:mm:ss yyyy", "E MMM d HH:mm:ss yyyy", "MMM  d HH:mm:ss", "MMM dd HH:mm:ss", "MMM d HH:mm:ss"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "_tmp.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date__tmp_timestamp_5d9cfc58")?;
                                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("event.start") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("event.start") {
                    match parse_date_out(&date_str, &["E MMM dd HH:mm:ss yyyy", "E MMM  d HH:mm:ss yyyy", "E MMM d HH:mm:ss yyyy", "MMM  d HH:mm:ss", "MMM dd HH:mm:ss", "MMM d HH:mm:ss"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "event.start".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date event.start")?;
                        event.remove("event.timezone");
                    let _cond = { event.has_value("event.start") };
                    if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("event.start") {
                            match parse_date_out(&date_str, &["E MMM dd HH:mm:ss yyyy", "E MMM  d HH:mm:ss yyyy", "E MMM d HH:mm:ss yyyy", "MMM  d HH:mm:ss", "MMM dd HH:mm:ss", "MMM d HH:mm:ss"], None, None) {
                                Some(parsed) => event.set("event.start", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "event.start".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date re-parse event.start")?;
                                event.remove("event.start");
                                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("event.end") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("event.end") {
                    match parse_date_out(&date_str, &["E MMM dd HH:mm:ss yyyy", "E MMM  d HH:mm:ss yyyy", "E MMM d HH:mm:ss yyyy", "MMM  d HH:mm:ss", "MMM dd HH:mm:ss", "MMM d HH:mm:ss"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "event.end".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date event.end")?;
                        event.remove("event.timezone");
                    let _cond = { event.has_value("event.end") };
                    if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("event.end") {
                            match parse_date_out(&date_str, &["E MMM dd HH:mm:ss yyyy", "E MMM  d HH:mm:ss yyyy", "E MMM d HH:mm:ss yyyy", "MMM  d HH:mm:ss", "MMM dd HH:mm:ss", "MMM d HH:mm:ss"], None, None) {
                                Some(parsed) => event.set("event.end", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "event.end".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date re-parse event.end")?;
                                event.remove("event.end");
                                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_tmp");
                Ok(())
            })();

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
