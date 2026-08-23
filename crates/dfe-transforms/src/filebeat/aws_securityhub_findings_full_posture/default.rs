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

            event.set("event.kind", json!("state"))?;

            event.append_unique("event.type", json!("info"))?;

            event.append_unique("event.category", json!("configuration"))?;

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("event.original") {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "event.original".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("json", parsed)?;
                }
                Ok(())
            })();

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.UpdatedAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.Id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.CreatedAt") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("observer.vendor", json!("AWS Security Hub CSPM"))?;

            event.set("cloud.provider", json!("aws"))?;

            if event.has("json.Action.ActionType") {
                event.rename(
                    "json.Action.ActionType",
                    "aws.securityhub_findings_full_posture.action.type",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.action.type")
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
                Ok(())
            })();

            if event.has_value("event.action") {
                if let Some(s) = event.get_string("event.action") {
                    let lowered = s.to_lowercase();
                    event.set("event.action", lowered)?;
                }
            }

            if event.has("json.Action.AwsApiCallAction.AffectedResources") {
                event.rename(
                    "json.Action.AwsApiCallAction.AffectedResources",
                    "aws.securityhub_findings_full_posture.action.aws_api_call.affected_resources",
                )?;
            }

            if event.has("json.Action.AwsApiCallAction.Api") {
                event.rename(
                    "json.Action.AwsApiCallAction.Api",
                    "aws.securityhub_findings_full_posture.action.aws_api_call.api",
                )?;
            }

            if event.has("json.Action.AwsApiCallAction.CallerType") {
                event.rename(
                    "json.Action.AwsApiCallAction.CallerType",
                    "aws.securityhub_findings_full_posture.action.aws_api_call.caller.type",
                )?;
            }

            if event.has("json.Action.AwsApiCallAction.DomainDetails.Domain") {
                event.rename("json.Action.AwsApiCallAction.DomainDetails.Domain", "aws.securityhub_findings_full_posture.action.aws_api_call.domain_details.domain")?;
            }

            let _cond = {
                event.has_value("json.Action.AwsApiCallAction.FirstSeen")
                    && event.get_str("json.Action.AwsApiCallAction.FirstSeen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.Action.AwsApiCallAction.FirstSeen")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.securityhub_findings_full_posture.action.aws_api_call.first_seen", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_Action_AwsApiCallAction_FirstSeen_to_aws_securityhub_findings_full_posture_action_aws_api_call_first_seen_d7b57296")?;
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

            let _cond = {
                event.has_value("json.Action.AwsApiCallAction.LastSeen")
                    && event.get_str("json.Action.AwsApiCallAction.LastSeen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.Action.AwsApiCallAction.LastSeen")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.securityhub_findings_full_posture.action.aws_api_call.last_seen", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_Action_AwsApiCallAction_LastSeen_to_aws_securityhub_findings_full_posture_action_aws_api_call_last_seen_702bbc50")?;
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

            if event.has("json.Action.AwsApiCallAction.RemoteIpDetails.City.CityName") {
                event.rename(
                    "json.Action.AwsApiCallAction.RemoteIpDetails.City.CityName",
                    "aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.city.name",
                )?;
            }

            if event.has("json.Action.AwsApiCallAction.RemoteIpDetails.Country.CountryCode") {
                event.rename("json.Action.AwsApiCallAction.RemoteIpDetails.Country.CountryCode", "aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.country.code")?;
            }

            if event.has("json.Action.AwsApiCallAction.RemoteIpDetails.Country.CountryName") {
                event.rename("json.Action.AwsApiCallAction.RemoteIpDetails.Country.CountryName", "aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.country.name")?;
            }

            let _cond = {
                event.get_str("json.Action.AwsApiCallAction.RemoteIpDetails.GeoLocation.Lat")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("json.Action.AwsApiCallAction.RemoteIpDetails.GeoLocation.Lat")
                    {
                        if let Some(val) = event
                            .get("json.Action.AwsApiCallAction.RemoteIpDetails.GeoLocation.Lat")
                        {
                            let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.Action.AwsApiCallAction.RemoteIpDetails.GeoLocation.Lat".into(),
                            message,
                        })?;
                            event.set("aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.geolocation.latitude", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_AwsApiCallAction_RemoteIpDetails_GeoLocation_Lat_to_aws_securityhub_findings_full_posture_action_aws_api_call_remote_ip_geolocation_latitude_3485d26f")?;
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

            let _cond = {
                event.get_str("json.Action.AwsApiCallAction.RemoteIpDetails.GeoLocation.Lon")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("json.Action.AwsApiCallAction.RemoteIpDetails.GeoLocation.Lon")
                    {
                        if let Some(val) = event
                            .get("json.Action.AwsApiCallAction.RemoteIpDetails.GeoLocation.Lon")
                        {
                            let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.Action.AwsApiCallAction.RemoteIpDetails.GeoLocation.Lon".into(),
                            message,
                        })?;
                            event.set("aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.geolocation.longitude", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_AwsApiCallAction_RemoteIpDetails_GeoLocation_Lon_to_aws_securityhub_findings_full_posture_action_aws_api_call_remote_ip_geolocation_longitude_ceb3046c")?;
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

            let _cond = {
                event.get_str("json.Action.AwsApiCallAction.RemoteIpDetails.IpAddressV4")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Action.AwsApiCallAction.RemoteIpDetails.IpAddressV4") {
                        if let Some(val) =
                            event.get("json.Action.AwsApiCallAction.RemoteIpDetails.IpAddressV4")
                        {
                            let converted =
                                convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                            path: "json.Action.AwsApiCallAction.RemoteIpDetails.IpAddressV4".into(),
                            message,
                        }
                                })?;
                            event.set("aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.ip.address_v4", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_AwsApiCallAction_RemoteIpDetails_IpAddressV4_to_aws_securityhub_findings_full_posture_action_aws_api_call_remote_ip_ip_address_v4_e47e580c")?;
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

            let _cond = {
                event.get_str("json.Action.AwsApiCallAction.RemoteIpDetails.Organization.Asn")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("json.Action.AwsApiCallAction.RemoteIpDetails.Organization.Asn")
                    {
                        if let Some(val) = event
                            .get("json.Action.AwsApiCallAction.RemoteIpDetails.Organization.Asn")
                        {
                            let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.Action.AwsApiCallAction.RemoteIpDetails.Organization.Asn".into(),
                            message,
                        })?;
                            event.set("aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.organization.asn", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_AwsApiCallAction_RemoteIpDetails_Organization_Asn_to_aws_securityhub_findings_full_posture_action_aws_api_call_remote_ip_organization_asn_327bda3e")?;
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

            if event.has("json.Action.AwsApiCallAction.RemoteIpDetails.Organization.AsnOrg") {
                event.rename("json.Action.AwsApiCallAction.RemoteIpDetails.Organization.AsnOrg", "aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.organization.asn_organization")?;
            }

            if event.has("json.Action.AwsApiCallAction.RemoteIpDetails.Organization.Isp") {
                event.rename("json.Action.AwsApiCallAction.RemoteIpDetails.Organization.Isp", "aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.organization.internet_service_provider")?;
            }

            if event.has("json.Action.AwsApiCallAction.RemoteIpDetails.Organization.Org") {
                event.rename("json.Action.AwsApiCallAction.RemoteIpDetails.Organization.Org", "aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.organization.internet_provider")?;
            }

            if event.has("json.Action.AwsApiCallAction.ServiceName") {
                event.rename(
                    "json.Action.AwsApiCallAction.ServiceName",
                    "aws.securityhub_findings_full_posture.action.aws_api_call.service.name",
                )?;
            }

            let _cond = { event.get_str("json.Action.DnsRequestAction.Blocked") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Action.DnsRequestAction.Blocked") {
                        if let Some(val) = event.get("json.Action.DnsRequestAction.Blocked") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Action.DnsRequestAction.Blocked".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.action.dns_request.blocked",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_DnsRequestAction_Blocked_to_aws_securityhub_findings_full_posture_action_dns_request_blocked_c1a4d0aa")?;
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

            if event.has("json.Action.DnsRequestAction.Domain") {
                event.rename(
                    "json.Action.DnsRequestAction.Domain",
                    "aws.securityhub_findings_full_posture.action.dns_request.domain",
                )?;
            }

            if event.has("json.Action.DnsRequestAction.Protocol") {
                event.rename(
                    "json.Action.DnsRequestAction.Protocol",
                    "aws.securityhub_findings_full_posture.action.dns_request.protocol",
                )?;
            }

            let _cond =
                { event.get_str("json.Action.NetworkConnectionAction.Blocked") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Action.NetworkConnectionAction.Blocked") {
                        if let Some(val) = event.get("json.Action.NetworkConnectionAction.Blocked")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Action.NetworkConnectionAction.Blocked".into(),
                                    message,
                                }
                            })?;
                            event.set("aws.securityhub_findings_full_posture.action.network_connection.blocked", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_NetworkConnectionAction_Blocked_to_aws_securityhub_findings_full_posture_action_network_connection_blocked_facd05ce")?;
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

            if event.has("json.Action.NetworkConnectionAction.ConnectionDirection") {
                event.rename(
                    "json.Action.NetworkConnectionAction.ConnectionDirection",
                    "aws.securityhub_findings_full_posture.action.network_connection.direction",
                )?;
            }

            let _cond = {
                event.get_str("json.Action.NetworkConnectionAction.LocalPortDetails.Port")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Action.NetworkConnectionAction.LocalPortDetails.Port")
                    {
                        if let Some(val) =
                            event.get("json.Action.NetworkConnectionAction.LocalPortDetails.Port")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "json.Action.NetworkConnectionAction.LocalPortDetails.Port"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set("aws.securityhub_findings_full_posture.action.network_connection.local.port.number", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_NetworkConnectionAction_LocalPortDetails_Port_to_aws_securityhub_findings_full_posture_action_network_connection_local_port_number_e8bc8418")?;
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

            if event.has("json.Action.NetworkConnectionAction.LocalPortDetails.PortName") {
                event.rename("json.Action.NetworkConnectionAction.LocalPortDetails.PortName", "aws.securityhub_findings_full_posture.action.network_connection.local.port.name")?;
            }

            if event.has("json.Action.NetworkConnectionAction.Protocol") {
                event.rename(
                    "json.Action.NetworkConnectionAction.Protocol",
                    "aws.securityhub_findings_full_posture.action.network_connection.protocol",
                )?;
            }

            if event.has("json.Action.NetworkConnectionAction.RemoteIpDetails.City.CityName") {
                event.rename("json.Action.NetworkConnectionAction.RemoteIpDetails.City.CityName", "aws.securityhub_findings_full_posture.action.network_connection.remote_ip.city.name")?;
            }

            if event.has("json.Action.NetworkConnectionAction.RemoteIpDetails.Country.CountryCode")
            {
                event.rename("json.Action.NetworkConnectionAction.RemoteIpDetails.Country.CountryCode", "aws.securityhub_findings_full_posture.action.network_connection.remote_ip.country.code")?;
            }

            if event.has("json.Action.NetworkConnectionAction.RemoteIpDetails.Country.CountryName")
            {
                event.rename("json.Action.NetworkConnectionAction.RemoteIpDetails.Country.CountryName", "aws.securityhub_findings_full_posture.action.network_connection.remote_ip.country.name")?;
            }

            let _cond = {
                event.get_str("json.Action.NetworkConnectionAction.RemoteIpDetails.GeoLocation.Lat")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "json.Action.NetworkConnectionAction.RemoteIpDetails.GeoLocation.Lat",
                    ) {
                        if let Some(val) = event.get(
                            "json.Action.NetworkConnectionAction.RemoteIpDetails.GeoLocation.Lat",
                        ) {
                            let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.Action.NetworkConnectionAction.RemoteIpDetails.GeoLocation.Lat".into(),
                            message,
                        })?;
                            event.set("aws.securityhub_findings_full_posture.action.network_connection.remote_ip.geolocation.latitude", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_NetworkConnectionAction_RemoteIpDetails_GeoLocation_Lat_to_aws_securityhub_findings_full_posture_action_network_connection_remote_ip_geolocation_latitude_570161e7")?;
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

            let _cond = {
                event.get_str("json.Action.NetworkConnectionAction.RemoteIpDetails.GeoLocation.Lon")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "json.Action.NetworkConnectionAction.RemoteIpDetails.GeoLocation.Lon",
                    ) {
                        if let Some(val) = event.get(
                            "json.Action.NetworkConnectionAction.RemoteIpDetails.GeoLocation.Lon",
                        ) {
                            let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.Action.NetworkConnectionAction.RemoteIpDetails.GeoLocation.Lon".into(),
                            message,
                        })?;
                            event.set("aws.securityhub_findings_full_posture.action.network_connection.remote_ip.geolocation.longitude", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_NetworkConnectionAction_RemoteIpDetails_GeoLocation_Lon_to_aws_securityhub_findings_full_posture_action_network_connection_remote_ip_geolocation_longitude_3aa8d0a0")?;
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

            let _cond = {
                event.get_str("json.Action.NetworkConnectionAction.RemoteIpDetails.IpAddressV4")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "json.Action.NetworkConnectionAction.RemoteIpDetails.IpAddressV4",
                    ) {
                        if let Some(val) = event
                            .get("json.Action.NetworkConnectionAction.RemoteIpDetails.IpAddressV4")
                        {
                            let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.Action.NetworkConnectionAction.RemoteIpDetails.IpAddressV4".into(),
                            message,
                        })?;
                            event.set("aws.securityhub_findings_full_posture.action.network_connection.remote_ip.ip.address_v4", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_NetworkConnectionAction_RemoteIpDetails_IpAddressV4_to_aws_securityhub_findings_full_posture_action_network_connection_remote_ip_ip_address_v4_87901f94")?;
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

            let _cond = {
                event
                    .get_str("json.Action.NetworkConnectionAction.RemoteIpDetails.Organization.Asn")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "json.Action.NetworkConnectionAction.RemoteIpDetails.Organization.Asn",
                    ) {
                        if let Some(val) = event.get(
                            "json.Action.NetworkConnectionAction.RemoteIpDetails.Organization.Asn",
                        ) {
                            let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.Action.NetworkConnectionAction.RemoteIpDetails.Organization.Asn".into(),
                            message,
                        })?;
                            event.set("aws.securityhub_findings_full_posture.action.network_connection.remote_ip.organization.asn", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_NetworkConnectionAction_RemoteIpDetails_Organization_Asn_to_aws_securityhub_findings_full_posture_action_network_connection_remote_ip_organization_asn_135d7b38")?;
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

            if event.has("json.Action.NetworkConnectionAction.RemoteIpDetails.Organization.AsnOrg")
            {
                event.rename("json.Action.NetworkConnectionAction.RemoteIpDetails.Organization.AsnOrg", "aws.securityhub_findings_full_posture.action.network_connection.remote_ip.organization.asn_organization")?;
            }

            if event.has("json.Action.NetworkConnectionAction.RemoteIpDetails.Organization.Isp") {
                event.rename("json.Action.NetworkConnectionAction.RemoteIpDetails.Organization.Isp", "aws.securityhub_findings_full_posture.action.network_connection.remote_ip.organization.internet_service_provider")?;
            }

            if event.has("json.Action.NetworkConnectionAction.RemoteIpDetails.Organization.Org") {
                event.rename("json.Action.NetworkConnectionAction.RemoteIpDetails.Organization.Org", "aws.securityhub_findings_full_posture.action.network_connection.remote_ip.organization.internet_provider")?;
            }

            let _cond = {
                event.get_str("json.Action.NetworkConnectionAction.RemotePortDetails.Port")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Action.NetworkConnectionAction.RemotePortDetails.Port")
                    {
                        if let Some(val) =
                            event.get("json.Action.NetworkConnectionAction.RemotePortDetails.Port")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "json.Action.NetworkConnectionAction.RemotePortDetails.Port"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set("aws.securityhub_findings_full_posture.action.network_connection.remote.port.number", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_NetworkConnectionAction_RemotePortDetails_Port_to_aws_securityhub_findings_full_posture_action_network_connection_remote_port_number_46a9223b")?;
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

            if event.has("json.Action.NetworkConnectionAction.RemotePortDetails.PortName") {
                event.rename("json.Action.NetworkConnectionAction.RemotePortDetails.PortName", "aws.securityhub_findings_full_posture.action.network_connection.remote.port.name")?;
            }

            let _cond = { event.get_str("json.Action.PortProbeAction.Blocked") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Action.PortProbeAction.Blocked") {
                        if let Some(val) = event.get("json.Action.PortProbeAction.Blocked") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Action.PortProbeAction.Blocked".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.action.port_probe.blocked",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Action_PortProbeAction_Blocked_to_aws_securityhub_findings_full_posture_action_port_probe_blocked_3ece0655")?;
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

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.LocalIpDetails.IpAddressV4") {
                                    if let Some(val) =
                                        event.get("_ingest._value.LocalIpDetails.IpAddressV4")
                                    {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path:
                                                        "_ingest._value.LocalIpDetails.IpAddressV4"
                                                            .into(),
                                                    message,
                                                }
                                            })?;
                                        event
                                            .set("_ingest._value.local.ip.address_v4", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
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
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.LocalPortDetails.Port") {
                                    if let Some(val) =
                                        event.get("_ingest._value.LocalPortDetails.Port")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.LocalPortDetails.Port"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.local.port.number", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
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
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.LocalPortDetails.PortName") {
                                event.rename(
                                    "_ingest._value.LocalPortDetails.PortName",
                                    "_ingest._value.local.port.name",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.RemoteIpDetails.City.CityName") {
                                event.rename(
                                    "_ingest._value.RemoteIpDetails.City.CityName",
                                    "_ingest._value.remote_ip.city.name",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.RemoteIpDetails.Country.CountryCode") {
                                event.rename(
                                    "_ingest._value.RemoteIpDetails.Country.CountryCode",
                                    "_ingest._value.remote_ip.country.code",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.RemoteIpDetails.Country.CountryName") {
                                event.rename(
                                    "_ingest._value.RemoteIpDetails.Country.CountryName",
                                    "_ingest._value.remote_ip.country.name",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.RemoteIpDetails.GeoLocation.Lat")
                                {
                                    if let Some(val) =
                                        event.get("_ingest._value.RemoteIpDetails.GeoLocation.Lat")
                                    {
                                        let converted =
                                            convert_value(val, "double").map_err(|message| {
                                                TransformError::ParseError {
                        path: "_ingest._value.RemoteIpDetails.GeoLocation.Lat".into(),
                        message,
                        }
                                            })?;
                                        event.set(
                                            "_ingest._value.remote_ip.geolocation.latitude",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
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
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.RemoteIpDetails.GeoLocation.Lon")
                                {
                                    if let Some(val) =
                                        event.get("_ingest._value.RemoteIpDetails.GeoLocation.Lon")
                                    {
                                        let converted =
                                            convert_value(val, "double").map_err(|message| {
                                                TransformError::ParseError {
                        path: "_ingest._value.RemoteIpDetails.GeoLocation.Lon".into(),
                        message,
                        }
                                            })?;
                                        event.set(
                                            "_ingest._value.remote_ip.geolocation.longitude",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
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
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.RemoteIpDetails.IpAddressV4") {
                                    if let Some(val) =
                                        event.get("_ingest._value.RemoteIpDetails.IpAddressV4")
                                    {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path:
                                                        "_ingest._value.RemoteIpDetails.IpAddressV4"
                                                            .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.remote_ip.ip.address_v4",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
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
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event
                                    .has_value("_ingest._value.RemoteIpDetails.Organization.Asn")
                                {
                                    if let Some(val) =
                                        event.get("_ingest._value.RemoteIpDetails.Organization.Asn")
                                    {
                                        let converted =
                                            convert_value(val, "string").map_err(|message| {
                                                TransformError::ParseError {
                        path: "_ingest._value.RemoteIpDetails.Organization.Asn".into(),
                        message,
                        }
                                            })?;
                                        event.set(
                                            "_ingest._value.remote_ip.organization.asn",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
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
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.RemoteIpDetails.Organization.AsnOrg") {
                                event.rename(
                                    "_ingest._value.RemoteIpDetails.Organization.AsnOrg",
                                    "_ingest._value.remote_ip.organization.asn_organization",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.RemoteIpDetails.Organization.Isp") {
                                event.rename("_ingest._value.RemoteIpDetails.Organization.Isp", "_ingest._value.remote_ip.organization.internet_service_provider")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.RemoteIpDetails.Organization.Org") {
                                event.rename(
                                    "_ingest._value.RemoteIpDetails.Organization.Org",
                                    "_ingest._value.remote_ip.organization.internet_provider",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Action.PortProbeAction.PortProbeDetails")
                    && event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.Action.PortProbeAction.PortProbeDetails")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            event.remove("_ingest._value.LocalIpDetails");
                            event.remove("_ingest._value.LocalPortDetails");
                            event.remove("_ingest._value.RemoteIpDetails");
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.Action.PortProbeAction.PortProbeDetails",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            if event.has("json.Action.PortProbeAction.PortProbeDetails") {
                event.rename(
                    "json.Action.PortProbeAction.PortProbeDetails",
                    "aws.securityhub_findings_full_posture.action.port_probe.details",
                )?;
            }

            if event.has("json.AwsAccountId") {
                event.rename(
                    "json.AwsAccountId",
                    "aws.securityhub_findings_full_posture.aws_account_id",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.aws_account_id")
                    .cloned()
                {
                    event.set("cloud.account.id", v)?;
                }
                Ok(())
            })();

            if event.has("json.CompanyName") {
                event.rename(
                    "json.CompanyName",
                    "aws.securityhub_findings_full_posture.company.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.company.name")
                    .cloned()
                {
                    event.set("organization.name", v)?;
                }
                Ok(())
            })();

            if event.has("json.Compliance.RelatedRequirements") {
                event.rename(
                    "json.Compliance.RelatedRequirements",
                    "aws.securityhub_findings_full_posture.compliance.related_requirements",
                )?;
            }

            let _cond = {
                event
                    .get("aws.securityhub_findings_full_posture.compliance.related_requirements")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event
                    .get("aws.securityhub_findings_full_posture.compliance.related_requirements")
                    .cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "rule.ruleset",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set(
                        "aws.securityhub_findings_full_posture.compliance.related_requirements",
                        Value::Array(out),
                    )?;
                }
            }

            if event.has("json.Compliance.Status") {
                event.rename(
                    "json.Compliance.Status",
                    "aws.securityhub_findings_full_posture.compliance.status",
                )?;
            }

            let _cond = {
                event.get_str("aws.securityhub_findings_full_posture.compliance.status")
                    == Some("PASSED")
            };
            if _cond {
                let v = json!("passed");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            let _cond = {
                event.get_str("aws.securityhub_findings_full_posture.compliance.status")
                    == Some("FAILED")
            };
            if _cond {
                let v = json!("failed");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            let _cond = { !event.has_value("result.evaluation") };
            if _cond {
                let v = json!("unknown");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            let _cond = {
                event.get_str("aws.securityhub_findings_full_posture.compliance.status")
                    == Some("PASSED")
            };
            if _cond {
                let v = json!("success");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
            }

            let _cond = {
                event.get_str("aws.securityhub_findings_full_posture.compliance.status")
                    == Some("FAILED")
            };
            if _cond {
                let v = json!("failure");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = {
                event.has_value("json.Compliance.StatusReasons")
                    && event
                        .get("json.Compliance.StatusReasons")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) =
                        event.get("json.Compliance.StatusReasons").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Description") {
                                event.rename(
                                    "_ingest._value.Description",
                                    "_ingest._value.description",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Compliance.StatusReasons", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Compliance.StatusReasons")
                    && event
                        .get("json.Compliance.StatusReasons")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) =
                        event.get("json.Compliance.StatusReasons").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.ReasonCode") {
                                event.rename(
                                    "_ingest._value.ReasonCode",
                                    "_ingest._value.reason_code",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Compliance.StatusReasons", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            if event.has("json.Compliance.StatusReasons") {
                event.rename(
                    "json.Compliance.StatusReasons",
                    "aws.securityhub_findings_full_posture.compliance.status_reasons",
                )?;
            }

            let _cond = { event.get_str("json.Confidence") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Confidence") {
                        if let Some(val) = event.get("json.Confidence") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Confidence".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.confidence",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Confidence_to_aws_securityhub_findings_full_posture_confidence_507c2887")?;
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

            let _cond = {
                event.has_value("json.CreatedAt") && event.get_str("json.CreatedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.CreatedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event
                                .set("aws.securityhub_findings_full_posture.created_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_CreatedAt_to_aws_securityhub_findings_full_posture_created_at_187e2750")?;
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

            let _cond = {
                event.has_value("json.UpdatedAt") && event.get_str("json.UpdatedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.UpdatedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event
                                .set("aws.securityhub_findings_full_posture.updated_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_updated_at")?;
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
            }

            event.set(
                "@timestamp",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, painless_to_string)
                ),
            )?;

            let _cond = { event.get_str("json.Criticality") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Criticality") {
                        if let Some(val) = event.get("json.Criticality") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Criticality".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.criticality",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Criticality_to_aws_securityhub_findings_full_posture_criticality_693fc1fc")?;
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

            if event.has("json.Description") {
                event.rename(
                    "json.Description",
                    "aws.securityhub_findings_full_posture.description",
                )?;
            }

            if let Some(v) = event
                .get("aws.securityhub_findings_full_posture.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.description", v)?;
            }

            let _cond = { event.get_str("json.FindingProviderFields.Confidence") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.FindingProviderFields.Confidence") {
                        if let Some(val) = event.get("json.FindingProviderFields.Confidence") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.FindingProviderFields.Confidence".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.provider_fields.confidence",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_FindingProviderFields_Confidence_to_aws_securityhub_findings_full_posture_provider_fields_confidence_9ce2bcfb")?;
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

            let _cond = { event.get_str("json.FindingProviderFields.Criticality") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.FindingProviderFields.Criticality") {
                        if let Some(val) = event.get("json.FindingProviderFields.Criticality") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.FindingProviderFields.Criticality".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.provider_fields.criticality",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_FindingProviderFields_Criticality_to_aws_securityhub_findings_full_posture_provider_fields_criticality_90ef4822")?;
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

            let _cond = {
                event.has_value("json.FindingProviderFields.RelatedFindings")
                    && event
                        .get("json.FindingProviderFields.RelatedFindings")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.FindingProviderFields.RelatedFindings")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Id") {
                                event.rename("_ingest._value.Id", "_ingest._value.id")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.FindingProviderFields.RelatedFindings",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.FindingProviderFields.RelatedFindings")
                    && event
                        .get("json.FindingProviderFields.RelatedFindings")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("json.FindingProviderFields.RelatedFindings")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.ProductArn") {
                                event.rename(
                                    "_ingest._value.ProductArn",
                                    "_ingest._value.product.arn",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "json.FindingProviderFields.RelatedFindings",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            if event.has("json.FindingProviderFields.RelatedFindings") {
                event.rename(
                    "json.FindingProviderFields.RelatedFindings",
                    "aws.securityhub_findings_full_posture.provider_fields.related_findings",
                )?;
            }

            if event.has("json.FindingProviderFields.Severity.Label") {
                event.rename(
                    "json.FindingProviderFields.Severity.Label",
                    "aws.securityhub_findings_full_posture.provider_fields.severity.label",
                )?;
            }

            if event.has("json.FindingProviderFields.Severity.Original") {
                event.rename(
                    "json.FindingProviderFields.Severity.Original",
                    "aws.securityhub_findings_full_posture.provider_fields.severity.original",
                )?;
            }

            let _cond =
                { event.get_str("json.FindingProviderFields.Severity.Normalized") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.FindingProviderFields.Severity.Normalized") {
                        if let Some(val) =
                            event.get("json.FindingProviderFields.Severity.Normalized")
                        {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.FindingProviderFields.Severity.Normalized".into(),
                                    message,
                                }
                            })?;
                            event.set("aws.securityhub_findings_full_posture.provider_fields.severity.normalized", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_FindingProviderFields_Severity_Normalized_to_aws_securityhub_findings_full_posture_provider_fields_severity_normalized_bc5cc1bb")?;
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

            let _cond =
                { event.get_str("json.FindingProviderFields.Severity.Product") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.FindingProviderFields.Severity.Product") {
                        if let Some(val) = event.get("json.FindingProviderFields.Severity.Product")
                        {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.FindingProviderFields.Severity.Product".into(),
                                    message,
                                }
                            })?;
                            event.set("aws.securityhub_findings_full_posture.provider_fields.severity.product", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_FindingProviderFields_Severity_Product_to_aws_securityhub_findings_full_posture_provider_fields_severity_product_97ef4783")?;
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

            if event.has("json.FindingProviderFields.Types") {
                event.rename(
                    "json.FindingProviderFields.Types",
                    "aws.securityhub_findings_full_posture.provider_fields.types",
                )?;
            }

            let _cond = {
                event.has_value("json.FirstObservedAt")
                    && event.get_str("json.FirstObservedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.FirstObservedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.securityhub_findings_full_posture.first_observed_at",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_FirstObservedAt_to_aws_securityhub_findings_full_posture_first_observed_at_ee1f7241")?;
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

            if event.has("json.GeneratorId") {
                event.rename(
                    "json.GeneratorId",
                    "aws.securityhub_findings_full_posture.generator.id",
                )?;
            }

            if let Some(v) = event
                .get("aws.securityhub_findings_full_posture.generator.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if event.has("json.Compliance.SecurityControlId") {
                event.rename(
                    "json.Compliance.SecurityControlId",
                    "aws.securityhub_findings_full_posture.compliance.security_control_id",
                )?;
            }

            let _cond = { !event.has_value("rule.id") };
            if _cond {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.compliance.security_control_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.id", v)?;
                }
            }

            if event.has("json.Id") {
                event.rename("json.Id", "aws.securityhub_findings_full_posture.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.id")
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("json.LastObservedAt")
                    && event.get_str("json.LastObservedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.LastObservedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.securityhub_findings_full_posture.last_observed_at",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_LastObservedAt_to_aws_securityhub_findings_full_posture_last_observed_at_31973917")?;
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

            let _cond = {
                event.has_value("json.ProcessedAt") && event.get_str("json.ProcessedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.ProcessedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.securityhub_findings_full_posture.processed_at",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_processed_at")?;
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
            }

            if let Some(v) = event
                .get("aws.securityhub_findings_full_posture.processed_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            let _cond = {
                event.has_value("json.Malware")
                    && event.get("json.Malware").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Malware").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Name") {
                                event.rename("_ingest._value.Name", "_ingest._value.name")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Malware", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Malware")
                    && event.get("json.Malware").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Malware").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Path") {
                                event.rename("_ingest._value.Path", "_ingest._value.path")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Malware", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Malware")
                    && event.get("json.Malware").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Malware").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.State") {
                                event.rename("_ingest._value.State", "_ingest._value.state")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Malware", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Malware")
                    && event.get("json.Malware").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Malware").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Type") {
                                event.rename("_ingest._value.Type", "_ingest._value.type")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Malware", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            if event.has("json.Malware") {
                event.rename(
                    "json.Malware",
                    "aws.securityhub_findings_full_posture.malware",
                )?;
            }

            if event.has("json.Network.DestinationDomain") {
                event.rename(
                    "json.Network.DestinationDomain",
                    "aws.securityhub_findings_full_posture.network.destination.domain",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.network.destination.domain")
                    .cloned()
                {
                    event.set("destination.domain", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.Network.DestinationIpV4") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Network.DestinationIpV4") {
                        if let Some(val) = event.get("json.Network.DestinationIpV4") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Network.DestinationIpV4".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.network.destination.ip.v4",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Network_DestinationIpV4_to_aws_securityhub_findings_full_posture_network_destination_ip_v4_f281d634")?;
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

            let _cond = {
                event.has_value("aws.securityhub_findings_full_posture.network.destination.ip.v4")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique("destination.ip", json!(event.get("aws.securityhub_findings_full_posture.network.destination.ip.v4").map_or_else(String::new, painless_to_string)))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("json.Network.DestinationIpV6") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Network.DestinationIpV6") {
                        if let Some(val) = event.get("json.Network.DestinationIpV6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Network.DestinationIpV6".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.network.destination.ip.v6",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Network_DestinationIpV6_to_aws_securityhub_findings_full_posture_network_destination_ip_v6_f1cb817e")?;
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

            let _cond = {
                event.has_value("aws.securityhub_findings_full_posture.network.destination.ip.v6")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique("destination.ip", json!(event.get("aws.securityhub_findings_full_posture.network.destination.ip.v6").map_or_else(String::new, painless_to_string)))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("json.Network.DestinationPort") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Network.DestinationPort") {
                        if let Some(val) = event.get("json.Network.DestinationPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Network.DestinationPort".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.network.destination.port",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Network_DestinationPort_to_aws_securityhub_findings_full_posture_network_destination_port_792dabe1")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.network.destination.port")
                    .cloned()
                {
                    event.set("destination.port", v)?;
                }
                Ok(())
            })();

            if event.has("json.Network.Direction") {
                event.rename(
                    "json.Network.Direction",
                    "aws.securityhub_findings_full_posture.network.direction",
                )?;
            }

            let _cond = {
                event.get_str("aws.securityhub_findings_full_posture.network.direction")
                    == Some("IN")
            };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            let _cond = {
                event.get_str("aws.securityhub_findings_full_posture.network.direction")
                    == Some("OUT")
            };
            if _cond {
                event.set("network.direction", json!("outbound"))?;
            }

            let _cond = { event.get_str("json.Network.OpenPortRange.Begin") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Network.OpenPortRange.Begin") {
                        if let Some(val) = event.get("json.Network.OpenPortRange.Begin") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Network.OpenPortRange.Begin".into(),
                                    message,
                                }
                            })?;
                            event.set("aws.securityhub_findings_full_posture.network.open_port_range.begin", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Network_OpenPortRange_Begin_to_aws_securityhub_findings_full_posture_network_open_port_range_begin_10203b92")?;
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

            let _cond = { event.get_str("json.Network.OpenPortRange.End") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Network.OpenPortRange.End") {
                        if let Some(val) = event.get("json.Network.OpenPortRange.End") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Network.OpenPortRange.End".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.network.open_port_range.end",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Network_OpenPortRange_End_to_aws_securityhub_findings_full_posture_network_open_port_range_end_1a46c98e")?;
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

            if event.has("json.Network.Protocol") {
                event.rename(
                    "json.Network.Protocol",
                    "aws.securityhub_findings_full_posture.network.protocol",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.network.protocol")
                    .cloned()
                {
                    event.set("network.protocol", v)?;
                }
                Ok(())
            })();

            if event.has_value("network.protocol") {
                if let Some(s) = event.get_string("network.protocol") {
                    let lowered = s.to_lowercase();
                    event.set("network.protocol", lowered)?;
                }
            }

            if event.has("json.Network.SourceDomain") {
                event.rename(
                    "json.Network.SourceDomain",
                    "aws.securityhub_findings_full_posture.network.source.domain",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.network.source.domain")
                    .cloned()
                {
                    event.set("source.domain", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.Network.SourceIpV4") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Network.SourceIpV4") {
                        if let Some(val) = event.get("json.Network.SourceIpV4") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Network.SourceIpV4".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.network.source.ip.v4",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Network_SourceIpV4_to_aws_securityhub_findings_full_posture_network_source_ip_v4_3e09c5d7")?;
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

            let _cond =
                { event.has_value("aws.securityhub_findings_full_posture.network.source.ip.v4") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "source.ip",
                        json!(
                            event
                                .get("aws.securityhub_findings_full_posture.network.source.ip.v4")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("json.Network.SourceIpV6") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Network.SourceIpV6") {
                        if let Some(val) = event.get("json.Network.SourceIpV6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Network.SourceIpV6".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.network.source.ip.v6",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Network_SourceIpV6_to_aws_securityhub_findings_full_posture_network_source_ip_v6_9520fa9d")?;
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

            let _cond =
                { event.has_value("aws.securityhub_findings_full_posture.network.source.ip.v6") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "source.ip",
                        json!(
                            event
                                .get("aws.securityhub_findings_full_posture.network.source.ip.v6")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.Network.SourceMac") {
                event.rename(
                    "json.Network.SourceMac",
                    "aws.securityhub_findings_full_posture.network.source.mac",
                )?;
            }

            if event.has_value("aws.securityhub_findings_full_posture.network.source.mac") {
                if let Some(s) =
                    event.get_string("aws.securityhub_findings_full_posture.network.source.mac")
                {
                    let re = cached_regex!("[-:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set(
                        "aws.securityhub_findings_full_posture.network.source.mac",
                        replaced,
                    )?;
                }
            }

            if event.has_value("aws.securityhub_findings_full_posture.network.source.mac") {
                if let Some(s) =
                    event.get_string("aws.securityhub_findings_full_posture.network.source.mac")
                {
                    let uppered = s.to_uppercase();
                    event.set(
                        "aws.securityhub_findings_full_posture.network.source.mac",
                        uppered,
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.network.source.mac")
                    .cloned()
                {
                    event.set("source.mac", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.Network.SourcePort") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Network.SourcePort") {
                        if let Some(val) = event.get("json.Network.SourcePort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Network.SourcePort".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.network.source.port",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Network_SourcePort_to_aws_securityhub_findings_full_posture_network_source_port_ab64dd08")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.network.source.port")
                    .cloned()
                {
                    event.set("source.port", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.ComponentId") {
                                event.rename(
                                    "_ingest._value.ComponentId",
                                    "_ingest._value.component.id",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.ComponentType") {
                                event.rename(
                                    "_ingest._value.ComponentType",
                                    "_ingest._value.component.type",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Egress.Destination.Address") {
                                event.rename(
                                    "_ingest._value.Egress.Destination.Address",
                                    "_ingest._value.egress.destination.address",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Egress.Destination.PortRanges") {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Egress.Destination.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.Begin") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.Begin")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.Begin"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.begin",
                                                            converted,
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.append(
                                                    "error.message",
                                                    json!(
                                                        event
                                                            .get("_ingest.on_failure_message")
                                                            .map_or_else(
                                                                String::new,
                                                                painless_to_string
                                                            )
                                                    ),
                                                )?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Egress.Destination.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Egress.Destination.PortRanges") {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Egress.Destination.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.End") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.End")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.End"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event
                                                            .set("_ingest._value.end", converted)?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.append(
                                                    "error.message",
                                                    json!(
                                                        event
                                                            .get("_ingest.on_failure_message")
                                                            .map_or_else(
                                                                String::new,
                                                                painless_to_string
                                                            )
                                                    ),
                                                )?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Egress.Destination.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Egress.Destination.PortRanges") {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Egress.Destination.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            event.remove("_ingest._value.Begin");
                                            event.remove("_ingest._value.End");
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Egress.Destination.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Egress.Destination.PortRanges") {
                                event.rename(
                                    "_ingest._value.Egress.Destination.PortRanges",
                                    "_ingest._value.egress.destination.port_ranges",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Egress.Protocol") {
                                event.rename(
                                    "_ingest._value.Egress.Protocol",
                                    "_ingest._value.egress.protocol",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Egress.Source.Address") {
                                event.rename(
                                    "_ingest._value.Egress.Source.Address",
                                    "_ingest._value.egress.source.address",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Egress.Source.PortRanges") {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Egress.Source.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.Begin") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.Begin")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.Begin"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.begin",
                                                            converted,
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.append(
                                                    "error.message",
                                                    json!(
                                                        event
                                                            .get("_ingest.on_failure_message")
                                                            .map_or_else(
                                                                String::new,
                                                                painless_to_string
                                                            )
                                                    ),
                                                )?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Egress.Source.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Egress.Source.PortRanges") {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Egress.Source.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.End") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.End")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.End"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event
                                                            .set("_ingest._value.end", converted)?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.append(
                                                    "error.message",
                                                    json!(
                                                        event
                                                            .get("_ingest.on_failure_message")
                                                            .map_or_else(
                                                                String::new,
                                                                painless_to_string
                                                            )
                                                    ),
                                                )?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Egress.Source.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Egress.Source.PortRanges") {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Egress.Source.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            event.remove("_ingest._value.Begin");
                                            event.remove("_ingest._value.End");
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Egress.Source.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Egress.Source.PortRanges") {
                                event.rename(
                                    "_ingest._value.Egress.Source.PortRanges",
                                    "_ingest._value.egress.source.port_ranges",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Ingress.Destination.Address") {
                                event.rename(
                                    "_ingest._value.Ingress.Destination.Address",
                                    "_ingest._value.ingress.destination.address",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Ingress.Destination.PortRanges")
                                {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Ingress.Destination.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.Begin") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.Begin")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.Begin"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.begin",
                                                            converted,
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.append(
                                                    "error.message",
                                                    json!(
                                                        event
                                                            .get("_ingest.on_failure_message")
                                                            .map_or_else(
                                                                String::new,
                                                                painless_to_string
                                                            )
                                                    ),
                                                )?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Ingress.Destination.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Ingress.Destination.PortRanges")
                                {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Ingress.Destination.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.End") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.End")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.End"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event
                                                            .set("_ingest._value.end", converted)?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.append(
                                                    "error.message",
                                                    json!(
                                                        event
                                                            .get("_ingest.on_failure_message")
                                                            .map_or_else(
                                                                String::new,
                                                                painless_to_string
                                                            )
                                                    ),
                                                )?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Ingress.Destination.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Ingress.Destination.PortRanges")
                                {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Ingress.Destination.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            event.remove("_ingest._value.Begin");
                                            event.remove("_ingest._value.End");
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Ingress.Destination.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Ingress.Destination.PortRanges") {
                                event.rename(
                                    "_ingest._value.Ingress.Destination.PortRanges",
                                    "_ingest._value.ingress.destination.port_ranges",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Ingress.Protocol") {
                                event.rename(
                                    "_ingest._value.Ingress.Protocol",
                                    "_ingest._value.ingress.protocol",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Ingress.Source.Address") {
                                event.rename(
                                    "_ingest._value.Ingress.Source.Address",
                                    "_ingest._value.ingress.source.address",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Ingress.Source.PortRanges") {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Ingress.Source.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.Begin") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.Begin")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.Begin"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.begin",
                                                            converted,
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Ingress.Source.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Ingress.Source.PortRanges") {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Ingress.Source.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.End") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.End")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.End"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event
                                                            .set("_ingest._value.end", converted)?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.append(
                                                    "error.message",
                                                    json!(
                                                        event
                                                            .get("_ingest.on_failure_message")
                                                            .map_or_else(
                                                                String::new,
                                                                painless_to_string
                                                            )
                                                    ),
                                                )?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Ingress.Source.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Ingress.Source.PortRanges") {
                                    if let Some(Value::Array(items)) = event
                                        .get("_ingest._value.Ingress.Source.PortRanges")
                                        .cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            event.remove("_ingest._value.Begin");
                                            event.remove("_ingest._value.End");
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.Ingress.Source.PortRanges",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.NetworkPath")
                    && event.get("json.NetworkPath").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.NetworkPath").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Ingress.Source.PortRanges") {
                                event.rename(
                                    "_ingest._value.Ingress.Source.PortRanges",
                                    "_ingest._value.ingress.source.port_ranges",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.NetworkPath", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            if event.has("json.NetworkPath") {
                event.rename(
                    "json.NetworkPath",
                    "aws.securityhub_findings_full_posture.network_path",
                )?;
            }

            if event.has("json.Note.Text") {
                event.rename(
                    "json.Note.Text",
                    "aws.securityhub_findings_full_posture.note.text",
                )?;
            }

            let _cond = {
                event.has_value("json.Note.UpdatedAt")
                    && event.get_str("json.Note.UpdatedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Note.UpdatedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.securityhub_findings_full_posture.note.updated_at",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_Note_UpdatedAt_to_aws_securityhub_findings_full_posture_note_updated_at_5bf9f1f6")?;
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

            if event.has("json.Note.UpdatedBy") {
                event.rename(
                    "json.Note.UpdatedBy",
                    "aws.securityhub_findings_full_posture.note.updated_by",
                )?;
            }

            let _cond = { event.get_str("json.PatchSummary.FailedCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.PatchSummary.FailedCount") {
                        if let Some(val) = event.get("json.PatchSummary.FailedCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.PatchSummary.FailedCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.patch_summary.failed.count",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_PatchSummary_FailedCount_to_aws_securityhub_findings_full_posture_patch_summary_failed_count_07cd176f")?;
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

            if event.has("json.PatchSummary.Id") {
                event.rename(
                    "json.PatchSummary.Id",
                    "aws.securityhub_findings_full_posture.patch_summary.id",
                )?;
            }

            let _cond = { event.get_str("json.PatchSummary.InstalledCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.PatchSummary.InstalledCount") {
                        if let Some(val) = event.get("json.PatchSummary.InstalledCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.PatchSummary.InstalledCount".into(),
                                    message,
                                }
                            })?;
                            event.set("aws.securityhub_findings_full_posture.patch_summary.installed.count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_PatchSummary_InstalledCount_to_aws_securityhub_findings_full_posture_patch_summary_installed_count_cb596cd0")?;
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

            let _cond = { event.get_str("json.PatchSummary.InstalledOtherCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.PatchSummary.InstalledOtherCount") {
                        if let Some(val) = event.get("json.PatchSummary.InstalledOtherCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.PatchSummary.InstalledOtherCount".into(),
                                    message,
                                }
                            })?;
                            event.set("aws.securityhub_findings_full_posture.patch_summary.installed.other.count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_PatchSummary_InstalledOtherCount_to_aws_securityhub_findings_full_posture_patch_summary_installed_other_count_bd4f44f2")?;
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

            let _cond = { event.get_str("json.PatchSummary.InstalledPendingReboot") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.PatchSummary.InstalledPendingReboot") {
                        if let Some(val) = event.get("json.PatchSummary.InstalledPendingReboot") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.PatchSummary.InstalledPendingReboot".into(),
                                    message,
                                }
                            })?;
                            event.set("aws.securityhub_findings_full_posture.patch_summary.installed.pending_reboot", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_PatchSummary_InstalledPendingReboot_to_aws_securityhub_findings_full_posture_patch_summary_installed_pending_reboot_be8a3cb0")?;
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

            let _cond = { event.get_str("json.PatchSummary.InstalledRejectedCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.PatchSummary.InstalledRejectedCount") {
                        if let Some(val) = event.get("json.PatchSummary.InstalledRejectedCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.PatchSummary.InstalledRejectedCount".into(),
                                    message,
                                }
                            })?;
                            event.set("aws.securityhub_findings_full_posture.patch_summary.installed.rejected.count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_PatchSummary_InstalledRejectedCount_to_aws_securityhub_findings_full_posture_patch_summary_installed_rejected_count_a5036996")?;
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

            let _cond = { event.get_str("json.PatchSummary.MissingCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.PatchSummary.MissingCount") {
                        if let Some(val) = event.get("json.PatchSummary.MissingCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.PatchSummary.MissingCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.patch_summary.missing.count",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_PatchSummary_MissingCount_to_aws_securityhub_findings_full_posture_patch_summary_missing_count_829654ca")?;
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

            if event.has("json.PatchSummary.Operation") {
                event.rename(
                    "json.PatchSummary.Operation",
                    "aws.securityhub_findings_full_posture.patch_summary.operation.type",
                )?;
            }

            let _cond = {
                event.has_value("json.PatchSummary.OperationEndTime")
                    && event.get_str("json.PatchSummary.OperationEndTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.PatchSummary.OperationEndTime")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.securityhub_findings_full_posture.patch_summary.operation.end_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_PatchSummary_OperationEndTime_to_aws_securityhub_findings_full_posture_patch_summary_operation_end_time_136a9737")?;
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

            let _cond = {
                event.has_value("json.PatchSummary.OperationStartTime")
                    && event.get_str("json.PatchSummary.OperationStartTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.PatchSummary.OperationStartTime")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.securityhub_findings_full_posture.patch_summary.operation.start_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_PatchSummary_OperationStartTime_to_aws_securityhub_findings_full_posture_patch_summary_operation_start_time_22b85da1")?;
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

            if event.has("json.PatchSummary.RebootOption") {
                event.rename(
                    "json.PatchSummary.RebootOption",
                    "aws.securityhub_findings_full_posture.patch_summary.reboot_option",
                )?;
            }

            let _cond = {
                event.has_value("json.Process.LaunchedAt")
                    && event.get_str("json.Process.LaunchedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Process.LaunchedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.securityhub_findings_full_posture.process.launched_at",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_Process_LaunchedAt_to_aws_securityhub_findings_full_posture_process_launched_at_b6ab758a")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.process.launched_at")
                    .cloned()
                {
                    event.set("process.start", v)?;
                }
                Ok(())
            })();

            if event.has("json.Process.Name") {
                event.rename(
                    "json.Process.Name",
                    "aws.securityhub_findings_full_posture.process.name",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.process.name")
                    .cloned()
                {
                    event.set("process.name", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.Process.ParentPid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Process.ParentPid") {
                        if let Some(val) = event.get("json.Process.ParentPid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Process.ParentPid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.process.parent.pid",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Process_ParentPid_to_aws_securityhub_findings_full_posture_process_parent_pid_40ab8c66")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.process.parent.pid")
                    .cloned()
                {
                    event.set("process.parent.pid", v)?;
                }
                Ok(())
            })();

            if event.has("json.Process.Path") {
                event.rename(
                    "json.Process.Path",
                    "aws.securityhub_findings_full_posture.process.path",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.process.path")
                    .cloned()
                {
                    event.set("process.executable", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.Process.Pid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Process.Pid") {
                        if let Some(val) = event.get("json.Process.Pid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Process.Pid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.process.pid",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Process_Pid_to_aws_securityhub_findings_full_posture_process_pid_0fe12de2")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.process.pid")
                    .cloned()
                {
                    event.set("process.pid", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("json.Process.TerminatedAt")
                    && event.get_str("json.Process.TerminatedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Process.TerminatedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.securityhub_findings_full_posture.process.terminated_at",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_Process_TerminatedAt_to_aws_securityhub_findings_full_posture_process_terminated_at_865bd43c")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("aws.securityhub_findings_full_posture.process.terminated_at")
                    .cloned()
                {
                    event.set("process.end", v)?;
                }
                Ok(())
            })();

            if event.has("json.ProductArn") {
                event.rename(
                    "json.ProductArn",
                    "aws.securityhub_findings_full_posture.product.arn",
                )?;
            }

            if event.has("json.ProductFields") {
                event.rename(
                    "json.ProductFields",
                    "aws.securityhub_findings_full_posture.product.fields",
                )?;
            }

            if event.has("json.ProductName") {
                event.rename(
                    "json.ProductName",
                    "aws.securityhub_findings_full_posture.product.name",
                )?;
            }

            if event.has("json.RecordState") {
                event.rename(
                    "json.RecordState",
                    "aws.securityhub_findings_full_posture.record_state",
                )?;
            }

            if event.has("json.Region") {
                event.rename(
                    "json.Region",
                    "aws.securityhub_findings_full_posture.region",
                )?;
            }

            if let Some(v) = event
                .get("aws.securityhub_findings_full_posture.region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            let _cond = {
                event.has_value("json.RelatedFindings")
                    && event
                        .get("json.RelatedFindings")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.RelatedFindings").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Id") {
                                event.rename("_ingest._value.Id", "_ingest._value.id")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.RelatedFindings", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.RelatedFindings")
                    && event
                        .get("json.RelatedFindings")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.RelatedFindings").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.ProductArn") {
                                event.rename(
                                    "_ingest._value.ProductArn",
                                    "_ingest._value.product.arn",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.RelatedFindings", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            if event.has("json.RelatedFindings") {
                event.rename(
                    "json.RelatedFindings",
                    "aws.securityhub_findings_full_posture.related_findings",
                )?;
            }

            if event.has("json.Remediation.Recommendation.Text") {
                event.rename(
                    "json.Remediation.Recommendation.Text",
                    "aws.securityhub_findings_full_posture.remediation.recommendation.text",
                )?;
            }

            if event.has("json.Remediation.Recommendation.Url") {
                event.rename(
                    "json.Remediation.Recommendation.Url",
                    "aws.securityhub_findings_full_posture.remediation.recommendation.url",
                )?;
            }

            if let Some(v) = event
                .get("aws.securityhub_findings_full_posture.remediation.recommendation.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.reference", v)?;
            }

            let _cond = {
                event.has_value(
                    "aws.securityhub_findings_full_posture.remediation.recommendation.url",
                ) && event.has_value(
                    "aws.securityhub_findings_full_posture.remediation.recommendation.text",
                )
            };
            if _cond {
                let v = json!(format!(
                    "{}\r\n{}",
                    event
                        .get(
                            "aws.securityhub_findings_full_posture.remediation.recommendation.text"
                        )
                        .map_or_else(String::new, painless_to_string),
                    event
                        .get("aws.securityhub_findings_full_posture.remediation.recommendation.url")
                        .map_or_else(String::new, painless_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("rule.remediation", v)?;
                }
            }

            if event.has("json.Resources") {
                event.rename(
                    "json.Resources",
                    "aws.securityhub_findings_full_posture.resources",
                )?;
            }

            let _cond = {
                event.get("aws.securityhub_findings_full_posture.resources").is_some_and(|v| v.is_array()) && event.get("aws.securityhub_findings_full_posture.resources").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: // Arrays won't work in general in current UI of Cloud Security Posture workflow. In AWS SecurityHub, a finding may contain multiple resources, but rarely.\n// When a finding has single-resource, we extract fields as single-value so that the Findings UI behaves as expected for almost all cases.\n// But in the rare multi-resource case, we extract fields into an array to not miss any affected resources for a finding. \n// This trade-off is okay as not many findings will be affected. When our UI natively supports multi-resources, the single-value resource extraction must be removed.\n\ndef resources = ctx.aws.securityhub_findings_full_posture.resources;\n\n// Define fields to be extracted. \nif (ctx.resource == null) {\n  ctx.resource = new HashMap();\n}\nif (ctx.user == null) {\n  ctx.user = new HashMap();\n}\nif (ctx.host == null) {\n  ctx.host = new HashMap();\n}\nif (ctx.host.ip == null) {\n  ctx.host.ip = new ArrayList();\n}\nif (ctx.orchestrator == null) {\n  ctx.orchestrator = new HashMap();\n}\nif (ctx.orchestrator.cluster == null) {\n  ctx.orchestrator.cluster = new HashMap();\n}\nif (ctx.orchestrator.resource == null) {\n  ctx.orchestrator.resource = new HashMap();\n}\nif (ctx.cloud == null) {\n  ctx.cloud = new HashMap();\n}\nif (ctx.cloud.instance == null) {\n  ctx.cloud.instance = new HashMap();\n}\nif (ctx.cloud.service == null) {\n  ctx.cloud.service = new HashMap();\n}\n\n// This extraction logic is only for single resource case. Multiple resources are extracted inside script - script_extract_fields_from_multiple_resources.\nif (resources.size() == 1){\n  def res = resources[0];\n\n  // Extract resource field\n  ctx.resource.type = res.Type;\n  ctx.resource.id = res.Id;\n  def res_name;\n  String[] tokenList = res.Id.splitOnToken(\":\");\n  if (res.Details != null && res.Details[res.Type]?.Name != null) {\n    res_name = res.Details[res.Type].Name;\n  } else {\n    res_name = tokenList[tokenList.length - 1];\n  }\n  ctx.resource.name = res_name;\n\n  // Extract ECS fields from res.Details\n  if (res.Details != null) {\n    // Extract ECS user field from res.Details\n    if (res.Type == 'AwsIamUser' && res.Details.AwsIamUser?.UserName != null) {\n      ctx.user.name = res.Details.AwsIamUser.UserName;\n    }\n    if (res.Type == 'AwsIamAccessKey' && res.Details.AwsIamAccessKey?.UserName != null) {\n      ctx.user.name = res.Details.AwsIamAccessKey.UserName;\n    }\n    if (res.Type == 'AwsS3Bucket' && res.Details.AwsS3Bucket?.OwnerName != null) {\n      ctx.user.name = res.Details.AwsS3Bucket.OwnerName;\n    } \n    if (res.Type == 'AwsIamUser' && res.Details.AwsIamUser?.UserId != null) {\n      ctx.user.id = res.Details.AwsIamUser.UserId;\n    } \n    if (res.Type == 'AwsS3Bucket' && res.Details.AwsS3Bucket?.OwnerId != null) {\n      ctx.user.id = res.Details.AwsS3Bucket.OwnerId;\n    }\n\n    // Extract ECS host field from res.Details\n    if (res.Type == 'AwsEcsContainer' && res.Details.AwsEcsContainer?.Name != null) {\n      ctx.host.name = res.Details.AwsEcsContainer.Name;\n    }\n    if (res.Type == 'AwsEc2Instance') {\n      if (res.Details.AwsEc2Instance?.IpV4Addresses != null) {\n        for (def ipv4 : res.Details.AwsEc2Instance.IpV4Addresses) {\n          if (ipv4 instanceof String) {\n            ctx.host.ip.add(ipv4);\n          }\n        }\n      }\n      if (res.Details.AwsEc2Instance?.IpV6Addresses != null) {\n        for (def ipv6 : res.Details.AwsEc2Instance.IpV6Addresses) {\n          if (ipv6 instanceof String) {\n            ctx.host.ip.add(ipv6);\n          }\n        }\n      }\n    }\n\n    // Extract ECS orchestrator field from res.Details\n    if (['AwsEcsCluster', 'AwsEcsTask'].contains(res.Type) && res.Details.AwsEcsCluster?.ClusterArn != null) {\n      ctx.orchestrator.cluster.id = res.Details.AwsEcsCluster.ClusterArn;\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Arn != null) {\n      ctx.orchestrator.cluster.id = res.Details.AwsEksCluster.Arn;\n    }\n    if (res.Type == 'AwsEcsCluster' && res.Details.AwsEcsCluster?.ClusterName != null) {\n      ctx.orchestrator.cluster.name = res.Details.AwsEcsCluster.ClusterName;\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Name != null) {\n      ctx.orchestrator.cluster.name = res.Details.AwsEksCluster.Name;\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Version != null) {\n      ctx.orchestrator.cluster.version = res.Details.AwsEksCluster.Version;\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Endpoint != null) {\n      ctx.orchestrator.cluster.url = res.Details.AwsEksCluster.Endpoint;\n    }\n\n    // Extract ECS cloud field from res.Details\n    if (['AwsEc2Subnet', 'AwsRedshiftCluster', 'AwsDmsReplicationInstance'].contains(res.Type) && res.Details[res.Type]?.AvailabilityZone != null) {\n      ctx.cloud.availability_zone = res.Details[res.Type].AvailabilityZone;\n    }\n    if ((['AwsEc2VpcEndpointService', 'AwsElbLoadBalancer', 'AwsRdsDbCluster'].contains(res.Type)) && res.Details[res.Type]?.AvailabilityZones != null) {\n      for (def az: res.Details[res.Type].AvailabilityZones){\n        ctx.cloud.availability_zone = az;\n      }\n    }\n    if (res.Type == 'AwsAutoScalingAutoScalingGroup' && res.Details.AwsAutoScalingAutoScalingGroup?.AvailabilityZones != null) {\n      for (def az: res.Details.AwsAutoScalingAutoScalingGroup.AvailabilityZones){\n        ctx.cloud.availability_zone = az.Value;\n      }\n    }\n    if (res.Type == 'AwsEc2LaunchTemplate' && res.Details.AwsEc2LaunchTemplate?.LaunchTemplateData?.Placement?.AvailabilityZone != null) {\n      ctx.cloud.availability_zone = res.Details.AwsEc2LaunchTemplate.LaunchTemplateData.Placement.AvailabilityZone;\n    }\n    if (res.Type == 'AwsElbv2LoadBalancer' && res.Details.AwsElbv2LoadBalancer?.AvailabilityZones != null) {\n      for (def az: res.Details.AwsElbv2LoadBalancer.AvailabilityZones){\n        ctx.cloud.availability_zone = az.ZoneName;\n      }\n    }\n  }\n\n  // Extract ECS host field not in res.Details\n  if (res.Type == 'AwsEc2Instance' && res.Id != null) {\n    ctx.host.id = res.Id;\n  }\n\n  // Extract ECS orchestrator field not in res.Details\n  if (res.Type.startsWith('AwsEks') || res.Type.startsWith('AwsEcs')) {\n    ctx.orchestrator.resource.id = res.Id;\n    ctx.orchestrator.resource.name = res_name;\n    ctx.orchestrator.resource.type = res.Type;\n    if (res.Type.startsWith('AwsEks')) {\n      ctx.orchestrator.type = 'kubernetes';\n    } else {\n      ctx.orchestrator.type = 'ecs';\n    }\n  }\n  \n  // Extract ECS cloud field not in res.Details\n  if (res.Type == 'AwsEc2Instance') {\n    ctx.cloud.instance.id = res.Id;\n    ctx.cloud.instance.name = res_name;\n  }\n  if (tokenList.length > 2) {\n    ctx.cloud.service.name = tokenList[2];\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// Arrays won't work in general in current UI of Cloud Security Posture workflow. In AWS SecurityHub, a finding may contain multiple resources, but rarely.\n// When a finding has single-resource, we extract fields as single-value so that the Findings UI behaves as expected for almost all cases.\n// But in the rare multi-resource case, we extract fields into an array to not miss any affected resources for a finding. \n// This trade-off is okay as not many findings will be affected. When our UI natively supports multi-resources, the single-value resource extraction must be removed.\n\ndef resources = ctx.aws.securityhub_findings_full_posture.resources;\n\n// Define fields to be extracted. \nif (ctx.resource == null) {\n  ctx.resource = new HashMap();\n}\nif (ctx.user == null) {\n  ctx.user = new HashMap();\n}\nif (ctx.host == null) {\n  ctx.host = new HashMap();\n}\nif (ctx.host.ip == null) {\n  ctx.host.ip = new ArrayList();\n}\nif (ctx.orchestrator == null) {\n  ctx.orchestrator = new HashMap();\n}\nif (ctx.orchestrator.cluster == null) {\n  ctx.orchestrator.cluster = new HashMap();\n}\nif (ctx.orchestrator.resource == null) {\n  ctx.orchestrator.resource = new HashMap();\n}\nif (ctx.cloud == null) {\n  ctx.cloud = new HashMap();\n}\nif (ctx.cloud.instance == null) {\n  ctx.cloud.instance = new HashMap();\n}\nif (ctx.cloud.service == null) {\n  ctx.cloud.service = new HashMap();\n}\n\n// This extraction logic is only for single resource case. Multiple resources are extracted inside script - script_extract_fields_from_multiple_resources.\nif (resources.size() == 1){\n  def res = resources[0];\n\n  // Extract resource field\n  ctx.resource.type = res.Type;\n  ctx.resource.id = res.Id;\n  def res_name;\n  String[] tokenList = res.Id.splitOnToken(\":\");\n  if (res.Details != null && res.Details[res.Type]?.Name != null) {\n    res_name = res.Details[res.Type].Name;\n  } else {\n    res_name = tokenList[tokenList.length - 1];\n  }\n  ctx.resource.name = res_name;\n\n  // Extract ECS fields from res.Details\n  if (res.Details != null) {\n    // Extract ECS user field from res.Details\n    if (res.Type == 'AwsIamUser' && res.Details.AwsIamUser?.UserName != null) {\n      ctx.user.name = res.Details.AwsIamUser.UserName;\n    }\n    if (res.Type == 'AwsIamAccessKey' && res.Details.AwsIamAccessKey?.UserName != null) {\n      ctx.user.name = res.Details.AwsIamAccessKey.UserName;\n    }\n    if (res.Type == 'AwsS3Bucket' && res.Details.AwsS3Bucket?.OwnerName != null) {\n      ctx.user.name = res.Details.AwsS3Bucket.OwnerName;\n    } \n    if (res.Type == 'AwsIamUser' && res.Details.AwsIamUser?.UserId != null) {\n      ctx.user.id = res.Details.AwsIamUser.UserId;\n    } \n    if (res.Type == 'AwsS3Bucket' && res.Details.AwsS3Bucket?.OwnerId != null) {\n      ctx.user.id = res.Details.AwsS3Bucket.OwnerId;\n    }\n\n    // Extract ECS host field from res.Details\n    if (res.Type == 'AwsEcsContainer' && res.Details.AwsEcsContainer?.Name != null) {\n      ctx.host.name = res.Details.AwsEcsContainer.Name;\n    }\n    if (res.Type == 'AwsEc2Instance') {\n      if (res.Details.AwsEc2Instance?.IpV4Addresses != null) {\n        for (def ipv4 : res.Details.AwsEc2Instance.IpV4Addresses) {\n          if (ipv4 instanceof String) {\n            ctx.host.ip.add(ipv4);\n          }\n        }\n      }\n      if (res.Details.AwsEc2Instance?.IpV6Addresses != null) {\n        for (def ipv6 : res.Details.AwsEc2Instance.IpV6Addresses) {\n          if (ipv6 instanceof String) {\n            ctx.host.ip.add(ipv6);\n          }\n        }\n      }\n    }\n\n    // Extract ECS orchestrator field from res.Details\n    if (['AwsEcsCluster', 'AwsEcsTask'].contains(res.Type) && res.Details.AwsEcsCluster?.ClusterArn != null) {\n      ctx.orchestrator.cluster.id = res.Details.AwsEcsCluster.ClusterArn;\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Arn != null) {\n      ctx.orchestrator.cluster.id = res.Details.AwsEksCluster.Arn;\n    }\n    if (res.Type == 'AwsEcsCluster' && res.Details.AwsEcsCluster?.ClusterName != null) {\n      ctx.orchestrator.cluster.name = res.Details.AwsEcsCluster.ClusterName;\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Name != null) {\n      ctx.orchestrator.cluster.name = res.Details.AwsEksCluster.Name;\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Version != null) {\n      ctx.orchestrator.cluster.version = res.Details.AwsEksCluster.Version;\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Endpoint != null) {\n      ctx.orchestrator.cluster.url = res.Details.AwsEksCluster.Endpoint;\n    }\n\n    // Extract ECS cloud field from res.Details\n    if (['AwsEc2Subnet', 'AwsRedshiftCluster', 'AwsDmsReplicationInstance'].contains(res.Type) && res.Details[res.Type]?.AvailabilityZone != null) {\n      ctx.cloud.availability_zone = res.Details[res.Type].AvailabilityZone;\n    }\n    if ((['AwsEc2VpcEndpointService', 'AwsElbLoadBalancer', 'AwsRdsDbCluster'].contains(res.Type)) && res.Details[res.Type]?.AvailabilityZones != null) {\n      for (def az: res.Details[res.Type].AvailabilityZones){\n        ctx.cloud.availability_zone = az;\n      }\n    }\n    if (res.Type == 'AwsAutoScalingAutoScalingGroup' && res.Details.AwsAutoScalingAutoScalingGroup?.AvailabilityZones != null) {\n      for (def az: res.Details.AwsAutoScalingAutoScalingGroup.AvailabilityZones){\n        ctx.cloud.availability_zone = az.Value;\n      }\n    }\n    if (res.Type == 'AwsEc2LaunchTemplate' && res.Details.AwsEc2LaunchTemplate?.LaunchTemplateData?.Placement?.AvailabilityZone != null) {\n      ctx.cloud.availability_zone = res.Details.AwsEc2LaunchTemplate.LaunchTemplateData.Placement.AvailabilityZone;\n    }\n    if (res.Type == 'AwsElbv2LoadBalancer' && res.Details.AwsElbv2LoadBalancer?.AvailabilityZones != null) {\n      for (def az: res.Details.AwsElbv2LoadBalancer.AvailabilityZones){\n        ctx.cloud.availability_zone = az.ZoneName;\n      }\n    }\n  }\n\n  // Extract ECS host field not in res.Details\n  if (res.Type == 'AwsEc2Instance' && res.Id != null) {\n    ctx.host.id = res.Id;\n  }\n\n  // Extract ECS orchestrator field not in res.Details\n  if (res.Type.startsWith('AwsEks') || res.Type.startsWith('AwsEcs')) {\n    ctx.orchestrator.resource.id = res.Id;\n    ctx.orchestrator.resource.name = res_name;\n    ctx.orchestrator.resource.type = res.Type;\n    if (res.Type.startsWith('AwsEks')) {\n      ctx.orchestrator.type = 'kubernetes';\n    } else {\n      ctx.orchestrator.type = 'ecs';\n    }\n  }\n  \n  // Extract ECS cloud field not in res.Details\n  if (res.Type == 'AwsEc2Instance') {\n    ctx.cloud.instance.id = res.Id;\n    ctx.cloud.instance.name = res_name;\n  }\n  if (tokenList.length > 2) {\n    ctx.cloud.service.name = tokenList[2];\n  }\n}"#
                    ),
                )?;
            }

            let _cond = {
                event.get("aws.securityhub_findings_full_posture.resources").is_some_and(|v| v.is_array()) && event.get("aws.securityhub_findings_full_posture.resources").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 1)
            };
            if _cond {
                // Painless script
                // Source: def resources = ctx.aws.securityhub_findings_full_posture.resources;\n\n// Define fields to be extracted. \nif (ctx.resource.type == null) {\n  ctx.resource.type = new ArrayList();\n}\nif (ctx.resource.id == null) {\n  ctx.resource.id = new ArrayList();\n}\nif (ctx.resource.name == null) {\n  ctx.resource.name = new ArrayList();\n}\n\nif (ctx.user.name == null) {\n  ctx.user.name = new ArrayList();\n}\nif (ctx.user.id == null) {\n  ctx.user.id = new ArrayList();\n}\n\nif (ctx.host.id == null) {\n  ctx.host.id = new ArrayList();\n}\nif (ctx.host.ip == null) {\n  ctx.host.ip = new ArrayList();\n}\nif (ctx.host.name == null) {\n  ctx.host.name = new ArrayList();\n}\n\nif (ctx.orchestrator.type == null) {\n  ctx.orchestrator.type = new ArrayList();\n}\nif (ctx.orchestrator.cluster.id == null) {\n  ctx.orchestrator.cluster.id = new ArrayList();\n}\nif (ctx.orchestrator.cluster.name == null) {\n  ctx.orchestrator.cluster.name = new ArrayList();\n}\nif (ctx.orchestrator.cluster.version == null) {\n  ctx.orchestrator.cluster.version = new ArrayList();\n}\nif (ctx.orchestrator.resource.id == null) {\n  ctx.orchestrator.resource.id = new ArrayList();\n}\nif (ctx.orchestrator.resource.name == null) {\n  ctx.orchestrator.resource.name = new ArrayList();\n}\nif (ctx.orchestrator.resource.type == null) {\n  ctx.orchestrator.resource.type = new ArrayList();\n}\n\nif (ctx.cloud.instance.id == null) {\n  ctx.cloud.instance.id = new ArrayList();\n}\nif (ctx.cloud.instance.name == null) {\n  ctx.cloud.instance.name = new ArrayList();\n}\nif (ctx.cloud.service.name == null) {\n  ctx.cloud.service.name = new ArrayList();\n}\nif (ctx.cloud.availability_zone == null) {\n  ctx.cloud.availability_zone = new ArrayList();\n}\n\nfor (res in resources) {\n  // Extract resource field\n  ctx.resource.type.add(res.Type);\n  ctx.resource.id.add(res.Id);\n  def res_name;\n  String[] tokenList = res.Id.splitOnToken(\":\");\n  if (res.Details != null && res.Details[res.Type]?.Name != null) {\n    res_name = res.Details[res.Type].Name;\n  } else {\n    res_name = tokenList[tokenList.length - 1];\n  }\n  ctx.resource.name.add(res_name);\n\n  // Extract ECS fields from res.Details\n  if (res.Details != null) {\n    // Extract ECS user field from res.Details\n    if (res.Type == 'AwsIamUser' && res.Details.AwsIamUser?.UserName != null) {\n      ctx.user.name.add(res.Details.AwsIamUser.UserName);\n    }\n    if (res.Type == 'AwsIamAccessKey' && res.Details.AwsIamAccessKey?.UserName != null) {\n      ctx.user.name.add(res.Details.AwsIamAccessKey.UserName);\n    }\n    if (res.Type == 'AwsS3Bucket' && res.Details.AwsS3Bucket?.OwnerName != null) {\n      ctx.user.name.add(res.Details.AwsS3Bucket.OwnerName);\n    } \n    if (res.Type == 'AwsIamUser' && res.Details.AwsIamUser?.UserId != null) {\n      ctx.user.id.add(res.Details.AwsIamUser.UserId);\n    } \n    if (res.Type == 'AwsS3Bucket' && res.Details.AwsS3Bucket?.OwnerId != null) {\n      ctx.user.id.add(res.Details.AwsS3Bucket.OwnerId);\n    }\n\n    // Extract ECS host field from res.Details\n    if (res.Type == 'AwsEcsContainer' && res.Details.AwsEcsContainer?.Name != null) {\n      ctx.host.name.add(res.Details.AwsEcsContainer.Name);\n    }\n    if (res.Type == 'AwsEc2Instance') {\n      if (res.Details.AwsEc2Instance?.IpV4Addresses != null) {\n        for (def ipv4 : res.Details.AwsEc2Instance.IpV4Addresses) {\n          if (ipv4 instanceof String) {\n            ctx.host.ip.add(ipv4);\n          }\n        }\n      }\n      if (res.Details.AwsEc2Instance?.IpV6Addresses != null) {\n        for (def ipv6 : res.Details.AwsEc2Instance.IpV6Addresses) {\n          if (ipv6 instanceof String) {\n            ctx.host.ip.add(ipv6);\n          }\n        }\n      }\n    }\n\n    // Extract ECS orchestrator field from res.Details\n    if (['AwsEcsCluster', 'AwsEcsTask'].contains(res.Type) && res.Details.AwsEcsCluster?.ClusterArn != null) {\n      ctx.orchestrator.cluster.id.add(res.Details.AwsEcsCluster.ClusterArn);\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Arn != null) {\n      ctx.orchestrator.cluster.id.add(res.Details.AwsEksCluster.Arn);\n    }\n    if (res.Type == 'AwsEcsCluster' && res.Details.AwsEcsCluster?.ClusterName != null) {\n      ctx.orchestrator.cluster.name.add(res.Details.AwsEcsCluster.ClusterName);\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Name != null) {\n      ctx.orchestrator.cluster.name.add(res.Details.AwsEksCluster.Name);\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Version != null) {\n      ctx.orchestrator.cluster.version.add(res.Details.AwsEksCluster.Version);\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Endpoint != null) {\n      ctx.orchestrator.cluster.url.add(res.Details.AwsEksCluster.Endpoint);\n    }\n\n    // Extract ECS cloud field from res.Details\n    if (['AwsEc2Subnet', 'AwsRedshiftCluster', 'AwsDmsReplicationInstance'].contains(res.Type) && res.Details[res.Type]?.AvailabilityZone != null) {\n      ctx.cloud.availability_zone.add(res.Details[res.Type].AvailabilityZone);\n    }\n    if ((['AwsEc2VpcEndpointService', 'AwsElbLoadBalancer', 'AwsRdsDbCluster'].contains(res.Type)) && res.Details[res.Type]?.AvailabilityZones != null) {\n      for (def az: res.Details[res.Type].AvailabilityZones){\n        ctx.cloud.availability_zone.add(az);\n      }\n    }\n    if (res.Type == 'AwsAutoScalingAutoScalingGroup' && res.Details.AwsAutoScalingAutoScalingGroup?.AvailabilityZones != null) {\n      for (def az: res.Details.AwsAutoScalingAutoScalingGroup.AvailabilityZones){\n        ctx.cloud.availability_zone.add(az.Value);\n      }\n    }\n    if (res.Type == 'AwsEc2LaunchTemplate' && res.Details.AwsEc2LaunchTemplate?.LaunchTemplateData?.Placement?.AvailabilityZone != null) {\n      ctx.cloud.availability_zone.add(res.Details.AwsEc2LaunchTemplate.LaunchTemplateData.Placement.AvailabilityZone);\n    }\n    if (res.Type == 'AwsElbv2LoadBalancer' && res.Details.AwsElbv2LoadBalancer?.AvailabilityZones != null) {\n      for (def az: res.Details.AwsElbv2LoadBalancer.AvailabilityZones){\n        ctx.cloud.availability_zone.add(az.ZoneName);\n      }\n    }\n  }\n\n  // Extract ECS host field not in res.Details\n  if (res.Type == 'AwsEc2Instance' && res.Id != null) {\n    ctx.host.id.add(res.Id);\n  }\n\n  // Extract ECS orchestrator field not in res.Details\n  if (res.Type.startsWith('AwsEks') || res.Type.startsWith('AwsEcs')) {\n    ctx.orchestrator.resource.id.add(res.Id);\n    ctx.orchestrator.resource.name.add(res_name);\n    ctx.orchestrator.resource.type.add(res.Type);\n    if (res.Type.startsWith('AwsEks')) {\n      ctx.orchestrator.type.add('kubernetes');\n    } else {\n      ctx.orchestrator.type.add('ecs');\n    }\n  }\n  \n  // Extract ECS cloud field not in res.Details\n  if (res.Type == 'AwsEc2Instance') {\n    ctx.cloud.instance.id.add(res.Id);\n    ctx.cloud.instance.name.add(res_name);\n  }\n  if (tokenList.length > 2) {\n    ctx.cloud.service.name.add(tokenList[2]);\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def resources = ctx.aws.securityhub_findings_full_posture.resources;\n\n// Define fields to be extracted. \nif (ctx.resource.type == null) {\n  ctx.resource.type = new ArrayList();\n}\nif (ctx.resource.id == null) {\n  ctx.resource.id = new ArrayList();\n}\nif (ctx.resource.name == null) {\n  ctx.resource.name = new ArrayList();\n}\n\nif (ctx.user.name == null) {\n  ctx.user.name = new ArrayList();\n}\nif (ctx.user.id == null) {\n  ctx.user.id = new ArrayList();\n}\n\nif (ctx.host.id == null) {\n  ctx.host.id = new ArrayList();\n}\nif (ctx.host.ip == null) {\n  ctx.host.ip = new ArrayList();\n}\nif (ctx.host.name == null) {\n  ctx.host.name = new ArrayList();\n}\n\nif (ctx.orchestrator.type == null) {\n  ctx.orchestrator.type = new ArrayList();\n}\nif (ctx.orchestrator.cluster.id == null) {\n  ctx.orchestrator.cluster.id = new ArrayList();\n}\nif (ctx.orchestrator.cluster.name == null) {\n  ctx.orchestrator.cluster.name = new ArrayList();\n}\nif (ctx.orchestrator.cluster.version == null) {\n  ctx.orchestrator.cluster.version = new ArrayList();\n}\nif (ctx.orchestrator.resource.id == null) {\n  ctx.orchestrator.resource.id = new ArrayList();\n}\nif (ctx.orchestrator.resource.name == null) {\n  ctx.orchestrator.resource.name = new ArrayList();\n}\nif (ctx.orchestrator.resource.type == null) {\n  ctx.orchestrator.resource.type = new ArrayList();\n}\n\nif (ctx.cloud.instance.id == null) {\n  ctx.cloud.instance.id = new ArrayList();\n}\nif (ctx.cloud.instance.name == null) {\n  ctx.cloud.instance.name = new ArrayList();\n}\nif (ctx.cloud.service.name == null) {\n  ctx.cloud.service.name = new ArrayList();\n}\nif (ctx.cloud.availability_zone == null) {\n  ctx.cloud.availability_zone = new ArrayList();\n}\n\nfor (res in resources) {\n  // Extract resource field\n  ctx.resource.type.add(res.Type);\n  ctx.resource.id.add(res.Id);\n  def res_name;\n  String[] tokenList = res.Id.splitOnToken(\":\");\n  if (res.Details != null && res.Details[res.Type]?.Name != null) {\n    res_name = res.Details[res.Type].Name;\n  } else {\n    res_name = tokenList[tokenList.length - 1];\n  }\n  ctx.resource.name.add(res_name);\n\n  // Extract ECS fields from res.Details\n  if (res.Details != null) {\n    // Extract ECS user field from res.Details\n    if (res.Type == 'AwsIamUser' && res.Details.AwsIamUser?.UserName != null) {\n      ctx.user.name.add(res.Details.AwsIamUser.UserName);\n    }\n    if (res.Type == 'AwsIamAccessKey' && res.Details.AwsIamAccessKey?.UserName != null) {\n      ctx.user.name.add(res.Details.AwsIamAccessKey.UserName);\n    }\n    if (res.Type == 'AwsS3Bucket' && res.Details.AwsS3Bucket?.OwnerName != null) {\n      ctx.user.name.add(res.Details.AwsS3Bucket.OwnerName);\n    } \n    if (res.Type == 'AwsIamUser' && res.Details.AwsIamUser?.UserId != null) {\n      ctx.user.id.add(res.Details.AwsIamUser.UserId);\n    } \n    if (res.Type == 'AwsS3Bucket' && res.Details.AwsS3Bucket?.OwnerId != null) {\n      ctx.user.id.add(res.Details.AwsS3Bucket.OwnerId);\n    }\n\n    // Extract ECS host field from res.Details\n    if (res.Type == 'AwsEcsContainer' && res.Details.AwsEcsContainer?.Name != null) {\n      ctx.host.name.add(res.Details.AwsEcsContainer.Name);\n    }\n    if (res.Type == 'AwsEc2Instance') {\n      if (res.Details.AwsEc2Instance?.IpV4Addresses != null) {\n        for (def ipv4 : res.Details.AwsEc2Instance.IpV4Addresses) {\n          if (ipv4 instanceof String) {\n            ctx.host.ip.add(ipv4);\n          }\n        }\n      }\n      if (res.Details.AwsEc2Instance?.IpV6Addresses != null) {\n        for (def ipv6 : res.Details.AwsEc2Instance.IpV6Addresses) {\n          if (ipv6 instanceof String) {\n            ctx.host.ip.add(ipv6);\n          }\n        }\n      }\n    }\n\n    // Extract ECS orchestrator field from res.Details\n    if (['AwsEcsCluster', 'AwsEcsTask'].contains(res.Type) && res.Details.AwsEcsCluster?.ClusterArn != null) {\n      ctx.orchestrator.cluster.id.add(res.Details.AwsEcsCluster.ClusterArn);\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Arn != null) {\n      ctx.orchestrator.cluster.id.add(res.Details.AwsEksCluster.Arn);\n    }\n    if (res.Type == 'AwsEcsCluster' && res.Details.AwsEcsCluster?.ClusterName != null) {\n      ctx.orchestrator.cluster.name.add(res.Details.AwsEcsCluster.ClusterName);\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Name != null) {\n      ctx.orchestrator.cluster.name.add(res.Details.AwsEksCluster.Name);\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Version != null) {\n      ctx.orchestrator.cluster.version.add(res.Details.AwsEksCluster.Version);\n    }\n    if (res.Type == 'AwsEksCluster' && res.Details.AwsEksCluster?.Endpoint != null) {\n      ctx.orchestrator.cluster.url.add(res.Details.AwsEksCluster.Endpoint);\n    }\n\n    // Extract ECS cloud field from res.Details\n    if (['AwsEc2Subnet', 'AwsRedshiftCluster', 'AwsDmsReplicationInstance'].contains(res.Type) && res.Details[res.Type]?.AvailabilityZone != null) {\n      ctx.cloud.availability_zone.add(res.Details[res.Type].AvailabilityZone);\n    }\n    if ((['AwsEc2VpcEndpointService', 'AwsElbLoadBalancer', 'AwsRdsDbCluster'].contains(res.Type)) && res.Details[res.Type]?.AvailabilityZones != null) {\n      for (def az: res.Details[res.Type].AvailabilityZones){\n        ctx.cloud.availability_zone.add(az);\n      }\n    }\n    if (res.Type == 'AwsAutoScalingAutoScalingGroup' && res.Details.AwsAutoScalingAutoScalingGroup?.AvailabilityZones != null) {\n      for (def az: res.Details.AwsAutoScalingAutoScalingGroup.AvailabilityZones){\n        ctx.cloud.availability_zone.add(az.Value);\n      }\n    }\n    if (res.Type == 'AwsEc2LaunchTemplate' && res.Details.AwsEc2LaunchTemplate?.LaunchTemplateData?.Placement?.AvailabilityZone != null) {\n      ctx.cloud.availability_zone.add(res.Details.AwsEc2LaunchTemplate.LaunchTemplateData.Placement.AvailabilityZone);\n    }\n    if (res.Type == 'AwsElbv2LoadBalancer' && res.Details.AwsElbv2LoadBalancer?.AvailabilityZones != null) {\n      for (def az: res.Details.AwsElbv2LoadBalancer.AvailabilityZones){\n        ctx.cloud.availability_zone.add(az.ZoneName);\n      }\n    }\n  }\n\n  // Extract ECS host field not in res.Details\n  if (res.Type == 'AwsEc2Instance' && res.Id != null) {\n    ctx.host.id.add(res.Id);\n  }\n\n  // Extract ECS orchestrator field not in res.Details\n  if (res.Type.startsWith('AwsEks') || res.Type.startsWith('AwsEcs')) {\n    ctx.orchestrator.resource.id.add(res.Id);\n    ctx.orchestrator.resource.name.add(res_name);\n    ctx.orchestrator.resource.type.add(res.Type);\n    if (res.Type.startsWith('AwsEks')) {\n      ctx.orchestrator.type.add('kubernetes');\n    } else {\n      ctx.orchestrator.type.add('ecs');\n    }\n  }\n  \n  // Extract ECS cloud field not in res.Details\n  if (res.Type == 'AwsEc2Instance') {\n    ctx.cloud.instance.id.add(res.Id);\n    ctx.cloud.instance.name.add(res_name);\n  }\n  if (tokenList.length > 2) {\n    ctx.cloud.service.name.add(tokenList[2]);\n  }\n}"#
                    ),
                )?;
            }

            let _cond = { event.get_str("json.Sample") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Sample") {
                        if let Some(val) = event.get("json.Sample") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Sample".into(),
                                    message,
                                }
                            })?;
                            event.set("aws.securityhub_findings_full_posture.sample", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Sample_to_aws_securityhub_findings_full_posture_sample_426a9f05")?;
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

            if event.has("json.SchemaVersion") {
                event.rename(
                    "json.SchemaVersion",
                    "aws.securityhub_findings_full_posture.schema.version",
                )?;
            }

            if event.has("json.Severity.Label") {
                event.rename(
                    "json.Severity.Label",
                    "aws.securityhub_findings_full_posture.severity.label",
                )?;
            }

            let _cond = { event.get_str("json.Severity.Normalized") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Severity.Normalized") {
                        if let Some(val) = event.get("json.Severity.Normalized") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Severity.Normalized".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.severity.normalized",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Severity_Normalized_to_aws_securityhub_findings_full_posture_severity_normalized_f525eba9")?;
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

            let _cond =
                { event.has_value("aws.securityhub_findings_full_posture.severity.normalized") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("aws.securityhub_findings_full_posture.severity.normalized")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "aws.securityhub_findings_full_posture.severity.normalized"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set("event.severity", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_severity_normalized",
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
            }

            if event.has("json.Severity.Original") {
                event.rename(
                    "json.Severity.Original",
                    "aws.securityhub_findings_full_posture.severity.original",
                )?;
            }

            let _cond = { event.get_str("json.Severity.Product") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.Severity.Product") {
                        if let Some(val) = event.get("json.Severity.Product") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.Severity.Product".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "aws.securityhub_findings_full_posture.severity.product",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_Severity_Product_to_aws_securityhub_findings_full_posture_severity_product_a54cd555")?;
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

            if event.has("json.SourceUrl") {
                event.rename(
                    "json.SourceUrl",
                    "aws.securityhub_findings_full_posture.source_url",
                )?;
            }

            let _cond = {
                event.get_str("aws.securityhub_findings_full_posture.source_url") != Some("")
                    && event.has_value("aws.securityhub_findings_full_posture.source_url")
            };
            if _cond {
                uri_parts(
                    event,
                    "aws.securityhub_findings_full_posture.source_url",
                    "url",
                    true,
                    false,
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "url.full",
                    json!(
                        event
                            .get("url.original")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                Ok(())
            })();

            let _cond = {
                event.has_value("json.ThreatIntelIndicators")
                    && event
                        .get("json.ThreatIntelIndicators")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) =
                        event.get("json.ThreatIntelIndicators").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Category") {
                                event
                                    .rename("_ingest._value.Category", "_ingest._value.category")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.ThreatIntelIndicators", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.ThreatIntelIndicators")
                    && event
                        .get("json.ThreatIntelIndicators")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) =
                        event.get("json.ThreatIntelIndicators").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.LastObservedAt")
                                {
                                    if let Some(parsed) = parse_date_out(
                                        &date_str,
                                        &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                                        None,
                                        None,
                                    ) {
                                        event.set("_ingest._value.last_observed_at", parsed)?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.ThreatIntelIndicators", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.ThreatIntelIndicators")
                    && event
                        .get("json.ThreatIntelIndicators")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) =
                        event.get("json.ThreatIntelIndicators").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            event.remove("_ingest._value.LastObservedAt");
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.ThreatIntelIndicators", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.ThreatIntelIndicators")
                    && event
                        .get("json.ThreatIntelIndicators")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) =
                        event.get("json.ThreatIntelIndicators").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Source") {
                                event.rename("_ingest._value.Source", "_ingest._value.source")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.ThreatIntelIndicators", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.ThreatIntelIndicators")
                    && event
                        .get("json.ThreatIntelIndicators")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) =
                        event.get("json.ThreatIntelIndicators").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.SourceUrl") {
                                event.rename(
                                    "_ingest._value.SourceUrl",
                                    "_ingest._value.source_url",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.ThreatIntelIndicators", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.ThreatIntelIndicators")
                    && event
                        .get("json.ThreatIntelIndicators")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) =
                        event.get("json.ThreatIntelIndicators").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Value") {
                                event.rename("_ingest._value.Value", "_ingest._value.value")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.ThreatIntelIndicators", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.ThreatIntelIndicators")
                    && event
                        .get("json.ThreatIntelIndicators")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) =
                        event.get("json.ThreatIntelIndicators").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Type") {
                                event.rename("_ingest._value.Type", "_ingest._value.type")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.ThreatIntelIndicators", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.ThreatIntelIndicators")
                    && event
                        .get("json.ThreatIntelIndicators")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) =
                        event.get("json.ThreatIntelIndicators").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(v) =
                                    event.get("_ingest._value.last_observed_at").cloned()
                                {
                                    event.set("threat.indicator.last_seen", v)?;
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.ThreatIntelIndicators", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.ThreatIntelIndicators")
                    && event
                        .get("json.ThreatIntelIndicators")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: for (ti in ctx.json.ThreatIntelIndicators) { \n  def type = ti.type;\n  if (params.containsKey(type)) {\n    def mapped_type = params.get(type);\n    ctx.threat.indicator.type = mapped_type;\n\n    if (mapped_type == 'file') {\n      def hash_name = type.splitOnToken(\"_\")[1].toLowerCase();\n      def hash_value = ti.value;\n\n      Map hash = new HashMap();\n      hash.put(hash_name,hash_value);\n      Map file = new HashMap();\n      file.put(\"hash\",hash);\n      Map indicator = new HashMap();\n      indicator.indicator = new HashMap();\n      indicator.indicator.put(\"file\", file);\n      \n      if (ctx.threat.enrichments == null) {\n        ctx.threat.enrichments = new ArrayList();\n      }\n\n      ctx.threat.enrichments.add(indicator);\n    }\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"for (ti in ctx.json.ThreatIntelIndicators) { \n  def type = ti.type;\n  if (params.containsKey(type)) {\n    def mapped_type = params.get(type);\n    ctx.threat.indicator.type = mapped_type;\n\n    if (mapped_type == 'file') {\n      def hash_name = type.splitOnToken(\"_\")[1].toLowerCase();\n      def hash_value = ti.value;\n\n      Map hash = new HashMap();\n      hash.put(hash_name,hash_value);\n      Map file = new HashMap();\n      file.put(\"hash\",hash);\n      Map indicator = new HashMap();\n      indicator.indicator = new HashMap();\n      indicator.indicator.put(\"file\", file);\n      \n      if (ctx.threat.enrichments == null) {\n        ctx.threat.enrichments = new ArrayList();\n      }\n\n      ctx.threat.enrichments.add(indicator);\n    }\n  }\n} "#
                    ),
                    cached_params!(
                        "{\"DOMAIN\":\"domain-name\",\"EMAIL_ADDRESS\":\"email-addr\",\"HASH_MD5\":\"file\",\"HASH_SHA1\":\"file\",\"HASH_SHA256\":\"file\",\"HASH_SHA512\":\"file\",\"IPV4_ADDRESS\":\"ipv4-addr\",\"IPV6_ADDRESS\":\"ipv6-addr\",\"MUTEX\":\"mutex\",\"PROCESS\":\"process\",\"URL\":\"url\"}"
                    ),
                )?;
            }

            if event.has("json.ThreatIntelIndicators") {
                event.rename(
                    "json.ThreatIntelIndicators",
                    "aws.securityhub_findings_full_posture.threat_intel_indicators",
                )?;
            }

            if event.has("json.Title") {
                event.rename("json.Title", "aws.securityhub_findings_full_posture.title")?;
            }

            if let Some(v) = event
                .get("aws.securityhub_findings_full_posture.title")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has("json.Types") {
                event.rename("json.Types", "aws.securityhub_findings_full_posture.types")?;
            }

            if event.has("json.UserDefinedFields") {
                event.rename(
                    "json.UserDefinedFields",
                    "aws.securityhub_findings_full_posture.user_defined_fields",
                )?;
            }

            if event.has("json.VerificationState") {
                event.rename(
                    "json.VerificationState",
                    "aws.securityhub_findings_full_posture.verification_state",
                )?;
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Cvss") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.Cvss").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.Adjustments") {
                                                    if let Some(Value::Array(items)) = event
                                                        .get("_ingest._value.Adjustments")
                                                        .cloned()
                                                    {
                                                        let mut out =
                                                            Vec::with_capacity(items.len());
                                                        for item in items {
                                                            event.set("_ingest._value", item)?;
                                                            if event.has("_ingest._value.Metric") {
                                                                event.rename(
                                                                    "_ingest._value.Metric",
                                                                    "_ingest._value.metric",
                                                                )?;
                                                            }
                                                            out.push(
                                                                event
                                                                    .remove("_ingest._value")
                                                                    .unwrap_or(Value::Null),
                                                            );
                                                        }
                                                        event.remove("_ingest");
                                                        event.set(
                                                            "_ingest._value.Adjustments",
                                                            Value::Array(out),
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set("_ingest._value.Cvss", Value::Array(out))?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Cvss") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.Cvss").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.Adjustments") {
                                                    if let Some(Value::Array(items)) = event
                                                        .get("_ingest._value.Adjustments")
                                                        .cloned()
                                                    {
                                                        let mut out =
                                                            Vec::with_capacity(items.len());
                                                        for item in items {
                                                            event.set("_ingest._value", item)?;
                                                            if event.has("_ingest._value.Reason") {
                                                                event.rename(
                                                                    "_ingest._value.Reason",
                                                                    "_ingest._value.reason",
                                                                )?;
                                                            }
                                                            out.push(
                                                                event
                                                                    .remove("_ingest._value")
                                                                    .unwrap_or(Value::Null),
                                                            );
                                                        }
                                                        event.remove("_ingest");
                                                        event.set(
                                                            "_ingest._value.Adjustments",
                                                            Value::Array(out),
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set("_ingest._value.Cvss", Value::Array(out))?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Cvss") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.Cvss").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.Adjustments") {
                                                event.rename(
                                                    "_ingest._value.Adjustments",
                                                    "_ingest._value.adjustments",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set("_ingest._value.Cvss", Value::Array(out))?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Cvss") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.Cvss").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 1 handler(s)
                                            if let Err(err) =
                                                (|| -> Result<()> {
                                                    if event.has_value("_ingest._value.BaseScore") {
                                                        if let Some(val) =
                                                            event.get("_ingest._value.BaseScore")
                                                        {
                                                            let converted =
                                                                convert_value(val, "double")
                                                                    .map_err(|message| {
                                                                        TransformError::ParseError {
                        path: "_ingest._value.BaseScore".into(),
                        message,
                        }
                                                                    })?;
                                                            event.set(
                                                                "_ingest._value.base_score",
                                                                converted,
                                                            )?;
                                                        }
                                                    }
                                                    Ok(())
                                                })()
                                            {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.append(
                                                    "error.message",
                                                    json!(
                                                        event
                                                            .get("_ingest.on_failure_message")
                                                            .map_or_else(
                                                                String::new,
                                                                painless_to_string
                                                            )
                                                    ),
                                                )?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set("_ingest._value.Cvss", Value::Array(out))?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Cvss") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.Cvss").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.BaseVector") {
                                                event.rename(
                                                    "_ingest._value.BaseVector",
                                                    "_ingest._value.base_vector",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set("_ingest._value.Cvss", Value::Array(out))?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Cvss") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.Cvss").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.Source") {
                                                event.rename(
                                                    "_ingest._value.Source",
                                                    "_ingest._value.source",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set("_ingest._value.Cvss", Value::Array(out))?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Cvss") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.Cvss").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.Version") {
                                                event.rename(
                                                    "_ingest._value.Version",
                                                    "_ingest._value.version",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set("_ingest._value.Cvss", Value::Array(out))?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Cvss") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.Cvss").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                if let Some(v) =
                                                    event.get("_ingest._value.base_score").cloned()
                                                {
                                                    event.set("vulnerability.score.base", v)?;
                                                }
                                                Ok(())
                                            })(
                                            );
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set("_ingest._value.Cvss", Value::Array(out))?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Cvss") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.Cvss").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                if let Some(v) =
                                                    event.get("_ingest._value.version").cloned()
                                                {
                                                    event.set("vulnerability.score.version", v)?;
                                                }
                                                Ok(())
                                            })(
                                            );
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set("_ingest._value.Cvss", Value::Array(out))?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Cvss") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.Cvss").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            event.remove("_ingest._value.BaseScore");
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set("_ingest._value.Cvss", Value::Array(out))?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Cvss") {
                                event.rename("_ingest._value.Cvss", "_ingest._value.cvss")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Id") {
                                event.rename("_ingest._value.Id", "_ingest._value.id")?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(v) = event.get("_ingest._value.id").cloned() {
                                    event.set("vulnerability.id", v)?;
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.ReferenceUrls") {
                                event.rename(
                                    "_ingest._value.ReferenceUrls",
                                    "_ingest._value.reference_urls",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(v) = event.get("_ingest._value.reference_urls").cloned()
                                {
                                    event.set("vulnerability.reference", v)?;
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.RelatedVulnerabilities") {
                                event.rename(
                                    "_ingest._value.RelatedVulnerabilities",
                                    "_ingest._value.related_vulnerabilities",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Vendor.Name") {
                                event.rename(
                                    "_ingest._value.Vendor.Name",
                                    "_ingest._value.vendor.name",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(v) = event.get("_ingest._value.vendor.name").cloned() {
                                    event.set("vulnerability.scanner.vendor", v)?;
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Vendor.Url") {
                                event.rename(
                                    "_ingest._value.Vendor.Url",
                                    "_ingest._value.vendor.url",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.Vendor.VendorCreatedAt")
                                {
                                    if let Some(parsed) = parse_date_out(
                                        &date_str,
                                        &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                                        None,
                                        None,
                                    ) {
                                        event.set("_ingest._value.vendor.created_at", parsed)?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.Vendor.VendorSeverity") {
                                event.rename(
                                    "_ingest._value.Vendor.VendorSeverity",
                                    "_ingest._value.vendor.severity",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.Vendor.VendorUpdatedAt")
                                {
                                    if let Some(parsed) = parse_date_out(
                                        &date_str,
                                        &["ISO8601", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                                        None,
                                        None,
                                    ) {
                                        event.set("_ingest._value.vendor.updated_at", parsed)?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            event.remove("_ingest._value.Vendor.VendorCreatedAt");
                            event.remove("_ingest._value.Vendor.VendorUpdatedAt");
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.VulnerablePackages") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.VulnerablePackages").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.Category") {
                                                event.rename(
                                                    "_ingest._value.Category",
                                                    "_ingest._value.category",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.VulnerablePackages",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.VulnerablePackages") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.VulnerablePackages").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.Architecture") {
                                                event.rename(
                                                    "_ingest._value.Architecture",
                                                    "_ingest._value.architecture",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.VulnerablePackages",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.VulnerablePackages") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.VulnerablePackages").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.Epoch") {
                                                event.rename(
                                                    "_ingest._value.Epoch",
                                                    "_ingest._value.epoch",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.VulnerablePackages",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.VulnerablePackages") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.VulnerablePackages").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.FilePath") {
                                                event.rename(
                                                    "_ingest._value.FilePath",
                                                    "_ingest._value.file_path",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.VulnerablePackages",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.VulnerablePackages") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.VulnerablePackages").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.Name") {
                                                event.rename(
                                                    "_ingest._value.Name",
                                                    "_ingest._value.name",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.VulnerablePackages",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.VulnerablePackages") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.VulnerablePackages").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.PackageManager") {
                                                event.rename(
                                                    "_ingest._value.PackageManager",
                                                    "_ingest._value.package_manager",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.VulnerablePackages",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.VulnerablePackages") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.VulnerablePackages").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.Release") {
                                                event.rename(
                                                    "_ingest._value.Release",
                                                    "_ingest._value.release",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.VulnerablePackages",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if event.has_value("_ingest._value.VulnerablePackages") {
                                    if let Some(Value::Array(items)) =
                                        event.get("_ingest._value.VulnerablePackages").cloned()
                                    {
                                        let mut out = Vec::with_capacity(items.len());
                                        for item in items {
                                            event.set("_ingest._value", item)?;
                                            if event.has("_ingest._value.Version") {
                                                event.rename(
                                                    "_ingest._value.Version",
                                                    "_ingest._value.version",
                                                )?;
                                            }
                                            out.push(
                                                event
                                                    .remove("_ingest._value")
                                                    .unwrap_or(Value::Null),
                                            );
                                        }
                                        event.remove("_ingest");
                                        event.set(
                                            "_ingest._value.VulnerablePackages",
                                            Value::Array(out),
                                        )?;
                                    }
                                }
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Vulnerabilities")
                    && event
                        .get("json.Vulnerabilities")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("json.Vulnerabilities").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.VulnerablePackages") {
                                event.rename(
                                    "_ingest._value.VulnerablePackages",
                                    "_ingest._value.vulnerable_packages",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("json.Vulnerabilities", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            if event.has("json.Vulnerabilities") {
                event.rename(
                    "json.Vulnerabilities",
                    "aws.securityhub_findings_full_posture.vulnerabilities",
                )?;
            }

            if event.has("json.Workflow.Status") {
                event.rename(
                    "json.Workflow.Status",
                    "aws.securityhub_findings_full_posture.workflow.status",
                )?;
            }

            if event.has("json.WorkflowState") {
                event.rename(
                    "json.WorkflowState",
                    "aws.securityhub_findings_full_posture.workflow.state",
                )?;
            }

            event.remove("json");

            let _cond = {
                event.has_value("aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.ip.address_v4")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique("related.ip", json!(event.get("aws.securityhub_findings_full_posture.action.aws_api_call.remote_ip.ip.address_v4").map_or_else(String::new, painless_to_string)))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("aws.securityhub_findings_full_posture.action.network_connection.remote_ip.ip.address_v4")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique("related.ip", json!(event.get("aws.securityhub_findings_full_posture.action.network_connection.remote_ip.ip.address_v4").map_or_else(String::new, painless_to_string)))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("aws.securityhub_findings_full_posture.action.port_probe.details")
                    && event
                        .get("aws.securityhub_findings_full_posture.action.port_probe.details")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("aws.securityhub_findings_full_posture.action.port_probe.details")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.local.ip.address_v4")
                                            .map_or_else(String::new, painless_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "aws.securityhub_findings_full_posture.action.port_probe.details",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("aws.securityhub_findings_full_posture.action.port_probe.details")
                    && event
                        .get("aws.securityhub_findings_full_posture.action.port_probe.details")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event
                        .get("aws.securityhub_findings_full_posture.action.port_probe.details")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.remote_ip.ip.address_v4")
                                            .map_or_else(String::new, painless_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set(
                            "aws.securityhub_findings_full_posture.action.port_probe.details",
                            Value::Array(out),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("aws.securityhub_findings_full_posture.network.destination.ip.v4")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique("related.ip", json!(event.get("aws.securityhub_findings_full_posture.network.destination.ip.v4").map_or_else(String::new, painless_to_string)))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("aws.securityhub_findings_full_posture.network.destination.ip.v6")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique("related.ip", json!(event.get("aws.securityhub_findings_full_posture.network.destination.ip.v6").map_or_else(String::new, painless_to_string)))?;
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("aws.securityhub_findings_full_posture.network.source.ip.v4") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("aws.securityhub_findings_full_posture.network.source.ip.v4")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("aws.securityhub_findings_full_posture.network.source.ip.v6") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("aws.securityhub_findings_full_posture.network.source.ip.v6")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("aws.securityhub_findings_full_posture.created_at");
                    event.remove("aws.securityhub_findings_full_posture.action.type");
                    event.remove("aws.securityhub_findings_full_posture.id");
                    event
                        .remove("aws.securityhub_findings_full_posture.network.destination.domain");
                    event.remove("aws.securityhub_findings_full_posture.network.destination.ip.v4");
                    event.remove("aws.securityhub_findings_full_posture.network.destination.ip.v6");
                    event.remove("aws.securityhub_findings_full_posture.network.destination.port");
                    event.remove("aws.securityhub_findings_full_posture.network.direction");
                    event.remove("aws.securityhub_findings_full_posture.network.protocol");
                    event.remove("aws.securityhub_findings_full_posture.network.source.domain");
                    event.remove("aws.securityhub_findings_full_posture.network.source.ip.v4");
                    event.remove("aws.securityhub_findings_full_posture.network.source.ip.v6");
                    event.remove("aws.securityhub_findings_full_posture.network.source.mac");
                    event.remove("aws.securityhub_findings_full_posture.network.source.port");
                    event.remove("aws.securityhub_findings_full_posture.process.launched_at");
                    event.remove("aws.securityhub_findings_full_posture.process.name");
                    event.remove("aws.securityhub_findings_full_posture.process.parent.pid");
                    event.remove("aws.securityhub_findings_full_posture.process.path");
                    event.remove("aws.securityhub_findings_full_posture.process.pid");
                    event.remove("aws.securityhub_findings_full_posture.process.terminated_at");
                    Ok(())
                })();
            }

            if event.has_value("aws.securityhub_findings_full_posture.threat_intel_indicators") {
                if let Some(Value::Array(items)) = event
                    .get("aws.securityhub_findings_full_posture.threat_intel_indicators")
                    .cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.remove("_ingest._value.last_observed_at");
                                event.remove("_ingest._value.type");
                                Ok(())
                            })();
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set(
                        "aws.securityhub_findings_full_posture.threat_intel_indicators",
                        Value::Array(out),
                    )?;
                }
            }

            if event.has_value("aws.securityhub_findings_full_posture.vulnerabilities") {
                if let Some(Value::Array(items)) = event
                    .get("aws.securityhub_findings_full_posture.vulnerabilities")
                    .cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.remove("_ingest._value.cvss.base_score");
                                event.remove("_ingest._value.cvss.version");
                                event.remove("_ingest._value.id");
                                event.remove("_ingest._value.reference_urls");
                                event.remove("_ingest._value.vendor.name");
                                Ok(())
                            })();
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set(
                        "aws.securityhub_findings_full_posture.vulnerabilities",
                        Value::Array(out),
                    )?;
                }
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
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
