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

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "github.dependabot")?;

            let _cond = {
                !(event
                    .get("github.dependabot")
                    .is_some_and(|v| v.is_object()))
            };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("Missing JSON object").to_string(),
                });
            }

            event.set("event.kind", json!("alert"))?;

            event.set(
                "_temp.updated_at",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("github.dependabot.createdAt") {
                event.rename(
                    "github.dependabot.createdAt",
                    "github.dependabot.created_at",
                )?;
            }

            let _cond = { event.has_value("github.dependabot.created_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.dependabot.created_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.dependabot.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("github.dependabot.dependabotUpdate") {
                event.rename(
                    "github.dependabot.dependabotUpdate",
                    "github.dependabot.dependabot_update",
                )?;
            }

            if event.has_value("github.dependabot.dependabot_update.error.errorType") {
                event.rename(
                    "github.dependabot.dependabot_update.error.errorType",
                    "github.dependabot.dependabot_update.error.error_type",
                )?;
            }

            if event.has_value("github.dependabot.dependencyScope") {
                event.rename(
                    "github.dependabot.dependencyScope",
                    "github.dependabot.dependency_scope",
                )?;
            }

            if event.has_value("github.dependabot.dismissReason") {
                event.rename(
                    "github.dependabot.dismissReason",
                    "github.dependabot.dismiss_reason",
                )?;
            }

            if event.has_value("github.dependabot.dismissedAt") {
                event.rename(
                    "github.dependabot.dismissedAt",
                    "github.dependabot.dismissed_at",
                )?;
            }

            if event.has_value("github.dependabot.fixedAt") {
                event.rename("github.dependabot.fixedAt", "github.dependabot.fixed_at")?;
            }

            let _cond = { event.has_value("github.dependabot.created_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.dependabot.created_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.dependabot.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("github.dependabot.dismissed_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.dependabot.dismissed_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.dependabot.dismissed_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("github.dependabot.fixed_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("github.dependabot.fixed_at") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "github.dependabot.fixed_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                !event.has_value("github.dependabot.fixed_at")
                    && !event.has_value("github.dependabot.dismissed_at")
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.has_value("github.dependabot.fixed_at")
                    || event.has_value("github.dependabot.dismissed_at")
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            if event.has_value("github.dependabot.repository.isInOrganization") {
                event.rename(
                    "github.dependabot.repository.isInOrganization",
                    "github.dependabot.repository.is_in_organization",
                )?;
            }

            if event.has_value("github.dependabot.repository.isPrivate") {
                event.rename(
                    "github.dependabot.repository.isPrivate",
                    "github.dependabot.repository.private",
                )?;
            }

            if event.has_value("github.dependabot.securityAdvisory") {
                event.rename(
                    "github.dependabot.securityAdvisory",
                    "github.dependabot.security_advisory",
                )?;
            }

            if event.has_value("github.dependabot.security_advisory.cvss.vectorString") {
                event.rename(
                    "github.dependabot.security_advisory.cvss.vectorString",
                    "github.dependabot.security_advisory.cvss.vector_string",
                )?;
            }

            if event.has_value("github.dependabot.security_advisory.ghsaId") {
                event.rename(
                    "github.dependabot.security_advisory.ghsaId",
                    "github.dependabot.security_advisory.ghsa_id",
                )?;
            }

            if event.has_value("github.dependabot.security_advisory.publishedAt") {
                event.rename(
                    "github.dependabot.security_advisory.publishedAt",
                    "github.dependabot.security_advisory.published_at",
                )?;
            }

            if event.has_value("github.dependabot.security_advisory.updatedAt") {
                event.rename(
                    "github.dependabot.security_advisory.updatedAt",
                    "github.dependabot.security_advisory.updated_at",
                )?;
            }

            let _cond = { event.has_value("github.dependabot.security_advisory.cwes.nodes") };
            if _cond {
                if let Some(v) = event
                    .get("github.dependabot.security_advisory.cwes.nodes")
                    .cloned()
                {
                    event.set("_temp.cwes", v)?;
                }
            }

            event.remove("github.dependabot.security_advisory.cwes");

            let _cond = { event.has_value("_temp.cwes") };
            if _cond {
                if let Some(v) = event.get("_temp.cwes").cloned() {
                    event.set("github.dependabot.security_advisory.cwes", v)?;
                }
            }

            let _cond = {
                event
                    .get("github.dependabot.security_advisory.cwes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "github.dependabot.security_advisory.cwes", |event| {
                        if event.has_value("_ingest._value.cweId") {
                            event.rename("_ingest._value.cweId", "_ingest._value.cwe_id")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("github.dependabot.securityVulnerability") {
                event.rename(
                    "github.dependabot.securityVulnerability",
                    "github.dependabot.security_vulnerability",
                )?;
            }

            if event.has_value("github.dependabot.security_vulnerability.firstPatchedVersion") {
                event.rename(
                    "github.dependabot.security_vulnerability.firstPatchedVersion",
                    "github.dependabot.security_vulnerability.first_patched_version",
                )?;
            }

            if event.has_value("github.dependabot.security_vulnerability.updatedAt") {
                event.rename(
                    "github.dependabot.security_vulnerability.updatedAt",
                    "github.dependabot.security_vulnerability.updated_at",
                )?;
            }

            if event.has_value("github.dependabot.security_vulnerability.vulnerableVersionRange") {
                event.rename(
                    "github.dependabot.security_vulnerability.vulnerableVersionRange",
                    "github.dependabot.security_vulnerability.vulnerable_version_range",
                )?;
            }

            if event.has_value("github.dependabot.vulnerableManifestFilename") {
                event.rename(
                    "github.dependabot.vulnerableManifestFilename",
                    "github.dependabot.vulnerable_manifest_filename",
                )?;
            }

            if event.has_value("github.dependabot.vulnerableManifestPath") {
                event.rename(
                    "github.dependabot.vulnerableManifestPath",
                    "github.dependabot.vulnerable_manifest_path",
                )?;
            }

            if event.has_value("github.dependabot.vulnerableRequirements") {
                event.rename(
                    "github.dependabot.vulnerableRequirements",
                    "github.dependabot.vulnerable_requirements",
                )?;
            }

            event.set("vulnerability.classification", json!("CVSS"))?;

            if event.has_value("github.dependabot.security_advisory.description") {
                event.rename(
                    "github.dependabot.security_advisory.description",
                    "vulnerability.description",
                )?;
            }

            let _cond = { event.has_value("github.dependabot.security_advisory.identifiers") };
            if _cond {
                // Painless script
                // Source: def enumeration = \"GHSA\";\ndef id = \"\";\ndef sa_ids = ctx.github.dependabot.security_advisory.identifiers;\nfor (def sa_id: sa_ids) {\n    id = sa_id.value;\n    if (!sa_id.type.equals(\"GHSA\")) {\n        enumeration = sa_id.type;\n        break;\n    }\n}\nctx.vulnerability.enumeration = enumeration;\nctx.vulnerability.id = id;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def enumeration = \"GHSA\";\ndef id = \"\";\ndef sa_ids = ctx.github.dependabot.security_advisory.identifiers;\nfor (def sa_id: sa_ids) {\n    id = sa_id.value;\n    if (!sa_id.type.equals(\"GHSA\")) {\n        enumeration = sa_id.type;\n        break;\n    }\n}\nctx.vulnerability.enumeration = enumeration;\nctx.vulnerability.id = id;\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("github.dependabot.security_advisory.references") };
            if _cond {
                // Painless script
                // Source: List references = new ArrayList();\ndef sa_references = ctx.github.dependabot.security_advisory.references;\nfor (def ref: sa_references) {\n    references.add(ref.url);\n}\nctx.vulnerability.reference = references;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"List references = new ArrayList();\ndef sa_references = ctx.github.dependabot.security_advisory.references;\nfor (def ref: sa_references) {\n    references.add(ref.url);\n}\nctx.vulnerability.reference = references;\n"#
                    ),
                )?;
            }

            event.remove("github.dependabot.security_advisory.references");

            event.set("vulnerability.scanner.vendor", json!("Github"))?;

            if let Some(val) = event.get("github.dependabot.security_advisory.cvss.score") {
                let converted =
                    convert_value(val, "float").map_err(|message| TransformError::ParseError {
                        path: "github.dependabot.security_advisory.cvss.score".into(),
                        message,
                    })?;
                event.set("github.dependabot.security_advisory.cvss.score", converted)?;
            }

            if event.has_value("github.dependabot.security_advisory.cvss.score") {
                event.rename(
                    "github.dependabot.security_advisory.cvss.score",
                    "vulnerability.score.base",
                )?;
            }

            if event.has_value("github.dependabot.security_advisory.cvss.vector_string") {
                if let Some(kv_str) =
                    event.get_string("github.dependabot.security_advisory.cvss.vector_string")
                {
                    let mut kv_gap = false;
                    for pair in kv_str.split("/") {
                        if pair.is_empty() {
                            kv_gap = true;
                            continue;
                        }
                        let Some((key, value)) = pair.split_once(":").filter(|_| !kv_gap) else {
                            return Err(TransformError::KvValueSplit {
                                field: "github.dependabot.security_advisory.cvss.vector_string"
                                    .into(),
                                split: ":".into(),
                            });
                        };
                        {
                            if !["CVSS"].contains(&key) {
                                continue;
                            }
                            if !key.is_empty() {
                                kv_put(event, &format!("_temp.score.version.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            if event.has_value("_temp.score.version.CVSS") {
                event.rename("_temp.score.version.CVSS", "vulnerability.score.version")?;
            }

            if event.has_value("github.dependabot.security_vulnerability.severity") {
                event.rename(
                    "github.dependabot.security_vulnerability.severity",
                    "vulnerability.severity",
                )?;
            }

            let _cond = { event.has_value("github.dependabot.created_at") };
            if _cond {
                if let Some(v) = event.get("github.dependabot.created_at").cloned() {
                    event.set("event.start", v)?;
                }
            }

            let _cond = { event.has_value("github.dependabot.fixed_at") };
            if _cond {
                if let Some(v) = event.get("github.dependabot.fixed_at").cloned() {
                    event.set("event.end", v)?;
                }
            }

            let _cond = {
                !event.has_value("event.end") && event.has_value("github.dependabot.dismissed_at")
            };
            if _cond {
                if let Some(v) = event.get("github.dependabot.dismissed_at").cloned() {
                    event.set("event.end", v)?;
                }
            }

            let _cond = { event.has_value("event.start") && event.has_value("event.end") };
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

            if event.has_value("github.dependabot.state") {
                map_strings(
                    event,
                    "github.dependabot.state",
                    "github.dependabot.state",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("vulnerability.severity") {
                map_strings(
                    event,
                    "vulnerability.severity",
                    "vulnerability.severity",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("github.dependabot.repository") {
                event.rename("github.dependabot.repository", "github.repository")?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("github.dependabot.created_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.dependabot.dismissed_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.dependabot.fixed_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.dependabot.number") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.repository.name") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("github.repository.owner.login") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.remove("_temp");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
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
