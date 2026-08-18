// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `azure_shared_pipeline` pipeline.
pub struct AzureSharedPipeline;

impl Transform for AzureSharedPipeline {
    fn name(&self) -> &str {
        "azure_shared_pipeline"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("cloud.provider", json!("azure"))?;

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            // Pattern definitions for grok
            // PROVIDERNAME = .+
            // NAMESPACE = .+
            // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
            // GROUPID = .+
            // RULE = .+
            if let Some(input) = event.get_string("azure.resource_id") {
                // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/NAMESPACES/%{NAMESPACE:azure.resource.namespace}/AUTHORIZATIONRULES/%{RULE:azure.resource.authorization_rule}
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/NAMESPACES/%{NAMESPACE:azure.resource.namespace}/AUTHORIZATIONRULES/%{RULE:azure.resource.authorization_rule}").extract_into(input, event)?;
                // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}/providers/%{PROVIDERNAME:azure.resource.provider}/namespaces/%{NAMESPACE:azure.resource.namespace}/authorizationRules/%{RULE:azure.resource.authorization_rule}
            }
            Ok(())
        })();

        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // PROVIDERNAME = ([A-Za-z])\w+.([A-Za-z])\w+/([A-Za-z])\w+.
                // NAME = ((?!AUTHORIZATIONRULES).)*$
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                // GROUPID = .+
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}").extract_into(input, event)?;
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}/providers/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                }
                Ok(())
            })();
        }

        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                // NAME = .+
                // GROUPID = .+
                // PROVIDERNAME = ([A-Za-z])\w+.([A-Za-z])\w+\/([A-Za-z][^\/])\w+
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}").extract_into(input, event)?;
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}/providers/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                }
                Ok(())
            })();
        }

        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // PROVIDER = .+
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /providers/%{PROVIDER:azure.resource.provider}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/providers/%{PROVIDER:azure.resource.provider}")
                        .extract_into(input, event)?;
                    // Additional grok pattern 1: /PROVIDERS/%{PROVIDER:azure.resource.provider}
                }
                Ok(())
            })();
        }

        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // PROVIDERNAME = ([A-Za-z])\w+.([A-Za-z])\w+\/([A-Za-z][^\/])\w+
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}").extract_into(input, event)?;
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/providers/%{PROVIDERNAME:azure.resource.provider}
                }
                Ok(())
            })();
        }

        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // GROUPID = .+
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}").extract_into(input, event)?;
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}
                }
                Ok(())
            })();
        }

        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}")
                        .extract_into(input, event)?;
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}
                }
                Ok(())
            })();
        }

        if event.has("azure.resource_id") {
            event.rename("azure.resource_id", "azure.resource.id")?;
        }

        if event.has("event.outcome") {
            if let Some(s) = event.get_string("event.outcome") {
                let lowered = s.to_lowercase();
                event.set("event.outcome", lowered)?;
            }
        }

        // --- Post-processing ---
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
        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
