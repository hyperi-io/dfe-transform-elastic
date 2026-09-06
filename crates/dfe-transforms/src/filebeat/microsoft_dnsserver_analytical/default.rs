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
            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("protocol"))?;

            let _cond = {
                event.has_value("winlog.event_data")
                    && event.get_str("winlog.event_data") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    event.rename("winlog.event_data", "microsoft_dnsserver.analytical")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "rename")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "rename_winlog_eventdata",
                    )?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("event.code") };
            if _cond {
                // Painless script
                // Source: def t = params.get(ctx.event.code);\nif (t == null) {\n  return;\n}\nif (ctx.microsoft_dnsserver?.analytical == null ) {\n  Map map = new HashMap();\n  ctx.microsoft_dnsserver.put(\"analytical\", map);\n}\nctx.microsoft_dnsserver.analytical.put(\"description\", t)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def t = params.get(ctx.event.code);\nif (t == null) {\n  return;\n}\nif (ctx.microsoft_dnsserver?.analytical == null ) {\n  Map map = new HashMap();\n  ctx.microsoft_dnsserver.put(\"analytical\", map);\n}\nctx.microsoft_dnsserver.analytical.put(\"description\", t)"#
                    ),
                    cached_params!(
                        "{\"256\":\"Query received\",\"257\":\"Response success\",\"258\":\"Response failure\",\"259\":\"Ignored query\",\"260\":\"Recursive query out\",\"261\":\"Response in\",\"262\":\"Recursive query timeout\",\"263\":\"Update in\",\"264\":\"Update response\",\"265\":\"IXFR request out\",\"266\":\"IXFR request in\",\"267\":\"IXFR response out\",\"268\":\"IXFR response in\",\"269\":\"AXFR request out\",\"270\":\"AXFR request in\",\"271\":\"AXFR response out\",\"272\":\"AXFR response in\",\"273\":\"XFR notification in\",\"274\":\"XFR notification out\",\"275\":\"XFR notify ACK in\",\"276\":\"XFR notify ACK out\",\"277\":\"Dynamic update forward\",\"278\":\"Dynamic update response in\",\"279\":\"Internal lookup CNAME\",\"280\":\"Internal lookup additional\"}"
                    ),
                )?;
            }

            let _cond = { event.has_value("microsoft_dnsserver.analytical.QTYPE") };
            if _cond {
                // Painless script
                // Source: def t = params.get(ctx.microsoft_dnsserver.analytical.QTYPE);\nif (t != null) {\n  ctx.microsoft_dnsserver.analytical.put(\"question_type\", t);\n}\nctx.microsoft_dnsserver.analytical.remove(\"QTYPE\");
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def t = params.get(ctx.microsoft_dnsserver.analytical.QTYPE);\nif (t != null) {\n  ctx.microsoft_dnsserver.analytical.put(\"question_type\", t);\n}\nctx.microsoft_dnsserver.analytical.remove(\"QTYPE\");"#
                    ),
                    cached_params!(
                        "{\"1\":\"A\",\"2\":\"NS\",\"3\":\"MD\",\"4\":\"MF\",\"5\":\"CNAME\",\"6\":\"SOA\",\"7\":\"MB\",\"8\":\"MG\",\"9\":\"MR\",\"11\":\"WKS\",\"12\":\"PTR\",\"13\":\"HINFO\",\"14\":\"MINFO\",\"15\":\"MX\",\"16\":\"TXT\",\"17\":\"RP\",\"18\":\"AFSDB\",\"19\":\"X25\",\"20\":\"ISDN\",\"21\":\"RT\",\"24\":\"SIG\",\"25\":\"KEY\",\"26\":\"PX\",\"27\":\"GPOS\",\"28\":\"AAAA\",\"29\":\"LOC\",\"31\":\"EID\",\"32\":\"NIMLOC\",\"33\":\"SRV\",\"34\":\"ATMA\",\"35\":\"NAPTR\",\"36\":\"KX\",\"37\":\"CERT\",\"39\":\"DNAME\",\"40\":\"SINK\",\"41\":\"OPL\",\"42\":\"APL\",\"43\":\"DS\",\"44\":\"SSHFP\",\"45\":\"IPSECKEY\",\"46\":\"RRSIG\",\"47\":\"NSEC\",\"48\":\"DNSKEY\",\"49\":\"DHCID\",\"50\":\"NSEC3\",\"51\":\"NSEC3PARAM\",\"52\":\"TLSA\",\"53\":\"SMIMEA\",\"55\":\"HIP\",\"56\":\"NINFO\",\"57\":\"RKEY\",\"58\":\"TALINK\",\"59\":\"CDS\",\"60\":\"CDNSKEY\",\"61\":\"OPENPGPKEY\",\"62\":\"CSYNC\",\"63\":\"ZONEMD\",\"64\":\"SVCB\",\"65\":\"HTTPS\",\"99\":\"SPF\",\"104\":\"NID\",\"105\":\"L32\",\"106\":\"L64\",\"107\":\"LP\",\"108\":\"EUI48\",\"109\":\"EUI64\",\"249\":\"TKEY\",\"250\":\"TSIG\",\"251\":\"IXFR\",\"252\":\"AXFR\",\"253\":\"MAILB\",\"254\":\"MAILA\",\"256\":\"URI\",\"257\":\"CAA\",\"258\":\"AVC\",\"259\":\"DOA\",\"260\":\"AMTRELAY\",\"261\":\"RESINFO\",\"32768\":\"TA\",\"32769\":\"DLV\"}"
                    ),
                )?;
            }

            let _cond = { event.has_value("microsoft_dnsserver.analytical.RCODE") };
            if _cond {
                // Painless script
                // Source: def t = params.get(ctx.microsoft_dnsserver.analytical.RCODE);\nif (t != null) {\n  ctx.microsoft_dnsserver.analytical.put(\"response_code\", t);\n}\nctx.microsoft_dnsserver.analytical.remove(\"RCODE\");
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def t = params.get(ctx.microsoft_dnsserver.analytical.RCODE);\nif (t != null) {\n  ctx.microsoft_dnsserver.analytical.put(\"response_code\", t);\n}\nctx.microsoft_dnsserver.analytical.remove(\"RCODE\");"#
                    ),
                    cached_params!(
                        "{\"0\":\"NoError\",\"1\":\"FormErr\",\"2\":\"ServFail\",\"3\":\"NXDomain\",\"4\":\"NotImp\",\"5\":\"Refused\",\"6\":\"YXDomain\",\"7\":\"YXRRSet\",\"8\":\"NXRRSet\",\"9\":\"NotAuth\",\"10\":\"NotZone\",\"11\":\"DSOTYPENI\",\"16\":\"BADVERS/BADSIG\",\"17\":\"BADKEY\",\"18\":\"BADTIME\",\"19\":\"BADMODE\",\"20\":\"BADNAME\",\"21\":\"BADALG\",\"22\":\"BADTRUNC\",\"23\":\"BADCOOKIE\"}"
                    ),
                )?;
            }

            let _cond =
                { event.get_str("microsoft_dnsserver.analytical.PolicyName") == Some("NULL") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("microsoft_dnsserver.analytical.PolicyName");
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("microsoft_dnsserver.analytical.Flags");
                Ok(())
            })();

            let _cond = { !event.has_value("winlog.task_raw") && event.has_value("winlog.task") };
            if _cond {
                // Painless script
                // Source: def t = params.get(ctx.winlog.task);\nif (t != null) {\n  ctx.winlog.task = t;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def t = params.get(ctx.winlog.task);\nif (t != null) {\n  ctx.winlog.task = t;\n}"#
                    ),
                    cached_params!(
                        "{\"0\":\"DEFAULT\",\"1\":\"LOOK_UP\",\"2\":\"RECURSE_QUERY\",\"3\":\"DYNAMIC_UPDATE\",\"4\":\"ZONE_XFR\",\"5\":\"ZONE_OP\",\"6\":\"AGEING\",\"7\":\"OnlineSigning\",\"8\":\"DNSSEC_OP\",\"9\":\"CACHE_OP\",\"10\":\"Configuration\",\"11\":\"SERVER_OP\",\"13\":\"POLICY_OP\",\"14\":\"RRL\",\"15\":\"RRL_OP\",\"16\":\"VIRTUALIZATION_OP\"}"
                    ),
                )?;
            }

            let _cond = { event.has_value("microsoft_dnsserver.analytical") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map src, Map keyMap) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        dst[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = updatedList;\n      } else {\n        dst[key] = value;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = value;\n      } else {\n        dst[key] = value;\n      }\n    }\n  }\n  return dst;\n}\nctx.microsoft_dnsserver.analytical = renameKeys(ctx.microsoft_dnsserver.analytical, params)\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def renameKeys(Map src, Map keyMap) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        dst[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = updatedList;\n      } else {\n        dst[key] = value;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = value;\n      } else {\n        dst[key] = value;\n      }\n    }\n  }\n  return dst;\n}\nctx.microsoft_dnsserver.analytical = renameKeys(ctx.microsoft_dnsserver.analytical, params)\n"#
                    ),
                    cached_params!(
                        "{\"AdditionalInfo\":\"additional_info\",\"BufferSize\":\"bytes_sent\",\"CacheScope\":\"cache_scope\",\"DNSSEC\":\"dnssec\",\"ElapsedTime\":\"elapsed_time\",\"GUID\":\"guid\",\"PacketData\":\"packet_data\",\"PolicyName\":\"policy_name\",\"QNAME\":\"question_name\",\"QueriesAttached\":\"queries_attached\",\"QXID\":\"qxid\",\"Reason\":\"reason\",\"RecursionDepth\":\"recursion_depth\",\"RecursionScope\":\"recursion_scope\",\"Scope\":\"scope\",\"Secure\":\"secure\",\"XID\":\"xid\",\"Zone\":\"zone\",\"ZoneScope\":\"zone_scope\"}"
                    ),
                )?;
            }

            if event.has_value("microsoft_dnsserver.analytical.extended_data.SID") {
                event.rename(
                    "microsoft_dnsserver.analytical.extended_data.SID",
                    "user.id",
                )?;
            }

            event.remove("microsoft_dnsserver.analytical.extended_data");

            if let Some(v) = event
                .get("microsoft_dnsserver.analytical.xid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.id", v)?;
            }

            if event.has_value("microsoft_dnsserver.analytical.question_name") {
                gsub_field(
                    event,
                    "microsoft_dnsserver.analytical.question_name",
                    "_temp.question_name",
                    cached_regex!("\\.$"),
                    "",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("_temp.question_name") {
                    if let Some(domain_str) = event.get_string("_temp.question_name") {
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
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "registered_domain")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "registered_domain_question_name",
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

            if event.has_value("dns.question.domain") {
                event.rename("dns.question.domain", "dns.question.name")?;
            }

            if let Some(v) = event
                .get("microsoft_dnsserver.analytical.question_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.type", v)?;
            }

            if let Some(v) = event
                .get("microsoft_dnsserver.analytical.response_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.response_code", v)?;
            }

            let _cond = { event.get_str("dns.response_code") == Some("NoError") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("dns.response_code")
                    && event.get_str("dns.response_code") != Some("NoError")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("microsoft_dnsserver.analytical.AA") == Some("1") };
            if _cond {
                event.append_unique("dns.header_flags", json!("AA"))?;
            }

            let _cond = { event.get_str("microsoft_dnsserver.analytical.AD") == Some("1") };
            if _cond {
                event.append_unique("dns.header_flags", json!("AD"))?;
            }

            let _cond = { event.get_str("microsoft_dnsserver.analytical.RD") == Some("1") };
            if _cond {
                event.append_unique("dns.header_flags", json!("RD"))?;
            }

            let _cond = { event.get_str("microsoft_dnsserver.analytical.TC") == Some("1") };
            if _cond {
                event.append_unique("dns.header_flags", json!("TC"))?;
            }

            event.remove("_temp");
            event.remove("microsoft_dnsserver.analytical.AA");
            event.remove("microsoft_dnsserver.analytical.AD");
            event.remove("microsoft_dnsserver.analytical.RD");
            event.remove("microsoft_dnsserver.analytical.TC");

            event.set("network.protocol", json!("dns"))?;

            let _cond = { event.get_str("microsoft_dnsserver.analytical.TCP") == Some("1") };
            if _cond {
                event.set("network.transport", json!("tcp"))?;
            }

            let _cond = { event.get_str("microsoft_dnsserver.analytical.TCP") == Some("0") };
            if _cond {
                event.set("network.transport", json!("udp"))?;
            }

            event.remove("microsoft_dnsserver.analytical.TCP");

            if let Some(v) = event
                .get("microsoft_dnsserver.analytical.reason")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft_dnsserver.analytical.bytes_sent") {
                    if let Some(val) = event.get("microsoft_dnsserver.analytical.bytes_sent") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft_dnsserver.analytical.bytes_sent".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_dnsserver.analytical.bytes_sent", converted)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("microsoft_dnsserver.analytical.bytes_sent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.bytes", v)?;
            }

            if let Some(v) = event
                .get("winlog.task")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("winlog.process_id") {
                    if let Some(val) = event.get("winlog.process_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.process_id".into(),
                                message,
                            }
                        })?;
                        event.set("process.pid", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("winlog.thread_id") {
                    if let Some(val) = event.get("winlog.thread_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "winlog.thread_id".into(),
                                message,
                            }
                        })?;
                        event.set("process.thread.id", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("winlog.process_id");
                event.remove("winlog.thread_id");
                Ok(())
            })();

            let _cond = {
                event.get_str("winlog.activity_guid")
                    == Some("{00000000-0000-0000-0000-000000000000}")
            };
            if _cond {
                if event.remove("winlog.activity_guid").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "winlog.activity_guid".into(),
                    });
                }
            }

            if event.has_value("winlog.activity_guid") {
                event.rename("winlog.activity_guid", "winlog.activity_id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("winlog.keywords");
                Ok(())
            })();

            let _cond = { event.has_value("event.code") };
            if _cond {
                // Painless script
                // Source: def t = params.get(ctx.event.code);\nif (t == null) {\n  return;\n}\ndef list = new ArrayList();\nlist.add(t);\nctx.winlog.keywords = list;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def t = params.get(ctx.event.code);\nif (t == null) {\n  return;\n}\ndef list = new ArrayList();\nlist.add(t);\nctx.winlog.keywords = list;"#
                    ),
                    cached_params!(
                        "{\"256\":\"QUERY_RECEIVED\",\"257\":\"RESPONSE_SUCCESS\",\"258\":\"RESPONSE_FAILURE\",\"259\":\"IGNORED_QUERY\",\"260\":\"RECURSE_QUERY_OUT\",\"261\":\"RECURSE_RESPONSE_IN\",\"262\":\"RECURSE_QUERY_TIMEOUT\",\"263\":\"DYN_UPDATE_RECV\",\"264\":\"DYN_UPDATE_RESPONSE\",\"265\":\"IXFR_REQ_OUT\",\"266\":\"IXFR_REQ_RECV\",\"267\":\"IXFR_RESP_OUT\",\"268\":\"IXFR_RESP_RECV\",\"269\":\"AXFR_REQ_OUT\",\"270\":\"AXFR_REQ_RECV\",\"271\":\"AXFR_RESP_OUT\",\"272\":\"AXFR_RESP_RECV\",\"273\":\"XFR_NOTIFY_RECV\",\"274\":\"XFR_NOTIFY_OUT\",\"275\":\"XFR_NOTIFY_ACK_IN\",\"276\":\"XFR_NOTIFY_ACK_OUT\",\"277\":\"DYN_UPDATE_FORWARD\",\"278\":\"DYN_UPDATE_RESPONSE_IN\",\"279\":\"INTERNAL_LOOKUP_CNAME\",\"280\":\"INTERNAL_LOOKUP_ADDITIONAL\"}"
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft_dnsserver.analytical.Destination") {
                    if let Some(val) = event.get("microsoft_dnsserver.analytical.Destination") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft_dnsserver.analytical.Destination".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_dnsserver.analytical.destination.ip", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("microsoft_dnsserver.analytical.Destination");
                Ok(())
            })();

            if let Some(v) = event
                .get("microsoft_dnsserver.analytical.destination.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            let _cond = {
                event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
            };
            if _cond {
                if event.has_value("microsoft_dnsserver.analytical.Port") {
                    event.rename(
                        "microsoft_dnsserver.analytical.Port",
                        "microsoft_dnsserver.analytical.destination.port",
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft_dnsserver.analytical.destination.port") {
                    if let Some(val) = event.get("microsoft_dnsserver.analytical.destination.port")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft_dnsserver.analytical.destination.port".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_dnsserver.analytical.destination.port", converted)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("microsoft_dnsserver.analytical.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
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
                event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
            };
            if _cond {
                event.set("network.direction", json!("egress"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft_dnsserver.analytical.Source") {
                    if let Some(val) = event.get("microsoft_dnsserver.analytical.Source") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft_dnsserver.analytical.Source".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_dnsserver.analytical.source.ip", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("microsoft_dnsserver.analytical.Source");
                Ok(())
            })();

            if let Some(v) = event
                .get("microsoft_dnsserver.analytical.source.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                if event.has_value("microsoft_dnsserver.analytical.Port") {
                    event.rename(
                        "microsoft_dnsserver.analytical.Port",
                        "microsoft_dnsserver.analytical.source.port",
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft_dnsserver.analytical.source.port") {
                    if let Some(val) = event.get("microsoft_dnsserver.analytical.source.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft_dnsserver.analytical.source.port".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_dnsserver.analytical.source.port", converted)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("microsoft_dnsserver.analytical.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
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

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                event.set("network.direction", json!("ingress"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft_dnsserver.analytical.InterfaceIP") {
                    if let Some(val) = event.get("microsoft_dnsserver.analytical.InterfaceIP") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft_dnsserver.analytical.InterfaceIP".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_dnsserver.analytical.interface_ip", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft_dnsserver.analytical.ForwardInterfaceIP") {
                    if let Some(val) =
                        event.get("microsoft_dnsserver.analytical.ForwardInterfaceIP")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft_dnsserver.analytical.ForwardInterfaceIP".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft_dnsserver.analytical.forward_interface_ip",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("microsoft_dnsserver.analytical.InterfaceIP");
                event.remove("microsoft_dnsserver.analytical.ForwardInterfaceIP");
                Ok(())
            })();

            let _cond = {
                event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
            };
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

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
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

            let _cond = {
                event.has_value("microsoft_dnsserver.analytical.interface_ip")
                    && event.get_str("microsoft_dnsserver.analytical.interface_ip") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("microsoft_dnsserver.analytical.interface_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("microsoft_dnsserver.analytical.forward_interface_ip")
                    && event.get_str("microsoft_dnsserver.analytical.forward_interface_ip")
                        != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("microsoft_dnsserver.analytical.forward_interface_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("microsoft_dnsserver.analytical.question_name");
                    event.remove("microsoft_dnsserver.analytical.question_type");
                    event.remove("microsoft_dnsserver.analytical.response_code");
                    event.remove("microsoft_dnsserver.analytical.bytes_sent");
                    event.remove("microsoft_dnsserver.analytical.xid");
                    event.remove("microsoft_dnsserver.analytical.task");
                    event.remove("microsoft_dnsserver.analytical.reason");
                    event.remove("microsoft_dnsserver.analytical.destination.ip");
                    event.remove("microsoft_dnsserver.analytical.destination.port");
                    event.remove("microsoft_dnsserver.analytical.source.ip");
                    event.remove("microsoft_dnsserver.analytical.source.port");
                    event.remove("winlog.task");
                    event.remove("winlog.task_raw");
                    event.remove("winlog.level");
                    event.remove("winlog.level_raw");
                    event.remove("winlog.provider_message");
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
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
