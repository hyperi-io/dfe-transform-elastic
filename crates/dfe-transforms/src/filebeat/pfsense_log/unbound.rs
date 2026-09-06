// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `unbound` pipeline.
pub struct Unbound;

impl Transform for Unbound {
    fn name(&self) -> &str {
        "unbound"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: %{LOGLEVEL:log.level}: %{IP:source.address} %{HOSTNAME:_tmp.question.name}(\\.) %{WORD:_tmp.question.type} %{WORD:_tmp.question.class}
                    if !cached_grok!("%{LOGLEVEL:log.level}: %{IP:source.address} %{HOSTNAME:_tmp.question.name}(\\.) %{WORD:_tmp.question.type} %{WORD:_tmp.question.class}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("_ingest.on_failure_processor_tag", "grok_message_88ceaee5")?;
                        return Ok(TransformResult::Drop);
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                event.append_unique("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("message").is_some_and(|s| s.to_lowercase().contains("disconnected")) };
            if _cond {
                event.append_unique("event.type", json!("end"))?;
            }

            event.set("network.protocol", json!("dns"))?;

            let _cond = { event.has_value("_tmp.question.name") };
            if _cond {
            event.set("dns.type", json!("question"))?;
            }

            if event.has_value("_tmp.question.name") {
                if let Some(domain_str) = event.get_string("_tmp.question.name") {
                    let domain = domain_str.to_string();
                    event.set("dns.question.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        if let Some(registered) = rd.registered_domain {
                            event.set("dns.question.registered_domain", json!(registered))?;
                        }
                        event.set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("dns.question.subdomain", json!(sub))?;
                        }
                    }
                }
            }

                if event.has_value("dns.question.domain") {
                    event.rename("dns.question.domain", "dns.question.name")?;
                }

                if event.has_value("_tmp.question.type") {
                    event.rename("_tmp.question.type", "dns.question.type")?;
                }

                if event.has_value("_tmp.question.class") {
                    event.rename("_tmp.question.class", "dns.question.class")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }
                Ok(())
            })();

            if let Some(v) = event.get("source").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client", v)?;
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
