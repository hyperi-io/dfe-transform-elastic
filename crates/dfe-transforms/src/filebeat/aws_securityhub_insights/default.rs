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

            event.set("event.kind", json!("event"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("observer.vendor", json!("AWS Security Hub CSPM"))?;

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
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            if event.has("json.Filters.AwsAccountId") {
                event.rename(
                    "json.Filters.AwsAccountId",
                    "aws.securityhub_insights.filters.aws_account_id",
                )?;
            }

            if event.has("json.Filters.CompanyName") {
                event.rename(
                    "json.Filters.CompanyName",
                    "aws.securityhub_insights.filters.company.name",
                )?;
            }

            if event.has("json.Filters.ComplianceStatus") {
                event.rename(
                    "json.Filters.ComplianceStatus",
                    "aws.securityhub_insights.filters.compliance.status",
                )?;
            }

            if event.has("json.Filters.Confidence") {
                event.rename(
                    "json.Filters.Confidence",
                    "aws.securityhub_insights.filters.confidence",
                )?;
            }

            let _cond = {
                event.has_value("json.Filters.CreatedAt")
                    && event
                        .get("json.Filters.CreatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.CreatedAt", |event| {
                        if event.has("_ingest._value.DateRange.Unit") {
                            event.rename(
                                "_ingest._value.DateRange.Unit",
                                "_ingest._value.date_range.unit",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.CreatedAt")
                    && event
                        .get("json.Filters.CreatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.CreatedAt", |event| {
                        if event.has("_ingest._value.DateRange.Value") {
                            event.rename(
                                "_ingest._value.DateRange.Value",
                                "_ingest._value.date_range.value",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.CreatedAt")
                    && event
                        .get("json.Filters.CreatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.CreatedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.End") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.end", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.CreatedAt")
                    && event
                        .get("json.Filters.CreatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.CreatedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.Start") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.start", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.CreatedAt")
                    && event
                        .get("json.Filters.CreatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.CreatedAt", |event| {
                        event.remove("_ingest._value.Start");
                        event.remove("_ingest._value.End");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has("json.Filters.CreatedAt") {
                event.rename(
                    "json.Filters.CreatedAt",
                    "aws.securityhub_insights.filters.created_at",
                )?;
            }

            if event.has("json.Filters.Criticality") {
                event.rename(
                    "json.Filters.Criticality",
                    "aws.securityhub_insights.filters.criticality",
                )?;
            }

            if event.has("json.Filters.Description") {
                event.rename(
                    "json.Filters.Description",
                    "aws.securityhub_insights.filters.description",
                )?;
            }

            if event.has("json.Filters.FindingProviderFieldsConfidence") {
                event.rename(
                    "json.Filters.FindingProviderFieldsConfidence",
                    "aws.securityhub_insights.filters.finding_provider_fields.confidence",
                )?;
            }

            if event.has("json.Filters.FindingProviderFieldsCriticality") {
                event.rename(
                    "json.Filters.FindingProviderFieldsCriticality",
                    "aws.securityhub_insights.filters.finding_provider_fields.criticality",
                )?;
            }

            if event.has("json.Filters.FindingProviderFieldsRelatedFindingsId") {
                event.rename(
                    "json.Filters.FindingProviderFieldsRelatedFindingsId",
                    "aws.securityhub_insights.filters.finding_provider_fields.related_findings.id",
                )?;
            }

            if event.has("json.Filters.FindingProviderFieldsRelatedFindingsProductArn") {
                event.rename("json.Filters.FindingProviderFieldsRelatedFindingsProductArn", "aws.securityhub_insights.filters.finding_provider_fields.related_findings.product.arn")?;
            }

            if event.has("json.Filters.FindingProviderFieldsSeverityLabel") {
                event.rename(
                    "json.Filters.FindingProviderFieldsSeverityLabel",
                    "aws.securityhub_insights.filters.finding_provider_fields.severity.label",
                )?;
            }

            if event.has("json.Filters.FindingProviderFieldsSeverityOriginal") {
                event.rename(
                    "json.Filters.FindingProviderFieldsSeverityOriginal",
                    "aws.securityhub_insights.filters.finding_provider_fields.severity.original",
                )?;
            }

            if event.has("json.Filters.FindingProviderFieldsTypes") {
                event.rename(
                    "json.Filters.FindingProviderFieldsTypes",
                    "aws.securityhub_insights.filters.finding_provider_fields.types",
                )?;
            }

            let _cond = {
                event.has_value("json.Filters.FirstObservedAt")
                    && event
                        .get("json.Filters.FirstObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.FirstObservedAt", |event| {
                        if event.has("_ingest._value.DateRange.Unit") {
                            event.rename(
                                "_ingest._value.DateRange.Unit",
                                "_ingest._value.date_range.unit",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.FirstObservedAt")
                    && event
                        .get("json.Filters.FirstObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.FirstObservedAt", |event| {
                        if event.has("_ingest._value.DateRange.Value") {
                            event.rename(
                                "_ingest._value.DateRange.Value",
                                "_ingest._value.date_range.value",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.FirstObservedAt")
                    && event
                        .get("json.Filters.FirstObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.FirstObservedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.End") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.end", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.FirstObservedAt")
                    && event
                        .get("json.Filters.FirstObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.FirstObservedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.Start") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.start", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.FirstObservedAt")
                    && event
                        .get("json.Filters.FirstObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.FirstObservedAt", |event| {
                        event.remove("_ingest._value.Start");
                        event.remove("_ingest._value.End");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has("json.Filters.FirstObservedAt") {
                event.rename(
                    "json.Filters.FirstObservedAt",
                    "aws.securityhub_insights.filters.first_observed_at",
                )?;
            }

            if event.has("json.Filters.GeneratorId") {
                event.rename(
                    "json.Filters.GeneratorId",
                    "aws.securityhub_insights.filters.generator.id",
                )?;
            }

            if event.has("json.Filters.Id") {
                event.rename("json.Filters.Id", "aws.securityhub_insights.filters.id")?;
            }

            if event.has("json.Filters.Keyword") {
                event.rename(
                    "json.Filters.Keyword",
                    "aws.securityhub_insights.filters.keyword",
                )?;
            }

            let _cond = {
                event.has_value("json.Filters.LastObservedAt")
                    && event
                        .get("json.Filters.LastObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.LastObservedAt", |event| {
                        if event.has("_ingest._value.DateRange.Unit") {
                            event.rename(
                                "_ingest._value.DateRange.Unit",
                                "_ingest._value.date_range.unit",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.LastObservedAt")
                    && event
                        .get("json.Filters.LastObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.LastObservedAt", |event| {
                        if event.has("_ingest._value.DateRange.Value") {
                            event.rename(
                                "_ingest._value.DateRange.Value",
                                "_ingest._value.date_range.value",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.LastObservedAt")
                    && event
                        .get("json.Filters.LastObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.LastObservedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.End") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.end", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.LastObservedAt")
                    && event
                        .get("json.Filters.LastObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.LastObservedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.Start") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.start", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.LastObservedAt")
                    && event
                        .get("json.Filters.LastObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.LastObservedAt", |event| {
                        event.remove("_ingest._value.Start");
                        event.remove("_ingest._value.End");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has("json.Filters.LastObservedAt") {
                event.rename(
                    "json.Filters.LastObservedAt",
                    "aws.securityhub_insights.filters.last_observed_at",
                )?;
            }

            if event.has("json.Filters.MalwareName") {
                event.rename(
                    "json.Filters.MalwareName",
                    "aws.securityhub_insights.filters.malware.name",
                )?;
            }

            if event.has("json.Filters.MalwarePath") {
                event.rename(
                    "json.Filters.MalwarePath",
                    "aws.securityhub_insights.filters.malware.path",
                )?;
            }

            if event.has("json.Filters.MalwareState") {
                event.rename(
                    "json.Filters.MalwareState",
                    "aws.securityhub_insights.filters.malware.state",
                )?;
            }

            if event.has("json.Filters.MalwareType") {
                event.rename(
                    "json.Filters.MalwareType",
                    "aws.securityhub_insights.filters.malware.type",
                )?;
            }

            if event.has("json.Filters.NetworkDestinationDomain") {
                event.rename(
                    "json.Filters.NetworkDestinationDomain",
                    "aws.securityhub_insights.filters.network.destination.domain",
                )?;
            }

            if event.has("json.Filters.NetworkDestinationIpV4") {
                event.rename(
                    "json.Filters.NetworkDestinationIpV4",
                    "aws.securityhub_insights.filters.network.destination.ip.v4",
                )?;
            }

            if event.has("json.Filters.NetworkDestinationIpV6") {
                event.rename(
                    "json.Filters.NetworkDestinationIpV6",
                    "aws.securityhub_insights.filters.network.destination.ip.v6",
                )?;
            }

            if event.has("json.Filters.NetworkDestinationPort") {
                event.rename(
                    "json.Filters.NetworkDestinationPort",
                    "aws.securityhub_insights.filters.network.destination.port",
                )?;
            }

            if event.has("json.Filters.NetworkDirection") {
                event.rename(
                    "json.Filters.NetworkDirection",
                    "aws.securityhub_insights.filters.network.direction",
                )?;
            }

            if event.has("json.Filters.NetworkProtocol") {
                event.rename(
                    "json.Filters.NetworkProtocol",
                    "aws.securityhub_insights.filters.network.protocol",
                )?;
            }

            if event.has("json.Filters.NetworkSourceDomain") {
                event.rename(
                    "json.Filters.NetworkSourceDomain",
                    "aws.securityhub_insights.filters.network.source.domain",
                )?;
            }

            if event.has("json.Filters.NetworkSourceIpV4") {
                event.rename(
                    "json.Filters.NetworkSourceIpV4",
                    "aws.securityhub_insights.filters.network.source.ip.v4",
                )?;
            }

            if event.has("json.Filters.NetworkSourceIpV6") {
                event.rename(
                    "json.Filters.NetworkSourceIpV6",
                    "aws.securityhub_insights.filters.network.source.ip.v6",
                )?;
            }

            if event.has("json.Filters.NetworkSourceMac") {
                event.rename(
                    "json.Filters.NetworkSourceMac",
                    "aws.securityhub_insights.filters.network.source.mac",
                )?;
            }

            if event.has("json.Filters.NetworkSourcePort") {
                event.rename(
                    "json.Filters.NetworkSourcePort",
                    "aws.securityhub_insights.filters.network.source.port",
                )?;
            }

            if event.has("json.Filters.NoteText") {
                event.rename(
                    "json.Filters.NoteText",
                    "aws.securityhub_insights.filters.note.text",
                )?;
            }

            let _cond = {
                event.has_value("json.Filters.NoteUpdatedAt")
                    && event
                        .get("json.Filters.NoteUpdatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.NoteUpdatedAt", |event| {
                        if event.has("_ingest._value.DateRange.Unit") {
                            event.rename(
                                "_ingest._value.DateRange.Unit",
                                "_ingest._value.date_range.unit",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.NoteUpdatedAt")
                    && event
                        .get("json.Filters.NoteUpdatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.NoteUpdatedAt", |event| {
                        if event.has("_ingest._value.DateRange.Value") {
                            event.rename(
                                "_ingest._value.DateRange.Value",
                                "_ingest._value.date_range.value",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.NoteUpdatedAt")
                    && event
                        .get("json.Filters.NoteUpdatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.NoteUpdatedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.End") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.end", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.NoteUpdatedAt")
                    && event
                        .get("json.Filters.NoteUpdatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.NoteUpdatedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.Start") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.start", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.NoteUpdatedAt")
                    && event
                        .get("json.Filters.NoteUpdatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.NoteUpdatedAt", |event| {
                        event.remove("_ingest._value.Start");
                        event.remove("_ingest._value.End");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has("json.Filters.NoteUpdatedAt") {
                event.rename(
                    "json.Filters.NoteUpdatedAt",
                    "aws.securityhub_insights.filters.note.updated_at",
                )?;
            }

            if event.has("json.Filters.NoteUpdatedBy") {
                event.rename(
                    "json.Filters.NoteUpdatedBy",
                    "aws.securityhub_insights.filters.note.updated_by",
                )?;
            }

            let _cond = {
                event.has_value("json.Filters.ProcessLaunchedAt")
                    && event
                        .get("json.Filters.ProcessLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ProcessLaunchedAt", |event| {
                        if event.has("_ingest._value.DateRange.Unit") {
                            event.rename(
                                "_ingest._value.DateRange.Unit",
                                "_ingest._value.date_range.unit",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ProcessLaunchedAt")
                    && event
                        .get("json.Filters.ProcessLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ProcessLaunchedAt", |event| {
                        if event.has("_ingest._value.DateRange.Value") {
                            event.rename(
                                "_ingest._value.DateRange.Value",
                                "_ingest._value.date_range.value",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ProcessLaunchedAt")
                    && event
                        .get("json.Filters.ProcessLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ProcessLaunchedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.End") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.end", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ProcessLaunchedAt")
                    && event
                        .get("json.Filters.ProcessLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ProcessLaunchedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.Start") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.start", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ProcessLaunchedAt")
                    && event
                        .get("json.Filters.ProcessLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ProcessLaunchedAt", |event| {
                        event.remove("_ingest._value.Start");
                        event.remove("_ingest._value.End");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has("json.Filters.ProcessLaunchedAt") {
                event.rename(
                    "json.Filters.ProcessLaunchedAt",
                    "aws.securityhub_insights.filters.process.launched_at",
                )?;
            }

            if event.has("json.Filters.ProcessName") {
                event.rename(
                    "json.Filters.ProcessName",
                    "aws.securityhub_insights.filters.process.name",
                )?;
            }

            if event.has("json.Filters.ProcessParentPid") {
                event.rename(
                    "json.Filters.ProcessParentPid",
                    "aws.securityhub_insights.filters.process.parent.pid",
                )?;
            }

            if event.has("json.Filters.ProcessPath") {
                event.rename(
                    "json.Filters.ProcessPath",
                    "aws.securityhub_insights.filters.process.path",
                )?;
            }

            if event.has("json.Filters.ProcessPid") {
                event.rename(
                    "json.Filters.ProcessPid",
                    "aws.securityhub_insights.filters.process.pid",
                )?;
            }

            let _cond = {
                event.has_value("json.Filters.ProcessTerminatedAt")
                    && event
                        .get("json.Filters.ProcessTerminatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ProcessTerminatedAt", |event| {
                        if event.has("_ingest._value.DateRange.Unit") {
                            event.rename(
                                "_ingest._value.DateRange.Unit",
                                "_ingest._value.date_range.unit",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ProcessTerminatedAt")
                    && event
                        .get("json.Filters.ProcessTerminatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ProcessTerminatedAt", |event| {
                        if event.has("_ingest._value.DateRange.Value") {
                            event.rename(
                                "_ingest._value.DateRange.Value",
                                "_ingest._value.date_range.value",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ProcessTerminatedAt")
                    && event
                        .get("json.Filters.ProcessTerminatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ProcessTerminatedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.End") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.end", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ProcessTerminatedAt")
                    && event
                        .get("json.Filters.ProcessTerminatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ProcessTerminatedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.Start") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.start", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ProcessTerminatedAt")
                    && event
                        .get("json.Filters.ProcessTerminatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ProcessTerminatedAt", |event| {
                        event.remove("_ingest._value.Start");
                        event.remove("_ingest._value.End");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has("json.Filters.ProcessTerminatedAt") {
                event.rename(
                    "json.Filters.ProcessTerminatedAt",
                    "aws.securityhub_insights.filters.process.terminated_at",
                )?;
            }

            if event.has("json.Filters.ProductArn") {
                event.rename(
                    "json.Filters.ProductArn",
                    "aws.securityhub_insights.filters.product.arn",
                )?;
            }

            if event.has("json.Filters.ProductFields") {
                event.rename(
                    "json.Filters.ProductFields",
                    "aws.securityhub_insights.filters.product.fields",
                )?;
            }

            if event.has("json.Filters.ProductName") {
                event.rename(
                    "json.Filters.ProductName",
                    "aws.securityhub_insights.filters.product.name",
                )?;
            }

            if event.has("json.Filters.RecommendationText") {
                event.rename(
                    "json.Filters.RecommendationText",
                    "aws.securityhub_insights.filters.recommendation_text",
                )?;
            }

            if event.has("json.Filters.RecordState") {
                event.rename(
                    "json.Filters.RecordState",
                    "aws.securityhub_insights.filters.record_state",
                )?;
            }

            if event.has("json.Filters.Region") {
                event.rename(
                    "json.Filters.Region",
                    "aws.securityhub_insights.filters.region",
                )?;
            }

            if event.has("json.Filters.RelatedFindingsId") {
                event.rename(
                    "json.Filters.RelatedFindingsId",
                    "aws.securityhub_insights.filters.related_findings.id",
                )?;
            }

            if event.has("json.Filters.RelatedFindingsProductArn") {
                event.rename(
                    "json.Filters.RelatedFindingsProductArn",
                    "aws.securityhub_insights.filters.related_findings.product.arn",
                )?;
            }

            if event.has("json.Filters.ResourceAwsEc2InstanceIamInstanceProfileArn") {
                event.rename("json.Filters.ResourceAwsEc2InstanceIamInstanceProfileArn", "aws.securityhub_insights.filters.resource.aws_ec2_instance.iam_instance_profile.arn")?;
            }

            if event.has("json.Filters.ResourceAwsEc2InstanceImageId") {
                event.rename(
                    "json.Filters.ResourceAwsEc2InstanceImageId",
                    "aws.securityhub_insights.filters.resource.aws_ec2_instance.image.id",
                )?;
            }

            if event.has("json.Filters.ResourceAwsEc2InstanceIpV4Addresses") {
                event.rename(
                    "json.Filters.ResourceAwsEc2InstanceIpV4Addresses",
                    "aws.securityhub_insights.filters.resource.aws_ec2_instance.ip.v4_addresses",
                )?;
            }

            if event.has("json.Filters.ResourceAwsEc2InstanceIpV6Addresses") {
                event.rename(
                    "json.Filters.ResourceAwsEc2InstanceIpV6Addresses",
                    "aws.securityhub_insights.filters.resource.aws_ec2_instance.ip.v6_addresses",
                )?;
            }

            if event.has("json.Filters.ResourceAwsEc2InstanceKeyName") {
                event.rename(
                    "json.Filters.ResourceAwsEc2InstanceKeyName",
                    "aws.securityhub_insights.filters.resource.aws_ec2_instance.key.name",
                )?;
            }

            let _cond = {
                event.has_value("json.Filters.ResourceAwsEc2InstanceLaunchedAt")
                    && event
                        .get("json.Filters.ResourceAwsEc2InstanceLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ResourceAwsEc2InstanceLaunchedAt",
                        |event| {
                            if event.has("_ingest._value.DateRange.Unit") {
                                event.rename(
                                    "_ingest._value.DateRange.Unit",
                                    "_ingest._value.date_range.unit",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceAwsEc2InstanceLaunchedAt")
                    && event
                        .get("json.Filters.ResourceAwsEc2InstanceLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ResourceAwsEc2InstanceLaunchedAt",
                        |event| {
                            if event.has("_ingest._value.DateRange.Value") {
                                event.rename(
                                    "_ingest._value.DateRange.Value",
                                    "_ingest._value.date_range.value",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceAwsEc2InstanceLaunchedAt")
                    && event
                        .get("json.Filters.ResourceAwsEc2InstanceLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ResourceAwsEc2InstanceLaunchedAt",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_ingest._value.End") {
                                    if let Some(parsed) = parse_date_out(
                                        &date_str,
                                        &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                        None,
                                        None,
                                    ) {
                                        event.set("_ingest._value.end", parsed)?;
                                    }
                                }
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceAwsEc2InstanceLaunchedAt")
                    && event
                        .get("json.Filters.ResourceAwsEc2InstanceLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ResourceAwsEc2InstanceLaunchedAt",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_ingest._value.Start")
                                {
                                    if let Some(parsed) = parse_date_out(
                                        &date_str,
                                        &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                        None,
                                        None,
                                    ) {
                                        event.set("_ingest._value.start", parsed)?;
                                    }
                                }
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceAwsEc2InstanceLaunchedAt")
                    && event
                        .get("json.Filters.ResourceAwsEc2InstanceLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ResourceAwsEc2InstanceLaunchedAt",
                        |event| {
                            event.remove("_ingest._value.Start");
                            event.remove("_ingest._value.End");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.Filters.ResourceAwsEc2InstanceLaunchedAt") {
                event.rename(
                    "json.Filters.ResourceAwsEc2InstanceLaunchedAt",
                    "aws.securityhub_insights.filters.resource.aws_ec2_instance.launched_at",
                )?;
            }

            if event.has("json.Filters.ResourceAwsEc2InstanceSubnetId") {
                event.rename(
                    "json.Filters.ResourceAwsEc2InstanceSubnetId",
                    "aws.securityhub_insights.filters.resource.aws_ec2_instance.subnet.id",
                )?;
            }

            if event.has("json.Filters.ResourceAwsEc2InstanceType") {
                event.rename(
                    "json.Filters.ResourceAwsEc2InstanceType",
                    "aws.securityhub_insights.filters.resource.aws_ec2_instance.type",
                )?;
            }

            if event.has("json.Filters.ResourceAwsEc2InstanceVpcId") {
                event.rename(
                    "json.Filters.ResourceAwsEc2InstanceVpcId",
                    "aws.securityhub_insights.filters.resource.aws_ec2_instance.vpc.id",
                )?;
            }

            let _cond = {
                event.has_value("json.Filters.ResourceAwsIamAccessKeyCreatedAt")
                    && event
                        .get("json.Filters.ResourceAwsIamAccessKeyCreatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ResourceAwsIamAccessKeyCreatedAt",
                        |event| {
                            if event.has("_ingest._value.DateRange.Unit") {
                                event.rename(
                                    "_ingest._value.DateRange.Unit",
                                    "_ingest._value.date_range.unit",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceAwsIamAccessKeyCreatedAt")
                    && event
                        .get("json.Filters.ResourceAwsIamAccessKeyCreatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ResourceAwsIamAccessKeyCreatedAt",
                        |event| {
                            if event.has("_ingest._value.DateRange.Value") {
                                event.rename(
                                    "_ingest._value.DateRange.Value",
                                    "_ingest._value.date_range.value",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceAwsIamAccessKeyCreatedAt")
                    && event
                        .get("json.Filters.ResourceAwsIamAccessKeyCreatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ResourceAwsIamAccessKeyCreatedAt",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_ingest._value.End") {
                                    if let Some(parsed) = parse_date_out(
                                        &date_str,
                                        &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                        None,
                                        None,
                                    ) {
                                        event.set("_ingest._value.end", parsed)?;
                                    }
                                }
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceAwsIamAccessKeyCreatedAt")
                    && event
                        .get("json.Filters.ResourceAwsIamAccessKeyCreatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ResourceAwsIamAccessKeyCreatedAt",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_ingest._value.Start")
                                {
                                    if let Some(parsed) = parse_date_out(
                                        &date_str,
                                        &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                        None,
                                        None,
                                    ) {
                                        event.set("_ingest._value.start", parsed)?;
                                    }
                                }
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceAwsIamAccessKeyCreatedAt")
                    && event
                        .get("json.Filters.ResourceAwsIamAccessKeyCreatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ResourceAwsIamAccessKeyCreatedAt",
                        |event| {
                            event.remove("_ingest._value.Start");
                            event.remove("_ingest._value.End");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.Filters.ResourceAwsIamAccessKeyCreatedAt") {
                event.rename(
                    "json.Filters.ResourceAwsIamAccessKeyCreatedAt",
                    "aws.securityhub_insights.filters.resource.aws_iam_access_key.created_at",
                )?;
            }

            if event.has("json.Filters.ResourceAwsIamAccessKeyPrincipalName") {
                event.rename(
                    "json.Filters.ResourceAwsIamAccessKeyPrincipalName",
                    "aws.securityhub_insights.filters.resource.aws_iam_access_key.principal.name",
                )?;
            }

            if event.has("json.Filters.ResourceAwsIamAccessKeyStatus") {
                event.rename(
                    "json.Filters.ResourceAwsIamAccessKeyStatus",
                    "aws.securityhub_insights.filters.resource.aws_iam_access_key.status",
                )?;
            }

            if event.has("json.Filters.ResourceAwsIamAccessKeyUserName") {
                event.rename(
                    "json.Filters.ResourceAwsIamAccessKeyUserName",
                    "aws.securityhub_insights.filters.resource.aws_iam_access_key.user.name",
                )?;
            }

            if event.has("json.Filters.ResourceAwsIamUserUserName") {
                event.rename(
                    "json.Filters.ResourceAwsIamUserUserName",
                    "aws.securityhub_insights.filters.resource.aws_iam_user.user.name",
                )?;
            }

            if event.has("json.Filters.ResourceAwsS3BucketOwnerId") {
                event.rename(
                    "json.Filters.ResourceAwsS3BucketOwnerId",
                    "aws.securityhub_insights.filters.resource.aws_s3_bucket.owner.id",
                )?;
            }

            if event.has("json.Filters.ResourceAwsS3BucketOwnerName") {
                event.rename(
                    "json.Filters.ResourceAwsS3BucketOwnerName",
                    "aws.securityhub_insights.filters.resource.aws_s3_bucket.owner.name",
                )?;
            }

            if event.has("json.Filters.ResourceContainerImageId") {
                event.rename(
                    "json.Filters.ResourceContainerImageId",
                    "aws.securityhub_insights.filters.resource.container.image.id",
                )?;
            }

            if event.has("json.Filters.ResourceContainerImageName") {
                event.rename(
                    "json.Filters.ResourceContainerImageName",
                    "aws.securityhub_insights.filters.resource.container.image.name",
                )?;
            }

            let _cond = {
                event.has_value("json.Filters.ResourceContainerLaunchedAt")
                    && event
                        .get("json.Filters.ResourceContainerLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ResourceContainerLaunchedAt", |event| {
                        if event.has("_ingest._value.DateRange.Unit") {
                            event.rename(
                                "_ingest._value.DateRange.Unit",
                                "_ingest._value.date_range.unit",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceContainerLaunchedAt")
                    && event
                        .get("json.Filters.ResourceContainerLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ResourceContainerLaunchedAt", |event| {
                        if event.has("_ingest._value.DateRange.Value") {
                            event.rename(
                                "_ingest._value.DateRange.Value",
                                "_ingest._value.date_range.value",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceContainerLaunchedAt")
                    && event
                        .get("json.Filters.ResourceContainerLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ResourceContainerLaunchedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.End") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.end", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceContainerLaunchedAt")
                    && event
                        .get("json.Filters.ResourceContainerLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ResourceContainerLaunchedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.Start") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.start", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ResourceContainerLaunchedAt")
                    && event
                        .get("json.Filters.ResourceContainerLaunchedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.ResourceContainerLaunchedAt", |event| {
                        event.remove("_ingest._value.Start");
                        event.remove("_ingest._value.End");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has("json.Filters.ResourceContainerLaunchedAt") {
                event.rename(
                    "json.Filters.ResourceContainerLaunchedAt",
                    "aws.securityhub_insights.filters.resource.container.launched_at",
                )?;
            }

            if event.has("json.Filters.ResourceContainerName") {
                event.rename(
                    "json.Filters.ResourceContainerName",
                    "aws.securityhub_insights.filters.resource.container.name",
                )?;
            }

            if event.has("json.Filters.ResourceDetailsOther") {
                event.rename(
                    "json.Filters.ResourceDetailsOther",
                    "aws.securityhub_insights.filters.resource.details_other",
                )?;
            }

            if event.has("json.Filters.ResourceId") {
                event.rename(
                    "json.Filters.ResourceId",
                    "aws.securityhub_insights.filters.resource.id",
                )?;
            }

            if event.has("json.Filters.ResourcePartition") {
                event.rename(
                    "json.Filters.ResourcePartition",
                    "aws.securityhub_insights.filters.resource.partition",
                )?;
            }

            if event.has("json.Filters.ResourceRegion") {
                event.rename(
                    "json.Filters.ResourceRegion",
                    "aws.securityhub_insights.filters.resource.region",
                )?;
            }

            if event.has("json.Filters.ResourceTags") {
                event.rename(
                    "json.Filters.ResourceTags",
                    "aws.securityhub_insights.filters.resource.tags",
                )?;
            }

            if event.has("json.Filters.ResourceType") {
                event.rename(
                    "json.Filters.ResourceType",
                    "aws.securityhub_insights.filters.resource.type",
                )?;
            }

            if event.has("json.Filters.Sample") {
                event.rename(
                    "json.Filters.Sample",
                    "aws.securityhub_insights.filters.sample",
                )?;
            }

            if event.has("json.Filters.SeverityLabel") {
                event.rename(
                    "json.Filters.SeverityLabel",
                    "aws.securityhub_insights.filters.severity.label",
                )?;
            }

            if event.has("json.Filters.SeverityNormalized") {
                event.rename(
                    "json.Filters.SeverityNormalized",
                    "aws.securityhub_insights.filters.severity.normalized",
                )?;
            }

            if event.has("json.Filters.SeverityProduct") {
                event.rename(
                    "json.Filters.SeverityProduct",
                    "aws.securityhub_insights.filters.severity.product",
                )?;
            }

            if event.has("json.Filters.SourceUrl") {
                event.rename(
                    "json.Filters.SourceUrl",
                    "aws.securityhub_insights.filters.source_url",
                )?;
            }

            if event.has("json.Filters.ThreatIntelIndicatorCategory") {
                event.rename(
                    "json.Filters.ThreatIntelIndicatorCategory",
                    "aws.securityhub_insights.filters.threat_intel_indicator.category",
                )?;
            }

            let _cond = {
                event.has_value("json.Filters.ThreatIntelIndicatorLastObservedAt")
                    && event
                        .get("json.Filters.ThreatIntelIndicatorLastObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ThreatIntelIndicatorLastObservedAt",
                        |event| {
                            if event.has("_ingest._value.DateRange.Unit") {
                                event.rename(
                                    "_ingest._value.DateRange.Unit",
                                    "_ingest._value.date_range.unit",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ThreatIntelIndicatorLastObservedAt")
                    && event
                        .get("json.Filters.ThreatIntelIndicatorLastObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ThreatIntelIndicatorLastObservedAt",
                        |event| {
                            if event.has("_ingest._value.DateRange.Value") {
                                event.rename(
                                    "_ingest._value.DateRange.Value",
                                    "_ingest._value.date_range.value",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ThreatIntelIndicatorLastObservedAt")
                    && event
                        .get("json.Filters.ThreatIntelIndicatorLastObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ThreatIntelIndicatorLastObservedAt",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_ingest._value.End") {
                                    if let Some(parsed) = parse_date_out(
                                        &date_str,
                                        &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                        None,
                                        None,
                                    ) {
                                        event.set("_ingest._value.end", parsed)?;
                                    }
                                }
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ThreatIntelIndicatorLastObservedAt")
                    && event
                        .get("json.Filters.ThreatIntelIndicatorLastObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ThreatIntelIndicatorLastObservedAt",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_ingest._value.Start")
                                {
                                    if let Some(parsed) = parse_date_out(
                                        &date_str,
                                        &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                        None,
                                        None,
                                    ) {
                                        event.set("_ingest._value.start", parsed)?;
                                    }
                                }
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.ThreatIntelIndicatorLastObservedAt")
                    && event
                        .get("json.Filters.ThreatIntelIndicatorLastObservedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.Filters.ThreatIntelIndicatorLastObservedAt",
                        |event| {
                            event.remove("_ingest._value.Start");
                            event.remove("_ingest._value.End");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has("json.Filters.ThreatIntelIndicatorLastObservedAt") {
                event.rename(
                    "json.Filters.ThreatIntelIndicatorLastObservedAt",
                    "aws.securityhub_insights.filters.threat_intel_indicator.last_observed_at",
                )?;
            }

            if event.has("json.Filters.ThreatIntelIndicatorSource") {
                event.rename(
                    "json.Filters.ThreatIntelIndicatorSource",
                    "aws.securityhub_insights.filters.threat_intel_indicator.source",
                )?;
            }

            if event.has("json.Filters.ThreatIntelIndicatorSourceUrl") {
                event.rename(
                    "json.Filters.ThreatIntelIndicatorSourceUrl",
                    "aws.securityhub_insights.filters.threat_intel_indicator.source_url",
                )?;
            }

            if event.has("json.Filters.ThreatIntelIndicatorType") {
                event.rename(
                    "json.Filters.ThreatIntelIndicatorType",
                    "aws.securityhub_insights.filters.threat_intel_indicator.type",
                )?;
            }

            if event.has("json.Filters.ThreatIntelIndicatorValue") {
                event.rename(
                    "json.Filters.ThreatIntelIndicatorValue",
                    "aws.securityhub_insights.filters.threat_intel_indicator.value",
                )?;
            }

            if event.has("json.Filters.Title") {
                event.rename(
                    "json.Filters.Title",
                    "aws.securityhub_insights.filters.title",
                )?;
            }

            if event.has("json.Filters.Type") {
                event.rename("json.Filters.Type", "aws.securityhub_insights.filters.type")?;
            }

            let _cond = {
                event.has_value("json.Filters.UpdatedAt")
                    && event
                        .get("json.Filters.UpdatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.UpdatedAt", |event| {
                        if event.has("_ingest._value.DateRange.Unit") {
                            event.rename(
                                "_ingest._value.DateRange.Unit",
                                "_ingest._value.date_range.unit",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.UpdatedAt")
                    && event
                        .get("json.Filters.UpdatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.UpdatedAt", |event| {
                        if event.has("_ingest._value.DateRange.Value") {
                            event.rename(
                                "_ingest._value.DateRange.Value",
                                "_ingest._value.date_range.value",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.UpdatedAt")
                    && event
                        .get("json.Filters.UpdatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.UpdatedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.End") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.end", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.UpdatedAt")
                    && event
                        .get("json.Filters.UpdatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.UpdatedAt", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.Start") {
                                if let Some(parsed) = parse_date_out(
                                    &date_str,
                                    &["ISO8601", "yyyy-MM-dd HH:mm:ss.SSS"],
                                    None,
                                    None,
                                ) {
                                    event.set("_ingest._value.start", parsed)?;
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Filters.UpdatedAt")
                    && event
                        .get("json.Filters.UpdatedAt")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.Filters.UpdatedAt", |event| {
                        event.remove("_ingest._value.Start");
                        event.remove("_ingest._value.End");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has("json.Filters.UpdatedAt") {
                event.rename(
                    "json.Filters.UpdatedAt",
                    "aws.securityhub_insights.filters.updated_at",
                )?;
            }

            if event.has("json.Filters.UserDefinedFields") {
                event.rename(
                    "json.Filters.UserDefinedFields",
                    "aws.securityhub_insights.filters.user_defined_fields",
                )?;
            }

            if event.has("json.Filters.VerificationState") {
                event.rename(
                    "json.Filters.VerificationState",
                    "aws.securityhub_insights.filters.verification.state",
                )?;
            }

            if event.has("json.Filters.WorkflowState") {
                event.rename(
                    "json.Filters.WorkflowState",
                    "aws.securityhub_insights.filters.workflow.state",
                )?;
            }

            if event.has("json.Filters.WorkflowStatus") {
                event.rename(
                    "json.Filters.WorkflowStatus",
                    "aws.securityhub_insights.filters.workflow.status",
                )?;
            }

            if event.has("json.GroupByAttribute") {
                event.rename(
                    "json.GroupByAttribute",
                    "aws.securityhub_insights.group_by_attribute",
                )?;
            }

            if event.has("json.InsightArn") {
                event.rename("json.InsightArn", "aws.securityhub_insights.insight_arn")?;
            }

            if event.has("json.Name") {
                event.rename("json.Name", "aws.securityhub_insights.name")?;
            }

            event.remove("json");

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
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
