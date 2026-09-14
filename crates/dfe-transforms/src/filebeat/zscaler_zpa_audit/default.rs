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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            let _cond = {
                event.has_value("json.ModifiedTime")
                    && event.get_str("json.ModifiedTime") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.ModifiedTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.ModifiedTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.ModifiedTime").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.ModifiedTime".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
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
                event.get_str("json.ModifiedTime") == Some("")
                    && event.has_value("json.CreationTime")
                    && event.get_str("json.CreationTime") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.CreationTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.CreationTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.CreationTime").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.CreationTime".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
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
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("event.created", v)?;
                }
                Ok(())
            })();

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("json.AuditOperationType")
                    && event.get_str("json.AuditOperationType") != Some("")
            };
            if _cond {
                // Painless script
                // Source: def class = params.event_classification[ctx.json.AuditOperationType?.toLowerCase()];\nif (class == null) {\n  return;\n}\nctx.event.type = class.type;\nctx.event.category = class.category;\nctx.event.outcome = class.outcome;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def class = params.event_classification[ctx.json.AuditOperationType?.toLowerCase()];\nif (class == null) {\n  return;\n}\nctx.event.type = class.type;\nctx.event.category = class.category;\nctx.event.outcome = class.outcome;\n"#
                    ),
                    cached_params!(
                        "{\"event_classification\":{\"create\":{\"type\":[\"creation\"],\"category\":[\"iam\"],\"outcome\":\"success\"},\"delete\":{\"type\":[\"deletion\"],\"category\":[\"iam\"],\"outcome\":\"success\"},\"update\":{\"type\":[\"change\"],\"category\":[\"iam\"],\"outcome\":\"success\"},\"sign in\":{\"type\":[\"start\"],\"category\":[\"authentication\",\"session\"],\"outcome\":\"success\"},\"sign in failure\":{\"type\":[\"start\",\"error\"],\"category\":\"failure - session\",\"outcome\":null},\"download\":{\"type\":[\"info\",\"access\"],\"outcome\":\"success\",\"category\":[\"file\"]},\"sign out\":{\"type\":[\"end\"],\"category\":[\"session\"],\"outcome\":\"success\"},\"client session revoked\":{\"type\":[\"change\",\"deletion\"],\"category\":[\"iam\"],\"outcome\":\"success\"}}}"
                    ),
                )?;
            }

            if event.has_value("json.RequestID") {
                event.rename("json.RequestID", "event.id")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.CustomerID") {
                    if let Some(val) = event.get("json.CustomerID") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.CustomerID".into(),
                                message,
                            }
                        })?;
                        event.set("organization.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.CustomerID").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.CustomerID".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.ModifiedBy") {
                event.rename("json.ModifiedBy", "user.id")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("user.id") {
                    if let Some(val) = event.get("user.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "user.id".into(),
                                message,
                            }
                        })?;
                        event.set("user.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("user.id").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "user.id".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.User") {
                event.rename("json.User", "user.name")?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("user.name").cloned() {
                    event.set("user.email", v)?;
                }
            }

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

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.SessionID") {
                event.rename("json.SessionID", "zscaler_zpa.audit.session.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "json.AuditOldValue", "json.AuditOldValue")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "json.AuditNewValue", "json.AuditNewValue")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.AuditOldValue").cloned() {
                    event.set("zscaler_zpa.audit.value.old", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.AuditNewValue").cloned() {
                    event.set("zscaler_zpa.audit.value.new", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.ObjectType").cloned() {
                    event.set("zscaler_zpa.audit.object.type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.ObjectName").cloned() {
                    event.set("zscaler_zpa.audit.object.name", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.ClientAuditUpdate") {
                event.rename(
                    "json.ClientAuditUpdate",
                    "zscaler_zpa.audit.client_audit_update",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ObjectID") {
                    if let Some(val) = event.get("json.ObjectID") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ObjectID".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zpa.audit.object.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ObjectID").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ObjectID".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.ObjectType") };
            if _cond {
                // Painless script
                // Source: def objectType = ctx.json.ObjectType?.toLowerCase();\ndef operationType = ctx.json.AuditOperationType?.toLowerCase();\ndef valuesMap;\nif (operationType == 'delete' || operationType == 'sign out') {\n  valuesMap = ctx.json.AuditOldValue;\n} else if (operationType == 'create' || operationType == 'sign in' || operationType == 'update') {\n  valuesMap = ctx.json.AuditNewValue;\n}\n\nif (objectType == 'administrator') {\n  ctx.user.target = new HashMap();\n  ctx.user.target.roles = new ArrayList();\n  def roles = (valuesMap?.roles == null) ? [] : new ArrayList(valuesMap?.roles);\n  ctx.user.target.email = valuesMap?.email;\n  for (int i = 0; i < roles.length; i++) {\n    ctx.user.target.roles.add(roles[i].name);\n  }\n} else if (objectType == 'app connector group') {\n  ctx.group = new HashMap();\n  ctx.group.id = valuesMap?.id;\n  ctx.group.name = valuesMap?.name;\n  ctx.observer = new HashMap();\n  ctx.observer.geo = new HashMap();\n  ctx.observer.geo.location = new HashMap();\n  ctx.observer.geo.location.lat = valuesMap?.latitude;\n  ctx.observer.geo.location.lon = valuesMap?.longitude;\n  ctx.observer.geo.city_name = valuesMap?.cityCountry;\n  ctx.observer.geo.country_name = valuesMap?.location;\n} else if (objectType == 'browser access') {\n  ctx.network = new HashMap();\n  ctx.network.protocol = valuesMap?.applicationProtocol?.toLowerCase();\n} else if (objectType == 'authentication') {\n  ctx.client = new HashMap();\n  ctx.client.ip = valuesMap?.remoteIP;\n} else if (objectType == 'certificate') {\n  ctx.x509 = new HashMap();\n  ctx.x509.issuer = new HashMap();\n  ctx.x509.alternative_names = valuesMap?.subjectAlternateNames;\n  ctx.x509.issuer.common_name = valuesMap?.commonName;\n  ctx.x509.issuer.distinguished_name = valuesMap?.issuedTo;\n} else if (objectType == 'executive insights user') {\n  ctx.user = new HashMap();\n  ctx.user.target = new HashMap();\n  ctx.user.target.id = valuesMap?.id;\n  ctx.user.target.email = valuesMap?.email;\n  ctx.user.target.name = valuesMap?.name;\n} else if (objectType == 'idp certificate') {\n  ctx.x509 = new HashMap();\n  ctx.x509.issuer = new HashMap();\n  if (valuesMap?.creationTimeInSeconds != null) {\n    ctx.x509.not_before = Long.parseLong(valuesMap?.creationTimeInSeconds);\n  }\n  if (valuesMap?.expirationTimeInSeconds != null) {\n    ctx.x509.not_after = Long.parseLong(valuesMap?.expirationTimeInSeconds);\n  }\n  ctx.x509.issuer.common_name = valuesMap?.commonName;\n} else if (objectType == 'server') {\n  ctx.server = new HashMap();\n  ctx.server.address = valuesMap?.domainOrIpAddress;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def objectType = ctx.json.ObjectType?.toLowerCase();\ndef operationType = ctx.json.AuditOperationType?.toLowerCase();\ndef valuesMap;\nif (operationType == 'delete' || operationType == 'sign out') {\n  valuesMap = ctx.json.AuditOldValue;\n} else if (operationType == 'create' || operationType == 'sign in' || operationType == 'update') {\n  valuesMap = ctx.json.AuditNewValue;\n}\n\nif (objectType == 'administrator') {\n  ctx.user.target = new HashMap();\n  ctx.user.target.roles = new ArrayList();\n  def roles = (valuesMap?.roles == null) ? [] : new ArrayList(valuesMap?.roles);\n  ctx.user.target.email = valuesMap?.email;\n  for (int i = 0; i < roles.length; i++) {\n    ctx.user.target.roles.add(roles[i].name);\n  }\n} else if (objectType == 'app connector group') {\n  ctx.group = new HashMap();\n  ctx.group.id = valuesMap?.id;\n  ctx.group.name = valuesMap?.name;\n  ctx.observer = new HashMap();\n  ctx.observer.geo = new HashMap();\n  ctx.observer.geo.location = new HashMap();\n  ctx.observer.geo.location.lat = valuesMap?.latitude;\n  ctx.observer.geo.location.lon = valuesMap?.longitude;\n  ctx.observer.geo.city_name = valuesMap?.cityCountry;\n  ctx.observer.geo.country_name = valuesMap?.location;\n} else if (objectType == 'browser access') {\n  ctx.network = new HashMap();\n  ctx.network.protocol = valuesMap?.applicationProtocol?.toLowerCase();\n} else if (objectType == 'authentication') {\n  ctx.client = new HashMap();\n  ctx.client.ip = valuesMap?.remoteIP;\n} else if (objectType == 'certificate') {\n  ctx.x509 = new HashMap();\n  ctx.x509.issuer = new HashMap();\n  ctx.x509.alternative_names = valuesMap?.subjectAlternateNames;\n  ctx.x509.issuer.common_name = valuesMap?.commonName;\n  ctx.x509.issuer.distinguished_name = valuesMap?.issuedTo;\n} else if (objectType == 'executive insights user') {\n  ctx.user = new HashMap();\n  ctx.user.target = new HashMap();\n  ctx.user.target.id = valuesMap?.id;\n  ctx.user.target.email = valuesMap?.email;\n  ctx.user.target.name = valuesMap?.name;\n} else if (objectType == 'idp certificate') {\n  ctx.x509 = new HashMap();\n  ctx.x509.issuer = new HashMap();\n  if (valuesMap?.creationTimeInSeconds != null) {\n    ctx.x509.not_before = Long.parseLong(valuesMap?.creationTimeInSeconds);\n  }\n  if (valuesMap?.expirationTimeInSeconds != null) {\n    ctx.x509.not_after = Long.parseLong(valuesMap?.expirationTimeInSeconds);\n  }\n  ctx.x509.issuer.common_name = valuesMap?.commonName;\n} else if (objectType == 'server') {\n  ctx.server = new HashMap();\n  ctx.server.address = valuesMap?.domainOrIpAddress;\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.get("client.ip").is_some_and(|v| v.is_string())
                    && event.get("client.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(s) = event.get_string("client.ip") {
                    let mut parts: Vec<Value> = cached_regex!(", *")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("client.ip", Value::Array(parts))?;
                }
            }

            let _cond = { event.get("client.ip").is_some_and(|v| v.is_string()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.get("client.ip").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "client.ip", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("server.address") {
                    if let Some(val) = event.get("server.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "server.address".into(),
                                message,
                            }
                        })?;
                        event.set("server.ip", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("server.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("server.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("x509.not_after") && event.get_str("x509.not_after") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("x509.not_after") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("x509.not_after", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "x509.not_after".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("x509.not_after").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "x509.not_after".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
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
                event.has_value("x509.not_before") && event.get_str("x509.not_before") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("x509.not_before") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("x509.not_before", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "x509.not_before".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("x509.not_before").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "x509.not_before".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.AuditOperationType") {
                event.rename(
                    "json.AuditOperationType",
                    "zscaler_zpa.audit.operation_type",
                )?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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

            event.remove("json.ObjectType");
            event.remove("json.ModifiedTime");
            event.remove("json.AuditOldValue");
            event.remove("json.AuditNewValue");
            event.remove("json.ObjectName");
            event.remove("json.CreationTime");
            event.remove("json.CustomerID");
            event.remove("json.ObjectID");

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.zscaler_zpa?.audit[m.getKey()] = m.getValue();\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.zscaler_zpa?.audit[m.getKey()] = m.getValue();\n}\n"#
                    ),
                )?;
            }

            event.remove("json");

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
