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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "_temp_")?;

            let _cond = { !event.has_value("_temp_.ts") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.rename("_temp_", "zeek.smb_cmd")?;

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("connection"))?;

            event.append("event.type", json!("protocol"))?;

            event.set("network.transport", json!("tcp"))?;

            event.set("network.protocol", json!("smb"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.ts")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.id.orig_p")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.id.resp_p")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.size")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.times.modified")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.times.accessed")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.times.created")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.times.changed")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.uid")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.id.orig_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.id.resp_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.action")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.name")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "referenced_file.path")?;
                Ok(())
            })();

            event.remove("zeek.smb_cmd.referenced_file.ts");
            event.remove("zeek.smb_cmd.referenced_file.id.orig_p");
            event.remove("zeek.smb_cmd.referenced_file.id.resp_p");
            event.remove("zeek.smb_cmd.referenced_file.size");
            event.remove("zeek.smb_cmd.referenced_file.times.modified");
            event.remove("zeek.smb_cmd.referenced_file.times.accessed");
            event.remove("zeek.smb_cmd.referenced_file.times.created");
            event.remove("zeek.smb_cmd.referenced_file.times.changed");

            let _cond = { !event.has_value("zeek.smb_cmd.referenced_file.action") };
            if _cond {
                event.remove("zeek.smb_cmd.referenced_file.uid");
                event.remove("zeek.smb_cmd.referenced_file.id.orig_h");
                event.remove("zeek.smb_cmd.referenced_file.id.resp_h");
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "id.orig_p")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "id.orig_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "id.resp_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.smb_cmd", "id.resp_p")?;
                Ok(())
            })();

            if event.has_value("zeek.smb_cmd.id.orig_h") {
                event.rename("zeek.smb_cmd.id.orig_h", "source.address")?;
            }

            if event.has_value("zeek.smb_cmd.id.orig_p") {
                event.rename("zeek.smb_cmd.id.orig_p", "source.port")?;
            }

            if event.has_value("zeek.smb_cmd.id.resp_h") {
                event.rename("zeek.smb_cmd.id.resp_h", "destination.address")?;
            }

            if event.has_value("zeek.smb_cmd.id.resp_p") {
                event.rename("zeek.smb_cmd.id.resp_p", "destination.port")?;
            }

            if event.has_value("zeek.smb_cmd.uid") {
                event.rename("zeek.smb_cmd.uid", "zeek.session_id")?;
            }

            let _cond = { event.has_value("zeek.session_id") };
            if _cond {
                if let Some(v) = event.get("zeek.session_id").cloned() {
                    event.set("event.id", v)?;
                }
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                if let Some(v) = event.get("source.address").cloned() {
                    event.set("source.ip", v)?;
                }
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                if let Some(v) = event.get("destination.address").cloned() {
                    event.set("destination.ip", v)?;
                }
            }

            if event.has_value("zeek.smb_cmd.referenced_file.uid") {
                event.rename("zeek.smb_cmd.referenced_file.uid", "zeek.smb_cmd.file.uid")?;
            }

            if event.has_value("zeek.smb_cmd.referenced_file.id.orig_h") {
                event.rename(
                    "zeek.smb_cmd.referenced_file.id.orig_h",
                    "zeek.smb_cmd.file.host.tx",
                )?;
            }

            if event.has_value("zeek.smb_cmd.referenced_file.id.resp_h") {
                event.rename(
                    "zeek.smb_cmd.referenced_file.id.resp_h",
                    "zeek.smb_cmd.file.host.rx",
                )?;
            }

            if event.has_value("zeek.smb_cmd.referenced_file.name") {
                event.rename(
                    "zeek.smb_cmd.referenced_file.name",
                    "zeek.smb_cmd.file.name",
                )?;
            }

            if event.has_value("zeek.smb_cmd.referenced_file.path") {
                event.rename(
                    "zeek.smb_cmd.referenced_file.path",
                    "zeek.smb_cmd.file.path",
                )?;
            }

            if event.has_value("zeek.smb_cmd.referenced_file.action") {
                event.rename(
                    "zeek.smb_cmd.referenced_file.action",
                    "zeek.smb_cmd.file.action",
                )?;
            }

            let _cond = { event.has_value("zeek.smb_cmd.command") };
            if _cond {
                if let Some(v) = event.get("zeek.smb_cmd.command").cloned() {
                    event.set("event.action", v)?;
                }
            }

            let _cond = { event.has_value("zeek.smb_cmd.username") };
            if _cond {
                if let Some(v) = event.get("zeek.smb_cmd.username").cloned() {
                    event.set("user.name", v)?;
                }
            }

            if let Some(date_str) = event.get_as_string("zeek.smb_cmd.ts") {
                match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "zeek.smb_cmd.ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("zeek.smb_cmd.ts").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "zeek.smb_cmd.ts".into(),
                });
            }

            event.remove("zeek.smb_cmd.referenced_file");

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
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

            let _cond = {
                event.has_value("zeek.smb_cmd.status")
                    && !(event
                        .get_str("zeek.smb_cmd.status")
                        .is_some_and(|s| s.eq_ignore_ascii_case("success")))
            };
            if _cond {
                event.append("event.type", json!("error"))?;
            }

            let _cond = {
                event.has_value("zeek.smb_cmd.status")
                    && event
                        .get_str("zeek.smb_cmd.status")
                        .is_some_and(|s| s.eq_ignore_ascii_case("success"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("zeek.smb_cmd.status")
                    && !(event
                        .get_str("zeek.smb_cmd.status")
                        .is_some_and(|s| s.eq_ignore_ascii_case("success")))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            // Community ID v1 hash
            if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                event.get_string("source.ip"),
                event.get_string("destination.ip"),
                event
                    .get_as_string("network.iana_number")
                    .or_else(|| event.get_as_string("network.transport")),
            ) {
                let icmp = matches!(
                    protocol.to_ascii_lowercase().as_str(),
                    "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                );
                let (src_field, dst_field) = if icmp {
                    ("icmp.type", "icmp.code")
                } else {
                    ("source.port", "destination.port")
                };
                let src_port = u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                let dst_port = u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                    Ok(cid) => event.set("network.community_id", cid)?,
                    Err(message) => {
                        return Err(TransformError::ParseError {
                            path: "network.community_id".into(),
                            message,
                        });
                    }
                }
            }

            event.remove("zeek.smb_cmd.id");

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
