// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_user` pipeline.
pub struct PipelineObjectUser;

impl Transform for PipelineObjectUser {
    fn name(&self) -> &str {
        "pipeline_object_user"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(v) = event.get("ocsf.user.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.domain", v)?;
            }

            if let Some(v) = event.get("ocsf.user.email_addr").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.email", v)?;
            }

            let _cond = { event.has_value("ocsf.user.email_addr") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.user.email_addr").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.user.full_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.full_name", v)?;
            }

            let _cond = { event.has_value("ocsf.user.full_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.user.full_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.user.groups").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.user.groups", |event| {
                    event.append_unique("user.target.group.id", json!(event.get("_ingest._value.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.user.groups").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.user.groups", |event| {
                    event.append_unique("user.target.group.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.user.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.id", v)?;
            }

            let _cond = { event.has_value("ocsf.user.uid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.user.uid").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.name", v)?;
            }

            let _cond = { event.has_value("ocsf.user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.user.name").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("ocsf.user.account.type_id") {
                if let Some(val) = event.get("ocsf.user.account.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.user.account.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.user.account.type_id", converted)?;
                }
            }

            if event.has_value("ocsf.user.type_id") {
                if let Some(val) = event.get("ocsf.user.type_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.user.type_id".into(),
                            message,
                        })?;
                    event.set("ocsf.user.type_id", converted)?;
                }
            }

            let _cond = { event.has_value("ocsf.user.uid_alt") };
            if _cond {
                event.append_unique("related.user", json!(event.get("ocsf.user.uid_alt").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("ocsf.user.ldap_person.email_addrs").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.user.ldap_person.email_addrs", |event| {
                    event.append_unique("user.ldap_person.email_addrs", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.user.ldap_person.labels").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.user.ldap_person.labels", |event| {
                    event.append_unique("user.ldap_person.labels", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.user.ldap_person.location.is_on_premises") {
                if let Some(val) = event.get("ocsf.user.ldap_person.location.is_on_premises") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.user.ldap_person.location.is_on_premises".into(),
                            message,
                        })?;
                    event.set("ocsf.user.ldap_person.location.is_on_premises", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_user_ldap_person_location_is_on_premises_to_boolean")?;
                        event.remove("ocsf.user.ldap_person.location.is_on_premises");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
