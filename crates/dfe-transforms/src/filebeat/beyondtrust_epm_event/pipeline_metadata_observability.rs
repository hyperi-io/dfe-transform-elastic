// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_metadata_observability` pipeline.
pub struct PipelineMetadataObservability;

impl Transform for PipelineMetadataObservability {
    fn name(&self) -> &str {
        "pipeline_metadata_observability"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // SKIPPED: condition not transpiled: ctx.beyondtrust_epm?.event != null && ctx.beyondtrust_epm.event['@timestamp'] != null && ctx.beyondtrust_epm.event['@timestamp'] != ''
        #[allow(unreachable_code, unused_variables)]
        if false {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.@timestamp") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.@timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_@timestamp")?;
                    event.remove("beyondtrust_epm.event.@timestamp");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.timestamp").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("@timestamp", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.EPMWinMac.GroupId").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("organization.id", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.message") };
        if _cond {
        if let Some(v) = event.get("beyondtrust_epm.event.message").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("message", v)?;
        }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.organization.id") };
        if _cond {
            event.append_unique("organization.id", json!(event.get("beyondtrust_epm.event.organization.id").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.organization.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("organization.name", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.related.hash").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.related.hash", |event| {
                event.append_unique("related.hash", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.related.hosts").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.related.hosts", |event| {
                event.append_unique("related.hosts", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.related.ip").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.related.ip", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value") {
                if let Some(val) = event.get("_ingest._value") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value".into(),
                message,
                })?;
                event.set("_ingest._value", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_related_ip_to_ip")?;
                event.remove("_ingest._value");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.related.ip").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.related.ip", |event| {
                event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.related.user").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.related.user", |event| {
                event.append_unique("related.user", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.span.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("span.id", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tags").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tags", |event| {
                event.append_unique("tags", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.trace.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("trace.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.transaction.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("transaction.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.EPMWinMac.Configuration.RuleScript.Outcome.Output").filter(|v| !painless_is_empty_value(v)).cloned() {
            if !event.has("message") {
                event.set("message", v)?;
            }
        }

            event.remove("beyondtrust_epm.event.EPMWinMac.Configuration.RuleScript.Outcome.Output");
            event.remove("beyondtrust_epm.event.EPMWinMac.GroupId");
            event.remove("beyondtrust_epm.event.message");
            event.remove("beyondtrust_epm.event.organization.id");
            event.remove("beyondtrust_epm.event.organization.name");
            event.remove("beyondtrust_epm.event.related.hash");
            event.remove("beyondtrust_epm.event.related.hosts");
            event.remove("beyondtrust_epm.event.related.ip");
            event.remove("beyondtrust_epm.event.related.user");
            event.remove("beyondtrust_epm.event.span.id");
            event.remove("beyondtrust_epm.event.tags");
            event.remove("beyondtrust_epm.event.timestamp");
            event.remove("beyondtrust_epm.event.trace.id");
            event.remove("beyondtrust_epm.event.transaction.id");

        Ok(TransformResult::Continue)
    }
}
