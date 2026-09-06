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
            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "json")?;

            event.set("cloud.provider", json!("aws"))?;

            if event.has_value("json.account_id") {
                event.rename("json.account_id", "cloud.account.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.query_timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.query_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("json.region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            if event.has_value("json.vpc_id") {
                event.rename("json.vpc_id", "aws.vpc_id")?;
            }

            if event.has_value("json.srcids.instance") {
                event.rename("json.srcids.instance", "aws.instance_id")?;
            }

            if let Some(v) = event
                .get("aws.instance_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.id", v)?;
            }

            if event.has_value("json.query_name") {
                gsub_field(
                    event,
                    "json.query_name",
                    "json.query_name",
                    cached_regex!("\\.$"),
                    "",
                )?;
            }

            let _cond = {
                !(event
                    .get_str("json.query_name")
                    .is_some_and(|s| s.ends_with("in-addr.arpa")))
                    && !(event
                        .get_str("json.query_name")
                        .is_some_and(|s| s.ends_with("ip6.arpa")))
            };
            if _cond {
                if event.has_value("json.query_name") {
                    if let Some(domain_str) = event.get_string("json.query_name") {
                        let domain = domain_str.to_string();
                        event.set("dns.question.domain", json!(domain.clone()))?;
                        // Public suffix list lookup for registered domain extraction
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            if let Some(registered) = rd.registered_domain {
                                event.set("dns.question.registered_domain", json!(registered))?;
                            }
                            event
                                .set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                            if let Some(sub) = rd.subdomain {
                                event.set("dns.question.subdomain", json!(sub))?;
                            }
                        }
                    }
                }
            }

            if event.has_value("dns.question.domain") {
                event.rename("dns.question.domain", "dns.question.name")?;
            }

            let _cond = { !event.has_value("dns.question.name") };
            if _cond {
                if event.has_value("json.query_name") {
                    event.rename("json.query_name", "dns.question.name")?;
                }
            }

            if event.has_value("json.query_class") {
                event.rename("json.query_class", "dns.question.class")?;
            }

            if event.has_value("json.query_type") {
                event.rename("json.query_type", "dns.question.type")?;
            }

            if event.has_value("json.rcode") {
                event.rename("json.rcode", "dns.response_code")?;
            }

            if event.has_value("json.answers") {
                event.rename("json.answers", "dns.answers")?;
            }

            let _cond = {
                event.has_value("dns.answers")
                    && event.get("dns.answers").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: List answers = new ArrayList(); for (answer in ctx.dns.answers) {\n  Map new_answer = new HashMap();\n  if(answer?.Class != null) {\n    new_answer.put(\"class\", answer?.Class);\n  }\n  if(answer?.Type != null) {\n    new_answer.put(\"type\", answer?.Type);\n  }\n  if(answer?.Rdata != null) {\n    new_answer.put(\"data\", answer?.Rdata);\n    if (new_answer?.data != null && new_answer.data.length() > 0 && new_answer.data.substring(new_answer.data.length() - 1) == '.') {\n        new_answer.data = new_answer.data.substring(0, new_answer.data.length() - 1);\n    }\n    if (new_answer?.type != null && new_answer.type == 'CNAME') {\n      new_answer.put(\"name\", new_answer?.data);\n    }\n  }\n  answers.add(new_answer);\n  if(ctx.related == null) {\n    ctx.put('related', new HashMap());\n  }\n  if(ctx.related?.ip == null) {\n    ctx.related.put('ip',new ArrayList());\n  }\n  if(ctx.related?.hosts == null) {\n    ctx.related.put('hosts',new ArrayList());\n  }\n  if(['A','AAAA'].contains(new_answer.type)) {\n    ctx.related.ip.add(new_answer.data);\n  }\n  if(['CNAME', 'PTR'].contains(new_answer.type)) {\n    ctx.related.hosts.add(new_answer.data);\n  }\n} ctx.dns.answers = answers;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"List answers = new ArrayList(); for (answer in ctx.dns.answers) {\n  Map new_answer = new HashMap();\n  if(answer?.Class != null) {\n    new_answer.put(\"class\", answer?.Class);\n  }\n  if(answer?.Type != null) {\n    new_answer.put(\"type\", answer?.Type);\n  }\n  if(answer?.Rdata != null) {\n    new_answer.put(\"data\", answer?.Rdata);\n    if (new_answer?.data != null && new_answer.data.length() > 0 && new_answer.data.substring(new_answer.data.length() - 1) == '.') {\n        new_answer.data = new_answer.data.substring(0, new_answer.data.length() - 1);\n    }\n    if (new_answer?.type != null && new_answer.type == 'CNAME') {\n      new_answer.put(\"name\", new_answer?.data);\n    }\n  }\n  answers.add(new_answer);\n  if(ctx.related == null) {\n    ctx.put('related', new HashMap());\n  }\n  if(ctx.related?.ip == null) {\n    ctx.related.put('ip',new ArrayList());\n  }\n  if(ctx.related?.hosts == null) {\n    ctx.related.put('hosts',new ArrayList());\n  }\n  if(['A','AAAA'].contains(new_answer.type)) {\n    ctx.related.ip.add(new_answer.data);\n  }\n  if(['CNAME', 'PTR'].contains(new_answer.type)) {\n    ctx.related.hosts.add(new_answer.data);\n  }\n} ctx.dns.answers = answers;"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.transport") {
                event.rename("json.transport", "network.transport")?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.get_str("network.transport") == Some("tcp") };
            if _cond {
                event.set("network.iana_number", json!("6"))?;
            }

            let _cond = { event.get_str("network.transport") == Some("udp") };
            if _cond {
                event.set("network.iana_number", json!("17"))?;
            }

            event.set("network.protocol", json!("dns"))?;

            if event.has_value("json.srcport") {
                if let Some(val) = event.get("json.srcport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.srcport".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
            }

            if event.has_value("json.srcaddr") {
                event.rename("json.srcaddr", "source.address")?;
            }

            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = {
                event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = {
                event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            if event.has_value("json.firewall_rule_action") {
                event.rename("json.firewall_rule_action", "aws.route53.firewall.action")?;
            }

            if event.has_value("json.firewall_rule_group_id") {
                event.rename(
                    "json.firewall_rule_group_id",
                    "aws.route53.firewall.rule_group.id",
                )?;
            }

            if event.has_value("json.firewall_domain_list_id") {
                event.rename(
                    "json.firewall_domain_list_id",
                    "aws.route53.firewall.domain_list.id",
                )?;
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("protocol"))?;

            let _cond = { event.get_str("dns.response_code") == Some("NOERROR") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("dns.response_code") != Some("NOERROR") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("dns.question.name")
                    && event.get_str("dns.question.type") == Some("PTR")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: String ip; if(ctx.dns?.question?.name.contains(\".in-addr.arpa\")) {\n  List reverse_ip = Arrays.asList(ctx.dns?.question?.name.replace(\".in-addr.arpa\", \"\").splitOnToken(\".\"));\n  List ip_arr = new ArrayList();\n  for (int i = reverse_ip.length; i > 0 ; i--) {\n      ip_arr.add(reverse_ip[i-1]);\n  }\n  ip = String.join(\".\",ip_arr);\n} else if (ctx.dns?.question?.name.contains(\".ip6.arpa\")) {\n  List reverse_ip = Arrays.asList(ctx.dns?.question?.name.replace(\".ip6.arpa\", \"\").splitOnToken(\".\"));\n  List ip_arr = new ArrayList();\n  int j = 1;\n  for (int i = reverse_ip.length; i > 0 ; i--) {\n      ip_arr.add(reverse_ip[i-1]);\n      if(j % 4 == 0 && i != 1) {\n        j = 0;\n        ip_arr.add(\":\");\n      }\n      j++;\n  }\n  ip = String.join(\"\",ip_arr);    \n}     if(ctx.related?.ip == null) {\n  ctx.related.put('ip',new ArrayList());\n} if(ip != null && !ctx.related?.ip.contains(ip)) {\n  ctx.related.ip.add(ip);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String ip; if(ctx.dns?.question?.name.contains(\".in-addr.arpa\")) {\n  List reverse_ip = Arrays.asList(ctx.dns?.question?.name.replace(\".in-addr.arpa\", \"\").splitOnToken(\".\"));\n  List ip_arr = new ArrayList();\n  for (int i = reverse_ip.length; i > 0 ; i--) {\n      ip_arr.add(reverse_ip[i-1]);\n  }\n  ip = String.join(\".\",ip_arr);\n} else if (ctx.dns?.question?.name.contains(\".ip6.arpa\")) {\n  List reverse_ip = Arrays.asList(ctx.dns?.question?.name.replace(\".ip6.arpa\", \"\").splitOnToken(\".\"));\n  List ip_arr = new ArrayList();\n  int j = 1;\n  for (int i = reverse_ip.length; i > 0 ; i--) {\n      ip_arr.add(reverse_ip[i-1]);\n      if(j % 4 == 0 && i != 1) {\n        j = 0;\n        ip_arr.add(\":\");\n      }\n      j++;\n  }\n  ip = String.join(\"\",ip_arr);    \n}     if(ctx.related?.ip == null) {\n  ctx.related.put('ip',new ArrayList());\n} if(ip != null && !ctx.related?.ip.contains(ip)) {\n  ctx.related.ip.add(ip);\n}   "#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("dns.question.name")
                    && event.get_str("dns.question.type") != Some("PTR")
            };
            if _cond {
                event.append(
                    "related.hosts",
                    json!(
                        event
                            .get("dns.question.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
