// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `tls` pipeline.
pub struct Tls;

impl Transform for Tls {
    fn name(&self) -> &str {
        "tls"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get_str("suricata.eve.tls.version") != Some("UNDETERMINED") };
            if _cond {
                if let Some(input) = event.get_string("suricata.eve.tls.version") {
                    // Grok pattern: %{DATA:tls.version_protocol} %{GREEDYDATA:tls.version}
                    // Grok pattern: %{DATA:tls.version_protocol}v%{GREEDYDATA:tls.version}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("%{DATA:tls.version_protocol} %{GREEDYDATA:tls.version}"),
                            cached_grok!("%{DATA:tls.version_protocol}v%{GREEDYDATA:tls.version}"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            if event.has_value("tls.version_protocol") {
                map_strings(event, "tls.version_protocol", "tls.version_protocol", str::to_lowercase)?;
            }

            let _cond = { event.has_value("suricata.eve.tls.sni") };
            if _cond {
                // Painless script
                // Source: def sni = ctx.suricata.eve.tls.sni;\nif (!sni.endsWith(\".\")) {\n    return;\n}\nctx.suricata.eve.tls.sni = sni.substring(0, sni.length() - 1);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def sni = ctx.suricata.eve.tls.sni;\nif (!sni.endsWith(\".\")) {\n    return;\n}\nctx.suricata.eve.tls.sni = sni.substring(0, sni.length() - 1);\n"#))?;
            }

            let v = json!(event.get("suricata.eve.tls.subject").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("tls.server.subject", v)?;
            }

            if event.has_value("suricata.eve.tls.subject") {
                if let Some(kv_str) = event.get_string("suricata.eve.tls.subject") {
                    for pair in cached_regex!(", (?=[a-zA-Z]+=)").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "suricata.eve.tls.subject".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("suricata.eve.tls.kv_subject.{}", key), value)?;
                            }
                        }
                    }
                }
            }

                if event.has_value("suricata.eve.tls.kv_subject.C") {
                    event.rename("suricata.eve.tls.kv_subject.C", "tls.server.x509.subject.country")?;
                }

            let _cond = { event.get("tls.server.x509.subject.country").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.subject.country", Value::Array(vec![json!(event.get("tls.server.x509.subject.country").map_or_else(String::new, template_to_string))]))?;
            }

                if event.has_value("suricata.eve.tls.kv_subject.CN") {
                    event.rename("suricata.eve.tls.kv_subject.CN", "tls.server.x509.subject.common_name")?;
                }

            let _cond = { event.get("tls.server.x509.subject.common_name").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.subject.common_name", Value::Array(vec![json!(event.get("tls.server.x509.subject.common_name").map_or_else(String::new, template_to_string))]))?;
            }

                if event.has_value("suricata.eve.tls.kv_subject.L") {
                    event.rename("suricata.eve.tls.kv_subject.L", "tls.server.x509.subject.locality")?;
                }

            let _cond = { event.get("tls.server.x509.subject.locality").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.subject.locality", Value::Array(vec![json!(event.get("tls.server.x509.subject.locality").map_or_else(String::new, template_to_string))]))?;
            }

                if event.has_value("suricata.eve.tls.kv_subject.O") {
                    event.rename("suricata.eve.tls.kv_subject.O", "tls.server.x509.subject.organization")?;
                }

            let _cond = { event.get("tls.server.x509.subject.organization").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.subject.organization", Value::Array(vec![json!(event.get("tls.server.x509.subject.organization").map_or_else(String::new, template_to_string))]))?;
            }

                if event.has_value("suricata.eve.tls.kv_subject.OU") {
                    event.rename("suricata.eve.tls.kv_subject.OU", "tls.server.x509.subject.organizational_unit")?;
                }

            let _cond = { event.get("tls.server.x509.subject.organizational_unit").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.subject.organizational_unit", Value::Array(vec![json!(event.get("tls.server.x509.subject.organizational_unit").map_or_else(String::new, template_to_string))]))?;
            }

                if event.has_value("suricata.eve.tls.kv_subject.ST") {
                    event.rename("suricata.eve.tls.kv_subject.ST", "tls.server.x509.subject.state_or_province")?;
                }

            let _cond = { event.get("tls.server.x509.subject.state_or_province").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.subject.state_or_province", Value::Array(vec![json!(event.get("tls.server.x509.subject.state_or_province").map_or_else(String::new, template_to_string))]))?;
            }

            let v = json!(event.get("suricata.eve.tls.issuerdn").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("tls.server.issuer", v)?;
            }

            if event.has_value("suricata.eve.tls.issuerdn") {
                gsub_field(event, "suricata.eve.tls.issuerdn", "suricata.eve.tls.issuerdn", cached_regex!("\\\\,"), "")?;
            }

            if event.has_value("suricata.eve.tls.issuerdn") {
                if let Some(kv_str) = event.get_string("suricata.eve.tls.issuerdn") {
                    for pair in cached_regex!(", (?=[a-zA-Z]+=)").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "suricata.eve.tls.issuerdn".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("suricata.eve.tls.kv_issuerdn.{}", key), value)?;
                            }
                        }
                    }
                }
            }

                if event.has_value("suricata.eve.tls.kv_issuerdn.C") {
                    event.rename("suricata.eve.tls.kv_issuerdn.C", "tls.server.x509.issuer.country")?;
                }

            let _cond = { event.get("tls.server.x509.issuer.country").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.issuer.country", Value::Array(vec![json!(event.get("tls.server.x509.issuer.country").map_or_else(String::new, template_to_string))]))?;
            }

                if event.has_value("suricata.eve.tls.kv_issuerdn.CN") {
                    event.rename("suricata.eve.tls.kv_issuerdn.CN", "tls.server.x509.issuer.common_name")?;
                }

            let _cond = { event.get("tls.server.x509.issuer.common_name").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.issuer.common_name", Value::Array(vec![json!(event.get("tls.server.x509.issuer.common_name").map_or_else(String::new, template_to_string))]))?;
            }

                if event.has_value("suricata.eve.tls.kv_issuerdn.L") {
                    event.rename("suricata.eve.tls.kv_issuerdn.L", "tls.server.x509.issuer.locality")?;
                }

            let _cond = { event.get("tls.server.x509.issuer.locality").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.issuer.locality", Value::Array(vec![json!(event.get("tls.server.x509.issuer.locality").map_or_else(String::new, template_to_string))]))?;
            }

                if event.has_value("suricata.eve.tls.kv_issuerdn.O") {
                    event.rename("suricata.eve.tls.kv_issuerdn.O", "tls.server.x509.issuer.organization")?;
                }

            let _cond = { event.get("tls.server.x509.issuer.organization").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.issuer.organization", Value::Array(vec![json!(event.get("tls.server.x509.issuer.organization").map_or_else(String::new, template_to_string))]))?;
            }

                if event.has_value("suricata.eve.tls.kv_issuerdn.OU") {
                    event.rename("suricata.eve.tls.kv_issuerdn.OU", "tls.server.x509.issuer.organizational_unit")?;
                }

            let _cond = { event.get("tls.server.x509.issuer.organizational_unit").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.issuer.organizational_unit", Value::Array(vec![json!(event.get("tls.server.x509.issuer.organizational_unit").map_or_else(String::new, template_to_string))]))?;
            }

                if event.has_value("suricata.eve.tls.kv_issuerdn.ST") {
                    event.rename("suricata.eve.tls.kv_issuerdn.ST", "tls.server.x509.issuer.state_or_province")?;
                }

            let _cond = { event.get("tls.server.x509.issuer.state_or_province").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("tls.server.x509.issuer.state_or_province", Value::Array(vec![json!(event.get("tls.server.x509.issuer.state_or_province").map_or_else(String::new, template_to_string))]))?;
            }

            if event.has_value("suricata.eve.tls.session_resumed") {
                if let Some(val) = event.get("suricata.eve.tls.session_resumed") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "suricata.eve.tls.session_resumed".into(),
                            message,
                        })?;
                    event.set("tls.resumed", converted)?;
                }
            }

            let v = json!(event.get("suricata.eve.tls.fingerprint").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("tls.server.hash.sha1", v)?;
            }

            if event.has_value("tls.server.hash.sha1") {
                map_strings(event, "tls.server.hash.sha1", "tls.server.hash.sha1", str::to_uppercase)?;
            }

            if event.has_value("tls.server.hash.sha1") {
                if let Some(s) = event.get_string("tls.server.hash.sha1") {
                    let parts: Vec<Value> = s.split(":").map(|p| json!(p)).collect();
                    event.set("tls.server.hash.sha1", Value::Array(parts))?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let joined = event.get("tls.server.hash.sha1").and_then(|v| join_values(v, ""));
                if let Some(joined) = joined {
                    event.set("tls.server.hash.sha1", json!(joined))?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("tls.server.hash.sha1") };
            if _cond {
                event.append("related.hash", json!(event.get("tls.server.hash.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let v = json!(event.get("suricata.eve.tls.sni").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("tls.client.server_name", v)?;
            }

            let v = json!(event.get("suricata.eve.tls.sni").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("destination.domain", v)?;
            }

            let _cond = { event.has_value("suricata.eve.tls.sni") && event.get_str("suricata.eve.tls.sni") != Some("") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("suricata.eve.tls.sni").map_or_else(String::new, template_to_string)))?;
            }

            let v = json!(event.get("suricata.eve.tls.ja3s.hash").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("tls.server.ja3s", v)?;
            }

            let v = json!(event.get("suricata.eve.tls.ja3.hash").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("tls.client.ja3", v)?;
            }

            let v = json!(event.get("suricata.eve.tls.certificate").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("tls.server.certificate", v)?;
            }

            let v = json!(event.get("suricata.eve.tls.chain").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("tls.server.certificate_chain", v)?;
            }

            let v = json!(event.get("suricata.eve.tls.serial").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("tls.server.x509.serial_number", v)?;
            }

            if event.has_value("tls.server.x509.serial_number") {
                gsub_field(event, "tls.server.x509.serial_number", "tls.server.x509.serial_number", cached_regex!(":"), "")?;
            }

            let _cond = { event.has_value("suricata.eve.tls.notafter") };
            if _cond {
                if let Some(date_str) = event.get_as_string("suricata.eve.tls.notafter") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("tls.server.not_after", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "suricata.eve.tls.notafter".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("suricata.eve.tls.notbefore") };
            if _cond {
                if let Some(date_str) = event.get_as_string("suricata.eve.tls.notbefore") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("tls.server.not_before", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "suricata.eve.tls.notbefore".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let v = json!(event.get("tls.server.not_after").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("tls.server.x509.not_after", v)?;
            }

            let v = json!(event.get("tls.server.not_before").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("tls.server.x509.not_before", v)?;
            }

                event.remove("suricata.eve.tls.kv_issuerdn");
                event.remove("suricata.eve.tls.kv_subject");

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
