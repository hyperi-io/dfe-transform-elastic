// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `detection_summary` pipeline.
pub struct DetectionSummary;

impl Transform for DetectionSummary {
    fn name(&self) -> &str {
        "detection_summary"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            event.append("event.category", json!("malware"))?;

            event.append("event.type", json!("info"))?;

            if event.has("crowdstrike.event.UserName") {
                event.rename("crowdstrike.event.UserName", "user.name")?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.ProcessStartTime")
                    && event
                        .get_as_string("crowdstrike.event.ProcessStartTime")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ProcessStartTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("process.start", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ProcessStartTime")
                    && event
                        .get_as_string("crowdstrike.event.ProcessStartTime")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ProcessStartTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("process.start", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ProcessEndTime")
                    && event
                        .get_as_string("crowdstrike.event.ProcessEndTime")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ProcessEndTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("process.end", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ProcessEndTime")
                    && event
                        .get_as_string("crowdstrike.event.ProcessEndTime")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ProcessEndTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("process.end", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.LocalIP")
                    && event.get_str("crowdstrike.event.LocalIP") != Some("")
            };
            if _cond {
                if event.has("crowdstrike.event.LocalIP") {
                    event.rename("crowdstrike.event.LocalIP", "source.ip")?;
                }
            }

            if event.has("crowdstrike.event.ProcessId") {
                event.rename("crowdstrike.event.ProcessId", "process.pid")?;
            }

            if event.has_value("crowdstrike.event.HostGroups") {
                if let Some(s) = event.get_string("crowdstrike.event.HostGroups") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("crowdstrike.event.HostGroups", Value::Array(parts))?;
                }
            }

            if event.has("crowdstrike.event.ParentProcessId") {
                event.rename("crowdstrike.event.ParentProcessId", "process.parent.pid")?;
            }

            let _cond = { !event.has_value("process.parent.executable") };
            if _cond {
                if event.has("crowdstrike.event.ParentImageFileName") {
                    event.rename(
                        "crowdstrike.event.ParentImageFileName",
                        "process.parent.executable",
                    )?;
                }
            }

            if event.has("crowdstrike.event.PatternDispositionDescription") {
                event.rename(
                    "crowdstrike.event.PatternDispositionDescription",
                    "event.action",
                )?;
            }

            if event.has("crowdstrike.event.FalconHostLink") {
                event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
            }

            if event.has("crowdstrike.event.DetectDescription") {
                event.rename("crowdstrike.event.DetectDescription", "message")?;
            }

            let _cond = { event.has_value("message") };
            if _cond {
                if let Some(v) = event.get("message").cloned() {
                    event.set("rule.description", v)?;
                }
            }

            if let Some(v) = event
                .get("crowdstrike.event.FileName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if event.has("crowdstrike.event.MachineDomain") {
                event.rename("crowdstrike.event.MachineDomain", "host.domain")?;
            }

            if event.has("crowdstrike.event.ComputerName") {
                event.rename("crowdstrike.event.ComputerName", "host.name")?;
            }

            if event.has("crowdstrike.event.SHA256String") {
                event.rename("crowdstrike.event.SHA256String", "file.hash.sha256")?;
            }

            if event.has("crowdstrike.event.MD5String") {
                event.rename("crowdstrike.event.MD5String", "file.hash.md5")?;
            }

            if event.has("crowdstrike.event.SHA1String") {
                event.rename("crowdstrike.event.SHA1String", "file.hash.sha1")?;
            }

            let _cond = {
                event.has_value("file.hash.sha1") && event.get_str("file.hash.sha1") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha1")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("file.hash.sha256") && event.get_str("file.hash.sha256") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("file.hash.md5") && event.get_str("file.hash.md5") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.md5")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("crowdstrike.event.FileName") {
                event.rename("crowdstrike.event.FileName", "file.name")?;
            }

            if event.has("crowdstrike.event.FilePath") {
                event.rename("crowdstrike.event.FilePath", "file.path")?;
            }

            if event.has("crowdstrike.event.DetectName") {
                event.rename("crowdstrike.event.DetectName", "rule.name")?;
            }

            if event.has("crowdstrike.event.DetectId") {
                event.rename("crowdstrike.event.DetectId", "rule.id")?;
            }

            let _cond = { event.has_value("crowdstrike.event.MacAddress") };
            if _cond {
                if event.has("crowdstrike.event.MacAddress") {
                    event.rename("crowdstrike.event.MacAddress", "host.mac")?;
                }
            }

            let _cond = { event.has_value("host.mac") };
            if _cond {
                if event.has_value("host.mac") {
                    if let Some(s) = event.get_string("host.mac") {
                        let uppered = s.to_uppercase();
                        event.set("host.mac", uppered)?;
                    }
                }
            }

            let _cond = { event.has_value("crowdstrike.event.Technique") };
            if _cond {
                event.append(
                    "threat.technique.name",
                    json!(
                        event
                            .get("crowdstrike.event.Technique")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.TechniqueId") };
            if _cond {
                event.append(
                    "threat.technique.id",
                    json!(
                        event
                            .get("crowdstrike.event.TechniqueId")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.Tactic") };
            if _cond {
                event.append(
                    "threat.tactic.name",
                    json!(
                        event
                            .get("crowdstrike.event.Tactic")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.TacticId") };
            if _cond {
                event.append(
                    "threat.tactic.id",
                    json!(
                        event
                            .get("crowdstrike.event.TacticId")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
