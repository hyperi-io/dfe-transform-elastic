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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: ^(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)\\s*)?(?P<_tmp_timestamp>(?:(?:%{YEAR}:%{MONTHNUM}:%{MONTHDAY}-%{HOUR}:%{MINUTE}:%{SECOND}))) (?:%{HOSTNAME:host.hostname}) %{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?: %{GREEDYDATA:_tmp.raw_data}
                        if !cached_grok_mapped!("^(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)\\s*)?(?P<_tmp_timestamp>(?:(?:%{YEAR}:%{MONTHNUM}:%{MONTHDAY}-%{HOUR}:%{MINUTE}:%{SECOND}))) (?:%{HOSTNAME:host.hostname}) %{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?: %{GREEDYDATA:_tmp.raw_data}", [("_tmp_timestamp", "_tmp.timestamp")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_syslog_header")?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.set("event.kind", json!("pipeline_error"))?;
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
                if event.has_value("host.hostname") {
                    map_strings(event, "host.hostname", "host.name", str::to_lowercase)?;
                }
                Ok(())
            })();

            // Painless script, resolved to its runners at generation time
            // Source: if (ctx.log?.syslog?.priority != null) {\n  def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  facility['code'] = ctx.log.syslog.priority>>3;\n  ctx.log.syslog['facility'] = facility;\n}\n
            syslog_priority(event, &SyslogPriorityScript::new(None, true, true, false));

            // Painless script
            // Source: if (ctx.log?.syslog?.facility?.code == null || !params.containsKey((ctx.log.syslog.facility.code).toString())) {\n  return;\n}\nctx.log.syslog.facility.name = params[(ctx.log.syslog.facility.code).toString()];
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.log?.syslog?.facility?.code == null || !params.containsKey((ctx.log.syslog.facility.code).toString())) {\n  return;\n}\nctx.log.syslog.facility.name = params[(ctx.log.syslog.facility.code).toString()];"#
                ),
                cached_params!(
                    "{\"0\":\"Kernel\",\"1\":\"User\",\"2\":\"Mail\",\"3\":\"System\",\"4\":\"Security\",\"5\":\"Syslog\",\"6\":\"Line printer\",\"7\":\"Network news\",\"8\":\"UUCP\",\"9\":\"Clock\",\"10\":\"Security\",\"11\":\"FTPd\",\"12\":\"NTPd\",\"13\":\"Log audit\",\"14\":\"Log alert\",\"15\":\"Clock daemon\",\"16\":\"Local 0\",\"17\":\"Local 1\",\"18\":\"Local 2\",\"19\":\"Local 3\",\"20\":\"Local 4\",\"21\":\"Local 5\",\"22\":\"Local 6\",\"23\":\"Local 7\"}"
                ),
            )?;

            // Painless script
            // Source: if (ctx.log?.syslog?.severity?.code == null || !params.containsKey((ctx.log.syslog.severity.code).toString())) {\n  return;\n}\nctx.log.syslog.severity.name = params[(ctx.log.syslog.severity.code).toString()];
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.log?.syslog?.severity?.code == null || !params.containsKey((ctx.log.syslog.severity.code).toString())) {\n  return;\n}\nctx.log.syslog.severity.name = params[(ctx.log.syslog.severity.code).toString()];"#
                ),
                cached_params!(
                    "{\"0\":\"Emergency\",\"1\":\"Alert\",\"2\":\"Critical\",\"3\":\"Error\",\"4\":\"Warning\",\"5\":\"Notice\",\"6\":\"Informational\",\"7\":\"Debug\"}"
                ),
            )?;

            let _cond = { event.has_value("_conf.tz_offset") };
            if _cond {
                if event.has_value("_conf.tz_offset") {
                    event.rename("_conf.tz_offset", "event.timezone")?;
                }
            }

            if !event.has("event.timezone") {
                event.set("event.timezone", json!("UTC"))?;
            }

            let _cond = { event.has_value("_tmp.timestamp") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy:MM:dd-HH:mm:ss"],
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
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_set_timestamp")?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.set("event.kind", json!("pipeline_error"))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_str("process.name") == Some("named") };
            if _cond {
                event.set("event.provider", json!("dns"))?;
            }

            let _cond = { event.get_str("process.name") == Some("dhcpd") };
            if _cond {
                event.set("event.provider", json!("dhcp"))?;
            }

            let _cond = { event.get_str("process.name") == Some("httpproxy") };
            if _cond {
                event.set("event.provider", json!("http"))?;
            }

            let _cond = { event.get_str("process.name") == Some("ulogd") };
            if _cond {
                event.set("event.provider", json!("packetfilter"))?;
            }

            let _cond = { event.get_str("event.provider") == Some("dns") };
            if _cond {
                // Begin nested pipeline: "dns"
                event.append_unique("event.type", json!("connection"))?;
                event.append_unique("event.type", json!("protocol"))?;
                event.set("network.protocol", json!("dns"))?;
                event.set("network.transport", json!("udp"))?;
                let _cond = {
                    event.has_value("_tmp.raw_data")
                        && event.get("_tmp.raw_data").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("listening"))
                            }
                            serde_json::Value::String(s) => s.contains("listening"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.raw_data") {
                        // Grok pattern: ^%{WORD:event.action} on %{WORD:network.type} interface %{WORD:observer.ingress.interface.name}, %{IP:server.ip}#%{NUMBER:server.port:long}$
                        // Grok pattern: ^no longer %{WORD:event.action} on %{IP:server.ip}#%{NUMBER:server.port:long}$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} on %{WORD:network.type} interface %{WORD:observer.ingress.interface.name}, %{IP:server.ip}#%{NUMBER:server.port:long}$"
                                ),
                                cached_grok!(
                                    "^no longer %{WORD:event.action} on %{IP:server.ip}#%{NUMBER:server.port:long}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = { !event.has_value("event.action") };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.raw_data") {
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !cached_grok!("^%{GREEDYDATA:message}$").extract_into(&input, event)? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("network.type") {
                        map_strings(event, "network.type", "network.type", str::to_lowercase)?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("event.action") };
                if _cond {
                    event.set(
                        "event.action",
                        json!(format!(
                            "{}-{}",
                            event
                                .get("process.name")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("event.action")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond = { event.get_str("server.ip") != Some("") };
                if _cond {
                    if event.has_value("server.ip") {
                        if let Some(ip_str) = event.get_string("server.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("server.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("server.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("server.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("server.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("server.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("server.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("server.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("server.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { event.get_str("server.ip") != Some("") };
                if _cond {
                    if event.has_value("server.ip") {
                        if let Some(ip_str) = event.get_string("server.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("server.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("server.as.organization_name", v.clone())?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("server.as.asn") {
                    event.rename("server.as.asn", "server.as.number")?;
                }
                if event.has_value("server.as.organization_name") {
                    event.rename("server.as.organization_name", "server.as.organization.name")?;
                }
                let _cond =
                    { event.has_value("server.ip") && event.get_str("server.ip") != Some("") };
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
                // End nested pipeline: "dns"
            }

            let _cond = { event.get_str("event.provider") == Some("dhcp") };
            if _cond {
                // Begin nested pipeline: "dhcp"
                event.append_unique("event.type", json!("connection"))?;
                event.append_unique("event.type", json!("protocol"))?;
                event.set("network.protocol", json!("dhcp"))?;
                event.set("network.transport", json!("udp"))?;
                let _cond = {
                    event.has_value("_tmp.raw_data")
                        && event
                            .get_str("_tmp.raw_data")
                            .is_some_and(|s| s.starts_with("DHCPDISCOVER"))
                };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.raw_data") {
                        // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}: %{GREEDYDATA:message}$
                        // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}$
                        // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} from %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}: %{GREEDYDATA:message}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} from %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}$"
                                ),
                                cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.has_value("_tmp.raw_data")
                        && event
                            .get_str("_tmp.raw_data")
                            .is_some_and(|s| s.starts_with("DHCPOFFER"))
                };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.raw_data") {
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}$
                        // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via %{WORD:observer.ingress.interface.name}$"
                                ),
                                cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.has_value("_tmp.raw_data")
                        && event
                            .get_str("_tmp.raw_data")
                            .is_some_and(|s| s.starts_with("DHCPREQUEST"))
                };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.raw_data") {
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip}( \\(%{IP:sophos.utm.router.ip}\\))? from %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}(: %{GREEDYDATA:message})?$
                        // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip}( \\(%{IP:sophos.utm.router.ip}\\))? from %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}(: %{GREEDYDATA:message})?$"
                                ),
                                cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.has_value("_tmp.raw_data")
                        && event
                            .get_str("_tmp.raw_data")
                            .is_some_and(|s| s.starts_with("DHCPACK"))
                };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.raw_data") {
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}$
                        // Grok pattern: ^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$
                        // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$"
                                ),
                                cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.has_value("_tmp.raw_data")
                        && event
                            .get_str("_tmp.raw_data")
                            .is_some_and(|s| s.starts_with("DHCPNACK"))
                };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.raw_data") {
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}$
                        // Grok pattern: ^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$
                        // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}( \\(%{DATA:sophos.utm.client.hostname}\\))? via %{WORD:observer.ingress.interface.name}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$"
                                ),
                                cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.has_value("_tmp.raw_data")
                        && event
                            .get_str("_tmp.raw_data")
                            .is_some_and(|s| s.starts_with("DHCPINFORM"))
                };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.raw_data") {
                        // Grok pattern: ^%{WORD:event.action} from %{IP:client.ip} via %{WORD:observer.ingress.interface.name}: %{GREEDYDATA:message}$
                        // Grok pattern: ^%{WORD:event.action} from %{IP:client.ip} via %{WORD:observer.ingress.interface.name}$
                        // Grok pattern: ^%{WORD:event.action} %{GREEDYDATA:message}$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} from %{IP:client.ip} via %{WORD:observer.ingress.interface.name}: %{GREEDYDATA:message}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} from %{IP:client.ip} via %{WORD:observer.ingress.interface.name}$"
                                ),
                                cached_grok!("^%{WORD:event.action} %{GREEDYDATA:message}$"),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.has_value("_tmp.raw_data")
                        && event
                            .get_str("_tmp.raw_data")
                            .is_some_and(|s| s.starts_with("Listening"))
                };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.raw_data") {
                        // Grok pattern: ^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{MAC:client.mac}/%{DATA:sophos.utm.subnet}$
                        // Grok pattern: ^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{DATA:sophos.utm.subnet}$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{MAC:client.mac}/%{DATA:sophos.utm.subnet}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{DATA:sophos.utm.subnet}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.has_value("_tmp.raw_data")
                        && event
                            .get_str("_tmp.raw_data")
                            .is_some_and(|s| s.starts_with("Sending"))
                };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.raw_data") {
                        // Grok pattern: ^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{MAC:client.mac}/%{DATA:sophos.utm.subnet}$
                        // Grok pattern: ^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{DATA:sophos.utm.subnet}$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{MAC:client.mac}/%{DATA:sophos.utm.subnet}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action}[ ]+on[ ]+%{WORD:sophos.utm.socket}/%{WORD:observer.ingress.interface.name}/%{DATA:sophos.utm.subnet}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = { !event.has_value("event.action") };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.raw_data") {
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !cached_grok!("^%{GREEDYDATA:message}$").extract_into(&input, event)? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("event.action")
                        && (event.get_str("event.action") == Some("sending")
                            || event.get_str("event.action") == Some("listening"))
                };
                if _cond {
                    event.set(
                        "event.action",
                        json!(format!(
                            "{}-{}",
                            event
                                .get("process.name")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("event.action")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                if event.has_value("client.mac") {
                    gsub_field(event, "client.mac", "client.mac", cached_regex!("[:]"), "-")?;
                }
                if event.has_value("client.mac") {
                    map_strings(event, "client.mac", "client.mac", str::to_uppercase)?;
                }
                let _cond = { event.get_str("client.ip") != Some("") };
                if _cond {
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
                }
                let _cond = { event.get_str("client.ip") != Some("") };
                if _cond {
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
                }
                if event.has_value("client.as.asn") {
                    event.rename("client.as.asn", "client.as.number")?;
                }
                if event.has_value("client.as.organization_name") {
                    event.rename("client.as.organization_name", "client.as.organization.name")?;
                }
                let _cond = {
                    event.has_value("sophos.utm.client.hostname")
                        && event.get_str("sophos.utm.client.hostname") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("sophos.utm.client.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond =
                    { event.has_value("client.ip") && event.get_str("client.ip") != Some("") };
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
                // End nested pipeline: "dhcp"
            }

            let _cond = { event.get_str("event.provider") == Some("http") };
            if _cond {
                // Begin nested pipeline: "http"
                event.set("network.protocol", json!("http"))?;
                event.set("event.category", Value::Array(vec![json!("web")]))?;
                let _cond = { event.has_value("_tmp.raw_data") };
                if _cond {
                    if event.has_value("_tmp.raw_data") {
                        if let Some(kv_str) = event.get_string("_tmp.raw_data") {
                            let mut kv_gap = false;
                            for pair in cached_regex!(" (?=[a-z0-9\\_\\-]+=\")")
                                .split(&kv_str)
                                .into_iter()
                            {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "_tmp.raw_data".into(),
                                        split: "=".into(),
                                    });
                                };
                                {
                                    let value =
                                        value.trim_matches(|c: char| matches!(c, ' ' | '\"'));
                                    if !key.is_empty() {
                                        kv_put(event, &format!("sophos.utm.{}", key), value)?;
                                    }
                                }
                            }
                        }
                    }
                }
                let _cond = {
                    event.has_value("sophos.utm.action") && !event.has_value("sophos.utm.reason")
                };
                if _cond {
                    event.rename("sophos.utm.action", "event.action")?;
                }
                let _cond = {
                    event.has_value("sophos.utm.action") && event.has_value("sophos.utm.reason")
                };
                if _cond {
                    event.set(
                        "event.action",
                        json!(format!(
                            "{}-{}",
                            event
                                .get("sophos.utm.action")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("sophos.utm.reason")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                if event.has_value("sophos.utm.application") {
                    event.rename("sophos.utm.application", "network.application")?;
                }
                if event.has_value("sophos.utm.dstip") {
                    event.rename("sophos.utm.dstip", "destination.ip")?;
                }
                if event.has_value("sophos.utm.device") {
                    event.rename("sophos.utm.device", "device.id")?;
                }
                let _cond = {
                    event.has_value("sophos.utm.error")
                        && event.get_str("sophos.utm.error") != Some("")
                };
                if _cond {
                    event.rename("sophos.utm.error", "error.message")?;
                }
                let _cond = {
                    event.has_value("sophos.utm.file")
                        && event.get_str("sophos.utm.file") != Some("")
                };
                if _cond {
                    event.rename("sophos.utm.file", "file.name")?;
                }
                let _cond = {
                    event.has_value("sophos.utm.filename")
                        && event.get_str("sophos.utm.filename") != Some("")
                };
                if _cond {
                    event.rename("sophos.utm.filename", "file.name")?;
                }
                if event.has_value("sophos.utm.group") {
                    event.rename("sophos.utm.group", "group.name")?;
                }
                if event.has_value("sophos.utm.id") {
                    event.rename("sophos.utm.id", "event.id")?;
                }
                if event.has_value("sophos.utm.message") {
                    event.rename("sophos.utm.message", "message")?;
                }
                if event.has_value("sophos.utm.method") {
                    event.rename("sophos.utm.method", "http.request.method")?;
                }
                if event.has_value("sophos.utm.referer") {
                    event.rename("sophos.utm.referer", "http.request.referrer")?;
                }
                if event.has_value("sophos.utm.request") {
                    event.rename("sophos.utm.request", "http.request.id")?;
                }
                if event.has_value("sophos.utm.size") {
                    if let Some(val) = event.get("sophos.utm.size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.size".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.size", converted)?;
                    }
                }
                if event.has_value("sophos.utm.size") {
                    event.rename("sophos.utm.size", "http.request.bytes")?;
                }
                if event.has_value("sophos.utm.srcip") {
                    event.rename("sophos.utm.srcip", "source.ip")?;
                }
                if event.has_value("sophos.utm.statuscode") {
                    if let Some(val) = event.get("sophos.utm.statuscode") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.statuscode".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.statuscode", converted)?;
                    }
                }
                if event.has_value("sophos.utm.statuscode") {
                    event.rename("sophos.utm.statuscode", "http.response.status_code")?;
                }
                if event.has_value("sophos.utm.ua") {
                    event.rename("sophos.utm.ua", "user_agent.original")?;
                }
                if event.has_value("user_agent.original") {
                    if let Some(ua_str) = event.get_string("user_agent.original") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.remove("user_agent");
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
                let _cond = { event.has_value("sophos.utm.url") };
                if _cond {
                    uri_parts(event, "sophos.utm.url", "url", true, true)?;
                }
                if event.has_value("sophos.utm.user") {
                    event.rename("sophos.utm.user", "user.name")?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("emergency") };
                if _cond {
                    event.set("event.severity", json!(0))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("alert") };
                if _cond {
                    event.set("event.severity", json!(1))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("critical") };
                if _cond {
                    event.set("event.severity", json!(2))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("error") };
                if _cond {
                    event.set("event.severity", json!(3))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("warning") };
                if _cond {
                    event.set("event.severity", json!(4))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("notice") };
                if _cond {
                    event.set("event.severity", json!(5))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("info") };
                if _cond {
                    event.set("event.severity", json!(6))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("debug") };
                if _cond {
                    event.set("event.severity", json!(7))?;
                }
                if event.has_value("sophos.utm.category") {
                    if let Some(s) = event.get_string("sophos.utm.category") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("sophos.utm.category", Value::Array(parts))?;
                    }
                }
                if event.has_value("sophos.utm.categoryname") {
                    if let Some(s) = event.get_string("sophos.utm.categoryname") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("sophos.utm.categoryname", Value::Array(parts))?;
                    }
                }
                if event.has_value("sophos.utm.exceptions") {
                    if let Some(s) = event.get_string("sophos.utm.exceptions") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("sophos.utm.exceptions", Value::Array(parts))?;
                    }
                }
                foreach_array(event, "sophos.utm", |event| {
                    gsub_field(
                        event,
                        "_ingest._key",
                        "_ingest._key",
                        cached_regex!("-"),
                        "_",
                    )?;
                    Ok(())
                })?;
                let _cond = { event.get_str("source.ip") != Some("") };
                if _cond {
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
                }
                let _cond = { event.get_str("destination.ip") != Some("") };
                if _cond {
                    if event.has_value("destination.ip") {
                        if let Some(ip_str) = event.get_string("destination.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { event.get_str("source.ip") != Some("") };
                if _cond {
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
                }
                let _cond = { event.get_str("destination.ip") != Some("") };
                if _cond {
                    if event.has_value("destination.ip") {
                        if let Some(ip_str) = event.get_string("destination.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("destination.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("destination.as.organization_name", v.clone())?;
                                }
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
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                if event.has_value("sophos.utm.aptptime") {
                    if let Some(val) = event.get("sophos.utm.aptptime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.aptptime".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.aptptime", converted)?;
                    }
                }
                if event.has_value("sophos.utm.authtime") {
                    if let Some(val) = event.get("sophos.utm.authtime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.authtime".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.authtime", converted)?;
                    }
                }
                if event.has_value("sophos.utm.avscantime") {
                    if let Some(val) = event.get("sophos.utm.avscantime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.avscantime".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.avscantime", converted)?;
                    }
                }
                if event.has_value("sophos.utm.cattime") {
                    if let Some(val) = event.get("sophos.utm.cattime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.cattime".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.cattime", converted)?;
                    }
                }
                if event.has_value("sophos.utm.dnstime") {
                    if let Some(val) = event.get("sophos.utm.dnstime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.dnstime".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.dnstime", converted)?;
                    }
                }
                if event.has_value("sophos.utm.fullreqtime") {
                    if let Some(val) = event.get("sophos.utm.fullreqtime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.fullreqtime".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.fullreqtime", converted)?;
                    }
                }
                let _cond =
                    { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
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
                let _cond = {
                    event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
                };
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
                let _cond =
                    { event.has_value("user.name") && event.get_str("user.name") != Some("") };
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
                // End nested pipeline: "http"
            }

            let _cond = { event.get_str("event.provider") == Some("packetfilter") };
            if _cond {
                // Begin nested pipeline: "packetfilter"
                let _cond = { event.has_value("_tmp.raw_data") };
                if _cond {
                    if event.has_value("_tmp.raw_data") {
                        if let Some(kv_str) = event.get_string("_tmp.raw_data") {
                            let mut kv_gap = false;
                            for pair in cached_regex!(" (?=[a-z0-9\\_\\-]+=)")
                                .split(&kv_str)
                                .into_iter()
                            {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "_tmp.raw_data".into(),
                                        split: "=".into(),
                                    });
                                };
                                {
                                    let value =
                                        value.trim_matches(|c: char| matches!(c, ' ' | '\"'));
                                    if !key.is_empty() {
                                        kv_put(event, &format!("sophos.utm.{}", key), value)?;
                                    }
                                }
                            }
                        }
                    }
                }
                if event.has_value("sophos.utm.action") {
                    event.rename("sophos.utm.action", "event.action")?;
                }
                let _cond = { event.get_str("event.action") == Some("accept") };
                if _cond {
                    event.append_unique("event.type", json!("allowed"))?;
                }
                let _cond = { event.get_str("event.action") == Some("drop") };
                if _cond {
                    event.append_unique("event.type", json!("denied"))?;
                }
                if event.has_value("sophos.utm.dstip") {
                    event.rename("sophos.utm.dstip", "destination.ip")?;
                }
                if event.has_value("sophos.utm.dstmac") {
                    event.rename("sophos.utm.dstmac", "destination.mac")?;
                }
                if event.has_value("sophos.utm.dstport") {
                    if let Some(val) = event.get("sophos.utm.dstport") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.dstport".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.dstport", converted)?;
                    }
                }
                if event.has_value("sophos.utm.dstport") {
                    event.rename("sophos.utm.dstport", "destination.port")?;
                }
                if event.has_value("sophos.utm.id") {
                    event.rename("sophos.utm.id", "event.id")?;
                }
                if event.has_value("sophos.utm.srcip") {
                    event.rename("sophos.utm.srcip", "source.ip")?;
                }
                if event.has_value("sophos.utm.srcmac") {
                    event.rename("sophos.utm.srcmac", "source.mac")?;
                }
                if event.has_value("sophos.utm.srcport") {
                    if let Some(val) = event.get("sophos.utm.srcport") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.srcport".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.srcport", converted)?;
                    }
                }
                if event.has_value("sophos.utm.srcport") {
                    event.rename("sophos.utm.srcport", "source.port")?;
                }
                if event.has_value("sophos.utm.fwrule") {
                    event.rename("sophos.utm.fwrule", "rule.id")?;
                }
                if event.has_value("sophos.utm.initf") {
                    event.rename("sophos.utm.initf", "observer.ingress.interface.name")?;
                }
                if event.has_value("sophos.utm.outitf") {
                    event.rename("sophos.utm.outitf", "observer.egress.interface.name")?;
                }
                if event.has_value("sophos.utm.proto") {
                    event.rename("sophos.utm.proto", "network.iana_number")?;
                }
                if event.has_value("sophos.utm.message") {
                    event.rename("sophos.utm.message", "message")?;
                }
                if event.has_value("sophos.utm.app") {
                    event.rename("sophos.utm.app", "sophos.utm.app_id")?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("emergency") };
                if _cond {
                    event.set("event.severity", json!(0))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("alert") };
                if _cond {
                    event.set("event.severity", json!(1))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("critical") };
                if _cond {
                    event.set("event.severity", json!(2))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("error") };
                if _cond {
                    event.set("event.severity", json!(3))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("warning") };
                if _cond {
                    event.set("event.severity", json!(4))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("notice") };
                if _cond {
                    event.set("event.severity", json!(5))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("info") };
                if _cond {
                    event.set("event.severity", json!(6))?;
                }
                let _cond = { event.get_str("sophos.utm.severity") == Some("debug") };
                if _cond {
                    event.set("event.severity", json!(7))?;
                }
                if event.has_value("sophos.utm.tcpflags") {
                    if let Some(s) = event.get_string("sophos.utm.tcpflags") {
                        let mut parts: Vec<Value> = cached_regex!("\\s+")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("sophos.utm.tcpflags", Value::Array(parts))?;
                    }
                }
                if event.has_value("sophos.utm.tcpflags") {
                    map_strings(
                        event,
                        "sophos.utm.tcpflags",
                        "sophos.utm.tcpflags",
                        str::to_lowercase,
                    )?;
                }
                if event.has_value("source.mac") {
                    gsub_field(event, "source.mac", "source.mac", cached_regex!("[:]"), "-")?;
                }
                if event.has_value("source.mac") {
                    map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
                }
                if event.has_value("destination.mac") {
                    gsub_field(
                        event,
                        "destination.mac",
                        "destination.mac",
                        cached_regex!("[:]"),
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
                let _cond = { event.get_str("source.ip") != Some("") };
                if _cond {
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
                }
                let _cond = { event.get_str("destination.ip") != Some("") };
                if _cond {
                    if event.has_value("destination.ip") {
                        if let Some(ip_str) = event.get_string("destination.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { event.get_str("source.ip") != Some("") };
                if _cond {
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
                }
                let _cond = { event.get_str("destination.ip") != Some("") };
                if _cond {
                    if event.has_value("destination.ip") {
                        if let Some(ip_str) = event.get_string("destination.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("destination.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("destination.as.organization_name", v.clone())?;
                                }
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
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                if event.has_value("sophos.utm.length") {
                    if let Some(val) = event.get("sophos.utm.length") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.length".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.length", converted)?;
                    }
                }
                if event.has_value("sophos.utm.ttl") {
                    if let Some(val) = event.get("sophos.utm.ttl") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sophos.utm.ttl".into(),
                                message,
                            }
                        })?;
                        event.set("sophos.utm.ttl", converted)?;
                    }
                }
                let _cond =
                    { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
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
                let _cond = {
                    event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
                };
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
                // End nested pipeline: "packetfilter"
            }

            event.set("observer.vendor", json!("Sophos"))?;

            event.set("observer.product", json!("UTM"))?;

            event.set("observer.type", json!("firewall"))?;

            let _cond =
                { event.has_value("host.hostname") && event.get_str("host.hostname") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_tmp");
                Ok(())
            })();

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
