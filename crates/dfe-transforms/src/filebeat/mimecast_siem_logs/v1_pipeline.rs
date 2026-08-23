// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `v1_pipeline` pipeline.
pub struct V1Pipeline;

impl Transform for V1Pipeline {
    fn name(&self) -> &str {
        "v1_pipeline"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("mimecast.datetime") };
            if _cond {
                if let Some(date_str) = event.get_as_string("mimecast.datetime") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ssZ"], Some("UTC"), None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
            }

            let _cond = { event.get("mimecast").is_some_and(|v| v.is_object()) };
            if _cond {
                // Painless script
                // Source: // Canonicalise keys to lowercase. If this causes issues in future\n// because case becomes significant, this table space optimisation\n// will need to be reverted.\ndef keys = new HashSet();\nfor (def k: ctx.mimecast.keySet()) {\n  keys.add(k.toLowerCase());\n}        \nfor (def k: keys) {\n  def typ = params.definite_positive.get(k);\n  if (typ != null) {\n    // We have a definitive known log_type.\n    ctx.mimecast.log_type = typ;\n    return;\n  }\n}\ndef score = params.candidates.clone();\nfor (def k: keys) {\n  def typ = params.negative.get(k);\n  if (typ == null) {\n    continue;\n  }\n  for (String e: typ) {\n    score.remove(e);\n  }\n}\nif (score.size() == 1) {\n  // We have removed all but one of the candidates.\n  ctx.mimecast.log_type = score.keySet().toArray()[0];\n  return;\n}\n// Find best remaining and list all co-equal winners.\nint max = 0;\nfor (def k: keys) {\n  def typ = params.positive.get(k);\n  if (typ == null) {\n    continue;\n  }\n  for (String e: typ) {\n    def s = score.get(e);\n    if (s == null) {\n      continue;\n    }\n    s++;\n    if (s > max) {\n      max = s;\n    }\n    score.put(e, s);\n  }\n}\nfor (def e: score.entrySet()) {\n  if (e.getValue() < max) {\n    score.remove(e.getKey());\n  }\n}\nctx.mimecast.log_type = score.keySet();\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"// Canonicalise keys to lowercase. If this causes issues in future\n// because case becomes significant, this table space optimisation\n// will need to be reverted.\ndef keys = new HashSet();\nfor (def k: ctx.mimecast.keySet()) {\n  keys.add(k.toLowerCase());\n}        \nfor (def k: keys) {\n  def typ = params.definite_positive.get(k);\n  if (typ != null) {\n    // We have a definitive known log_type.\n    ctx.mimecast.log_type = typ;\n    return;\n  }\n}\ndef score = params.candidates.clone();\nfor (def k: keys) {\n  def typ = params.negative.get(k);\n  if (typ == null) {\n    continue;\n  }\n  for (String e: typ) {\n    score.remove(e);\n  }\n}\nif (score.size() == 1) {\n  // We have removed all but one of the candidates.\n  ctx.mimecast.log_type = score.keySet().toArray()[0];\n  return;\n}\n// Find best remaining and list all co-equal winners.\nint max = 0;\nfor (def k: keys) {\n  def typ = params.positive.get(k);\n  if (typ == null) {\n    continue;\n  }\n  for (String e: typ) {\n    def s = score.get(e);\n    if (s == null) {\n      continue;\n    }\n    s++;\n    if (s > max) {\n      max = s;\n    }\n    score.put(e, s);\n  }\n}\nfor (def e: score.entrySet()) {\n  if (e.getValue() < max) {\n    score.remove(e.getKey());\n  }\n}\nctx.mimecast.log_type = score.keySet();\n"#
                    ),
                    cached_params!(
                        "{\"candidates\":{\"attachment-protect\":0,\"avlog\":0,\"delivery\":0,\"impersonation-protect\":0,\"internal-email-protect\":0,\"jrnl\":0,\"process\":0,\"receipt\":0,\"spam\":0,\"url-protect\":0},\"definite_positive\":{\"action\":\"receipt\",\"attempt\":\"delivery\",\"attnames\":\"process\",\"customerip\":\"avlog\",\"customname\":\"impersonation-protect\",\"customthreatdictionary\":\"impersonation-protect\",\"definition\":\"impersonation-protect\",\"delivered\":\"delivery\",\"err\":\"delivery\",\"error\":\"receipt\",\"filename\":\"attachment-protect\",\"hits\":\"impersonation-protect\",\"hld\":\"process\",\"internalname\":\"impersonation-protect\",\"ipinternalname\":\"process\",\"ipnewdomain\":\"process\",\"ipreplymismatch\":\"process\",\"ipsimilardomain\":\"process\",\"ipthreaddict\":\"process\",\"latency\":\"delivery\",\"mimecastip\":\"avlog\",\"msgsize\":\"process\",\"newdomain\":\"impersonation-protect\",\"rcptacttype\":\"jrnl\",\"reason\":\"url-protect\",\"receiptack\":\"delivery\",\"replymismatch\":\"impersonation-protect\",\"scanresultinfo\":\"internal-email-protect\",\"senderdomaininternal\":\"avlog\",\"similarcustomexternaldomain\":\"impersonation-protect\",\"similarinternaldomain\":\"impersonation-protect\",\"similarmimecastexternaldomain\":\"impersonation-protect\",\"snt\":\"delivery\",\"spaminfo\":\"receipt\",\"spamlimit\":\"receipt\",\"spamprocessingdetail\":\"receipt\",\"spamscore\":\"receipt\",\"taggedexternal\":\"impersonation-protect\",\"taggedmalicious\":\"impersonation-protect\",\"threatdictionary\":\"impersonation-protect\",\"usetls\":\"delivery\"},\"negative\":{\"acode\":[\"avlog\",\"url-protect\",\"attachment-protect\"],\"act\":[\"delivery\",\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\",\"jrnl\"],\"attcnt\":[\"receipt\",\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\",\"jrnl\"],\"attsize\":[\"receipt\",\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\",\"jrnl\"],\"cphr\":[\"process\",\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\",\"jrnl\"],\"dir\":[\"process\",\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\"],\"fileext\":[\"receipt\",\"process\",\"delivery\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"jrnl\"],\"filemime\":[\"receipt\",\"process\",\"delivery\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"jrnl\"],\"headerfrom\":[\"process\",\"delivery\",\"avlog\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\",\"jrnl\"],\"ip\":[\"process\",\"spam\",\"internal-email-protect\",\"url-protect\",\"jrnl\"],\"md5\":[\"receipt\",\"process\",\"delivery\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"jrnl\"],\"rcpt\":[\"process\",\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\"],\"recipient\":[\"receipt\",\"process\",\"delivery\",\"jrnl\"],\"rejcode\":[\"process\",\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\",\"jrnl\"],\"rejinfo\":[\"process\",\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\",\"jrnl\"],\"rejtype\":[\"process\",\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\",\"jrnl\"],\"route\":[\"receipt\",\"process\",\"jrnl\"],\"senderdomain\":[\"receipt\",\"process\",\"delivery\",\"internal-email-protect\",\"impersonation-protect\",\"jrnl\"],\"sha1\":[\"receipt\",\"process\",\"delivery\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"jrnl\"],\"sha256\":[\"receipt\",\"process\",\"delivery\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"jrnl\"],\"size\":[\"receipt\",\"process\",\"delivery\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"jrnl\"],\"sourceip\":[\"receipt\",\"process\",\"delivery\",\"avlog\",\"internal-email-protect\",\"impersonation-protect\",\"attachment-protect\",\"jrnl\"],\"tlsver\":[\"process\",\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\",\"jrnl\"],\"url\":[\"receipt\",\"process\",\"delivery\",\"avlog\",\"spam\",\"impersonation-protect\",\"attachment-protect\",\"jrnl\"],\"urlcategory\":[\"receipt\",\"process\",\"delivery\",\"avlog\",\"spam\",\"impersonation-protect\",\"attachment-protect\",\"jrnl\"],\"virus\":[\"process\",\"delivery\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\",\"jrnl\"]},\"positive\":{\"acode\":[\"receipt\",\"process\",\"delivery\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"jrnl\"],\"act\":[\"receipt\",\"process\"],\"attcnt\":[\"process\",\"delivery\"],\"attsize\":[\"process\",\"delivery\"],\"cphr\":[\"receipt\",\"delivery\"],\"dir\":[\"receipt\",\"delivery\",\"jrnl\"],\"fileext\":[\"avlog\",\"attachment-protect\"],\"filemime\":[\"avlog\",\"attachment-protect\"],\"headerfrom\":[\"receipt\",\"spam\"],\"ip\":[\"receipt\",\"delivery\",\"avlog\",\"impersonation-protect\",\"attachment-protect\"],\"md5\":[\"avlog\",\"attachment-protect\"],\"rcpt\":[\"receipt\",\"delivery\",\"jrnl\"],\"recipient\":[\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\"],\"rejcode\":[\"receipt\",\"delivery\"],\"rejinfo\":[\"receipt\",\"delivery\"],\"rejtype\":[\"receipt\",\"delivery\"],\"route\":[\"delivery\",\"avlog\",\"spam\",\"internal-email-protect\",\"impersonation-protect\",\"url-protect\",\"attachment-protect\"],\"senderdomain\":[\"avlog\",\"spam\",\"url-protect\",\"attachment-protect\"],\"sha1\":[\"avlog\",\"attachment-protect\"],\"sha256\":[\"avlog\",\"attachment-protect\"],\"size\":[\"avlog\",\"attachment-protect\"],\"sourceip\":[\"spam\",\"url-protect\"],\"tlsver\":[\"receipt\",\"delivery\"],\"url\":[\"internal-email-protect\",\"url-protect\"],\"urlcategory\":[\"internal-email-protect\",\"url-protect\"],\"virus\":[\"receipt\",\"avlog\"]}}"
                    ),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("mimecast.MsgId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.aCode") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.datetime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.Sender") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.Rcpt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.Attempt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.log_type") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.sha256") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.url") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has("mimecast.aCode") {
                event.rename("mimecast.aCode", "email.local_id")?;
            }

            if event.has("mimecast.Act") {
                event.rename("mimecast.Act", "event.action")?;
            }

            if event.has("mimecast.Cphr") {
                event.rename("mimecast.Cphr", "tls.cipher")?;
            }

            if event.has("mimecast.Dir") {
                event.rename("mimecast.Dir", "email.direction")?;
            }

            if event.has("mimecast.Error") {
                event.rename("mimecast.Error", "error.message")?;
            }

            if event.has("mimecast.IP") {
                event.rename("mimecast.IP", "source.ip")?;
            }

            if event.has("mimecast.MsgId") {
                event.rename("mimecast.MsgId", "email.message_id")?;
            }

            let _cond = { event.has_value("mimecast.Rcpt") };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("mimecast.Rcpt")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("mimecast.headerFrom") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.headerFrom")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("mimecast.RejCode") {
                event.rename("mimecast.RejCode", "error.code")?;
            }

            if event.has("mimecast.RejInfo") {
                event.rename("mimecast.RejInfo", "event.reason")?;
            }

            let _cond = {
                event.has_value("mimecast.RejType") && event.get_str("mimecast.RejType") != Some("")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has("mimecast.RejType") {
                event.rename("mimecast.RejType", "error.type")?;
            }

            let _cond = { event.has_value("mimecast.Sender") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.Sender")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("mimecast.Subject") {
                event.rename("mimecast.Subject", "email.subject")?;
            }

            if event.has("mimecast.TlsVer") {
                event.rename("mimecast.TlsVer", "tls.version")?;
            }

            if event.has("mimecast.AttSize") {
                event.rename("mimecast.AttSize", "email.attachments.file.size")?;
            }

            if event.has("mimecast.AttNames") {
                event.rename("mimecast.AttNames", "email.attachments.file.name")?;
            }

            let _cond =
                { event.has_value("mimecast.Hld") && event.get_str("mimecast.Hld") != Some("") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has("mimecast.Hld") {
                event.rename("mimecast.Hld", "event.reason")?;
            }

            if event.has("mimecast.Err") {
                event.rename("mimecast.Err", "error.message")?;
            }

            if event.has("mimecast.UseTls") {
                event.rename("mimecast.UseTls", "tls.established")?;
            }

            let _cond = {
                event.get("tls.established").is_some_and(|v| v.is_string())
                    && event
                        .get_str("tls.established")
                        .is_some_and(|s| s.to_lowercase() == "yes")
            };
            if _cond {
                event.set("tls.established", json!(true))?;
            }

            let _cond = {
                event.get("tls.established").is_some_and(|v| v.is_string())
                    && event
                        .get_str("tls.established")
                        .is_some_and(|s| s.to_lowercase() == "no")
            };
            if _cond {
                event.set("tls.established", json!(false))?;
            }

            let _cond = {
                event.has_value("mimecast.fileExt") && event.get_str("mimecast.fileExt") != Some("")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has("mimecast.fileExt") {
                event.rename("mimecast.fileExt", "email.attachments.file.extension")?;
            }

            if event.has("mimecast.fileMime") {
                event.rename("mimecast.fileMime", "email.attachments.file.mime_type")?;
            }

            if event.has("mimecast.md5") {
                event.rename("mimecast.md5", "email.attachments.file.hash.md5")?;
            }

            let _cond = { event.has_value("mimecast.Recipient") };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("mimecast.Recipient")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("mimecast.SenderDomain") {
                event.rename("mimecast.SenderDomain", "source.domain")?;
            }

            if event.has("mimecast.sha1") {
                event.rename("mimecast.sha1", "email.attachments.file.hash.sha1")?;
            }

            if event.has("mimecast.sha256") {
                event.rename("mimecast.sha256", "email.attachments.file.hash.sha256")?;
            }

            if event.has("mimecast.Size") {
                event.rename("mimecast.Size", "email.attachments.file.size")?;
            }

            if event.has("mimecast.fileName") {
                event.rename("mimecast.fileName", "email.attachments.file.name")?;
            }

            let _cond = {
                event.has_value("mimecast.SourceIP")
                    && event.get_str("mimecast.SourceIP") != Some("")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has("mimecast.SourceIP") {
                event.rename("mimecast.SourceIP", "source.ip")?;
            }

            let _cond =
                { event.has_value("mimecast.URL") && event.get_str("mimecast.URL") != Some("") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has("mimecast.URL") {
                event.rename("mimecast.URL", "url.full")?;
            }

            let _cond = {
                event.get_bool("mimecast.TaggedMalicious") == Some(true)
                    || event.get_str("mimecast.TaggedMalicious") == Some("true")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has("mimecast.Action") {
                event.rename("mimecast.Action", "event.action")?;
            }

            if event.has("mimecast.Definition") {
                event.rename("mimecast.Definition", "rule.name")?;
            }

            if event.has("mimecast.NewDomain") {
                event.rename("mimecast.NewDomain", "source.domain")?;
            }

            if event.has("mimecast.reason") {
                event.rename("mimecast.reason", "event.reason")?;
            }

            let _cond = { event.has_value("mimecast.recipient") };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("mimecast.recipient")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("mimecast.route") {
                event.rename("mimecast.route", "email.direction")?;
            }

            let _cond = { event.has_value("mimecast.sender") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.sender")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("mimecast.senderDomain") {
                event.rename("mimecast.senderDomain", "source.domain")?;
            }

            if event.has("mimecast.sourceIp") {
                event.rename("mimecast.sourceIp", "source.ip")?;
            }

            if event.has("mimecast.subject") {
                event.rename("mimecast.subject", "email.subject")?;
            }

            if event.has("mimecast.url") {
                event.rename("mimecast.url", "url.full")?;
            }

            if event.has("mimecast.action") {
                event.rename("mimecast.action", "event.action")?;
            }

            let _cond = { event.has_value("mimecast.datetime") };
            if _cond {
                event.set(
                    "event.created",
                    json!(
                        event
                            .get("mimecast.datetime")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("tls.established") == Some("No") };
            if _cond {
                event.set("tls.established", json!(false))?;
            }

            let _cond = { event.get_str("tls.established") == Some("Yes") };
            if _cond {
                event.set("tls.established", json!(true))?;
            }

            if event.has("mimecast.Delivered") {
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
                if let Some(s) = event.get_string("email.direction") {
                    let lowered = s.to_lowercase();
                    event.set("email.direction", lowered)?;
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

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            event.remove("mimecast.eventTime");
            event.remove("mimecast.Content-Disposition");
            event.remove("mimecast.datetime");
            event.remove("mimecast.headerFrom");
            event.remove("mimecast.log_type_part1");
            event.remove("mimecast.log_type_part2");
            event.remove("mimecast.log_type_parts");
            event.remove("mimecast.recipient");
            event.remove("mimecast.Rcpt");
            event.remove("mimecast.sender");
            event.remove("mimecast.Sender");

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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
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
