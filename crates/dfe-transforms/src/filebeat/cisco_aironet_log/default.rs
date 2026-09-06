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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("message") {
                        event.rename("message", "event.original")?;
                    }
                    Ok(())
                })();
            }

            event.set("ecs.version", json!("8.17.0"))?;

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (?:(?:<%{NONNEGINT:log.syslog.priority:int}>)%{DATA:host.name}:\\s\\*%{DATA:process.name}:\\s(?:%{MONTH:_temp_.raw_date_month}\\s+%{MONTHDAY:_temp_.raw_date_monthday}(\\s+%{YEAR:_temp_.raw_date_year})?\\s+%{TIME:_temp_.raw_date_time}(\\s+%{WORD:_temp_.raw_date_timezone})?)):\\s%%{GREEDYDATA:_temp_.full_message}
                // Grok pattern: (?:<%{NONNEGINT:log.syslog.priority:int}>)%{INT}: %{DATA:host.name}: %{INT}: (?:%{MONTH:_temp_.raw_date_month}\\s+%{MONTHDAY:_temp_.raw_date_monthday}(\\s+%{YEAR:_temp_.raw_date_year})?\\s+%{TIME:_temp_.raw_date_time}(\\s+%{WORD:_temp_.raw_date_timezone})?): %%{GREEDYDATA:_temp_.full_message}
                // Grok pattern: (?:<%{NONNEGINT:log.syslog.priority:int}>)%{INT}: AP:%{MAC:host.mac}: \\*(?:%{MONTH:_temp_.raw_date_month}\\s+%{MONTHDAY:_temp_.raw_date_monthday}(\\s+%{YEAR:_temp_.raw_date_year})?\\s+%{TIME:_temp_.raw_date_time}(\\s+%{WORD:_temp_.raw_date_timezone})?): %%{GREEDYDATA:_temp_.full_message}
                // Grok pattern: (?:<%{NONNEGINT:log.syslog.priority:int}>)%{INT}: (?:%{MONTH:_temp_.raw_date_month}\\s+%{MONTHDAY:_temp_.raw_date_monthday}(\\s+%{YEAR:_temp_.raw_date_year})?\\s+%{TIME:_temp_.raw_date_time}(\\s+%{WORD:_temp_.raw_date_timezone})?): %%{GREEDYDATA:_temp_.full_message}
                // Grok pattern: (?:<%{NONNEGINT:log.syslog.priority:int}>)%{DATA:host.name}: -%{GREEDYDATA:_temp_.full_message}
                if !extract_first_match(
                    &[
                        cached_grok!(
                            "(?:(?:<%{NONNEGINT:log.syslog.priority:int}>)%{DATA:host.name}:\\s\\*%{DATA:process.name}:\\s(?:%{MONTH:_temp_.raw_date_month}\\s+%{MONTHDAY:_temp_.raw_date_monthday}(\\s+%{YEAR:_temp_.raw_date_year})?\\s+%{TIME:_temp_.raw_date_time}(\\s+%{WORD:_temp_.raw_date_timezone})?)):\\s%%{GREEDYDATA:_temp_.full_message}"
                        ),
                        cached_grok!(
                            "(?:<%{NONNEGINT:log.syslog.priority:int}>)%{INT}: %{DATA:host.name}: %{INT}: (?:%{MONTH:_temp_.raw_date_month}\\s+%{MONTHDAY:_temp_.raw_date_monthday}(\\s+%{YEAR:_temp_.raw_date_year})?\\s+%{TIME:_temp_.raw_date_time}(\\s+%{WORD:_temp_.raw_date_timezone})?): %%{GREEDYDATA:_temp_.full_message}"
                        ),
                        cached_grok!(
                            "(?:<%{NONNEGINT:log.syslog.priority:int}>)%{INT}: AP:%{MAC:host.mac}: \\*(?:%{MONTH:_temp_.raw_date_month}\\s+%{MONTHDAY:_temp_.raw_date_monthday}(\\s+%{YEAR:_temp_.raw_date_year})?\\s+%{TIME:_temp_.raw_date_time}(\\s+%{WORD:_temp_.raw_date_timezone})?): %%{GREEDYDATA:_temp_.full_message}"
                        ),
                        cached_grok!(
                            "(?:<%{NONNEGINT:log.syslog.priority:int}>)%{INT}: (?:%{MONTH:_temp_.raw_date_month}\\s+%{MONTHDAY:_temp_.raw_date_monthday}(\\s+%{YEAR:_temp_.raw_date_year})?\\s+%{TIME:_temp_.raw_date_time}(\\s+%{WORD:_temp_.raw_date_timezone})?): %%{GREEDYDATA:_temp_.full_message}"
                        ),
                        cached_grok!(
                            "(?:<%{NONNEGINT:log.syslog.priority:int}>)%{DATA:host.name}: -%{GREEDYDATA:_temp_.full_message}"
                        ),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            // Painless script, resolved to its runners at generation time
            // Source: if (ctx.log?.syslog?.priority != null) {\n  def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  facility['code'] = ctx.log.syslog.priority>>3;\n  ctx.log.syslog['facility'] = facility;\n}\n
            syslog_priority(event, &SyslogPriorityScript::new(None, true, true, false));

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_temp_.full_message") {
                    // Grok pattern: %{DATA:event.provider}-%{INT:event.severity:long}-%{DATA:event.action}: %{DATA}:%{INT} %{GREEDYDATA:message}
                    // Grok pattern: %{DATA:event.provider}-%{INT:event.severity:long}-%{DATA:event.action}: %{GREEDYDATA:message}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "%{DATA:event.provider}-%{INT:event.severity:long}-%{DATA:event.action}: %{DATA}:%{INT} %{GREEDYDATA:message}"
                            ),
                            cached_grok!(
                                "%{DATA:event.provider}-%{INT:event.severity:long}-%{DATA:event.action}: %{GREEDYDATA:message}"
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_temp_.full_message") {
                    // Grok pattern: %{DATA:_temp_.reason}:
                    if !cached_grok!("%{DATA:_temp_.reason}:").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            event.set("log.level", json!("unknown"))?;

            let _cond = { event.get_i64("log.syslog.severity.code") == Some(1) };
            if _cond {
                event.set("log.level", json!("alert"))?;
            }

            let _cond = { event.get_i64("log.syslog.severity.code") == Some(2) };
            if _cond {
                event.set("log.level", json!("critical"))?;
            }

            let _cond = { event.get_i64("log.syslog.severity.code") == Some(3) };
            if _cond {
                event.set("log.level", json!("error"))?;
            }

            let _cond = { event.get_i64("log.syslog.severity.code") == Some(4) };
            if _cond {
                event.set("log.level", json!("warning"))?;
            }

            let _cond = { event.get_i64("log.syslog.severity.code") == Some(5) };
            if _cond {
                event.set("log.level", json!("notification"))?;
            }

            let _cond = { event.get_i64("log.syslog.severity.code") == Some(6) };
            if _cond {
                event.set("log.level", json!("informational"))?;
            }

            let _cond = { event.get_i64("log.syslog.severity.code") == Some(7) };
            if _cond {
                event.set("log.level", json!("debug"))?;
            }

            let _cond = { event.has_value("_temp_.raw_date_timezone") };
            if _cond {
                if !event.has("_conf.tz_offset") {
                    event.set(
                        "_conf.tz_offset",
                        json!(
                            event
                                .get("_temp_.raw_date_timezone")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
            }

            if !event.has("_conf.tz_offset") {
                event.set("_conf.tz_offset", json!("UTC"))?;
            }

            let _cond = {
                event.has_value("_temp_.raw_date_month")
                    && event.has_value("_temp_.raw_date_monthday")
                    && event.has_value("_temp_.raw_date_time")
                    && event.has_value("_temp_.raw_date_year")
            };
            if _cond {
                if !event.has("_temp_.raw_date") {
                    event.set(
                        "_temp_.raw_date",
                        json!(format!(
                            "{} {} {} {}",
                            event
                                .get("_temp_.raw_date_month")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_temp_.raw_date_monthday")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_temp_.raw_date_year")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_temp_.raw_date_time")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
            }

            let _cond = {
                event.has_value("_temp_.raw_date_month")
                    && event.has_value("_temp_.raw_date_monthday")
                    && event.has_value("_temp_.raw_date_time")
            };
            if _cond {
                if !event.has("_temp_.raw_date") {
                    event.set(
                        "_temp_.raw_date",
                        json!(format!(
                            "{} {} {}",
                            event
                                .get("_temp_.raw_date_month")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_temp_.raw_date_monthday")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_temp_.raw_date_time")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
            }

            let _cond = { event.has_value("_temp_.raw_date") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_temp_.raw_date") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "MMM d yyyy HH:mm:ss.SSS",
                            "MMM d HH:mm:ss.SSS",
                            "MMM d HH:mm:ss",
                        ],
                        event.get_str("_conf.tz_offset"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.raw_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason") == Some("SISF-6-ENTRY_CREATED")
                    || event.get_str("_temp_.reason") == Some("SISF-6-ENTRY_DELETED")
                    || event.get_str("_temp_.reason") == Some("SISF-6-ENTRY_CHANGED")
                    || event.get_str("_temp_.reason") == Some("LOG-6-Q_IND")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: A=%{IP:client.ip} V=%{INT} I=%{DATA:cisco.interface.type}:%{INT} P=%{INT} M=((%{MAC:client.mac})|$)
                    // Grok pattern: Username entry \\(%{DATA:user.name}\\)%{DATA}mobile %{MAC:client.mac}
                    // Grok pattern: Radius overrides %{WORD:cisco.radius.status}(?:, ignoring source %{INT:cisco.radius.source:int})?
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "A=%{IP:client.ip} V=%{INT} I=%{DATA:cisco.interface.type}:%{INT} P=%{INT} M=((%{MAC:client.mac})|$)"
                            ),
                            cached_grok!(
                                "Username entry \\(%{DATA:user.name}\\)%{DATA}mobile %{MAC:client.mac}"
                            ),
                            cached_grok!(
                                "Radius overrides %{WORD:cisco.radius.status}(?:, ignoring source %{INT:cisco.radius.source:int})?"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("AAA-5-AAA_AUTH_ADMIN_USER") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: for admin user '%{USER:user.name}' on %{IP:client.ip}
                    if !cached_grok!("for admin user '%{USER:user.name}' on %{IP:client.ip}")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("NIM-3-ADMIN_MODE_DISABLE") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Port %{INT:observer.ingress.interface.id}
                    if !cached_grok!("Port %{INT:observer.ingress.interface.id}")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("WPS-4-SIG_ALARM_OFF") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: AP %{MAC:host.mac}.*?track=%{DATA:cisco.wps.track} preced=%{INT:cisco.wps.preced:int} hits=%{INT:cisco.wps.hits:int} slot=%{INT:cisco.wps.slot:int} channel=%{INT:cisco.wps.channel:int}
                    if !cached_grok!("AP %{MAC:host.mac}.*?track=%{DATA:cisco.wps.track} preced=%{INT:cisco.wps.preced:int} hits=%{INT:cisco.wps.hits:int} slot=%{INT:cisco.wps.slot:int} channel=%{INT:cisco.wps.channel:int}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("WPS-4-SIG_ALARM_OFF_CONT") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: source mac= %{MAC:client.mac}
                    if !cached_grok!("source mac= %{MAC:client.mac}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason") == Some("WPS-4-SIG_ALARM_OFF")
                    || event.get_str("_temp_.reason") == Some("WPS-4-SIG_ALARM_OFF_CONT")
                    || event.get_str("_temp_.reason") == Some("LWAPP-4-SIG_INFO1")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.kind", json!("alert"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("_temp_.reason") == Some("LWAPP-4-SIG_INFO1") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: Signature information; AP %{MAC:destination.mac}, alarm ON, (?P<threat_indicator_description>(?:%{DATA} sig %{DATA})), track %{DATA}precedence %{INT}, hits %{INT}, slot %{INT}, channel %{INT}, most offending MAC %{MAC:source.mac}
                        if !cached_grok_mapped!("Signature information; AP %{MAC:destination.mac}, alarm ON, (?P<threat_indicator_description>(?:%{DATA} sig %{DATA})), track %{DATA}precedence %{INT}, hits %{INT}, slot %{INT}, channel %{INT}, most offending MAC %{MAC:source.mac}", [("threat_indicator_description", "threat.indicator.description")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("_temp_.reason") == Some("LWAPP-4-SIG_INFO1")
                    && event.has_value("threat.indicator.description")
                    && event.get_str("threat.indicator.description") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("threat.indicator.type", json!("process"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("_temp_.reason") == Some("DOT1X-4-MAX_EAPOL_KEY_RETRANS") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: client %{MAC:client.mac}
                    if !cached_grok!("client %{MAC:client.mac}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("RRM-3-RRM_LOGMSG") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Client not found: %{MAC:client.mac}
                    // Grok pattern: AP:\\s+%{MAC:host.mac}
                    if !extract_first_match(
                        &[
                            cached_grok!("Client not found: %{MAC:client.mac}"),
                            cached_grok!("AP:\\s+%{MAC:host.mac}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("DOT1X-3-ABORT_AUTH") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: client %{MAC:client.mac} Abort Reason:%{DATA:event.reason}$
                    if !cached_grok!("client %{MAC:client.mac} Abort Reason:%{DATA:event.reason}$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason") == Some("LOG-3-Q_IND")
                    || event.get_str("_temp_.reason") == Some("DOT1X-3-INVALID_WPA_KEY_STATE")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: version %{INT:cisco.eapol.version:int}, type %{INT:cisco.eapol.type:int}, descriptor %{INT:cisco.eapol.descriptor:int}, client %{MAC:client.mac}
                        // Grok pattern: client %{MAC:client.mac}
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "version %{INT:cisco.eapol.version:int}, type %{INT:cisco.eapol.type:int}, descriptor %{INT:cisco.eapol.descriptor:int}, client %{MAC:client.mac}"
                                ),
                                cached_grok!("client %{MAC:client.mac}"),
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
                event.get_str("_temp_.reason") == Some("APF-6-USER_NAME_CREATED")
                    || event.get_str("_temp_.reason") == Some("APF-6-USER_NAME_DELETED")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Username entry \\(%{DATA:user.name}\\)%{DATA}mobile %{MAC:client.mac}
                    if !cached_grok!(
                        "Username entry \\(%{DATA:user.name}\\)%{DATA}mobile %{MAC:client.mac}"
                    )
                    .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason") == Some("DTL-4-ARP_ORPHANPKT_DETECTED")
                    || event.get_str("_temp_.reason") == Some("LOG-4-Q_IND")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: STA\\(Target MAC Address\\) \\[%{MAC:client.mac}.*?\\] %{DATA:event.reason}\\(Source IP Address\\) %{IP:client.ip}%{DATA}\\(Destination IP Address\\) %{IP:server.ip}
                    if !cached_grok!("STA\\(Target MAC Address\\) \\[%{MAC:client.mac}.*?\\] %{DATA:event.reason}\\(Source IP Address\\) %{IP:client.ip}%{DATA}\\(Destination IP Address\\) %{IP:server.ip}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason")
                    == Some("CLIENT_ORCH_LOG-6-CLIENT_ADDED_TO_RUN_STATE")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: R%{INT:cisco.wps.slot}/%{INT:cisco.wps.radio}: wncd: Username entry \\(%{DATA:user.name}\\) joined with ssid \\(%{DATA:cisco.ssid}\\) for device with MAC: %{MAC:client.mac}(\\s+on channel \\(%{INT:cisco.wps.channel:int}\\))?
                    if !cached_grok!("R%{INT:cisco.wps.slot}/%{INT:cisco.wps.radio}: wncd: Username entry \\(%{DATA:user.name}\\) joined with ssid \\(%{DATA:cisco.ssid}\\) for device with MAC: %{MAC:client.mac}(\\s+on channel \\(%{INT:cisco.wps.channel:int}\\))?").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason") == Some("APMGR_AWIPS_SYSLOG-6-APMGR_AWIPS_MESSAGE")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AWIPS alarm:\\(%{DATA:cisco.ap_name}\\) %{MAC:client.mac} +Radio MAC %{MAC:destination.mac} +detected %{DATA:cisco.awips.alarm_type} \\(%{INT:cisco.awips.alarm_code:int}\\)
                    if !cached_grok!("Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AWIPS alarm:\\(%{DATA:cisco.ap_name}\\) %{MAC:client.mac} +Radio MAC %{MAC:destination.mac} +detected %{DATA:cisco.awips.alarm_type} \\(%{INT:cisco.awips.alarm_code:int}\\)").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("SESSION_MGR-5-FAIL") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Authorization failed or unapplied for client \\(%{MAC:client.mac}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{DATA:cisco.audit_session_id}\\. Failure Reason: %{DATA:event.reason}\\. Failed attribute name %{INT}\\.
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Authorization failed or unapplied for client \\(%{MAC:client.mac}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{DATA:cisco.audit_session_id}\\. Failure reason: Authc fail\\. Authc failure reason: %{DATA:event.reason}\\.
                    // Grok pattern: R%{INT}/%{INT}: %{WORD}: Authorization failed or unapplied for client \\(%{MAC:client.mac}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{DATA:cisco.audit_session_id}\\. Failure Reason: %{DATA:event.reason}\\. Failed attribute name %{INT}\\.
                    // Grok pattern: R%{INT}/%{INT}: %{WORD}: Authorization failed or unapplied for client \\(%{MAC:client.mac}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{DATA:cisco.audit_session_id}\\. Failure reason: Authc fail\\. Authc failure reason: %{DATA:event.reason}\\.
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Authorization failed or unapplied for client \\(%{MAC:client.mac}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{DATA:cisco.audit_session_id}\\. Failure Reason: %{DATA:event.reason}\\. Failed attribute name %{INT}\\."
                            ),
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Authorization failed or unapplied for client \\(%{MAC:client.mac}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{DATA:cisco.audit_session_id}\\. Failure reason: Authc fail\\. Authc failure reason: %{DATA:event.reason}\\."
                            ),
                            cached_grok!(
                                "R%{INT}/%{INT}: %{WORD}: Authorization failed or unapplied for client \\(%{MAC:client.mac}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{DATA:cisco.audit_session_id}\\. Failure Reason: %{DATA:event.reason}\\. Failed attribute name %{INT}\\."
                            ),
                            cached_grok!(
                                "R%{INT}/%{INT}: %{WORD}: Authorization failed or unapplied for client \\(%{MAC:client.mac}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{DATA:cisco.audit_session_id}\\. Failure reason: Authc fail\\. Authc failure reason: %{DATA:event.reason}\\."
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("DOT1X-5-FAIL") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id} Username: %{GREEDYDATA:user.name}
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id}
                    // Grok pattern: R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id} Username: %{GREEDYDATA:user.name}
                    // Grok pattern: R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id} Username: %{GREEDYDATA:user.name}"
                            ),
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id}"
                            ),
                            cached_grok!(
                                "R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id} Username: %{GREEDYDATA:user.name}"
                            ),
                            cached_grok!(
                                "R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id}"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("MAB-5-FAIL") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id}
                    // Grok pattern: R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id}"
                            ),
                            cached_grok!(
                                "R%{INT}/%{INT}: %{WORD}: Authentication failed for client \\(%{MAC:client.mac}\\) with reason \\(%{DATA:event.reason}\\) on Interface %{DATA:observer.ingress.interface.name} AuditSessionID %{NOTSPACE:cisco.audit_session_id}"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason")
                    == Some("CLIENT_EXCLUSION_SERVER-5-ADD_TO_EXCLUSIONLIST_REASON_DYNAMIC")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Client MAC: %{MAC:client.mac} was added to exclusion list associated with AP Name:%{DATA:cisco.ap_name}, BSSID:MAC: %{MAC:destination.mac}, reason:%{GREEDYDATA:event.reason}
                    if !cached_grok!("Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Client MAC: %{MAC:client.mac} was added to exclusion list associated with AP Name:%{DATA:cisco.ap_name}, BSSID:MAC: %{MAC:destination.mac}, reason:%{GREEDYDATA:event.reason}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason")
                    == Some("CLIENT_ORCH_LOG-5-ADD_TO_EXCLUSIONLIST_REASON")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Client MAC: %{MAC:client.mac} with IP: %{IP:client.ip} was added to exclusion list, legit Client MAC: %{MAC:destination.mac}, IP: %{IP:server.ip}, reason: %{GREEDYDATA:event.reason}
                    if !cached_grok!("Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Client MAC: %{MAC:client.mac} with IP: %{IP:client.ip} was added to exclusion list, legit Client MAC: %{MAC:destination.mac}, IP: %{IP:server.ip}, reason: %{GREEDYDATA:event.reason}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason")
                    == Some("CLIENT_ORCH_LOG-5-ADD_TO_EXCLUSIONLIST_MAC_THEFT_REASON")
            };
            if _cond {
                if let Some(input) = event.get_string("_temp_.full_message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Client MAC: %{MAC:client.mac} with IP: %{IP:client.ip} was added to exclusion list, legit ifid: %{DATA}, current ifid: %{DATA}, reason: %{GREEDYDATA:event.reason}
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Client MAC: %{MAC:client.mac} with IP: %{DATA:client.ip} was added to exclusion list, legit ifid: %{DATA}, current ifid: %{DATA}, reason: %{GREEDYDATA:event.reason}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Client MAC: %{MAC:client.mac} with IP: %{IP:client.ip} was added to exclusion list, legit ifid: %{DATA}, current ifid: %{DATA}, reason: %{GREEDYDATA:event.reason}"
                            ),
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Client MAC: %{MAC:client.mac} with IP: %{DATA:client.ip} was added to exclusion list, legit ifid: %{DATA}, current ifid: %{DATA}, reason: %{GREEDYDATA:event.reason}"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("FMANFP-6-IPACCESSLOGP") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} F%{INT}/%{INT}: %{WORD}: list %{DATA:cisco.acl.name} %{WORD:cisco.acl.action} %{WORD:network.transport} %{IP:source.ip}\\(%{INT:source.port:int}\\) -> %{IP:destination.ip}\\(%{INT:destination.port:int}\\), %{INT} packets?
                    // Grok pattern: Chassis %{INT} F%{INT}/%{INT}: %{WORD}: list %{DATA:cisco.acl.name} %{WORD:cisco.acl.action} %{WORD:network.transport} \\[%{DATA}\\] %{IP:source.ip}\\(%{INT:source.port:int}\\) -> %{IP:destination.ip}\\(%{INT:destination.port:int}\\), %{INT} packets?
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Chassis %{INT} F%{INT}/%{INT}: %{WORD}: list %{DATA:cisco.acl.name} %{WORD:cisco.acl.action} %{WORD:network.transport} %{IP:source.ip}\\(%{INT:source.port:int}\\) -> %{IP:destination.ip}\\(%{INT:destination.port:int}\\), %{INT} packets?"
                            ),
                            cached_grok!(
                                "Chassis %{INT} F%{INT}/%{INT}: %{WORD}: list %{DATA:cisco.acl.name} %{WORD:cisco.acl.action} %{WORD:network.transport} \\[%{DATA}\\] %{IP:source.ip}\\(%{INT:source.port:int}\\) -> %{IP:destination.ip}\\(%{INT:destination.port:int}\\), %{INT} packets?"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("FMANFP-6-IPACCESSLOGNP") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} F%{INT}/%{INT}: %{WORD}: list %{DATA:cisco.acl.name} %{WORD:cisco.acl.action} %{INT} %{IP:source.ip} -> %{IP:destination.ip}, %{INT} packets?
                    if !cached_grok!("Chassis %{INT} F%{INT}/%{INT}: %{WORD}: list %{DATA:cisco.acl.name} %{WORD:cisco.acl.action} %{INT} %{IP:source.ip} -> %{IP:destination.ip}, %{INT} packets?").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("FMANFP-6-IPACCESSLOGDP") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} F%{INT}/%{INT}: %{WORD}: list %{DATA:cisco.acl.name} %{WORD:cisco.acl.action} icmp %{IP:source.ip} -> %{IP:destination.ip} \\(%{DATA}\\), %{INT} packets
                    if !cached_grok!("Chassis %{INT} F%{INT}/%{INT}: %{WORD}: list %{DATA:cisco.acl.name} %{WORD:cisco.acl.action} icmp %{IP:source.ip} -> %{IP:destination.ip} \\(%{DATA}\\), %{INT} packets").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason") == Some("SEC_LOGIN-5-LOGIN_SUCCESS")
                    || event.get_str("_temp_.reason") == Some("SEC_LOGIN-5-WEBLOGIN_SUCCESS")
            };
            if _cond {
                if let Some(input) = event.get_string("_temp_.full_message") {
                    // Grok pattern: SEC_LOGIN-5-LOGIN_SUCCESS: Login Success \\[user: %{DATA:user.name}\\] \\[Source: %{IP:client.ip}\\] \\[localport: +%{INT:source.port:int}\\] at %{GREEDYDATA}
                    // Grok pattern: SEC_LOGIN-5-WEBLOGIN_SUCCESS: Login Success \\[user: %{DATA:user.name}\\] \\[Source: %{IP:client.ip}\\] at %{GREEDYDATA}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "SEC_LOGIN-5-LOGIN_SUCCESS: Login Success \\[user: %{DATA:user.name}\\] \\[Source: %{IP:client.ip}\\] \\[localport: +%{INT:source.port:int}\\] at %{GREEDYDATA}"
                            ),
                            cached_grok!(
                                "SEC_LOGIN-5-WEBLOGIN_SUCCESS: Login Success \\[user: %{DATA:user.name}\\] \\[Source: %{IP:client.ip}\\] at %{GREEDYDATA}"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("DMI-5-AUTH_PASSED") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.full_message") {
                    // Grok pattern: DMI-5-AUTH_PASSED: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: User '%{DATA:user.name}' authenticated successfully from %{IP:client.ip}:%{INT:source.port:int}  for %{NOTSPACE:cisco.auth.method} over %{WORD}\\. External groups: %{GREEDYDATA}
                    if !cached_grok!("DMI-5-AUTH_PASSED: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: User '%{DATA:user.name}' authenticated successfully from %{IP:client.ip}:%{INT:source.port:int}  for %{NOTSPACE:cisco.auth.method} over %{WORD}\\. External groups: %{GREEDYDATA}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason") == Some("SSH-5-SSH2_SESSION")
                    || event.get_str("_temp_.reason") == Some("SSH-5-SSH2_CLOSE")
                    || event.get_str("_temp_.reason") == Some("SSH-5-SSH2_USERAUTH")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: SSH2 Session request from %{IP:client.ip} \\(tty = %{INT}\\) using crypto cipher '%{DATA:tls.cipher}'
                    // Grok pattern: SSH2 Session from %{IP:client.ip} \\(tty = %{INT}\\) for user '%{DATA:user.name}' using crypto cipher '%{DATA:tls.cipher}'
                    // Grok pattern: User '%{DATA:user.name}' authentication for SSH2 Session from %{IP:client.ip} \\(tty = %{INT}\\) using crypto cipher '%{DATA:tls.cipher}'
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "SSH2 Session request from %{IP:client.ip} \\(tty = %{INT}\\) using crypto cipher '%{DATA:tls.cipher}'"
                            ),
                            cached_grok!(
                                "SSH2 Session from %{IP:client.ip} \\(tty = %{INT}\\) for user '%{DATA:user.name}' using crypto cipher '%{DATA:tls.cipher}'"
                            ),
                            cached_grok!(
                                "User '%{DATA:user.name}' authentication for SSH2 Session from %{IP:client.ip} \\(tty = %{INT}\\) using crypto cipher '%{DATA:tls.cipher}'"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason") == Some("RADIUS_AUDIT_MESSAGE-6-RADIUS_ALIVE")
                    || event.get_str("_temp_.reason") == Some("RADIUS_AUDIT_MESSAGE-6-RADIUS_DEAD")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: RADIUS server %{DATA:cisco.radius.server} is being marked alive\\.
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: RADIUS server %{DATA:cisco.radius.server} is not responding\\.
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: RADIUS server %{DATA:cisco.radius.server} is being marked alive\\."
                            ),
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: RADIUS server %{DATA:cisco.radius.server} is not responding\\."
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason")
                    == Some("CAPWAPAC_SMGR_TRACE_MESSAGE-5-AP_JOIN_DISJOIN")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AP Event: AP Name: %{DATA:cisco.ap_name} Mac: %{MAC:destination.mac} Session-IP: %{IP:source.ip}\\[%{INT:source.port:int}\\] %{IP:destination.ip}\\[%{INT:destination.port:int}\\] %{GREEDYDATA:event.reason}
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AP Event: AP Name: %{DATA:cisco.ap_name} Mac: %{MAC:destination.mac} Session-IP: %{IP:source.ip}\\[%{INT:source.port:int}\\] %{GREEDYDATA:event.reason}
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AP Event: AP Name: %{DATA:cisco.ap_name} Mac:\\s*Session-IP: %{IP:source.ip}\\[%{INT:source.port:int}\\] %{IP:destination.ip}\\[%{INT:destination.port:int}\\] %{GREEDYDATA:event.reason}
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AP Event: AP Name:\\s*Mac: %{MAC:destination.mac} Session-IP: %{IP:source.ip}\\[%{INT:source.port:int}\\] %{GREEDYDATA:event.reason}
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AP Event: AP Name:\\s*Mac:\\s*Session-IP: %{IP:source.ip}\\[%{INT:source.port:int}\\] %{GREEDYDATA:event.reason}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AP Event: AP Name: %{DATA:cisco.ap_name} Mac: %{MAC:destination.mac} Session-IP: %{IP:source.ip}\\[%{INT:source.port:int}\\] %{IP:destination.ip}\\[%{INT:destination.port:int}\\] %{GREEDYDATA:event.reason}"
                            ),
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AP Event: AP Name: %{DATA:cisco.ap_name} Mac: %{MAC:destination.mac} Session-IP: %{IP:source.ip}\\[%{INT:source.port:int}\\] %{GREEDYDATA:event.reason}"
                            ),
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AP Event: AP Name: %{DATA:cisco.ap_name} Mac:\\s*Session-IP: %{IP:source.ip}\\[%{INT:source.port:int}\\] %{IP:destination.ip}\\[%{INT:destination.port:int}\\] %{GREEDYDATA:event.reason}"
                            ),
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AP Event: AP Name:\\s*Mac: %{MAC:destination.mac} Session-IP: %{IP:source.ip}\\[%{INT:source.port:int}\\] %{GREEDYDATA:event.reason}"
                            ),
                            cached_grok!(
                                "Chassis %{INT} R%{INT}/%{INT}: %{WORD}: AP Event: AP Name:\\s*Mac:\\s*Session-IP: %{IP:source.ip}\\[%{INT:source.port:int}\\] %{GREEDYDATA:event.reason}"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("MM_LOG-4-RETRIES_FAILED") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: MAC: %{MAC:client.mac}: All retries of handoff_end \\(XID: %{INT}\\) to ipv4: %{IP:destination.ip}  have been exhausted
                    if !cached_grok!("Chassis %{INT} R%{INT}/%{INT}: %{WORD}: MAC: %{MAC:client.mac}: All retries of handoff_end \\(XID: %{INT}\\) to ipv4: %{IP:destination.ip}  have been exhausted").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("SISF-4-EXCESS_ARP_ACTIVITY") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Excessive ARP activity detected for the client %{MAC:client.mac}\\. client is brought down and added to the exclusion list
                    if !cached_grok!("Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Excessive ARP activity detected for the client %{MAC:client.mac}\\. client is brought down and added to the exclusion list").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("SWPORT-4-MAC_CONFLICT") };
            if _cond {
                if let Some(input) = event.get_string("_temp_.full_message") {
                    // Grok pattern: SWPORT-4-MAC_CONFLICT: Chassis %{INT} F%{INT}: %{WORD}: Dynamic mac %{MAC:client.mac} from %{NOTSPACE} conflict with %{GREEDYDATA}
                    if !cached_grok!("SWPORT-4-MAC_CONFLICT: Chassis %{INT} F%{INT}: %{WORD}: Dynamic mac %{MAC:client.mac} from %{NOTSPACE} conflict with %{GREEDYDATA}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("SYS-5-CONFIG_P") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Configured programmatically by process %{DATA} from console as %{DATA:user.name} on %{DATA}
                    if !cached_grok!("Configured programmatically by process %{DATA} from console as %{DATA:user.name} on %{DATA}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("_temp_.reason") == Some("SYS-6-LOGOUT") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: User %{DATA:user.name} has exited tty session %{INT}\\(%{IP:client.ip}\\)
                    if !cached_grok!(
                        "User %{DATA:user.name} has exited tty session %{INT}\\(%{IP:client.ip}\\)"
                    )
                    .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason") == Some("LOADBALANCE_TRACE_MESSAGE-5-LB_LOG_MSG")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Loadbalancer Log : AP \\( %{IP:source.ip} \\) : Loadbalancer algorithm assigned Instance \\(%{INT:cisco.loadbalance.instance:int}\\) for site tag \\(%{DATA:cisco.site_tag}\\)
                    if !cached_grok!("Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Loadbalancer Log : AP \\( %{IP:source.ip} \\) : Loadbalancer algorithm assigned Instance \\(%{INT:cisco.loadbalance.instance:int}\\) for site tag \\(%{DATA:cisco.site_tag}\\)").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.get_str("_temp_.reason")
                    == Some("MCAST_ERROR_MESSAGE-3-MCAST_GRP_JOIN_LEAVE_FAILED")
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Failed to handle IGMP join\\. No MAC address for client IP: %{IP:client.ip}, group IP: %{IP:destination.ip}, vlan: %{INT:network.vlan.id} for client %{MAC:client.mac}  multicast group %{WORD}, mgid %{INT}
                    if !cached_grok!("Chassis %{INT} R%{INT}/%{INT}: %{WORD}: Failed to handle IGMP join\\. No MAC address for client IP: %{IP:client.ip}, group IP: %{IP:destination.ip}, vlan: %{INT:network.vlan.id} for client %{MAC:client.mac}  multicast group %{WORD}, mgid %{INT}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: client %{MAC:client.mac}
                        if !cached_grok!("client %{MAC:client.mac}").extract_into(&input, event)? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("client.mac") {
                map_strings(event, "client.mac", "client.mac", str::to_uppercase)?;
            }

            if event.has_value("client.mac") {
                gsub_field(
                    event,
                    "client.mac",
                    "client.mac",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            let _cond = { event.has_value("client.mac") };
            if _cond {
                // Painless script
                // Source: def mac = ctx.client.mac;\ndef pattern = /^[A-F0-9]{4}(-[A-F0-9]{4}){2}$/;\ndef matcher = pattern.matcher(mac);\nif (matcher.matches()) {\n   ctx.client.mac = mac.substring(0,2) + \"-\" + mac.substring(2,4) + \"-\" + mac.substring(5,7) + \"-\" + mac.substring(7,9) + \"-\" + mac.substring(10,12) + \"-\" + mac.substring(12,14);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def mac = ctx.client.mac;\ndef pattern = /^[A-F0-9]{4}(-[A-F0-9]{4}){2}$/;\ndef matcher = pattern.matcher(mac);\nif (matcher.matches()) {\n   ctx.client.mac = mac.substring(0,2) + \"-\" + mac.substring(2,4) + \"-\" + mac.substring(5,7) + \"-\" + mac.substring(7,9) + \"-\" + mac.substring(10,12) + \"-\" + mac.substring(12,14);\n}\n"#
                    ),
                )?;
            }

            if event.has_value("source.mac") {
                map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
            }

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("[-:.]"),
                    "-",
                )?;
            }

            if event.has_value("destination.mac") {
                map_strings(
                    event,
                    "destination.mac",
                    "destination.mac",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!("[-:.]"),
                    "-",
                )?;
            }

            let _cond = { event.has_value("destination.mac") };
            if _cond {
                // Painless script
                // Source: def mac = ctx.destination.mac;\ndef pattern = /^[A-F0-9]{4}(-[A-F0-9]{4}){2}$/;\ndef matcher = pattern.matcher(mac);\nif (matcher.matches()) {\n   ctx.destination.mac = mac.substring(0,2) + \"-\" + mac.substring(2,4) + \"-\" + mac.substring(5,7) + \"-\" + mac.substring(7,9) + \"-\" + mac.substring(10,12) + \"-\" + mac.substring(12,14);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def mac = ctx.destination.mac;\ndef pattern = /^[A-F0-9]{4}(-[A-F0-9]{4}){2}$/;\ndef matcher = pattern.matcher(mac);\nif (matcher.matches()) {\n   ctx.destination.mac = mac.substring(0,2) + \"-\" + mac.substring(2,4) + \"-\" + mac.substring(5,7) + \"-\" + mac.substring(7,9) + \"-\" + mac.substring(10,12) + \"-\" + mac.substring(12,14);\n}\n"#
                    ),
                )?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "")?;
            }

            if event.has_value("host.mac") {
                gsub_field(
                    event,
                    "host.mac",
                    "host.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("client.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("client.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("client.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("client.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("client.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("client.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("client.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("client.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("client.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("client.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("client.as.asn") {
                event.rename("client.as.asn", "client.as.number")?;
            }

            if event.has_value("client.as.organization_name") {
                event.rename("client.as.organization_name", "client.as.organization.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_temp_");
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_conf");
                Ok(())
            })();

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
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
