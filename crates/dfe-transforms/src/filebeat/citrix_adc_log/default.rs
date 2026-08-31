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
            event.set("ecs.version", json!("8.11.0"))?;

            event.set("observer.vendor", json!("Citrix"))?;

            event.set("observer.product", json!("Netscaler"))?;

            event.set("observer.type", json!("firewall"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if event.has_value("event.original") {
                gsub_field(
                    event,
                    "event.original",
                    "event.original",
                    cached_regex!("[\r\n]"),
                    "",
                )?;
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^(?:(?:%{SYSLOGTIMESTAMP:_tmp.syslog_timestamp}|(?P<_tmp_syslog_timestamp8601>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:_tmp.tz}?))))( (?:<?(?P<citrix_facility>(?:[a-zA-Z][a-zA-Z0-9]*))\\.(?P<citrix_priority>(?:[a-zA-Z][a-zA-Z0-9]*))>?) %{IP:client.ip:ip})?( %{HOSTNAME:citrix.hostname})? %{GREEDYDATA:citrix.detail}
                // Grok pattern: ^%{GREEDYDATA:citrix.detail}
                if !extract_first_match(
                    &[
                        cached_grok_mapped!(
                            "^(?:(?:%{SYSLOGTIMESTAMP:_tmp.syslog_timestamp}|(?P<_tmp_syslog_timestamp8601>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:_tmp.tz}?))))( (?:<?(?P<citrix_facility>(?:[a-zA-Z][a-zA-Z0-9]*))\\.(?P<citrix_priority>(?:[a-zA-Z][a-zA-Z0-9]*))>?) %{IP:client.ip:ip})?( %{HOSTNAME:citrix.hostname})? %{GREEDYDATA:citrix.detail}",
                            [
                                ("_tmp_syslog_timestamp8601", "_tmp.syslog_timestamp8601"),
                                ("citrix_facility", "citrix.facility"),
                                ("citrix_priority", "citrix.priority")
                            ]
                        ),
                        cached_grok!("^%{GREEDYDATA:citrix.detail}"),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            let _cond = {
                event.has_value("citrix.detail")
                    && !(event
                        .get_str("citrix.detail")
                        .is_some_and(|s| s.starts_with("CEF:")))
            };
            if _cond {
                // Begin nested pipeline: "native"
                event.set("citrix.cef_format", json!(false))?;
                if let Some(input) = event.get_string("citrix.detail") {
                    // Grok pattern: ^%{SPACE}(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}|%{MONTHDAY}/%{MONTHNUM}/%{YEAR}):%{HOUR}:%{MINUTE}:%{SECOND})) (?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : %{DATA:_tmp.details} : +\"%{GREEDYDATA:citrix.extended.message}\"
                    // Grok pattern: ^%{SPACE}(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}|%{MONTHDAY}/%{MONTHNUM}/%{YEAR}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone}?%{SPACE}(?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : %{DATA:_tmp.details} : +\"%{GREEDYDATA:citrix.extended.message}\"
                    // Grok pattern: ^%{SPACE}(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}|%{MONTHDAY}/%{MONTHNUM}/%{YEAR}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone}?%{SPACE}(?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : %{DATA:_tmp.details} : +%{GREEDYDATA:citrix.extended.message}
                    // Grok pattern: ^<%{NUMBER}>%{NUMBER} (%{TIMESTAMP_ISO8601:_tmp.timestamp}|-) (%{SYSLOGHOST:citrix.host}|-) (%{DATA:_tmp.appname}|-) (%{DATA:_tmp.procid}|-) (%{DATA:_tmp.msgid}|-) (%{DATA:_tmp.structured_data}|-) (%{DATA:_tmp.details} :)?%{SPACE}\"?%{GREEDYDATA:citrix.extended.message}\"?$
                    // Grok pattern: ^(?P<_tmp_details>(?:(?:<%{NUMBER}>%{SPACE})?default %{WORD} %{INT} %{INT})) : +\"%{GREEDYDATA:citrix.extended.message}\"$
                    // Grok pattern: ^(?P<_tmp_details>(?:(?:<%{NUMBER}>%{SPACE})?default %{WORD} %{INT} %{INT})) : +%{GREEDYDATA:citrix.extended.message}$
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^%{SPACE}(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}|%{MONTHDAY}/%{MONTHNUM}/%{YEAR}):%{HOUR}:%{MINUTE}:%{SECOND})) (?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : %{DATA:_tmp.details} : +\"%{GREEDYDATA:citrix.extended.message}\"",
                                [("_tmp_timestamp_native", "_tmp.timestamp_native")]
                            ),
                            cached_grok_mapped!(
                                "^%{SPACE}(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}|%{MONTHDAY}/%{MONTHNUM}/%{YEAR}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone}?%{SPACE}(?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : %{DATA:_tmp.details} : +\"%{GREEDYDATA:citrix.extended.message}\"",
                                [("_tmp_timestamp_native", "_tmp.timestamp_native")]
                            ),
                            cached_grok_mapped!(
                                "^%{SPACE}(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}|%{MONTHDAY}/%{MONTHNUM}/%{YEAR}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone}?%{SPACE}(?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : %{DATA:_tmp.details} : +%{GREEDYDATA:citrix.extended.message}",
                                [("_tmp_timestamp_native", "_tmp.timestamp_native")]
                            ),
                            cached_grok!(
                                "^<%{NUMBER}>%{NUMBER} (%{TIMESTAMP_ISO8601:_tmp.timestamp}|-) (%{SYSLOGHOST:citrix.host}|-) (%{DATA:_tmp.appname}|-) (%{DATA:_tmp.procid}|-) (%{DATA:_tmp.msgid}|-) (%{DATA:_tmp.structured_data}|-) (%{DATA:_tmp.details} :)?%{SPACE}\"?%{GREEDYDATA:citrix.extended.message}\"?$"
                            ),
                            cached_grok_mapped!(
                                "^(?P<_tmp_details>(?:(?:<%{NUMBER}>%{SPACE})?default %{WORD} %{INT} %{INT})) : +\"%{GREEDYDATA:citrix.extended.message}\"$",
                                [("_tmp_details", "_tmp.details")]
                            ),
                            cached_grok_mapped!(
                                "^(?P<_tmp_details>(?:(?:<%{NUMBER}>%{SPACE})?default %{WORD} %{INT} %{INT})) : +%{GREEDYDATA:citrix.extended.message}$",
                                [("_tmp_details", "_tmp.details")]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                if event.has_value("_tmp.details") {
                    if let Some(input) = event.get_string("_tmp.details") {
                        // Grok pattern: ^(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_default>(?:default ))%{WORD:citrix.device_event_class_id} %{DATA:citrix.name} %{INT:event.id} %{INT:event.severity}$
                        // Grok pattern: ^(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_default>(?:default ))%{WORD:citrix.name} %{INT:event.id} %{INT:event.severity}$
                        // Grok pattern: ^(?:<%{NUMBER}>%{SPACE})?%{WORD:citrix.device_event_class_id} %{DATA:citrix.name} %{INT:event.id} %{INT:event.severity}$
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_default>(?:default ))%{WORD:citrix.device_event_class_id} %{DATA:citrix.name} %{INT:event.id} %{INT:event.severity}$",
                                    [("_tmp_default", "_tmp.default")]
                                ),
                                cached_grok_mapped!(
                                    "^(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_default>(?:default ))%{WORD:citrix.name} %{INT:event.id} %{INT:event.severity}$",
                                    [("_tmp_default", "_tmp.default")]
                                ),
                                cached_grok!(
                                    "^(?:<%{NUMBER}>%{SPACE})?%{WORD:citrix.device_event_class_id} %{DATA:citrix.name} %{INT:event.id} %{INT:event.severity}$"
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
                    !event.has_value("citrix.device_event_class_id")
                        && event.has_value("_tmp.appname")
                        && event.get_str("_tmp.appname").is_some_and(|s| {
                            [
                                "TCP",
                                "ACL",
                                "ALG",
                                "SSLVPN",
                                "AAATM",
                                "CI",
                                "SSLLOG",
                                "TRANSFORM",
                                "ICA",
                                "APPFW",
                                "CVPN",
                                "BOT",
                                "PITBOSS",
                                "UI",
                                "CLI",
                                "GUI",
                                "API",
                                "CONSOLE",
                                "AAA",
                                "DNS",
                                "SSLI",
                            ]
                            .contains(&s.to_uppercase().as_str())
                        })
                };
                if _cond {
                    if let Some(v) = event.get("_tmp.appname").cloned() {
                        event.set("citrix.device_event_class_id", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("citrix.device_event_class_id")
                        && event.has_value("citrix.name")
                        && [
                            "LOGOUT",
                            "LOGIN",
                            "HTTPREQUEST",
                            "ICASTART",
                            "ICAEND_CONNSTAT",
                            "TCPCONNSTAT",
                            "TCPCONN_TIMEDOUT",
                            "UDPFLOWSTAT",
                            "NONHTTP_RESOURCEACCESS_DENIED",
                            "HTTP_RESOURCEACCESS_DENIED",
                            "LICLMT_REACHED",
                            "CLISEC_CHECK",
                            "STA_VALIDATE_RESP",
                            "REMOVE_SESSION_DEBUG",
                            "CLISEC_EXP_EVAL",
                        ]
                        .contains(&event.get_str("citrix.name").unwrap_or(""))
                };
                if _cond {
                    event.set("citrix.device_event_class_id", json!("SSLVPN"))?;
                }
                let _cond = {
                    !event.has_value("citrix.device_event_class_id")
                        && event.has_value("citrix.name")
                        && ["CMD_EXECUTED"].contains(&event.get_str("citrix.name").unwrap_or(""))
                };
                if _cond {
                    event.set("citrix.device_event_class_id", json!("CLI"))?;
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && !(event
                            .get_str("citrix.device_event_class_id")
                            .is_some_and(|s| {
                                ["UI", "CLI", "GUI", "API", "CONSOLE", "AAA", "SSLVPN"]
                                    .contains(&s.to_uppercase().as_str())
                            }))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("network")]))?;
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event
                            .get_str("citrix.device_event_class_id")
                            .is_some_and(|s| ["AAA", "SSLVPN"].contains(&s.to_uppercase().as_str()))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication")]),
                    )?;
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && !(event
                            .get_str("citrix.device_event_class_id")
                            .is_some_and(|s| {
                                ["TCP", "UI", "CLI", "GUI", "API", "CONSOLE"]
                                    .contains(&s.to_uppercase().as_str())
                            }))
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event
                            .get_str("citrix.device_event_class_id")
                            .is_some_and(|s| ["TCP"].contains(&s.to_uppercase().as_str()))
                };
                if _cond {
                    event.set(
                        "event.type",
                        Value::Array(vec![json!("end"), json!("connection")]),
                    )?;
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event
                            .get_str("citrix.device_event_class_id")
                            .is_some_and(|s| {
                                ["UI", "CLI", "GUI", "API", "CONSOLE"]
                                    .contains(&s.to_uppercase().as_str())
                            })
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("process")]))?;
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event
                            .get_str("citrix.device_event_class_id")
                            .is_some_and(|s| {
                                ["UI", "CLI", "GUI", "API", "CONSOLE"]
                                    .contains(&s.to_uppercase().as_str())
                            })
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("citrix.extended.message") {
                        // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip} - Destination %{IP:citrix_adc.log.destination.ip} URL %{DATA:citrix_adc.log.url} - Category %{DATA:citrix_adc.log.category} - Category%{SPACE}group %{DATA:citrix_adc.log.category_group} - Reputation %{INT:citrix_adc.log.reputation} - Policy%{SPACE}action %{WORD:citrix_adc.log.policy_action}$
                        // Grok pattern: ^User %{USER:citrix_adc.log.user} -( ADM_User %{DATA:citrix_adc.log.adm_user} -)? Remote_ip %{IP:citrix_adc.log.remote_ip} - Command \\\"%{DATA:citrix_adc.log.command}\\\" - Status \\\"%{DATA:citrix_adc.log.status}\\\"$
                        // Grok pattern: ^Session %{GREEDYDATA:citrix_adc.log.session}$
                        // Grok pattern: ^User%{SPACE}Name%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.username}$
                        // Grok pattern: ^Failure%{SPACE}Reason%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.failure_reason}$
                        // Grok pattern: ^User %{USER:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Failure_reason \\\"%{DATA:citrix_adc.log.failure_reason}\\\" - Browser %{DATA:citrix_adc.log.browser}$
                        // Grok pattern: ^Extracted_groups \\\"%{GREEDYDATA:citrix_adc.log.groups}\\\"$
                        // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^Source %{IP:citrix_adc.log.source.ip} - Destination %{IP:citrix_adc.log.destination.ip} URL %{DATA:citrix_adc.log.url} - Category %{DATA:citrix_adc.log.category} - Category%{SPACE}group %{DATA:citrix_adc.log.category_group} - Reputation %{INT:citrix_adc.log.reputation} - Policy%{SPACE}action %{WORD:citrix_adc.log.policy_action}$"
                                ),
                                cached_grok!(
                                    "^User %{USER:citrix_adc.log.user} -( ADM_User %{DATA:citrix_adc.log.adm_user} -)? Remote_ip %{IP:citrix_adc.log.remote_ip} - Command \\\"%{DATA:citrix_adc.log.command}\\\" - Status \\\"%{DATA:citrix_adc.log.status}\\\"$"
                                ),
                                cached_grok!("^Session %{GREEDYDATA:citrix_adc.log.session}$"),
                                cached_grok!(
                                    "^User%{SPACE}Name%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.username}$"
                                ),
                                cached_grok!(
                                    "^Failure%{SPACE}Reason%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.failure_reason}$"
                                ),
                                cached_grok!(
                                    "^User %{USER:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Failure_reason \\\"%{DATA:citrix_adc.log.failure_reason}\\\" - Browser %{DATA:citrix_adc.log.browser}$"
                                ),
                                cached_grok!(
                                    "^Extracted_groups \\\"%{GREEDYDATA:citrix_adc.log.groups}\\\"$"
                                ),
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
                let _cond = { event.get_str("_tmp.default") == Some("default ") };
                if _cond {
                    event.set("citrix.default_class", json!(true))?;
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && (event.get_str("citrix.device_event_class_id") == Some("TCP")
                            || event.get_str("citrix.device_event_class_id") == Some("ACL"))
                };
                if _cond {
                    // Begin nested pipeline: "tcp_and_acl_feature"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$
                            // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$
                            // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start Time %{DATA:_tmp.start_time}(?: %{DATA:citrix_adc.log.start_time_timezone})? - End Time %{DATA:_tmp.end_time}(?: %{DATA:citrix_adc.log.end_time_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$
                            // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.original_destination.ip}:%{INT:citrix_adc.log.original_destination.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.translated_destination.ip}:%{INT:citrix_adc.log.translated_destination.port} - Start Time %{DATA:_tmp.start_time}(?: %{DATA:citrix_adc.log.start_time_timezone})? - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long} - Closure%{SPACE}Reason %{GREEDYDATA:citrix_adc.log.closure_reason}$
                            // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip} --> Destination %{IP:citrix_adc.log.destination.ip} - Protocol %{WORD:citrix_adc.log.protocol} - Type %{INT:citrix_adc.log.type} - Code %{INT:citrix_adc.log.code} - Time%{SPACE}Stamp %{DATA:citrix_adc.log.timestamp}%{SPACE}\\(ms\\) - Hitcount %{INT:citrix_adc.log.hit.count:int} - Hit%{SPACE}Rule %{GREEDYDATA:citrix_adc.log.hit.rule} - Action %{WORD:citrix_adc.log.action} - Data%{SPACE}$
                            // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$"
                                    ),
                                    cached_grok!(
                                        "^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$"
                                    ),
                                    cached_grok!(
                                        "^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start Time %{DATA:_tmp.start_time}(?: %{DATA:citrix_adc.log.start_time_timezone})? - End Time %{DATA:_tmp.end_time}(?: %{DATA:citrix_adc.log.end_time_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long}%{SPACE}$"
                                    ),
                                    cached_grok!(
                                        "^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.original_destination.ip}:%{INT:citrix_adc.log.original_destination.port} - NatIP %{IP:citrix_adc.log.nat.ip}:%{INT:citrix_adc.log.nat.port} - Destination %{IP:citrix_adc.log.translated_destination.ip}:%{INT:citrix_adc.log.translated_destination.port} - Start Time %{DATA:_tmp.start_time}(?: %{DATA:citrix_adc.log.start_time_timezone})? - Delink Time %{DATA:_tmp.delink_time}(?: %{DATA:citrix_adc.log.delink_timezone})? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send:long} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received:long} - Closure%{SPACE}Reason %{GREEDYDATA:citrix_adc.log.closure_reason}$"
                                    ),
                                    cached_grok!(
                                        "^Source %{IP:citrix_adc.log.source.ip} --> Destination %{IP:citrix_adc.log.destination.ip} - Protocol %{WORD:citrix_adc.log.protocol} - Type %{INT:citrix_adc.log.type} - Code %{INT:citrix_adc.log.code} - Time%{SPACE}Stamp %{DATA:citrix_adc.log.timestamp}%{SPACE}\\(ms\\) - Hitcount %{INT:citrix_adc.log.hit.count:int} - Hit%{SPACE}Rule %{GREEDYDATA:citrix_adc.log.hit.rule} - Action %{WORD:citrix_adc.log.action} - Data%{SPACE}$"
                                    ),
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
                    let _cond = {
                        event.has_value("citrix_adc.log.timestamp")
                            && event.get_str("citrix_adc.log.timestamp") != Some("")
                            && !event.has_value("_conf.custom_date_format")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("citrix_adc.log.timestamp")
                            {
                                match parse_date_out(
                                    &date_str,
                                    &[
                                        "UNIX_MS",
                                        "ISO8601",
                                        "MM/dd/yyyy:HH:mm:ss",
                                        "MMM d HH:mm:ss",
                                        "MMM  d HH:mm:ss",
                                        "MMM dd HH:mm:ss",
                                        "MMMM d HH:mm:ss",
                                        "MMMM  d HH:mm:ss",
                                        "MMMM dd HH:mm:ss",
                                        "yyyy MMM d HH:mm:ss",
                                        "yyyy MMM  d HH:mm:ss",
                                        "yyyy MMM dd HH:mm:ss",
                                        "yyyy MMMM d HH:mm:ss",
                                        "yyyy MMMM  d HH:mm:ss",
                                        "yyyy MMMM dd HH:mm:ss",
                                    ],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => {
                                        event.set("citrix_adc.log.timestamp", parsed)?
                                    }
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "citrix_adc.log.timestamp".into(),
                                            message: format!("unable to parse date [{date_str}]"),
                                        });
                                    }
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.total_bytes_received") {
                            if let Some(val) = event.get("citrix_adc.log.total_bytes_received") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.total_bytes_received".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.total_bytes_received", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_bytes_received_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.total_bytes_received")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.bytes", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.destination.ip")
                            && event.get_str("citrix_adc.log.destination.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.destination.ip") {
                                if let Some(val) = event.get("citrix_adc.log.destination.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.destination.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.destination.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_destination_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.destination.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.ip", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.original_destination.ip")
                            && event.get_str("citrix_adc.log.original_destination.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.original_destination.ip") {
                                if let Some(val) =
                                    event.get("citrix_adc.log.original_destination.ip")
                                {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.original_destination.ip"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event
                                        .set("citrix_adc.log.original_destination.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_original_destination_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.translated_destination.ip")
                            && event.get_str("citrix_adc.log.translated_destination.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.translated_destination.ip") {
                                if let Some(val) =
                                    event.get("citrix_adc.log.translated_destination.ip")
                                {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.translated_destination.ip"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "citrix_adc.log.translated_destination.ip",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_translated_destination_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.translated_destination.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.destination.port") {
                            if let Some(val) = event.get("citrix_adc.log.destination.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.destination.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_destination_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.original_destination.port") {
                            if let Some(val) = event.get("citrix_adc.log.original_destination.port")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.original_destination.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.original_destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_original_destination_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.translated_destination.port") {
                            if let Some(val) =
                                event.get("citrix_adc.log.translated_destination.port")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.translated_destination.port".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("citrix_adc.log.translated_destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_translated_destination_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.destination.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.translated_destination.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.action")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.action", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.closure_reason")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.reason", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.protocol")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("network.protocol", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.vserver.ip")
                            && event.get_str("citrix_adc.log.vserver.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.vserver.ip") {
                                if let Some(val) = event.get("citrix_adc.log.vserver.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.vserver.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.vserver.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_vserver_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.vserver.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("server.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.vserver.port") {
                            if let Some(val) = event.get("citrix_adc.log.vserver.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.vserver.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.vserver.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_vserver_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.vserver.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("server.port", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.total_bytes_send") {
                            if let Some(val) = event.get("citrix_adc.log.total_bytes_send") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.total_bytes_send".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.total_bytes_send", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_bytes_send_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.total_bytes_send")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.bytes", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.source.ip")
                            && event.get_str("citrix_adc.log.source.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.source.ip") {
                                if let Some(val) = event.get("citrix_adc.log.source.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.source.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.source.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_source_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.source.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.nat.ip")
                            && event.get_str("citrix_adc.log.nat.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.nat.ip") {
                                if let Some(val) = event.get("citrix_adc.log.nat.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.nat.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.nat.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event
                                .set("_ingest.on_failure_processor_tag", "convert_nat_ip_to_ip")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.nat.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.nat.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.nat.port") {
                            if let Some(val) = event.get("citrix_adc.log.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nat_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.nat.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.nat.port", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.source.port") {
                            if let Some(val) = event.get("citrix_adc.log.source.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.source.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.source.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_source_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.source.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.type") {
                            if let Some(val) = event.get("citrix_adc.log.type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.type".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.type", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_type_to_string")?;
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.code") {
                            if let Some(val) = event.get("citrix_adc.log.code") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.code".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.code", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_code_to_string")?;
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.code")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.code", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.hit.count") {
                            if let Some(val) = event.get("citrix_adc.log.hit.count") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.hit.count".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.hit.count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_hit_count_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    // End nested pipeline: "tcp_and_acl_feature"
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event.get_str("citrix.device_event_class_id") == Some("ALG")
                };
                if _cond {
                    // Begin nested pipeline: "alg_feature"
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
                                    cached_grok!(
                                        "^Errmsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.errmsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Call_ID%{SPACE}:%{SPACE}%{NOTSPACE:citrix_adc.log.call_id} - Transport%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.transport} - Source_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.source.ip} - Source_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.source.port} - Destination_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.destination.ip} - Destination_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.destination.port} - Natted_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.natted.ip} - Natted_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.natted.port} - Method%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.method} - Sequence_Number%{SPACE}:%{SPACE}%{INT:citrix_adc.log.sequence_number} - Register%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.register} - Content_Type%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.content_type} - Error%{SPACE}:%{SPACE}\\\"%{DATA:citrix_adc.log.error}\\\" - Error_line%{SPACE}:%{SPACE}\\\"%{DATA:citrix_adc.log.error_line}\\\" - Caller_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.caller.user_name} - Callee_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.callee.user_name} - Caller_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.caller.domain_name} - Callee_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.callee.domain_name} -$"
                                    ),
                                    cached_grok!(
                                        "^Errmsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.errmsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Call_ID%{SPACE}:%{SPACE}%{NOTSPACE:citrix_adc.log.call_id} - Transport%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.transport} - Source_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.source.ip} - Source_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.source.port} - Destination_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.destination.ip} - Destination_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.destination.port} - Natted_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.natted.ip} - Natted_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.natted.port} - Method%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.method} - Sequence_Number%{SPACE}:%{SPACE}%{INT:citrix_adc.log.sequence_number} - Register%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.register} - Content_Type%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.content_type} - Caller_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.caller.user_name} - Callee_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.callee.user_name} - Caller_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.caller.domain_name} - Callee_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.callee.domain_name}$"
                                    ),
                                    cached_grok!(
                                        "^Errmsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.errmsg}\\\" - Error_Code%{SPACE}:%{SPACE}%{INT:citrix_adc.log.error_code} - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group}$"
                                    ),
                                    cached_grok!(
                                        "^Infomsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.infomsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Session_ID%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.session_id} -$"
                                    ),
                                    cached_grok!(
                                        "^Infomsg%{SPACE}:%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.infomsg}\\\" - Group%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.group} - Call_ID%{SPACE}:%{SPACE}%{NOTSPACE:citrix_adc.log.call_id} - Transport%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.transport} - Source_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.source.ip} - Source_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.source.port} - Destination_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.destination.ip} - Destination_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.destination.port} - Natted_IP%{SPACE}:%{SPACE}%{IP:citrix_adc.log.natted.ip} - Natted_port%{SPACE}:%{SPACE}%{INT:citrix_adc.log.natted.port} - Method%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.method} - Sequence_Number%{SPACE}:%{SPACE}%{INT:citrix_adc.log.sequence_number} - Register%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.register} - Content_Type%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.content_type} - Caller_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.caller.user_name} - Callee_user_name%{SPACE}:%{SPACE}%{USER:citrix_adc.log.callee.user_name} - Caller_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.caller.domain_name} - Callee_domain_name%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.callee.domain_name} -$"
                                    ),
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
                    let _cond = {
                        event.has_value("citrix_adc.log.destination.ip")
                            && event.get_str("citrix_adc.log.destination.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.destination.ip") {
                                if let Some(val) = event.get("citrix_adc.log.destination.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.destination.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.destination.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_destination_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.destination.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.ip", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.natted.ip")
                            && event.get_str("citrix_adc.log.natted.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.natted.ip") {
                                if let Some(val) = event.get("citrix_adc.log.natted.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.natted.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.natted.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_natted_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.natted.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.nat.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.natted.port") {
                            if let Some(val) = event.get("citrix_adc.log.natted.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.natted.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.natted.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_natted_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.natted.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.nat.port", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.destination.port") {
                            if let Some(val) = event.get("citrix_adc.log.destination.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.destination.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_destination_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.destination.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.callee.domain_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.user.domain", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.callee.user_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.user.name", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.error_code") {
                            if let Some(val) = event.get("citrix_adc.log.error_code") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.error_code".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.error_code", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_error_code_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.error_code")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("error.code", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.errmsg")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("error.message", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.group")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("group.name", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.method")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("http.request.method", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.infomsg")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("message", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.transport")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("network.transport", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.source.ip")
                            && event.get_str("citrix_adc.log.source.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.source.ip") {
                                if let Some(val) = event.get("citrix_adc.log.source.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.source.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.source.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_source_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.source.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.source.port") {
                            if let Some(val) = event.get("citrix_adc.log.source.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.source.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.source.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_source_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.source.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.caller.domain_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.user.domain", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.caller.user_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.user.name", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.sequence_number") {
                            if let Some(val) = event.get("citrix_adc.log.sequence_number") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.sequence_number".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.sequence_number", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_sequence_number_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.session_id") {
                            if let Some(val) = event.get("citrix_adc.log.session_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.session_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.session_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_session_id_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    // End nested pipeline: "alg_feature"
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && ((event.get_str("citrix.device_event_class_id") == Some("SSLVPN")
                            && !(event
                                .get_str("citrix.name")
                                .is_some_and(|s| s.eq_ignore_ascii_case("MESSAGE"))))
                            || event.get_str("citrix.device_event_class_id") == Some("AAATM"))
                };
                if _cond {
                    // Begin nested pipeline: "sslvpn_and_aaatm_feature"
                    let _cond = { event.get_str("citrix.name") == Some("LOGIN") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                            // Grok pattern: ^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - SSLVPN_client_type %{DATA:citrix_adc.log.sslvpn_client_type} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                            // Grok pattern: ^(Logout handler : )?Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - (SSLVPN_client_type %{WORD:citrix_adc.log.sslvpn_client_type} - )?Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"
                                    ),
                                    cached_grok!(
                                        "^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - SSLVPN_client_type %{DATA:citrix_adc.log.sslvpn_client_type} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"
                                    ),
                                    cached_grok!(
                                        "^(Logout handler : )?Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - (SSLVPN_client_type %{WORD:citrix_adc.log.sslvpn_client_type} - )?Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("LOGOUT") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^(?:User %{DATA:citrix_adc.log.user})(?:%{SPACE}-%{SPACE})(?:Client_ip (%{IP:citrix_adc.log.client_ip})?)(?:%{SPACE}-%{SPACE})(?:Nat_ip (%{IP:citrix_adc.log.nat.ip}|%{DATA}))(?:%{SPACE}-%{SPACE})(?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port})(?:%{SPACE}-%{SPACE})(?:Start_time \"%{DATA:_tmp.start_time}\"(?:%{SPACE}-%{SPACE})End_time \"%{DATA:_tmp.end_time}\"(?:%{SPACE}-%{SPACE})Duration %{NOTSPACE:citrix_adc.log.duration})(?:%{SPACE}-%{SPACE})(?:Http_resources_accessed %{INT:citrix_adc.log.http_resources_accessed})(?:%{SPACE}-%{SPACE})(?:(?:NonHttp_services_accessed %{INT:citrix_adc.log.non_http_services_accessed}(?:%{SPACE}-%{SPACE}))?)(?:Total_TCP_connections %{INT:citrix_adc.log.total_tcp_connections}(?:(?:%{SPACE}-%{SPACE})Total_UDP_flows %{INT:citrix_adc.log.total_udp_flows})?)(?:%{SPACE}-%{SPACE})?(?:Total_policies_allowed %{INT:citrix_adc.log.total_policies_allowed}(?:%{SPACE}-%{SPACE})Total_policies_denied %{INT:citrix_adc.log.total_policies_denied})(?:%{SPACE}-%{SPACE})(?:Total_bytes_send %{INT:citrix_adc.log.total_bytes_send}(?:%{SPACE}-%{SPACE})Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received})(?:%{SPACE}-%{SPACE})(?:Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send}(?:%{SPACE}-%{SPACE})Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved})(?:%{SPACE}-%{SPACE})(?:Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send}%(?:%{SPACE}-%{SPACE})Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved}%)(?:%{SPACE}-%{SPACE})(?:LogoutMethod \"%{DATA:citrix_adc.log.logout_method}\"(?:%{SPACE}-%{SPACE})Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\") ?$
                            // Grok pattern: ^(Logout handler : )?Context %{DATA:citrix_adc.log.username}@%{IP}(?:%{SPACE}-%{SPACE})SessionId: %{NUMBER:citrix_adc.log.session_id}(?:%{SPACE}-%{SPACE})(?:User %{DATA:citrix_adc.log.user})(?:%{SPACE}-%{SPACE})(?:Client_ip (%{IP:citrix_adc.log.client_ip})?)(?:%{SPACE}-%{SPACE})(?:Nat_ip (%{IP:citrix_adc.log.nat.ip}|\\\\?\"%{DATA}\\\\?\"))(?:%{SPACE}-%{SPACE})(?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port})(?:%{SPACE}-%{SPACE})(?:Start_time \\\\?\"%{DATA:_tmp.start_time}\\\\?\"(?:%{SPACE}-%{SPACE})End_time \\\\?\"%{DATA:_tmp.end_time}\\\\?\"(?:%{SPACE}-%{SPACE})Duration %{NOTSPACE:citrix_adc.log.duration})(?:%{SPACE}-%{SPACE})(?:Http_resources_accessed %{INT:citrix_adc.log.http_resources_accessed})(?:%{SPACE}-%{SPACE})(?:(?:NonHttp_services_accessed %{INT:citrix_adc.log.non_http_services_accessed}(?:%{SPACE}-%{SPACE}))?)(?:Total_TCP_connections %{INT:citrix_adc.log.total_tcp_connections}(?:(?:%{SPACE}-%{SPACE})Total_UDP_flows %{INT:citrix_adc.log.total_udp_flows})?)(?:%{SPACE}-%{SPACE})(?:Total_policies_allowed %{INT:citrix_adc.log.total_policies_allowed}(?:%{SPACE}-%{SPACE})Total_policies_denied %{INT:citrix_adc.log.total_policies_denied})(?:%{SPACE}-%{SPACE})(?:Total_bytes_send %{INT:citrix_adc.log.total_bytes_send}(?:%{SPACE}-%{SPACE})Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received})(?:%{SPACE}-%{SPACE})(?:Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send}(?:%{SPACE}-%{SPACE})Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved})(?:%{SPACE}-%{SPACE})(?:Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send}%(?:%{SPACE}-%{SPACE})Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved}%)(?:%{SPACE}-%{SPACE})(?:LogoutMethod \\\\?\"%{DATA:citrix_adc.log.logout_method}\\\\?\"(?:%{SPACE}-%{SPACE})Group\\(s\\) \\\\?\"%{DATA:citrix_adc.log.groups}\\\\?\") ?$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^(?:User %{DATA:citrix_adc.log.user})(?:%{SPACE}-%{SPACE})(?:Client_ip (%{IP:citrix_adc.log.client_ip})?)(?:%{SPACE}-%{SPACE})(?:Nat_ip (%{IP:citrix_adc.log.nat.ip}|%{DATA}))(?:%{SPACE}-%{SPACE})(?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port})(?:%{SPACE}-%{SPACE})(?:Start_time \"%{DATA:_tmp.start_time}\"(?:%{SPACE}-%{SPACE})End_time \"%{DATA:_tmp.end_time}\"(?:%{SPACE}-%{SPACE})Duration %{NOTSPACE:citrix_adc.log.duration})(?:%{SPACE}-%{SPACE})(?:Http_resources_accessed %{INT:citrix_adc.log.http_resources_accessed})(?:%{SPACE}-%{SPACE})(?:(?:NonHttp_services_accessed %{INT:citrix_adc.log.non_http_services_accessed}(?:%{SPACE}-%{SPACE}))?)(?:Total_TCP_connections %{INT:citrix_adc.log.total_tcp_connections}(?:(?:%{SPACE}-%{SPACE})Total_UDP_flows %{INT:citrix_adc.log.total_udp_flows})?)(?:%{SPACE}-%{SPACE})?(?:Total_policies_allowed %{INT:citrix_adc.log.total_policies_allowed}(?:%{SPACE}-%{SPACE})Total_policies_denied %{INT:citrix_adc.log.total_policies_denied})(?:%{SPACE}-%{SPACE})(?:Total_bytes_send %{INT:citrix_adc.log.total_bytes_send}(?:%{SPACE}-%{SPACE})Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received})(?:%{SPACE}-%{SPACE})(?:Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send}(?:%{SPACE}-%{SPACE})Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved})(?:%{SPACE}-%{SPACE})(?:Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send}%(?:%{SPACE}-%{SPACE})Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved}%)(?:%{SPACE}-%{SPACE})(?:LogoutMethod \"%{DATA:citrix_adc.log.logout_method}\"(?:%{SPACE}-%{SPACE})Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\") ?$"
                                    ),
                                    cached_grok!(
                                        "^(Logout handler : )?Context %{DATA:citrix_adc.log.username}@%{IP}(?:%{SPACE}-%{SPACE})SessionId: %{NUMBER:citrix_adc.log.session_id}(?:%{SPACE}-%{SPACE})(?:User %{DATA:citrix_adc.log.user})(?:%{SPACE}-%{SPACE})(?:Client_ip (%{IP:citrix_adc.log.client_ip})?)(?:%{SPACE}-%{SPACE})(?:Nat_ip (%{IP:citrix_adc.log.nat.ip}|\\\\?\"%{DATA}\\\\?\"))(?:%{SPACE}-%{SPACE})(?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port})(?:%{SPACE}-%{SPACE})(?:Start_time \\\\?\"%{DATA:_tmp.start_time}\\\\?\"(?:%{SPACE}-%{SPACE})End_time \\\\?\"%{DATA:_tmp.end_time}\\\\?\"(?:%{SPACE}-%{SPACE})Duration %{NOTSPACE:citrix_adc.log.duration})(?:%{SPACE}-%{SPACE})(?:Http_resources_accessed %{INT:citrix_adc.log.http_resources_accessed})(?:%{SPACE}-%{SPACE})(?:(?:NonHttp_services_accessed %{INT:citrix_adc.log.non_http_services_accessed}(?:%{SPACE}-%{SPACE}))?)(?:Total_TCP_connections %{INT:citrix_adc.log.total_tcp_connections}(?:(?:%{SPACE}-%{SPACE})Total_UDP_flows %{INT:citrix_adc.log.total_udp_flows})?)(?:%{SPACE}-%{SPACE})(?:Total_policies_allowed %{INT:citrix_adc.log.total_policies_allowed}(?:%{SPACE}-%{SPACE})Total_policies_denied %{INT:citrix_adc.log.total_policies_denied})(?:%{SPACE}-%{SPACE})(?:Total_bytes_send %{INT:citrix_adc.log.total_bytes_send}(?:%{SPACE}-%{SPACE})Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received})(?:%{SPACE}-%{SPACE})(?:Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send}(?:%{SPACE}-%{SPACE})Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved})(?:%{SPACE}-%{SPACE})(?:Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send}%(?:%{SPACE}-%{SPACE})Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved}%)(?:%{SPACE}-%{SPACE})(?:LogoutMethod \\\\?\"%{DATA:citrix_adc.log.logout_method}\\\\?\"(?:%{SPACE}-%{SPACE})Group\\(s\\) \\\\?\"%{DATA:citrix_adc.log.groups}\\\\?\") ?$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("ICASTART") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - customername(?:%{SPACE}%{WORD:citrix_adc.log.customer_name})?(?:%{SPACE}-%{SPACE})username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - applicationName %{DATA:citrix_adc.log.application_name} - startTime \"%{DATA:_tmp.start_time}\" - connectionId %{WORD:citrix_adc.log.connection_id}%{SPACE}$
                            // Grok pattern: ^%{DATA} Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - customername(?:%{SPACE}%{WORD:citrix_adc.log.customer_name})?(?:%{SPACE}-%{SPACE})username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - applicationName %{DATA:citrix_adc.log.application_name} - startTime \"%{DATA:_tmp.start_time}\" - connectionId %{WORD:citrix_adc.log.connection_id}%{SPACE}$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - customername(?:%{SPACE}%{WORD:citrix_adc.log.customer_name})?(?:%{SPACE}-%{SPACE})username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - applicationName %{DATA:citrix_adc.log.application_name} - startTime \"%{DATA:_tmp.start_time}\" - connectionId %{WORD:citrix_adc.log.connection_id}%{SPACE}$"
                                    ),
                                    cached_grok!(
                                        "^%{DATA} Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - customername(?:%{SPACE}%{WORD:citrix_adc.log.customer_name})?(?:%{SPACE}-%{SPACE})username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - applicationName %{DATA:citrix_adc.log.application_name} - startTime \"%{DATA:_tmp.start_time}\" - connectionId %{WORD:citrix_adc.log.connection_id}%{SPACE}$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("ICAEND_CONNSTAT") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^%{DATA} ?Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - (SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - )?customername (%{WORD:citrix_adc.log.customer_name})? - username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - startTime \"%{DATA:_tmp.start_time}\" - endTime \"%{DATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} ? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received} - Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send} - Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - connectionId %{WORD:citrix_adc.log.connection_id} ?$
                            // Grok pattern: ^%{DATA} ?Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - (SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - )?customername (%{WORD:citrix_adc.log.customer_name})? ?- username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - startTime \"%{DATA:_tmp.start_time}\" - endTime \"%{DATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} ? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received} - Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send} - Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - connectionId %{WORD:citrix_adc.log.connection_id} - Total_bytes_wire_send %{INT:citrix_adc.log.total_bytes_wire_send} - Total_bytes_wire_recv %{INT:citrix_adc.log.total_bytes_wire_recieved} ?$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^%{DATA} ?Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - (SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - )?customername (%{WORD:citrix_adc.log.customer_name})? - username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - startTime \"%{DATA:_tmp.start_time}\" - endTime \"%{DATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} ? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received} - Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send} - Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - connectionId %{WORD:citrix_adc.log.connection_id} ?$"
                                    ),
                                    cached_grok!(
                                        "^%{DATA} ?Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - (SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - )?customername (%{WORD:citrix_adc.log.customer_name})? ?- username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - startTime \"%{DATA:_tmp.start_time}\" - endTime \"%{DATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} ? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received} - Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send} - Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - connectionId %{WORD:citrix_adc.log.connection_id} - Total_bytes_wire_send %{INT:citrix_adc.log.total_bytes_wire_send} - Total_bytes_wire_recv %{INT:citrix_adc.log.total_bytes_wire_recieved} ?$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("TCPCONNSTAT") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Total_compressedbytes_send %{NUMBER:citrix_adc.log.total_compressed_bytes_send:int} - Total_compressedbytes_recv %{NUMBER:citrix_adc.log.total_compressed_bytes_recieved:int} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\"$
                            // Grok pattern: ^Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Total_compressedbytes_send %{NUMBER:citrix_adc.log.total_compressed_bytes_send:int} - Total_compressedbytes_recv %{NUMBER:citrix_adc.log.total_compressed_bytes_recieved:int} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Total_compressedbytes_send %{NUMBER:citrix_adc.log.total_compressed_bytes_send:int} - Total_compressedbytes_recv %{NUMBER:citrix_adc.log.total_compressed_bytes_recieved:int} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\"$"
                                    ),
                                    cached_grok!(
                                        "^Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Total_compressedbytes_send %{NUMBER:citrix_adc.log.total_compressed_bytes_send:int} - Total_compressedbytes_recv %{NUMBER:citrix_adc.log.total_compressed_bytes_recieved:int} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("TCPCONN_TIMEDOUT") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Last_contact \"%{DATA:citrix_adc.log.last_contact}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                            // Grok pattern: ^Context %{DATA} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Last_contact \"%{DATA:citrix_adc.log.last_contact}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Last_contact \"%{DATA:citrix_adc.log.last_contact}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"
                                    ),
                                    cached_grok!(
                                        "^Context %{DATA} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Last_contact \"%{DATA:citrix_adc.log.last_contact}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("UDPFLOWSTAT") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"${DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                            // Grok pattern: ^(Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - )?(\\[%{DATA}\\] )?User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"${DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"${DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"
                                    ),
                                    cached_grok!(
                                        "^(Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - )?(\\[%{DATA}\\] )?User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"${DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("HTTPREQUEST") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^(?:(?:Context (?:(%{USERNAME:citrix_adc.log.username}|%{EMAILADDRESS:citrix_adc.log.username}|%{DATA:citrix_adc.log.username}))@%{IP:citrix_adc.log.client_ip} ?- SessionId: %{NUMBER:citrix_adc.log.session_id} ?-) )?(?:(?:\\[TECHSUPPORT\\]\\[ENUMERATION\\] )?)(?:%{HOSTNAME:citrix_adc.log.hostname} User (?:(%{USERNAME:citrix_adc.log.user}|%{EMAILADDRESS:citrix_adc.log.user}|%{DATA:citrix_adc.log.user})) ?: Group\\(s\\) %{DATA:citrix_adc.log.groups}) : (?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{NUMBER:citrix_adc.log.vserver.port}) - %{DATA:_tmp.timestamp}(?: %{WORD:citrix_adc.log.timezone})?(?: : (?:Message = )?SSO is %{WORD:citrix_adc.log.sso_status})? : (?:%{WORD:citrix_adc.log.method} %{DATA:citrix_adc.log.request.path} - -) ?$
                            // Grok pattern: ^(?:Context (?:(%{USERNAME:citrix_adc.log.username}|%{EMAILADDRESS:citrix_adc.log.username}|%{DATA:citrix_adc.log.username}))@%{IP:citrix_adc.log.client_ip} ?- SessionId: %{NUMBER:citrix_adc.log.session_id} ?-) (?:(?:\\[TECHSUPPORT\\]\\[ENUMERATION\\] )?)(?:%{HOSTNAME:citrix_adc.log.hostname} User (?:(%{USERNAME:citrix_adc.log.user}|%{EMAILADDRESS:citrix_adc.log.user}|%{DATA:citrix_adc.log.user})) ?: Group\\(s\\) %{DATA:citrix_adc.log.groups}) : (?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{NUMBER:citrix_adc.log.vserver.port}) - (?:%{DATA:_tmp.timestamp} %{DATA:citrix_adc.log.timezone}) (?:%{WORD:citrix_adc.log.method} %{DATA:citrix_adc.log.request.path} - -) ?$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^(?:(?:Context (?:(%{USERNAME:citrix_adc.log.username}|%{EMAILADDRESS:citrix_adc.log.username}|%{DATA:citrix_adc.log.username}))@%{IP:citrix_adc.log.client_ip} ?- SessionId: %{NUMBER:citrix_adc.log.session_id} ?-) )?(?:(?:\\[TECHSUPPORT\\]\\[ENUMERATION\\] )?)(?:%{HOSTNAME:citrix_adc.log.hostname} User (?:(%{USERNAME:citrix_adc.log.user}|%{EMAILADDRESS:citrix_adc.log.user}|%{DATA:citrix_adc.log.user})) ?: Group\\(s\\) %{DATA:citrix_adc.log.groups}) : (?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{NUMBER:citrix_adc.log.vserver.port}) - %{DATA:_tmp.timestamp}(?: %{WORD:citrix_adc.log.timezone})?(?: : (?:Message = )?SSO is %{WORD:citrix_adc.log.sso_status})? : (?:%{WORD:citrix_adc.log.method} %{DATA:citrix_adc.log.request.path} - -) ?$"
                                    ),
                                    cached_grok!(
                                        "^(?:Context (?:(%{USERNAME:citrix_adc.log.username}|%{EMAILADDRESS:citrix_adc.log.username}|%{DATA:citrix_adc.log.username}))@%{IP:citrix_adc.log.client_ip} ?- SessionId: %{NUMBER:citrix_adc.log.session_id} ?-) (?:(?:\\[TECHSUPPORT\\]\\[ENUMERATION\\] )?)(?:%{HOSTNAME:citrix_adc.log.hostname} User (?:(%{USERNAME:citrix_adc.log.user}|%{EMAILADDRESS:citrix_adc.log.user}|%{DATA:citrix_adc.log.user})) ?: Group\\(s\\) %{DATA:citrix_adc.log.groups}) : (?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{NUMBER:citrix_adc.log.vserver.port}) - (?:%{DATA:_tmp.timestamp} %{DATA:citrix_adc.log.timezone}) (?:%{WORD:citrix_adc.log.method} %{DATA:citrix_adc.log.request.path} - -) ?$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond =
                        { event.get_str("citrix.name") == Some("NONHTTP_RESOURCEACCESS_DENIED") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^- Denied_by_policy \"%{DATA:citrix_adc.log.policy_violation}\" ?$
                            if !cached_grok!(
                                "^- Denied_by_policy \"%{DATA:citrix_adc.log.policy_violation}\" ?$"
                            )
                            .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond =
                        { event.get_str("citrix.name") == Some("HTTP_RESOURCEACCESS_DENIED") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^- Denied_by_policy \"%{DATA:citrix_adc.log.policy_violation}\" ?$
                            if !cached_grok!(
                                "^- Denied_by_policy \"%{DATA:citrix_adc.log.policy_violation}\" ?$"
                            )
                            .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("LICLMT_REACHED") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - License_limit %{NUMBER:citrix_adc.log.license_limit:int} ?$
                            if !cached_grok!("^Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - License_limit %{NUMBER:citrix_adc.log.license_limit:int} ?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("CLISEC_CHECK") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^%{WORD:citrix_adc.log.alert_type} ?: %{WORD:citrix_adc.log.alert_level} - ClientIP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client_security_expression \"%{DATA:citrix_adc.log.client_security_expression}\" - ?$
                            // Grok pattern: ^CaseID: %{WORD} - Client IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client_security_expression \"%{GREEDYDATA:citrix_adc.log.client_security_expression}\" - Client_security_check (?:\"%{GREEDYDATA:citrix_adc.log.client_security_check_status}\"|%{WORD:citrix_adc.log.client_security_check_status})$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^%{WORD:citrix_adc.log.alert_type} ?: %{WORD:citrix_adc.log.alert_level} - ClientIP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client_security_expression \"%{DATA:citrix_adc.log.client_security_expression}\" - ?$"
                                    ),
                                    cached_grok!(
                                        "^CaseID: %{WORD} - Client IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client_security_expression \"%{GREEDYDATA:citrix_adc.log.client_security_expression}\" - Client_security_check (?:\"%{GREEDYDATA:citrix_adc.log.client_security_check_status}\"|%{WORD:citrix_adc.log.client_security_check_status})$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("STA_VALIDATE_RESP") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^Xdatalen %{NUMBER:citrix_adc.log.data_length:int} - Xdata %{GREEDYDATA:citrix_adc.log.data} ?$
                            if !cached_grok!("^Xdatalen %{NUMBER:citrix_adc.log.data_length:int} - Xdata %{GREEDYDATA:citrix_adc.log.data} ?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("REMOVE_SESSION_DEBUG") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^(Sessionid|Session id) %{NUMBER:citrix_adc.log.session_id:int} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver_ip %{IP:citrix_adc.log.vserver.ip} - Errmsg \"%{DATA:citrix_adc.log.errmsg}\" ?$
                            if !cached_grok!("^(Sessionid|Session id) %{NUMBER:citrix_adc.log.session_id:int} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver_ip %{IP:citrix_adc.log.vserver.ip} - Errmsg \"%{DATA:citrix_adc.log.errmsg}\" ?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("CLISEC_EXP_EVAL") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^User %{USER:citrix_adc.log.user}%{SPACE}:%{SPACE}- Client%{SPACE}IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client%{SPACE}security%{SPACE}check%{SPACE}Passed\\(%{NUMBER:citrix_adc.log.client_security_check_status:int}\\)%{SPACE}on%{SPACE}the%{SPACE}client%{SPACE}machine$
                            // Grok pattern: ^CaseID %{WORD}: - Client IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client security check %{GREEDYDATA} EXISTS %{GREEDYDATA:citrix_adc.log.client_security_check_status} on the client machine$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^User %{USER:citrix_adc.log.user}%{SPACE}:%{SPACE}- Client%{SPACE}IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client%{SPACE}security%{SPACE}check%{SPACE}Passed\\(%{NUMBER:citrix_adc.log.client_security_check_status:int}\\)%{SPACE}on%{SPACE}the%{SPACE}client%{SPACE}machine$"
                                    ),
                                    cached_grok!(
                                        "^CaseID %{WORD}: - Client IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client security check %{GREEDYDATA} EXISTS %{GREEDYDATA:citrix_adc.log.client_security_check_status} on the client machine$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond = { event.get_str("citrix.name") == Some("Message") };
                    if _cond {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^Logout handler : %{DATA}, for user <%{USERNAME|EMAILADDRESS:citrix_adc.log.username}>$
                            // Grok pattern: ^aaatm_handler successfully parsed assertion client ip is %{IP:citrix_adx.log.client_ip}, username is %{DATA:citrix_adc.log.user}$
                            // Grok pattern: %{DATA}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^Logout handler : %{DATA}, for user <%{USERNAME|EMAILADDRESS:citrix_adc.log.username}>$"
                                    ),
                                    cached_grok!(
                                        "^aaatm_handler successfully parsed assertion client ip is %{IP:citrix_adx.log.client_ip}, username is %{DATA:citrix_adc.log.user}$"
                                    ),
                                    cached_grok!("%{DATA}"),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.client_ip")
                            && event.get_str("citrix_adc.log.client_ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.client_ip") {
                                if let Some(val) = event.get("citrix_adc.log.client_ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.client_ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.client_ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_client_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.client_ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("client.ip", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.hostname")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("url.domain", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.total_bytes_received") {
                            if let Some(val) = event.get("citrix_adc.log.total_bytes_received") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.total_bytes_received".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.total_bytes_received", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_bytes_received_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.total_bytes_received")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.bytes", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.destination.ip")
                            && event.get_str("citrix_adc.log.destination.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.destination.ip") {
                                if let Some(val) = event.get("citrix_adc.log.destination.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.destination.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.destination.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_destination_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.destination.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.destination.port") {
                            if let Some(val) = event.get("citrix_adc.log.destination.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.destination.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_destination_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.destination.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.groups")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("group.name", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.vserver.ip")
                            && event.get_str("citrix_adc.log.vserver.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.vserver.ip") {
                                if let Some(val) = event.get("citrix_adc.log.vserver.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.vserver.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.vserver.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_vserver_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.vserver.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("server.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.vserver.port") {
                            if let Some(val) = event.get("citrix_adc.log.vserver.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.vserver.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.vserver.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_vserver_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.vserver.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("server.port", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.total_bytes_send") {
                            if let Some(val) = event.get("citrix_adc.log.total_bytes_send") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.total_bytes_send".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.total_bytes_send", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_bytes_send_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.total_bytes_send")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.bytes", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.source.ip")
                            && event.get_str("citrix_adc.log.source.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.source.ip") {
                                if let Some(val) = event.get("citrix_adc.log.source.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.source.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.source.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_source_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.source.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.nat.ip")
                            && event.get_str("citrix_adc.log.nat.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.nat.ip") {
                                if let Some(val) = event.get("citrix_adc.log.nat.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.nat.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.nat.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event
                                .set("_ingest.on_failure_processor_tag", "convert_nat_ip_to_ip")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.nat.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.nat.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.source.port") {
                            if let Some(val) = event.get("citrix_adc.log.source.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.source.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.source.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_source_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.source.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.user")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.ssl_relay.address")
                            && event.get_str("citrix_adc.log.ssl_relay.address") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.ssl_relay.address") {
                                if let Some(val) = event.get("citrix_adc.log.ssl_relay.address") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.ssl_relay.address".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.ssl_relay.address", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_ssl_relay_address_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.ssl_relay.port") {
                            if let Some(val) = event.get("citrix_adc.log.ssl_relay.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.ssl_relay.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.ssl_relay.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_ssl_relay_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.session_id") {
                            if let Some(val) = event.get("citrix_adc.log.session_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.session_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.session_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_session_id_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.total_tcp_connections") {
                            if let Some(val) = event.get("citrix_adc.log.total_tcp_connections") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.total_tcp_connections".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.total_tcp_connections", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_tcp_connections_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.total_udp_flows") {
                            if let Some(val) = event.get("citrix_adc.log.total_udp_flows") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.total_udp_flows".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.total_udp_flows", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_udp_flows_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.total_policies_allowed") {
                            if let Some(val) = event.get("citrix_adc.log.total_policies_allowed") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.total_policies_allowed".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.total_policies_allowed", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_policies_allowed_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.total_bytes_wire_send") {
                            if let Some(val) = event.get("citrix_adc.log.total_bytes_wire_send") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.total_bytes_wire_send".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.total_bytes_wire_send", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_bytes_wire_send_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.total_bytes_wire_recieved") {
                            if let Some(val) = event.get("citrix_adc.log.total_bytes_wire_recieved")
                            {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.total_bytes_wire_recieved".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.total_bytes_wire_recieved", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_bytes_wire_recieved_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.total_policies_denied") {
                            if let Some(val) = event.get("citrix_adc.log.total_policies_denied") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.total_policies_denied".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.total_policies_denied", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_policies_denied_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.total_compressed_bytes_send") {
                            if let Some(val) =
                                event.get("citrix_adc.log.total_compressed_bytes_send")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.total_compressed_bytes_send".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("citrix_adc.log.total_compressed_bytes_send", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_compressed_bytes_send_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.total_compressed_bytes_recieved") {
                            if let Some(val) =
                                event.get("citrix_adc.log.total_compressed_bytes_recieved")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.total_compressed_bytes_recieved"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "citrix_adc.log.total_compressed_bytes_recieved",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_compressed_bytes_recieved_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.compression_ratio_send") {
                            if let Some(val) = event.get("citrix_adc.log.compression_ratio_send") {
                                let converted =
                                    convert_value(val, "double").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.compression_ratio_send".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.compression_ratio_send", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_compression_ratio_send_to_double",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.compression_ratio_recieved") {
                            if let Some(val) =
                                event.get("citrix_adc.log.compression_ratio_recieved")
                            {
                                let converted =
                                    convert_value(val, "double").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.compression_ratio_recieved"
                                                .into(),
                                            message,
                                        }
                                    })?;
                                event
                                    .set("citrix_adc.log.compression_ratio_recieved", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_compression_ratio_recieved_to_double",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.license_limit") {
                            if let Some(val) = event.get("citrix_adc.log.license_limit") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.license_limit".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.license_limit", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_license_limit_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.client_security_check_status") {
                            if let Some(val) =
                                event.get("citrix_adc.log.client_security_check_status")
                            {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.client_security_check_status"
                                                .into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "citrix_adc.log.client_security_check_status",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_security_check_status_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.data_length") {
                            if let Some(val) = event.get("citrix_adc.log.data_length") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.data_length".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.data_length", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_data_length_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.domain_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.domain", v)?;
                    }
                    // End nested pipeline: "sslvpn_and_aaatm_feature"
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event.get_str("citrix.device_event_class_id") == Some("CI")
                };
                if _cond {
                    // Begin nested pipeline: "ci_feature"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Domain %{DATA:citrix_adc.log.domain} - Content-Type %{DATA:citrix_adc.log.content_type} - ICAP%{SPACE}Server %{IP:citrix_adc.log.icap_server.ip}:%{INT:citrix_adc.log.icap_server.port} - Mode %{WORD:citrix_adc.log.mode} - Service %{WORD:citrix_adc.log.service} - Response %{INT:citrix_adc.log.response.code} - Action %{WORD:citrix_adc.log.action}$
                            // Grok pattern: ^ID %{NUMBER:citrix_adc.log.id} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} Protocol %{WORD:citrix_adc.log.protocol} - URL %{URI:citrix_adc.log.url} - Domain %{DATA:citrix_adc.log.domain} - Service %{DATA:citrix_adc.log.service} - %{DATA}%{SPACE}%{DATA} - Action %{WORD:citrix_adc.log.action} - Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.bytes.sent} - Bytes%{SPACE}Received %{NUMBER:citrix_adc.log.bytes.received} - Origin%{SPACE}Server %{IP:citrix_adc.log.icap_server.ip}:%{INT:citrix_adc.log.icap_server.port}$
                            // Grok pattern: ^ID %{NUMBER:citrix_adc.log.id} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} Protocol %{WORD:citrix_adc.log.protocol} - URL %{URI:citrix_adc.log.url} - Domain %{DATA:citrix_adc.log.domain} - Service %{DATA:citrix_adc.log.service} - %{DATA}%{SPACE}%{DATA} - Action %{WORD:citrix_adc.log.action} - Request%{SPACE}Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.request.bytes_sent} - Response%{SPACE}Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.response.bytes_sent} - Origin%{SPACE}Server %{IP:citrix_adc.log.origin_server.ip}:%{INT:citrix_adc.log.origin_server.port}$
                            // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Domain %{DATA:citrix_adc.log.domain} - Content-Type %{DATA:citrix_adc.log.content_type} - ICAP%{SPACE}Server %{IP:citrix_adc.log.icap_server.ip}:%{INT:citrix_adc.log.icap_server.port} - Mode %{WORD:citrix_adc.log.mode} - Service %{WORD:citrix_adc.log.service} - Response %{INT:citrix_adc.log.response.code} - Action %{WORD:citrix_adc.log.action}$"
                                    ),
                                    cached_grok!(
                                        "^ID %{NUMBER:citrix_adc.log.id} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} Protocol %{WORD:citrix_adc.log.protocol} - URL %{URI:citrix_adc.log.url} - Domain %{DATA:citrix_adc.log.domain} - Service %{DATA:citrix_adc.log.service} - %{DATA}%{SPACE}%{DATA} - Action %{WORD:citrix_adc.log.action} - Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.bytes.sent} - Bytes%{SPACE}Received %{NUMBER:citrix_adc.log.bytes.received} - Origin%{SPACE}Server %{IP:citrix_adc.log.icap_server.ip}:%{INT:citrix_adc.log.icap_server.port}$"
                                    ),
                                    cached_grok!(
                                        "^ID %{NUMBER:citrix_adc.log.id} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} Protocol %{WORD:citrix_adc.log.protocol} - URL %{URI:citrix_adc.log.url} - Domain %{DATA:citrix_adc.log.domain} - Service %{DATA:citrix_adc.log.service} - %{DATA}%{SPACE}%{DATA} - Action %{WORD:citrix_adc.log.action} - Request%{SPACE}Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.request.bytes_sent} - Response%{SPACE}Bytes%{SPACE}Sent %{NUMBER:citrix_adc.log.response.bytes_sent} - Origin%{SPACE}Server %{IP:citrix_adc.log.origin_server.ip}:%{INT:citrix_adc.log.origin_server.port}$"
                                    ),
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
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.bytes.received") {
                            if let Some(val) = event.get("citrix_adc.log.bytes.received") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.bytes.received".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.bytes.received", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bytes_received_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.bytes.received")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.bytes", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.destination.ip")
                            && event.get_str("citrix_adc.log.destination.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.destination.ip") {
                                if let Some(val) = event.get("citrix_adc.log.destination.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.destination.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.destination.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_destination_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.destination.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.destination.port") {
                            if let Some(val) = event.get("citrix_adc.log.destination.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.destination.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_destination_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.destination.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.action")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.action", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.response.code") {
                            if let Some(val) = event.get("citrix_adc.log.response.code") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.response.code".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.response.code", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_response_code_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.response.code")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("http.response.status_code", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.protocol")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("network.protocol", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.icap_server.ip")
                            && event.get_str("citrix_adc.log.icap_server.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.icap_server.ip") {
                                if let Some(val) = event.get("citrix_adc.log.icap_server.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.icap_server.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.icap_server.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_icap_server_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.icap_server.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("server.ip", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.origin_server.ip")
                            && event.get_str("citrix_adc.log.origin_server.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.origin_server.ip") {
                                if let Some(val) = event.get("citrix_adc.log.origin_server.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.origin_server.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.origin_server.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_origin_server_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.origin_server.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("server.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.icap_server.port") {
                            if let Some(val) = event.get("citrix_adc.log.icap_server.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.icap_server.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.icap_server.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_icap_server_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.icap_server.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("server.port", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.origin_server.port") {
                            if let Some(val) = event.get("citrix_adc.log.origin_server.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.origin_server.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.origin_server.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_origin_server_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.origin_server.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("server.port", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.bytes.sent") {
                            if let Some(val) = event.get("citrix_adc.log.bytes.sent") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.bytes.sent".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.bytes.sent", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bytes_sent_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.bytes.sent")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.bytes", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.source.ip")
                            && event.get_str("citrix_adc.log.source.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.source.ip") {
                                if let Some(val) = event.get("citrix_adc.log.source.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.source.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.source.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_source_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.source.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.source.port") {
                            if let Some(val) = event.get("citrix_adc.log.source.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.source.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.source.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_source_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.source.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.url")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("url.original", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.domain")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.domain", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.request.bytes_sent") {
                            if let Some(val) = event.get("citrix_adc.log.request.bytes_sent") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.request.bytes_sent".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.request.bytes_sent", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_request_bytes_sent_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.response.bytes_sent") {
                            if let Some(val) = event.get("citrix_adc.log.response.bytes_sent") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.response.bytes_sent".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.response.bytes_sent", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_response_bytes_sent_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.id") {
                            if let Some(val) = event.get("citrix_adc.log.id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_id_to_string")?;
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
                                    .get("_ingest.on_failure_pipeline")
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
                    // End nested pipeline: "ci_feature"
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event.get_str("citrix.device_event_class_id") == Some("SSLLOG")
                };
                if _cond {
                    // Begin nested pipeline: "ssllog_feature"
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
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^Backend%{SPACE}SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - Server%{SPACE}IP %{IP:citrix_adc.log.server.ip} - Server%{SPACE}Port %{NUMBER:citrix_adc.log.server.port:int} - Protocol%{SPACE}Version %{DATA:citrix_adc.log.protocol_version} - Cipher%{SPACE}Suite \\\"%{DATA:citrix_adc.log.cipher_suite}\\\" - Session %{DATA:citrix_adc.log.session}(%{SPACE}- %{WORD:citrix_adc.log.server_authentication} -%{SPACE}SerialNumber \\\"%{DATA:citrix_adc.log.serial_number}\\\" - SignatureAlgorithm \\\"%{DATA:citrix_adc.log.signature_algorithm}\\\" - ValidFrom \\\"%{DATA:citrix_adc.log.valid_from}\\\" - ValidTo \\\"%{DATA:citrix_adc.log.valid_to}\\\" - HandshakeTime %{INT:citrix_adc.log.handshake_time} ms)?$"
                                    ),
                                    cached_grok!(
                                        "^Certificate%{SPACE}Key%{SPACE}Pair %{DATA:citrix_adc.log.certificate_key_pair} - Days%{SPACE}To%{SPACE}Expire %{NUMBER:citrix_adc.log.days_to_expire:int}$"
                                    ),
                                    cached_grok!(
                                        "^SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - Issuer%{SPACE}Name \\\"%{GREEDYDATA:citrix_adc.log.issuer_name}\\\"$"
                                    ),
                                    cached_grok!(
                                        "^SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - Subject%{SPACE}Name \\\"%{GREEDYDATA:citrix_adc.log.subject_name}\\\"$"
                                    ),
                                    cached_grok!(
                                        "^crl_name %{DATA:citrix_adc.log.crl_name} - server_ip %{IP:citrix_adc.log.server.ip} - server_port %{NUMBER:citrix_adc.log.server.port:int} - method %{WORD:citrix_adc.log.method} - ldapscope %{WORD:citrix_adc.log.ldap_scope}$"
                                    ),
                                    cached_grok!(
                                        "^Domainname %{DATA:citrix_adc.log.domain_name} Ipaddress %{IP:citrix_adc.log.ip_address}$"
                                    ),
                                    cached_grok!(
                                        "^SPCBId %{NUMBER:citrix_adc.log.spcb_id:int} - ClientIP %{IP:citrix_adc.log.client_ip} - ClientPort %{NUMBER:citrix_adc.log.client_port} - VserverServiceIP %{IP:citrix_adc.log.vserver.ip} - VserverServicePort %{NUMBER:citrix_adc.log.vserver.port} - ClientVersion %{DATA:citrix_adc.log.client_version} - CipherSuite \\\"%{GREEDYDATA:citrix_adc.log.cipher_suite}\\\"( - )?Session %{WORD:citrix_adc.log.session}(%{SPACE}- HandshakeTime %{INT:citrix_adc.log.handshake_time} ms)?( - Reason \\\"%{GREEDYDATA:citrix_adc.log.reason}\\\")?$"
                                    ),
                                    cached_grok!("^%{GREEDYDATA:citrix_adc.log.message}$"),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                    let _cond = {
                        event.has_value("citrix_adc.log.valid_from")
                            && event.get_str("citrix_adc.log.valid_from") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("citrix_adc.log.valid_from")
                            {
                                match parse_date_out(
                                    &date_str,
                                    &["MMM dd HH:mm:ss yyyy z", "MMM  d HH:mm:ss yyyy z"],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => {
                                        event.set("citrix_adc.log.valid_from", parsed)?
                                    }
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
                    let _cond = {
                        event.has_value("citrix_adc.log.valid_to")
                            && event.get_str("citrix_adc.log.valid_to") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("citrix_adc.log.valid_to") {
                                match parse_date_out(
                                    &date_str,
                                    &["MMM dd HH:mm:ss yyyy z", "MMM  d HH:mm:ss yyyy z"],
                                    None,
                                    None,
                                ) {
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
                    if let Some(v) = event
                        .get("citrix_adc.log.method")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("http.request.method", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.server.ip")
                            && event.get_str("citrix_adc.log.server.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.server.ip") {
                                if let Some(val) = event.get("citrix_adc.log.server.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.server.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.server.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_server_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.server.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.server.port") {
                            if let Some(val) = event.get("citrix_adc.log.server.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.server.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.server.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_server_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.server.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.domain_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.domain", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.spcb_id") {
                            if let Some(val) = event.get("citrix_adc.log.spcb_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.spcb_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.spcb_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_spcb_id_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.days_to_expire") {
                            if let Some(val) = event.get("citrix_adc.log.days_to_expire") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.days_to_expire".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.days_to_expire", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_days_to_expire_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    let _cond = {
                        event.has_value("citrix_adc.log.ip_address")
                            && event.get_str("citrix_adc.log.ip_address") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.ip_address") {
                                if let Some(val) = event.get("citrix_adc.log.ip_address") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.ip_address".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.ip_address", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_ip_address_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.client_ip")
                            && event.get_str("citrix_adc.log.client_ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.client_ip") {
                                if let Some(val) = event.get("citrix_adc.log.client_ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.client_ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.client_ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_client_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.client_ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("client.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.client_port") {
                            if let Some(val) = event.get("citrix_adc.log.client_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.client_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.client_port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.client_port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("client.port", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.vserver.ip")
                            && event.get_str("citrix_adc.log.vserver.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.vserver.ip") {
                                if let Some(val) = event.get("citrix_adc.log.vserver.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.vserver.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.vserver.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_vserver_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.vserver.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("server.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.vserver.port") {
                            if let Some(val) = event.get("citrix_adc.log.vserver.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.vserver.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.vserver.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_vserver_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.vserver.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("server.port", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.handshake_time") {
                            if let Some(val) = event.get("citrix_adc.log.handshake_time") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.handshake_time".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.handshake_time", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_handshake_time_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.cipher_suite")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("tls.cipher", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.issuer_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("tls.server.issuer", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.subject_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("tls.server.subject", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.protocol_version")
                            && event.get_str("citrix_adc.log.protocol_version") != Some("")
                    };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(input) = event.get_string("citrix_adc.log.protocol_version")
                            {
                                // Grok pattern: ^%{DATA:tls.version_protocol}v%{DATA:tls.version}$
                                if !cached_grok!(
                                    "^%{DATA:tls.version_protocol}v%{DATA:tls.version}$"
                                )
                                .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                            Ok(())
                        })();
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.client_version")
                            && event.get_str("citrix_adc.log.client_version") != Some("")
                    };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(input) = event.get_string("citrix_adc.log.client_version") {
                                // Grok pattern: ^%{DATA:tls.version_protocol}v%{DATA:tls.version}$
                                if !cached_grok!(
                                    "^%{DATA:tls.version_protocol}v%{DATA:tls.version}$"
                                )
                                .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                            Ok(())
                        })();
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.reason")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.reason", v)?;
                    }
                    // End nested pipeline: "ssllog_feature"
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event.get_str("citrix.device_event_class_id") == Some("TRANSFORM")
                };
                if _cond {
                    // Begin nested pipeline: "transform_feature"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^Client %{IP:citrix_adc.log.client_ip} - Profile %{DATA:citrix_adc.log.profile} - Action %{DATA:citrix_adc.log.action} - Value %{GREEDYDATA:citrix_adc.log.value}$
                            // Grok pattern: ^Client %{IP:citrix_adc.log.client_ip} - Profile %{DATA:citrix_adc.log.profile} - Action %{DATA:citrix_adc.log.action} - PCRE%{SPACE}error%{SPACE}code %{INT:citrix_adc.log.pcre_error_code}$
                            // Grok pattern: ^Client %{IP:citrix_adc.log.client_ip} - Profile %{DATA:citrix_adc.log.profile} - Failed%{SPACE}to%{SPACE}write%{SPACE}%{DATA:citrix_adc.log.header}%{SPACE}request%{SPACE}header$
                            // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^Client %{IP:citrix_adc.log.client_ip} - Profile %{DATA:citrix_adc.log.profile} - Action %{DATA:citrix_adc.log.action} - Value %{GREEDYDATA:citrix_adc.log.value}$"
                                    ),
                                    cached_grok!(
                                        "^Client %{IP:citrix_adc.log.client_ip} - Profile %{DATA:citrix_adc.log.profile} - Action %{DATA:citrix_adc.log.action} - PCRE%{SPACE}error%{SPACE}code %{INT:citrix_adc.log.pcre_error_code}$"
                                    ),
                                    cached_grok!(
                                        "^Client %{IP:citrix_adc.log.client_ip} - Profile %{DATA:citrix_adc.log.profile} - Failed%{SPACE}to%{SPACE}write%{SPACE}%{DATA:citrix_adc.log.header}%{SPACE}request%{SPACE}header$"
                                    ),
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
                    let _cond = {
                        event.has_value("citrix_adc.log.client_ip")
                            && event.get_str("citrix_adc.log.client_ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.client_ip") {
                                if let Some(val) = event.get("citrix_adc.log.client_ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.client_ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.client_ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_client_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.client_ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("client.ip", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.action")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.action", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.pcre_error_code") {
                            if let Some(val) = event.get("citrix_adc.log.pcre_error_code") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.pcre_error_code".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.pcre_error_code", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_pcre_error_code_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    // End nested pipeline: "transform_feature"
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event.get_str("citrix.device_event_class_id") == Some("ICA")
                };
                if _cond {
                    // Begin nested pipeline: "ica_feature"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - session_setup_time %{DATA:citrix_adc.log.session_setup_time} - client_ip %{IP:citrix_adc.log.client_ip} - client_type %{NUMBER:citrix_adc.log.client_type:int} - client_launcher %{NUMBER:citrix_adc.log.client_launcher:int} - client_version %{DATA:citrix_adc.log.client_version} - client_hostname %{DATA:citrix_adc.log.client_hostname} - domain_name %{DATA:citrix_adc.log.domain_name} - server_name %{DATA:citrix_adc.log.server.name} - connection_priority %{NUMBER:citrix_adc.log.connection_priority:int} - access_type %{NUMBER:citrix_adc.log.access_type:int} - status %{NUMBER:citrix_adc.log.status:int} - username %{USERNAME:citrix_adc.log.username}$
                            // Grok pattern: ^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - channel_update_begin %{DATA:citrix_adc.log.channel_update.begin} - channel_update_end %{DATA:citrix_adc.log.channel_update.end} - channel_id_1 %{NUMBER:citrix_adc.log.channel_id_1:int} - channel_id_1_val %{NUMBER:citrix_adc.log.channel_id_1_val:int} - channel_id_2 %{NUMBER:citrix_adc.log.channel_id_2:int} - channel_id_2_val %{NUMBER:citrix_adc.log.channel_id_2_val:int} - channel_id_3 %{NUMBER:citrix_adc.log.channel_id_3:int} - channel_id_3_val %{NUMBER:citrix_adc.log.channel_id_3_val:int} - channel_id_4 %{NUMBER:citrix_adc.log.channel_id_4:int} - channel_id_4_val %{NUMBER:citrix_adc.log.channel_id_4_val:int} - channel_id_5 %{NUMBER:citrix_adc.log.channel_id_5:int} - channel_id_5_val %{NUMBER:citrix_adc.log.channel_id_5_val:int}$
                            // Grok pattern: ^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - nsica_session_status %{NUMBER:citrix_adc.log.nsica_session.status:int} - nsica_session_client_ip %{IP:citrix_adc.log.nsica_session.client.ip} - nsica_session_client_port %{NUMBER:citrix_adc.log.nsica_session.client.port:int} - nsica_session_server_ip %{IP:citrix_adc.log.nsica_session.server.ip} - nsica_session_server_port %{NUMBER:citrix_adc.log.nsica_session.server.port:int} - nsica_session_reconnect_count %{NUMBER:citrix_adc.log.nsica_session.reconnect_count:int} - nsica_session_acr_count %{NUMBER:citrix_adc.log.nsica_session.acr_count:int} - connection_priority %{NUMBER:citrix_adc.log.connection_priority:int} - timestamp %{DATA:_tmp.timestamp} -$
                            // Grok pattern: ^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - nsica_status %{NUMBER:citrix_adc.log.nsica_status:int} - L7LatencyThresholdFactor %{NUMBER:citrix_adc.log.l7_latency.threshold_factor:int} - L7LatencyWaittime %{NUMBER:citrix_adc.log.l7_latency.waittime:int} - L7LatencyNotifyInterval %{NUMBER:citrix_adc.log.l7_latency.notify_interval:int} - L7LatencyMaxNotifyCount %{NUMBER:citrix_adc.log.l7_latency.max_notify_count:int} - L7ThresholdBreachAvgClientsideLatency %{NUMBER:citrix_adc.log.l7_threshold_breach.avg_clientside_latency:int} - L7ThresholdBreachMaxClientsideLatency %{NUMBER:citrix_adc.log.l7_threshold_breach.max_clientside_latency:int} - L7ThresholdBreachAvgServersideLatency %{NUMBER:citrix_adc.log.l7_threshold_breach.avg_serverside_latency:int} - L7ThresholdBreachMaxServersideLatency %{NUMBER:citrix_adc.log.l7_threshold_breach.max_serverside_latency:int} - MinL7Latency %{NUMBER:citrix_adc.log.min_l7_latency:int} -$
                            // Grok pattern: ^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - session_end_time %{DATA:citrix_adc.log.session_end_time}$
                            // Grok pattern: ^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - ica_rtt %{NUMBER:citrix_adc.log.ica_rtt:int} - clientside_rxbytes %{NUMBER:citrix_adc.log.clientside.rxbytes:int} - clientside_txbytes %{NUMBER:citrix_adc.log.clientside.txbytes:int} - clientside_packet_retransmits %{NUMBER:citrix_adc.log.clientside.packet_retransmits:int} - serverside_packet_retransmits %{NUMBER:citrix_adc.log.serverside.packet_retransmits:int} - clientside_rtt %{NUMBER:citrix_adc.log.clientside.rtt:int} - serverside_rtt %{NUMBER:citrix_adc.log.serverside.rtt:int} - clientside_jitter %{NUMBER:citrix_adc.log.clientside.jitter:int} - serverside_jitter %{NUMBER:citrix_adc.log.serverside.jitter:int}$
                            // Grok pattern: ^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - startup_duration %{NUMBER:citrix_adc.log.startup_duration:int} - launch_mechanism %{NUMBER:citrix_adc.log.launch_mechanism:int} - app_launch_time %{DATA:citrix_adc.log.app.launch_time} - app_process_id %{NUMBER:citrix_adc.log.app.process_id:int} - app_name %{DATA:citrix_adc.log.app.name} - module_path %{GREEDYDATA:citrix_adc.log.module_path}$
                            // Grok pattern: ^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - app_termination_type %{NUMBER:citrix_adc.log.app.termination_type:int} - app_process_id %{NUMBER:citrix_adc.log.app.process_id:int} - app_termination_time %{DATA:citrix_adc.log.app.termination_time}$
                            // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - session_setup_time %{DATA:citrix_adc.log.session_setup_time} - client_ip %{IP:citrix_adc.log.client_ip} - client_type %{NUMBER:citrix_adc.log.client_type:int} - client_launcher %{NUMBER:citrix_adc.log.client_launcher:int} - client_version %{DATA:citrix_adc.log.client_version} - client_hostname %{DATA:citrix_adc.log.client_hostname} - domain_name %{DATA:citrix_adc.log.domain_name} - server_name %{DATA:citrix_adc.log.server.name} - connection_priority %{NUMBER:citrix_adc.log.connection_priority:int} - access_type %{NUMBER:citrix_adc.log.access_type:int} - status %{NUMBER:citrix_adc.log.status:int} - username %{USERNAME:citrix_adc.log.username}$"
                                    ),
                                    cached_grok!(
                                        "^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - channel_update_begin %{DATA:citrix_adc.log.channel_update.begin} - channel_update_end %{DATA:citrix_adc.log.channel_update.end} - channel_id_1 %{NUMBER:citrix_adc.log.channel_id_1:int} - channel_id_1_val %{NUMBER:citrix_adc.log.channel_id_1_val:int} - channel_id_2 %{NUMBER:citrix_adc.log.channel_id_2:int} - channel_id_2_val %{NUMBER:citrix_adc.log.channel_id_2_val:int} - channel_id_3 %{NUMBER:citrix_adc.log.channel_id_3:int} - channel_id_3_val %{NUMBER:citrix_adc.log.channel_id_3_val:int} - channel_id_4 %{NUMBER:citrix_adc.log.channel_id_4:int} - channel_id_4_val %{NUMBER:citrix_adc.log.channel_id_4_val:int} - channel_id_5 %{NUMBER:citrix_adc.log.channel_id_5:int} - channel_id_5_val %{NUMBER:citrix_adc.log.channel_id_5_val:int}$"
                                    ),
                                    cached_grok!(
                                        "^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - nsica_session_status %{NUMBER:citrix_adc.log.nsica_session.status:int} - nsica_session_client_ip %{IP:citrix_adc.log.nsica_session.client.ip} - nsica_session_client_port %{NUMBER:citrix_adc.log.nsica_session.client.port:int} - nsica_session_server_ip %{IP:citrix_adc.log.nsica_session.server.ip} - nsica_session_server_port %{NUMBER:citrix_adc.log.nsica_session.server.port:int} - nsica_session_reconnect_count %{NUMBER:citrix_adc.log.nsica_session.reconnect_count:int} - nsica_session_acr_count %{NUMBER:citrix_adc.log.nsica_session.acr_count:int} - connection_priority %{NUMBER:citrix_adc.log.connection_priority:int} - timestamp %{DATA:_tmp.timestamp} -$"
                                    ),
                                    cached_grok!(
                                        "^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - nsica_status %{NUMBER:citrix_adc.log.nsica_status:int} - L7LatencyThresholdFactor %{NUMBER:citrix_adc.log.l7_latency.threshold_factor:int} - L7LatencyWaittime %{NUMBER:citrix_adc.log.l7_latency.waittime:int} - L7LatencyNotifyInterval %{NUMBER:citrix_adc.log.l7_latency.notify_interval:int} - L7LatencyMaxNotifyCount %{NUMBER:citrix_adc.log.l7_latency.max_notify_count:int} - L7ThresholdBreachAvgClientsideLatency %{NUMBER:citrix_adc.log.l7_threshold_breach.avg_clientside_latency:int} - L7ThresholdBreachMaxClientsideLatency %{NUMBER:citrix_adc.log.l7_threshold_breach.max_clientside_latency:int} - L7ThresholdBreachAvgServersideLatency %{NUMBER:citrix_adc.log.l7_threshold_breach.avg_serverside_latency:int} - L7ThresholdBreachMaxServersideLatency %{NUMBER:citrix_adc.log.l7_threshold_breach.max_serverside_latency:int} - MinL7Latency %{NUMBER:citrix_adc.log.min_l7_latency:int} -$"
                                    ),
                                    cached_grok!(
                                        "^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - session_end_time %{DATA:citrix_adc.log.session_end_time}$"
                                    ),
                                    cached_grok!(
                                        "^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - ica_rtt %{NUMBER:citrix_adc.log.ica_rtt:int} - clientside_rxbytes %{NUMBER:citrix_adc.log.clientside.rxbytes:int} - clientside_txbytes %{NUMBER:citrix_adc.log.clientside.txbytes:int} - clientside_packet_retransmits %{NUMBER:citrix_adc.log.clientside.packet_retransmits:int} - serverside_packet_retransmits %{NUMBER:citrix_adc.log.serverside.packet_retransmits:int} - clientside_rtt %{NUMBER:citrix_adc.log.clientside.rtt:int} - serverside_rtt %{NUMBER:citrix_adc.log.serverside.rtt:int} - clientside_jitter %{NUMBER:citrix_adc.log.clientside.jitter:int} - serverside_jitter %{NUMBER:citrix_adc.log.serverside.jitter:int}$"
                                    ),
                                    cached_grok!(
                                        "^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - startup_duration %{NUMBER:citrix_adc.log.startup_duration:int} - launch_mechanism %{NUMBER:citrix_adc.log.launch_mechanism:int} - app_launch_time %{DATA:citrix_adc.log.app.launch_time} - app_process_id %{NUMBER:citrix_adc.log.app.process_id:int} - app_name %{DATA:citrix_adc.log.app.name} - module_path %{GREEDYDATA:citrix_adc.log.module_path}$"
                                    ),
                                    cached_grok!(
                                        "^session_guid %{WORD:citrix_adc.log.session_guid} - device_serial_number %{NUMBER:citrix_adc.log.device_serial_number:int} - client_cookie %{WORD:citrix_adc.log.client_cookie} - flags %{NUMBER:citrix_adc.log.flags:int} - app_termination_type %{NUMBER:citrix_adc.log.app.termination_type:int} - app_process_id %{NUMBER:citrix_adc.log.app.process_id:int} - app_termination_time %{DATA:citrix_adc.log.app.termination_time}$"
                                    ),
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
                    let _cond = {
                        event.has_value("citrix_adc.log.client_ip")
                            && event.get_str("citrix_adc.log.client_ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.client_ip") {
                                if let Some(val) = event.get("citrix_adc.log.client_ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.client_ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.client_ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_client_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.client_ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("client.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.clientside.rxbytes") {
                            if let Some(val) = event.get("citrix_adc.log.clientside.rxbytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.clientside.rxbytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.clientside.rxbytes", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_clientside_rxbytes_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.clientside.rxbytes")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.bytes", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.nsica_session.server.ip")
                            && event.get_str("citrix_adc.log.nsica_session.server.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.nsica_session.server.ip") {
                                if let Some(val) =
                                    event.get("citrix_adc.log.nsica_session.server.ip")
                                {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.nsica_session.server.ip"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event
                                        .set("citrix_adc.log.nsica_session.server.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_destination_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.nsica_session.server.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.nsica_session.server.port") {
                            if let Some(val) = event.get("citrix_adc.log.nsica_session.server.port")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.nsica_session.server.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.nsica_session.server.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nsica_session_server_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.nsica_session.server.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.clientside.txbytes") {
                            if let Some(val) = event.get("citrix_adc.log.clientside.txbytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.clientside.txbytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.clientside.txbytes", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_clientside_txbytes_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.clientside.txbytes")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.bytes", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.nsica_session.client.ip")
                            && event.get_str("citrix_adc.log.nsica_session.client.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.nsica_session.client.ip") {
                                if let Some(val) =
                                    event.get("citrix_adc.log.nsica_session.client.ip")
                                {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.nsica_session.client.ip"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event
                                        .set("citrix_adc.log.nsica_session.client.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_nsica_session_client_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.nsica_session.client.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.nsica_session.client.port") {
                            if let Some(val) = event.get("citrix_adc.log.nsica_session.client.port")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.nsica_session.client.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.nsica_session.client.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nsica_session_client_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.nsica_session.client.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.domain_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.domain", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.username")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.device_serial_number") {
                            if let Some(val) = event.get("citrix_adc.log.device_serial_number") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.device_serial_number".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.device_serial_number", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_device_serial_number_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.flags") {
                            if let Some(val) = event.get("citrix_adc.log.flags") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.flags".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.flags", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_flags_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.nsica_session.status") {
                            if let Some(val) = event.get("citrix_adc.log.nsica_session.status") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.nsica_session.status".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.nsica_session.status", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nsica_session_status_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.access_type") {
                            if let Some(val) = event.get("citrix_adc.log.access_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.access_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.access_type", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_access_type_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.app.termination_type") {
                            if let Some(val) = event.get("citrix_adc.log.app.termination_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.app.termination_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.app.termination_type", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_app_termination_type_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.client_launcher") {
                            if let Some(val) = event.get("citrix_adc.log.client_launcher") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.client_launcher".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.client_launcher", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_launcher_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.client_type") {
                            if let Some(val) = event.get("citrix_adc.log.client_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.client_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.client_type", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_type_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.client_version") {
                            if let Some(val) = event.get("citrix_adc.log.client_version") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.client_version".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.client_version", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_version_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.connection_priority") {
                            if let Some(val) = event.get("citrix_adc.log.connection_priority") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.connection_priority".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.connection_priority", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_connection_priority_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.launch_mechanism") {
                            if let Some(val) = event.get("citrix_adc.log.launch_mechanism") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.launch_mechanism".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.launch_mechanism", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_launch_mechanism_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.status") {
                            if let Some(val) = event.get("citrix_adc.log.status") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.status".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.status", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_status_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.channel_id_1") {
                            if let Some(val) = event.get("citrix_adc.log.channel_id_1") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.channel_id_1".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.channel_id_1", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_channel_id_1_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.channel_id_1_val") {
                            if let Some(val) = event.get("citrix_adc.log.channel_id_1_val") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.channel_id_1_val".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.channel_id_1_val", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_channel_id_1_val_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.channel_id_2") {
                            if let Some(val) = event.get("citrix_adc.log.channel_id_2") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.channel_id_2".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.channel_id_2", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_channel_id_2_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.channel_id_2_val") {
                            if let Some(val) = event.get("citrix_adc.log.channel_id_2_val") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.channel_id_2_val".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.channel_id_2_val", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_channel_id_2_val_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.channel_id_3") {
                            if let Some(val) = event.get("citrix_adc.log.channel_id_3") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.channel_id_3".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.channel_id_3", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_channel_id_3_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.channel_id_3_val") {
                            if let Some(val) = event.get("citrix_adc.log.channel_id_3_val") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.channel_id_3_val".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.channel_id_3_val", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_channel_id_3_val_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.channel_id_4") {
                            if let Some(val) = event.get("citrix_adc.log.channel_id_4") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.channel_id_4".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.channel_id_4", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_channel_id_4_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.channel_id_4_val") {
                            if let Some(val) = event.get("citrix_adc.log.channel_id_4_val") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.channel_id_4_val".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.channel_id_4_val", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_channel_id_4_val_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.channel_id_5") {
                            if let Some(val) = event.get("citrix_adc.log.channel_id_5") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.channel_id_5".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.channel_id_5", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_channel_id_5_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.channel_id_5_val") {
                            if let Some(val) = event.get("citrix_adc.log.channel_id_5_val") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.channel_id_5_val".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.channel_id_5_val", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_channel_id_5_val_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.nsica_session.reconnect_count") {
                            if let Some(val) =
                                event.get("citrix_adc.log.nsica_session.reconnect_count")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.nsica_session.reconnect_count".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "citrix_adc.log.nsica_session.reconnect_count",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nsica_session_reconnect_count_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.nsica_session.acr_count") {
                            if let Some(val) = event.get("citrix_adc.log.nsica_session.acr_count") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.nsica_session.acr_count".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.nsica_session.acr_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nsica_session_acr_count_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.nsica_status") {
                            if let Some(val) = event.get("citrix_adc.log.nsica_status") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.nsica_status".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.nsica_status", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nsica_status_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.l7_latency.max_notify_count") {
                            if let Some(val) =
                                event.get("citrix_adc.log.l7_latency.max_notify_count")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.l7_latency.max_notify_count".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("citrix_adc.log.l7_latency.max_notify_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_l7_latency_max_notify_count_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.l7_latency.notify_interval") {
                            if let Some(val) =
                                event.get("citrix_adc.log.l7_latency.notify_interval")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.l7_latency.notify_interval".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("citrix_adc.log.l7_latency.notify_interval", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_l7_latency_notify_interval_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.l7_latency.threshold_factor") {
                            if let Some(val) =
                                event.get("citrix_adc.log.l7_latency.threshold_factor")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.l7_latency.threshold_factor".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("citrix_adc.log.l7_latency.threshold_factor", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_l7_latency_threshold_factor_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    let _cond = {
                        event.has_value("citrix_adc.log.l7_latency.waittime")
                            && event.get_str("citrix_adc.log.l7_latency.waittime") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("citrix_adc.log.l7_latency.waittime")
                            {
                                match parse_date_out(&date_str, &["UNIX"], None, None) {
                                    Some(parsed) => {
                                        event.set("citrix_adc.log.l7_latency.waittime", parsed)?
                                    }
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "citrix_adc.log.l7_latency.waittime".into(),
                                            message: format!("unable to parse date [{date_str}]"),
                                        });
                                    }
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "date_l7_latency_waittime",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event
                            .has_value("citrix_adc.log.l7_threshold_breach.avg_clientside_latency")
                        {
                            if let Some(val) = event
                                .get("citrix_adc.log.l7_threshold_breach.avg_clientside_latency")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                path: "citrix_adc.log.l7_threshold_breach.avg_clientside_latency".into(),
                message,
                }
                                })?;
                                event.set(
                                    "citrix_adc.log.l7_threshold_breach.avg_clientside_latency",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_l7_threshold_breach_avg_clientside_latency_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event
                            .has_value("citrix_adc.log.l7_threshold_breach.avg_serverside_latency")
                        {
                            if let Some(val) = event
                                .get("citrix_adc.log.l7_threshold_breach.avg_serverside_latency")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                path: "citrix_adc.log.l7_threshold_breach.avg_serverside_latency".into(),
                message,
                }
                                })?;
                                event.set(
                                    "citrix_adc.log.l7_threshold_breach.avg_serverside_latency",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_l7_threshold_breach_avg_serverside_latency_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event
                            .has_value("citrix_adc.log.l7_threshold_breach.max_clientside_latency")
                        {
                            if let Some(val) = event
                                .get("citrix_adc.log.l7_threshold_breach.max_clientside_latency")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                path: "citrix_adc.log.l7_threshold_breach.max_clientside_latency".into(),
                message,
                }
                                })?;
                                event.set(
                                    "citrix_adc.log.l7_threshold_breach.max_clientside_latency",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_l7_threshold_breach_max_clientside_latency_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event
                            .has_value("citrix_adc.log.l7_threshold_breach.max_serverside_latency")
                        {
                            if let Some(val) = event
                                .get("citrix_adc.log.l7_threshold_breach.max_serverside_latency")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                path: "citrix_adc.log.l7_threshold_breach.max_serverside_latency".into(),
                message,
                }
                                })?;
                                event.set(
                                    "citrix_adc.log.l7_threshold_breach.max_serverside_latency",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_l7_threshold_breach_max_serverside_latency_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.min_l7_latency") {
                            if let Some(val) = event.get("citrix_adc.log.min_l7_latency") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.min_l7_latency".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.min_l7_latency", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_min_l7_latency_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.clientside.packet_retransmits") {
                            if let Some(val) =
                                event.get("citrix_adc.log.clientside.packet_retransmits")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.clientside.packet_retransmits".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "citrix_adc.log.clientside.packet_retransmits",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_clientside_packet_retransmits_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.serverside_packet_retransmits") {
                            if let Some(val) =
                                event.get("citrix_adc.log.serverside_packet_retransmits")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.serverside_packet_retransmits".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "citrix_adc.log.serverside_packet_retransmits",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_serverside_packet_retransmits_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.clientside.jitter") {
                            if let Some(val) = event.get("citrix_adc.log.clientside.jitter") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.clientside.jitter".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.clientside.jitter", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_clientside_jitter_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.serverside_jitter") {
                            if let Some(val) = event.get("citrix_adc.log.serverside_jitter") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.serverside_jitter".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.serverside_jitter", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_serverside_jitter_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.startup_duration") {
                            if let Some(val) = event.get("citrix_adc.log.startup_duration") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.startup_duration".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.startup_duration", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_startup_duration_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.app.process_id") {
                            if let Some(val) = event.get("citrix_adc.log.app.process_id") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.app.process_id".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.app.process_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_app_process_id_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.clientside.rtt") {
                            if let Some(val) = event.get("citrix_adc.log.clientside.rtt") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.clientside.rtt".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.clientside.rtt", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_clientside_rtt_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.serverside.rtt") {
                            if let Some(val) = event.get("citrix_adc.log.serverside.rtt") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.serverside.rtt".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.serverside.rtt", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_serverside_rtt_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.ica_rtt") {
                            if let Some(val) = event.get("citrix_adc.log.ica_rtt") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.ica_rtt".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.ica_rtt", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_ica_rtt_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.l7_latency.waittime") {
                            if let Some(val) = event.get("citrix_adc.log.l7_latency.waittime") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.l7_latency.waittime".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.l7_latency.waittime", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_l7_latency_waittime_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    let _cond = {
                        event.has_value("citrix_adc.log.client_version")
                            && event.get_str("citrix_adc.log.client_version") != Some("")
                    };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(input) = event.get_string("citrix_adc.log.client_version") {
                                // Grok pattern: ^%{DATA:tls.version_protocol}v%{DATA:tls.version}$
                                if !cached_grok!(
                                    "^%{DATA:tls.version_protocol}v%{DATA:tls.version}$"
                                )
                                .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                            Ok(())
                        })();
                    }
                    // End nested pipeline: "ica_feature"
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event.get_str("citrix.device_event_class_id") == Some("APPFW")
                };
                if _cond {
                    // Begin nested pipeline: "appfw_feature"
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
                                    cached_grok!(
                                        "^%{IP:source.ip:ip} %{NUMBER:citrix_adc.log.transaction_id}-%{DATA:citrix_adc.log.ppe} - %{NOTSPACE:citrix_adc.log.profile}(?: %{NOTSPACE:url.original})?(?: %{GREEDYDATA:citrix_adc.log.message})?$"
                                    ),
                                    cached_grok!(
                                        "^XML%{SPACE}Mismatched%{SPACE}content-type%{SPACE}in%{SPACE}HTTP%{SPACE}header%{SPACE}detected%{SPACE}=%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.content_type_mismatch}\\\"\\.$"
                                    ),
                                    cached_grok!(
                                        "^Disallow%{SPACE}Deny%{SPACE}URL%{SPACE}for%{SPACE}rule%{SPACE}pattern%{SPACE}=%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.url}\\\"\\.$"
                                    ),
                                    cached_grok!(
                                        "^Unknown%{SPACE}content-type%{SPACE}header%{SPACE}value%{SPACE}=%{SPACE}\\\"%{GREEDYDATA:citrix_adc.log.unknown_content_type}\\\"\\.$"
                                    ),
                                    cached_grok!(
                                        "^parsing%{SPACE}referer%{SPACE}header%{SPACE}\\'%{GREEDYDATA:citrix_adc.log.referer_header}\\'%{SPACE}failed$"
                                    ),
                                    cached_grok!(
                                        "^URL%{SPACE}length\\(%{NUMBER:citrix_adc.log.url_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.url_length:int}\\)\\.$"
                                    ),
                                    cached_grok!(
                                        "^Cookie%{SPACE}header%{SPACE}length\\(%{NUMBER:citrix_adc.log.cookie_header_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.cookie_header_length:int}\\)\\.$"
                                    ),
                                    cached_grok!(
                                        "^Header\\(Referer\\)%{SPACE}length\\(%{NUMBER:citrix_adc.log.header_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.header_length:int}\\)\\.$"
                                    ),
                                    cached_grok!(
                                        "^Query%{SPACE}string%{SPACE}length\\(%{NUMBER:citrix_adc.log.query_string_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.query_string_length:int}\\)\\.$"
                                    ),
                                    cached_grok!(
                                        "^Total%{SPACE}HTTP%{SPACE}header%{SPACE}length\\(%{NUMBER:citrix_adc.log.total_http_header_length:int}\\)%{SPACE}is%{SPACE}greater%{SPACE}than%{SPACE}maximum%{SPACE}allowed\\(%{NUMBER:citrix_adc.log.max_allowed.total_http_header_length:int}\\)\\.$"
                                    ),
                                    cached_grok!(
                                        "^Profile%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.profile}$"
                                    ),
                                    cached_grok!(
                                        "^Field%{SPACE}Type%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.field_type}$"
                                    ),
                                    cached_grok!(
                                        "^Field%{SPACE}Name%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.field_name}$"
                                    ),
                                    cached_grok!(
                                        "^Content%{SPACE}length%{SPACE}is%{SPACE}too%{SPACE}large\\(%{NUMBER:citrix_adc.log.content_length_bytes:long}%{SPACE}Bytes\\).%{SPACE}Memory%{SPACE}Allocation%{SPACE}failed.$"
                                    ),
                                    cached_grok!(
                                        "^Signature%{SPACE}id%{SPACE}%{NUMBER:citrix_adc.log.signature_id:int}%{SPACE}contains%{SPACE}no%{SPACE}fast%{SPACE}match%{SPACE}pattern$"
                                    ),
                                    cached_grok!(
                                        "^Appfw%{SPACE}maximum%{SPACE}session%{SPACE}Limit%{SPACE}reached%{SPACE}for%{SPACE}PEID%{SPACE}%{NUMBER:citrix_adc.log.peid:int}$"
                                    ),
                                    cached_grok!(
                                        "^APPFW%{SPACE}RFC%{SPACE}Profile:%{SPACE}%{GREEDYDATA:citrix_adc.log.appfw_rfc_profile}$"
                                    ),
                                    cached_grok!(
                                        "^New%{SPACE}signature%{SPACE}available%{SPACE}:%{SPACE}RuleID%{SPACE}=%{SPACE}%{NUMBER:citrix_adc.log.rule_id:int}$"
                                    ),
                                    cached_grok!(
                                        "^Learned%{SPACE}rule%{SPACE}will%{SPACE}be%{SPACE}auto-deployed%{SPACE}after%{SPACE}%{NUMBER:citrix_adc.log.auto_deploy_mins:int}mins.%{SPACE}ViolType%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.violation_type}.%{SPACE}Profile%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.profile}$"
                                    ),
                                    cached_grok!(
                                        "^Rest%{SPACE}Validation%{SPACE}relaxation%{SPACE}rule%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.rule}%{SPACE}hit%{SPACE}at%{SPACE}url%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.url}$"
                                    ),
                                    cached_grok!(
                                        "^gRPC%{SPACE}Validation%{SPACE}relaxation%{SPACE}rule%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.rule}%{SPACE}hit%{SPACE}at%{SPACE}url%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.url}$"
                                    ),
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
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("source.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.referer_header")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("http.request.referrer", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.rule_id") {
                            if let Some(val) = event.get("citrix_adc.log.rule_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.rule_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.rule_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_rule_id_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.rule_id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("rule.id", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.url")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("url.original", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.peid") {
                            if let Some(val) = event.get("citrix_adc.log.peid") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.peid".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.peid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_peid_to_string")?;
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.signature_id") {
                            if let Some(val) = event.get("citrix_adc.log.signature_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.signature_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.signature_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_signature_id_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.url_length") {
                            if let Some(val) = event.get("citrix_adc.log.url_length") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.url_length".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.url_length", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_url_length_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.max_allowed.url_length") {
                            if let Some(val) = event.get("citrix_adc.log.max_allowed.url_length") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.max_allowed.url_length".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.max_allowed.url_length", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_allowed_url_length_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.cookie_header_length") {
                            if let Some(val) = event.get("citrix_adc.log.cookie_header_length") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.cookie_header_length".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.cookie_header_length", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_cookie_header_length_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.max_allowed.cookie_header_length") {
                            if let Some(val) =
                                event.get("citrix_adc.log.max_allowed.cookie_header_length")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.max_allowed.cookie_header_length"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "citrix_adc.log.max_allowed.cookie_header_length",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_allowed_cookie_header_length_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.header_length") {
                            if let Some(val) = event.get("citrix_adc.log.header_length") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.header_length".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.header_length", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_header_length_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.max_allowed.header_length") {
                            if let Some(val) = event.get("citrix_adc.log.max_allowed.header_length")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.max_allowed.header_length".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.max_allowed.header_length", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_allowed_header_length_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.query_string_length") {
                            if let Some(val) = event.get("citrix_adc.log.query_string_length") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.query_string_length".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.query_string_length", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_query_string_length_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.max_allowed.query_string_length") {
                            if let Some(val) =
                                event.get("citrix_adc.log.max_allowed.query_string_length")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.max_allowed.query_string_length"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "citrix_adc.log.max_allowed.query_string_length",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_allowed_query_string_length_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.total_http_header_length") {
                            if let Some(val) = event.get("citrix_adc.log.total_http_header_length")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.total_http_header_length".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.total_http_header_length", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_total_http_header_length_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.max_allowed.total_http_header_length") {
                            if let Some(val) =
                                event.get("citrix_adc.log.max_allowed.total_http_header_length")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.max_allowed.total_http_header_length"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "citrix_adc.log.max_allowed.total_http_header_length",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_allowed_total_http_header_length_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.content_length_bytes") {
                            if let Some(val) = event.get("citrix_adc.log.content_length_bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.content_length_bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.content_length_bytes", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_content_length_bytes_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    // End nested pipeline: "appfw_feature"
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event.get_str("citrix.device_event_class_id") == Some("CVPN")
                };
                if _cond {
                    // Begin nested pipeline: "cvpn_feature"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^HTML_URL %{URI:citrix_adc.log.html_url}$
                            // Grok pattern: ^REWRITTEN_URL %{URI:citrix_adc.log.rewritten_url}$
                            // Grok pattern: ^MATCHED_URL %{URI:citrix_adc.log.matched_url}$
                            // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                            if !extract_first_match(
                                &[
                                    cached_grok!("^HTML_URL %{URI:citrix_adc.log.html_url}$"),
                                    cached_grok!(
                                        "^REWRITTEN_URL %{URI:citrix_adc.log.rewritten_url}$"
                                    ),
                                    cached_grok!("^MATCHED_URL %{URI:citrix_adc.log.matched_url}$"),
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
                    if let Some(v) = event
                        .get("citrix_adc.log.html_url")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("url.original", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.rewritten_url")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("url.original", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.matched_url")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("url.original", v)?;
                    }
                    // End nested pipeline: "cvpn_feature"
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event.get_str("citrix.device_event_class_id") == Some("BOT")
                };
                if _cond {
                    // Begin nested pipeline: "bot_feature"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^Bot%{SPACE}New%{SPACE}Signature%{SPACE}Available.%{SPACE}Newly%{SPACE}added%{SPACE}Rules%{SPACE}:%{SPACE}%{INT:citrix_adc.log.newly_added_rules}%{SPACE}Deleted%{SPACE}Rules%{SPACE}:%{SPACE}%{INT:citrix_adc.log.deleted_rules}$
                            // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^Bot%{SPACE}New%{SPACE}Signature%{SPACE}Available.%{SPACE}Newly%{SPACE}added%{SPACE}Rules%{SPACE}:%{SPACE}%{INT:citrix_adc.log.newly_added_rules}%{SPACE}Deleted%{SPACE}Rules%{SPACE}:%{SPACE}%{INT:citrix_adc.log.deleted_rules}$"
                                    ),
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
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.newly_added_rules") {
                            if let Some(val) = event.get("citrix_adc.log.newly_added_rules") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.newly_added_rules".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.newly_added_rules", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_newly_added_rules_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.deleted_rules") {
                            if let Some(val) = event.get("citrix_adc.log.deleted_rules") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.deleted_rules".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.deleted_rules", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_deleted_rules_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    // End nested pipeline: "bot_feature"
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && event.get_str("citrix.device_event_class_id") == Some("PITBOSS")
                };
                if _cond {
                    // Begin nested pipeline: "pitboss_feature"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^Adding pitboss watch on \\(%{INT:citrix_adc.log.watch_id}\\)$
                            // Grok pattern: ^Deleting watch on \\(%{INT:citrix_adc.log.watch_id}\\)$
                            // Grok pattern: ^proc \\(%{INT:citrix_adc.log.process.id}\\) \\(%{DATA:citrix_adc.log.process.name}\\) has had its maximum number of restarts \\(%{INT:citrix_adc.log.max_restarts}\\), rebooting the system$
                            // Grok pattern: ^Restarting process old pid \\(%{INT:citrix_adc.log.old_pid}\\) action \\(%{DATA:citrix_adc.log.action}\\)$
                            // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^Adding pitboss watch on \\(%{INT:citrix_adc.log.watch_id}\\)$"
                                    ),
                                    cached_grok!(
                                        "^Deleting watch on \\(%{INT:citrix_adc.log.watch_id}\\)$"
                                    ),
                                    cached_grok!(
                                        "^proc \\(%{INT:citrix_adc.log.process.id}\\) \\(%{DATA:citrix_adc.log.process.name}\\) has had its maximum number of restarts \\(%{INT:citrix_adc.log.max_restarts}\\), rebooting the system$"
                                    ),
                                    cached_grok!(
                                        "^Restarting process old pid \\(%{INT:citrix_adc.log.old_pid}\\) action \\(%{DATA:citrix_adc.log.action}\\)$"
                                    ),
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
                    if let Some(v) = event
                        .get("citrix_adc.log.action")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.action", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.process.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.name", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.process.id") {
                            if let Some(val) = event.get("citrix_adc.log.process.id") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.process.id".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.process.id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_process_id_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.process.id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.pid", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.max_restarts") {
                            if let Some(val) = event.get("citrix_adc.log.max_restarts") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.max_restarts".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.max_restarts", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_max_restarts_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.old_pid") {
                            if let Some(val) = event.get("citrix_adc.log.old_pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.old_pid".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.old_pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_old_pid_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                        if event.has_value("citrix_adc.log.watch_id") {
                            if let Some(val) = event.get("citrix_adc.log.watch_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.watch_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.watch_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_watch_id_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    // End nested pipeline: "pitboss_feature"
                }
                let _cond = {
                    event.has_value("citrix.device_event_class_id")
                        && (event.get_str("citrix.device_event_class_id") == Some("DNS")
                            || event.get_str("citrix.device_event_class_id") == Some("SSLI"))
                };
                if _cond {
                    // Begin nested pipeline: "dns_and_ssli_feature"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("citrix.extended.message") {
                            // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} User%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.user} - Domain%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.domain} - Category%{SPACE}:%{SPACE}%{INT:citrix_adc.log.category} Action%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.action} - Reason%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.reason}$
                            // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} User%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.user} - Domain%{SPACE}:%{SPACE}%{DATA:citrix_adc.log.domain} - Category%{SPACE}:%{SPACE}%{INT:citrix_adc.log.category} Action%{SPACE}:%{SPACE}%{WORD:citrix_adc.log.action} - Reason%{SPACE}:%{SPACE}%{GREEDYDATA:citrix_adc.log.reason}$"
                                    ),
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
                    let _cond = {
                        event.has_value("citrix_adc.log.destination.ip")
                            && event.get_str("citrix_adc.log.destination.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.destination.ip") {
                                if let Some(val) = event.get("citrix_adc.log.destination.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.destination.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.destination.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_destination_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.destination.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.destination.port") {
                            if let Some(val) = event.get("citrix_adc.log.destination.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.destination.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_destination_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.destination.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.action")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.action", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.reason")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.reason", v)?;
                    }
                    let _cond = {
                        event.has_value("citrix_adc.log.source.ip")
                            && event.get_str("citrix_adc.log.source.ip") != Some("")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("citrix_adc.log.source.ip") {
                                if let Some(val) = event.get("citrix_adc.log.source.ip") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "citrix_adc.log.source.ip".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("citrix_adc.log.source.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_source_ip_to_ip",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.source.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.source.port") {
                            if let Some(val) = event.get("citrix_adc.log.source.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.source.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.source.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_source_port_to_long",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    if let Some(v) = event
                        .get("citrix_adc.log.source.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.domain")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.domain", v)?;
                    }
                    if let Some(v) = event
                        .get("citrix_adc.log.user")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.category") {
                            if let Some(val) = event.get("citrix_adc.log.category") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "citrix_adc.log.category".into(),
                                            message,
                                        }
                                    })?;
                                event.set("citrix_adc.log.category", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_category_to_string",
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
                                    .get("_ingest.on_failure_pipeline")
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
                    // End nested pipeline: "dns_and_ssli_feature"
                }
                let _cond = {
                    event.has_value("citrix_adc.log.client_ip")
                        && event.get_str("citrix_adc.log.client_ip") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.client_ip") {
                            if let Some(val) = event.get("citrix_adc.log.client_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.client_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.client_ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_ip_to_ip",
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
                                    .get("_ingest.on_failure_pipeline")
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
                }
                let _cond = {
                    event.has_value("citrix_adc.log.status")
                        && event
                            .get_str("citrix_adc.log.status")
                            .is_some_and(|s| s.to_lowercase() == "success")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.has_value("citrix_adc.log.status")
                        && event
                            .get_str("citrix_adc.log.status")
                            .is_some_and(|s| s.to_lowercase() == "failure")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.has_value("citrix.name")
                        && event.get_str("citrix.name") == Some("LOGIN_FAILED")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                if let Some(v) = event
                    .get("citrix_adc.log.client_ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.ip", v)?;
                }
                let _cond = {
                    event.has_value("citrix_adc.log.destination.ip")
                        && event.get_str("citrix_adc.log.destination.ip") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.destination.ip") {
                            if let Some(val) = event.get("citrix_adc.log.destination.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.destination.ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.destination.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_destination_ip_to_ip",
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
                                    .get("_ingest.on_failure_pipeline")
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
                }
                if let Some(v) = event
                    .get("citrix_adc.log.destination.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.ip", v)?;
                }
                if let Some(v) = event
                    .get("citrix_adc.log.failure_reason")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.reason", v)?;
                }
                if let Some(v) = event
                    .get("citrix_adc.log.groups")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.name", v)?;
                }
                if let Some(v) = event
                    .get("citrix_adc.log.command")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.command_line", v)?;
                }
                let _cond = {
                    event.has_value("citrix_adc.log.remote_ip")
                        && event.get_str("citrix_adc.log.remote_ip") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.remote_ip") {
                            if let Some(val) = event.get("citrix_adc.log.remote_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.remote_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.remote_ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_remote_ip_ip_to_ip",
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
                                    .get("_ingest.on_failure_pipeline")
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
                }
                if let Some(v) = event
                    .get("citrix_adc.log.remote_ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
                }
                let _cond = {
                    event.has_value("citrix_adc.log.source.ip")
                        && event.get_str("citrix_adc.log.source.ip") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("citrix_adc.log.source.ip") {
                            if let Some(val) = event.get("citrix_adc.log.source.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "citrix_adc.log.source.ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("citrix_adc.log.source.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_source_ip_to_ip",
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
                                    .get("_ingest.on_failure_pipeline")
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
                }
                if let Some(v) = event
                    .get("citrix_adc.log.source.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
                }
                if let Some(v) = event
                    .get("citrix_adc.log.url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.original", v)?;
                }
                if let Some(v) = event
                    .get("citrix_adc.log.browser")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user_agent.original", v)?;
                }
                let _cond = {
                    event.has_value("user_agent.original")
                        && event.get_str("user_agent.original") != Some("")
                };
                if _cond {
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
                if let Some(v) = event
                    .get("citrix_adc.log.user")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                if let Some(v) = event
                    .get("citrix_adc.log.username")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                event.set("event.kind", json!("event"))?;
                let _cond = {
                    !event.has_value("event.outcome")
                        && event.get_str("event.category") == Some("authentication")
                        && event.get_str("citrix_adc.log.access") == Some("Allowed")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    !event.has_value("event.outcome")
                        && event.get_str("event.category") == Some("authentication")
                        && event.get_str("citrix_adc.log.access") != Some("Allowed")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                if let Some(v) = event
                    .get("citrix.host")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("observer.hostname", v)?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("citrix_adc.log.reputation") {
                        if let Some(val) = event.get("citrix_adc.log.reputation") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "citrix_adc.log.reputation".into(),
                                    message,
                                }
                            })?;
                            event.set("citrix_adc.log.reputation", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_reputation_to_long",
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
                                .get("_ingest.on_failure_pipeline")
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
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                if event.has_value("network.protocol") {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                if event.has_value("network.transport") {
                    map_strings(
                        event,
                        "network.transport",
                        "network.transport",
                        str::to_lowercase,
                    )?;
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
                let _cond = { event.has_value("destination.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("destination.nat.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.nat.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("source.nat.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.nat.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("server.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("citrix_adc.log.ip_address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("citrix_adc.log.ip_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("citrix_adc.log.original_destination.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("citrix_adc.log.original_destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("citrix_adc.log.ssl_relay.address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("citrix_adc.log.ssl_relay.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    !event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        }))
                };
                if _cond {
                    event.remove("citrix_adc.log.action");
                    event.remove("citrix_adc.log.browser");
                    event.remove("citrix_adc.log.bytes.received");
                    event.remove("citrix_adc.log.bytes.sent");
                    event.remove("citrix_adc.log.callee.domain_name");
                    event.remove("citrix_adc.log.callee.user_name");
                    event.remove("citrix_adc.log.caller.domain_name");
                    event.remove("citrix_adc.log.caller.user_name");
                    event.remove("citrix_adc.log.cipher_suite");
                    event.remove("citrix_adc.log.client_ip");
                    event.remove("citrix_adc.log.client_port");
                    event.remove("citrix_adc.log.clientside.rxbytes");
                    event.remove("citrix_adc.log.clientside.txbytes");
                    event.remove("citrix_adc.log.closure_reason");
                    event.remove("citrix_adc.log.code");
                    event.remove("citrix_adc.log.command");
                    event.remove("citrix_adc.log.delink_time");
                    event.remove("citrix_adc.log.destination.ip");
                    event.remove("citrix_adc.log.destination.port");
                    event.remove("citrix_adc.log.domain");
                    event.remove("citrix_adc.log.domain_name");
                    event.remove("citrix_adc.log.end_time");
                    event.remove("citrix_adc.log.errmsg");
                    event.remove("citrix_adc.log.error_code");
                    event.remove("citrix_adc.log.failure_reason");
                    event.remove("citrix_adc.log.group");
                    event.remove("citrix_adc.log.groups");
                    event.remove("citrix_adc.log.hostname");
                    event.remove("citrix_adc.log.html_url");
                    event.remove("citrix_adc.log.icap_server.ip");
                    event.remove("citrix_adc.log.icap_server.port");
                    event.remove("citrix_adc.log.infomsg");
                    event.remove("citrix_adc.log.issuer_name");
                    event.remove("citrix_adc.log.matched_url");
                    event.remove("citrix_adc.log.method");
                    event.remove("citrix_adc.log.nat.ip");
                    event.remove("citrix_adc.log.nat.port");
                    event.remove("citrix_adc.log.natted.ip");
                    event.remove("citrix_adc.log.natted.port");
                    event.remove("citrix_adc.log.nsica_session.client.ip");
                    event.remove("citrix_adc.log.nsica_session.client.port");
                    event.remove("citrix_adc.log.nsica_session.server.ip");
                    event.remove("citrix_adc.log.nsica_session.server.port");
                    event.remove("citrix_adc.log.origin_server.ip");
                    event.remove("citrix_adc.log.origin_server.port");
                    event.remove("citrix_adc.log.process.id");
                    event.remove("citrix_adc.log.process.name");
                    event.remove("citrix_adc.log.protocol");
                    event.remove("citrix_adc.log.reason");
                    event.remove("citrix_adc.log.referer_header");
                    event.remove("citrix_adc.log.remote_ip");
                    event.remove("citrix_adc.log.request.path");
                    event.remove("citrix_adc.log.response.code");
                    event.remove("citrix_adc.log.rewritten_url");
                    event.remove("citrix_adc.log.rule_id");
                    event.remove("citrix_adc.log.server.ip");
                    event.remove("citrix_adc.log.source.ip");
                    event.remove("citrix_adc.log.source.port");
                    event.remove("citrix_adc.log.start_time");
                    event.remove("citrix_adc.log.subject_name");
                    event.remove("citrix_adc.log.total_bytes_received");
                    event.remove("citrix_adc.log.total_bytes_send");
                    event.remove("citrix_adc.log.translated_destination.ip");
                    event.remove("citrix_adc.log.translated_destination.port");
                    event.remove("citrix_adc.log.transport");
                    event.remove("citrix_adc.log.url");
                    event.remove("citrix_adc.log.user");
                    event.remove("citrix_adc.log.username");
                    event.remove("citrix_adc.log.vserver.ip");
                    event.remove("citrix_adc.log.vserver.port");
                }
                // End nested pipeline: "native"
            }

            let _cond = {
                event.has_value("citrix.extended.message")
                    && event
                        .get_str("citrix.extended.message")
                        .is_some_and(|s| s.starts_with("CEF:"))
            };
            if _cond {
                if let Some(v) = event.get("citrix.extended.message").cloned() {
                    event.set("citrix.detail", v)?;
                }
            }

            let _cond = {
                event.has_value("citrix.detail")
                    && event
                        .get_str("citrix.detail")
                        .is_some_and(|s| s.starts_with("CEF:"))
            };
            if _cond {
                // Begin nested pipeline: "cef"
                event.set("citrix.cef_format", json!(true))?;
                if let Some(input) = event.get_string("citrix.detail") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("CEF:") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.cef_version", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.device_vendor", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.device_product", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.device_version", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.device_event_class_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("event.severity", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("citrix.extended.message", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "citrix.detail".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
                if event.has_value("citrix.extended.message") {
                    if let Some(kv_str) = event.get_string("citrix.extended.message") {
                        for pair in cached_regex!(" (?=[a-zA-Z][a-zA-Z0-9]*=)")
                            .split(&kv_str)
                            .into_iter()
                        {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "citrix.extended.message".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(event, &format!("citrix.extended_kv.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                let _cond = { event.has_value("citrix.extended_kv") };
                if _cond {
                    if event.remove("citrix.extended").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "citrix.extended".into(),
                        });
                    }
                }
                if event.has_value("citrix.extended_kv.src") {
                    if let Some(val) = event.get("citrix.extended_kv.src") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "citrix.extended_kv.src".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                event.remove("citrix.extended_kv.src");
                if event.has_value("citrix.extended_kv.spt") {
                    if let Some(val) = event.get("citrix.extended_kv.spt") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "citrix.extended_kv.spt".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                event.remove("citrix.extended_kv.spt");
                if event.has_value("citrix.extended_kv.method") {
                    event.rename("citrix.extended_kv.method", "http.request.method")?;
                }
                if event.has_value("citrix.extended_kv.request") {
                    event.rename("citrix.extended_kv.request", "url.original")?;
                }
                if event.has_value("citrix.extended_kv.act") {
                    event.rename("citrix.extended_kv.act", "event.action")?;
                }
                if event.has_value("citrix.extended_kv.msg") {
                    event.rename("citrix.extended_kv.msg", "message")?;
                }
                if event.has_value("citrix.extended_kv.cn1") {
                    event.rename("citrix.extended_kv.cn1", "event.id")?;
                }
                if event.has_value("citrix.extended_kv.cn2") {
                    event.rename("citrix.extended_kv.cn2", "http.request.id")?;
                }
                if event.has_value("citrix.extended_kv.cs1") {
                    event.rename("citrix.extended_kv.cs1", "citrix.profile_name")?;
                }
                if event.has_value("citrix.extended_kv.cs2") {
                    event.rename("citrix.extended_kv.cs2", "citrix.ppe_id")?;
                }
                if event.has_value("citrix.extended_kv.cs3") {
                    event.rename("citrix.extended_kv.cs3", "citrix.session_id")?;
                }
                if event.has_value("citrix.extended_kv.cs4") {
                    event.rename("citrix.extended_kv.cs4", "citrix.severity")?;
                }
                if event.has_value("citrix.extended_kv.cs5") {
                    event.rename("citrix.extended_kv.cs5", "citrix.event_year")?;
                }
                if event.has_value("citrix.extended_kv.cs6") {
                    event.rename(
                        "citrix.extended_kv.cs6",
                        "citrix.signature_violation_category",
                    )?;
                }
                if event.has_value("citrix.extended_kv") {
                    event.rename("citrix.extended_kv", "citrix.extended")?;
                }
                // End nested pipeline: "cef"
            }

            if event.has_value("event.severity") {
                if let Some(val) = event.get("event.severity") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "event.severity".into(),
                            message,
                        }
                    })?;
                    event.set("event.severity", converted)?;
                }
            }

            let _cond = { event.get_str("_tmp.tz") == Some("Z") };
            if _cond {
                event.set("_tmp.tz", json!("UTC"))?;
            }

            if let Some(v) = event
                .get("_conf.tz_offset")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("_tmp.tz") {
                    event.set("_tmp.tz", v)?;
                }
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                if let Some(v) = event.get("event.timezone").cloned() {
                    if !event.has("_tmp.tz") {
                        event.set("_tmp.tz", v)?;
                    }
                }
            }

            if !event.has("_tmp.tz") {
                event.set("_tmp.tz", json!("UTC"))?;
            }

            if let Some(v) = event.get("_tmp.tz").cloned() {
                event.set("event.timezone", v)?;
            }

            let _cond = { event.has_value("_tmp.timestamp8601") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.syslog_timestamp8601") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.syslog_timestamp8601".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_syslog_timestamp8601",
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
                                .get("_ingest.on_failure_pipeline")
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
            }

            let _cond = {
                event.has_value("_tmp.syslog_timestamp") && event.has_value("citrix.event_year")
            };
            if _cond {
                event.set(
                    "_tmp.syslog_timestamp",
                    json!(format!(
                        "{} {}",
                        event
                            .get("citrix.event_year")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_tmp.syslog_timestamp")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            event.remove("citrix.event_year");

            let _cond = { event.has_value("_tmp.syslog_timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.syslog_timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "MMM d HH:mm:ss",
                                "MMM  d HH:mm:ss",
                                "MMM dd HH:mm:ss",
                                "MMMM d HH:mm:ss",
                                "MMMM  d HH:mm:ss",
                                "MMMM dd HH:mm:ss",
                                "yyyy MMM d HH:mm:ss",
                                "yyyy MMM  d HH:mm:ss",
                                "yyyy MMM dd HH:mm:ss",
                                "yyyy MMMM d HH:mm:ss",
                                "yyyy MMMM  d HH:mm:ss",
                                "yyyy MMMM dd HH:mm:ss",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.syslog_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_syslog_timestamp")?;
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
                                .get("_ingest.on_failure_pipeline")
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
            }

            // Painless script
            // Source: params.fields.forEach(field -> {\n  if (!ctx._tmp?.containsKey(field) || !(ctx._tmp[field] instanceof String)) {\n    return true;\n  }\n\n  String val = ctx._tmp[field];\n  ctx._tmp[field] = val.trim();\n});
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"params.fields.forEach(field -> {\n  if (!ctx._tmp?.containsKey(field) || !(ctx._tmp[field] instanceof String)) {\n    return true;\n  }\n\n  String val = ctx._tmp[field];\n  ctx._tmp[field] = val.trim();\n});"#
                ),
                cached_params!(
                    "{\"fields\":[\"timestamp_native\",\"timestamp\",\"delink_time\",\"start_time\",\"end_time\"]}"
                ),
            )?;

            let _cond = { event.has_value("_conf.custom_date_format") };
            if _cond {
                // Painless script
                // Source: def zone = ctx.event?.timezone != null ? ZoneId.of(ctx.event.timezone) : null;\ndef formatter = DateTimeFormatter.ofPattern(ctx._conf.custom_date_format);\ndef outFormatter = DateTimeFormatter.ofPattern(\"yyyy-MM-dd'T'HH:mm:ss.SSSXXX\");\n\nparams.fields.forEach(field -> {\n  if (!ctx._tmp?.containsKey(field)) {\n    return true;\n  }\n\n  try {\n    def localDateTime = LocalDateTime.parse(ctx._tmp[field], formatter);\n    ctx.citrix_adc.log[field] = outFormatter.format(ZonedDateTime.of(localDateTime, zone));\n  } catch (Exception e) {\n    /* Intentionally ignored */\n    return true;\n  }\n});
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def zone = ctx.event?.timezone != null ? ZoneId.of(ctx.event.timezone) : null;\ndef formatter = DateTimeFormatter.ofPattern(ctx._conf.custom_date_format);\ndef outFormatter = DateTimeFormatter.ofPattern(\"yyyy-MM-dd'T'HH:mm:ss.SSSXXX\");\n\nparams.fields.forEach(field -> {\n  if (!ctx._tmp?.containsKey(field)) {\n    return true;\n  }\n\n  try {\n    def localDateTime = LocalDateTime.parse(ctx._tmp[field], formatter);\n    ctx.citrix_adc.log[field] = outFormatter.format(ZonedDateTime.of(localDateTime, zone));\n  } catch (Exception e) {\n    /* Intentionally ignored */\n    return true;\n  }\n});"#
                    ),
                    cached_params!(
                        "{\"fields\":[\"timestamp_native\",\"timestamp\",\"delink_time\",\"start_time\",\"end_time\"]}"
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_tmp.timestamp_native")
                    && !event.has_value("citrix_adc.log.timestamp_native")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp_native") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy/MM/dd:HH:mm:ss",
                                "yyyy/MM/dd:HH:mm:ss z",
                                "MM/dd/yyyy:HH:mm:ss",
                                "MM/dd/yyyy:HH:mm:ss z",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("citrix_adc.log.timestamp_native", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.timestamp_native".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timestamp_native")?;
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
                                .get("_ingest.on_failure_pipeline")
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
            }

            let _cond = {
                event.has_value("_tmp.timestamp") && !event.has_value("citrix_adc.log.timestamp")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy/MM/dd:HH:mm:ss",
                                "yyyy/MM/dd:HH:mm:ss z",
                                "MM/dd/yyyy:HH:mm:ss",
                                "MM/dd/yyyy:HH:mm:ss z",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("citrix_adc.log.timestamp", parsed)?,
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
                    event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
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
                                .get("_ingest.on_failure_pipeline")
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
            }

            let _cond = {
                event.has_value("_tmp.start_time") && !event.has_value("citrix_adc.log.start_time")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.start_time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy/MM/dd:HH:mm:ss",
                                "yyyy/MM/dd:HH:mm:ss z",
                                "MM/dd/yyyy:HH:mm:ss",
                                "MM/dd/yyyy:HH:mm:ss z",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("citrix_adc.log.start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.start_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_start_time")?;
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
                                .get("_ingest.on_failure_pipeline")
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
            }

            let _cond =
                { event.has_value("_tmp.end_time") && !event.has_value("citrix_adc.log.end_time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.end_time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy/MM/dd:HH:mm:ss",
                                "yyyy/MM/dd:HH:mm:ss z",
                                "yyyy/MM/dd:HH:mm:ssz",
                                "MM/dd/yyyy:HH:mm:ss",
                                "MM/dd/yyyy:HH:mm:ss z",
                                "MM/dd/yyyy:HH:mm:ssz",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("citrix_adc.log.end_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.end_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_end_time")?;
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
                                .get("_ingest.on_failure_pipeline")
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
            }

            let _cond = {
                event.has_value("_tmp.delink_time")
                    && !event.has_value("citrix_adc.log.delink_time")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.delink_time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy/MM/dd:HH:mm:ss",
                                "yyyy/MM/dd:HH:mm:ss z",
                                "MM/dd/yyyy:HH:mm:ss",
                                "MM/dd/yyyy:HH:mm:ss z",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("citrix_adc.log.delink_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.delink_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_delink_time")?;
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
                                .get("_ingest.on_failure_pipeline")
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
            }

            if let Some(v) = event
                .get("citrix_adc.log.timestamp_native")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            event.remove("citrix_adc.log.timestamp_native");

            if let Some(v) = event
                .get("citrix_adc.log.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if let Some(v) = event
                .get("citrix_adc.log.delink_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            if let Some(v) = event
                .get("citrix_adc.log.end_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = {
                event.has_value("event.start")
                    && event.has_value("event.end")
                    && !event.has_value("event.duration")
            };
            if _cond {
                // Painless script
                // Source: ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);"#
                    ),
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

            if event.has_value("source.ip") {
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

            if event.has_value("source.ip") {
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("client.as.asn") {
                event.rename("client.as.asn", "client.as.number")?;
            }

            if event.has_value("client.as.organization_name") {
                event.rename("client.as.organization_name", "client.as.organization.name")?;
            }

            let _cond =
                { event.has_value("url.original") && event.get_str("url.original") != Some("") };
            if _cond {
                uri_parts(event, "url.original", "url", true, false)?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("user.name", "user.email")?;
            }

            let _cond = { !event.has_value("user.name") && !event.has_value("user.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
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

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.user.name")
                    && event
                        .get_str("source.user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("source.user.name", "source.user.email")?;
            }

            let _cond =
                { !event.has_value("source.user.name") && !event.has_value("source.user.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("source.user.email") {
                        if let Some(input) = event.get_string("source.user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("source.user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("source.user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("destination.user.name")
                    && event
                        .get_str("destination.user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("destination.user.name", "destination.user.email")?;
            }

            let _cond = {
                !event.has_value("destination.user.name")
                    && !event.has_value("destination.user.domain")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("destination.user.email") {
                        if let Some(input) = event.get_string("destination.user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("destination.user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("destination.user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            event.remove("_tmp");
            event.remove("_conf");

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
                event.remove("_tmp");
                event.remove("_conf");
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
