// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_host_scoring` pipeline.
pub struct PipelineHostScoring;

impl Transform for PipelineHostScoring {
    fn name(&self) -> &str {
        "pipeline_host_scoring"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

                if event.has_value("json.tags") {
                    event.rename("json.tags", "vectra_detect.log.tags")?;
                }

            let _cond = { event.has_value("vectra_detect.log.tags") };
            if _cond {
                event.append_unique("tags", json!(event.get("vectra_detect.log.tags").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.href") {
                    event.rename("json.href", "vectra_detect.log.href")?;
                }

            if let Some(v) = event.get("vectra_detect.log.href").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reference", v)?;
            }

            let _cond = { event.has_value("event.reference") };
            if _cond {
                uri_parts(event, "event.reference", "url", true, false)?;
            }

                if event.has_value("json.dvchost") {
                    event.rename("json.dvchost", "vectra_detect.log.dvchost")?;
                }

            if let Some(v) = event.get("vectra_detect.log.dvchost").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.has_value("observer.hostname") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("observer.hostname").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("json.host_groups").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.host_groups", "json.host_groups")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_to_split_host_groups")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.host_groups").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.host_groups", |event| {
                    if event.has_value("_ingest._value.groupType") {
                    event.rename("_ingest._value.groupType", "_ingest._value.group_type")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.host_groups").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.host_groups", |event| {
                    if event.has_value("_ingest._value.lastModifiedBy") {
                    event.rename("_ingest._value.lastModifiedBy", "_ingest._value.last_modified_by")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.host_groups").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.host_groups", |event| {
                    if event.has_value("_ingest._value.triageFilters") {
                    foreach_array(event, "_ingest._value.triageFilters", |event| {
                    if event.has_value("_ingest._value.triageAs") {
                    event.rename("_ingest._value.triageAs", "_ingest._value.triage_as")?;
                    }
                    Ok(())
                    })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.host_groups").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.host_groups", |event| {
                    if event.has_value("_ingest._value.triageFilters") {
                    event.rename("_ingest._value.triageFilters", "_ingest._value.triage_filters")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.host_groups").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.host_groups", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.cognitoManaged") {
                    if let Some(val) = event.get("_ingest._value.cognitoManaged") {
                    let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.cognitoManaged".into(),
                    message,
                    })?;
                    event.set("_ingest._value.cognito_managed", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_host_groups_cognitoManaged_to_boolean")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("json.host_groups").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.host_groups", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.canEdit") {
                    if let Some(val) = event.get("_ingest._value.canEdit") {
                    let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.canEdit".into(),
                    message,
                    })?;
                    event.set("_ingest._value.can_edit", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_host_groups_canEdit_to_boolean")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("json.host_groups").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.host_groups", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.canDelete") {
                    if let Some(val) = event.get("_ingest._value.canDelete") {
                    let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.canDelete".into(),
                    message,
                    })?;
                    event.set("_ingest._value.can_delete", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_host_groups_canDelete_to_boolean")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("json.host_groups").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.host_groups").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.lastModified") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_ingest._value.last_modified", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.lastModified".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("json.host_groups", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = { event.get("json.host_groups").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.host_groups", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.filterCount") {
                    if let Some(val) = event.get("_ingest._value.filterCount") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.filterCount".into(),
                    message,
                    })?;
                    event.set("_ingest._value.filter_count", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_host_groups_filterCount_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("json.host_groups").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.host_groups", |event| {
                    event.remove("_ingest._value.filterCount");
                    event.remove("_ingest._value.canDelete");
                    event.remove("_ingest._value.canEdit");
                    event.remove("_ingest._value.cognitoManaged");
                    event.remove("_ingest._value.lastModified");
                    Ok(())
                })?;
            }

                if event.has_value("json.host_groups") {
                    event.rename("json.host_groups", "vectra_detect.log.host.groups")?;
                }

            let _cond = { event.get_str("json.host_id") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.host_id") {
                if let Some(val) = event.get("json.host_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.host_id".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.host.id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_host_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.host.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.id", v)?;
            }

            let _cond = { event.has_value("host.id") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.host_ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.host_ip") {
                if let Some(val) = event.get("json.host_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.host_ip".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.host.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_host_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("vectra_detect.log.host.ip") };
            if _cond {
                event.append_unique("host.ip", json!(event.get("vectra_detect.log.host.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("vectra_detect.log.host.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("vectra_detect.log.host.ip").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("host.ip") {
                if let Some(ip_str) = event.get_string("host.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("host.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("host.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("host.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("host.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("host.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("host.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("host.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("host.geo.location", v.clone())?;
                        }
                    }
                }
            }

                if event.has_value("json.host_name") {
                    event.rename("json.host_name", "vectra_detect.log.host.name")?;
                }

            if let Some(v) = event.get("vectra_detect.log.host.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.name").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("json.mac_address") {
                gsub_field(event, "json.mac_address", "json.mac_address", cached_regex!("[:.]"), "-")?;
            }

            if event.has_value("json.mac_address") {
                map_strings(event, "json.mac_address", "json.mac_address", str::to_uppercase)?;
            }

                if event.has_value("json.mac_address") {
                    event.rename("json.mac_address", "vectra_detect.log.mac.address")?;
                }

            let _cond = { event.has_value("vectra_detect.log.mac.address") };
            if _cond {
                event.append_unique("host.mac", json!(event.get("vectra_detect.log.mac.address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("host.mac") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.mac").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.threat") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.threat") {
                if let Some(val) = event.get("json.threat") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.threat".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.threat.score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_threat_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.quadrant") {
                    event.rename("json.quadrant", "vectra_detect.log.quadrant")?;
                }

            if let Some(v) = event.get("vectra_detect.log.quadrant").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.risk.static_level", v)?;
            }

            let _cond = { event.has_value("host.risk.static_level") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.risk.static_level").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.privilege") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.privilege") {
                if let Some(val) = event.get("json.privilege") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.privilege".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.privilege", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_privilege_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.score_decreases") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.score_decreases") {
                if let Some(val) = event.get("json.score_decreases") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.score_decreases".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.score_decreases", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_score_decreases_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.src_key_asset") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.src_key_asset") {
                if let Some(val) = event.get("json.src_key_asset") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.src_key_asset".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.src.key_asset", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_key_asset_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.dst_key_asset") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.dst_key_asset") {
                if let Some(val) = event.get("json.dst_key_asset") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.dst_key_asset".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.dst.key_asset", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_key_asset_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.certainty") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.certainty") {
                if let Some(val) = event.get("json.certainty") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.certainty".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.certainty", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_certainty_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.sensor") {
                    event.rename("json.sensor", "vectra_detect.log.sensor")?;
                }

                if event.has_value("json.category") {
                    event.rename("json.category", "vectra_detect.log.category")?;
                }

            let _cond = { event.get("json.detection_profile").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.detection_profile", "json.detection_profile")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_to_split_detection_profile")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.detection_profile") {
                    event.rename("json.detection_profile", "vectra_detect.log.detection.profile")?;
                }

                if event.has_value("vectra_detect.log.detection.profile.scoringDetections") {
                    event.rename("vectra_detect.log.detection.profile.scoringDetections", "vectra_detect.log.detection.profile.scoring_detections")?;
                }

            let _cond = { event.get("json.account_access_history").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.account_access_history", "json.account_access_history")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_to_split_account_access_history")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.account_access_history") {
                    event.rename("json.account_access_history", "vectra_detect.log.account.access_history")?;
                }

            let _cond = { event.get("vectra_detect.log.account.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.account.access_history", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.id") {
                    if let Some(val) = event.get("_ingest._value.id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_account_access_history_id_to_string")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("vectra_detect.log.account.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("vectra_detect.log.account.access_history").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.lastSeen") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_ingest._value.last_seen", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.lastSeen".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("vectra_detect.log.account.access_history", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = { event.get("vectra_detect.log.account.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.account.access_history", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.privilege") {
                    if let Some(val) = event.get("_ingest._value.privilege") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.privilege".into(),
                    message,
                    })?;
                    event.set("_ingest._value.privilege_value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_account_access_history_privilege_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("vectra_detect.log.account.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.account.access_history", |event| {
                    if event.has_value("_ingest._value.privilegeCategory") {
                    event.rename("_ingest._value.privilegeCategory", "_ingest._value.privilege_category")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("vectra_detect.log.account.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("vectra_detect.log.account.access_history") {
                foreach_array(event, "vectra_detect.log.account.access_history", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }
            }

            let _cond = { event.get("vectra_detect.log.account.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.account.access_history", |event| {
                    event.remove("_ingest._value.lastSeen");
                    event.remove("_ingest._value.privilege");
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.service_access_history").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.service_access_history", "json.service_access_history")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_to_split_service_access_history")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.service_access_history") {
                    event.rename("json.service_access_history", "vectra_detect.log.service.access_history")?;
                }

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.service.access_history", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.id") {
                    if let Some(val) = event.get("_ingest._value.id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.id", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_service_access_history_id_to_string")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("vectra_detect.log.service.access_history").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.lastSeen") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_ingest._value.last_seen", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.lastSeen".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("vectra_detect.log.service.access_history", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.service.access_history", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.privilege") {
                    if let Some(val) = event.get("_ingest._value.privilege") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.privilege".into(),
                    message,
                    })?;
                    event.set("_ingest._value.privilege_value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_service_access_history_privilege_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.service.access_history", |event| {
                    if event.has_value("_ingest._value.privilegeCategory") {
                    event.rename("_ingest._value.privilegeCategory", "_ingest._value.privilege_category")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("vectra_detect.log.service.access_history") {
                foreach_array(event, "vectra_detect.log.service.access_history", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }
            }

            let _cond = { event.get("vectra_detect.log.service.access_history").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.service.access_history", |event| {
                    event.remove("_ingest._value.lastSeen");
                    event.remove("_ingest._value.privilege");
                    Ok(())
                })?;
            }

                if event.has_value("json.mac_vendor") {
                    event.rename("json.mac_vendor", "vectra_detect.log.mac.vendor")?;
                }

                if event.has_value("json.last_detection_type") {
                    event.rename("json.last_detection_type", "vectra_detect.log.last_detection_type")?;
                }

                if event.has_value("json.host_roles") {
                    event.rename("json.host_roles", "vectra_detect.log.host.roles")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
