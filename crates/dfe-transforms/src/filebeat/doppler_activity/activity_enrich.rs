// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `activity_enrich` pipeline.
pub struct ActivityEnrich;

impl Transform for ActivityEnrich {
    fn name(&self) -> &str {
        "activity_enrich"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("doppler.slug") {
                    event.rename("doppler.slug", "doppler.activity.log_id")?;
                }

                if event.has_value("doppler.link") {
                    event.rename("doppler.link", "doppler.activity.url")?;
                }

                if event.has_value("doppler.text") {
                    event.rename("doppler.text", "doppler.activity.description")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("doppler.activity.description") {
                map_strings(event, "doppler.activity.description", "doppler.activity.description", html_strip)?;
            }
                Ok(())
            })();

            if let Some(v) = event.get("doppler.metadata.projectId").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.project.id", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.projectName").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.project.name", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.configName").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.config.name", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.environmentId").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.environment.id", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.environmentName").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.environment.name", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.diff.added").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.secrets_changed.added", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.diff.removed").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.secrets_changed.removed", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.diff.updated").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.secrets_changed.updated", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.role").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.role.name", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.newRole").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.role.new", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.oldRole").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.role.old", v)?;
            }

            let _cond = { !event.has_value("doppler.activity.role.new") };
            if _cond {
            if let Some(v) = event.get("doppler.metadata.newWorkplaceRole").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.role.new", v)?;
            }
            }

            let _cond = { !event.has_value("doppler.activity.role.old") };
            if _cond {
            if let Some(v) = event.get("doppler.metadata.oldWorkplaceRole").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.role.old", v)?;
            }
            }

            let _cond = { event.has_value("event.action") && event.get_str("event.action").is_some_and(|s| s.starts_with("custom_roles")) };
            if _cond {
            if let Some(v) = event.get("doppler.metadata.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.role.name", v)?;
            }
            }

            if let Some(v) = event.get("doppler.metadata.userEmail").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.email", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.userName").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.name", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.email").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.email", v)?;
            }

            let _cond = { event.has_value("doppler.metadata.email") };
            if _cond {
            if let Some(v) = event.get("doppler.metadata.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.full_name", v)?;
            }
            }

            if let Some(v) = event.get("doppler.metadata.serviceAccountId").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.id", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.serviceAccountName").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.name", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.groupId").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("group.id", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.groupName").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("group.name", v)?;
            }

            let _cond = { event.has_value("doppler.metadata.addedMembers") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def am = ctx.doppler.metadata.addedMembers;\nArrayList members = new ArrayList();\nif (am instanceof List) {\n  members.addAll(am);\n} else if (am instanceof Map) {\n  members.add(am);\n}\nArrayList emails = new ArrayList();\nfor (def m : members) {\n  if (m instanceof Map && m.email != null && !emails.contains(m.email)) {\n    emails.add(m.email);\n  }\n}\nif (members.size() > 0 && members.get(0) instanceof Map) {\n  def first = members.get(0);\n  if (ctx.user == null) { ctx.user = new HashMap(); }\n  if (ctx.user.target == null) { ctx.user.target = new HashMap(); }\n  if (first.email != null && ctx.user.target.email == null) {\n    ctx.user.target.email = first.email;\n  }\n  if (first.name != null && ctx.user.target.name == null) {\n    ctx.user.target.name = first.name;\n  }\n}\nif (emails.size() > 0) {\n  if (ctx.doppler.activity == null) { ctx.doppler.activity = new HashMap(); }\n  ctx.doppler.activity.members_added = emails;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def am = ctx.doppler.metadata.addedMembers;\nArrayList members = new ArrayList();\nif (am instanceof List) {\n  members.addAll(am);\n} else if (am instanceof Map) {\n  members.add(am);\n}\nArrayList emails = new ArrayList();\nfor (def m : members) {\n  if (m instanceof Map && m.email != null && !emails.contains(m.email)) {\n    emails.add(m.email);\n  }\n}\nif (members.size() > 0 && members.get(0) instanceof Map) {\n  def first = members.get(0);\n  if (ctx.user == null) { ctx.user = new HashMap(); }\n  if (ctx.user.target == null) { ctx.user.target = new HashMap(); }\n  if (first.email != null && ctx.user.target.email == null) {\n    ctx.user.target.email = first.email;\n  }\n  if (first.name != null && ctx.user.target.name == null) {\n    ctx.user.target.name = first.name;\n  }\n}\nif (emails.size() > 0) {\n  if (ctx.doppler.activity == null) { ctx.doppler.activity = new HashMap(); }\n  ctx.doppler.activity.members_added = emails;\n}"#))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "promote_added_members")?;
                        event.append("error.message", json!(format!("Processor '{}' with tag '{}' in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("doppler.metadata.serviceAccountTokenId").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.token.id", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.serviceAccountTokenName").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.token.name", v)?;
            }

            let _cond = { !event.has_value("doppler.activity.token.name") };
            if _cond {
            if let Some(v) = event.get("doppler.metadata.serviceTokenName").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.token.name", v)?;
            }
            }

            if let Some(v) = event.get("doppler.metadata.readAccess").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.service_token.read_access", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("doppler.activity.service_token.read_access") {
                if let Some(val) = event.get("doppler.activity.service_token.read_access") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "doppler.activity.service_token.read_access".into(),
                            message,
                        })?;
                    event.set("doppler.activity.service_token.read_access", converted)?;
                }
            }
                Ok(())
            })();

            if let Some(v) = event.get("doppler.metadata.writeAccess").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.service_token.write_access", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("doppler.activity.service_token.write_access") {
                if let Some(val) = event.get("doppler.activity.service_token.write_access") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "doppler.activity.service_token.write_access".into(),
                            message,
                        })?;
                    event.set("doppler.activity.service_token.write_access", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { !event.has_value("doppler.activity.environment.id") };
            if _cond {
            if let Some(v) = event.get("doppler.metadata.newEnvironmentId").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.environment.id", v)?;
            }
            }

            let _cond = { !event.has_value("doppler.activity.environment.name") };
            if _cond {
            if let Some(v) = event.get("doppler.metadata.newEnvironmentName").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.environment.name", v)?;
            }
            }

            if let Some(v) = event.get("doppler.metadata.oldEnvironmentId").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.environment.old_id", v)?;
            }

            if let Some(v) = event.get("doppler.metadata.oldEnvironmentName").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("doppler.activity.environment.old_name", v)?;
            }

                event.remove("doppler.metadata.serviceAccountTokenApiKeyPreview");

                if event.has_value("doppler.metadata") {
                    event.rename("doppler.metadata", "doppler.activity.metadata")?;
                }

            let _cond = { event.has_value("doppler.activity.url") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "doppler.activity.url", "url", false, false)?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("doppler.activity.url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.full", v)?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' with tag '{}' in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("error.type", json!("pipeline_error"))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
