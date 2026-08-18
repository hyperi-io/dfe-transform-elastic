// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_extract_message` pipeline.
pub struct PipelineExtractMessage;

impl Transform for PipelineExtractMessage {
    fn name(&self) -> &str {
        "pipeline_extract_message"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        // TODO: conditional: ['IF_DOWN_ADMIN_DOWN','IF_ADMIN_UP','SPEED','IF_DUPLEX','IF_RX_FLOW_CONTROL','IF_TX_FLOW_CONTROL','IF_UP','IF_XCVR_WARNING'].contains(ctx.event?.code.toUpperCase())
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_str("message").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is up in mode %{DATA:cisco_nexus.log.interface.mode}$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re = regex::Regex::new(&grok_to_regex("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is up in mode %{DATA:cisco_nexus.log.interface.mode}$")).unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is %{GREEDYDATA}$
                    // Additional grok pattern 2: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational speed changed to %{DATA:cisco_nexus.log.operational.speed}$
                    // Additional grok pattern 3: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational duplex mode changed to %{DATA:cisco_nexus.log.operational.duplex_mode}$
                    // Additional grok pattern 4: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Receive Flow Control state changed to %{DATA:cisco_nexus.log.operational.receive_flow_control_state}$
                    // Additional grok pattern 5: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Transmit Flow Control state changed to %{DATA:cisco_nexus.log.operational.transmit_flow_control_state}$
                    // Additional grok pattern 6: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, %{GREEDYDATA}$
                }
                Ok(())
            })();
        }

        // TODO: conditional: ['VSHD_SYSLOG_CONFIG_I','DETECT_MULTIPLE_PEERS','UPDOWN','CFGWRITE_STARTED','LINEPROTO'].contains(ctx.event?.code.toUpperCase())
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_str("message").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^Configured from vty by %{USERNAME:user.name} on %{IP:source.ip}@%{DATA:cisco_nexus.log.terminal}$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re = regex::Regex::new(&grok_to_regex("^Configured from vty by %{USERNAME:user.name} on %{IP:source.ip}@%{DATA:cisco_nexus.log.terminal}$")).unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: ^Multiple peers detected on %{DATA:cisco_nexus.log.interface.name}$
                    // Additional grok pattern 2: ^Line (?i)protocol on Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.line_protocol_state}$
                    // Additional grok pattern 3: ^Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.state}$
                    // Additional grok pattern 4: ^%{DATA}(PID %{NUMBER:process.pid:long})%{GREEDYDATA}$
                }
                Ok(())
            })();
        }

        // TODO: conditional: ['SYSTEM_MSG'].contains(ctx.event?.code.toUpperCase())
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_str("message").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^%{DATA}authentication failure; %{GREEDYDATA:temp.message} - %{GREEDYDATA}$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re = regex::Regex::new(&grok_to_regex("^%{DATA}authentication failure; %{GREEDYDATA:temp.message} - %{GREEDYDATA}$")).unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: ^%{DATA}Authentication failure for %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
                    // Additional grok pattern 2: ^%{DATA}Authentication failed for user %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
                    // Additional grok pattern 3: ^Login failed for user %{USERNAME:user.name} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
                    // Additional grok pattern 4: ^%{DATA} : %{GREEDYDATA:temp.message2}$
                }
                Ok(())
            })();
        }

        // TODO: conditional: ['INVAL_IP','L2FM_MAC_MOVE2','DUPLEX_MISMATCH','NATIVE_VLAN_MISMATCH','THRESHOLD_VIOLATION'].contains(ctx.event?.code.toUpperCase())
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_str("message").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^%{DATA:network.protocol} %{DATA}%{SPACE}Received packet with invalid destination IP address (%{DATA}) from %{CISCOMAC:source.mac} on %{DATA:cisco_nexus.log.interface.name}$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re = regex::Regex::new(&grok_to_regex("^%{DATA:network.protocol} %{DATA}%{SPACE}Received packet with invalid destination IP address (%{DATA}) from %{CISCOMAC:source.mac} on %{DATA:cisco_nexus.log.interface.name}$")).unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: ^Mac %{CISCOMAC:source.mac} in %{DATA:cisco_nexus.log.interface.name} has moved from %{GREEDYDATA}$
                    // Additional grok pattern 2: ^%{DATA} mismatch discovered on %{DATA:cisco_nexus.log.network.ingress_interface}(?:\\(%{DATA}\\))?, with %{DATA:cisco_nexus.log.network.egress_interface}(?:\\(%{DATA}\\))?$
                    // Additional grok pattern 3: ^%{DATA:cisco_nexus.log.interface.name}: Rx power high warning; Operating value: %{DATA:cisco_nexus.log.operating_value}, Threshold value: %{DATA:cisco_nexus.log.threshold_value}.$
                }
                Ok(())
            })();
        }

        // TODO: conditional: ['LOGIN_SUCCESS','LOGOUT','LOGOUT_C6K'].contains(ctx.event?.code.toUpperCase())
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_str("message").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^Login Success \\[user: %{USERNAME:user.name}\\] \\[Source: %{IP:source.ip}\\] \\[localport: %{NUMBER:source.port:long}\\] at %{GREEDYDATA}$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re = regex::Regex::new(&grok_to_regex("^Login Success \\[user: %{USERNAME:user.name}\\] \\[Source: %{IP:source.ip}\\] \\[localport: %{NUMBER:source.port:long}\\] at %{GREEDYDATA}$")).unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: ^User %{USERNAME:user.name} %{GREEDYDATA}\\(%{IP:source.ip}\\)$
                }
                Ok(())
            })();
        }

        if event.has("source.mac") {
            if let Some(s) = event.get_str("source.mac").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("[.]").unwrap();
                let replaced = re.replace_all(s, "").into_owned();
                event.set("source.mac", replaced)?;
            }
        }

        if event.has("source.mac") {
            if let Some(s) = event.get_str("source.mac").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("(..)(?!$)").unwrap();
                let replaced = re.replace_all(s, "$1-").into_owned();
                event.set("source.mac", replaced)?;
            }
        }

        if event.has("source.mac") {
            if let Some(s) = event.get_str("source.mac").map(String::from) {
                let s = s.as_str();
                let uppered = s.to_uppercase();
                event.set("source.mac", uppered)?;
            }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("temp.message") {
                if let Some(kv_str) = event.get_str("temp.message").map(String::from) {
                    let kv_str = kv_str.as_str();
                    for pair in kv_str.split("\\s+") {
                        if let Some((key, value)) = pair.split_once("=") {
                            if !key.is_empty() {
                                event.set(&format!("temp.{}", key), value)?;
                            }
                        }
                    }
                }
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has("temp.message2") {
                if let Some(kv_str) = event.get_str("temp.message2").map(String::from) {
                    let kv_str = kv_str.as_str();
                    for pair in kv_str.split(" ; ") {
                        if let Some((key, value)) = pair.split_once("=") {
                            if !key.is_empty() {
                                event.set(&format!("temp.{}", key), value)?;
                            }
                        }
                    }
                }
            }
            Ok(())
        })();

        if event.has("temp.logname") {
            event.rename("temp.logname", "cisco_nexus.log.logname")?;
        }

        if event.has("temp.uid") {
            event.rename("temp.uid", "cisco_nexus.log.uid")?;
        }

        if event.has("temp.euid") {
            event.rename("temp.euid", "cisco_nexus.log.euid")?;
        }

        if event.has("temp.tty") {
            event.rename("temp.tty", "cisco_nexus.log.tty")?;
        }

        if event.has("temp.ruser") {
            event.rename("temp.ruser", "cisco_nexus.log.ruser")?;
        }

        if event.has("temp.rhost") {
            event.rename("temp.rhost", "cisco_nexus.log.rhost")?;
        }

        if event.has("temp.user") {
            event.rename("temp.user", "user.name")?;
        }

        if event.has("temp.COMMAND") {
            event.rename("temp.COMMAND", "cisco_nexus.log.command")?;
        }

        if event.has("temp.PWD") {
            event.rename("temp.PWD", "cisco_nexus.log.pwd")?;
        }

        if event.has("temp.TTY") {
            event.rename("temp.TTY", "cisco_nexus.log.tty")?;
        }

        if event.has("temp.USER") {
            event.rename("temp.USER", "user.name")?;
        }

        if event.has("network.protocol") {
            if let Some(s) = event.get_str("network.protocol").map(String::from) {
                let s = s.as_str();
                let lowered = s.to_lowercase();
                event.set("network.protocol", lowered)?;
            }
        }

        // TODO: conditional: ctx.cisco_nexus?.log?.interface?.name != null || ctx.cisco_nexus?.log?.network?.ingress_interface != null || ctx.cisco_nexus?.log?.network?.egress_interface != null || ['L2FM_MAC_MOVE2','L3_VPC_UNEQUAL_WEIGHT','AAA_ACCOUNTING_MESSAGE','DUP_HOSTS','NF_PARITY_ERROR','EXCESSIVE_PARITY_ERROR'].contains(ctx.event?.code.toUpperCase()) || ctx.message.toLowerCase().contains('kex_exchange_identification')
        {
            event.set("event.category", json!(["network"]))?;
        }

        // TODO: conditional: ctx.cisco_nexus?.log?.interface?.name != null || ctx.cisco_nexus?.log?.network?.ingress_interface != null || ctx.cisco_nexus?.log?.network?.egress_interface != null || ['VSHD_SYSLOG_CONFIG_I','L2FM_MAC_MOVE2','L3_VPC_UNEQUAL_WEIGHT','AAA_ACCOUNTING_MESSAGE','DUP_HOSTS','NF_PARITY_ERROR','EXCESSIVE_PARITY_ERROR'].contains(ctx.event?.code.toUpperCase())
        {
            event.set("event.type", json!(["info"]))?;
        }

        // TODO: conditional: ctx.event?.code == 'VSHD_SYSLOG_CONFIG_I'
        {
            event.set("event.category", json!(["configuration"]))?;
        }

        // TODO: conditional: ctx.event?.code == 'LOGIN_SUCCESS' || (ctx.event?.code == 'SYSTEM_MSG' && (ctx.message.toLowerCase().contains('authentication') || ctx.message.toLowerCase().contains('authentication failure') || ctx.message.toLowerCase().contains('login')))
        {
            event.set("event.category", json!(["authentication"]))?;
        }

        // TODO: conditional: ctx.event?.code == 'LOGIN_SUCCESS' || (ctx.event?.code == 'SYSTEM_MSG' && (ctx.message.toLowerCase().contains('authentication failed') || ctx.message.toLowerCase().contains('authentication failure') || ctx.message.toLowerCase().contains('login failed')))
        {
            event.set("event.type", json!(["end"]))?;
        }

        // TODO: conditional: ctx.message.toLowerCase().contains('kex_exchange_identification')
        {
            event.set("event.type", json!(["connection"]))?;
        }

        // TODO: conditional: ctx.message.toLowerCase().contains('failed') || ctx.message.toLowerCase().contains('failure')
        {
            event.set("event.outcome", json!("failure"))?;
        }

        // TODO: conditional: ctx.message.toLowerCase().contains('successful') || ctx.message.toLowerCase().contains('success') || ctx.event?.code == 'IF_ADMIN_UP'
        {
            event.set("event.outcome", json!("success"))?;
        }

        // TODO: conditional: ctx.source?.ip != null
        {
            event.append(
                "related.ip",
                event.get("source.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx.user?.name != null
        {
            event.append(
                "related.user",
                event.get("user.name").cloned().unwrap_or(Value::Null),
            )?;
        }

        Ok(TransformResult::Continue)
    }
}
