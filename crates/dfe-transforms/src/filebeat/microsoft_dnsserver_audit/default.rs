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

            event.set("event.kind", json!("event"))?;

            let _cond = { event.get_str("winlog.task") == Some("Configuration") };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("configuration")]))?;
            }

            let _cond = {
                event.has_value("winlog.event_data")
                    && event.get_str("winlog.event_data") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    event.rename("winlog.event_data", "microsoft_dnsserver.audit")?;
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
                                .map_or_else(String::new, painless_to_string)
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

            let _cond = { event.has_value("microsoft_dnsserver.audit.QTYPE") };
            if _cond {
                // Painless script
                // Source: def t = params.get(ctx.microsoft_dnsserver.audit.QTYPE);\nif (t != null) {\n  ctx.microsoft_dnsserver.audit.put(\"question_type\", t);\n}\nctx.microsoft_dnsserver.audit.remove(\"QTYPE\");
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def t = params.get(ctx.microsoft_dnsserver.audit.QTYPE);\nif (t != null) {\n  ctx.microsoft_dnsserver.audit.put(\"question_type\", t);\n}\nctx.microsoft_dnsserver.audit.remove(\"QTYPE\");"#
                    ),
                    cached_params!(
                        "{\"1\":\"A\",\"104\":\"NID\",\"105\":\"L32\",\"106\":\"L64\",\"107\":\"LP\",\"108\":\"EUI48\",\"109\":\"EUI64\",\"11\":\"WKS\",\"12\":\"PTR\",\"13\":\"HINFO\",\"14\":\"MINFO\",\"15\":\"MX\",\"16\":\"TXT\",\"17\":\"RP\",\"18\":\"AFSDB\",\"19\":\"X25\",\"2\":\"NS\",\"20\":\"ISDN\",\"21\":\"RT\",\"24\":\"SIG\",\"249\":\"TKEY\",\"25\":\"KEY\",\"250\":\"TSIG\",\"251\":\"IXFR\",\"252\":\"AXFR\",\"253\":\"MAILB\",\"254\":\"MAILA\",\"256\":\"URI\",\"257\":\"CAA\",\"258\":\"AVC\",\"259\":\"DOA\",\"26\":\"PX\",\"260\":\"AMTRELAY\",\"261\":\"RESINFO\",\"27\":\"GPOS\",\"28\":\"AAAA\",\"29\":\"LOC\",\"3\":\"MD\",\"31\":\"EID\",\"32\":\"NIMLOC\",\"32768\":\"TA\",\"32769\":\"DLV\",\"33\":\"SRV\",\"34\":\"ATMA\",\"35\":\"NAPTR\",\"36\":\"KX\",\"37\":\"CERT\",\"39\":\"DNAME\",\"4\":\"MF\",\"40\":\"SINK\",\"41\":\"OPL\",\"42\":\"APL\",\"43\":\"DS\",\"44\":\"SSHFP\",\"45\":\"IPSECKEY\",\"46\":\"RRSIG\",\"47\":\"NSEC\",\"48\":\"DNSKEY\",\"49\":\"DHCID\",\"5\":\"CNAME\",\"50\":\"NSEC3\",\"51\":\"NSEC3PARAM\",\"52\":\"TLSA\",\"53\":\"SMIMEA\",\"55\":\"HIP\",\"56\":\"NINFO\",\"57\":\"RKEY\",\"58\":\"TALINK\",\"59\":\"CDS\",\"6\":\"SOA\",\"60\":\"CDNSKEY\",\"61\":\"OPENPGPKEY\",\"62\":\"CSYNC\",\"63\":\"ZONEMD\",\"64\":\"SVCB\",\"65\":\"HTTPS\",\"7\":\"MB\",\"8\":\"MG\",\"9\":\"MR\",\"99\":\"SPF\"}"
                    ),
                )?;
            }

            let _cond = { event.has_value("microsoft_dnsserver.audit") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map src, Map keyMap) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        dst[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = updatedList;\n      } else {\n        dst[key] = value;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = value;\n      } else {\n        dst[key] = value;\n      }\n    }\n  }\n  return dst;\n}\nctx.microsoft_dnsserver.audit = renameKeys(ctx.microsoft_dnsserver.audit, params)\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def renameKeys(Map src, Map keyMap) {\n  def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        dst[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = updatedList;\n      } else {\n        dst[key] = value;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = value;\n      } else {\n        dst[key] = value;\n      }\n    }\n  }\n  return dst;\n}\nctx.microsoft_dnsserver.audit = renameKeys(ctx.microsoft_dnsserver.audit, params)\n"#
                    ),
                    cached_params!(
                        "{\"Action\":\"action\",\"ActiveKey\":\"active_key\",\"Base64Data\":\"base64_data\",\"BufferSize\":\"bytes_sent\",\"ChildZone\":\"child_zone\",\"ClientSubnetList\":\"client_subnet_list\",\"ClientSubnetRecord\":\"client_subnet_record\",\"Condition\":\"condition\",\"Criteria\":\"criteria\",\"CryptoAlgorithm\":\"crypto_algorithm\",\"CurrentRolloverStatus\":\"current_rollover_status\",\"CurrentState\":\"current_state\",\"DSRecordGenerationAlgorithm\":\"ds_record_generation_algorithm\",\"DSRecordSetTtl\":\"ds_record_set_ttl\",\"DSSignatureValidityPeriod\":\"ds_signature_validity_period\",\"DenialOfExistence\":\"denial_of_existence\",\"Digest\":\"digest\",\"DigestType\":\"digest_type\",\"DistributeTrustAnchor\":\"distribute_trust_anchor\",\"DnsKeyRecordSetTtl\":\"key_record_set_ttl\",\"DnsKeySignatureValidityPeriod\":\"key_signature_validity_period\",\"EnableRfc5011KeyRollover\":\"enable_rfc_5011_key_rollover\",\"ErrorsPerSecond\":\"errors_per_second\",\"EventString\":\"event_string\",\"FilePath\":\"file_path\",\"Forwarders\":\"forwarders\",\"FriendlyName\":\"friendly_name\",\"GUID\":\"guid\",\"IPv4PrefixLength\":\"ipv4_prefix_length\",\"IPv6PrefixLength\":\"ipv6_prefix_length\",\"InitialRolloverOffset\":\"initial_rollover_offset\",\"IsEnabled\":\"is_enabled\",\"IsKeyMasterServer\":\"is_key_master_server\",\"KeyId\":\"key_id\",\"KeyLength\":\"key_length\",\"KeyMasterServer\":\"key_master_server\",\"KeyOrZone\":\"key_or_zone\",\"KeyProtocol\":\"key_protocol\",\"KeyStorageProvider\":\"key_storage_provider\",\"KeyTag\":\"key_tag\",\"KeyType\":\"key_type\",\"KskOrZsk\":\"ksk_or_zsk\",\"LastRolloverTime\":\"last_rollover_time\",\"LeakRate\":\"leak_rate\",\"ListenAddresses\":\"listen_addresses\",\"MasterServer\":\"master_server\",\"Mode\":\"mode\",\"NAME\":\"name\",\"NSec3HashAlgorithm\":\"nsec3_hash_algorithm\",\"NSec3Iterations\":\"nsec3_iterations\",\"NSec3OptOut\":\"nsec3_opt_out\",\"NSec3RandomSaltLength\":\"nsec3_random_salt_length\",\"NSec3UserSalt\":\"nsec3_user_salt\",\"Name\":\"name\",\"NameServer\":\"name_server\",\"NewFriendlyName\":\"new_friendly_name\",\"NewPropertyValues\":\"new_property_values\",\"NewScope\":\"new_scope\",\"NewValue\":\"new_value\",\"NextKey\":\"next_key\",\"NextRolloverAction\":\"next_rollover_action\",\"NextRolloverTime\":\"next_rollover_time\",\"NodeName\":\"node_name\",\"OldFriendlyName\":\"old_friendly_name\",\"OldPropertyValues\":\"old_property_values\",\"OldScope\":\"old_scope\",\"ParentHasSecureDelegation\":\"parent_has_secure_delegation\",\"Policy\":\"policy\",\"ProcessingOrder\":\"processing_order\",\"PropagationTime\":\"propagation_time\",\"PropertyKey\":\"property_key\",\"QNAME\":\"question_name\",\"RDATA\":\"resolved_data\",\"RRLExceptionlist\":\"rrl_exception_list\",\"RecursionScope\":\"recursion_scope\",\"ResponsePerSecond\":\"response_per_second\",\"RolloverPeriod\":\"rollover_period\",\"RolloverType\":\"rollover_type\",\"ScavengeServers\":\"scavenge_servers\",\"Scope\":\"scope\",\"ScopeWeight\":\"scope_weight\",\"ScopeWeightNew\":\"scope_weight_new\",\"ScopeWeightOld\":\"scope_weight_old\",\"Scopes\":\"scopes\",\"SecureDelegationPollingPeriod\":\"secure_delegation_polling_period\",\"SeizedOrTransfered\":\"seized_or_transfered\",\"ServerName\":\"name_server\",\"Setting\":\"setting\",\"SignatureInceptionOffset\":\"signature_inception_offset\",\"StandbyKey\":\"standby_key\",\"StoreKeysInAD\":\"store_keys_in_AD\",\"SubTreeAging\":\"subtree_aging\",\"TCRate\":\"tc_rate\",\"TTL\":\"ttl\",\"TotalResponsesInWindow\":\"total_responses_in_window\",\"Type\":\"type\",\"VirtualizationID\":\"virtualization_id\",\"WindowSize\":\"window_size\",\"WithNewKeys\":\"with_new_keys\",\"WithWithout\":\"with_without\",\"Zone\":\"zone\",\"ZoneName\":\"zone\",\"ZoneScope\":\"zone_scope\",\"ZoneSignatureValidityPeriod\":\"zone_signature_validity_period\"}"
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft_dnsserver.audit.ttl") {
                    if let Some(val) = event.get("microsoft_dnsserver.audit.ttl") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft_dnsserver.audit.ttl".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_dnsserver.audit.ttl", converted)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("microsoft_dnsserver.audit.ttl")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.answers.ttl", v)?;
            }

            if event.has_value("microsoft_dnsserver.audit.question_name") {
                if let Some(s) = event.get_string("microsoft_dnsserver.audit.question_name") {
                    let re = cached_regex!("\\.$");
                    let replaced = re.replace_all(&s, "").into_owned();
                    event.set("_temp.question_name", replaced)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("_temp.question_name") {
                    if let Some(domain_str) = event.get_string("_temp.question_name") {
                        let domain = domain_str.to_string();
                        event.set("dns.question.domain", json!(domain.clone()))?;
                        // Public suffix list lookup for registered domain extraction
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            event.set(
                                "dns.question.registered_domain",
                                json!(rd.registered_domain),
                            )?;
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
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("dns.question.domain") {
                event.rename("dns.question.domain", "dns.question.name")?;
            }

            if let Some(v) = event
                .get("microsoft_dnsserver.audit.question_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.type", v)?;
            }

            if let Some(v) = event
                .get("microsoft_dnsserver.audit.file_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft_dnsserver.audit.bytes_sent") {
                    if let Some(val) = event.get("microsoft_dnsserver.audit.bytes_sent") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft_dnsserver.audit.bytes_sent".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_dnsserver.audit.bytes_sent", converted)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("microsoft_dnsserver.audit.bytes_sent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.bytes", v)?;
            }

            event.remove("_temp");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("microsoft_dnsserver.audit.Source") {
                    if let Some(val) = event.get("microsoft_dnsserver.audit.Source") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "microsoft_dnsserver.audit.Source".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_dnsserver.audit.source_ip", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("microsoft_dnsserver.audit.Source");
                Ok(())
            })();

            if let Some(v) = event
                .get("microsoft_dnsserver.audit.source_ip")
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

            if event.has("winlog.process") {
                event.rename("winlog.process", "process")?;
            }

            if event.has_value("winlog.record_id") {
                if let Some(val) = event.get("winlog.record_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "winlog.record_id".into(),
                            message,
                        }
                    })?;
                    event.set("winlog.record_id", converted)?;
                }
            }

            let _cond = {
                event.has_value("winlog.user.name") && event.get_str("winlog.user.name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("winlog.user.name")
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has_value("error.code") {
                if let Some(val) = event.get("error.code") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "error.code".into(),
                            message,
                        }
                    })?;
                    event.set("error.code", converted)?;
                }
            }

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

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
                    event.remove("microsoft_dnsserver.audit.bytes_sent");
                    event.remove("microsoft_dnsserver.audit.file_path");
                    event.remove("microsoft_dnsserver.audit.question_name");
                    event.remove("microsoft_dnsserver.audit.question_type");
                    event.remove("microsoft_dnsserver.audit.source_ip");
                    event.remove("microsoft_dnsserver.audit.ttl");
                    event.remove("winlog.event_id");
                    event.remove("winlog.provider_name");
                    event.remove("winlog.task");
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
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
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
