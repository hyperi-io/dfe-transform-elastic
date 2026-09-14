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

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.type", Value::Array(vec![json!("connection")]))?;

            event.remove("routing.category");

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.time") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "M/d/yyyy h:mm:ss a XXX",
                            "M/d/yyyy h:mm:ss a",
                            "M/d/yyyy H:mm:ss",
                            "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("json.time");

            if event.has_value("json.resourceId") {
                event.rename("json.resourceId", "azure.resource_id")?;
            }

            if event.has_value("json.properties.DestinationIp") {
                event.rename("json.properties.DestinationIp", "destination.ip")?;
            }

            if event.has_value("json.properties.DestinationPort") {
                event.rename("json.properties.DestinationPort", "destination.port")?;
            }

            if event.has_value("json.properties.Policy") {
                event.rename("json.properties.Policy", "azure.firewall.policy")?;
            }

            if event.has_value("json.properties.Protocol") {
                event.rename("json.properties.Protocol", "azure.firewall.protocol")?;
            }

            if event.has_value("json.properties.Rule") {
                event.rename("json.properties.Rule", "rule.name")?;
            }

            if event.has_value("json.properties.RuleCollection") {
                event.rename("json.properties.RuleCollection", "rule.ruleset")?;
            }

            if event.has_value("json.properties.RuleCollectionGroup") {
                event.rename(
                    "json.properties.RuleCollectionGroup",
                    "azure.firewall.rule_collection_group",
                )?;
            }

            if event.has_value("json.properties.SourceIp") {
                event.rename("json.properties.SourceIp", "source.ip")?;
            }

            if event.has_value("json.properties.SourcePort") {
                event.rename("json.properties.SourcePort", "source.port")?;
            }

            if let Some(v) = event
                .get("json.properties.TranslatedIp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.nat.ip", v)?;
            }

            if let Some(v) = event
                .get("json.properties.TranslatedPort")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.nat.port", v)?;
            }

            if event.has_value("json.properties.Action") {
                event.rename("json.properties.Action", "event.action")?;
            }

            if event.has_value("json.properties.ActionReason") {
                event.rename("json.properties.ActionReason", "event.reason")?;
            }

            if event.has_value("json.properties.Fqdn") {
                event.rename("json.properties.Fqdn", "destination.domain")?;
            }

            if event.has_value("json.properties.IsExplicitProxyRequest") {
                event.rename(
                    "json.properties.IsExplicitProxyRequest",
                    "azure.firewall.is_explicit_proxy_request",
                )?;
            }

            if event.has_value("json.properties.IsTlsInspected") {
                event.rename(
                    "json.properties.IsTlsInspected",
                    "azure.firewall.is_tls_inspected",
                )?;
            }

            let _cond = { event.has_value("json.properties.TargetUrl") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if !uri_parts(event, "json.properties.TargetUrl", "url", true, false)?
                        && event
                            .get_str("json.properties.TargetUrl")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "json.properties.TargetUrl".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uri_parts")?;
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

            event.remove("json.properties.TargetUrl");

            if event.has_value("json.properties.WebCategory") {
                event.rename("json.properties.WebCategory", "azure.firewall.web_category")?;
            }

            if event.has_value("json.properties.DnssecOkBit") {
                event.rename(
                    "json.properties.DnssecOkBit",
                    "azure.firewall.dnssec_ok_bit",
                )?;
            }

            if event.has_value("json.properties.EDNS0BufferSize") {
                event.rename(
                    "json.properties.EDNS0BufferSize",
                    "azure.firewall.edns0_buffer_size",
                )?;
            }

            if event.has_value("json.properties.ErrorMessage") {
                event.rename("json.properties.ErrorMessage", "error.message")?;
            }

            let _cond = { event.get_str("json.properties.ErrorNumber") != Some("") };
            if _cond {
                if event.has_value("json.properties.ErrorNumber") {
                    if let Some(val) = event.get("json.properties.ErrorNumber") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.properties.ErrorNumber".into(),
                                message,
                            }
                        })?;
                        event.set("error.id", converted)?;
                    }
                }
            }

            if event.has_value("json.properties.QueryClass") {
                event.rename("json.properties.QueryClass", "dns.question.class")?;
            }

            let _cond = { event.get_str("json.properties.QueryId") != Some("") };
            if _cond {
                if event.has_value("json.properties.QueryId") {
                    if let Some(val) = event.get("json.properties.QueryId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.properties.QueryId".into(),
                                message,
                            }
                        })?;
                        event.set("dns.id", converted)?;
                    }
                }
            }

            if event.has_value("json.properties.QueryName") {
                event.rename("json.properties.QueryName", "dns.question.name")?;
            }

            if event.has_value("json.properties.QueryType") {
                event.rename("json.properties.QueryType", "dns.question.type")?;
            }

            if event.has_value("json.properties.RequestDurationSecs") {
                event.rename(
                    "json.properties.RequestDurationSecs",
                    "azure.firewall.request_duration_secs",
                )?;
            }

            if event.has_value("json.properties.RequestSize") {
                event.rename("json.properties.RequestSize", "azure.firewall.request_size")?;
            }

            if event.has_value("json.properties.ResponseCode") {
                event.rename("json.properties.ResponseCode", "dns.response_code")?;
            }

            if event.has_value("json.properties.ResponseFlags") {
                event.rename("json.properties.ResponseFlags", "dns.header_flags")?;
            }

            if event.has_value("json.properties.ResponseSize") {
                event.rename(
                    "json.properties.ResponseSize",
                    "azure.firewall.response_size",
                )?;
            }

            let _cond = {
                event.get_str("json.operationName") == Some("AzureFirewallNetworkRuleLog")
                    || event.get_str("json.operationName") == Some("AzureFirewallNatRuleLog")
            };
            if _cond {
                if let Some(input) = event.get_string("json.properties.msg") {
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. $
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. Policy: %{DATA:azure.firewall.policy}. Rule Collection Group: %{DATA:azure.firewall.rule_collection_group}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$
                    // Grok pattern: ^%{DATA:azure.firewall.proto} Type=%{DATA:azure.firewall.icmp.request.code} request from %{IPORHOST:source.address} to %{IPORHOST:destination.address}. Action: %{DATA:azure.firewall.action}. $
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long} was DNAT'ed to %{IP:destination.nat.ip}:%{NUMBER:destination.nat.port:long}$
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long} was DNAT'ed to %{IP:destination.nat.ip}:%{NUMBER:destination.nat.port:long}. Policy: %{DATA:azure.firewall.policy}. Rule Collection Group: %{DATA:azure.firewall.rule_collection_group}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long} was DNAT'ed to %{IP:destination.nat.ip}:%{NUMBER:destination.nat.port:long}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:(%{NUMBER:source.port:long})? to %{IPORHOST:destination.address}:(%{NUMBER:destination.port:long})?. Action: %{DATA:azure.firewall.action}. Signature: %{DATA:rule.id}. IDS: %{DATA:rule.name}. Priority: %{NUMBER:event.risk_score:long}. Classification: %{DATA:rule.category}$
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Url: %{HOSTNAME:url.original}. Action: %{DATA:azure.firewall.action}. ThreatIntel: %{DATA:rule.name}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. $"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. Policy: %{DATA:azure.firewall.policy}. Rule Collection Group: %{DATA:azure.firewall.rule_collection_group}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} Type=%{DATA:azure.firewall.icmp.request.code} request from %{IPORHOST:source.address} to %{IPORHOST:destination.address}. Action: %{DATA:azure.firewall.action}. $"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long} was DNAT'ed to %{IP:destination.nat.ip}:%{NUMBER:destination.nat.port:long}$"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long} was DNAT'ed to %{IP:destination.nat.ip}:%{NUMBER:destination.nat.port:long}. Policy: %{DATA:azure.firewall.policy}. Rule Collection Group: %{DATA:azure.firewall.rule_collection_group}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long} was DNAT'ed to %{IP:destination.nat.ip}:%{NUMBER:destination.nat.port:long}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:(%{NUMBER:source.port:long})? to %{IPORHOST:destination.address}:(%{NUMBER:destination.port:long})?. Action: %{DATA:azure.firewall.action}. Signature: %{DATA:rule.id}. IDS: %{DATA:rule.name}. Priority: %{NUMBER:event.risk_score:long}. Classification: %{DATA:rule.category}$"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Url: %{HOSTNAME:url.original}. Action: %{DATA:azure.firewall.action}. ThreatIntel: %{DATA:rule.name}$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond =
                { event.get_str("json.operationName") == Some("AzureFirewallThreatIntelLog") };
            if _cond {
                if let Some(input) = event.get_string("json.properties.msg") {
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. ThreatIntel: %{DATA:rule.name}$
                    if !cached_grok!("^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. ThreatIntel: %{DATA:rule.name}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond =
                { event.get_str("json.operationName") == Some("AzureFirewallApplicationRuleLog") };
            if _cond {
                if let Some(input) = event.get_string("json.properties.msg") {
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. Policy: %{DATA:azure.firewall.policy}. Rule Collection Group: %{DATA:azure.firewall.rule_collection_group}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}. Web Category: %{DATA:rule.category}$
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Url: %{DATA:url.original}. Action: %{DATA:azure.firewall.action}. Policy: %{DATA:azure.firewall.policy}. Rule Collection Group: %{DATA:azure.firewall.rule_collection_group}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Url: %{DATA:url.original}. Action: %{DATA:azure.firewall.action}. %{DATA:event.reason}$
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. %{DATA:event.reason}$
                    // Grok pattern: ^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long}. Action: %{DATA:azure.firewall.action}. Reason: %{DATA:event.reason}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. Policy: %{DATA:azure.firewall.policy}. Rule Collection Group: %{DATA:azure.firewall.rule_collection_group}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}. Web Category: %{DATA:rule.category}$"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Url: %{DATA:url.original}. Action: %{DATA:azure.firewall.action}. Policy: %{DATA:azure.firewall.policy}. Rule Collection Group: %{DATA:azure.firewall.rule_collection_group}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Url: %{DATA:url.original}. Action: %{DATA:azure.firewall.action}. %{DATA:event.reason}$"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. Rule Collection: %{DATA:rule.ruleset}. Rule: %{DATA:rule.name}$"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long} to %{IPORHOST:destination.address}:%{NUMBER:destination.port:long}. Action: %{DATA:azure.firewall.action}. %{DATA:event.reason}$"
                            ),
                            cached_grok!(
                                "^%{DATA:azure.firewall.proto} request from %{IPORHOST:source.address}:%{NUMBER:source.port:long}. Action: %{DATA:azure.firewall.action}. Reason: %{DATA:event.reason}$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("json.operationName") == Some("AzureFirewallDnsProxyLog") };
            if _cond {
                if let Some(input) = event.get_string("json.properties.msg") {
                    // Grok pattern: ^DNS Request: %{IPORHOST:source.address}:%{NUMBER:source.port:long} - %{DATA:azure.firewall.event_original_uid} %{DATA:dns.question.type} %{DATA:dns.question.class} %{DATA:dns.question.name}. %{DATA:network.transport} %{NUMBER:source.bytes:long} %{DATA:azure.firewall.dnssec_bool_flag:boolean} %{NUMBER:azure.firewall.dnssec_buffer_size:long} %{DATA:dns.response_code} %{DATA:dns.header_flags} %{NUMBER:destination.bytes:long} %{DATA:azure.firewall.duration}$
                    if !cached_grok!("^DNS Request: %{IPORHOST:source.address}:%{NUMBER:source.port:long} - %{DATA:azure.firewall.event_original_uid} %{DATA:dns.question.type} %{DATA:dns.question.class} %{DATA:dns.question.name}. %{DATA:network.transport} %{NUMBER:source.bytes:long} %{DATA:azure.firewall.dnssec_bool_flag:boolean} %{NUMBER:azure.firewall.dnssec_buffer_size:long} %{DATA:dns.response_code} %{DATA:dns.header_flags} %{NUMBER:destination.bytes:long} %{DATA:azure.firewall.duration}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get_str("azure.firewall.action") == Some("Deny") };
            if _cond {
                event.set("json.event.alert.action", json!("denied"))?;
            }

            let _cond = { event.get_str("azure.firewall.action") == Some("Allow") };
            if _cond {
                event.set("json.event.alert.action", json!("allowed"))?;
            }

            let _cond = { event.has_value("json.event.alert.action") };
            if _cond {
                event.append(
                    "event.type",
                    json!(
                        event
                            .get("json.event.alert.action")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.eq_ignore_ascii_case("Allow"))
            };
            if _cond {
                event.append_unique("event.type", json!("allowed"))?;
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.eq_ignore_ascii_case("Deny"))
            };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            if event.has_value("source.address") {
                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: %{IP:source.ip}
                    // Grok pattern: %{HOSTNAME:source.domain}
                    if !extract_first_match(
                        &[
                            cached_grok!("%{IP:source.ip}"),
                            cached_grok!("%{HOSTNAME:source.domain}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { !event.has_value("source.address") };
            if _cond {
                if let Some(v) = event
                    .get("source.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.address", v)?;
                }
            }

            if event.has_value("destination.address") {
                if let Some(input) = event.get_string("destination.address") {
                    // Grok pattern: %{IP:destination.ip}
                    // Grok pattern: %{HOSTNAME:destination.domain}
                    if !extract_first_match(
                        &[
                            cached_grok!("%{IP:destination.ip}"),
                            cached_grok!("%{HOSTNAME:destination.domain}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { !event.has_value("destination.address") };
            if _cond {
                if let Some(v) = event
                    .get("destination.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.address", v)?;
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.nat.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.nat.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("source.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("destination.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("dns.question.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("dns.question.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(
                    event,
                    "azure.firewall.proto",
                    "azure.firewall.proto",
                    str::to_lowercase,
                )?;
                Ok(())
            })();

            let _cond = {
                event.has_value("azure.firewall.proto")
                    && ["udp", "tcp", "icmp"]
                        .contains(&event.get_str("azure.firewall.proto").unwrap_or(""))
            };
            if _cond {
                event.set(
                    "network.transport",
                    json!(
                        event
                            .get("azure.firewall.proto")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("network.transport") };
            if _cond {
                event.set(
                    "network.protocol",
                    json!(
                        event
                            .get("azure.firewall.proto")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("azure.firewall.proto");

            let _cond = { event.has_value("azure.firewall.protocol") };
            if _cond {
                map_strings(
                    event,
                    "azure.firewall.protocol",
                    "azure.firewall.protocol",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event.has_value("azure.firewall.protocol")
                    && ["udp", "tcp", "icmp"]
                        .contains(&event.get_str("azure.firewall.protocol").unwrap_or(""))
                    && event.has_value("json.category")
                    && [
                        "AZFWDnsQuery",
                        "AZFWApplicationRule",
                        "AZFWNetworkRule",
                        "AZFWNatRule",
                    ]
                    .contains(&event.get_str("json.category").unwrap_or(""))
            };
            if _cond {
                event.set(
                    "network.transport",
                    json!(
                        event
                            .get("azure.firewall.protocol")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("network.transport")
                    && event.has_value("json.category")
                    && [
                        "AZFWDnsQuery",
                        "AZFWApplicationRule",
                        "AZFWNetworkRule",
                        "AZFWNatRule",
                    ]
                    .contains(&event.get_str("json.category").unwrap_or(""))
            };
            if _cond {
                event.set(
                    "network.protocol",
                    json!(
                        event
                            .get("azure.firewall.protocol")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("azure.firewall.protocol");

            let _cond = { event.has_value("network.transport") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def transport = ctx.network.transport;\nif (transport == 'udp') {\n    ctx.network.iana_number = '17';\n} else if (transport == 'tcp') {\n    ctx.network.iana_number = '6';\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def transport = ctx.network.transport;\nif (transport == 'udp') {\n    ctx.network.iana_number = '17';\n} else if (transport == 'tcp') {\n    ctx.network.iana_number = '6';\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("dns.header_flags") {
                map_strings(
                    event,
                    "dns.header_flags",
                    "dns.header_flags",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("dns.header_flags") {
                if let Some(s) = event.get_string("dns.header_flags") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("dns.header_flags", Value::Array(parts))?;
                }
            }

            let _cond = { event.get_str("json.category") == Some("AZFWDnsQuery") };
            if _cond {
                event.set("dns.type", json!("query"))?;
            }

            if event.has_value("json.category") {
                event.rename("json.category", "azure.firewall.category")?;
            }

            if event.has_value("json.operationName") {
                event.rename("json.operationName", "azure.firewall.operation_name")?;
            }

            event.set("observer.type", json!("firewall"))?;

            event.set("observer.vendor", json!("Azure"))?;

            event.set("observer.product", json!("Network Firewall"))?;

            event.remove("json");
            event.remove("azure.firewall.proto");

            // Begin nested pipeline: "azure-shared-pipeline"
            event.set("cloud.provider", json!("azure"))?;
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))
                    if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_namespace", "azure.resource.namespace"), ("azure_resource_authorization_rule", "azure.resource.authorization_rule")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                }
                Ok(())
            })();
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))
                        if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))
                        if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: (?i)/providers/(?P<azure_resource_provider>(?:.+))
                        if !cached_grok_mapped!(
                            "(?i)/providers/(?P<azure_resource_provider>(?:.+))",
                            [("azure_resource_provider", "azure.resource.provider")]
                        )
                        .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))
                        if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_provider", "azure.resource.provider")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))
                        if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))
                        if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))", [("azure_subscription_id", "azure.subscription_id")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })();
            }
            if event.has_value("azure.resource_id") {
                event.rename("azure.resource_id", "azure.resource.id")?;
            }
            if event.has_value("event.outcome") {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
            }
            if let Some(v) = event
                .get("azure.subscription_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }
            // End nested pipeline: "azure-shared-pipeline"

            if let Some(v) = event
                .get("azure.resource.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
                event.remove("json");
                event.remove("_conf");
                event.remove("message");
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
