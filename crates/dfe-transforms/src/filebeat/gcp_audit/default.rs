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
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("event.original") {
                        let parsed: Value =
                            serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                                path: "event.original".into(),
                                message: format!("failed to parse JSON: {}", e),
                            })?;
                        event.set("json", parsed)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    if let Some(v) = event
                        .get("event.original")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("gcp.audit.notification", v)?;
                    }
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
                if let Some(v) = event.get("json.protoPayload.@type").cloned() {
                    event.set("gcp.audit.type", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("gcp.audit.type")
                    && event.get_str("gcp.audit.type")
                        != Some("type.googleapis.com/google.cloud.audit.AuditLog")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { event.has_value("json.insertId") };
            if _cond {
                if let Some(v) = event.get("json.insertId").cloned() {
                    event.set("event.id", v)?;
                }
            }

            if event.has("json.logName") {
                event.rename("json.logName", "log.logger")?;
            }

            if event.has("json.severity") {
                event.rename("json.severity", "log.level")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("log.logger") {
                    if let Some(input) = event.get_string("log.logger") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("%2F") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("%2F") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("event.provider", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            event.set("event.kind", json!("event"))?;

            event.set("cloud.provider", json!("gcp"))?;

            let _cond = {
                event.has_value("json.timestamp") && event.get_str("json.timestamp") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("json.resource.labels.project_id") };
            if _cond {
                if let Some(v) = event.get("json.resource.labels.project_id").cloned() {
                    event.set("cloud.project.id", v)?;
                }
            }

            let _cond = { event.has_value("json.resource.labels.instance_id") };
            if _cond {
                if let Some(v) = event.get("json.resource.labels.instance_id").cloned() {
                    event.set("cloud.instance.id", v)?;
                }
            }

            if let Some(v) = event
                .get("json.resource.labels.location")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.availability_zone", v)?;
            }

            let _cond = {
                event.has_value("json.resource.type")
                    && (event.get_str("json.resource.type") == Some("k8s_cluster")
                        || event.get_str("json.resource.type") == Some("gke_cluster"))
            };
            if _cond {
                event.set("orchestrator.type", json!("kubernetes"))?;
            }

            let _cond = {
                event.has_value("json.resource.type")
                    && (event.get_str("json.resource.type") == Some("k8s_cluster")
                        || event.get_str("json.resource.type") == Some("gke_cluster"))
            };
            if _cond {
                if let Some(v) = event
                    .get("json.resource.labels.cluster_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.cluster.name", v)?;
                }
            }

            let _cond = {
                event.has_value("json.resource.type")
                    && event.get_str("json.resource.type") == Some("k8s_cluster")
            };
            if _cond {
                if let Some(v) = event
                    .get("json.protoPayload.resourceName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("_temp.type", v)?;
                }
            }

            if event.has_value("_temp.type") {
                if let Some(input) = event.get_string("_temp.type") {
                    // Grok pattern: %{DATA}/(?P<orchestrator_api_version>(?:(v\\d+([a-z]+)?(\\d+)?)))/namespaces/%{DATA:orchestrator.namespace}/(?P<orchestrator_resource_type>(?:([a-z]+((\\.[a-z0-9]+)+)?)))(/%{HOSTNAME:orchestrator.resource.name})?
                    if !cached_grok_mapped!("%{DATA}/(?P<orchestrator_api_version>(?:(v\\d+([a-z]+)?(\\d+)?)))/namespaces/%{DATA:orchestrator.namespace}/(?P<orchestrator_resource_type>(?:([a-z]+((\\.[a-z0-9]+)+)?)))(/%{HOSTNAME:orchestrator.resource.name})?", [("orchestrator_api_version", "orchestrator.api_version"), ("orchestrator_resource_type", "orchestrator.resource.type")]).extract_into(&input, event)? {
                        // Grok pattern: %{DATA}/(?P<orchestrator_api_version>(?:(v\\d+([a-z]+)?(\\d+)?)))/(?P<orchestrator_resource_type>(?:([a-z]+((\\.[a-z0-9]+)+)?)))
                        if !cached_grok_mapped!("%{DATA}/(?P<orchestrator_api_version>(?:(v\\d+([a-z]+)?(\\d+)?)))/(?P<orchestrator_resource_type>(?:([a-z]+((\\.[a-z0-9]+)+)?)))", [("orchestrator_api_version", "orchestrator.api_version"), ("orchestrator_resource_type", "orchestrator.resource.type")]).extract_into(&input, event)? {
                            // Grok pattern: apis/(?P<orchestrator_resource_type>(?:([a-z]+((\\.[a-z0-9]+)+)?)))/(?P<orchestrator_api_version>(?:(v\\d+([a-z]+)?(\\d+)?)))
                            if !cached_grok_mapped!("apis/(?P<orchestrator_resource_type>(?:([a-z]+((\\.[a-z0-9]+)+)?)))/(?P<orchestrator_api_version>(?:(v\\d+([a-z]+)?(\\d+)?)))", [("orchestrator_resource_type", "orchestrator.resource.type"), ("orchestrator_api_version", "orchestrator.api_version")]).extract_into(&input, event)? {
                                // Grok pattern: api/(?P<orchestrator_api_version>(?:(v\\d+([a-z]+)?(\\d+)?)))
                                if !cached_grok_mapped!("api/(?P<orchestrator_api_version>(?:(v\\d+([a-z]+)?(\\d+)?)))", [("orchestrator_api_version", "orchestrator.api_version")]).extract_into(&input, event)? {
                                    // Grok pattern: (?P<orchestrator_resource_type>(?:([a-z]+((\\.[a-z0-9]+)+)?)))
                                    if !cached_grok_mapped!("(?P<orchestrator_resource_type>(?:([a-z]+((\\.[a-z0-9]+)+)?)))", [("orchestrator_resource_type", "orchestrator.resource.type")]).extract_into(&input, event)? {
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: void addValue(Set entities, def value) {\n  if (value != null && value != \"\") {\n    entities.add(value);\n  }\n}\n\nboolean isKubernetes = false;\nif (ctx.json?.resource?.type != null) {\n  String typ = ctx.json.resource.type;\n  isKubernetes = (typ == \"k8s_cluster\" || typ == \"gke_cluster\" || typ == \"kubernetes\");\n}\n\n// Using tree set to ensure a sorting is kept (testing purposes)\nTreeSet entities = new TreeSet();\n\naddValue(entities, ctx.json?.protoPayload?.request?.parent);\nif (!isKubernetes) {\n  addValue(entities, ctx.json?.protoPayload?.resourceName);\n  addValue(entities, ctx.json?.protoPayload?.response?.user);\n}\n\nHashMap authInfo = ctx.json?.protoPayload?.authenticationInfo ?: new HashMap();\nif (!isKubernetes) {\n  addValue(entities, authInfo.principalEmail);\n}\naddValue(entities, authInfo.principalSubject);\naddValue(entities, authInfo.serviceAccountKeyName);\nif (authInfo.serviceAccountDelegationInfo instanceof List) {\n  for (def i: authInfo.serviceAccountDelegationInfo) {\n    addValue(entities, i.principalSubject);\n    addValue(entities, i.firstPartyPrincipal?.principalEmail);\n    addValue(entities, i.thirdPartyPrincipal?.principalEmail);\n  }\n}\n\nString serviceName = ctx.json?.protoPayload?.serviceName ?: '';\nif (serviceName == \"compute.googleapis.com\") {\n  if (ctx.json?.protoPayload?.request?.networkInterfaces instanceof List) {\n    for (def e: ctx.json.protoPayload.request.networkInterfaces) {\n      addValue(entities, e.network);\n    }\n  }\n  if (ctx.json?.protoPayload?.request?.serviceAccounts instanceof List) {\n    for (def e: ctx.json.protoPayload.request.serviceAccounts) {\n      addValue(entities, e.email);\n    }\n  }\n  if (ctx.json?.protoPayload?.request?.disks instanceof List) {\n    for (def e: ctx.json.protoPayload.request.disks) {\n      addValue(entities, e.source);\n    }\n  }\n} else if (serviceName == \"cloudresourcemanager.googleapis.com\") {\n  if (ctx.json?.protoPayload?.request?.policy?.bindings instanceof List) {\n    for (def e: ctx.json.protoPayload.request.policy.bindings) {\n      addValue(entities, e.role);\n      for (def m: e.members) {\n        addValue(entities, m);\n      }\n    }\n  }\n  if (ctx.json?.protoPayload?.response?.bindings instanceof List) {\n    for (def e: ctx.json.protoPayload.response.bindings) {\n      addValue(entities, e.role);\n      for (def m: e.members) {\n        addValue(entities, m);\n      }\n    }\n  }\n} else if (serviceName == \"iamcredentials.googleapis.com\") {\n  if (ctx.json?.protoPayload?.metadata?.identityDelegationChain instanceof List) {\n    for (def e: ctx.json.protoPayload.metadata.identityDelegationChain) {\n      addValue(entities, e);\n    }\n  }\n}\n\nif (entities.size() > 0) {\n  ctx.related = ctx.related ?: [:];\n  ctx.related.entity = entities;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"void addValue(Set entities, def value) {\n  if (value != null && value != \"\") {\n    entities.add(value);\n  }\n}\n\nboolean isKubernetes = false;\nif (ctx.json?.resource?.type != null) {\n  String typ = ctx.json.resource.type;\n  isKubernetes = (typ == \"k8s_cluster\" || typ == \"gke_cluster\" || typ == \"kubernetes\");\n}\n\n// Using tree set to ensure a sorting is kept (testing purposes)\nTreeSet entities = new TreeSet();\n\naddValue(entities, ctx.json?.protoPayload?.request?.parent);\nif (!isKubernetes) {\n  addValue(entities, ctx.json?.protoPayload?.resourceName);\n  addValue(entities, ctx.json?.protoPayload?.response?.user);\n}\n\nHashMap authInfo = ctx.json?.protoPayload?.authenticationInfo ?: new HashMap();\nif (!isKubernetes) {\n  addValue(entities, authInfo.principalEmail);\n}\naddValue(entities, authInfo.principalSubject);\naddValue(entities, authInfo.serviceAccountKeyName);\nif (authInfo.serviceAccountDelegationInfo instanceof List) {\n  for (def i: authInfo.serviceAccountDelegationInfo) {\n    addValue(entities, i.principalSubject);\n    addValue(entities, i.firstPartyPrincipal?.principalEmail);\n    addValue(entities, i.thirdPartyPrincipal?.principalEmail);\n  }\n}\n\nString serviceName = ctx.json?.protoPayload?.serviceName ?: '';\nif (serviceName == \"compute.googleapis.com\") {\n  if (ctx.json?.protoPayload?.request?.networkInterfaces instanceof List) {\n    for (def e: ctx.json.protoPayload.request.networkInterfaces) {\n      addValue(entities, e.network);\n    }\n  }\n  if (ctx.json?.protoPayload?.request?.serviceAccounts instanceof List) {\n    for (def e: ctx.json.protoPayload.request.serviceAccounts) {\n      addValue(entities, e.email);\n    }\n  }\n  if (ctx.json?.protoPayload?.request?.disks instanceof List) {\n    for (def e: ctx.json.protoPayload.request.disks) {\n      addValue(entities, e.source);\n    }\n  }\n} else if (serviceName == \"cloudresourcemanager.googleapis.com\") {\n  if (ctx.json?.protoPayload?.request?.policy?.bindings instanceof List) {\n    for (def e: ctx.json.protoPayload.request.policy.bindings) {\n      addValue(entities, e.role);\n      for (def m: e.members) {\n        addValue(entities, m);\n      }\n    }\n  }\n  if (ctx.json?.protoPayload?.response?.bindings instanceof List) {\n    for (def e: ctx.json.protoPayload.response.bindings) {\n      addValue(entities, e.role);\n      for (def m: e.members) {\n        addValue(entities, m);\n      }\n    }\n  }\n} else if (serviceName == \"iamcredentials.googleapis.com\") {\n  if (ctx.json?.protoPayload?.metadata?.identityDelegationChain instanceof List) {\n    for (def e: ctx.json.protoPayload.metadata.identityDelegationChain) {\n      addValue(entities, e);\n    }\n  }\n}\n\nif (entities.size() > 0) {\n  ctx.related = ctx.related ?: [:];\n  ctx.related.entity = entities;\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
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

            let _cond =
                { event.has_value("json.protoPayload.authenticationInfo.principalSubject") };
            if _cond {
                event.append(
                    "actor.entity.id",
                    json!(
                        event
                            .get("json.protoPayload.authenticationInfo.principalSubject")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.protoPayload.authenticationInfo.principalEmail") };
            if _cond {
                event.append(
                    "actor.entity.id",
                    json!(
                        event
                            .get("json.protoPayload.authenticationInfo.principalEmail")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("json.resource.type") != Some("k8s_cluster")
                    && event.has_value("json.protoPayload.resourceName")
            };
            if _cond {
                event.append(
                    "target.entity.id",
                    json!(
                        event
                            .get("json.protoPayload.resourceName")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.resouce.disk_id") };
            if _cond {
                event.append(
                    "target.entity.id",
                    json!(
                        event
                            .get("json.resource.disk_id")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: void addNestedValue(def currentCtx, String path, def value) {\n  if (value == null || value == \"\") {\n    return;\n  }\n  String[] parts = path.splitOnToken('.');\n  for (int i = 0; i < parts.length - 1; i++) {\n    if (currentCtx[parts[i]] == null) {\n      currentCtx[parts[i]] = [:];\n    }\n    currentCtx = currentCtx[parts[i]];\n  }\n  String lastPart = parts[parts.length - 1];\n  if (currentCtx[lastPart] == null) {\n    currentCtx[lastPart] = [];\n  }\n  if (!currentCtx[lastPart].contains(value)) {\n    currentCtx[lastPart].add(value);\n  }\n}\n\n// Classify actor entities\nif (ctx.actor?.entity?.id instanceof List) {\n  for (def actorId : ctx.actor.entity.id) {\n    if (actorId == null || actorId == \"\") {\n      continue;\n    }\n    String actor = actorId.toString();\n    \n    // Service accounts: serviceAccount:* or *.gserviceaccount.com (includes cloudservices and iam)\n    if (actor.startsWith(\"serviceAccount:\") || actor.contains(\".gserviceaccount.com\")) {\n      addNestedValue(ctx, \"service.entity.id\", actor);\n    }\n    // Service account paths: projects/*/serviceAccounts/*\n    else if (actor.contains(\"/serviceAccounts\")) {\n      addNestedValue(ctx, \"service.entity.id\", actor);\n    }\n    // User accounts: user:* or email addresses (not service accounts)\n    else if (actor.startsWith(\"user:\") || (actor.contains(\"@\") && !actor.contains(\".gserviceaccount.com\"))) {\n      addNestedValue(ctx, \"user.entity.id\", actor);\n    }\n    // Compute instances: projects/*/zones/*/instances/*\n    else if (actor.contains(\"/instances\")) {\n      addNestedValue(ctx, \"host.entity.id\", actor);\n    }\n    // GCP service or API names: *.googleapis.com (but not principal:// or workloadIdentityPools)\n    else if (actor.contains(\".googleapis.com\") && !actor.startsWith(\"principal://\") && !actor.contains(\"/workloadIdentityPools\")) {\n      addNestedValue(ctx, \"service.entity.id\", actor);\n    }\n    // Everything else (repo:, workloadIdentityPools, principal://, etc.)\n    else {\n      addNestedValue(ctx, \"entity.id\", actor);\n    }\n  }\n}\n\n// Classify target entities\nif (ctx.target?.entity?.id instanceof List) {\n  for (def targetId : ctx.target.entity.id) {\n    if (targetId == null || targetId == \"\") {\n      continue;\n    }\n    String target = targetId.toString();\n    \n    // Compute instances\n    if (target.contains(\"/instances\")) {\n      addNestedValue(ctx, \"host.target.entity.id\", target);\n    }\n    // Service accounts: projects/*/serviceAccounts/* or email addresses ending with .gserviceaccount.com\n    else if (target.contains(\"/serviceAccounts\") || \n             (target.contains(\"@\") && target.contains(\".gserviceaccount.com\"))) {\n      addNestedValue(ctx, \"service.target.entity.id\", target);\n    }\n    // IAM principals (users, groups)\n    else if (target.contains(\"/users\") || \n             target.contains(\"/groups\") ||\n             (target.contains(\"@\") && target.contains(\"/members\"))) {\n      addNestedValue(ctx, \"user.target.entity.id\", target);\n    }\n    // GCP service resources\n    else if (target.contains(\"/machineTypes\") ||\n             target.contains(\"/deployments\") ||\n             target.contains(\"/services\") ||\n             target.contains(\"/policies\") ||\n             target.contains(\"/operations\") ||\n             target.contains(\"/subnetworks\") ||\n             target.contains(\"/networkEdgeSecurityServices\") ||\n             target.contains(\"/functions\") ||\n             target.contains(\"/buckets\") ||\n             target.contains(\"/datasets\") ||\n             target.contains(\"/tables\") ||\n             target.contains(\"/disks\") ||\n             target.contains(\"/networks\") ||\n             target.contains(\"/firewalls\") ||\n             target.contains(\"/backendServices\") ||\n             target.contains(\"/forwardingRules\") ||\n             target.contains(\"/healthChecks\") ||\n             target.contains(\"/sslCertificates\") ||\n             target.contains(\"/targetPools\") ||\n             target.contains(\"/urlMaps\") ||\n             target.contains(\".googleapis.com\")) {\n      addNestedValue(ctx, \"service.target.entity.id\", target);\n    }\n    // Everything else (projects, zones, workloadIdentityPools, etc.)\n    else {\n      addNestedValue(ctx, \"entity.target.id\", target);\n    }\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"void addNestedValue(def currentCtx, String path, def value) {\n  if (value == null || value == \"\") {\n    return;\n  }\n  String[] parts = path.splitOnToken('.');\n  for (int i = 0; i < parts.length - 1; i++) {\n    if (currentCtx[parts[i]] == null) {\n      currentCtx[parts[i]] = [:];\n    }\n    currentCtx = currentCtx[parts[i]];\n  }\n  String lastPart = parts[parts.length - 1];\n  if (currentCtx[lastPart] == null) {\n    currentCtx[lastPart] = [];\n  }\n  if (!currentCtx[lastPart].contains(value)) {\n    currentCtx[lastPart].add(value);\n  }\n}\n\n// Classify actor entities\nif (ctx.actor?.entity?.id instanceof List) {\n  for (def actorId : ctx.actor.entity.id) {\n    if (actorId == null || actorId == \"\") {\n      continue;\n    }\n    String actor = actorId.toString();\n    \n    // Service accounts: serviceAccount:* or *.gserviceaccount.com (includes cloudservices and iam)\n    if (actor.startsWith(\"serviceAccount:\") || actor.contains(\".gserviceaccount.com\")) {\n      addNestedValue(ctx, \"service.entity.id\", actor);\n    }\n    // Service account paths: projects/*/serviceAccounts/*\n    else if (actor.contains(\"/serviceAccounts\")) {\n      addNestedValue(ctx, \"service.entity.id\", actor);\n    }\n    // User accounts: user:* or email addresses (not service accounts)\n    else if (actor.startsWith(\"user:\") || (actor.contains(\"@\") && !actor.contains(\".gserviceaccount.com\"))) {\n      addNestedValue(ctx, \"user.entity.id\", actor);\n    }\n    // Compute instances: projects/*/zones/*/instances/*\n    else if (actor.contains(\"/instances\")) {\n      addNestedValue(ctx, \"host.entity.id\", actor);\n    }\n    // GCP service or API names: *.googleapis.com (but not principal:// or workloadIdentityPools)\n    else if (actor.contains(\".googleapis.com\") && !actor.startsWith(\"principal://\") && !actor.contains(\"/workloadIdentityPools\")) {\n      addNestedValue(ctx, \"service.entity.id\", actor);\n    }\n    // Everything else (repo:, workloadIdentityPools, principal://, etc.)\n    else {\n      addNestedValue(ctx, \"entity.id\", actor);\n    }\n  }\n}\n\n// Classify target entities\nif (ctx.target?.entity?.id instanceof List) {\n  for (def targetId : ctx.target.entity.id) {\n    if (targetId == null || targetId == \"\") {\n      continue;\n    }\n    String target = targetId.toString();\n    \n    // Compute instances\n    if (target.contains(\"/instances\")) {\n      addNestedValue(ctx, \"host.target.entity.id\", target);\n    }\n    // Service accounts: projects/*/serviceAccounts/* or email addresses ending with .gserviceaccount.com\n    else if (target.contains(\"/serviceAccounts\") || \n             (target.contains(\"@\") && target.contains(\".gserviceaccount.com\"))) {\n      addNestedValue(ctx, \"service.target.entity.id\", target);\n    }\n    // IAM principals (users, groups)\n    else if (target.contains(\"/users\") || \n             target.contains(\"/groups\") ||\n             (target.contains(\"@\") && target.contains(\"/members\"))) {\n      addNestedValue(ctx, \"user.target.entity.id\", target);\n    }\n    // GCP service resources\n    else if (target.contains(\"/machineTypes\") ||\n             target.contains(\"/deployments\") ||\n             target.contains(\"/services\") ||\n             target.contains(\"/policies\") ||\n             target.contains(\"/operations\") ||\n             target.contains(\"/subnetworks\") ||\n             target.contains(\"/networkEdgeSecurityServices\") ||\n             target.contains(\"/functions\") ||\n             target.contains(\"/buckets\") ||\n             target.contains(\"/datasets\") ||\n             target.contains(\"/tables\") ||\n             target.contains(\"/disks\") ||\n             target.contains(\"/networks\") ||\n             target.contains(\"/firewalls\") ||\n             target.contains(\"/backendServices\") ||\n             target.contains(\"/forwardingRules\") ||\n             target.contains(\"/healthChecks\") ||\n             target.contains(\"/sslCertificates\") ||\n             target.contains(\"/targetPools\") ||\n             target.contains(\"/urlMaps\") ||\n             target.contains(\".googleapis.com\")) {\n      addNestedValue(ctx, \"service.target.entity.id\", target);\n    }\n    // Everything else (projects, zones, workloadIdentityPools, etc.)\n    else {\n      addNestedValue(ctx, \"entity.target.id\", target);\n    }\n  }\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: void addValue(Set entities, def value) {\n  if (value != null && value != \"\") {\n    entities.add(value);\n  }\n}\n\nboolean isKubernetes = false;\nif (ctx.json?.resource?.type != null) {\n  String typ = ctx.json.resource.type;\n  isKubernetes = (typ == \"k8s_cluster\" || typ == \"gke_cluster\" || typ == \"kubernetes\");\n}\n\n// Using tree set to ensure a sorting is kept (testing purposes)\nTreeSet entities = new TreeSet();\n\naddValue(entities, ctx.json?.protoPayload?.request?.parent);\nif (!isKubernetes) {\n  addValue(entities, ctx.json?.protoPayload?.resourceName);\n  addValue(entities, ctx.json?.protoPayload?.response?.user);\n}\n\nHashMap authInfo = ctx.json?.protoPayload?.authenticationInfo ?: new HashMap();\nif (!isKubernetes) {\n  addValue(entities, authInfo.principalEmail);\n}\naddValue(entities, authInfo.principalSubject);\naddValue(entities, authInfo.serviceAccountKeyName);\nif (authInfo.serviceAccountDelegationInfo instanceof List) {\n  for (def i: authInfo.serviceAccountDelegationInfo) {\n    addValue(entities, i.principalSubject);\n    addValue(entities, i.firstPartyPrincipal?.principalEmail);\n    addValue(entities, i.thirdPartyPrincipal?.principalEmail);\n  }\n}\n\nString serviceName = ctx.json?.protoPayload?.serviceName ?: '';\nif (serviceName == \"compute.googleapis.com\") {\n  if (ctx.json?.protoPayload?.request?.networkInterfaces instanceof List) {\n    for (def e: ctx.json.protoPayload.request.networkInterfaces) {\n      addValue(entities, e.network);\n    }\n  }\n  if (ctx.json?.protoPayload?.request?.serviceAccounts instanceof List) {\n    for (def e: ctx.json.protoPayload.request.serviceAccounts) {\n      addValue(entities, e.email);\n    }\n  }\n  if (ctx.json?.protoPayload?.request?.disks instanceof List) {\n    for (def e: ctx.json.protoPayload.request.disks) {\n      addValue(entities, e.source);\n    }\n  }\n} else if (serviceName == \"cloudresourcemanager.googleapis.com\") {\n  if (ctx.json?.protoPayload?.request?.policy?.bindings instanceof List) {\n    for (def e: ctx.json.protoPayload.request.policy.bindings) {\n      addValue(entities, e.role);\n      for (def m: e.members) {\n        addValue(entities, m);\n      }\n    }\n  }\n  if (ctx.json?.protoPayload?.response?.bindings instanceof List) {\n    for (def e: ctx.json.protoPayload.response.bindings) {\n      addValue(entities, e.role);\n      for (def m: e.members) {\n        addValue(entities, m);\n      }\n    }\n  }\n} else if (serviceName == \"iamcredentials.googleapis.com\") {\n  if (ctx.json?.protoPayload?.metadata?.identityDelegationChain instanceof List) {\n    for (def e: ctx.json.protoPayload.metadata.identityDelegationChain) {\n      addValue(entities, e);\n    }\n  }\n}\n\nif (entities.size() > 0) {\n  ctx.related = ctx.related ?: [:];\n  ctx.related.entity = entities;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"void addValue(Set entities, def value) {\n  if (value != null && value != \"\") {\n    entities.add(value);\n  }\n}\n\nboolean isKubernetes = false;\nif (ctx.json?.resource?.type != null) {\n  String typ = ctx.json.resource.type;\n  isKubernetes = (typ == \"k8s_cluster\" || typ == \"gke_cluster\" || typ == \"kubernetes\");\n}\n\n// Using tree set to ensure a sorting is kept (testing purposes)\nTreeSet entities = new TreeSet();\n\naddValue(entities, ctx.json?.protoPayload?.request?.parent);\nif (!isKubernetes) {\n  addValue(entities, ctx.json?.protoPayload?.resourceName);\n  addValue(entities, ctx.json?.protoPayload?.response?.user);\n}\n\nHashMap authInfo = ctx.json?.protoPayload?.authenticationInfo ?: new HashMap();\nif (!isKubernetes) {\n  addValue(entities, authInfo.principalEmail);\n}\naddValue(entities, authInfo.principalSubject);\naddValue(entities, authInfo.serviceAccountKeyName);\nif (authInfo.serviceAccountDelegationInfo instanceof List) {\n  for (def i: authInfo.serviceAccountDelegationInfo) {\n    addValue(entities, i.principalSubject);\n    addValue(entities, i.firstPartyPrincipal?.principalEmail);\n    addValue(entities, i.thirdPartyPrincipal?.principalEmail);\n  }\n}\n\nString serviceName = ctx.json?.protoPayload?.serviceName ?: '';\nif (serviceName == \"compute.googleapis.com\") {\n  if (ctx.json?.protoPayload?.request?.networkInterfaces instanceof List) {\n    for (def e: ctx.json.protoPayload.request.networkInterfaces) {\n      addValue(entities, e.network);\n    }\n  }\n  if (ctx.json?.protoPayload?.request?.serviceAccounts instanceof List) {\n    for (def e: ctx.json.protoPayload.request.serviceAccounts) {\n      addValue(entities, e.email);\n    }\n  }\n  if (ctx.json?.protoPayload?.request?.disks instanceof List) {\n    for (def e: ctx.json.protoPayload.request.disks) {\n      addValue(entities, e.source);\n    }\n  }\n} else if (serviceName == \"cloudresourcemanager.googleapis.com\") {\n  if (ctx.json?.protoPayload?.request?.policy?.bindings instanceof List) {\n    for (def e: ctx.json.protoPayload.request.policy.bindings) {\n      addValue(entities, e.role);\n      for (def m: e.members) {\n        addValue(entities, m);\n      }\n    }\n  }\n  if (ctx.json?.protoPayload?.response?.bindings instanceof List) {\n    for (def e: ctx.json.protoPayload.response.bindings) {\n      addValue(entities, e.role);\n      for (def m: e.members) {\n        addValue(entities, m);\n      }\n    }\n  }\n} else if (serviceName == \"iamcredentials.googleapis.com\") {\n  if (ctx.json?.protoPayload?.metadata?.identityDelegationChain instanceof List) {\n    for (def e: ctx.json.protoPayload.metadata.identityDelegationChain) {\n      addValue(entities, e);\n    }\n  }\n}\n\nif (entities.size() > 0) {\n  ctx.related = ctx.related ?: [:];\n  ctx.related.entity = entities;\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.authoritySelector",
                    "gcp.audit.authentication_info.authority_selector",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.principalEmail",
                    "gcp.audit.authentication_info.principal_email",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.principalSubject",
                    "gcp.audit.authentication_info.principal_subject",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.serviceAccountKeyName",
                    "gcp.audit.authentication_info.service_account_key_name",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.serviceAccountDelegationInfo",
                    "gcp.audit.authentication_info.service_account_delegation_info",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.authenticationInfo.thirdPartyPrincipal",
                    "gcp.audit.authentication_info.third_party_principal",
                )?;
                Ok(())
            })();

            let _cond = { !event.has_value("client.user.email") };
            if _cond {
                if event.has("gcp.audit.authentication_info.principal_email") {
                    event.rename(
                        "gcp.audit.authentication_info.principal_email",
                        "client.user.email",
                    )?;
                }
            }

            let _cond = { !event.has_value("client.user.id") };
            if _cond {
                if event.has("gcp.audit.authentication_info.principal_subject") {
                    event.rename(
                        "gcp.audit.authentication_info.principal_subject",
                        "client.user.id",
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.protoPayload.authorizationInfo").cloned() {
                    event.set("gcp.audit.authorization_info", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("gcp.audit.authorization_info")
                    && event
                        .get("gcp.audit.authorization_info")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("gcp.audit.authorization_info") {
                        if let Some(Value::Array(items)) =
                            event.get("gcp.audit.authorization_info").cloned()
                        {
                            let mut out = Vec::with_capacity(items.len());
                            for item in items {
                                event.set("_ingest._value", item)?;
                                event.rename(
                                    "_ingest._value.resourceAttributes",
                                    "_ingest._value.resource_attributes",
                                )?;
                                out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                            }
                            event.remove("_ingest");
                            event.set("gcp.audit.authorization_info", Value::Array(out))?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.labels") };
            if _cond {
                if let Some(v) = event.get("json.labels").cloned() {
                    event.set("gcp.audit.labels", v)?;
                }
            }

            let _cond = { event.has_value("labels.payload") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(s) = event.get_string("labels.payload") {
                        let parsed: Value =
                            serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                                path: "labels.payload".into(),
                                message: format!("failed to parse JSON: {}", e),
                            })?;
                        event.set("labels.payload", parsed)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_labels_payload")?;
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

            let _cond =
                { !event.has_value("cloud.project.id") && event.has_value("labels.project_id") };
            if _cond {
                if let Some(v) = event
                    .get("labels.project_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("cloud.project.id") {
                        event.set("cloud.project.id", v)?;
                    }
                }
            }

            let _cond =
                { !event.has_value("orchestrator.type") && event.has_value("labels.cluster_name") };
            if _cond {
                if !event.has("orchestrator.type") {
                    event.set("orchestrator.type", json!("kubernetes"))?;
                }
            }

            let _cond = {
                !event.has_value("orchestrator.cluster.name")
                    && event.has_value("labels.cluster_name")
            };
            if _cond {
                if let Some(v) = event
                    .get("labels.cluster_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("orchestrator.cluster.name") {
                        event.set("orchestrator.cluster.name", v)?;
                    }
                }
            }

            let _cond = {
                !event.has_value("orchestrator.resource.type")
                    && event.has_value("labels.payload.resourceType")
            };
            if _cond {
                if let Some(v) = event
                    .get("labels.payload.resourceType")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("orchestrator.resource.type") {
                        event.set("orchestrator.resource.type", v)?;
                    }
                }
            }

            let _cond = {
                !event.has_value("orchestrator.resource.name")
                    && event.has_value("labels.payload.resource")
            };
            if _cond {
                if let Some(v) = event
                    .get("labels.payload.resource")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("orchestrator.resource.name") {
                        event.set("orchestrator.resource.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("json.protoPayload.requestMetadata.callerIp")
                    && event.get_str("json.protoPayload.requestMetadata.callerIp")
                        != Some("gce-internal-ip")
                    && event.get_str("json.protoPayload.requestMetadata.callerIp")
                        != Some("private")
            };
            if _cond {
                if event.has_value("json.protoPayload.requestMetadata.callerIp") {
                    if let Some(val) = event.get("json.protoPayload.requestMetadata.callerIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.protoPayload.requestMetadata.callerIp".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("json.protoPayload.requestMetadata.callerSuppliedUserAgent")
                    .cloned()
                {
                    event.set("user_agent.original", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("json.protoPayload.metadata") };
            if _cond {
                if let Some(v) = event.get("json.protoPayload.metadata").cloned() {
                    event.set("gcp.audit.metadata", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.violationInfo").cloned() {
                event.set("gcp.audit.policy_violation_info.violations", v)?;
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.payload",
                    "gcp.audit.policy_violation_info.payload",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.resourceType",
                    "gcp.audit.policy_violation_info.resource_type",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.resourceTags",
                    "gcp.audit.policy_violation_info.resource_tags",
                )?;
                Ok(())
            })();

            let _cond = {
                event.has_value("json.operation.id")
                    && event.has_value("event.id")
                    && event.get("event.id").filter(|v| !v.is_null())
                        != event.get("json.operation.id").filter(|v| !v.is_null())
            };
            if _cond {
                if let Some(v) = event.get("json.operation.id").cloned() {
                    event.set("gcp.audit.logentry_operation.id", v)?;
                }
            }

            let _cond = { event.has_value("json.operation") };
            if _cond {
                // Painless script
                // Source: def first = (ctx.json.operation.first == null) ? false : ctx.json.operation.first;\ndef last = (ctx.json.operation.last == null) ? false : ctx.json.operation.last;\nif (first && last) {\n  return;\n}\nif (ctx.event.category == null) {\n  ctx.event.category = new ArrayList();\n}\nif (ctx.event.type == null) {\n  ctx.event.type = new ArrayList();\n}\nctx.event.category.add('session');\nif (first == true && last == false) {\n  ctx.event.type.add('start');\n}\nif (first == false && last == true) {\n  ctx.event.type.add('end');\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"def first = (ctx.json.operation.first == null) ? false : ctx.json.operation.first;\ndef last = (ctx.json.operation.last == null) ? false : ctx.json.operation.last;\nif (first && last) {\n  return;\n}\nif (ctx.event.category == null) {\n  ctx.event.category = new ArrayList();\n}\nif (ctx.event.type == null) {\n  ctx.event.type = new ArrayList();\n}\nctx.event.category.add('session');\nif (first == true && last == false) {\n  ctx.event.type.add('start');\n}\nif (first == false && last == true) {\n  ctx.event.type.add('end');\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.protoPayload.serviceData.policyDelta.bindingDeltas")
                    && !(event
                        .get("json.protoPayload.serviceData.policyDelta.bindingDeltas")
                        .is_none_or(|v| match v {
                            serde_json::Value::String(s) => s.is_empty(),
                            serde_json::Value::Array(a) => a.is_empty(),
                            serde_json::Value::Object(o) => o.is_empty(),
                            serde_json::Value::Null => true,
                            _ => false,
                        }))
            };
            if _cond {
                if let Some(v) = event
                    .get("json.protoPayload.serviceData.policyDelta.bindingDeltas")
                    .cloned()
                {
                    event.set("gcp.audit.service_data.policy_delta.binding_deltas", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.protoPayload.methodName").cloned() {
                    event.set("event.action", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.protoPayload.numResponseItems") {
                if let Some(val) = event.get("json.protoPayload.numResponseItems") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.protoPayload.numResponseItems".into(),
                            message,
                        }
                    })?;
                    event.set("gcp.audit.num_response_items", converted)?;
                }
            }

            let _cond = { event.has_value("json.protoPayload.request") };
            if _cond {
                if let Some(v) = event.get("json.protoPayload.request").cloned() {
                    event.set("gcp.audit.request", v)?;
                }
            }

            let _cond = {
                event.has_value("json.protoPayload.request.policy")
                    && !(event
                        .get("json.protoPayload.request.policy")
                        .is_some_and(|v| v.is_object()))
            };
            if _cond {
                event.remove("gcp.audit.request.policy");
            }

            let _cond = {
                event.has_value("json.protoPayload.request.policy")
                    && !(event
                        .get("json.protoPayload.request.policy")
                        .is_some_and(|v| v.is_object()))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.protoPayload.request.policy").cloned() {
                        event.set("gcp.audit.request.policy_value", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.protoPayload.response") };
            if _cond {
                if let Some(v) = event.get("json.protoPayload.response").cloned() {
                    event.set("gcp.audit.response", v)?;
                }
            }

            let _cond = {
                event.has_value("json.protoPayload.response.status")
                    && !(event
                        .get("json.protoPayload.response.status")
                        .is_some_and(|v| v.is_object()))
            };
            if _cond {
                event.remove("gcp.audit.response.status");
            }

            let _cond = {
                event.has_value("json.protoPayload.response.status")
                    && !(event
                        .get("json.protoPayload.response.status")
                        .is_some_and(|v| v.is_object()))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("json.protoPayload.response.status").cloned() {
                        event.set("gcp.audit.response.status_value", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.protoPayload.resourceName").cloned() {
                    event.set("gcp.audit.resource_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("json.protoPayload.resourceLocation.currentLocations")
                    .cloned()
                {
                    event.set("gcp.audit.resource_location.current_locations", v)?;
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("json.resource.labels.resource_container")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("gcp.audit.resource.labels.resource_container", v)?;
            }

            if let Some(v) = event
                .get("json.resource.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("gcp.audit.resource.type", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.protoPayload.serviceName").cloned() {
                    event.set("gcp.audit.service_name", v)?;
                }
                Ok(())
            })();

            let _cond = { !event.has_value("service.name") };
            if _cond {
                if event.has("gcp.audit.service_name") {
                    event.rename("gcp.audit.service_name", "service.name")?;
                }
            }

            let _cond = {
                event.has_value("json.receiveTimestamp")
                    && event.get_str("json.receiveTimestamp") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.receiveTimestamp") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("gcp.audit.receive_timestamp", parsed)?;
                    }
                }
            }

            let _cond = { event.get_str("json.jsonPayload.access.callerIp") != Some("") };
            if _cond {
                if event.has_value("json.jsonPayload.access.callerIp") {
                    if let Some(val) = event.get("json.jsonPayload.access.callerIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.jsonPayload.access.callerIp".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
            }

            if event.has("json.jsonPayload.access.callerIpGeo.regionCode") {
                event.rename(
                    "json.jsonPayload.access.callerIpGeo.regionCode",
                    "gcp.audit.access.caller_ip_geo.region_code",
                )?;
            }

            if event.has("json.jsonPayload.access.methodName") {
                event.rename(
                    "json.jsonPayload.access.methodName",
                    "gcp.audit.access.method_name",
                )?;
            }

            if let Some(v) = event
                .get("gcp.audit.access.method_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("event.action") {
                    event.set("event.action", v)?;
                }
            }

            if event.has("json.jsonPayload.access.principalEmail") {
                event.rename(
                    "json.jsonPayload.access.principalEmail",
                    "gcp.audit.access.principal_email",
                )?;
            }

            if let Some(v) = event
                .get("gcp.audit.access.principal_email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("client.user.email") {
                    event.set("client.user.email", v)?;
                }
            }

            if event.has("json.jsonPayload.access.principalSubject") {
                event.rename(
                    "json.jsonPayload.access.principalSubject",
                    "gcp.audit.access.principal_subject",
                )?;
            }

            if event.has("json.jsonPayload.access.serviceName") {
                event.rename(
                    "json.jsonPayload.access.serviceName",
                    "gcp.audit.access.service_name",
                )?;
            }

            if let Some(v) = event
                .get("gcp.audit.access.service_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("service.name") {
                    event.set("service.name", v)?;
                }
            }

            if event.has("json.jsonPayload.access.userAgent") {
                event.rename(
                    "json.jsonPayload.access.userAgent",
                    "gcp.audit.access.user_agent",
                )?;
            }

            if let Some(v) = event
                .get("gcp.audit.access.user_agent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("user_agent.original") {
                    event.set("user_agent.original", v)?;
                }
            }

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

            let _cond = {
                event.has_value("json.jsonPayload.actionTime")
                    && event.get_str("json.jsonPayload.actionTime") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.jsonPayload.actionTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("gcp.audit.action_time", parsed)?;
                    }
                }
            }

            if event.has("json.jsonPayload.actionType") {
                event.rename("json.jsonPayload.actionType", "gcp.audit.action_type")?;
            }

            if event.has("json.jsonPayload.affectedResources") {
                event.rename(
                    "json.jsonPayload.affectedResources",
                    "gcp.audit.affected_resources",
                )?;
            }

            if event.has("json.jsonPayload.learnMoreUri") {
                event.rename("json.jsonPayload.learnMoreUri", "gcp.audit.learn_more_uri")?;
            }

            if event.has("json.jsonPayload.sourceLogIds") {
                event.rename("json.jsonPayload.sourceLogIds", "gcp.audit.source_log_ids")?;
            }

            let _cond = {
                event
                    .get("gcp.audit.source_log_ids")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("gcp.audit.source_log_ids") {
                    if let Some(Value::Array(items)) =
                        event.get("gcp.audit.source_log_ids").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.insertId") {
                                event.rename(
                                    "_ingest._value.insertId",
                                    "_ingest._value.insert_id",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("gcp.audit.source_log_ids", Value::Array(out))?;
                    }
                }
            }

            let _cond = {
                event
                    .get("gcp.audit.source_log_ids")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("gcp.audit.source_log_ids") {
                    if let Some(Value::Array(items)) =
                        event.get("gcp.audit.source_log_ids").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.queryUri") {
                                event.rename(
                                    "_ingest._value.queryUri",
                                    "_ingest._value.query_uri",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("gcp.audit.source_log_ids", Value::Array(out))?;
                    }
                }
            }

            let _cond = {
                event
                    .get("gcp.audit.source_log_ids")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("gcp.audit.source_log_ids") {
                    if let Some(Value::Array(items)) =
                        event.get("gcp.audit.source_log_ids").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has("_ingest._value.resourceContainer") {
                                event.rename(
                                    "_ingest._value.resourceContainer",
                                    "_ingest._value.resource_container",
                                )?;
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("gcp.audit.source_log_ids", Value::Array(out))?;
                    }
                }
            }

            let _cond = {
                event
                    .get("gcp.audit.source_log_ids")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("gcp.audit.source_log_ids") {
                    if let Some(Value::Array(items)) =
                        event.get("gcp.audit.source_log_ids").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.logTime")
                                {
                                    if let Some(parsed) =
                                        parse_date_out(&date_str, &["ISO8601"], None, None)
                                    {
                                        event.set("_ingest._value.log_time", parsed)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.remove("_ingest._value.logTime");
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
                        event.set("gcp.audit.source_log_ids", Value::Array(out))?;
                    }
                }
            }

            let _cond = {
                event
                    .get("gcp.audit.source_log_ids")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("gcp.audit.source_log_ids") {
                    if let Some(Value::Array(items)) =
                        event.get("gcp.audit.source_log_ids").cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            event.remove("_ingest._value.logTime");
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("gcp.audit.source_log_ids", Value::Array(out))?;
                    }
                }
            }

            if event.has_value("json.protoPayload.status.code") {
                if let Some(val) = event.get("json.protoPayload.status.code") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.protoPayload.status.code".into(),
                            message,
                        }
                    })?;
                    event.set("gcp.audit.status.code", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.protoPayload.status.message").cloned() {
                    event.set("gcp.audit.status.message", v)?;
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("json.protoPayload.status.details")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("gcp.audit.status.details", v)?;
            }

            let _cond = {
                event.has_value("gcp.audit.status.code")
                    && event.get_i64("gcp.audit.status.code") == Some(0)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("gcp.audit.status.code")
                    && event.get_i64("gcp.audit.status.code") != Some(0)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            // SKIPPED: condition not transpiled: ctx?.gcp?.audit?.status?.code == null && ctx?.gcp?.audit?.authorization_info != null && ctx?.gcp?.audit?.authorization_info instanceof List && ctx?.gcp?.audit?.authorization_info.size() == 1 && ctx?.g ...
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("event.outcome", json!("success"))?;
            }

            // SKIPPED: condition not transpiled: ctx?.gcp?.audit?.status?.code == null && ctx?.gcp?.audit?.authorization_info != null && ctx?.gcp?.audit?.authorization_info instanceof List && ctx?.gcp?.audit?.authorization_info.size() == 1 && ctx?.g ...
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("event.outcome", json!("failure"))?;
            }

            if !event.has("event.outcome") {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = {
                event.has_value("gcp.audit.authorization_info") && event.get("gcp.audit.authorization_info").is_some_and(|v| v.is_array()) && event.get("gcp.audit.authorization_info").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 1)
            };
            if _cond {
                event.append("event.category", json!("network"))?;
                event.append("event.category", json!("configuration"))?;
            }

            // SKIPPED: condition not transpiled: ctx?.gcp?.audit?.authorization_info != null && ctx?.gcp?.audit?.authorization_info instanceof List && ctx?.gcp?.audit?.authorization_info.size() == 1 && ctx?.gcp?.audit?.authorization_info[0]?.granted ...
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.append("event.type", json!("access"))?;
                event.append("event.type", json!("allowed"))?;
            }

            // SKIPPED: condition not transpiled: ctx?.gcp?.audit?.authorization_info != null && ctx?.gcp?.audit?.authorization_info instanceof List && ctx?.gcp?.audit?.authorization_info.size() == 1 && ctx?.gcp?.audit?.authorization_info[0]?.granted ...
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.append("event.type", json!("access"))?;
                event.append("event.type", json!("denied"))?;
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

            if let Some(v) = event
                .get("client.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            if let Some(v) = event
                .get("client.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("source.ip")
                        .map_or_else(String::new, painless_to_string)
                ),
            )?;

            let _cond = { event.has_value("client.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("client.user.email")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("json") && event.get_bool("_conf.keep_json") == Some(true) };
            if _cond {
                event.rename("json", "gcp.audit.flattened")?;
            }

            event.remove("_conf");
            event.remove("_temp");
            event.remove("json");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    def m = ((Map) o);\n    def it = m.entrySet().iterator();\n    while (it.hasNext()) {\n      def e = ((Map.Entry) it.next());\n      def key = ((String) e.getKey());\n      def value = e.getValue();\n      Pattern onlyDotsRegex = /^\\.+$/;\n      if (onlyDotsRegex.matcher(key).matches() || drop(value)) {\n        it.remove();\n      }\n    }\n    return (m.size() == 0);\n  } else if (o instanceof List) {\n    def l = ((List) o);\n    l.removeIf(v -> drop(v));\n    return (l.length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    def m = ((Map) o);\n    def it = m.entrySet().iterator();\n    while (it.hasNext()) {\n      def e = ((Map.Entry) it.next());\n      def key = ((String) e.getKey());\n      def value = e.getValue();\n      Pattern onlyDotsRegex = /^\\.+$/;\n      if (onlyDotsRegex.matcher(key).matches() || drop(value)) {\n        it.remove();\n      }\n    }\n    return (m.size() == 0);\n  } else if (o instanceof List) {\n    def l = ((List) o);\n    l.removeIf(v -> drop(v));\n    return (l.length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
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
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
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
