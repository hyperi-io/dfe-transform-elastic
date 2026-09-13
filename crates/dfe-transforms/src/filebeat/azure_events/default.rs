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

            let _cond = { event.has_value("azure.eventhub") };
            if _cond {
                if event.has_value("azure") {
                    event.rename("azure", "azure-eventhub")?;
                }
            }

            event.set("event.kind", json!("event"))?;

            parse_json_field(event, "message", "tmp_json")?;

            if event.has_value("tmp_json.category") {
                event.rename("tmp_json.category", "routing.category")?;
            }

            let _cond = { !event.has_value("routing.category") };
            if _cond {
                if event.has_value("tmp_json.Category") {
                    event.rename("tmp_json.Category", "routing.category")?;
                }
            }

            let _cond = { !event.has_value("routing.category") };
            if _cond {
                if event.has_value("tmp_json.CategoryValue") {
                    event.rename("tmp_json.CategoryValue", "routing.category")?;
                }
            }

            event.remove("tmp_json");

            event.set("event.dataset", json!("azure.events"))?;

            let _cond = { event.has_value("routing.category") };
            if _cond {
                event.set("event.dataset", json!("azure.platformlogs"))?;
            }

            let _cond = {
                event.get_str("routing.category") == Some("Administrative")
                    || event.get_str("routing.category") == Some("Security")
                    || event.get_str("routing.category") == Some("ServiceHealth")
                    || event.get_str("routing.category") == Some("Alert")
                    || event.get_str("routing.category") == Some("Recommendation")
                    || event.get_str("routing.category") == Some("Policy")
                    || event.get_str("routing.category") == Some("Autoscale")
                    || event.get_str("routing.category") == Some("ResourceHealth")
            };
            if _cond {
                event.set("event.dataset", json!("azure.activitylogs"))?;
            }

            let _cond = {
                event.get_str("routing.category") == Some("ApplicationGatewayFirewallLog")
                    || event.get_str("routing.category") == Some("ApplicationGatewayAccessLog")
            };
            if _cond {
                event.set("event.dataset", json!("azure.application_gateway"))?;
            }

            let _cond = { event.get_str("routing.category") == Some("AuditLogs") };
            if _cond {
                event.set("event.dataset", json!("azure.auditlogs"))?;
            }

            let _cond = {
                event.get_str("routing.category") == Some("AzureFirewallApplicationRule")
                    || event.get_str("routing.category") == Some("AzureFirewallNetworkRule")
                    || event.get_str("routing.category") == Some("AzureFirewallDnsProxy")
                    || event.get_str("routing.category") == Some("AZFWApplicationRule")
                    || event.get_str("routing.category") == Some("AZFWNetworkRule")
                    || event.get_str("routing.category") == Some("AZFWNatRule")
                    || event.get_str("routing.category") == Some("AZFWDnsQuery")
            };
            if _cond {
                event.set("event.dataset", json!("azure.firewall_logs"))?;
            }

            let _cond = { event.get_str("routing.category") == Some("MicrosoftGraphActivityLogs") };
            if _cond {
                event.set("event.dataset", json!("azure.graphactivitylogs"))?;
            }

            let _cond = { event.get_str("routing.category") == Some("AzureADGraphActivityLogs") };
            if _cond {
                event.set("event.dataset", json!("azure.aadgraphactivitylogs"))?;
            }

            let _cond = {
                event.get_str("routing.category") == Some("RiskyUsers")
                    || event.get_str("routing.category") == Some("UserRiskEvents")
            };
            if _cond {
                event.set("event.dataset", json!("azure.identity_protection"))?;
            }

            let _cond = { event.get_str("routing.category") == Some("ProvisioningLogs") };
            if _cond {
                event.set("event.dataset", json!("azure.provisioning"))?;
            }

            let _cond = {
                event.has_value("routing.category")
                    && event
                        .get_str("routing.category")
                        .is_some_and(|s| s.ends_with("SignInLogs"))
            };
            if _cond {
                event.set("event.dataset", json!("azure.signinlogs"))?;
            }

            let _cond = {
                event.get_str("routing.category") == Some("SystemLogs")
                    || event.get_str("routing.category") == Some("ApplicationConsole")
                    || event.get_str("routing.category") == Some("IngressLogs")
                    || event.get_str("routing.category") == Some("BuildLogs")
                    || event.get_str("routing.category") == Some("ContainerEventLogs")
            };
            if _cond {
                event.set("event.dataset", json!("azure.springcloudlogs"))?;
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
