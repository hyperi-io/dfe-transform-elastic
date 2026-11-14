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
            event.set("ecs.version", json!("9.4.0"))?;

            event.set("observer.vendor", json!("Doppler"))?;

            event.set("observer.product", json!("Activity Logs"))?;

            if let Some(v) = event
                .get("doppler.source")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("doppler.createdAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("doppler.createdAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "doppler.createdAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created_at")?;
                    event.append("error.message", json!(format!("Processor '{}' with tag '{}' in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { !event.has_value("@timestamp") };
            if _cond {
                event.set(
                    "@timestamp",
                    json!(
                        event
                            .get("_ingest.timestamp")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("doppler.createdAt") };
            if _cond {
                event.append(
                    "error.message",
                    json!("doppler.createdAt missing; @timestamp set from ingest time"),
                )?;
            }

            if let Some(v) = event
                .get("doppler.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if let Some(v) = event
                .get("doppler.slug")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("doppler.user") {
                event.rename("doppler.user", "doppler.actor")?;
            }

            if event.has_value("doppler.actor.slug") {
                event.rename("doppler.actor.slug", "user.id")?;
            }

            if event.has_value("doppler.actor.username") {
                event.rename("doppler.actor.username", "user.name")?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                if let Some(v) = event
                    .get("doppler.actor.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            if event.has_value("doppler.actor.name") {
                event.rename("doppler.actor.name", "user.full_name")?;
            }

            if event.has_value("doppler.actor.email") {
                event.rename("doppler.actor.email", "user.email")?;
            }

            let _cond = {
                event
                    .get("doppler.actor.type")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                if event.has_value("doppler.actor.type") {
                    event.rename("doppler.actor.type", "doppler.actor.token_type")?;
                }
            }

            let _cond = {
                event
                    .get("doppler.actor.type")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has_value("doppler.actor.type") {
                    event.rename("doppler.actor.type", "doppler.actor.bot_type")?;
                }
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                if let Some(v) = event
                    .get("doppler.actor.token_type.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = {
                event.get_str("event.action") == Some("security.secret_read")
                    && !event.has_value("event.id")
            };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("@timestamp") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("doppler.metadata.secrets") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("user.id") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("event.id", json!(fingerprint_default(&values)))?;
                    }
                }
            }

            if let Some(v) = event
                .get("doppler.workplace.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            if let Some(v) = event
                .get("doppler.workplace.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            let _cond = { event.get("event.action").is_some_and(|v| v.is_string()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "149ec1fa4842ac8b5701f6b1878e5c0c56600dda2c48f81e8f6863f538df3a71",
                            r#"{"params":null,"known":[{"CategoryFromAction":{"source":"event.action","separator":".","exempt":"security.secret_read","predicate":[{"Contains":".access."},{"StartsWith":"team.group"},{"StartsWith":"team.seat"},{"StartsWith":"team.service_account"},{"StartsWith":"custom_roles"},{"Contains":".service_token"}],"category":"event.category","category_when":"iam","category_otherwise":"configuration","types":"event.type","arms":[{"tests":[{"Whole":"security.secret_read"}],"value":"access"},{"tests":[{"Last":"create"}],"value":"creation"},{"tests":[{"Last":"delete"},{"Last":"revoke"},{"Last":"remove"}],"value":"deletion"},{"tests":[{"Last":"join"},{"Last":"add"}],"value":"creation"}],"fallback":"change","extra":[{"tests":[{"Contains":"group"}],"value":"group"},{"tests":[{"Contains":"seat"},{"Contains":".access."},{"Contains":"service_account"}],"value":"user"}],"extra_fallback":null}}]}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "categorize_event")?;
                    event.append("error.message", json!(format!("Processor '{}' with tag '{}' in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.set("event.outcome", json!("success"))?;

            let _cond = { event.get_str("event.action") != Some("security.secret_read") };
            if _cond {
                // Begin nested pipeline: "activity_enrich"
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
                        map_strings(
                            event,
                            "doppler.activity.description",
                            "doppler.activity.description",
                            html_strip,
                        )?;
                    }
                    Ok(())
                })();
                if let Some(v) = event
                    .get("doppler.metadata.projectId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.project.id", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.projectName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.project.name", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.configName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.config.name", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.environmentId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.environment.id", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.environmentName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.environment.name", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.diff.added")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.secrets_changed.added", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.diff.removed")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.secrets_changed.removed", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.diff.updated")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.secrets_changed.updated", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.role")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.role.name", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.newRole")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.role.new", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.oldRole")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.role.old", v)?;
                }
                let _cond = { !event.has_value("doppler.activity.role.new") };
                if _cond {
                    if let Some(v) = event
                        .get("doppler.metadata.newWorkplaceRole")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("doppler.activity.role.new", v)?;
                    }
                }
                let _cond = { !event.has_value("doppler.activity.role.old") };
                if _cond {
                    if let Some(v) = event
                        .get("doppler.metadata.oldWorkplaceRole")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("doppler.activity.role.old", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.action")
                        && event
                            .get_str("event.action")
                            .is_some_and(|s| s.starts_with("custom_roles"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("doppler.metadata.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("doppler.activity.role.name", v)?;
                    }
                }
                if let Some(v) = event
                    .get("doppler.metadata.userEmail")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.target.email", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.userName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.target.name", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.email")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.target.email", v)?;
                }
                let _cond = { event.has_value("doppler.metadata.email") };
                if _cond {
                    if let Some(v) = event
                        .get("doppler.metadata.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.target.full_name", v)?;
                    }
                }
                if let Some(v) = event
                    .get("doppler.metadata.serviceAccountId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.target.id", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.serviceAccountName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.target.name", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.groupId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.id", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.groupName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("group.name", v)?;
                }
                let _cond = { event.has_value("doppler.metadata.addedMembers") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // The pipeline's script, run by the runtime's equivalent
                        painless_exec_plan(
                            event,
                            compiled_painless!(
                                "b22150d21cc216ed618f1769110ec48c3fd267b7eddade7f406b9e79f022fae7",
                                r#"{"params":null,"known":["CollectingLadder"]}"#
                            ),
                        )?;
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
                if let Some(v) = event
                    .get("doppler.metadata.serviceAccountTokenId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.token.id", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.serviceAccountTokenName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.token.name", v)?;
                }
                let _cond = { !event.has_value("doppler.activity.token.name") };
                if _cond {
                    if let Some(v) = event
                        .get("doppler.metadata.serviceTokenName")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("doppler.activity.token.name", v)?;
                    }
                }
                if let Some(v) = event
                    .get("doppler.metadata.readAccess")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.service_token.read_access", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("doppler.activity.service_token.read_access") {
                        if let Some(val) = event.get("doppler.activity.service_token.read_access") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "doppler.activity.service_token.read_access".into(),
                                    message,
                                }
                            })?;
                            event.set("doppler.activity.service_token.read_access", converted)?;
                        }
                    }
                    Ok(())
                })();
                if let Some(v) = event
                    .get("doppler.metadata.writeAccess")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.service_token.write_access", v)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("doppler.activity.service_token.write_access") {
                        if let Some(val) = event.get("doppler.activity.service_token.write_access")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "doppler.activity.service_token.write_access".into(),
                                    message,
                                }
                            })?;
                            event.set("doppler.activity.service_token.write_access", converted)?;
                        }
                    }
                    Ok(())
                })();
                let _cond = { !event.has_value("doppler.activity.environment.id") };
                if _cond {
                    if let Some(v) = event
                        .get("doppler.metadata.newEnvironmentId")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("doppler.activity.environment.id", v)?;
                    }
                }
                let _cond = { !event.has_value("doppler.activity.environment.name") };
                if _cond {
                    if let Some(v) = event
                        .get("doppler.metadata.newEnvironmentName")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("doppler.activity.environment.name", v)?;
                    }
                }
                if let Some(v) = event
                    .get("doppler.metadata.oldEnvironmentId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppler.activity.environment.old_id", v)?;
                }
                if let Some(v) = event
                    .get("doppler.metadata.oldEnvironmentName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
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
                if let Some(v) = event
                    .get("doppler.activity.url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.full", v)?;
                }
                // End nested pipeline: "activity_enrich"
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
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

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.target.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.target.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.target.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.target.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("doppler.source");
            event.remove("doppler.title");
            event.remove("doppler.createdAt");
            event.remove("doppler.type");
            event.remove("doppler.actor.profile_image_url");
            event.remove("doppler.actor.is_bot");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // The pipeline's script, emitted as the runtime's equivalent
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
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "drop_null_values")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' with tag '{}' in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // SKIPPED: nested pipeline "logs-doppler.activity@custom" is not in this pipeline set

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                    json!(format!(
                        "Processor '{}' with tag '{}' in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
