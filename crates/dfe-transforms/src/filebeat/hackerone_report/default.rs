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
            event.set("ecs.version", json!("9.3.0"))?;

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

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

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "hackerone.report")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "parse_json_to_hackerone_report",
                    )?;
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

            event.set("observer.vendor", json!("HackerOne"))?;

            event.set("observer.product", json!("HackerOne"))?;

            event.set("vulnerability.scanner.vendor", json!("HackerOne"))?;

            if event.has_value("hackerone.report.id") {
                if let Some(val) = event.get("hackerone.report.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "hackerone.report.id".into(),
                            message,
                        }
                    })?;
                    event.set("hackerone.report.id", converted)?;
                }
            }

            if event.has_value("hackerone.report.id") {
                event.rename("hackerone.report.id", "event.id")?;
            }

            let _cond = { event.has_value("event.id") };
            if _cond {
                event.set(
                    "event.url",
                    json!(format!(
                        "https://hackerone.com/reports/{}",
                        event
                            .get("event.id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            if let Some(v) = event
                .get("event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.report_id", v)?;
            }

            let _cond = { event.has_value("event.id") };
            if _cond {
                event.set(
                    "vulnerability.reference",
                    json!(format!(
                        "https://hackerone.com/reports/{}",
                        event
                            .get("event.id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            if let Some(v) = event
                .get("hackerone.report.attributes.title")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has_value("hackerone.report.attributes.vulnerability_information") {
                event.rename(
                    "hackerone.report.attributes.vulnerability_information",
                    "vulnerability.description",
                )?;
            }

            // SKIPPED: condition not transpiled: ctx.hackerone?.report?.attributes?.cve_ids instanceof List && ((List) ctx.hackerone.report.attributes.cve_ids).isEmpty() == false && ((List) ctx.hackerone.report.attributes.cve_ids).get(0) != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set(
                    "vulnerability.id",
                    json!(
                        event
                            .get("hackerone.report.attributes.cve_ids.0")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("hackerone.report.relationships.severity.data.attributes.rating") {
                event.rename(
                    "hackerone.report.relationships.severity.data.attributes.rating",
                    "vulnerability.severity",
                )?;
            }

            if event.has_value("hackerone.report.relationships.severity.data.attributes.score") {
                event.rename(
                    "hackerone.report.relationships.severity.data.attributes.score",
                    "vulnerability.score.base",
                )?;
            }

            if event.has_value("vulnerability.score.base") {
                if let Some(val) = event.get("vulnerability.score.base") {
                    let converted = convert_value(val, "float").map_err(|message| {
                        TransformError::ParseError {
                            path: "vulnerability.score.base".into(),
                            message,
                        }
                    })?;
                    event.set("vulnerability.score.base", converted)?;
                }
            }

            let _cond = {
                event.get_str(
                    "hackerone.report.relationships.severity.data.attributes.calculation_method",
                ) == Some("cvss_3_0_hackerone")
            };
            if _cond {
                event.set("vulnerability.score.version", json!("3.0"))?;
            }

            let _cond = {
                event.get_str(
                    "hackerone.report.relationships.severity.data.attributes.calculation_method",
                ) == Some("cvss_3_1")
            };
            if _cond {
                event.set("vulnerability.score.version", json!("3.1"))?;
            }

            let _cond = {
                event.get_str("hackerone.report.relationships.severity.data.attributes.calculation_method") == Some("cvss_4_0") || event.get_str("hackerone.report.relationships.severity.data.attributes.calculation_method") == Some("cvss_4_0_metric_set")
            };
            if _cond {
                event.set("vulnerability.score.version", json!("4.0"))?;
            }

            if event.has_value("hackerone.report.relationships.weakness.data.attributes.name") {
                event.rename(
                    "hackerone.report.relationships.weakness.data.attributes.name",
                    "vulnerability.classification",
                )?;
            }

            // SKIPPED: condition not transpiled: ctx.hackerone?.report?.relationships?.weakness?.data?.attributes?.external_id != null && ((String) ctx.hackerone.report.relationships.weakness.data.attributes.external_id).toLowerCase().startsWith('cw ...
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("vulnerability.enumeration", json!("CWE"))?;
            }

            // SKIPPED: condition not transpiled: ctx.hackerone?.report?.relationships?.weakness?.data?.attributes?.external_id != null && ((String) ctx.hackerone.report.relationships.weakness.data.attributes.external_id).toLowerCase().startsWith('ca ...
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("vulnerability.enumeration", json!("CAPEC"))?;
            }

            if event.has_value("hackerone.report.relationships.reporter.data.id") {
                event.rename("hackerone.report.relationships.reporter.data.id", "user.id")?;
            }

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

            if event.has_value("hackerone.report.relationships.reporter.data.attributes.username") {
                event.rename(
                    "hackerone.report.relationships.reporter.data.attributes.username",
                    "user.name",
                )?;
            }

            if event.has_value("hackerone.report.relationships.reporter.data.attributes.name") {
                event.rename(
                    "hackerone.report.relationships.reporter.data.attributes.name",
                    "user.full_name",
                )?;
            }

            if event.has_value("hackerone.report.relationships.program.data.id") {
                event.rename(
                    "hackerone.report.relationships.program.data.id",
                    "organization.id",
                )?;
            }

            if event.has_value("organization.id") {
                if let Some(val) = event.get("organization.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "organization.id".into(),
                            message,
                        }
                    })?;
                    event.set("organization.id", converted)?;
                }
            }

            if event.has_value("hackerone.report.relationships.program.data.attributes.handle") {
                event.rename(
                    "hackerone.report.relationships.program.data.attributes.handle",
                    "organization.name",
                )?;
            }

            // SKIPPED: condition not transpiled: ctx.hackerone?.report?.relationships?.structured_scope?.data?.attributes?.asset_identifier != null && ctx.hackerone.report.relationships.structured_scope.data.attributes.asset_type instanceof String & ...
            #[allow(unreachable_code, unused_variables)]
            if false {
                if event.has_value("hackerone.report.relationships.structured_scope.data.attributes.asset_identifier") {
                    event.rename("hackerone.report.relationships.structured_scope.data.attributes.asset_identifier", "url.original")?;
                }
            }

            let _cond = { event.has_value("url.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "url.original", "_temp.url_parts", true, false)?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.url_parts.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("_temp.url_parts.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // SKIPPED: condition not transpiled: ctx.hackerone?.report?.relationships?.structured_scope?.data?.attributes?.asset_identifier != null && ctx.hackerone.report.relationships.structured_scope.data.attributes.asset_type instanceof String & ...
            #[allow(unreachable_code, unused_variables)]
            if false {
                if event.has_value("hackerone.report.relationships.structured_scope.data.attributes.asset_identifier") {
                    event.rename("hackerone.report.relationships.structured_scope.data.attributes.asset_identifier", "_temp.domain_host")?;
                }
            }

            if event.has_value("_temp.domain_host") {
                gsub_field(
                    event,
                    "_temp.domain_host",
                    "_temp.domain_host",
                    cached_regex!("^\\*\\."),
                    "",
                )?;
            }

            let _cond = {
                event.has_value("_temp.domain_host")
                    && event.get_str("_temp.domain_host") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("_temp.domain_host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Painless script
            // Source: LinkedHashSet users = new LinkedHashSet();\nif (ctx.user?.name != null) {\n  users.add(String.valueOf(ctx.user.name));\n}\nMap rels = ctx.hackerone?.report?.relationships instanceof Map ? (Map) ctx.hackerone.report.relationships : null;\nif (rels != null) {\n  if (rels.assignee instanceof Map && ((Map) rels.assignee).data instanceof Map) {\n    Map asgData = (Map) ((Map) rels.assignee).data;\n    if ('user'.equals(asgData.type) && asgData.attributes instanceof Map) {\n      Object u = ((Map) asgData.attributes).username;\n      if (u != null) users.add(String.valueOf(u));\n    }\n  }\n  if (rels.collaborators instanceof Map && ((Map) rels.collaborators).data instanceof List) {\n    for (Object r : (List) ((Map) rels.collaborators).data) {\n      if (r instanceof Map) {\n        Map row = (Map) r;\n        if (row.user instanceof Map && ((Map) row.user).attributes instanceof Map) {\n          Object u = ((Map) ((Map) row.user).attributes).username;\n          if (u != null) users.add(String.valueOf(u));\n        }\n      }\n    }\n  }\n  if (rels.summaries instanceof Map && ((Map) rels.summaries).data instanceof List) {\n    for (Object r : (List) ((Map) rels.summaries).data) {\n      if (r instanceof Map) {\n        Map row = (Map) r;\n        if (row.relationships instanceof Map) {\n          Map srels = (Map) row.relationships;\n          if (srels.user instanceof Map && ((Map) srels.user).data instanceof Map) {\n            Map udata = (Map) ((Map) srels.user).data;\n            if (udata.attributes instanceof Map) {\n              Object u = ((Map) udata.attributes).username;\n              if (u != null) users.add(String.valueOf(u));\n            }\n          }\n        }\n      }\n    }\n  }\n  if (rels.custom_remediation_guidance instanceof Map) {\n    Map crg = (Map) rels.custom_remediation_guidance;\n    if (crg.data instanceof Map) {\n      Map cdata = (Map) crg.data;\n      if (cdata.relationships instanceof Map) {\n        Map crels = (Map) cdata.relationships;\n        if (crels.author instanceof Map && ((Map) crels.author).data instanceof Map) {\n          Map adata = (Map) ((Map) crels.author).data;\n          if (adata.attributes instanceof Map) {\n            Object u = ((Map) adata.attributes).username;\n            if (u != null) users.add(String.valueOf(u));\n          }\n        }\n      }\n    }\n  }\n}\nif (users.isEmpty()) {\n  return;\n}\nif (ctx.related == null) {\n  ctx.related = new HashMap();\n}\nMap related = (Map) ctx.related;\nArrayList existing = related.user instanceof List ? new ArrayList((List) related.user) : new ArrayList();\nfor (Object u : users) {\n  if (existing.contains(u) == false) {\n    existing.add(u);\n  }\n}\nrelated.user = existing;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"LinkedHashSet users = new LinkedHashSet();\nif (ctx.user?.name != null) {\n  users.add(String.valueOf(ctx.user.name));\n}\nMap rels = ctx.hackerone?.report?.relationships instanceof Map ? (Map) ctx.hackerone.report.relationships : null;\nif (rels != null) {\n  if (rels.assignee instanceof Map && ((Map) rels.assignee).data instanceof Map) {\n    Map asgData = (Map) ((Map) rels.assignee).data;\n    if ('user'.equals(asgData.type) && asgData.attributes instanceof Map) {\n      Object u = ((Map) asgData.attributes).username;\n      if (u != null) users.add(String.valueOf(u));\n    }\n  }\n  if (rels.collaborators instanceof Map && ((Map) rels.collaborators).data instanceof List) {\n    for (Object r : (List) ((Map) rels.collaborators).data) {\n      if (r instanceof Map) {\n        Map row = (Map) r;\n        if (row.user instanceof Map && ((Map) row.user).attributes instanceof Map) {\n          Object u = ((Map) ((Map) row.user).attributes).username;\n          if (u != null) users.add(String.valueOf(u));\n        }\n      }\n    }\n  }\n  if (rels.summaries instanceof Map && ((Map) rels.summaries).data instanceof List) {\n    for (Object r : (List) ((Map) rels.summaries).data) {\n      if (r instanceof Map) {\n        Map row = (Map) r;\n        if (row.relationships instanceof Map) {\n          Map srels = (Map) row.relationships;\n          if (srels.user instanceof Map && ((Map) srels.user).data instanceof Map) {\n            Map udata = (Map) ((Map) srels.user).data;\n            if (udata.attributes instanceof Map) {\n              Object u = ((Map) udata.attributes).username;\n              if (u != null) users.add(String.valueOf(u));\n            }\n          }\n        }\n      }\n    }\n  }\n  if (rels.custom_remediation_guidance instanceof Map) {\n    Map crg = (Map) rels.custom_remediation_guidance;\n    if (crg.data instanceof Map) {\n      Map cdata = (Map) crg.data;\n      if (cdata.relationships instanceof Map) {\n        Map crels = (Map) cdata.relationships;\n        if (crels.author instanceof Map && ((Map) crels.author).data instanceof Map) {\n          Map adata = (Map) ((Map) crels.author).data;\n          if (adata.attributes instanceof Map) {\n            Object u = ((Map) adata.attributes).username;\n            if (u != null) users.add(String.valueOf(u));\n          }\n        }\n      }\n    }\n  }\n}\nif (users.isEmpty()) {\n  return;\n}\nif (ctx.related == null) {\n  ctx.related = new HashMap();\n}\nMap related = (Map) ctx.related;\nArrayList existing = related.user instanceof List ? new ArrayList((List) related.user) : new ArrayList();\nfor (Object u : users) {\n  if (existing.contains(u) == false) {\n    existing.add(u);\n  }\n}\nrelated.user = existing;"#
                ),
            )?;

            let _cond = { event.has_value("hackerone.report.attributes.last_activity_at") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("hackerone.report.attributes.last_activity_at")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "hackerone.report.attributes.last_activity_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            // SKIPPED: condition not transpiled: (!(ctx.containsKey('@timestamp')) || ctx['@timestamp'] == null) && ctx.hackerone?.report?.attributes?.created_at != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                if let Some(date_str) =
                    event.get_as_string("hackerone.report.attributes.created_at")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "hackerone.report.attributes.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("hackerone.report.attributes.created_at") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("hackerone.report.attributes.created_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("event.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "hackerone.report.attributes.created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("hackerone.report.attributes.created_at") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("hackerone.report.attributes.created_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("event.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "hackerone.report.attributes.created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("hackerone.report.attributes.last_activity_at") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("hackerone.report.attributes.last_activity_at")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("event.end", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "hackerone.report.attributes.last_activity_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("hackerone.report.relationships.bounties.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("hackerone.report.relationships.bounties.data") {
                    foreach_array(
                        event,
                        "hackerone.report.relationships.bounties.data",
                        |event| {
                            if event.has_value("_ingest._value.amount") {
                                if let Some(val) = event.get("_ingest._value.amount") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.amount".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.amount", converted)?;
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("hackerone.report.relationships.bounties.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("hackerone.report.relationships.bounties.data") {
                    foreach_array(
                        event,
                        "hackerone.report.relationships.bounties.data",
                        |event| {
                            if event.has_value("_ingest._value.bonus_amount") {
                                if let Some(val) = event.get("_ingest._value.bonus_amount") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.bonus_amount".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.bonus_amount", converted)?;
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("hackerone.report.relationships.bounties.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("hackerone.report.relationships.bounties.data") {
                    foreach_array(
                        event,
                        "hackerone.report.relationships.bounties.data",
                        |event| {
                            if event.has_value("_ingest._value.awarded_amount") {
                                if let Some(val) = event.get("_ingest._value.awarded_amount") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.awarded_amount".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.awarded_amount", converted)?;
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("hackerone.report.relationships.bounties.data")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("hackerone.report.relationships.bounties.data") {
                    foreach_array(
                        event,
                        "hackerone.report.relationships.bounties.data",
                        |event| {
                            if event.has_value("_ingest._value.awarded_bonus_amount") {
                                if let Some(val) = event.get("_ingest._value.awarded_bonus_amount")
                                {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.awarded_bonus_amount".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.awarded_bonus_amount", converted)?;
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = { event.get("hackerone.report").is_some_and(|v| v.is_object()) };
            if _cond {
                // Painless script
                // Source: ArrayList stack = new ArrayList();\nstack.add(ctx.hackerone.report);\nwhile (stack.isEmpty() == false) {\n  Object node = stack.remove(stack.size() - 1);\n  if (node instanceof Map) {\n    Map current = (Map) node;\n    ArrayList keys = new ArrayList(current.keySet());\n    for (Object k : keys) {\n      Object v = current.get(k);\n      if (v == null) {\n        current.remove(k);\n      } else if (v instanceof Map || v instanceof List) {\n        stack.add(v);\n      }\n    }\n  } else if (node instanceof List) {\n    for (Object item : (List) node) {\n      if (item instanceof Map || item instanceof List) {\n        stack.add(item);\n      }\n    }\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ArrayList stack = new ArrayList();\nstack.add(ctx.hackerone.report);\nwhile (stack.isEmpty() == false) {\n  Object node = stack.remove(stack.size() - 1);\n  if (node instanceof Map) {\n    Map current = (Map) node;\n    ArrayList keys = new ArrayList(current.keySet());\n    for (Object k : keys) {\n      Object v = current.get(k);\n      if (v == null) {\n        current.remove(k);\n      } else if (v instanceof Map || v instanceof List) {\n        stack.add(v);\n      }\n    }\n  } else if (node instanceof List) {\n    for (Object item : (List) node) {\n      if (item instanceof Map || item instanceof List) {\n        stack.add(item);\n      }\n    }\n  }\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("hackerone.report.attributes.state") };
            if _cond {
                // Painless script
                // Source: Object stRaw = ctx.hackerone.report.attributes.state;\nString st = stRaw instanceof String ? (String) stRaw : String.valueOf(stRaw);\nif (!params.containsKey(st)) {\n  return;\n}\nMap m = params[st] instanceof Map ? (Map) params[st] : null;\nif (m == null) {\n  return;\n}\nMap ev = ctx.containsKey('event') && ctx.event instanceof Map ? (Map) ctx.event : new HashMap();\nev.put('kind', 'event');\nArrayList ec = new ArrayList();\nec.add('vulnerability');\nev.put('category', ec);\nArrayList et = new ArrayList();\net.add('info');\nev.put('type', et);\nev.put('outcome', m.get('outcome'));\nev.put('action', m.get('action'));\nctx.event = ev;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"Object stRaw = ctx.hackerone.report.attributes.state;\nString st = stRaw instanceof String ? (String) stRaw : String.valueOf(stRaw);\nif (!params.containsKey(st)) {\n  return;\n}\nMap m = params[st] instanceof Map ? (Map) params[st] : null;\nif (m == null) {\n  return;\n}\nMap ev = ctx.containsKey('event') && ctx.event instanceof Map ? (Map) ctx.event : new HashMap();\nev.put('kind', 'event');\nArrayList ec = new ArrayList();\nec.add('vulnerability');\nev.put('category', ec);\nArrayList et = new ArrayList();\net.add('info');\nev.put('type', et);\nev.put('outcome', m.get('outcome'));\nev.put('action', m.get('action'));\nctx.event = ev;"#
                    ),
                    cached_params!(
                        "{\"new\":{\"outcome\":\"unknown\",\"action\":\"report-new\"},\"pending-program-review\":{\"outcome\":\"unknown\",\"action\":\"report-pending-program-review\"},\"triaged\":{\"outcome\":\"unknown\",\"action\":\"report-triaged\"},\"needs-more-info\":{\"outcome\":\"unknown\",\"action\":\"report-needs-more-info\"},\"resolved\":{\"outcome\":\"success\",\"action\":\"report-resolved\"},\"not-applicable\":{\"outcome\":\"failure\",\"action\":\"report-not-applicable\"},\"informative\":{\"outcome\":\"unknown\",\"action\":\"report-informative\"},\"duplicate\":{\"outcome\":\"failure\",\"action\":\"report-duplicate\"},\"spam\":{\"outcome\":\"failure\",\"action\":\"report-spam\"},\"retesting\":{\"outcome\":\"unknown\",\"action\":\"report-retesting\"}}"
                    ),
                )?;
            }

            event.remove("_temp");

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
                        "Processor '{}' {}failed with message '{}'",
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
