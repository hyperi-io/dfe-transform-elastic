// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `native` pipeline.
pub struct Native;

impl Transform for Native {
    fn name(&self) -> &str {
        "native"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("citrix.cef_format", json!(false))?;

                if let Some(input) = event.get_string("citrix.detail") {
                    // Grok pattern: ^(?:(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone} (?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : )?%{GREEDYDATA:_tmp.details} : +\"%{GREEDYDATA:citrix.extended.message}\"
                    // Grok pattern: ^(?:(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone} (?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : )?%{GREEDYDATA:_tmp.details} : +%{GREEDYDATA:citrix.extended.message}
                    let _ = extract_first_match(
                        &[
                            cached_grok_mapped!("^(?:(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone} (?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : )?%{GREEDYDATA:_tmp.details} : +\"%{GREEDYDATA:citrix.extended.message}\"", [("_tmp_timestamp_native", "_tmp.timestamp_native")]),
                            cached_grok_mapped!("^(?:(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone} (?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : )?%{GREEDYDATA:_tmp.details} : +%{GREEDYDATA:citrix.extended.message}", [("_tmp_timestamp_native", "_tmp.timestamp_native")]),
                        ],
                        &input,
                        event,
                    )?;
                }

            let _cond = { !event.has_value("log.syslog.appname") };
            if _cond {
                if let Some(input) = event.get_string("_tmp.details") {
                    // Grok pattern: ^(?P<_tmp_default>(?:default ))?%{WORD:citrix.device_event_class_id} %{GREEDYDATA:citrix.name} %{INT:event.id} %{INT:event.severity}$
                    let _ = cached_grok_mapped!("^(?P<_tmp_default>(?:default ))?%{WORD:citrix.device_event_class_id} %{GREEDYDATA:citrix.name} %{INT:event.id} %{INT:event.severity}$", [("_tmp_default", "_tmp.default")]).extract_into(&input, event)?;
                }
            }

            let _cond = { event.has_value("log.syslog.appname") };
            if _cond {
                if let Some(input) = event.get_string("_tmp.details") {
                    // Grok pattern: ^(?P<_tmp_default>(?:default ))?%{GREEDYDATA:citrix.name} %{INT:event.id} %{INT:event.severity}$
                    let _ = cached_grok_mapped!("^(?P<_tmp_default>(?:default ))?%{GREEDYDATA:citrix.name} %{INT:event.id} %{INT:event.severity}$", [("_tmp_default", "_tmp.default")]).extract_into(&input, event)?;
                }
            }

            let _cond = { !event.has_value("citrix.device_event_class_id") && event.has_value("log.syslog.appname") };
            if _cond {
            if let Some(v) = event.get("log.syslog.appname").cloned() {
                event.set("citrix.device_event_class_id", v)?;
            }
            }

            let _cond = { event.get_str("_tmp.default") == Some("default ") };
            if _cond {
            event.set("citrix.default_class", json!(true))?;
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
