// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_extract_message` pipeline.
pub struct PipelineExtractMessage;

impl Transform for PipelineExtractMessage {
    fn name(&self) -> &str {
        "pipeline_extract_message"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = {
                event.get_str("event.code").is_some_and(|s| {
                    [
                        "IF_DOWN_ADMIN_DOWN",
                        "IF_ADMIN_UP",
                        "SPEED",
                        "IF_DUPLEX",
                        "IF_RX_FLOW_CONTROL",
                        "IF_TX_FLOW_CONTROL",
                        "IF_UP",
                        "IF_XCVR_WARNING",
                    ]
                    .contains(&s.to_uppercase().as_str())
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is up in mode %{DATA:cisco_nexus.log.interface.mode}$
                        // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is %{GREEDYDATA}$
                        // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational speed changed to %{DATA:cisco_nexus.log.operational.speed}$
                        // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational duplex mode changed to %{DATA:cisco_nexus.log.operational.duplex_mode}$
                        // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Receive Flow Control state changed to %{DATA:cisco_nexus.log.operational.receive_flow_control_state}$
                        // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Transmit Flow Control state changed to %{DATA:cisco_nexus.log.operational.transmit_flow_control_state}$
                        // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, %{GREEDYDATA}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is up in mode %{DATA:cisco_nexus.log.interface.mode}$"
                                ),
                                cached_grok!(
                                    "^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is %{GREEDYDATA}$"
                                ),
                                cached_grok!(
                                    "^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational speed changed to %{DATA:cisco_nexus.log.operational.speed}$"
                                ),
                                cached_grok!(
                                    "^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational duplex mode changed to %{DATA:cisco_nexus.log.operational.duplex_mode}$"
                                ),
                                cached_grok!(
                                    "^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Receive Flow Control state changed to %{DATA:cisco_nexus.log.operational.receive_flow_control_state}$"
                                ),
                                cached_grok!(
                                    "^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Transmit Flow Control state changed to %{DATA:cisco_nexus.log.operational.transmit_flow_control_state}$"
                                ),
                                cached_grok!(
                                    "^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, %{GREEDYDATA}$"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("event.code").is_some_and(|s| {
                    [
                        "VSHD_SYSLOG_CONFIG_I",
                        "DETECT_MULTIPLE_PEERS",
                        "UPDOWN",
                        "CFGWRITE_STARTED",
                        "LINEPROTO",
                    ]
                    .contains(&s.to_uppercase().as_str())
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Configured from vty by %{USERNAME:user.name} on %{IP:source.ip}@%{DATA:cisco_nexus.log.terminal}$
                        // Grok pattern: ^Multiple peers detected on %{DATA:cisco_nexus.log.interface.name}$
                        // Grok pattern: ^Line (?i)protocol on Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.line_protocol_state}$
                        // Grok pattern: ^Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.state}$
                        // Grok pattern: ^%{DATA}(PID %{NUMBER:process.pid:long})%{GREEDYDATA}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^Configured from vty by %{USERNAME:user.name} on %{IP:source.ip}@%{DATA:cisco_nexus.log.terminal}$"
                                ),
                                cached_grok!(
                                    "^Multiple peers detected on %{DATA:cisco_nexus.log.interface.name}$"
                                ),
                                cached_grok!(
                                    "^Line (?i)protocol on Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.line_protocol_state}$"
                                ),
                                cached_grok!(
                                    "^Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.state}$"
                                ),
                                cached_grok!(
                                    "^%{DATA}(PID %{NUMBER:process.pid:long})%{GREEDYDATA}$"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get_str("event.code")
                    .is_some_and(|s| ["SYSTEM_MSG"].contains(&s.to_uppercase().as_str()))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{DATA}authentication failure; %{GREEDYDATA:temp.message} - %{GREEDYDATA}$
                        // Grok pattern: ^%{DATA}Authentication failure for %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
                        // Grok pattern: ^%{DATA}Authentication failed for user %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
                        // Grok pattern: ^Login failed for user %{USERNAME:user.name} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
                        // Grok pattern: ^%{DATA} : %{GREEDYDATA:temp.message2}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{DATA}authentication failure; %{GREEDYDATA:temp.message} - %{GREEDYDATA}$"
                                ),
                                cached_grok!(
                                    "^%{DATA}Authentication failure for %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$"
                                ),
                                cached_grok!(
                                    "^%{DATA}Authentication failed for user %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$"
                                ),
                                cached_grok!(
                                    "^Login failed for user %{USERNAME:user.name} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$"
                                ),
                                cached_grok!("^%{DATA} : %{GREEDYDATA:temp.message2}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("event.code").is_some_and(|s| {
                    [
                        "INVAL_IP",
                        "L2FM_MAC_MOVE2",
                        "DUPLEX_MISMATCH",
                        "NATIVE_VLAN_MISMATCH",
                        "THRESHOLD_VIOLATION",
                    ]
                    .contains(&s.to_uppercase().as_str())
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{DATA:network.protocol} %{DATA}%{SPACE}Received packet with invalid destination IP address (%{DATA}) from %{CISCOMAC:source.mac} on %{DATA:cisco_nexus.log.interface.name}$
                        // Grok pattern: ^Mac %{CISCOMAC:source.mac} in %{DATA:cisco_nexus.log.interface.name} has moved from %{GREEDYDATA}$
                        // Grok pattern: ^%{DATA} mismatch discovered on %{DATA:cisco_nexus.log.network.ingress_interface}(?:\\(%{DATA}\\))?, with %{DATA:cisco_nexus.log.network.egress_interface}(?:\\(%{DATA}\\))?$
                        // Grok pattern: ^%{DATA:cisco_nexus.log.interface.name}: Rx power high warning; Operating value: %{DATA:cisco_nexus.log.operating_value}, Threshold value: %{DATA:cisco_nexus.log.threshold_value}.$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{DATA:network.protocol} %{DATA}%{SPACE}Received packet with invalid destination IP address (%{DATA}) from %{CISCOMAC:source.mac} on %{DATA:cisco_nexus.log.interface.name}$"
                                ),
                                cached_grok!(
                                    "^Mac %{CISCOMAC:source.mac} in %{DATA:cisco_nexus.log.interface.name} has moved from %{GREEDYDATA}$"
                                ),
                                cached_grok!(
                                    "^%{DATA} mismatch discovered on %{DATA:cisco_nexus.log.network.ingress_interface}(?:\\(%{DATA}\\))?, with %{DATA:cisco_nexus.log.network.egress_interface}(?:\\(%{DATA}\\))?$"
                                ),
                                cached_grok!(
                                    "^%{DATA:cisco_nexus.log.interface.name}: Rx power high warning; Operating value: %{DATA:cisco_nexus.log.operating_value}, Threshold value: %{DATA:cisco_nexus.log.threshold_value}.$"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("event.code").is_some_and(|s| {
                    ["LOGIN_SUCCESS", "LOGOUT", "LOGOUT_C6K"].contains(&s.to_uppercase().as_str())
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Login Success \\[user: %{USERNAME:user.name}\\] \\[Source: %{IP:source.ip}\\] \\[localport: %{NUMBER:source.port:long}\\] at %{GREEDYDATA}$
                        // Grok pattern: ^User %{USERNAME:user.name} %{GREEDYDATA}\\(%{IP:source.ip}\\)$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^Login Success \\[user: %{USERNAME:user.name}\\] \\[Source: %{IP:source.ip}\\] \\[localport: %{NUMBER:source.port:long}\\] at %{GREEDYDATA}$"
                                ),
                                cached_grok!(
                                    "^User %{USERNAME:user.name} %{GREEDYDATA}\\(%{IP:source.ip}\\)$"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.mac") {
                    gsub_field(event, "source.mac", "source.mac", cached_regex!("[.]"), "")?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "gsub_sourcemac_remove_dot",
                )?;
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.mac") {
                    gsub_field(
                        event,
                        "source.mac",
                        "source.mac",
                        cached_regex!("(..)(?!$)"),
                        "$1-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "gsub_sourcemac_add_hyphen",
                )?;
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("source.mac") {
                map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("temp.message") {
                    if let Some(kv_str) = event.get_string("temp.message") {
                        let mut kv_gap = false;
                        for pair in cached_regex!("\\s+").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "temp.message".into(),
                                    split: "=".into(),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(event, &format!("temp.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("temp.message2") {
                    if let Some(kv_str) = event.get_string("temp.message2") {
                        let mut kv_gap = false;
                        for pair in kv_str.split(" ; ") {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "temp.message2".into(),
                                    split: "=".into(),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(event, &format!("temp.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("temp.logname") {
                event.rename("temp.logname", "cisco_nexus.log.logname")?;
            }

            if event.has_value("temp.uid") {
                event.rename("temp.uid", "cisco_nexus.log.uid")?;
            }

            if event.has_value("temp.euid") {
                event.rename("temp.euid", "cisco_nexus.log.euid")?;
            }

            if event.has_value("temp.tty") {
                event.rename("temp.tty", "cisco_nexus.log.tty")?;
            }

            if event.has_value("temp.ruser") {
                event.rename("temp.ruser", "cisco_nexus.log.ruser")?;
            }

            if event.has_value("temp.rhost") {
                event.rename("temp.rhost", "cisco_nexus.log.rhost")?;
            }

            if event.has_value("temp.user") {
                event.rename("temp.user", "user.name")?;
            }

            if event.has_value("temp.COMMAND") {
                event.rename("temp.COMMAND", "cisco_nexus.log.command")?;
            }

            if event.has_value("temp.PWD") {
                event.rename("temp.PWD", "cisco_nexus.log.pwd")?;
            }

            if event.has_value("temp.TTY") {
                event.rename("temp.TTY", "cisco_nexus.log.tty")?;
            }

            if event.has_value("temp.USER") {
                event.rename("temp.USER", "user.name")?;
            }

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event.has_value("cisco_nexus.log.interface.name")
                    || event.has_value("cisco_nexus.log.network.ingress_interface")
                    || event.has_value("cisco_nexus.log.network.egress_interface")
                    || event.get_str("event.code").is_some_and(|s| {
                        [
                            "L2FM_MAC_MOVE2",
                            "L3_VPC_UNEQUAL_WEIGHT",
                            "AAA_ACCOUNTING_MESSAGE",
                            "DUP_HOSTS",
                            "NF_PARITY_ERROR",
                            "EXCESSIVE_PARITY_ERROR",
                            "DETECT_MULTIPLE_PEERS",
                            "TACACS_WARNING",
                            "SYSLOG_SL_MSG_WARNING",
                        ]
                        .contains(&s.to_uppercase().as_str())
                    })
                    || event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("kex_exchange_identification"))
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("network")]))?;
            }

            let _cond = {
                event.has_value("cisco_nexus.log.interface.name")
                    || event.has_value("cisco_nexus.log.network.ingress_interface")
                    || event.has_value("cisco_nexus.log.network.egress_interface")
                    || event.get_str("event.code").is_some_and(|s| {
                        [
                            "VSHD_SYSLOG_CONFIG_I",
                            "L2FM_MAC_MOVE2",
                            "L3_VPC_UNEQUAL_WEIGHT",
                            "AAA_ACCOUNTING_MESSAGE",
                            "DUP_HOSTS",
                            "NF_PARITY_ERROR",
                            "EXCESSIVE_PARITY_ERROR",
                            "DETECT_MULTIPLE_PEERS",
                            "TACACS_WARNING",
                            "SYSLOG_SL_MSG_WARNING",
                        ]
                        .contains(&s.to_uppercase().as_str())
                    })
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            let _cond = {
                ["VSHD_SYSLOG_CONFIG_I", "CFGWRITE_STARTED", "CFGWRITE_DONE"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("configuration")]))?;
            }

            let _cond = {
                ["CFGWRITE_STARTED", "CFGWRITE_DONE"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("LOGIN_SUCCESS")
                    || (event.get_str("event.code") == Some("SYSTEM_MSG")
                        && (event
                            .get_str("message")
                            .is_some_and(|s| s.to_lowercase().contains("authentication"))
                            || event.get_str("message").is_some_and(|s| {
                                s.to_lowercase().contains("authentication failure")
                            })
                            || event
                                .get_str("message")
                                .is_some_and(|s| s.to_lowercase().contains("login"))))
            };
            if _cond {
                event.set(
                    "event.category",
                    Value::Array(vec![json!("authentication")]),
                )?;
            }

            let _cond =
                {
                    event.get_str("event.code") == Some("LOGIN_SUCCESS")
                        || (event.get_str("event.code") == Some("SYSTEM_MSG")
                            && (event.get_str("message").is_some_and(|s| {
                                s.to_lowercase().contains("authentication failed")
                            }) || event.get_str("message").is_some_and(|s| {
                                s.to_lowercase().contains("authentication failure")
                            }) || event
                                .get_str("message")
                                .is_some_and(|s| s.to_lowercase().contains("login failed"))))
                };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("end")]))?;
            }

            let _cond =
                { ["LOGOUT", "LOGOUT_C6K"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
                event.set(
                    "event.category",
                    Value::Array(vec![json!("authentication")]),
                )?;
            }

            let _cond =
                { ["LOGOUT", "LOGOUT_C6K"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("end")]))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && event.has_value("cisco_nexus.log.command")
            };
            if _cond {
                event.set(
                    "event.category",
                    Value::Array(vec![json!("iam"), json!("process")]),
                )?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && event.has_value("cisco_nexus.log.command")
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("start")]))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && !event.has_value("event.category")
                    && (event.get_str("cisco_nexus.log.facility") == Some("USER")
                        || event.get_str("cisco_nexus.log.facility") == Some("KERN"))
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("host")]))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && !event.has_value("event.type")
                    && (event.get_str("cisco_nexus.log.facility") == Some("USER")
                        || event.get_str("cisco_nexus.log.facility") == Some("KERN"))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            let _cond = {
                event
                    .get_str("message")
                    .is_some_and(|s| s.to_lowercase().contains("kex_exchange_identification"))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("connection")]))?;
            }

            let _cond = {
                event
                    .get_str("message")
                    .is_some_and(|s| s.to_lowercase().contains("failed"))
                    || event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("failure"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event
                    .get_str("message")
                    .is_some_and(|s| s.to_lowercase().contains("successful"))
                    || event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("success"))
                    || event.get_str("event.code") == Some("IF_ADMIN_UP")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("IF_DOWN_ADMIN_DOWN")
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                ["EXCESSIVE_PARITY_ERROR", "NF_PARITY_ERROR"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("INVAL_IP") && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                ["DUPLEX_MISMATCH", "NATIVE_VLAN_MISMATCH"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("L3_VPC_UNEQUAL_WEIGHT")
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("DUP_HOSTS")
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("TACACS_WARNING")
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && event.get_str("cisco_nexus.log.facility") == Some("KERN")
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && !event.has_value("event.outcome")
                    && event.has_value("message")
                    && event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("kex_exchange_identification"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("IF_UP") && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                ["LOGOUT", "LOGOUT_C6K"].contains(&event.get_str("event.code").unwrap_or(""))
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("VSHD_SYSLOG_CONFIG_I")
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("AAA_ACCOUNTING_MESSAGE")
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && event.has_value("cisco_nexus.log.command")
                    && event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("command not allowed"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && event.has_value("cisco_nexus.log.command")
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("CFGWRITE_STARTED")
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("CFGWRITE_DONE")
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("DETECT_MULTIPLE_PEERS")
                    && !event.has_value("event.outcome")
            };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("UPDOWN")
                    && !event.has_value("event.outcome")
                    && (event.get_str("cisco_nexus.log.line_protocol_state") == Some("up")
                        || event.get_str("cisco_nexus.log.state") == Some("up"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("UPDOWN")
                    && !event.has_value("event.outcome")
                    && (event.get_str("cisco_nexus.log.line_protocol_state") == Some("down")
                        || event.get_str("cisco_nexus.log.state") == Some("down"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("event.code") == Some("IF_DOWN_ADMIN_DOWN") };
            if _cond {
                event.set("event.action", json!("interface-down"))?;
            }

            let _cond =
                { ["IF_ADMIN_UP", "IF_UP"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
                event.set("event.action", json!("interface-up"))?;
            }

            let _cond = { event.get_str("event.code") == Some("SPEED") };
            if _cond {
                event.set("event.action", json!("interface-speed-changed"))?;
            }

            let _cond = { event.get_str("event.code") == Some("IF_DUPLEX") };
            if _cond {
                event.set("event.action", json!("interface-duplex-changed"))?;
            }

            let _cond = {
                ["IF_RX_FLOW_CONTROL", "IF_TX_FLOW_CONTROL"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                event.set("event.action", json!("interface-flow-control-changed"))?;
            }

            let _cond = { event.get_str("event.code") == Some("IF_XCVR_WARNING") };
            if _cond {
                event.set("event.action", json!("transceiver-warning"))?;
            }

            let _cond = { event.get_str("event.code") == Some("UPDOWN") };
            if _cond {
                event.set("event.action", json!("interface-state-changed"))?;
            }

            let _cond = { event.get_str("event.code") == Some("LINEPROTO") };
            if _cond {
                event.set("event.action", json!("interface-state-changed"))?;
            }

            let _cond = { event.get_str("event.code") == Some("VSHD_SYSLOG_CONFIG_I") };
            if _cond {
                event.set("event.action", json!("configuration-changed"))?;
            }

            let _cond = { event.get_str("event.code") == Some("CFGWRITE_STARTED") };
            if _cond {
                event.set("event.action", json!("config-write-started"))?;
            }

            let _cond = { event.get_str("event.code") == Some("CFGWRITE_DONE") };
            if _cond {
                event.set("event.action", json!("config-write-completed"))?;
            }

            let _cond = { event.get_str("event.code") == Some("LOGIN_SUCCESS") };
            if _cond {
                event.set("event.action", json!("logged-in"))?;
            }

            let _cond =
                { ["LOGOUT", "LOGOUT_C6K"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
                event.set("event.action", json!("logged-out"))?;
            }

            let _cond = { event.get_str("event.code") == Some("DETECT_MULTIPLE_PEERS") };
            if _cond {
                event.set("event.action", json!("multiple-peers-detected"))?;
            }

            let _cond = { event.get_str("event.code") == Some("INVAL_IP") };
            if _cond {
                event.set("event.action", json!("invalid-packet-received"))?;
            }

            let _cond = { event.get_str("event.code") == Some("SYSLOG_SL_MSG_WARNING") };
            if _cond {
                event.set("event.action", json!("arp-warning"))?;
            }

            let _cond = { event.get_str("event.code") == Some("L2FM_MAC_MOVE2") };
            if _cond {
                event.set("event.action", json!("mac-address-moved"))?;
            }

            let _cond = {
                ["EXCESSIVE_PARITY_ERROR", "NF_PARITY_ERROR"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                event.set("event.action", json!("hardware-error"))?;
            }

            let _cond = { event.get_str("event.code") == Some("DUPLEX_MISMATCH") };
            if _cond {
                event.set("event.action", json!("duplex-mismatch-detected"))?;
            }

            let _cond = { event.get_str("event.code") == Some("NATIVE_VLAN_MISMATCH") };
            if _cond {
                event.set("event.action", json!("vlan-mismatch-detected"))?;
            }

            let _cond = { event.get_str("event.code") == Some("L3_VPC_UNEQUAL_WEIGHT") };
            if _cond {
                event.set("event.action", json!("vpc-config-mismatch"))?;
            }

            let _cond = { event.get_str("event.code") == Some("AAA_ACCOUNTING_MESSAGE") };
            if _cond {
                event.set("event.action", json!("session-recorded"))?;
            }

            let _cond = { event.get_str("event.code") == Some("TACACS_WARNING") };
            if _cond {
                event.set("event.action", json!("tacacs-lookup-failed"))?;
            }

            let _cond = { event.get_str("event.code") == Some("DUP_HOSTS") };
            if _cond {
                event.set("event.action", json!("duplicate-host-detected"))?;
            }

            let _cond = { event.get_str("event.code") == Some("THRESHOLD_VIOLATION") };
            if _cond {
                event.set("event.action", json!("transceiver-threshold-violated"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && event.has_value("cisco_nexus.log.command")
                    && event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("command not allowed"))
            };
            if _cond {
                event.set("event.action", json!("command-denied"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && event.has_value("cisco_nexus.log.command")
                    && !event.has_value("event.action")
            };
            if _cond {
                event.set("event.action", json!("command-executed"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && !event.has_value("event.action")
                    && event.has_value("message")
                    && (event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("authentication"))
                        || event
                            .get_str("message")
                            .is_some_and(|s| s.to_lowercase().contains("login failed")))
            };
            if _cond {
                event.set("event.action", json!("authentication-failure"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && !event.has_value("event.action")
                    && event.has_value("message")
                    && event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("kex_exchange_identification"))
            };
            if _cond {
                event.set("event.action", json!("connection-failed"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && !event.has_value("event.action")
                    && event.get_str("cisco_nexus.log.facility") == Some("KERN")
            };
            if _cond {
                event.set("event.action", json!("hardware-error"))?;
            }

            let _cond = {
                event.get_str("event.code") == Some("SYSTEM_MSG")
                    && !event.has_value("event.action")
            };
            if _cond {
                event.set("event.action", json!("system-message"))?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
