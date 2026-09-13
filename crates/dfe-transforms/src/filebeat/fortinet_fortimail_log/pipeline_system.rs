// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_system` pipeline.
pub struct PipelineSystem;

impl Transform for PipelineSystem {
    fn name(&self) -> &str {
        "pipeline_system"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.outcome", json!("unknown"))?;

            let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("admin") && event.get_str("temp.status").is_some_and(|s| s.to_lowercase() == "success") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("admin") && event.get_str("temp.status").is_some_and(|s| s.to_lowercase() == "failure") };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("admin") && (event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("login")) || event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("logged"))) };
            if _cond {
            event.set("event.category", Value::Array(vec![json!("authentication")]))?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("admin") && (event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("login successfully")) || event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("logged in")) || event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("login"))) };
            if _cond {
                event.append_unique("event.type", json!("end"))?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("admin") && event.has_value("event.category") };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("config") };
            if _cond {
                event.append_unique("event.category", json!("configuration"))?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("config") };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("change")]))?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("config") && event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("delet")) };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("deletion")]))?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("system") && event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("system")) };
            if _cond {
            event.set("event.category", Value::Array(vec![json!("host")]))?;
            }

            let _cond = { (event.get_str("fortinet_fortimail.log.sub_type") == Some("system") && event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("change"))) || (event.get_str("fortinet_fortimail.log.sub_type") == Some("update") && event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("update"))) };
            if _cond {
            event.set("event.category", Value::Array(vec![json!("configuration")]))?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("system") };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("system") && event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("shutdown")) };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("end")]))?;
            }

            let _cond = { (event.get_str("fortinet_fortimail.log.sub_type") == Some("system") && event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("change"))) || (event.get_str("fortinet_fortimail.log.sub_type") == Some("update") && event.get_str("fortinet_fortimail.log.message").is_some_and(|s| s.to_lowercase().contains("update"))) };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("change")]))?;
            }

            let _cond = { event.has_value("temp.user") };
            if _cond {
                event.append_unique("fortinet_fortimail.log.user", json!(event.get("temp.user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.user") };
            if _cond {
                event.append_unique("user.name", json!(event.get("temp.user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("temp.user").map_or_else(String::new, template_to_string)))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("message") {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^(?:%{DATA}(?i)interface %{NUMBER:fortinet_fortimail.log.port:long}%{DATA} (?:(?i)user %{NOTSPACE:temp.user})) %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$
                    // Grok pattern: ^(?:%{DATA}(?i)interface %{NUMBER:fortinet_fortimail.log.port:long}%{DATA} (?:(?i)user %{NOTSPACE:temp.user}))%{GREEDYDATA:temp.msg}$
                    // Grok pattern: ^%{DATA}(?:(?i)user %{NOTSPACE:temp.user}) %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$
                    // Grok pattern: ^authserver: (?P<temp_block_action>(?:(?:added|removed))) block rule for %{IP:temp.block_ip}$
                    // Grok pattern: ^authserver: (?P<temp_block_action>(?:(?:added|removed))) block rule for %{IP:temp.block_ip} until %{GREEDYDATA}$
                    // Grok pattern: ^%{DATA}(?:(?i)user %{NOTSPACE:temp.user}) %{GREEDYDATA:temp.msg}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^(?:%{DATA}(?i)interface %{NUMBER:fortinet_fortimail.log.port:long}%{DATA} (?:(?i)user %{NOTSPACE:temp.user})) %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$"),
                            cached_grok!("^(?:%{DATA}(?i)interface %{NUMBER:fortinet_fortimail.log.port:long}%{DATA} (?:(?i)user %{NOTSPACE:temp.user}))%{GREEDYDATA:temp.msg}$"),
                            cached_grok!("^%{DATA}(?:(?i)user %{NOTSPACE:temp.user}) %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$"),
                            cached_grok_mapped!("^authserver: (?P<temp_block_action>(?:(?:added|removed))) block rule for %{IP:temp.block_ip}$", [("temp_block_action", "temp.block_action")]),
                            cached_grok_mapped!("^authserver: (?P<temp_block_action>(?:(?:added|removed))) block rule for %{IP:temp.block_ip} until %{GREEDYDATA}$", [("temp_block_action", "temp.block_action")]),
                            cached_grok!("^%{DATA}(?:(?i)user %{NOTSPACE:temp.user}) %{GREEDYDATA:temp.msg}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }
                Ok(())
            })();

            if let Some(v) = event.get("fortinet_fortimail.log.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            let _cond = { event.has_value("fortinet_fortimail.log.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("fortinet_fortimail.log.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.block_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("temp.block_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.block_ip") };
            if _cond {
                event.append_unique("client.ip", json!(event.get("temp.block_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.block_ip") };
            if _cond {
            event.set("event.action", json!(format!("{}_block_rule", event.get("temp.block_action").map_or_else(String::new, template_to_string))))?;
            }

            let _cond = { event.has_value("temp.user") };
            if _cond {
                event.append_unique("fortinet_fortimail.log.user", json!(event.get("temp.user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.user") };
            if _cond {
                event.append_unique("user.name", json!(event.get("temp.user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("temp.user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("temp.user").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("temp.action") {
                    event.rename("temp.action", "fortinet_fortimail.log.action")?;
                }

            if let Some(v) = event.get("fortinet_fortimail.log.action").filter(|v| !painless_is_empty_value(v)).cloned() {
                if !event.has("event.action") {
                    event.set("event.action", v)?;
                }
            }

                if event.has_value("temp.module") {
                    event.rename("temp.module", "fortinet_fortimail.log.module")?;
                }

                if event.has_value("temp.submodule") {
                    event.rename("temp.submodule", "fortinet_fortimail.log.sub_module")?;
                }

                if event.has_value("temp.ui") {
                    event.rename("temp.ui", "fortinet_fortimail.log.ui")?;
                }

                if event.has_value("temp.reason") {
                    event.rename("temp.reason", "fortinet_fortimail.log.reason")?;
                }

                if event.has_value("temp.status") {
                    event.rename("temp.status", "fortinet_fortimail.log.status")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("fortinet_fortimail.log.ui") {
                if let Some(input) = event.get_string("fortinet_fortimail.log.ui") {
                    // Grok pattern: ^(?P<fortinet_fortimail_log_network>(?:(?i)(?:telnet|ssh|http)))%{SPACE}\\(%{SPACE}%{IP:fortinet_fortimail.log.ui_ip}%{SPACE}\\)$
                    // Grok pattern: ^%{WORD}%{SPACE}\\(%{SPACE}%{IP:fortinet_fortimail.log.ui_ip}%{SPACE}\\)$
                    // Grok pattern: ^%{DATA}%{IP:fortinet_fortimail.log.ui_ip}%{GREEDYDATA:temp.msg}$
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!("^(?P<fortinet_fortimail_log_network>(?:(?i)(?:telnet|ssh|http)))%{SPACE}\\(%{SPACE}%{IP:fortinet_fortimail.log.ui_ip}%{SPACE}\\)$", [("fortinet_fortimail_log_network", "fortinet_fortimail.log.network")]),
                            cached_grok!("^%{WORD}%{SPACE}\\(%{SPACE}%{IP:fortinet_fortimail.log.ui_ip}%{SPACE}\\)$"),
                            cached_grok!("^%{DATA}%{IP:fortinet_fortimail.log.ui_ip}%{GREEDYDATA:temp.msg}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }
                Ok(())
            })();

            if let Some(v) = event.get("fortinet_fortimail.log.ui_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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

            let _cond = { event.has_value("fortinet_fortimail.log.ui_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("fortinet_fortimail.log.ui_ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("fortinet_fortimail.log.network").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.protocol", v)?;
            }

            if event.has_value("network.protocol") {
                map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
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
