// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `v2_pipeline` pipeline.
pub struct V2Pipeline;

impl Transform for V2Pipeline {
    fn name(&self) -> &str {
        "v2_pipeline"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("mimecast.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("mimecast.timestamp") {
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "mimecast.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("@timestamp") };
            if _cond {
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("event.created", v)?;
                }
            }

            let _cond = { event.has_value("mimecast.type") };
            if _cond {
                // Painless script
                // Source: ctx.mimecast.log_type = params.get(ctx.mimecast.type);\nctx.mimecast.type = null;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.mimecast.log_type = params.get(ctx.mimecast.type);\nctx.mimecast.type = null;\n"#
                    ),
                    cached_params!(
                        "{\"av\":\"avlog\",\"delivery\":\"delivery\",\"internal email protect\":\"internal-email-protect\",\"impersonation protect\":\"impersonation-protect\",\"journal\":\"jrnl\",\"process\":\"process\",\"receipt\":\"receipt\",\"attachment protect\":\"attachment-protect\",\"spam\":\"spam\",\"url protect\":\"url-protect\"}"
                    ),
                )?;
            }

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                    serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"),
                    _ => false,
                })
            };
            if _cond {
                if let Some(v) = event
                    .get("mimecast")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("original", v)?;
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("mimecast.accountId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.action") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.aggregateId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.log_type") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.messageId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.processingId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.subtype") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.timestamp") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event
                    .get("mimecast.recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "mimecast.recipients", |event| {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("_ingest._value.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("mimecast.recipients")
                    && !(event
                        .get("mimecast.recipients")
                        .is_some_and(|v| v.is_array()))
            };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("mimecast.recipients")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("mimecast.action") {
                event.rename("mimecast.action", "event.action")?;
            }

            if event.has_value("mimecast.tlsCipher") {
                event.rename("mimecast.tlsCipher", "tls.cipher")?;
            }

            if event.has_value("mimecast.direction") {
                event.rename("mimecast.direction", "email.direction")?;
            }

            if event.has_value("mimecast.receiptErrors") {
                event.rename("mimecast.receiptErrors", "error.message")?;
            }

            if event.has_value("mimecast.senderIp") {
                event.rename("mimecast.senderIp", "source.ip")?;
            }

            if event.has_value("mimecast.messageId") {
                event.rename("mimecast.messageId", "email.message_id")?;
            }

            let _cond = { event.has_value("mimecast.senderHeader") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.senderHeader")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("mimecast.rejectionCode") {
                event.rename("mimecast.rejectionCode", "error.code")?;
            }

            if event.has_value("mimecast.rejectionInfo") {
                event.rename("mimecast.rejectionInfo", "event.reason")?;
            }

            let _cond = {
                event.has_value("mimecast.rejectionType")
                    && event.get_str("mimecast.rejectionType") != Some("")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has_value("mimecast.rejectionType") {
                event.rename("mimecast.rejectionType", "error.type")?;
            }

            let _cond = { event.has_value("mimecast.senderEnvelope") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.senderEnvelope")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("mimecast.subject") {
                event.rename("mimecast.subject", "email.subject")?;
            }

            if event.has_value("mimecast.tlsVer") {
                event.rename("mimecast.tlsVer", "tls.version")?;
            }

            if event.has_value("mimecast.totalSizeAttachments") {
                event.rename(
                    "mimecast.totalSizeAttachments",
                    "email.attachments.file.size",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("email.attachments.file.size") {
                    if let Some(val) = event.get("email.attachments.file.size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "email.attachments.file.size".into(),
                                message,
                            }
                        })?;
                        event.set("email.attachments.file.size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("email.attachments.file.size").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "email.attachments.file.size".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("mimecast.attachments") {
                event.rename("mimecast.attachments", "email.attachments.file.name")?;
            }

            let _cond =
                { event.has_value("mimecast.Hld") && event.get_str("mimecast.Hld") != Some("") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has_value("mimecast.holdReason") {
                event.rename("mimecast.holdReason", "event.reason")?;
            }

            if event.has_value("mimecast.destinationIp") {
                event.rename("mimecast.destinationIp", "destination.ip")?;
            }

            if event.has_value("mimecast.deliveryErrors") {
                event.rename("mimecast.deliveryErrors", "error.message")?;
            }

            if event.has_value("mimecast.tlsUsed") {
                event.rename("mimecast.tlsUsed", "tls.established")?;
            }

            let _cond = {
                event.get("tls.established").is_some_and(|v| v.is_string())
                    && event
                        .get_str("tls.established")
                        .is_some_and(|s| s.eq_ignore_ascii_case("yes"))
            };
            if _cond {
                event.set("tls.established", json!(true))?;
            }

            let _cond = {
                event.get("tls.established").is_some_and(|v| v.is_string())
                    && event
                        .get_str("tls.established")
                        .is_some_and(|s| s.eq_ignore_ascii_case("no"))
            };
            if _cond {
                event.set("tls.established", json!(false))?;
            }

            let _cond = {
                event.has_value("mimecast.fileExtension")
                    && event.get_str("mimecast.fileExtension") != Some("")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has_value("mimecast.fileExtension") {
                event.rename("mimecast.fileExtension", "email.attachments.file.extension")?;
            }

            if event.has_value("mimecast.md5") {
                event.rename("mimecast.md5", "email.attachments.file.hash.md5")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("mimecast.senderDomainInternal") {
                    if let Some(val) = event.get("mimecast.senderDomainInternal") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "mimecast.senderDomainInternal".into(),
                                message,
                            }
                        })?;
                        event.set("mimecast.senderDomainInternal", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_sender_domain_internal_to_boolean",
                )?;
                if event.remove("mimecast.senderDomainInternal").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "mimecast.senderDomainInternal".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("mimecast.sha1") {
                event.rename("mimecast.sha1", "email.attachments.file.hash.sha1")?;
            }

            if event.has_value("mimecast.sha256") {
                event.rename("mimecast.sha256", "email.attachments.file.hash.sha256")?;
            }

            if event.has_value("mimecast.fileName") {
                event.rename("mimecast.fileName", "email.attachments.file.name")?;
            }

            let _cond = {
                event.has_value("mimecast.senderIp")
                    && event.get_str("mimecast.senderIp") != Some("")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has_value("mimecast.senderIp") {
                event.rename("mimecast.senderIp", "source.ip")?;
            }

            let _cond =
                { event.has_value("mimecast.url") && event.get_str("mimecast.url") != Some("") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has_value("mimecast.url") {
                event.rename("mimecast.url", "url.full")?;
            }

            let _cond = {
                event.get_bool("mimecast.taggedMalicious") == Some(true)
                    || event.get_str("mimecast.taggedMalicious") == Some("true")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has_value("mimecast.action") {
                event.rename("mimecast.action", "event.action")?;
            }

            if event.has_value("mimecast.policyDefinition") {
                event.rename("mimecast.policyDefinition", "rule.name")?;
            }

            if event.has_value("mimecast.newDomain") {
                event.rename("mimecast.newDomain", "source.domain")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("mimecast.taggedExternal") {
                    if let Some(val) = event.get("mimecast.taggedExternal") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "mimecast.taggedExternal".into(),
                                message,
                            }
                        })?;
                        event.set("mimecast.taggedExternal", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_tagged_external_to_boolean",
                )?;
                if event.remove("mimecast.taggedExternal").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "mimecast.taggedExternal".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("mimecast.taggedMalicious") {
                    if let Some(val) = event.get("mimecast.taggedMalicious") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "mimecast.taggedMalicious".into(),
                                message,
                            }
                        })?;
                        event.set("mimecast.taggedMalicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_tagged_malicious_to_boolean",
                )?;
                if event.remove("mimecast.taggedMalicious").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "mimecast.taggedMalicious".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
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

            if event.has_value("mimecast.blockReason") {
                event.rename("mimecast.blockReason", "event.reason")?;
            }

            let _cond = { !event.has_value("email.direction") };
            if _cond {
                if event.has_value("mimecast.route") {
                    event.rename("mimecast.route", "email.direction")?;
                }
            }

            let _cond = { event.has_value("mimecast.sender") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.sender")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("mimecast.senderDomain") {
                event.rename("mimecast.senderDomain", "source.domain")?;
            }

            if event.has_value("mimecast.sourceIp") {
                event.rename("mimecast.sourceIp", "source.ip")?;
            }

            if event.has_value("mimecast.subject") {
                event.rename("mimecast.subject", "email.subject")?;
            }

            if event.has_value("mimecast.url") {
                event.rename("mimecast.url", "url.full")?;
            }

            if event.has_value("mimecast.action") {
                event.rename("mimecast.action", "event.action")?;
            }

            if event.has_value("mimecast.Delivered") {
                event.rename("mimecast.Delivered", "event.outcome")?;
            }

            let _cond = { event.get_bool("event.outcome") == Some(true) };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_bool("event.outcome") == Some(false) };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            if event.has_value("email.direction") {
                map_strings(
                    event,
                    "email.direction",
                    "email.direction",
                    str::to_lowercase,
                )?;
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

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

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = {
                event
                    .get("email.from.address")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "email.from.address", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("email.to.address").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "email.to.address", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("mimecast.Hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("mimecast.Hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("email.attachments.file.hash")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                foreach_array(event, "email.attachments.file.hash", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("email.attachments") };
            if _cond {
                // Painless script
                // Source: def attachments = [];\nattachments.add(ctx.email.attachments);\nctx.email.attachments = attachments;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def attachments = [];\nattachments.add(ctx.email.attachments);\nctx.email.attachments = attachments;\n"#
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("mimecast.recipients");
                event.remove("mimecast.senderEnvelope");
            }

            if event.has_value("original") {
                event.rename_over("original", "mimecast")?;
            }

            event.remove("mimecast._offset");
            event.remove("mimecast._partition");
            event.remove("mimecast.timestamp");

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
