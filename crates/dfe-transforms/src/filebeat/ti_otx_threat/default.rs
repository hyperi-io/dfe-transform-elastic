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

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "otx")?;

            let _cond = { event.get_i64("otx.count") == Some(0) };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if event.has_value("otx.results") {
                event.rename("otx.results", "results")?;
            }

            if event.remove("otx").is_none() {
                return Err(TransformError::FieldNotFound { path: "otx".into() });
            }

            if event.has_value("results") {
                event.rename("results", "otx")?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("otx.id") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "otx.id".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event
                    .get_str("otx.type")
                    .is_some_and(|s| s.starts_with("FileHash"))
                    || event.get_str("otx.type") == Some("filepath")
            };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = { event.get_str("otx.type") == Some("FileHash-MD5") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.file.hash.md5")?;
                }
            }

            let _cond = { event.has_value("threat.indicator.file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threat.indicator.file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("otx.type") == Some("FileHash-SHA1") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.file.hash.sha1")?;
                }
            }

            let _cond = { event.has_value("threat.indicator.file.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threat.indicator.file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("otx.type") == Some("FileHash-SHA256") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.file.hash.sha256")?;
                }
            }

            let _cond = { event.has_value("threat.indicator.file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threat.indicator.file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("otx.type") == Some("FileHash-PEHASH") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.file.hash.pehash")?;
                }
            }

            let _cond = { event.has_value("threat.indicator.file.hash.pehash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threat.indicator.file.hash.pehash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("otx.type") == Some("FileHash-IMPHASH") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.file.hash.imphash")?;
                }
            }

            let _cond = { event.has_value("threat.indicator.file.hash.imphash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("threat.indicator.file.hash.imphash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("otx.type") == Some("IPv4") };
            if _cond {
                event.set("threat.indicator.type", json!("ipv4-addr"))?;
            }

            let _cond = { event.get_str("otx.type") == Some("IPv6") };
            if _cond {
                event.set("threat.indicator.type", json!("ipv6-addr"))?;
            }

            let _cond = {
                event.has_value("threat.indicator.type")
                    && ["ipv4-addr", "ipv6-addr"]
                        .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.ip")?;
                }
            }

            let _cond = { event.has_value("threat.indicator.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("threat.indicator.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("threat.indicator.type")
                    && ["URL", "URI"].contains(&event.get_str("otx.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("url") };
            if _cond {
                uri_parts(event, "otx.indicator", "threat.indicator.url", true, true)?;
            }

            let _cond = { event.get_str("otx.type") == Some("URL") };
            if _cond {
                let v = json!(
                    event
                        .get("threat.indicator.url.original")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("threat.indicator.url.full", v)?;
                }
            }

            let _cond = { event.get_str("otx.type") == Some("email") };
            if _cond {
                event.set("threat.indicator.type", json!("email-addr"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("email-addr") };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.email.address")?;
                }
            }

            let _cond = {
                !event.has_value("threat.indicator.type")
                    && ["domain", "hostname"].contains(&event.get_str("otx.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("domain-name")
                    && !event.has_value("threat.indicator.url.domain")
            };
            if _cond {
                if event.has_value("otx.indicator") {
                    event.rename("otx.indicator", "threat.indicator.url.domain")?;
                }
            }

            let _cond = { !event.has_value("threat.indicator.type") };
            if _cond {
                event.set("threat.indicator.type", json!("unknown"))?;
            }

            let _cond = { event.has_value("otx") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\nmap.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                drop_empty(
                    event,
                    &DropPolicy {
                        nulls: true,
                        ..DropPolicy::none()
                    },
                    None,
                );
            }

            let _cond = { event.get_str("otx.content") == Some("") };
            if _cond {
                event.remove("otx.content");
            }

            let _cond = { event.has_value("threat.indicator.type") };
            if _cond {
                event.remove("otx.type");
                event.remove("otx.id");
                event.remove("message");
                event.remove("otx.count");
                event.remove("otx.next");
                event.remove("otx.previous");
            }

            let _cond = { !event.has_value("otx") };
            if _cond {
                event.remove("otx");
            }

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
