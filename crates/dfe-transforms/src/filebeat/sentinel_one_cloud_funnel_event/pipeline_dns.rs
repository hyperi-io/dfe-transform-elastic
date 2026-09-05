// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_dns` pipeline.
pub struct PipelineDns;

impl Transform for PipelineDns {
    fn name(&self) -> &str {
        "pipeline_dns"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set(
                "event.type",
                Value::Array(vec![json!("info"), json!("protocol")]),
            )?;

            event.set(
                "event.action",
                Value::Array(vec![json!("lookup_requested")]),
            )?;

            if event.has_value("json.event.dns.request") {
                event.rename(
                    "json.event.dns.request",
                    "sentinel_one_cloud_funnel.event.dns.request",
                )?;
            }

            if event.has_value("json.event.dns.response") {
                event.rename(
                    "json.event.dns.response",
                    "sentinel_one_cloud_funnel.event.dns.response",
                )?;
            }

            let _cond = {
                (event.has_value("sentinel_one_cloud_funnel.event.dns.response")
                    && event.get_str("sentinel_one_cloud_funnel.event.dns.response") != Some(""))
                    || (event.has_value("sentinel_one_cloud_funnel.event.dns.request")
                        && event.get_str("sentinel_one_cloud_funnel.event.dns.request") != Some(""))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def ips = new ArrayList();\ndef relatedHosts = new ArrayList();\ndef dns = new HashMap();\n\nif (ctx.sentinel_one_cloud_funnel?.event?.dns?.request != null && ctx.sentinel_one_cloud_funnel.event.dns.request != \"\") {\n  def request = ctx.sentinel_one_cloud_funnel.event.dns.request;\n  def question = new HashMap();\n  def parts = /\\s+/.split(request);\n\n  if (parts.length == 3) {\n    question.put(\"type\", params[parts[1]]);\n    question.put(\"name\", parts[2]);   \n    relatedHosts.add(parts[2]);\n  } else {\n    question.put(\"name\", request); \n  }\n  dns.question = question;\n}\n\nif (ctx.sentinel_one_cloud_funnel?.event?.dns?.response != null && ctx.sentinel_one_cloud_funnel.event.dns.response != \"\") {\n  def response = /;/.split(ctx.sentinel_one_cloud_funnel.event.dns.response);\n  def answers = new ArrayList();\n\n  for (def i = 0; i < response.length; i++) {\n    def answer = response[i];\n    if (answer == \"\") {\n      continue;\n    }\n\n    if (answer.startsWith(\"type:\")) {\n      def parts = /\\s+/.split(answer);\n      if (parts.length < 2) {\n        throw new Exception(\"unexpected event.dns.response format\");\n      }\n      if (parts.length == 3) {\n        answers.add([\n          \"type\": params[parts[1]],\n          \"data\": parts[2]\n        ]);\n        relatedHosts.add(parts[2]);\n      } else {\n        answers.add([\n          \"type\": params[parts[1]]\n        ]);\n      }\n    } else {\n      ips.add(answer);\n    }\n  }\n\n  if (answers.length > 0) {\n    dns.answers = answers;\n  }\n  if (ips.length > 0) {\n    dns.resolved_ip = ips;\n  }\n  if (relatedHosts.length > 0) {\n    if (ctx?.related == null) {\n      ctx.related = new HashMap();\n    }\n    ctx.related.hosts = relatedHosts;\n  }\n}\nif (dns != null) {\n  ctx.dns = dns;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def ips = new ArrayList();\ndef relatedHosts = new ArrayList();\ndef dns = new HashMap();\n\nif (ctx.sentinel_one_cloud_funnel?.event?.dns?.request != null && ctx.sentinel_one_cloud_funnel.event.dns.request != \"\") {\n  def request = ctx.sentinel_one_cloud_funnel.event.dns.request;\n  def question = new HashMap();\n  def parts = /\\s+/.split(request);\n\n  if (parts.length == 3) {\n    question.put(\"type\", params[parts[1]]);\n    question.put(\"name\", parts[2]);   \n    relatedHosts.add(parts[2]);\n  } else {\n    question.put(\"name\", request); \n  }\n  dns.question = question;\n}\n\nif (ctx.sentinel_one_cloud_funnel?.event?.dns?.response != null && ctx.sentinel_one_cloud_funnel.event.dns.response != \"\") {\n  def response = /;/.split(ctx.sentinel_one_cloud_funnel.event.dns.response);\n  def answers = new ArrayList();\n\n  for (def i = 0; i < response.length; i++) {\n    def answer = response[i];\n    if (answer == \"\") {\n      continue;\n    }\n\n    if (answer.startsWith(\"type:\")) {\n      def parts = /\\s+/.split(answer);\n      if (parts.length < 2) {\n        throw new Exception(\"unexpected event.dns.response format\");\n      }\n      if (parts.length == 3) {\n        answers.add([\n          \"type\": params[parts[1]],\n          \"data\": parts[2]\n        ]);\n        relatedHosts.add(parts[2]);\n      } else {\n        answers.add([\n          \"type\": params[parts[1]]\n        ]);\n      }\n    } else {\n      ips.add(answer);\n    }\n  }\n\n  if (answers.length > 0) {\n    dns.answers = answers;\n  }\n  if (ips.length > 0) {\n    dns.resolved_ip = ips;\n  }\n  if (relatedHosts.length > 0) {\n    if (ctx?.related == null) {\n      ctx.related = new HashMap();\n    }\n    ctx.related.hosts = relatedHosts;\n  }\n}\nif (dns != null) {\n  ctx.dns = dns;\n}"#
                        ),
                        cached_params!(
                            "{\"1\":\"A\",\"2\":\"NS\",\"3\":\"MD\",\"4\":\"MF\",\"5\":\"CNAME\",\"6\":\"SOA\",\"7\":\"MB\",\"8\":\"MG\",\"9\":\"MR\",\"10\":\"NULL\",\"11\":\"WKS\",\"12\":\"PTR\",\"13\":\"HINFO\",\"14\":\"MINFO\",\"15\":\"MX\",\"16\":\"TXT\",\"17\":\"RP\",\"18\":\"AFSDB\",\"19\":\"X25\",\"20\":\"ISDN\",\"21\":\"RT\",\"22\":\"NSAP\",\"23\":\"NSAPPTR\",\"24\":\"SIG\",\"25\":\"KEY\",\"26\":\"PX\",\"27\":\"GPOS\",\"28\":\"AAAA\",\"29\":\"LOC\",\"30\":\"NXT\",\"31\":\"EID\",\"32\":\"NIMLOC\",\"33\":\"SRV\",\"34\":\"ATMA\",\"35\":\"NAPTR\",\"36\":\"KX\",\"37\":\"CERT\",\"38\":\"A6\",\"39\":\"DNAME\",\"40\":\"SINK\",\"41\":\"OPT\",\"43\":\"DS\",\"46\":\"RRSIG\",\"47\":\"NSEC\",\"48\":\"DNSKEY\",\"49\":\"DHCID\",\"100\":\"UINFO\",\"101\":\"UID\",\"102\":\"GID\",\"103\":\"UNSPEC\",\"248\":\"ADDRS\",\"249\":\"TKEY\",\"250\":\"TSIG\",\"251\":\"IXFR\",\"252\":\"AXFR\",\"253\":\"MAILB\",\"254\":\"MAILA\",\"255\":\"ANY\",\"65281\":\"WINS\",\"65282\":\"WINSR\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_dnsFieldParsing_to_ecs",
                    )?;
                    event.append_unique(
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

            let _cond = { event.get("dns.answers").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "dns.answers", |event| {
                        gsub_field(
                            event,
                            "_ingest._value",
                            "_ingest._value",
                            cached_regex!(
                                "^\\[?::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)(?:\\](?::[0-9]+)?)?$"
                            ),
                            "$1",
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("dns.resolved_ip").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "dns.resolved_ip", |event| {
                        gsub_field(
                            event,
                            "_ingest._value",
                            "_ingest._value",
                            cached_regex!(
                                "^\\[?::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)(?:\\](?::[0-9]+)?)?$"
                            ),
                            "$1",
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("dns.resolved_ip").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "dns.resolved_ip", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_dns_resolved_ip_to_ip",
                            )?;
                            event.remove("_ingest._value");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
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
                        "Processor '{}' {}failed with message '{}'",
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
