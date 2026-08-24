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
            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "resp")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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
            }

            let _cond = { event.get_str("input.type") == Some("http_endpoint") };
            if _cond {
                event.remove("json");
            }

            if event.has("resp.event") {
                event.rename("resp.event", "json")?;
            }

            let _cond =
                { event.has_value("json") && event.get_bool("_conf.strict_fields") == Some(true) };
            if _cond {
                // Painless script
                // Source: if (ctx.resp?.version == null) {\n  def fields = [];\n  for (e in ctx.json.entrySet()) {\n    fields.add(e.getKey());\n  }\n  Collections.sort(fields);\n  String signature = String.join(\"|\", fields);\n  if (signature != params.expect.fields) {\n    ctx.error = ctx.error ?: [:];\n    ctx.error.message = ctx.error.message ?: [];\n    ctx.error.message.add(\"field set mismatch: \"+signature+\" is not expected set of templated fields (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n  }\n} else if (ctx.resp.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add(\"template version mismatch: \"+ctx.resp.version.toString()+\" is not expected version (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.resp?.version == null) {\n  def fields = [];\n  for (e in ctx.json.entrySet()) {\n    fields.add(e.getKey());\n  }\n  Collections.sort(fields);\n  String signature = String.join(\"|\", fields);\n  if (signature != params.expect.fields) {\n    ctx.error = ctx.error ?: [:];\n    ctx.error.message = ctx.error.message ?: [];\n    ctx.error.message.add(\"field set mismatch: \"+signature+\" is not expected set of templated fields (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n  }\n} else if (ctx.resp.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add(\"template version mismatch: \"+ctx.resp.version.toString()+\" is not expected version (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n}"#
                    ),
                    cached_params!(
                        "{\"data_stream\":\"web-log\",\"expect\":{\"fields\":\"action|appclass|applayerprotocol|appname|appriskscore|apprulelabel|b64referer|b64url|bamd5|bandwidthclassname|bandwidthrulename|bwthrottle|bypassedtime|bypassedtraffic|client_tls_keyex_alg|client_tls_keyex_hybrid_offers|client_tls_keyex_non_pqc_offers|client_tls_keyex_pqc_offers|client_tls_keyex_unknown_offers|client_tls_sig_alg|client_tls_sig_hybrid_offers|client_tls_sig_non_pqc_offers|client_tls_sig_pqc_offers|client_tls_sig_unknown_offers|cloudname|cltintip|cltip|cltpubip|cltsourceport|cltsslcipher|cltsslfailcount|cltsslfailreason|cltsslsessreuse|clttlsversion|company|contenttype|datacenter|datacentercity|datacentercountry|day|day_of_month|dept|deviceappversion|devicehostname|devicemodel|devicename|deviceostype|deviceosversion|deviceowner|devicetype|df_hosthead|df_hostname|dlpdict|dlpdicthitcount|dlpeng|dlpidentifier|dlpmd5|dlprulename|dstip_country|eedone|epochtime|external_devid|externalsslpolicyreason|fileclass|filename|filesubtype|filetype|flow_type|forward_gateway_ip|forward_gateway_name|forward_type|ft_rulename|host|hour|is_dst_cntry_risky|is_src_cntry_risky|is_sslexpiredca|is_sslselfsigned|is_ssluntrustedca|keyprotectiontype|location|login|malwarecategory|malwareclass|minute|mobappcategory|mobappname|mobdevtype|module|month|month_of_year|nssserviceip|oapprulelabel|obwclassname|ocip|ocpubip|odevicehostname|odevicename|odeviceowner|odlpdict|odlpeng|odlprulename|ofwd_gw_name|ologin|ordr_rulename|ourlcat|ourlfilterrulelabel|ozpa_app_seg_name|productversion|prompt_req|proto|rdr_rulename|reason|recordid|refererhost|reqdatasize|reqheadersize|reqmethod|reqsize|reqversion|respcode|respdatasize|respheadersize|respsize|respversion|riskscore|rulelabel|ruletype|second|server_tls_keyex_alg|server_tls_sig_alg|serverip|serversslsessreuse|sha256|srcip_country|srvcertchainvalpass|srvcertvalidationtype|srvcertvalidityperiod|srvocspresult|srvsslcipher|srvtlsversion|srvwildcardcert|ssl_rulename|ssldecrypted|threatname|threatseverity|throttlereqsize|throttlerespsize|time|totalsize|trafficredirectmethod|tz|unscannabletype|upload_doctypename|upload_fileclass|upload_filename|upload_filesubtype|upload_filetype|urlcatmethod|urlclass|urlfilterrulelabel|urlsubcat|urlsupercat|useragent|useragentclass|useragenttoken|userlocationname|year|zpa_app_seg_name|ztunnelversion\",\"version\":\"v11\"},\"pkg_version\":\"3.17.0\"}"
                    ),
                )?;
            }

            event.remove("resp");

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: // Zscaler hex-encoded data is similar to URL encoded data, but it doesn't encode `%`.\n// This pre-processing allows the URL decode processor's strict logic to run successfully.\ndef pattern = /(%)(?![0-9A-Fa-f]{2})/;\nfor (String k: params.fields) {\n    String v = ctx.json[k];\n    if (v == null) {\n        continue;\n    }\n    ctx.json[k] = pattern.matcher(v).replaceAll('%25');\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"// Zscaler hex-encoded data is similar to URL encoded data, but it doesn't encode `%`.\n// This pre-processing allows the URL decode processor's strict logic to run successfully.\ndef pattern = /(%)(?![0-9A-Fa-f]{2})/;\nfor (String k: params.fields) {\n    String v = ctx.json[k];\n    if (v == null) {\n        continue;\n    }\n    ctx.json[k] = pattern.matcher(v).replaceAll('%25');\n}"#
                    ),
                    cached_params!(
                        "{\"fields\":[\"apprulelabel\",\"devicename\",\"filename\",\"host\",\"location\",\"login\",\"mobappname\",\"refererhost\",\"rulelabel\",\"upload_filename\",\"urlfilterrulelabel\",\"useragent\",\"userlocationname\"]}"
                    ),
                )?;
            }

            event.append("event.category", json!("web"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("access"))?;

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'NA' || object == 'None' || object == 'NotFound') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'NA' || object == 'None' || object == 'NotFound') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);"#
                ),
            )?;

            if event.has("json.action") {
                event.rename("json.action", "zscaler_zia.web.action")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        gsub_field(
                            event,
                            "event.action",
                            "event.action",
                            cached_regex!(" "),
                            "-",
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set("_ingest.on_failure_processor_tag", "gsub_event_action")?;
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
            }

            let _cond = { event.get_str("event.action") == Some("allowed") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("event.action") == Some("denied")
                    || event.get_str("event.action") == Some("blocked")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            if event.has("json.applayerprotocol") {
                event.rename("json.applayerprotocol", "zscaler_zia.web.alpn_protocol")?;
            }

            if event.has("json.appclass") {
                event.rename("json.appclass", "zscaler_zia.web.app.class")?;
            }

            if event.has("json.appname") {
                event.rename("json.appname", "zscaler_zia.web.app.name")?;
            }

            if event.has("json.appriskscore") {
                event.rename("json.appriskscore", "zscaler_zia.web.app.risk_score")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.apprulelabel") {
                    if let Some(s) = event.get_string("json.apprulelabel") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.web.app.rule_label", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.apprulelabel".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set("_ingest.on_failure_processor_tag", "urldecode_apprulelabel")?;
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

            let _cond = { event.has_value("zscaler_zia.web.app.rule_label") };
            if _cond {
                event.append_unique(
                    "rule.name",
                    json!(
                        event
                            .get("zscaler_zia.web.app.rule_label")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.bandwidthclassname") {
                event.rename(
                    "json.bandwidthclassname",
                    "zscaler_zia.web.bandwidth_class_name",
                )?;
            }

            if event.has("json.bandwidthrulename") {
                event.rename(
                    "json.bandwidthrulename",
                    "zscaler_zia.web.bandwidth_rule_name",
                )?;
            }

            if event.has("json.bwthrottle") {
                event.rename("json.bwthrottle", "zscaler_zia.web.bandwidth_throttle")?;
            }

            if event.has("json.bypassedtraffic") {
                event.rename("json.bypassedtraffic", "zscaler_zia.web.bypassed.traffic")?;
            }

            if event.has("json.cltsslcipher") {
                event.rename("json.cltsslcipher", "zscaler_zia.web.client.cipher")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.client.cipher")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.cipher", v)?;
            }

            if event.has("json.cltsslsessreuse") {
                event.rename(
                    "json.cltsslsessreuse",
                    "zscaler_zia.web.client.cipher_reuse",
                )?;
            }

            let _cond = { event.get_str("json.cltintip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.cltintip") {
                        if let Some(val) = event.get("json.cltintip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.cltintip".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.client.internet.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cltintip_to_ip")?;
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
            }

            if let Some(v) = event
                .get("zscaler_zia.web.client.internet.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.nat.ip", v)?;
            }

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.cltip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.cltip") {
                        if let Some(val) = event.get("json.cltip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.cltip".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.client.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cltip_to_ip")?;
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
            }

            if let Some(v) = event
                .get("zscaler_zia.web.client.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
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

            let _cond = { !event.has_value("source.geo") };
            if _cond {
                if event.has_value("source.nat.ip") {
                    if let Some(ip_str) = event.get_string("source.nat.ip") {
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
            }

            let _cond = { event.get_str("json.cltpubip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.cltpubip") {
                        if let Some(val) = event.get("json.cltpubip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.cltpubip".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.client.public_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cltpubip_to_ip")?;
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
            }

            let _cond = { event.has_value("zscaler_zia.web.client.public_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("zscaler_zia.web.client.public_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.cltsourceport") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.cltsourceport") {
                        if let Some(val) = event.get("json.cltsourceport") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.cltsourceport".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.client.source_port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cltsourceport_to_long",
                    )?;
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
            }

            if let Some(v) = event
                .get("zscaler_zia.web.client.source_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            let _cond = { event.get_str("json.cltsslfailcount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.cltsslfailcount") {
                        if let Some(val) = event.get("json.cltsslfailcount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.cltsslfailcount".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.client.ssl.fail_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cltsslfailcount_to_long",
                    )?;
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
            }

            if event.has("json.ssl_rulename") {
                event.rename("json.ssl_rulename", "zscaler_zia.web.ssl_rulename")?;
            }

            if event.has("json.ft_rulename") {
                event.rename("json.ft_rulename", "zscaler_zia.web.ft_rulename")?;
            }

            if event.has("json.cltsslfailreason") {
                event.rename(
                    "json.cltsslfailreason",
                    "zscaler_zia.web.client.ssl.fail_reason",
                )?;
            }

            if event.has("json.clttlsversion") {
                event.rename("json.clttlsversion", "zscaler_zia.web.client.tls_version")?;
            }

            let _cond = { event.get_str("json.client_tls_keyex_pqc_offers") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.client_tls_keyex_pqc_offers") {
                        if let Some(val) = event.get("json.client_tls_keyex_pqc_offers") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.client_tls_keyex_pqc_offers".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.client.tls_keyex_pqc_offers", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_tls_keyex_pqc_offers",
                    )?;
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
            }

            let _cond = { event.get_str("json.client_tls_keyex_non_pqc_offers") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.client_tls_keyex_non_pqc_offers") {
                        if let Some(val) = event.get("json.client_tls_keyex_non_pqc_offers") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.client_tls_keyex_non_pqc_offers".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "zscaler_zia.web.client.tls_keyex_non_pqc_offers",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_tls_keyex_non_pqc_offers",
                    )?;
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
            }

            let _cond = { event.get_str("json.client_tls_keyex_hybrid_offers") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.client_tls_keyex_hybrid_offers") {
                        if let Some(val) = event.get("json.client_tls_keyex_hybrid_offers") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.client_tls_keyex_hybrid_offers".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("zscaler_zia.web.client.tls_keyex_hybrid_offers", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_tls_keyex_hybrid_offers",
                    )?;
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
            }

            let _cond = { event.get_str("json.client_tls_keyex_unknown_offers") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.client_tls_keyex_unknown_offers") {
                        if let Some(val) = event.get("json.client_tls_keyex_unknown_offers") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.client_tls_keyex_unknown_offers".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "zscaler_zia.web.client.tls_keyex_unknown_offers",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_tls_keyex_unknown_offers",
                    )?;
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
            }

            let _cond = { event.get_str("json.hour") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.client_tls_sig_pqc_offers") {
                        if let Some(val) = event.get("json.client_tls_sig_pqc_offers") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.client_tls_sig_pqc_offers".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.client.tls_sig_pqc_offers", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_tls_sig_pqc_offers",
                    )?;
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
            }

            let _cond = { event.get_str("json.client_tls_sig_non_pqc_offers") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.client_tls_sig_non_pqc_offers") {
                        if let Some(val) = event.get("json.client_tls_sig_non_pqc_offers") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.client_tls_sig_non_pqc_offers".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("zscaler_zia.web.client.tls_sig_non_pqc_offers", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_tls_sig_non_pqc_offers",
                    )?;
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
            }

            let _cond = { event.get_str("json.client_tls_sig_hybrid_offers") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.client_tls_sig_hybrid_offers") {
                        if let Some(val) = event.get("json.client_tls_sig_hybrid_offers") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.client_tls_sig_hybrid_offers".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.client.tls_sig_hybrid_offers", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_tls_sig_hybrid_offers",
                    )?;
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
            }

            let _cond = { event.get_str("json.client_tls_sig_unknown_offers") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.client_tls_sig_unknown_offers") {
                        if let Some(val) = event.get("json.client_tls_sig_unknown_offers") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.client_tls_sig_unknown_offers".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("zscaler_zia.web.client.tls_sig_unknown_offers", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_tls_sig_unknown_offers",
                    )?;
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
            }

            if event.has("json.client_tls_keyex_alg") {
                event.rename(
                    "json.client_tls_keyex_alg",
                    "zscaler_zia.web.client.tls_keyex_alg",
                )?;
            }

            if event.has("json.client_tls_sig_alg") {
                event.rename(
                    "json.client_tls_sig_alg",
                    "zscaler_zia.web.client.tls_sig_alg",
                )?;
            }

            if event.has("json.cloudname") {
                event.rename("json.cloudname", "zscaler_zia.web.cloud_name")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.cloud_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.provider", v)?;
            }

            if event.has("json.company") {
                event.rename("json.company", "zscaler_zia.web.company")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.company")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            if event.has("json.contenttype") {
                event.rename("json.contenttype", "zscaler_zia.web.content_type")?;
            }

            if event.has("json.datacentercity") {
                event.rename("json.datacentercity", "zscaler_zia.web.datacenter.city")?;
            }

            if event.has("json.datacentercountry") {
                event.rename(
                    "json.datacentercountry",
                    "zscaler_zia.web.datacenter.country",
                )?;
            }

            if event.has("json.datacenter") {
                event.rename("json.datacenter", "zscaler_zia.web.datacenter.name")?;
            }

            if event.has("json.day") {
                event.rename("json.day", "zscaler_zia.web.day")?;
            }

            let _cond = { event.get_str("json.day_of_month") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.day_of_month") {
                        if let Some(val) = event.get("json.day_of_month") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.day_of_month".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.day_of_month", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_day_of_month_to_long",
                    )?;
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
            }

            if event.has("json.dept") {
                event.rename("json.dept", "zscaler_zia.web.department")?;
            }

            if event.has("json.deviceappversion") {
                event.rename("json.deviceappversion", "zscaler_zia.web.device.appversion")?;
            }

            if event.has("json.devicehostname") {
                event.rename("json.devicehostname", "zscaler_zia.web.device.hostname")?;
            }

            let _cond = { event.get_str("zscaler_zia.web.device.hostname") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("zscaler_zia.web.device.hostname") {
                        map_strings(
                            event,
                            "zscaler_zia.web.device.hostname",
                            "host.name",
                            str::to_lowercase,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "lowercase")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "lowercase_device_hostname",
                    )?;
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
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.devicemodel") {
                event.rename("json.devicemodel", "zscaler_zia.web.device.model")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.device.model")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.model.identifier", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.devicename") {
                    if let Some(s) = event.get_string("json.devicename") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.web.device.name", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.devicename".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set("_ingest.on_failure_processor_tag", "urldecode_devicename")?;
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

            if let Some(v) = event
                .get("zscaler_zia.web.device.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.deviceostype") {
                event.rename("json.deviceostype", "zscaler_zia.web.device.os.type")?;
            }

            let _cond = {
                event.has_value("zscaler_zia.web.device.os.type")
                    && event.get_str("zscaler_zia.web.device.os.type") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: String osType = ctx.zscaler_zia.web.device.os.type;\nif (ctx.host == null) {\n    Map map = new HashMap();\n    ctx.put('host', map);\n}\nif (ctx.host?.os == null) {\n    Map map = new HashMap();\n    ctx.host.put('os', map);\n}\nif (osType == 'iOS') {\n   ctx.host.os.put('type', 'ios');\n}\nelse if (osType == 'Android OS') {\n   ctx.host.os.put('type', 'android');\n}\nelse if (osType == 'Windows OS') {\n   ctx.host.os.put('type', 'windows');\n}\nelse if (osType == 'MAC OS') {\n   ctx.host.os.put('type', 'macos');\n}\nelse if (osType == 'Other OS') {\n   ctx.host.os.put('type', 'other');\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String osType = ctx.zscaler_zia.web.device.os.type;\nif (ctx.host == null) {\n    Map map = new HashMap();\n    ctx.put('host', map);\n}\nif (ctx.host?.os == null) {\n    Map map = new HashMap();\n    ctx.host.put('os', map);\n}\nif (osType == 'iOS') {\n   ctx.host.os.put('type', 'ios');\n}\nelse if (osType == 'Android OS') {\n   ctx.host.os.put('type', 'android');\n}\nelse if (osType == 'Windows OS') {\n   ctx.host.os.put('type', 'windows');\n}\nelse if (osType == 'MAC OS') {\n   ctx.host.os.put('type', 'macos');\n}\nelse if (osType == 'Other OS') {\n   ctx.host.os.put('type', 'other');\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_extract_device_os_type",
                    )?;
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
            }

            if event.has("json.deviceosversion") {
                event.rename("json.deviceosversion", "zscaler_zia.web.device.os.version")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.device.os.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if event.has("json.deviceowner") {
                event.rename("json.deviceowner", "zscaler_zia.web.device.owner")?;
            }

            let _cond = { event.has_value("zscaler_zia.web.device.owner") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.web.device.owner")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.devicetype") {
                event.rename("json.devicetype", "zscaler_zia.web.device.type")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.device.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.type", v)?;
            }

            if event.has("json.df_hosthead") {
                event.rename("json.df_hosthead", "zscaler_zia.web.df.host.head")?;
            }

            if event.has("json.df_hostname") {
                event.rename("json.df_hostname", "zscaler_zia.web.df.host.name")?;
            }

            if event.has("json.dlpdicthitcount") {
                event.rename(
                    "json.dlpdicthitcount",
                    "zscaler_zia.web.dlp.dictionaries.hit_count",
                )?;
            }

            if event.has("json.dlpdict") {
                event.rename("json.dlpdict", "zscaler_zia.web.dlp.dictionaries.name")?;
            }

            if event.has("json.dlpeng") {
                event.rename("json.dlpeng", "zscaler_zia.web.dlp.engine")?;
            }

            if event.has_value("json.dlpidentifier") {
                if let Some(val) = event.get("json.dlpidentifier") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.dlpidentifier".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.web.dlp.identifier", converted)?;
                }
            }

            if event.has("json.dlpmd5") {
                event.rename("json.dlpmd5", "zscaler_zia.web.dlp.md5")?;
            }

            let _cond = { event.has_value("zscaler_zia.web.dlp.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("zscaler_zia.web.dlp.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.dlprulename") {
                event.rename("json.dlprulename", "zscaler_zia.web.dlp.rule.name")?;
            }

            let _cond = { event.has_value("zscaler_zia.web.dlp.rule.name") };
            if _cond {
                event.append_unique(
                    "rule.name",
                    json!(
                        event
                            .get("zscaler_zia.web.dlp.rule.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.eedone") {
                event.rename("json.eedone", "zscaler_zia.web.eedone")?;
            }

            let _cond = {
                event.has_value("json.epochtime") && event.get_str("json.epochtime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.epochtime") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("zscaler_zia.web.epochtime", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.epochtime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_epochtime")?;
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
            }

            if event.has("json.external_devid") {
                event.rename("json.external_devid", "zscaler_zia.web.external.device.id")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.external.device.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            if event.has("json.fileclass") {
                event.rename("json.fileclass", "zscaler_zia.web.file.class")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.filename") {
                    if let Some(s) = event.get_string("json.filename") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.web.file.name", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.filename".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set("_ingest.on_failure_processor_tag", "urldecode_filename")?;
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

            let _cond = { event.has_value("zscaler_zia.web.file.name") };
            if _cond {
                event.append_unique(
                    "file.name",
                    json!(
                        event
                            .get("zscaler_zia.web.file.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.filesubtype") {
                event.rename("json.filesubtype", "zscaler_zia.web.file.subtype")?;
            }

            let _cond = { event.has_value("zscaler_zia.web.file.subtype") };
            if _cond {
                event.append_unique(
                    "file.extension",
                    json!(
                        event
                            .get("zscaler_zia.web.file.subtype")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.filetype") {
                event.rename("json.filetype", "zscaler_zia.web.file.type")?;
            }

            event.set("file.type", json!("file"))?;

            if event.has("json.flow_type") {
                event.rename("json.flow_type", "zscaler_zia.web.flow_type")?;
            }

            let _cond = { event.get_str("json.forward_gateway_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.forward_gateway_ip") {
                        if let Some(val) = event.get("json.forward_gateway_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.forward_gateway_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.forward_gateway.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_forward_gateway_ip_to_ip",
                    )?;
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
            }

            let _cond = { event.has_value("zscaler_zia.web.forward_gateway.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("zscaler_zia.web.forward_gateway.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.forward_gateway_name") {
                event.rename(
                    "json.forward_gateway_name",
                    "zscaler_zia.web.forward_gateway.name",
                )?;
            }

            if event.has("json.forward_type") {
                event.rename("json.forward_type", "zscaler_zia.web.forward_type")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.host") {
                    if let Some(s) = event.get_string("json.host") {
                        match url_decode(&s) {
                            Some(decoded) => event.set("zscaler_zia.web.host", json!(decoded))?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.host".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set("_ingest.on_failure_processor_tag", "urldecode_host")?;
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

            if let Some(v) = event
                .get("zscaler_zia.web.host")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            let _cond = { event.get_str("json.hour") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.hour") {
                        if let Some(val) = event.get("json.hour") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.hour".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.hour", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_hour_to_long")?;
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
            }

            if event.has("json.is_sslexpiredca") {
                event.rename(
                    "json.is_sslexpiredca",
                    "zscaler_zia.web.is_ssl_certificate_expired",
                )?;
            }

            if event.has("json.is_sslselfsigned") {
                event.rename(
                    "json.is_sslselfsigned",
                    "zscaler_zia.web.is_ssl_certificate_selfsigned",
                )?;
            }

            if event.has("json.is_ssluntrustedca") {
                event.rename(
                    "json.is_ssluntrustedca",
                    "zscaler_zia.web.is_ssl_certificate_untrusted",
                )?;
            }

            if event.has("json.is_src_cntry_risky") {
                event.rename(
                    "json.is_src_cntry_risky",
                    "zscaler_zia.web.is_src_cntry_risky",
                )?;
            }

            if event.has("json.srcip_country") {
                event.rename("json.srcip_country", "zscaler_zia.web.srcip_country")?;
            }

            if event.has("json.dstip_country") {
                event.rename("json.dstip_country", "zscaler_zia.web.dstip_country")?;
            }

            if event.has("json.is_dst_cntry_risky") {
                event.rename(
                    "json.is_dst_cntry_risky",
                    "zscaler_zia.web.is_dst_cntry_risky",
                )?;
            }

            if event.has("json.keyprotectiontype") {
                event.rename(
                    "json.keyprotectiontype",
                    "zscaler_zia.web.key_protection_type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.location") {
                    if let Some(s) = event.get_string("json.location") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.web.location", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.location".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set("_ingest.on_failure_processor_tag", "urldecode_location")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.login") {
                    if let Some(s) = event.get_string("json.login") {
                        match url_decode(&s) {
                            Some(decoded) => event.set("zscaler_zia.web.login", json!(decoded))?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.login".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set("_ingest.on_failure_processor_tag", "urldecode_login")?;
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

            if let Some(v) = event
                .get("zscaler_zia.web.login")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "user.email".into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "create_user_name_and_user_domain",
                    )?;
                    if let Some(v) = event.get("user.email").cloned() {
                        event.set("user.name", v)?;
                    }
                    if event.remove("user.email").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "user.email".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
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

            if event.has("json.malwarecategory") {
                event.rename("json.malwarecategory", "zscaler_zia.web.malware.category")?;
            }

            if event.has("json.malwareclass") {
                event.rename("json.malwareclass", "zscaler_zia.web.malware.class")?;
            }

            if event.has("json.bamd5") {
                event.rename("json.bamd5", "zscaler_zia.web.md5_hash")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.md5_hash")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.md5_hash")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.minute") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.minute") {
                        if let Some(val) = event.get("json.minute") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.minute".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.minute", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_minute_to_long")?;
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
            }

            if event.has("json.mobappcategory") {
                event.rename(
                    "json.mobappcategory",
                    "zscaler_zia.web.mobile.application.category",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.mobappname") {
                    if let Some(s) = event.get_string("json.mobappname") {
                        match url_decode(&s) {
                            Some(decoded) => event
                                .set("zscaler_zia.web.mobile.application.name", json!(decoded))?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.mobappname".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set("_ingest.on_failure_processor_tag", "urldecode_mobappname")?;
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

            if event.has("json.mobdevtype") {
                event.rename("json.mobdevtype", "zscaler_zia.web.mobile.dev.type")?;
            }

            if event.has("json.module") {
                event.rename("json.module", "zscaler_zia.web.module")?;
            }

            if event.has("json.month") {
                event.rename("json.month", "zscaler_zia.web.month")?;
            }

            let _cond = { event.get_str("json.month_of_year") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.month_of_year") {
                        if let Some(val) = event.get("json.month_of_year") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.month_of_year".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.month_of_year", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_month_of_year_to_long",
                    )?;
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
            }

            let _cond = { event.get_str("json.nssserviceip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.nssserviceip") {
                        if let Some(val) = event.get("json.nssserviceip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.nssserviceip".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.nss.service.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_nssserviceip_to_ip",
                    )?;
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
            }

            let _cond = { event.has_value("zscaler_zia.web.nss.service.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("zscaler_zia.web.nss.service.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.oapprulelabel") {
                event.rename(
                    "json.oapprulelabel",
                    "zscaler_zia.web.obfuscated.app_rule_label",
                )?;
            }

            if event.has("json.obwclassname") {
                event.rename(
                    "json.obwclassname",
                    "zscaler_zia.web.obfuscated.bendwidth.class_name",
                )?;
            }

            if event.has_value("json.ocip") {
                if let Some(val) = event.get("json.ocip") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.ocip".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.web.obfuscated.client.ip", converted)?;
                }
            }

            if event.has_value("json.ocpubip") {
                if let Some(val) = event.get("json.ocpubip") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.ocpubip".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.web.obfuscated.client.public.ip", converted)?;
                }
            }

            if event.has("json.odevicehostname") {
                event.rename(
                    "json.odevicehostname",
                    "zscaler_zia.web.obfuscated.device.host_name",
                )?;
            }

            if event.has("json.odevicename") {
                event.rename("json.odevicename", "zscaler_zia.web.obfuscated.device.name")?;
            }

            if event.has("json.odeviceowner") {
                event.rename(
                    "json.odeviceowner",
                    "zscaler_zia.web.obfuscated.device.owner",
                )?;
            }

            if event.has("json.odlpdict") {
                event.rename(
                    "json.odlpdict",
                    "zscaler_zia.web.obfuscated.dlp.dictionaries",
                )?;
            }

            if event.has("json.odlpeng") {
                event.rename("json.odlpeng", "zscaler_zia.web.obfuscated.dlp.engine")?;
            }

            if event.has("json.odlprulename") {
                event.rename(
                    "json.odlprulename",
                    "zscaler_zia.web.obfuscated.dlp.rule.name",
                )?;
            }

            if event.has("json.ofwd_gw_name") {
                event.rename(
                    "json.ofwd_gw_name",
                    "zscaler_zia.web.obfuscated.forward_gateway_name",
                )?;
            }

            if event.has("json.ologin") {
                event.rename("json.ologin", "zscaler_zia.web.obfuscated.login")?;
            }

            if event.has("json.ordr_rulename") {
                event.rename("json.ordr_rulename", "zscaler_zia.web.obfuscated.rule.name")?;
            }

            if event.has("json.ourlcat") {
                event.rename("json.ourlcat", "zscaler_zia.web.obfuscated.url.category")?;
            }

            if event.has("json.ourlfilterrulelabel") {
                event.rename(
                    "json.ourlfilterrulelabel",
                    "zscaler_zia.web.obfuscated.url.filter_rule_label",
                )?;
            }

            if event.has("json.ozpa_app_seg_name") {
                event.rename(
                    "json.ozpa_app_seg_name",
                    "zscaler_zia.web.obfuscated.zpa_app_segment",
                )?;
            }

            if event.has("json.externalsslpolicyreason") {
                event.rename(
                    "json.externalsslpolicyreason",
                    "zscaler_zia.web.policy.reason",
                )?;
            }

            if event.has("json.productversion") {
                event.rename("json.productversion", "zscaler_zia.web.product_version")?;
            }

            if event.has("json.prompt_req") {
                event.rename("json.prompt_req", "zscaler_zia.web.prompt_req")?;
            }

            if event.has("json.proto") {
                event.rename("json.proto", "zscaler_zia.web.prototype")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.prototype")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.protocol", v)?;
            }

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            if event.has("json.reason") {
                event.rename("json.reason", "zscaler_zia.web.reason")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.reason")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            if event.has_value("json.recordid") {
                if let Some(val) = event.get("json.recordid") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.recordid".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.web.record.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("zscaler_zia.web.record.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has("json.rdr_rulename") {
                event.rename("json.rdr_rulename", "zscaler_zia.web.redirect_policy_name")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.refererhost") {
                    if let Some(s) = event.get_string("json.refererhost") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.web.referer.host", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.refererhost".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set("_ingest.on_failure_processor_tag", "urldecode_refererhost")?;
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

            let _cond = {
                event.has_value("json.b64referer") && event.get_str("json.b64referer") != Some("")
            };
            if _cond {
                // Painless script
                // Source: ctx.json.referer = ctx.json.b64referer.decodeBase64();
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.json.referer = ctx.json.b64referer.decodeBase64();"#),
                )?;
            }

            if let Some(v) = event
                .get("json.referer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("zscaler_zia.web.referer.name", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.referer.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.referrer", v)?;
            }

            let _cond = { event.get_str("json.reqheadersize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.reqheadersize") {
                        if let Some(val) = event.get("json.reqheadersize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.reqheadersize".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.request.header_size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_reqheadersize_to_long",
                    )?;
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
            }

            if event.has("json.reqmethod") {
                event.rename("json.reqmethod", "zscaler_zia.web.request.method")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.request.method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            let _cond = { event.get_str("json.reqdatasize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.reqdatasize") {
                        if let Some(val) = event.get("json.reqdatasize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.reqdatasize".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.request.payload", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_reqdatasize_to_long",
                    )?;
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
            }

            let _cond = { event.get_str("json.reqsize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.reqsize") {
                        if let Some(val) = event.get("json.reqsize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.reqsize".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.request.size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_reqsize_to_long",
                    )?;
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
            }

            if let Some(v) = event
                .get("zscaler_zia.web.request.size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.bytes", v)?;
            }

            if event.has("json.reqversion") {
                event.rename("json.reqversion", "zscaler_zia.web.request.version")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.request.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.version", v)?;
            }

            if event.has("json.respcode") {
                event.rename("json.respcode", "zscaler_zia.web.response.code")?;
            }

            let _cond = { event.get_str("json.respheadersize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.respheadersize") {
                        if let Some(val) = event.get("json.respheadersize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.respheadersize".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.response.header_size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_respheadersize_to_long",
                    )?;
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
            }

            let _cond = { event.get_str("json.respdatasize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.respdatasize") {
                        if let Some(val) = event.get("json.respdatasize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.respdatasize".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.response.payload", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_respdatasize_to_long",
                    )?;
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
            }

            let _cond = { event.get_str("json.respsize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.respsize") {
                        if let Some(val) = event.get("json.respsize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.respsize".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.response.size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_respsize_to_long",
                    )?;
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
            }

            if let Some(v) = event
                .get("zscaler_zia.web.response.size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.bytes", v)?;
            }

            if event.has("json.respversion") {
                event.rename("json.respversion", "zscaler_zia.web.response.version")?;
            }

            let _cond = { event.has_value("zscaler_zia.web.response.version") };
            if _cond {
                event.append_unique(
                    "http.version",
                    json!(
                        event
                            .get("zscaler_zia.web.response.version")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.riskscore") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.riskscore") {
                        if let Some(val) = event.get("json.riskscore") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.riskscore".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.risk.score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_riskscore_to_long",
                    )?;
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
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.rulelabel") {
                    if let Some(s) = event.get_string("json.rulelabel") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.web.rule.name", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.rulelabel".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set("_ingest.on_failure_processor_tag", "urldecode_rulelabel")?;
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

            let _cond = { event.has_value("zscaler_zia.web.rule.name") };
            if _cond {
                event.append_unique(
                    "rule.name",
                    json!(
                        event
                            .get("zscaler_zia.web.rule.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.ruletype") {
                event.rename("json.ruletype", "zscaler_zia.web.rule.type")?;
            }

            let _cond = { event.get_str("json.second") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.second") {
                        if let Some(val) = event.get("json.second") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.second".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.second", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_second_to_long")?;
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
            }

            if event.has("json.srvcertvalidityperiod") {
                event.rename(
                    "json.srvcertvalidityperiod",
                    "zscaler_zia.web.server.certificate.validation.period",
                )?;
            }

            if event.has("json.srvcertchainvalpass") {
                event.rename(
                    "json.srvcertchainvalpass",
                    "zscaler_zia.web.server.certificate_validation_chain",
                )?;
            }

            if event.has("json.srvcertvalidationtype") {
                event.rename(
                    "json.srvcertvalidationtype",
                    "zscaler_zia.web.server.certificate_validation_type",
                )?;
            }

            if event.has("json.srvsslcipher") {
                event.rename("json.srvsslcipher", "zscaler_zia.web.server.cipher")?;
            }

            if event.has("json.serversslsessreuse") {
                event.rename(
                    "json.serversslsessreuse",
                    "zscaler_zia.web.server.cipher_reuse",
                )?;
            }

            let _cond = { event.get_str("json.serverip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.serverip") {
                        if let Some(val) = event.get("json.serverip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.serverip".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.server.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_serverip_to_ip")?;
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
            }

            if let Some(v) = event
                .get("zscaler_zia.web.server.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
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

            if event.has("json.srvocspresult") {
                event.rename("json.srvocspresult", "zscaler_zia.web.server.ocsp_result")?;
            }

            if event.has("json.srvtlsversion") {
                event.rename("json.srvtlsversion", "zscaler_zia.web.server.tls_version")?;
            }

            if event.has("json.srvwildcardcert") {
                event.rename(
                    "json.srvwildcardcert",
                    "zscaler_zia.web.server.wildcard_certificate",
                )?;
            }

            if event.has("json.server_tls_keyex_alg") {
                event.rename(
                    "json.server_tls_keyex_alg",
                    "zscaler_zia.web.server.tls_keyex_alg",
                )?;
            }

            if event.has("json.server_tls_sig_alg") {
                event.rename(
                    "json.server_tls_sig_alg",
                    "zscaler_zia.web.server.tls_sig_alg",
                )?;
            }

            if event.has("json.sha256") {
                event.rename("json.sha256", "zscaler_zia.web.sha256")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.ssldecrypted") {
                event.rename("json.ssldecrypted", "zscaler_zia.web.ssl_decrypted")?;
            }

            if event.has("json.threatname") {
                event.rename("json.threatname", "zscaler_zia.web.threat.name")?;
            }

            if event.has("json.threatseverity") {
                event.rename("json.threatseverity", "zscaler_zia.web.threat.severity")?;
            }

            let _cond = { event.get_str("json.throttlereqsize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.throttlereqsize") {
                        if let Some(val) = event.get("json.throttlereqsize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.throttlereqsize".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.throttle.request_size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_throttlereqsize_to_long",
                    )?;
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
            }

            let _cond = { event.get_str("json.throttlerespsize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.throttlerespsize") {
                        if let Some(val) = event.get("json.throttlerespsize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.throttlerespsize".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.throttle.response_size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_throttlerespsize_to_long",
                    )?;
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
            }

            if event.has("json.tz") {
                event.rename("json.tz", "zscaler_zia.web.timezone")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.web.timezone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                event.set("event.timezone", json!("UTC"))?;
            }

            let _cond = { event.has_value("json.time") && event.get_str("json.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-MM-dd HH:mm:ss",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("zscaler_zia.web.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_time")?;
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
            }

            if let Some(v) = event
                .get("zscaler_zia.web.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("json.bypassedtime")
                    && event.get_str("json.bypassedtime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.bypassedtime") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-MM-dd HH:mm:ss",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("zscaler_zia.web.bypassed.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.bypassedtime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_bypassedtime")?;
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
            }

            let _cond = { event.get_str("json.totalsize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.totalsize") {
                        if let Some(val) = event.get("json.totalsize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.totalsize".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.total.size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_totalsize_to_long",
                    )?;
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
            }

            if event.has("json.trafficredirectmethod") {
                event.rename(
                    "json.trafficredirectmethod",
                    "zscaler_zia.web.traffic_redirect_method",
                )?;
            }

            if event.has("json.unscannabletype") {
                event.rename("json.unscannabletype", "zscaler_zia.web.unscannable.type")?;
            }

            if event.has("json.upload_doctypename") {
                event.rename(
                    "json.upload_doctypename",
                    "zscaler_zia.web.upload.doc.type_name",
                )?;
            }

            if event.has("json.upload_fileclass") {
                event.rename("json.upload_fileclass", "zscaler_zia.web.upload.file.class")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.upload_filename") {
                    if let Some(s) = event.get_string("json.upload_filename") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.web.upload.file.name", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.upload_filename".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "urldecode_upload_filename",
                )?;
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

            let _cond = { event.has_value("zscaler_zia.web.upload.file.name") };
            if _cond {
                event.append_unique(
                    "file.name",
                    json!(
                        event
                            .get("zscaler_zia.web.upload.file.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.upload_filesubtype") {
                event.rename(
                    "json.upload_filesubtype",
                    "zscaler_zia.web.upload.file.subtype",
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.web.upload.file.subtype") };
            if _cond {
                event.append_unique(
                    "file.extension",
                    json!(
                        event
                            .get("zscaler_zia.web.upload.file.subtype")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.upload_filetype") {
                event.rename("json.upload_filetype", "zscaler_zia.web.upload.file.type")?;
            }

            if event.has("json.urlsubcat") {
                event.rename("json.urlsubcat", "zscaler_zia.web.url.category.sub")?;
            }

            if event.has("json.urlsupercat") {
                event.rename("json.urlsupercat", "zscaler_zia.web.url.category.super")?;
            }

            if event.has("json.urlcatmethod") {
                event.rename("json.urlcatmethod", "zscaler_zia.web.url.category_method")?;
            }

            if event.has("json.urlclass") {
                event.rename("json.urlclass", "zscaler_zia.web.url.class")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.urlfilterrulelabel") {
                    if let Some(s) = event.get_string("json.urlfilterrulelabel") {
                        match url_decode(&s) {
                            Some(decoded) => event
                                .set("zscaler_zia.web.url.filter_rule_label", json!(decoded))?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.urlfilterrulelabel".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "urldecode_urlfilterrulelabel",
                )?;
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

            let _cond = { event.has_value("zscaler_zia.web.url.filter_rule_label") };
            if _cond {
                event.append_unique(
                    "rule.name",
                    json!(
                        event
                            .get("zscaler_zia.web.url.filter_rule_label")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("json.b64url") && event.get_str("json.b64url") != Some("") };
            if _cond {
                // Painless script
                // Source: ctx.json.url = ctx.json.b64url.decodeBase64();
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.json.url = ctx.json.b64url.decodeBase64();"#),
                )?;
            }

            if let Some(v) = event
                .get("json.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("zscaler_zia.web.url.name", v)?;
            }

            let _cond = { event.has_value("network.protocol") && event.has_value("json.url") };
            if _cond {
                // Painless script
                // Source: // Remap network.protocol to a valid value, if necessary.\nif (params.valid_protocols.contains(ctx.network.protocol)) {\n  ctx.json['url'] = ctx.network.protocol + '://' + ctx.json.url;\n} else {\n  ctx.json['url'] = params.default_protocol + '://' + ctx.json.url;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"// Remap network.protocol to a valid value, if necessary.\nif (params.valid_protocols.contains(ctx.network.protocol)) {\n  ctx.json['url'] = ctx.network.protocol + '://' + ctx.json.url;\n} else {\n  ctx.json['url'] = params.default_protocol + '://' + ctx.json.url;\n}\n"#
                    ),
                    cached_params!(
                        "{\"default_protocol\":\"https\",\"valid_protocols\":[\"http\",\"https\"]}"
                    ),
                )?;
            }

            let _cond = { event.has_value("json.url") && event.get_str("json.url") != Some("") };
            if _cond {
                uri_parts(event, "json.url", "url", true, false)?;
            }

            let _cond =
                { event.has_value("url.original") && event.get_str("url.original") != Some("") };
            if _cond {
                event.set(
                    "url.full",
                    json!(
                        event
                            .get("url.original")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.useragentclass") {
                event.rename("json.useragentclass", "zscaler_zia.web.user_agent.class")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.useragent") {
                    if let Some(s) = event.get_string("json.useragent") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.web.user_agent.name", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.useragent".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("zscaler_zia.web.user_agent.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.original", v)?;
            }

            let _cond = { event.get_str("user_agent.original") != Some("") };
            if _cond {
                if event.has_value("user_agent.original") {
                    if let Some(ua_str) = event.get_string("user_agent.original") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.set("user_agent.original", json!(ua_str))?;
                            if let Some(name) = ua.name {
                                event.set("user_agent.name", json!(name))?;
                            }
                            if let Some(version) = ua.version {
                                event.set("user_agent.version", json!(version))?;
                            }
                            if let Some(os_name) = ua.os_name {
                                event.set("user_agent.os.name", json!(os_name))?;
                                if let Some(os_version) = ua.os_version {
                                    event.set("user_agent.os.version", json!(os_version))?;
                                    event.set(
                                        "user_agent.os.full",
                                        json!(format!("{} {}", os_name, os_version)),
                                    )?;
                                }
                            }
                            if let Some(device) = ua.device {
                                event.set("user_agent.device.name", json!(device))?;
                            }
                        }
                    }
                }
            }

            if event.has("json.useragenttoken") {
                event.rename("json.useragenttoken", "zscaler_zia.web.user_agent.token")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.userlocationname") {
                    if let Some(s) = event.get_string("json.userlocationname") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.web.user_location_name", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.userlocationname".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "urldecode_userlocationname",
                )?;
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

            let _cond = { event.get_str("json.year") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.year") {
                        if let Some(val) = event.get("json.year") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.year".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.web.year", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_year_to_long")?;
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
            }

            if event.has("json.ztunnelversion") {
                event.rename("json.ztunnelversion", "zscaler_zia.web.z_tunnel_version")?;
            }

            if event.has("json.zpa_app_seg_name") {
                event.rename("json.zpa_app_seg_name", "zscaler_zia.web.zpa_app_segment")?;
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
                event.remove("zscaler_zia.web.action");
                event.remove("zscaler_zia.web.app.rule_label");
                event.remove("zscaler_zia.web.client.cipher");
                event.remove("zscaler_zia.web.client.internet.ip");
                event.remove("zscaler_zia.web.client.ip");
                event.remove("zscaler_zia.web.client.source_port");
                event.remove("zscaler_zia.web.cloud_name");
                event.remove("zscaler_zia.web.company");
                event.remove("zscaler_zia.web.device.hostname");
                event.remove("zscaler_zia.web.device.model");
                event.remove("zscaler_zia.web.device.name");
                event.remove("zscaler_zia.web.device.os.version");
                event.remove("zscaler_zia.web.device.type");
                event.remove("zscaler_zia.web.dlp.rule.name");
                event.remove("zscaler_zia.web.external.device.id");
                event.remove("zscaler_zia.web.file.name");
                event.remove("zscaler_zia.web.file.subtype");
                event.remove("zscaler_zia.web.host");
                event.remove("zscaler_zia.web.login");
                event.remove("zscaler_zia.web.md5_hash");
                event.remove("zscaler_zia.web.prototype");
                event.remove("zscaler_zia.web.reason");
                event.remove("zscaler_zia.web.record.id");
                event.remove("zscaler_zia.web.referer.name");
                event.remove("zscaler_zia.web.request.method");
                event.remove("zscaler_zia.web.request.size");
                event.remove("zscaler_zia.web.request.version");
                event.remove("zscaler_zia.web.response.size");
                event.remove("zscaler_zia.web.response.version");
                event.remove("zscaler_zia.web.rule.name");
                event.remove("zscaler_zia.web.server.ip");
                event.remove("zscaler_zia.web.sha256");
                event.remove("zscaler_zia.web.time");
                event.remove("zscaler_zia.web.upload.file.name");
                event.remove("zscaler_zia.web.upload.file.subtype");
                event.remove("zscaler_zia.web.url.filter_rule_label");
                event.remove("zscaler_zia.web.url.name");
                event.remove("zscaler_zia.web.user_agent.name");
            }

            event.remove("json");
            event.remove("_conf");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
