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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("observer.vendor", json!("Pulse Secure"))?;

            event.set("observer.product", json!("Pulse Secure Connect"))?;

            event.set("observer.type", json!("vpn"))?;

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^(<%{NONNEGINT:log.syslog.priority:long}>%{NUMBER}?|%{SYSLOGTIMESTAMP} %{SYSLOGHOST:host.hostname} %{INT}) (?P<_tmp_timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:event.timezone}?)) (%{IP:observer.ip}|%{HOSTNAME:observer.hostname}) PulseSecure: - - - (?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY} %{HOUR}:?%{MINUTE}(?::?%{SECOND})?) - %{SYSLOGHOST:observer.name} - \\[%{IPORHOST:client.address}\\] (%{DATA}::)?(%{HOSTNAME:user.domain}(?:\\\\){1,2})?%{USERNAME:user.name}?(@%{USERNAME:user.domain})?\\(%{DATA:pulse_secure.realm}?\\)\\[%{DATA:pulse_secure.role}\\](?::?\\[%{DATA:pulse_secure.session.id_short}\\])? - %{GREEDYDATA:message}
                let _ = cached_grok_mapped!("^(<%{NONNEGINT:log.syslog.priority:long}>%{NUMBER}?|%{SYSLOGTIMESTAMP} %{SYSLOGHOST:host.hostname} %{INT}) (?P<_tmp_timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:event.timezone}?)) (%{IP:observer.ip}|%{HOSTNAME:observer.hostname}) PulseSecure: - - - (?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY} %{HOUR}:?%{MINUTE}(?::?%{SECOND})?) - %{SYSLOGHOST:observer.name} - \\[%{IPORHOST:client.address}\\] (%{DATA}::)?(%{HOSTNAME:user.domain}(?:\\\\){1,2})?%{USERNAME:user.name}?(@%{USERNAME:user.domain})?\\(%{DATA:pulse_secure.realm}?\\)\\[%{DATA:pulse_secure.role}\\](?::?\\[%{DATA:pulse_secure.session.id_short}\\])? - %{GREEDYDATA:message}", [("_tmp_timestamp", "_tmp.timestamp")]).extract_into(&input, event)?;
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &["ISO8601"],
                        event.get_str("event.timezone"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            if event.has_value("client.address") {
                if let Some(val) = event.get("client.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "client.address".into(),
                            message,
                        })?;
                    event.set("client.ip", converted)?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Agent login %{WORD:_tmp.outcome} for %{DATA}(?:\\(session:%{SPACE}%{NOTSPACE:pulse_secure.session.id}\\)) from %{IP} with %{GREEDYDATA:user_agent.original}.
                    // Grok pattern: VPN Tunneling: Session %{WORD:_tmp.type} for user (?:\\(session:%{SPACE}%{NOTSPACE:pulse_secure.session.id}\\)) with %{NOTSPACE:network.type} address %{IP:client.nat.ip}(, hostname %{HOSTNAME:host.name})?
                    // Grok pattern: Session %{WORD} from user agent '%{GREEDYDATA:user_agent.original}' (?:\\(session:%{SPACE}%{NOTSPACE:pulse_secure.session.id}\\)).
                    // Grok pattern: Login %{WORD:_tmp.outcome}( %{GREEDYDATA})?. Reason: %{GREEDYDATA:event.reason}
                    // Grok pattern: ^Primary authentication %{WORD_tmp.outcome}
                    // Grok pattern: (?:\\(session:%{SPACE}%{NOTSPACE:pulse_secure.session.id}\\))
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "Agent login %{WORD:_tmp.outcome} for %{DATA}(?:\\(session:%{SPACE}%{NOTSPACE:pulse_secure.session.id}\\)) from %{IP} with %{GREEDYDATA:user_agent.original}."
                            ),
                            cached_grok!(
                                "VPN Tunneling: Session %{WORD:_tmp.type} for user (?:\\(session:%{SPACE}%{NOTSPACE:pulse_secure.session.id}\\)) with %{NOTSPACE:network.type} address %{IP:client.nat.ip}(, hostname %{HOSTNAME:host.name})?"
                            ),
                            cached_grok!(
                                "Session %{WORD} from user agent '%{GREEDYDATA:user_agent.original}' (?:\\(session:%{SPACE}%{NOTSPACE:pulse_secure.session.id}\\))."
                            ),
                            cached_grok!(
                                "Login %{WORD:_tmp.outcome}( %{GREEDYDATA})?. Reason: %{GREEDYDATA:event.reason}"
                            ),
                            cached_grok!("^Primary authentication %{WORD_tmp.outcome}"),
                            cached_grok!(
                                "(?:\\(session:%{SPACE}%{NOTSPACE:pulse_secure.session.id}\\))"
                            ),
                        ],
                        &input,
                        event,
                    )?;
                }
                Ok(())
            })();

            if event.has_value("network.type") {
                map_strings(event, "network.type", "network.type", str::to_lowercase)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("user_agent.original") {
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
                Ok(())
            })();

            let _cond = {
                event.has_value("_tmp.outcome")
                    && ["failed"].contains(&event.get_str("_tmp.outcome").unwrap_or(""))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("_tmp.outcome")
                    && ["successful", "succeeded"]
                        .contains(&event.get_str("_tmp.outcome").unwrap_or(""))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond =
                { event.has_value("_tmp.type") && event.get_str("_tmp.type") == Some("started") };
            if _cond {
                event.append("event.type", json!("connection"))?;
                event.append("event.type", json!("start"))?;
            }

            let _cond =
                { event.has_value("_tmp.type") && event.get_str("_tmp.type") == Some("ended") };
            if _cond {
                event.append("event.type", json!("connection"))?;
                event.append("event.type", json!("end"))?;
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

            if let Some(v) = event.get("client").cloned() {
                event.set("source", v)?;
            }

            let _cond = { event.get("observer.ip").is_some_and(|v| v.is_string()) };
            if _cond {
                event.set(
                    "observer.ip",
                    Value::Array(vec![json!(
                        event
                            .get("observer.ip")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            event.remove("_tmp");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
