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

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("both")),
                    serde_json::Value::String(s) => s.contains("both"),
                    _ => false,
                })
            };
            if _cond {
                event.append("tags", json!("elastic_cloud_data"))?;
                event.append("tags", json!("provider_cloud_data"))?;
            }

            // Painless script
            // Source: if (ctx.tags != null) {\n  if (ctx.tags.contains('both')) {\n    ctx.tags.remove(ctx.tags.indexOf('both'));\n  }\n  if (ctx.cloud == null && ctx.tags.contains('elastic_cloud_data')) {\n    ctx.tags.remove(ctx.tags.indexOf('elastic_cloud_data'));\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.tags != null) {\n  if (ctx.tags.contains('both')) {\n    ctx.tags.remove(ctx.tags.indexOf('both'));\n  }\n  if (ctx.cloud == null && ctx.tags.contains('elastic_cloud_data')) {\n    ctx.tags.remove(ctx.tags.indexOf('elastic_cloud_data'));\n  }\n}\n"#
                ),
            )?;

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("provider_cloud_data"))
                    }
                    serde_json::Value::String(s) => s.contains("provider_cloud_data"),
                    _ => false,
                })
            };
            if _cond {
                event.set("_conf.want_provider_cloud", json!(true))?;
            }

            let _cond = {
                event.has_value("tags")
                    && !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("elastic_cloud_data"))
                        }
                        serde_json::Value::String(s) => s.contains("elastic_cloud_data"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("cloud");
                    Ok(())
                })();
            }

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

            parse_json_field(event, "event.original", "teleport.audit")?;

            let _cond = { event.has_value("teleport.audit.time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("teleport.audit.time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "teleport.audit.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.remove("teleport.audit.time");

            if event.has_value("teleport.audit.event") {
                event.rename("teleport.audit.event", "event.action")?;
            }

            if event.has_value("teleport.audit.code") {
                event.rename("teleport.audit.code", "event.code")?;
            }

            if event.has_value("teleport.audit.uid") {
                event.rename("teleport.audit.uid", "event.id")?;
            }

            if event.has_value("teleport.audit.ei") {
                event.rename("teleport.audit.ei", "event.sequence")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Begin nested pipeline: "event-categories"
                event.set("event.kind", json!("event"))?;
                let _cond = {
                    event
                        .get_str("event.code")
                        .is_some_and(|s| s.ends_with("E"))
                        || event
                            .get_str("event.code")
                            .is_some_and(|s| s.ends_with("W"))
                };
                if _cond {
                    event.set("event.outcome", Value::Array(vec![json!("failure")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("access_list."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("configuration"), json!("iam")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("access_list.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("access_list.delete") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("access_list.member.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("access_list.member.delete") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("access_list.member.delete_all_members")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("access_list.member.update") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("access_list.review") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("access_list.update") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("access_request."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("session"), json!("iam")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("access_request.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("access_request.delete") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("access_request.review") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("access_request.search") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("access_request.update") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("app."))
                        && !(event
                            .get_str("event.action")
                            .is_some_and(|s| s.starts_with("app.session.")))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("process"), json!("configuration")]),
                    )?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("app.session."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("session")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("app.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("app.delete") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("app.session.chunk")
                        || event.get_str("event.action") == Some("app.session.dynamodb.request")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("app.session.end") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("app.session.start") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("app.update") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("auth"))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("auth") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("auth_preference.update") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("billing."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("configuration"), json!("iam")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("billing.create_card") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("billing.delete_card") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("billing.update_card")
                        || event.get_str("event.action") == Some("billing.update_info")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("bot."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("process"), json!("host")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("bot.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("bot.delete") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("bot.join")
                        || event.get_str("event.action") == Some("bot.update")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("cert."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("configuration"), json!("iam")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("cert.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("client."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("network")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("client.disconnect") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("cluster_networking_config."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("configuration")]))?;
                }
                let _cond =
                    { event.get_str("event.action") == Some("cluster_networking_config.update") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("db."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("database")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("db.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("db.delete") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("db.session.cassandra.batch")
                        || event.get_str("event.action") == Some("db.session.cassandra.execute")
                        || event.get_str("event.action") == Some("db.session.cassandra.prepare")
                        || event.get_str("event.action") == Some("db.session.cassandra.register")
                        || event.get_str("event.action") == Some("db.session.dynamodb.request")
                        || event.get_str("event.action") == Some("db.session.elasticsearch.request")
                        || event.get_str("event.action") == Some("db.session.opensearch.request")
                        || event.get_str("event.action") == Some("db.session.permissions.update")
                        || event.get_str("event.action") == Some("db.session.postgres.function")
                        || event.get_str("event.action")
                            == Some("db.session.postgres.statements.bind")
                        || event.get_str("event.action")
                            == Some("db.session.postgres.statements.close")
                        || event.get_str("event.action")
                            == Some("db.session.postgres.statements.execute")
                        || event.get_str("event.action")
                            == Some("db.session.postgres.statements.parse")
                        || event.get_str("event.action") == Some("db.session.query")
                        || event.get_str("event.action") == Some("db.session.spanner.rpc")
                        || event.get_str("event.action") == Some("db.session.sqlserver.rpc_request")
                        || event.get_str("event.action") == Some("db.session.start")
                        || event.get_str("event.action") == Some("db.session.user.create")
                        || event.get_str("event.action") == Some("db.session.user.deactivate")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("db.session.end") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("db.session.malformed_packet")
                        || event.get_str("event.action") == Some("db.session.query.failed")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("error")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("db.session.start") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("db.update") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("desktop."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("host"), json!("file")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("desktop.clipboard.receive")
                        || event.get_str("event.action") == Some("desktop.clipboard.send")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("desktop.directory.read")
                        || event.get_str("event.action") == Some("desktop.directory.share")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("access")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("desktop.directory.write") };
                if _cond {
                    event.set(
                        "event.type",
                        Value::Array(vec![json!("creation"), json!("deletion")]),
                    )?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("device."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("configuration"), json!("iam")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("device.authenticate.confirm")
                        || event.get_str("event.action") == Some("device.authenticate")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("device.create")
                        || event.get_str("event.action") == Some("device.token.create")
                        || event.get_str("event.action") == Some("device.webtoken.create")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("device.delete") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("device.enroll")
                        || event.get_str("event.action") == Some("device.update")
                        || event.get_str("event.action") == Some("device.token.spent")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("exec"))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("process"), json!("host")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("exec") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("external_audit_storage."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("configuration")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("external_audit_storage.disable")
                        || event.get_str("event.action") == Some("external_audit_storage.enable")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("github."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication"), json!("iam")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("github.created") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("github.deleted") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("github.updated") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("instance."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("network"), json!("host")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("instance.join") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("join_token."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("session"), json!("iam")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("join_token.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("kube."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("process"), json!("host")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("kube.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("kube.delete") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("kube.request") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("kube.update") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("lock."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("configuration"), json!("iam")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("lock.created") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("lock.deleted") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("login_rule."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication"), json!("iam")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("login_rule.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("login_rule.delete") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("mfa."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication"), json!("iam")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("mfa.add") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("mfa.delete") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("mfa_auth_challenge.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond =
                    { event.get_str("event.action") == Some("mfa_auth_challenge.validate") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("oidc."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication"), json!("iam")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("oidc.created") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("oidc.deleted") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("oidc.updated") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("okta."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("iam")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("okta.assignment"))
                        || event
                            .get_str("event.action")
                            .is_some_and(|s| s.starts_with("okta."))
                            && event
                                .get_str("event.action")
                                .is_some_and(|s| s.ends_with(".update"))
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("okta.sync")
                        || event.get_str("event.action") == Some("okta.access_list.sync")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("port"))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("network")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("port") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("privilege_token."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("session"), json!("iam")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("privilege_token.create") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("recovery_"))
                        || event
                            .get_str("event.action")
                            .is_some_and(|s| s.starts_with("reset_password_token."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("session"), json!("iam")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("recovery_code.generated")
                        || event.get_str("event.action") == Some("recovery_token.create")
                        || event.get_str("event.action") == Some("reset_password_token.create")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("recovery_code.used") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("resize") };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("process")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("resize") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("role."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("iam")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("role.created") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("role.deleted") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("role.updated") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("saml."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication"), json!("iam")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("saml.created")
                        || event.get_str("event.action") == Some("saml.idp.service.provider.create")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("saml.deleted")
                        || event.get_str("event.action") == Some("saml.idp.service.provider.delete")
                        || event.get_str("event.action")
                            == Some("saml.idp.service.provider.delete_all")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("saml.updated")
                        || event.get_str("event.action") == Some("saml.idp.service.provider.update")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("saml.idp.auth") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("scp."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("file")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("secreports."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("threat")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("secreports.audit.query.run")
                        || event.get_str("event.action") == Some("secreports.report.run")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("indicator")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("session."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("session")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("session.start") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("session.end") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("session.connect")
                        || event.get_str("event.action") == Some("session.command")
                        || event.get_str("event.action") == Some("session.data")
                        || event.get_str("event.action") == Some("session.disk")
                        || event.get_str("event.action") == Some("session.join")
                        || event.get_str("event.action") == Some("session.network")
                        || event.get_str("event.action") == Some("session.leave")
                        || event.get_str("event.action") == Some("session.process_exit")
                        || event.get_str("event.action") == Some("session.recording.access")
                        || event.get_str("event.action") == Some("session.upload")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("session.rejected") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("denied")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("session_recording_config."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("configuration")]))?;
                }
                let _cond =
                    { event.get_str("event.action") == Some("session_recording_config.update") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("sftp"))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("file"), json!("network")]),
                    )?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("spiffe."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("iam"), json!("process")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("spiffe.svid.issued") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("ssm."))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("process")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("ssm.run") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("subsystem") };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("process")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("subsystem") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("trusted_cluster."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("network"), json!("host")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("trusted_cluster.create")
                        || event.get_str("event.action") == Some("trusted_cluster_token.create")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("creation")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("trusted_cluster.delete") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("user."))
                        && event.get_str("event.action") != Some("user.login")
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("iam")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("user.login") };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("user.create") };
                if _cond {
                    event.set(
                        "event.type",
                        Value::Array(vec![json!("creation"), json!("user")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("user.delete") };
                if _cond {
                    event.set(
                        "event.type",
                        Value::Array(vec![json!("deletion"), json!("user")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("user.password_change")
                        || event.get_str("event.action") == Some("user.update")
                };
                if _cond {
                    event.set(
                        "event.type",
                        Value::Array(vec![json!("change"), json!("user")]),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("user.login") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("windows."))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("host"), json!("session")]),
                    )?;
                }
                let _cond =
                    { event.get_str("event.action") == Some("windows.desktop.session.start") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond =
                    { event.get_str("event.action") == Some("windows.desktop.session.end") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("x11-forward") };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("network")]))?;
                }
                let _cond = { event.get_str("event.action") == Some("x11-forward") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                // End nested pipeline: "event-categories"
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("teleport.audit.kubernetes_labels") {
                    event.rename(
                        "teleport.audit.kubernetes_labels",
                        "teleport.audit.kubernetes.labels",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("teleport.audit.kube_labels") {
                    event.rename(
                        "teleport.audit.kube_labels",
                        "teleport.audit.kubernetes.labels",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("teleport.audit.server_labels") {
                    event.rename(
                        "teleport.audit.server_labels",
                        "teleport.audit.server.labels",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Begin nested pipeline: "event-groups"
                if event.has_value("teleport.audit.sid") {
                    event.rename("teleport.audit.sid", "teleport.audit.session.id")?;
                }
                if event.has_value("teleport.audit.with_mfa") {
                    event.rename("teleport.audit.with_mfa", "teleport.audit.mfa_device.uuid")?;
                }
                if event.has_value("teleport.audit.private_key_policy") {
                    event.rename(
                        "teleport.audit.private_key_policy",
                        "teleport.audit.session.private_key_policy",
                    )?;
                }
                event.remove("teleport.audit.user.user");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("teleport.audit.user") {
                        event.rename("teleport.audit.user", "user.name")?;
                    }
                    Ok(())
                })();
                if event.has_value("teleport.audit.login") {
                    event.rename("teleport.audit.login", "process.user.name")?;
                }
                if event.has_value("teleport.audit.impersonator") {
                    event.rename(
                        "teleport.audit.impersonator",
                        "teleport.audit.user.impersonator",
                    )?;
                }
                if event.has_value("teleport.audit.aws_role_arn") {
                    event.rename(
                        "teleport.audit.aws_role_arn",
                        "teleport.audit.user.aws_role_arn",
                    )?;
                }
                if event.has_value("teleport.audit.access_requests") {
                    event.rename(
                        "teleport.audit.access_requests",
                        "teleport.audit.user.access_requests",
                    )?;
                }
                if event.has_value("teleport.audit.azure_identity") {
                    event.rename(
                        "teleport.audit.azure_identity",
                        "teleport.audit.user.azure_identity",
                    )?;
                }
                if event.has_value("teleport.audit.gcp_service_account") {
                    event.rename(
                        "teleport.audit.gcp_service_account",
                        "teleport.audit.user.gcp_service_account",
                    )?;
                }
                if event.has_value("teleport.audit.trusted_device") {
                    event.rename(
                        "teleport.audit.trusted_device",
                        "teleport.audit.user.trusted_device",
                    )?;
                }
                if event.has_value("teleport.audit.required_private_key_policy") {
                    event.rename(
                        "teleport.audit.required_private_key_policy",
                        "teleport.audit.user.required_private_key_policy",
                    )?;
                }
                let _cond = { event.get_i64("teleport.audit.user_kind") == Some(0) };
                if _cond {
                    event.set("teleport.audit.user.kind", json!("unspecified"))?;
                }
                let _cond = { event.get_i64("teleport.audit.user_kind") == Some(1) };
                if _cond {
                    event.set("teleport.audit.user.kind", json!("human"))?;
                }
                let _cond = { event.get_i64("teleport.audit.user_kind") == Some(2) };
                if _cond {
                    event.set("teleport.audit.user.kind", json!("bot"))?;
                }
                let _cond = { event.has_value("teleport.audit.user.kind") };
                if _cond {
                    event.remove("teleport.audit.user_kind");
                }
                if event.has_value("teleport.audit.namespace") {
                    event.rename("teleport.audit.namespace", "group.name")?;
                }
                if event.has_value("teleport.audit.server_id") {
                    event.rename("teleport.audit.server_id", "host.id")?;
                }
                if event.has_value("teleport.audit.server_hostname") {
                    event.rename("teleport.audit.server_hostname", "host.hostname")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("teleport.audit.server_addr") {
                        if let Some(input) = event.get_string("teleport.audit.server_addr") {
                            // Grok pattern: ^(%{IPORHOST:server.address}|\\[%{IP:server.ip}\\])(:%{POSINT:server.port:long})?$
                            if !cached_grok!("^(%{IPORHOST:server.address}|\\[%{IP:server.ip}\\])(:%{POSINT:server.port:long})?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                    }
                    Ok(())
                })();
                event.remove("teleport.audit.server_addr");
                if event.has_value("teleport.audit.forwarded_by") {
                    event.rename(
                        "teleport.audit.forwarded_by",
                        "teleport.audit.server.forwarded_by",
                    )?;
                }
                if event.has_value("teleport.audit.server_sub_kind") {
                    event.rename(
                        "teleport.audit.server_sub_kind",
                        "teleport.audit.server.sub_kind",
                    )?;
                }
                if event.has_value("teleport.audit.server_version") {
                    event.rename(
                        "teleport.audit.server_version",
                        "teleport.audit.server.version",
                    )?;
                }
                let _cond = {
                    event
                        .get("teleport.audit.addr")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if event.has_value("teleport.audit.addr") {
                        event.rename("teleport.audit.addr", "_temp.teleport_audit_addr")?;
                    }
                }
                dot_expand(event, "teleport.audit", "addr.local")?;
                let _cond = { event.has_value("server.address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("teleport.audit.addr.local") {
                            if let Some(input) = event.get_string("teleport.audit.addr.local") {
                                // Grok pattern: ^(%{IPORHOST:destination.address}|\\[%{IP:destination.ip}\\])(:%{POSINT:destination.port:long})?$
                                if !cached_grok!("^(%{IPORHOST:destination.address}|\\[%{IP:destination.ip}\\])(:%{POSINT:destination.port:long})?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { !event.has_value("server.address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("teleport.audit.addr.local") {
                            if let Some(input) = event.get_string("teleport.audit.addr.local") {
                                // Grok pattern: ^(%{IPORHOST:server.address}|\\[%{IP:server.ip}\\])(:%{POSINT:server.port:long})?$
                                if !cached_grok!("^(%{IPORHOST:server.address}|\\[%{IP:server.ip}\\])(:%{POSINT:server.port:long})?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                            }
                        }
                        Ok(())
                    })();
                }
                event.remove("teleport.audit.addr.local");
                dot_expand(event, "teleport.audit", "addr.remote")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("teleport.audit.addr.remote") {
                        if let Some(input) = event.get_string("teleport.audit.addr.remote") {
                            // Grok pattern: ^(%{IPORHOST:client.address}|\\[%{IP:client.ip}\\])(:%{POSINT:client.port:long})?$
                            if !cached_grok!("^(%{IPORHOST:client.address}|\\[%{IP:client.ip}\\])(:%{POSINT:client.port:long})?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                    }
                    Ok(())
                })();
                event.remove("teleport.audit.addr.remote");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("teleport.audit.addr");
                    Ok(())
                })();
                if event.has_value("_temp.teleport_audit_addr") {
                    event.rename("_temp.teleport_audit_addr", "teleport.audit.addr")?;
                }
                if event.has_value("teleport.audit.proto") {
                    event.rename("teleport.audit.proto", "network.protocol")?;
                }
                event.remove("teleport.connection");
                if event.has_value("teleport.audit.user_agent") {
                    event.rename("teleport.audit.user_agent", "user_agent")?;
                }
                if event.has_value("user_agent") {
                    if let Some(ua_str) = event.get_string("user_agent") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.remove("user_agent");
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
                let _cond = { event.has_value("teleport.audit.kubernetes_cluster") };
                if _cond {
                    event.set("orchestrator.type", json!("kubernetes"))?;
                }
                if event.has_value("teleport.audit.kubernetes_cluster") {
                    event.rename(
                        "teleport.audit.kubernetes_cluster",
                        "orchestrator.cluster.name",
                    )?;
                }
                if event.has_value("teleport.audit.kubernetes_users") {
                    event.rename(
                        "teleport.audit.kubernetes_users",
                        "teleport.audit.kubernetes.users",
                    )?;
                }
                if event.has_value("teleport.audit.kubernetes_groups") {
                    event.rename(
                        "teleport.audit.kubernetes_groups",
                        "teleport.audit.kubernetes.groups",
                    )?;
                }
                if event.has_value("teleport.audit.kubernetes_pod_name") {
                    event.rename(
                        "teleport.audit.kubernetes_pod_name",
                        "orchestrator.resource.name",
                    )?;
                }
                if event.has_value("teleport.audit.kubernetes_pod_namespace") {
                    event.rename(
                        "teleport.audit.kubernetes_pod_namespace",
                        "orchestrator.namespace",
                    )?;
                }
                if event.has_value("teleport.audit.kubernetes_container_name") {
                    event.rename(
                        "teleport.audit.kubernetes_container_name",
                        "teleport.audit.kubernetes.pod.container_name",
                    )?;
                }
                if event.has_value("teleport.audit.kubernetes_container_image") {
                    event.rename(
                        "teleport.audit.kubernetes_container_image",
                        "teleport.audit.kubernetes.pod.container_image",
                    )?;
                }
                if event.has_value("teleport.audit.kubernetes_node_name") {
                    event.rename(
                        "teleport.audit.kubernetes_node_name",
                        "teleport.audit.kubernetes.pod.node_name",
                    )?;
                }
                if event.has_value("teleport.audit.service_provider_entity_id") {
                    event.rename(
                        "teleport.audit.service_provider_entity_id",
                        "teleport.audit.saml_idp_service_provider.entity_id",
                    )?;
                }
                if event.has_value("teleport.audit.service_provider_shortcut") {
                    event.rename(
                        "teleport.audit.service_provider_shortcut",
                        "teleport.audit.saml_idp_service_provider.shortcut",
                    )?;
                }
                if event.has_value("teleport.audit.attribute_mapping") {
                    event.rename(
                        "teleport.audit.attribute_mapping",
                        "teleport.audit.saml_idp_service_provider.attribute_mapping",
                    )?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("saml."))
                };
                if _cond {
                    if event.has_value("teleport.audit.session_id") {
                        event.rename("teleport.audit.session_id", "teleport.audit.session.id")?;
                    }
                }
                if event.has_value("teleport.audit.added") {
                    event.rename(
                        "teleport.audit.added",
                        "teleport.audit.okta.resources.added",
                    )?;
                }
                if event.has_value("teleport.audit.updated") {
                    event.rename(
                        "teleport.audit.updated",
                        "teleport.audit.okta.resources.updated",
                    )?;
                }
                if event.has_value("teleport.audit.deleted") {
                    event.rename(
                        "teleport.audit.deleted",
                        "teleport.audit.okta.resources.deleted",
                    )?;
                }
                if event.has_value("teleport.audit.source") {
                    event.rename(
                        "teleport.audit.source",
                        "teleport.audit.okta.assignment.source",
                    )?;
                }
                if event.has_value("teleport.audit.starting_status") {
                    event.rename(
                        "teleport.audit.starting_status",
                        "teleport.audit.okta.assignment.starting_status",
                    )?;
                }
                if event.has_value("teleport.audit.ending_status") {
                    event.rename(
                        "teleport.audit.ending_status",
                        "teleport.audit.okta.assignment.ending_status",
                    )?;
                }
                if event.has_value("teleport.audit.access_list_name") {
                    event.rename(
                        "teleport.audit.access_list_name",
                        "teleport.audit.access_list.name",
                    )?;
                }
                if event.has_value("teleport.audit.members") {
                    event.rename(
                        "teleport.audit.members",
                        "teleport.audit.access_list.members",
                    )?;
                }
                let _cond =
                    { event.get_str("teleport.audit.event.action") == Some("access_list.review") };
                if _cond {
                    if event.has_value("teleport.audit.roles") {
                        event.rename(
                            "teleport.audit.roles",
                            "teleport.audit.access_list.membership_requirements_changed.roles",
                        )?;
                    }
                }
                let _cond =
                    { event.get_str("teleport.audit.event.action") == Some("access_list.review") };
                if _cond {
                    if event.has_value("teleport.audit.traits") {
                        event.rename(
                            "teleport.audit.traits",
                            "teleport.audit.access_list.membership_requirements_changed.traits",
                        )?;
                    }
                }
                let _cond =
                    { event.get_str("teleport.audit.event.action") == Some("access_list.review") };
                if _cond {
                    if event.has_value("teleport.audit.message") {
                        event.rename(
                            "teleport.audit.message",
                            "teleport.audit.access_list.review_message",
                        )?;
                    }
                }
                if event.has_value("teleport.audit.review_id") {
                    event.rename(
                        "teleport.audit.review_id",
                        "teleport.audit.access_list.review_id",
                    )?;
                }
                if event.has_value("teleport.audit.review_frequency_changed") {
                    event.rename(
                        "teleport.audit.review_frequency_changed",
                        "teleport.audit.access_list.review_frequency_changed",
                    )?;
                }
                if event.has_value("teleport.audit.review_day_of_month_changed") {
                    event.rename(
                        "teleport.audit.review_day_of_month_changed",
                        "teleport.audit.access_list.review_day_of_month_changed",
                    )?;
                }
                if event.has_value("teleport.audit.removed_members") {
                    event.rename(
                        "teleport.audit.removed_members",
                        "teleport.audit.access_list.removed_members",
                    )?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("lock."))
                };
                if _cond {
                    if event.has_value("teleport.audit.target") {
                        event.rename("teleport.audit.target", "teleport.audit.lock.target")?;
                    }
                }
                if event.has_value("teleport.audit.size") {
                    event.rename(
                        "teleport.audit.size",
                        "teleport.audit.session.terminal_size",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("teleport.audit.session.terminal_size") {
                        if let Some(input) =
                            event.get_string("teleport.audit.session.terminal_size")
                        {
                            // Grok pattern: %{NUMBER:process.tty.columns:int}:%{NUMBER:process.tty.rows:int}
                            if !cached_grok!(
                                "%{NUMBER:process.tty.columns:int}:%{NUMBER:process.tty.rows:int}"
                            )
                            .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })();
                if event.has_value("teleport.audit.initial_command") {
                    event.rename("teleport.audit.initial_command", "process.command_line")?;
                }
                if event.has_value("teleport.audit.session_recording") {
                    event.rename(
                        "teleport.audit.session_recording",
                        "teleport.audit.session.session_recording",
                    )?;
                }
                if event.has_value("teleport.audit.enhanced_recording") {
                    event.rename(
                        "teleport.audit.enhanced_recording",
                        "teleport.audit.session.enhanced_recording",
                    )?;
                }
                if event.has_value("teleport.audit.interactive") {
                    event.rename(
                        "teleport.audit.interactive",
                        "teleport.audit.session.interactive",
                    )?;
                }
                if event.has_value("teleport.audit.participants") {
                    event.rename(
                        "teleport.audit.participants",
                        "teleport.audit.session.participants",
                    )?;
                }
                let _cond = { event.has_value("teleport.audit.session_start") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("teleport.audit.session_start")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set("event.start", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "teleport.audit.session_start".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("event.start") };
                if _cond {
                    event.remove("teleport.audit.session_start");
                }
                let _cond = { event.has_value("teleport.audit.session_stop") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("teleport.audit.session_stop") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set("event.end", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "teleport.audit.session_stop".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("event.end") };
                if _cond {
                    event.remove("teleport.audit.session_stop");
                }
                let _cond = { event.get_str("event.action") == Some("desktop.clipboard.send") };
                if _cond {
                    if event.has_value("teleport.audit.length") {
                        event.rename("teleport.audit.length", "client.bytes")?;
                    }
                }
                if event.has_value("teleport.audit.length") {
                    event.rename("teleport.audit.length", "server.bytes")?;
                }
                if event.has_value("teleport.audit.directory_name") {
                    event.rename("teleport.audit.directory_name", "file.directory")?;
                }
                if event.has_value("teleport.audit.directory_id") {
                    event.rename(
                        "teleport.audit.directory_id",
                        "teleport.audit.desktop.directory_id",
                    )?;
                }
                if event.has_value("teleport.audit.file_path") {
                    event.rename("teleport.audit.file_path", "file.path")?;
                }
                if event.has_value("teleport.audit.ms") {
                    event.rename("teleport.audit.ms", "teleport.audit.desktop.delay_ms")?;
                }
                if event.has_value("teleport.audit.offset") {
                    event.rename("teleport.audit.offset", "teleport.audit.desktop.offset")?;
                }
                if event.has_value("teleport.audit.requestID") {
                    event.rename(
                        "teleport.audit.requestID",
                        "teleport.audit.file_transfer_request.id",
                    )?;
                }
                if event.has_value("teleport.audit.approvers") {
                    event.rename(
                        "teleport.audit.approvers",
                        "teleport.audit.file_transfer_request.approvers",
                    )?;
                }
                if event.has_value("teleport.audit.requester") {
                    event.rename(
                        "teleport.audit.requester",
                        "teleport.audit.file_transfer_request.requester",
                    )?;
                }
                if event.has_value("teleport.audit.location") {
                    event.rename("teleport.audit.location", "file.path")?;
                }
                if event.has_value("teleport.audit.download") {
                    event.rename(
                        "teleport.audit.download",
                        "teleport.audit.file_transfer_request.is_download",
                    )?;
                }
                if event.has_value("teleport.audit.filename") {
                    event.rename("teleport.audit.filename", "file.name")?;
                }
                if event.has_value("teleport.audit.pid") {
                    event.rename("teleport.audit.pid", "process.pid")?;
                }
                if event.has_value("teleport.audit.cgroup_id") {
                    event.rename("teleport.audit.cgroup_id", "process.cgroup.id")?;
                }
                if event.has_value("teleport.audit.program") {
                    event.rename("teleport.audit.program", "process.name")?;
                }
                let _cond = { event.get_bool("teleport.audit.success") == Some(true) };
                if _cond {
                    event.append_unique("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_bool("teleport.audit.success") == Some(true) };
                if _cond {
                    if event.remove("teleport.audit.success").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "teleport.audit.success".into(),
                        });
                    }
                }
                let _cond = { event.get_bool("teleport.audit.success") == Some(false) };
                if _cond {
                    event.append_unique("event.outcome", json!("failure"))?;
                }
                let _cond = { event.get_bool("teleport.audit.success") == Some(false) };
                if _cond {
                    if event.remove("teleport.audit.success").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "teleport.audit.success".into(),
                        });
                    }
                }
                if event.has_value("teleport.audit.message") {
                    event.rename("teleport.audit.message", "message")?;
                }
                if event.has_value("teleport.audit.ppid") {
                    event.rename("teleport.audit.ppid", "process.parent.pid")?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("session."))
                };
                if _cond {
                    if event.has_value("teleport.audit.path") {
                        event.rename("teleport.audit.path", "process.executable")?;
                    }
                }
                if event.has_value("teleport.audit.argv") {
                    event.rename("teleport.audit.argv", "process.args")?;
                }
                if event.has_value("teleport.audit.return_code") {
                    event.rename("teleport.audit.return_code", "process.exit_code")?;
                }
                if event.has_value("teleport.audit.flags") {
                    event.rename("teleport.audit.flags", "process.flags")?;
                }
                if event.has_value("teleport.audit.src_addr") {
                    event.rename("teleport.audit.src_addr", "source.address")?;
                }
                if event.has_value("teleport.audit.dst_addr") {
                    event.rename("teleport.audit.dst_addr", "destination.address")?;
                }
                if event.has_value("teleport.audit.dst_port") {
                    if let Some(val) = event.get("teleport.audit.dst_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "teleport.audit.dst_port".into(),
                                message,
                            }
                        })?;
                        event.set("teleport.audit.dst_port", converted)?;
                    }
                }
                if event.has_value("teleport.audit.dst_port") {
                    event.rename("teleport.audit.dst_port", "destination.port")?;
                }
                if event.has_value("teleport.audit.version") {
                    event.rename(
                        "teleport.audit.version",
                        "teleport.audit.network.tcp_version",
                    )?;
                }
                let _cond = { event.get_i64("teleport.audit.network.tcp_version") == Some(4) };
                if _cond {
                    event.set("network.type", json!("ipv4"))?;
                }
                let _cond = { event.get_i64("teleport.audit.network.tcp_version") == Some(6) };
                if _cond {
                    event.set("network.type", json!("ipv6"))?;
                }
                let _cond = {
                    event.get_i64("teleport.audit.network.tcp_version") == Some(4)
                        || event.get_i64("teleport.audit.network.tcp_version") == Some(6)
                };
                if _cond {
                    if event.remove("teleport.audit.network.tcp_version").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "teleport.audit.network.tcp_version".into(),
                        });
                    }
                }
                if event.has_value("teleport.audit.network.tcp_version") {
                    if let Some(val) = event.get("teleport.audit.network.tcp_version") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "teleport.audit.network.tcp_version".into(),
                                message,
                            }
                        })?;
                        event.set("teleport.audit.network.tcp_version", converted)?;
                    }
                }
                if event.has_value("teleport.audit.network.tcp_version") {
                    event.rename("teleport.audit.network.tcp_version", "network.type")?;
                }
                let _cond = { event.get_i64("teleport.audit.operation") == Some(0) };
                if _cond {
                    event.set("teleport.audit.network.operation", json!("connect"))?;
                }
                let _cond = { event.get_i64("teleport.audit.operation") == Some(1) };
                if _cond {
                    event.set("teleport.audit.network.operation", json!("send"))?;
                }
                event.remove("teleport.audit.operation");
                let _cond = {
                    !event.has_value("audit.working_directory")
                        && event.get_i64("teleport.audit.action") == Some(0)
                };
                if _cond {
                    event.set("teleport.audit.network.action", json!("observed"))?;
                }
                let _cond = {
                    !event.has_value("audit.working_directory")
                        && event.get_i64("teleport.audit.action") == Some(1)
                };
                if _cond {
                    event.set("teleport.audit.network.action", json!("denied"))?;
                }
                let _cond = { event.has_value("teleport.audit.network.action") };
                if _cond {
                    event.remove("teleport.audit.action");
                }
                if event.has_value("teleport.audit.tx") {
                    event.rename("teleport.audit.tx", "source.bytes")?;
                }
                if event.has_value("teleport.audit.rx") {
                    event.rename("teleport.audit.rx", "destination.bytes")?;
                }
                if event.has_value("teleport.audit.expires") {
                    event.rename("teleport.audit.expires", "teleport.audit.resource.expires")?;
                }
                if event.has_value("teleport.audit.updated_by") {
                    event.rename(
                        "teleport.audit.updated_by",
                        "teleport.audit.resource.updated_by",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("teleport.audit.resource.updated_by") {
                        event.rename("teleport.audit.resource.updated_by", "user.name")?;
                    }
                    Ok(())
                })();
                if event.has_value("teleport.audit.ttl") {
                    event.rename("teleport.audit.ttl", "teleport.audit.resource.ttl")?;
                }
                let _cond = { event.get_str("event.action") == Some("user.login") };
                if _cond {
                    if event.has_value("teleport.audit.method") {
                        event.rename("teleport.audit.method", "teleport.audit.login.method")?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("user.login") };
                if _cond {
                    if event.has_value("teleport.audit.attributes") {
                        event.rename(
                            "teleport.audit.attributes",
                            "teleport.audit.login.identity_attributes",
                        )?;
                    }
                }
                if event.has_value("teleport.audit.mfa_device.mfa_device_name") {
                    event.rename(
                        "teleport.audit.mfa_device.mfa_device_name",
                        "teleport.audit.mfa_device.name",
                    )?;
                }
                if event.has_value("teleport.audit.mfa_device.mfa_device_type") {
                    event.rename(
                        "teleport.audit.mfa_device.mfa_device_type",
                        "teleport.audit.mfa_device.type",
                    )?;
                }
                if event.has_value("teleport.audit.mfa_device.mfa_device_uuid") {
                    event.rename(
                        "teleport.audit.mfa_device.mfa_device_uuid",
                        "teleport.audit.mfa_device.uuid",
                    )?;
                }
                if event.has_value("teleport.audit.applied_login_rules") {
                    event.rename(
                        "teleport.audit.applied_login_rules",
                        "teleport.audit.login.applied_rules",
                    )?;
                }
                if event.has_value("teleport.audit.challenge_scope") {
                    event.rename(
                        "teleport.audit.challenge_scope",
                        "teleport.audit.login.challenge_scope",
                    )?;
                }
                if event.has_value("teleport.audit.challenge_allow_reuse") {
                    event.rename(
                        "teleport.audit.challenge_allow_reuse",
                        "teleport.audit.login.challenge_allow_reuse",
                    )?;
                }
                if event.has_value("teleport.audit.connector") {
                    event.rename("teleport.audit.connector", "teleport.audit.user.connector")?;
                }
                if event.has_value("teleport.audit.roles") {
                    event.rename(
                        "teleport.audit.roles",
                        "teleport.audit.access_request.roles",
                    )?;
                }
                if event.has_value("teleport.audit.id") {
                    event.rename("teleport.audit.id", "teleport.audit.access_request.id")?;
                }
                if event.has_value("teleport.audit.state") {
                    event.rename(
                        "teleport.audit.state",
                        "teleport.audit.access_request.state",
                    )?;
                }
                if event.has_value("teleport.audit.delegator") {
                    event.rename(
                        "teleport.audit.delegator",
                        "teleport.audit.access_request.delegator",
                    )?;
                }
                if event.has_value("teleport.audit.annotations") {
                    event.rename(
                        "teleport.audit.annotations",
                        "teleport.audit.access_request.annotations",
                    )?;
                }
                if event.has_value("teleport.audit.reviewer") {
                    event.rename(
                        "teleport.audit.reviewer",
                        "teleport.audit.access_request.reviewer",
                    )?;
                }
                if event.has_value("teleport.audit.proposed_state") {
                    event.rename(
                        "teleport.audit.proposed_state",
                        "teleport.audit.access_request.proposed_state",
                    )?;
                }
                if event.has_value("teleport.audit.resource_ids") {
                    event.rename(
                        "teleport.audit.resource_ids",
                        "teleport.audit.access_request.resource_ids",
                    )?;
                }
                if event.has_value("teleport.audit.max_duration") {
                    event.rename(
                        "teleport.audit.max_duration",
                        "teleport.audit.access_request.max_duration",
                    )?;
                }
                if event.has_value("teleport.audit.promoted_access_list_name") {
                    event.rename(
                        "teleport.audit.promoted_access_list_name",
                        "teleport.audit.access_request.promoted_access_list_name",
                    )?;
                }
                if event.has_value("teleport.audit.assume_start_time") {
                    event.rename(
                        "teleport.audit.assume_start_time",
                        "teleport.audit.access_request.assume_start_time",
                    )?;
                }
                if event.has_value("teleport.audit.command") {
                    event.rename("teleport.audit.command", "process.command_line")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("teleport.audit.exitCode") {
                        if let Some(val) = event.get("teleport.audit.exitCode") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "teleport.audit.exitCode".into(),
                                    message,
                                }
                            })?;
                            event.set("teleport.audit.exitCode", converted)?;
                        }
                    }
                    Ok(())
                })();
                if event.has_value("teleport.audit.exitCode") {
                    event.rename("teleport.audit.exitCode", "process.exit_code")?;
                }
                if event.has_value("teleport.audit.exitError") {
                    event.rename("teleport.audit.exitError", "process.io.text")?;
                }
                let _cond = { event.get_str("event.action") == Some("scp") };
                if _cond {
                    if event.has_value("teleport.audit.path") {
                        event.rename("teleport.audit.path", "file.path")?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("scp") };
                if _cond {
                    if event.has_value("teleport.audit.action") {
                        event.rename("teleport.audit.action", "teleport.audit.scp.action")?;
                    }
                }
                if event.has_value("teleport.audit.working_directory") {
                    event.rename(
                        "teleport.audit.working_directory",
                        "process.working_directory",
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("sftp") };
                if _cond {
                    if event.has_value("teleport.audit.path") {
                        event.rename("teleport.audit.path", "file.path")?;
                    }
                }
                if event.has_value("teleport.audit.target_path") {
                    event.rename(
                        "teleport.audit.target_path",
                        "teleport.audit.sftp.target_path",
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("sftp") };
                if _cond {
                    if event.has_value("teleport.audit.attributes") {
                        event.rename(
                            "teleport.audit.attributes",
                            "teleport.audit.sftp.attributes",
                        )?;
                    }
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(0)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("INVALID")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(1)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("OPEN")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(2)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("CLOSE")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(3)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("READ")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(4)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("WRITE")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(5)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("LSTAT")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(6)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("FSTAT")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(7)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("SETSTAT")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(8)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("FSETSTAT")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(9)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("OPENDIR")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(10)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("READDIR")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(11)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("REMOVE")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(12)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("MKDIR")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(13)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("RMDIR")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(14)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("REALPATH")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(15)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("STAT")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(16)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("RENAME")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(17)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("READLINK")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(18)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("SYMLINK")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_i64("teleport.audit.action") == Some(19)
                };
                if _cond {
                    event.set(
                        "teleport.audit.sftp.action",
                        Value::Array(vec![json!("LINK")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && event.get_str("event.action") == Some("sftp")
                        && event.has_value("teleport.audit.sftp.action")
                };
                if _cond {
                    if event.remove("teleport.audit.action").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "teleport.audit.action".into(),
                        });
                    }
                }
                let _cond = { event.get_str("event.action") == Some("sftp") };
                if _cond {
                    if event.has_value("teleport.audit.action") {
                        if let Some(val) = event.get("teleport.audit.action") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "teleport.audit.action".into(),
                                    message,
                                }
                            })?;
                            event.set("teleport.audit.action", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.get_str("event.action") == Some("sftp")
                        && !event.has_value("teleport.audit.sftp.action")
                };
                if _cond {
                    if event.has_value("teleport.audit.action") {
                        event.rename("teleport.audit.action", "teleport.audit.sftp.action")?;
                    }
                }
                if event.has_value("teleport.audit.reason") {
                    event.rename("teleport.audit.reason", "event.reason")?;
                }
                if event.has_value("teleport.audit.request_path") {
                    event.rename("teleport.audit.request_path", "url.path")?;
                }
                if event.has_value("teleport.audit.verb") {
                    event.rename("teleport.audit.verb", "http.request.method")?;
                }
                if event.has_value("teleport.audit.resource_api_group") {
                    event.rename(
                        "teleport.audit.resource_api_group",
                        "orchestrator.api_version",
                    )?;
                }
                if event.has_value("teleport.audit.resource_namespace") {
                    event.rename(
                        "teleport.audit.resource_namespace",
                        "orchestrator.namespace",
                    )?;
                }
                if event.has_value("teleport.audit.resource_kind") {
                    event.rename("teleport.audit.resource_kind", "orchestrator.resource.type")?;
                }
                if event.has_value("teleport.audit.resource_name") {
                    event.rename("teleport.audit.resource_name", "orchestrator.resource.name")?;
                }
                if event.has_value("teleport.audit.response_code") {
                    event.rename("teleport.audit.response_code", "http.response.status_code")?;
                }
                if event.has_value("teleport.audit.app_uri") {
                    event.rename("teleport.audit.app_uri", "teleport.audit.app.uri")?;
                }
                if event.has_value("teleport.audit.app_public_addr") {
                    event.rename(
                        "teleport.audit.app_public_addr",
                        "teleport.audit.app.public_address",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("teleport.audit.public_addr") {
                        event.rename(
                            "teleport.audit.public_addr",
                            "teleport.audit.app.public_address",
                        )?;
                    }
                    Ok(())
                })();
                if event.has_value("teleport.audit.app_labels") {
                    event.rename("teleport.audit.app_labels", "teleport.audit.app.labels")?;
                }
                if event.has_value("teleport.audit.app_name") {
                    event.rename("teleport.audit.app_name", "teleport.audit.app.name")?;
                }
                if event.has_value("teleport.audit.session_chunk_id") {
                    event.rename(
                        "teleport.audit.session_chunk_id",
                        "teleport.audit.app.session.chunk_id",
                    )?;
                }
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && event.has_value("teleport.audit.aws_region")
                };
                if _cond {
                    event.set("cloud.provider", json!("aws"))?;
                }
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && !event.has_value("cloud.region")
                };
                if _cond {
                    if event.has_value("teleport.audit.aws_region") {
                        event.rename("teleport.audit.aws_region", "cloud.region")?;
                    }
                }
                event.remove("teleport.audit.aws_region");
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && !event.has_value("cloud.service.name")
                };
                if _cond {
                    if event.has_value("teleport.audit.aws_service") {
                        event.rename("teleport.audit.aws_service", "cloud.service.name")?;
                    }
                }
                event.remove("teleport.audit.aws_service");
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && !event.has_value("cloud.instance.id")
                };
                if _cond {
                    if event.has_value("teleport.audit.aws_host") {
                        event.rename("teleport.audit.aws_host", "cloud.instance.id")?;
                    }
                }
                event.remove("teleport.audit.aws_host");
                if event.has_value("teleport.audit.aws_assumed_role") {
                    event.rename(
                        "teleport.audit.aws_assumed_role",
                        "teleport.audit.app.aws.assumed_role",
                    )?;
                }
                if event.has_value("teleport.audit.db_service") {
                    event.rename("teleport.audit.db_service", "service.name")?;
                }
                if event.has_value("teleport.audit.db_protocol") {
                    event.rename(
                        "teleport.audit.db_protocol",
                        "teleport.audit.database.protocol",
                    )?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.starts_with("db."))
                };
                if _cond {
                    if event.has_value("teleport.audit.uri") {
                        event.rename("teleport.audit.uri", "teleport.audit.db_uri")?;
                    }
                }
                if event.has_value("teleport.audit.db_uri") {
                    event.rename("teleport.audit.db_uri", "teleport.audit.database.uri")?;
                }
                let _cond =
                    { event.has_value("teleport.audit.database.uri") && !event.has_value("url") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("teleport.audit.database.uri") {
                            if let Some(input) = event.get_string("teleport.audit.database.uri") {
                                // Grok pattern: ^(%{IPORHOST:url.domain})(:%{POSINT:url.port:long})?$
                                if !cached_grok!(
                                    "^(%{IPORHOST:url.domain})(:%{POSINT:url.port:long})?$"
                                )
                                .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond =
                    { event.has_value("teleport.audit.database.uri") && event.has_value("url") };
                if _cond {
                    if event.has_value("teleport.audit.database.uri") {
                        event.rename("teleport.audit.database.uri", "url.original")?;
                    }
                }
                let _cond = { !event.has_value("url") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("teleport.audit.database.uri") {
                            uri_parts(event, "teleport.audit.database.uri", "url", true, true)?;
                        }
                        Ok(())
                    })();
                }
                event.remove("teleport.audit.database.uri");
                if event.has_value("teleport.audit.db_name") {
                    event.rename("teleport.audit.db_name", "teleport.audit.database.name")?;
                }
                if event.has_value("teleport.audit.db_user") {
                    event.rename("teleport.audit.db_user", "teleport.audit.database.user")?;
                }
                if event.has_value("teleport.audit.db_labels") {
                    event.rename("teleport.audit.db_labels", "teleport.audit.database.labels")?;
                }
                let _cond = { event.has_value("teleport.audit.db_aws_region") };
                if _cond {
                    event.set("cloud.provider", json!("aws"))?;
                }
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && !event.has_value("cloud.region")
                };
                if _cond {
                    if event.has_value("teleport.audit.db_aws_region") {
                        event.rename("teleport.audit.db_aws_region", "cloud.region")?;
                    }
                }
                event.remove("teleport.audit.db_aws_region");
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && event.has_value("teleport.audit.redshift_cluster_id")
                };
                if _cond {
                    event.set("cloud.provider", json!("aws"))?;
                }
                if event.has_value("teleport.audit.db_aws_redshift_cluster_id") {
                    event.rename(
                        "teleport.audit.db_aws_redshift_cluster_id",
                        "teleport.audit.database.aws.redshift_cluster_id",
                    )?;
                }
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && event.has_value("teleport.audit.db_gcp_project_id")
                };
                if _cond {
                    event.set("cloud.provider", json!("gcp"))?;
                }
                if event.has_value("teleport.audit.db_gcp_project_id") {
                    event.rename("teleport.audit.db_gcp_project_id", "cloud.project.id")?;
                }
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && !event.has_value("cloud.instance.id")
                };
                if _cond {
                    if event.has_value("teleport.audit.db_gcp_instance_id") {
                        event.rename("teleport.audit.db_gcp_instance_id", "cloud.instance.id")?;
                    }
                }
                event.remove("teleport.audit.db_gcp_instance_id");
                if event.has_value("teleport.audit.db_roles") {
                    event.rename("teleport.audit.db_roles", "teleport.audit.database.roles")?;
                }
                if event.has_value("teleport.audit.db_type") {
                    event.rename("teleport.audit.db_type", "service.type")?;
                }
                if event.has_value("teleport.audit.db_origin") {
                    event.rename("teleport.audit.db_origin", "teleport.audit.database.origin")?;
                }
                if event.has_value("teleport.audit.db_query") {
                    event.rename("teleport.audit.db_query", "teleport.audit.database.query")?;
                }
                if event.has_value("teleport.audit.db_query_parameters") {
                    event.rename(
                        "teleport.audit.db_query_parameters",
                        "teleport.audit.database.query_parameters",
                    )?;
                }
                if event.has_value("teleport.audit.permission_summary") {
                    event.rename(
                        "teleport.audit.permission_summary",
                        "teleport.audit.database.permission_summary",
                    )?;
                }
                if event.has_value("teleport.audit.affected_object_counts") {
                    event.rename(
                        "teleport.audit.affected_object_counts",
                        "teleport.audit.database.affected_object_counts",
                    )?;
                }
                if event.has_value("teleport.audit.username") {
                    event.rename(
                        "teleport.audit.username",
                        "teleport.audit.database.user_change.username",
                    )?;
                }
                if event.has_value("teleport.audit.delete") {
                    event.rename(
                        "teleport.audit.delete",
                        "teleport.audit.database.user_change.is_deleted",
                    )?;
                }
                if event.has_value("teleport.audit.statement_name") {
                    event.rename(
                        "teleport.audit.statement_name",
                        "teleport.audit.database.postgres.statement_name",
                    )?;
                }
                if event.has_value("teleport.audit.query") {
                    event.rename("teleport.audit.query", "teleport.audit.database.query")?;
                }
                if event.has_value("teleport.audit.portal_name") {
                    event.rename(
                        "teleport.audit.portal_name",
                        "teleport.audit.database.postgres.portal_name",
                    )?;
                }
                if event.has_value("teleport.audit.parameters") {
                    event.rename(
                        "teleport.audit.parameters",
                        "teleport.audit.database.query_parameters",
                    )?;
                }
                if event.has_value("teleport.audit.function_oid") {
                    event.rename(
                        "teleport.audit.function_oid",
                        "teleport.audit.database.postgres.function_oid",
                    )?;
                }
                if event.has_value("teleport.audit.function_args") {
                    event.rename(
                        "teleport.audit.function_args",
                        "teleport.audit.database.postgres.function_args",
                    )?;
                }
                if event.has_value("teleport.audit.windows_desktop_service") {
                    event.rename(
                        "teleport.audit.windows_desktop_service",
                        "teleport.audit.desktop.windows_desktop_service",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("teleport.audit.desktop_addr") {
                        if let Some(input) = event.get_string("teleport.audit.desktop_addr") {
                            // Grok pattern: ^(%{IPORHOST:server.address}|\\[%{IP:server.ip}\\])(:%{POSINT:server.port:long})?$
                            if !cached_grok!("^(%{IPORHOST:server.address}|\\[%{IP:server.ip}\\])(:%{POSINT:server.port:long})?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                    }
                    Ok(())
                })();
                event.remove("teleport.audit.desktop_addr");
                if event.has_value("teleport.audit.windows_domain") {
                    event.rename("teleport.audit.windows_domain", "server.user.domain")?;
                }
                if event.has_value("teleport.audit.windows_user") {
                    event.rename("teleport.audit.windows_user", "server.user.name")?;
                }
                if event.has_value("teleport.audit.desktop_labels") {
                    event.rename(
                        "teleport.audit.desktop_labels",
                        "teleport.audit.desktop.labels",
                    )?;
                }
                if event.has_value("teleport.audit.desktop_name") {
                    event.rename("teleport.audit.desktop_name", "teleport.audit.desktop.name")?;
                }
                if event.has_value("teleport.audit.allow_user_creation") {
                    event.rename(
                        "teleport.audit.allow_user_creation",
                        "teleport.audit.desktop.allow_user_creation",
                    )?;
                }
                if event.has_value("teleport.audit.mfa_device_uuid") {
                    event.rename(
                        "teleport.audit.mfa_device_uuid",
                        "teleport.audit.mfa_device.uuid",
                    )?;
                }
                if event.has_value("teleport.audit.mfa_device_name") {
                    event.rename(
                        "teleport.audit.mfa_device_name",
                        "teleport.audit.mfa_device.name",
                    )?;
                }
                if event.has_value("teleport.audit.mfa_device_type") {
                    event.rename(
                        "teleport.audit.mfa_device_type",
                        "teleport.audit.mfa_device.type",
                    )?;
                }
                if event.has_value("teleport.audit.recorded") {
                    event.rename(
                        "teleport.audit.recorded",
                        "teleport.audit.desktop.is_recorded",
                    )?;
                }
                if event.has_value("teleport.audit.cert_type") {
                    event.rename(
                        "teleport.audit.cert_type",
                        "teleport.audit.certificate.type",
                    )?;
                }
                if event.has_value("teleport.audit.identity") {
                    event.rename(
                        "teleport.audit.identity",
                        "teleport.audit.certificate.identity",
                    )?;
                }
                if event.has_value("teleport.audit.bot_name") {
                    event.rename("teleport.audit.bot_name", "teleport.audit.join.bot_name")?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.ends_with(".join"))
                };
                if _cond {
                    if event.has_value("teleport.audit.method") {
                        event.rename("teleport.audit.method", "teleport.audit.join.method")?;
                    }
                }
                if event.has_value("teleport.audit.token_name") {
                    event.rename(
                        "teleport.audit.token_name",
                        "teleport.audit.join.token_name",
                    )?;
                }
                let _cond = {
                    event
                        .get_str("event.action")
                        .is_some_and(|s| s.ends_with(".join"))
                };
                if _cond {
                    if event.has_value("teleport.audit.attributes") {
                        event.rename(
                            "teleport.audit.attributes",
                            "teleport.audit.join.attributes",
                        )?;
                    }
                }
                if event.has_value("teleport.audit.user_name") {
                    event.rename("teleport.audit.user_name", "teleport.audit.join.user_name")?;
                }
                if event.has_value("teleport.audit.host_id") {
                    event.rename("teleport.audit.host_id", "host.id")?;
                }
                if event.has_value("teleport.audit.node_name") {
                    event.rename("teleport.audit.node_name", "host.name")?;
                }
                if event.has_value("teleport.audit.role") {
                    event.rename("teleport.audit.role", "teleport.audit.join.role")?;
                }
                if event.has_value("teleport.audit.token_expires") {
                    event.rename(
                        "teleport.audit.token_expires",
                        "teleport.audit.join.token_expires",
                    )?;
                }
                if event.has_value("teleport.audit.metadata") {
                    event.rename("teleport.audit.metadata", "teleport.audit.unknown.metadata")?;
                }
                if event.has_value("teleport.audit.unknown_event") {
                    event.rename(
                        "teleport.audit.unknown_event",
                        "teleport.audit.unknown.event_type",
                    )?;
                }
                if event.has_value("teleport.audit.unknown_code") {
                    event.rename("teleport.audit.unknown_code", "teleport.audit.unknown.code")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(event, "teleport.audit.data", "teleport.audit.unknown.data")?;
                    Ok(())
                })();
                event.remove("teleport.audit.data");
                if event.has_value("teleport.audit.device_id") {
                    event.rename(
                        "teleport.audit.device_id",
                        "teleport.audit.device.device_id",
                    )?;
                }
                let _cond = { event.get_i64("teleport.audit.device.os_type") == Some(0) };
                if _cond {
                    event.set("teleport.audit.device.os_type", json!("UNSPECIFIED"))?;
                }
                let _cond = { event.get_i64("teleport.audit.device.os_type") == Some(1) };
                if _cond {
                    event.set("teleport.audit.device.os_type", json!("LINUX"))?;
                }
                let _cond = { event.get_i64("teleport.audit.device.os_type") == Some(2) };
                if _cond {
                    event.set("teleport.audit.device.os_type", json!("MACOS"))?;
                }
                let _cond = { event.get_i64("teleport.audit.device.os_type") == Some(3) };
                if _cond {
                    event.set("teleport.audit.device.os_type", json!("WINDOWS"))?;
                }
                let _cond = { event.has_value("teleport.audit.device.os_type") };
                if _cond {
                    event.remove("teleport.audit.os_type");
                }
                if event.has_value("teleport.audit.os_type") {
                    if let Some(val) = event.get("teleport.audit.os_type") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "teleport.audit.os_type".into(),
                                message,
                            }
                        })?;
                        event.set("teleport.audit.os_type", converted)?;
                    }
                }
                let _cond = { !event.has_value("teleport.audit.device.os_type") };
                if _cond {
                    if event.has_value("teleport.audit.os_type") {
                        event.rename("teleport.audit.os_type", "teleport.audit.device.os_type")?;
                    }
                }
                if event.has_value("teleport.audit.asset_tag") {
                    event.rename(
                        "teleport.audit.asset_tag",
                        "teleport.audit.device.asset_tag",
                    )?;
                }
                if event.has_value("teleport.audit.credential_id") {
                    event.rename(
                        "teleport.audit.credential_id",
                        "teleport.audit.device.credential_id",
                    )?;
                }
                if event.has_value("teleport.audit.device_origin") {
                    event.rename(
                        "teleport.audit.device_origin",
                        "teleport.audit.device.origin",
                    )?;
                }
                if event.has_value("teleport.audit.web_authentication") {
                    event.rename(
                        "teleport.audit.web_authentication",
                        "teleport.audit.device.web_authentication",
                    )?;
                }
                if event.has_value("teleport.audit.web_session_id") {
                    event.rename(
                        "teleport.audit.web_session_id",
                        "teleport.audit.device.web_session_id",
                    )?;
                }
                let _cond = { !event.has_value("url") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("teleport.audit.url") {
                            uri_parts(event, "teleport.audit.url", "url", true, true)?;
                        }
                        Ok(())
                    })();
                }
                if event.has_value("teleport.audit.url") {
                    event.rename("teleport.audit.url", "url.original")?;
                }
                if event.has_value("teleport.audit.certificate.identity.mfa_device_uuid") {
                    event.rename(
                        "teleport.audit.certificate.identity.mfa_device_uuid",
                        "teleport.audit.mfa_device.uuid",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("teleport.audit.certificate.identity.client_ip") {
                        event
                            .rename("teleport.audit.certificate.identity.client_ip", "client.ip")?;
                    }
                    Ok(())
                })();
                if event.has_value("teleport.audit.search_as_roles") {
                    event.rename(
                        "teleport.audit.search_as_roles",
                        "teleport.audit.access_request.resource_search.search_as_roles",
                    )?;
                }
                if event.has_value("teleport.audit.resource_type") {
                    event.rename(
                        "teleport.audit.resource_type",
                        "teleport.audit.access_request.resource_search.resource_type",
                    )?;
                }
                if event.has_value("teleport.audit.labels") {
                    event.rename(
                        "teleport.audit.labels",
                        "teleport.audit.access_request.resource_search.labels",
                    )?;
                }
                if event.has_value("teleport.audit.predicate_expression") {
                    event.rename(
                        "teleport.audit.predicate_expression",
                        "teleport.audit.access_request.resource_search.predicate_expression",
                    )?;
                }
                if event.has_value("teleport.audit.search_keywords") {
                    event.rename(
                        "teleport.audit.search_keywords",
                        "teleport.audit.access_request.resource_search.search_keywords",
                    )?;
                }
                if event.has_value("teleport.audit.statement_id") {
                    event.rename(
                        "teleport.audit.statement_id",
                        "teleport.audit.database.mysql.statement_id",
                    )?;
                }
                if event.has_value("teleport.audit.schema_name") {
                    event.rename(
                        "teleport.audit.schema_name",
                        "teleport.audit.database.mysql.schema_name",
                    )?;
                }
                if event.has_value("teleport.audit.process_id") {
                    event.rename(
                        "teleport.audit.process_id",
                        "teleport.audit.database.mysql.process_id",
                    )?;
                }
                if event.has_value("teleport.audit.subcommand") {
                    event.rename(
                        "teleport.audit.subcommand",
                        "teleport.audit.database.mysql.subcommand",
                    )?;
                }
                if event.has_value("teleport.audit.parameter_id") {
                    event.rename(
                        "teleport.audit.parameter_id",
                        "teleport.audit.database.mysql.parameter_id",
                    )?;
                }
                if event.has_value("teleport.audit.data_size") {
                    event.rename(
                        "teleport.audit.data_size",
                        "teleport.audit.database.mysql.data_size",
                    )?;
                }
                if event.has_value("teleport.audit.rows_count") {
                    event.rename(
                        "teleport.audit.rows_count",
                        "teleport.audit.database.mysql.rows_count",
                    )?;
                }
                if event.has_value("teleport.audit.proc_name") {
                    event.rename(
                        "teleport.audit.proc_name",
                        "teleport.audit.database.proc_name",
                    )?;
                }
                if event.has_value("teleport.audit.payload") {
                    event.rename("teleport.audit.payload", "teleport.audit.database.payload")?;
                }
                if event.has_value("teleport.audit.headers") {
                    event.rename("teleport.audit.headers", "http.request.headers")?;
                }
                let _cond = {
                    event.get_i64("teleport.audit.category") == Some(0)
                        && event.get("event.action").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some(".session.elasticsearch")),
                            serde_json::Value::String(s) => s.contains(".session.elasticsearch"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set(
                        "teleport.audit.database.elasticsearch.category",
                        json!("GENERAL"),
                    )?;
                }
                let _cond = {
                    event.get_i64("teleport.audit.category") == Some(1)
                        && event.get("event.action").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some(".session.elasticsearch")),
                            serde_json::Value::String(s) => s.contains(".session.elasticsearch"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set(
                        "teleport.audit.database.elasticsearch.category",
                        json!("SECURITY"),
                    )?;
                }
                let _cond = {
                    event.get_i64("teleport.audit.category") == Some(2)
                        && event.get("event.action").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some(".session.elasticsearch")),
                            serde_json::Value::String(s) => s.contains(".session.elasticsearch"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set(
                        "teleport.audit.database.elasticsearch.category",
                        json!("SEARCH"),
                    )?;
                }
                let _cond = {
                    event.get_i64("teleport.audit.category") == Some(3)
                        && event.get("event.action").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some(".session.elasticsearch")),
                            serde_json::Value::String(s) => s.contains(".session.elasticsearch"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set(
                        "teleport.audit.database.elasticsearch.category",
                        json!("SQL"),
                    )?;
                }
                let _cond = {
                    event.has_value("teleport.audit.database.elasticsearch.category")
                        && event.get("event.action").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some(".session.elasticsearch")),
                            serde_json::Value::String(s) => s.contains(".session.elasticsearch"),
                            _ => false,
                        })
                };
                if _cond {
                    event.remove("teleport.audit.category");
                }
                let _cond = {
                    event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some(".session.elasticsearch")),
                        serde_json::Value::String(s) => s.contains(".session.elasticsearch"),
                        _ => false,
                    })
                };
                if _cond {
                    if event.has_value("teleport.audit.category") {
                        if let Some(val) = event.get("teleport.audit.category") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "teleport.audit.category".into(),
                                    message,
                                }
                            })?;
                            event.set("teleport.audit.category", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some(".session.elasticsearch")),
                        serde_json::Value::String(s) => s.contains(".session.elasticsearch"),
                        _ => false,
                    })
                };
                if _cond {
                    if event.has_value("teleport.audit.category") {
                        event.rename(
                            "teleport.audit.category",
                            "teleport.audit.database.elasticsearch.category",
                        )?;
                    }
                }
                let _cond = {
                    event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some(".session.elasticsearch")),
                        serde_json::Value::String(s) => s.contains(".session.elasticsearch"),
                        _ => false,
                    })
                };
                if _cond {
                    if event.has_value("teleport.audit.target") {
                        event.rename(
                            "teleport.audit.target",
                            "teleport.audit.database.elasticsearch.target",
                        )?;
                    }
                }
                let _cond = {
                    event.get_i64("teleport.audit.category") == Some(0)
                        && event.get("event.action").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(".session.opensearch"))
                            }
                            serde_json::Value::String(s) => s.contains(".session.opensearch"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set(
                        "teleport.audit.database.opensearch.category",
                        json!("GENERAL"),
                    )?;
                }
                let _cond = {
                    event.get_i64("teleport.audit.category") == Some(1)
                        && event.get("event.action").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(".session.opensearch"))
                            }
                            serde_json::Value::String(s) => s.contains(".session.opensearch"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set(
                        "teleport.audit.database.opensearch.category",
                        json!("SECURITY"),
                    )?;
                }
                let _cond = {
                    event.get_i64("teleport.audit.category") == Some(2)
                        && event.get("event.action").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(".session.opensearch"))
                            }
                            serde_json::Value::String(s) => s.contains(".session.opensearch"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set(
                        "teleport.audit.database.opensearch.category",
                        json!("SEARCH"),
                    )?;
                }
                let _cond = {
                    event.get_i64("teleport.audit.category") == Some(3)
                        && event.get("event.action").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(".session.opensearch"))
                            }
                            serde_json::Value::String(s) => s.contains(".session.opensearch"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("teleport.audit.database.opensearch.category", json!("SQL"))?;
                }
                let _cond = {
                    event.has_value("teleport.audit.database.opensearch.category")
                        && event.get("event.action").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(".session.opensearch"))
                            }
                            serde_json::Value::String(s) => s.contains(".session.opensearch"),
                            _ => false,
                        })
                };
                if _cond {
                    event.remove("teleport.audit.category");
                }
                let _cond = {
                    event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some(".session.opensearch"))
                        }
                        serde_json::Value::String(s) => s.contains(".session.opensearch"),
                        _ => false,
                    })
                };
                if _cond {
                    if event.has_value("teleport.audit.category") {
                        if let Some(val) = event.get("teleport.audit.category") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "teleport.audit.category".into(),
                                    message,
                                }
                            })?;
                            event.set("teleport.audit.category", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some(".session.opensearch"))
                        }
                        serde_json::Value::String(s) => s.contains(".session.opensearch"),
                        _ => false,
                    })
                };
                if _cond {
                    if event.has_value("teleport.audit.category") {
                        event.rename(
                            "teleport.audit.category",
                            "teleport.audit.database.opensearch.category",
                        )?;
                    }
                }
                let _cond = {
                    event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some(".session.opensearch"))
                        }
                        serde_json::Value::String(s) => s.contains(".session.opensearch"),
                        _ => false,
                    })
                };
                if _cond {
                    if event.has_value("teleport.audit.target") {
                        event.rename(
                            "teleport.audit.target",
                            "teleport.audit.database.opensearch.target",
                        )?;
                    }
                }
                let _cond = {
                    event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some(".session.dynamodb."))
                        }
                        serde_json::Value::String(s) => s.contains(".session.dynamodb."),
                        _ => false,
                    })
                };
                if _cond {
                    if event.has_value("teleport.audit.target") {
                        event.rename(
                            "teleport.audit.target",
                            "teleport.audit.database.dynamodb.target",
                        )?;
                    }
                }
                if event.has_value("teleport.audit.upgrade_window_start") {
                    event.rename(
                        "teleport.audit.upgrade_window_start",
                        "teleport.audit.upgradewindow.start",
                    )?;
                }
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && event.has_value("teleport.audit.command_id")
                };
                if _cond {
                    event.set("cloud.provider", json!("aws"))?;
                }
                if event.has_value("teleport.audit.command_id") {
                    event.rename(
                        "teleport.audit.command_id",
                        "teleport.audit.database.aws.ssm_run.command_id",
                    )?;
                }
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && !event.has_value("cloud.instance.id")
                };
                if _cond {
                    if event.has_value("teleport.audit.instance_id") {
                        event.rename("teleport.audit.instance_id", "cloud.instance.id")?;
                    }
                }
                event.remove("teleport.audit.instance_id");
                if event.has_value("teleport.audit.exit_code") {
                    event.rename("teleport.audit.exit_code", "process.exit_code")?;
                }
                let _cond = { event.get_str("teleport.audit.status") == Some("Success") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("teleport.audit.status") == Some("Failure") };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                event.remove("teleport.audit.status");
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && !event.has_value("cloud.account.id")
                };
                if _cond {
                    if event.has_value("teleport.audit.account_id") {
                        event.rename("teleport.audit.account_id", "cloud.account.id")?;
                    }
                }
                event.remove("teleport.audit.account_id");
                let _cond = {
                    event.get_bool("_conf.want_provider_cloud") == Some(true)
                        && !event.has_value("cloud.region")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("teleport.audit.region") {
                            event.rename("teleport.audit.region", "cloud.region")?;
                        }
                        Ok(())
                    })();
                }
                event.remove("teleport.audit.region");
                event.remove("_conf");
                if event.has_value("teleport.audit.stdout") {
                    event.rename(
                        "teleport.audit.stdout",
                        "teleport.audit.database.aws.ssm_run.stdout",
                    )?;
                }
                if event.has_value("teleport.audit.stderr") {
                    event.rename(
                        "teleport.audit.stderr",
                        "teleport.audit.database.aws.ssm_run.stderr",
                    )?;
                }
                if event.has_value("teleport.audit.invocation_url") {
                    event.rename(
                        "teleport.audit.invocation_url",
                        "teleport.audit.database.aws.ssm_run.invocation_url",
                    )?;
                }
                if event.has_value("teleport.audit.keyspace") {
                    event.rename(
                        "teleport.audit.keyspace",
                        "teleport.audit.database.cassandra.keyspace",
                    )?;
                }
                if event.has_value("teleport.audit.query_id") {
                    event.rename(
                        "teleport.audit.query_id",
                        "teleport.audit.database.cassandra.query_id",
                    )?;
                }
                if event.has_value("teleport.audit.consistency") {
                    event.rename(
                        "teleport.audit.consistency",
                        "teleport.audit.database.cassandra.consistency",
                    )?;
                }
                if event.has_value("teleport.audit.batch_type") {
                    event.rename(
                        "teleport.audit.batch_type",
                        "teleport.audit.database.cassandra.batch_type",
                    )?;
                }
                if event.has_value("teleport.audit.children") {
                    event.rename(
                        "teleport.audit.children",
                        "teleport.audit.database.cassandra.children",
                    )?;
                }
                if event.has_value("teleport.audit.event_types") {
                    event.rename(
                        "teleport.audit.event_types",
                        "teleport.audit.database.cassandra.event_types",
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("secreports.audit.query.run") };
                if _cond {
                    if event.has_value("teleport.audit.name") {
                        event.rename("teleport.audit.name", "teleport.audit.audit_query.name")?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("secreports.audit.query.run") };
                if _cond {
                    if event.has_value("teleport.audit.query") {
                        event.rename("teleport.audit.query", "teleport.audit.audit_query.query")?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("secreports.audit.query.run") };
                if _cond {
                    if event.has_value("teleport.audit.days") {
                        event.rename("teleport.audit.days", "teleport.audit.audit_query.days")?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("secreports.audit.query.run") };
                if _cond {
                    if event.has_value("teleport.audit.total_execution_time_in_millis") {
                        event.rename(
                            "teleport.audit.total_execution_time_in_millis",
                            "teleport.audit.audit_query.total_execution_time_in_millis",
                        )?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("secreports.audit.query.run") };
                if _cond {
                    if event.has_value("teleport.audit.data_scanned_in_bytes") {
                        event.rename(
                            "teleport.audit.data_scanned_in_bytes",
                            "teleport.audit.audit_query.data_scanned_in_bytes",
                        )?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("secreports.report.run") };
                if _cond {
                    if event.has_value("teleport.audit.name") {
                        event.rename("teleport.audit.name", "teleport.audit.sec_report.name")?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("secreports.report.run") };
                if _cond {
                    if event.has_value("teleport.audit.version") {
                        event.rename(
                            "teleport.audit.version",
                            "teleport.audit.sec_report.version",
                        )?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("secreports.report.run") };
                if _cond {
                    if event.has_value("teleport.audit.total_execution_time_in_millis") {
                        event.rename(
                            "teleport.audit.total_execution_time_in_millis",
                            "teleport.audit.sec_report.total_execution_time_in_millis",
                        )?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("secreports.report.run") };
                if _cond {
                    if event.has_value("teleport.audit.total_data_scanned_in_bytes") {
                        event.rename(
                            "teleport.audit.total_data_scanned_in_bytes",
                            "teleport.audit.sec_report.total_data_scanned_in_bytes",
                        )?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("secreports.report.run") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("teleport.audit.data_scanned_in_bytes") {
                            event.rename(
                                "teleport.audit.data_scanned_in_bytes",
                                "teleport.audit.sec_report.total_data_scanned_in_bytes",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("secreports.report.run") };
                if _cond {
                    if event.has_value("teleport.audit.sec_report.audit_queries") {
                        event.rename(
                            "teleport.audit.sec_report.audit_queries",
                            "teleport.audit.audit_query",
                        )?;
                    }
                }
                if event.has_value("teleport.audit.integration_name") {
                    event.rename(
                        "teleport.audit.integration_name",
                        "teleport.audit.external_audit_storage.integration_name",
                    )?;
                }
                if event.has_value("teleport.audit.session_recordings_uri") {
                    event.rename(
                        "teleport.audit.session_recordings_uri",
                        "teleport.audit.external_audit_storage.session_recordings_uri",
                    )?;
                }
                if event.has_value("teleport.audit.athena_workgroup") {
                    event.rename(
                        "teleport.audit.athena_workgroup",
                        "teleport.audit.external_audit_storage.athena_workgroup",
                    )?;
                }
                if event.has_value("teleport.audit.glue_database") {
                    event.rename(
                        "teleport.audit.glue_database",
                        "teleport.audit.external_audit_storage.glue_database",
                    )?;
                }
                if event.has_value("teleport.audit.glue_table") {
                    event.rename(
                        "teleport.audit.glue_table",
                        "teleport.audit.external_audit_storage.glue_table",
                    )?;
                }
                if event.has_value("teleport.audit.audit_events_long_term_uri") {
                    event.rename(
                        "teleport.audit.audit_events_long_term_uri",
                        "teleport.audit.external_audit_storage.audit_events_long_term_uri",
                    )?;
                }
                if event.has_value("teleport.audit.athena_results_uri") {
                    event.rename(
                        "teleport.audit.athena_results_uri",
                        "teleport.audit.external_audit_storage.athena_results_uri",
                    )?;
                }
                if event.has_value("teleport.audit.policy_name") {
                    event.rename(
                        "teleport.audit.policy_name",
                        "teleport.audit.external_audit_storage.policy_name",
                    )?;
                }
                if event.has_value("teleport.audit.org_url") {
                    event.rename("teleport.audit.org_url", "teleport.audit.okta.org_url")?;
                }
                if event.has_value("teleport.audit.app_id") {
                    event.rename("teleport.audit.app_id", "teleport.audit.okta.app_id")?;
                }
                if event.has_value("teleport.audit.num_users_created") {
                    event.rename(
                        "teleport.audit.num_users_created",
                        "teleport.audit.okta.users.created",
                    )?;
                }
                if event.has_value("teleport.audit.num_users_deleted") {
                    event.rename(
                        "teleport.audit.num_users_deleted",
                        "teleport.audit.okta.users.deleted",
                    )?;
                }
                if event.has_value("teleport.audit.num_users_modified") {
                    event.rename(
                        "teleport.audit.num_users_modified",
                        "teleport.audit.okta.users.modified",
                    )?;
                }
                if event.has_value("teleport.audit.num_users_total") {
                    event.rename(
                        "teleport.audit.num_users_total",
                        "teleport.audit.okta.users.total",
                    )?;
                }
                if event.has_value("teleport.audit.spiffe_id") {
                    event.rename("teleport.audit.spiffe_id", "teleport.audit.svid.spiffe_id")?;
                }
                if event.has_value("teleport.audit.dns_sans") {
                    event.rename("teleport.audit.dns_sans", "teleport.audit.svid.dns_sans")?;
                }
                if event.has_value("teleport.audit.ip_sans") {
                    event.rename("teleport.audit.ip_sans", "teleport.audit.svid.ip_sans")?;
                }
                if event.has_value("teleport.audit.svid_type") {
                    event.rename("teleport.audit.svid_type", "teleport.audit.svid.type")?;
                }
                if event.has_value("teleport.audit.serial_number") {
                    event.rename(
                        "teleport.audit.serial_number",
                        "teleport.audit.svid.serial_number",
                    )?;
                }
                if event.has_value("teleport.audit.hint") {
                    event.rename("teleport.audit.hint", "teleport.audit.svid.hint")?;
                }
                if event.has_value("teleport.audit.change_id") {
                    event.rename(
                        "teleport.audit.change_id",
                        "teleport.audit.access_path_change.id",
                    )?;
                }
                if event.has_value("teleport.audit.affected_resource_name") {
                    event.rename(
                        "teleport.audit.affected_resource_name",
                        "teleport.audit.access_path_change.resource.name",
                    )?;
                }
                if event.has_value("teleport.audit.affected_resource_source") {
                    event.rename(
                        "teleport.audit.affected_resource_source",
                        "teleport.audit.access_path_change.resource.source",
                    )?;
                }
                if event.has_value("teleport.audit.procedure") {
                    event.rename(
                        "teleport.audit.procedure",
                        "teleport.audit.database.spanner.rpc.procedure",
                    )?;
                }
                if event.has_value("teleport.audit.args") {
                    event.rename(
                        "teleport.audit.args",
                        "teleport.audit.database.spanner.rpc.args",
                    )?;
                }
                // End nested pipeline: "event-groups"
                Ok(())
            })();

            if event.has_value("teleport.audit.method") {
                event.rename("teleport.audit.method", "http.request.method")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("teleport.audit.path") {
                    event.rename("teleport.audit.path", "url.path")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("teleport.audit.raw_query") {
                    event.rename("teleport.audit.raw_query", "url.query")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("teleport.audit.cluster_name") {
                    event.rename("teleport.audit.cluster_name", "orchestrator.cluster.name")?;
                }
                Ok(())
            })();

            event.remove("teleport.audit.cluster_name");

            if event.has_value("teleport.audit.name") {
                event.rename("teleport.audit.name", "orchestrator.resource.name")?;
            }

            let _cond = { event.has_value("teleport.audit.body") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.teleport.audit.body = ctx.teleport.audit.body.decodeBase64();\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.teleport.audit.body = ctx.teleport.audit.body.decodeBase64();\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("teleport.audit.body") {
                event.rename(
                    "teleport.audit.body",
                    "teleport.audit.database.request_body",
                )?;
            }

            let _cond = {
                event.has_value("teleport.audit.database.request_body")
                    && (event
                        .get("teleport.audit.database.request_body")
                        .is_some_and(|v| v.is_string()))
            };
            if _cond {
                event.set(
                    "http.request.body.content",
                    json!(
                        event
                            .get("teleport.audit.database.request_body")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("teleport.audit.database.request_body") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "teleport.audit.database.request_body",
                        "teleport.audit.database.request_body",
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("teleport.audit.status_code") {
                event.rename("teleport.audit.status_code", "http.response.status_code")?;
            }

            let _cond = { event.has_value("teleport.audit.error") };
            if _cond {
                event.append_unique(
                    "message",
                    json!(
                        event
                            .get("teleport.audit.error")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("teleport.audit.error");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Begin nested pipeline: "event-enrich"
                if event.has_value("client.address") {
                    if let Some(input) = event.get_string("client.address") {
                        // Grok pattern: ^(%{IP:client.ip}|%{HOSTNAME:client.domain})$
                        if !cached_grok!("^(%{IP:client.ip}|%{HOSTNAME:client.domain})$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                if event.has_value("client.ip") {
                    if let Some(ip_str) = event.get_string("client.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("client.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("client.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("client.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("client.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("client.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("client.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("client.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("client.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("client.ip") {
                    if let Some(ip_str) = event.get_string("client.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("client.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("client.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("client.as.asn") {
                    event.rename("client.as.asn", "client.as.number")?;
                }
                if event.has_value("client.as.organization_name") {
                    event.rename("client.as.organization_name", "client.as.organization.name")?;
                }
                if event.has_value("server.address") {
                    if let Some(input) = event.get_string("server.address") {
                        // Grok pattern: ^(%{IP:server.ip}|%{HOSTNAME:server.domain})$
                        if !cached_grok!("^(%{IP:server.ip}|%{HOSTNAME:server.domain})$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                if event.has_value("server.ip") {
                    if let Some(ip_str) = event.get_string("server.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("server.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("server.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("server.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("server.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("server.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("server.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("server.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("server.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("server.ip") {
                    if let Some(ip_str) = event.get_string("server.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("server.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("server.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("server.as.asn") {
                    event.rename("server.as.asn", "server.as.number")?;
                }
                if event.has_value("server.as.organization_name") {
                    event.rename("server.as.organization_name", "server.as.organization.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("source.address") {
                        if let Some(input) = event.get_string("source.address") {
                            // Grok pattern: ^(%{IP:source.ip}|%{HOSTNAME:source.domain})$
                            if !cached_grok!("^(%{IP:source.ip}|%{HOSTNAME:source.domain})$")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })();
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("destination.address") {
                        if let Some(input) = event.get_string("destination.address") {
                            // Grok pattern: ^(%{IP:destination.ip}|%{HOSTNAME:destination.domain})$
                            if !cached_grok!(
                                "^(%{IP:destination.ip}|%{HOSTNAME:destination.domain})$"
                            )
                            .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })();
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
                    !event.has_value("user.email")
                        && event.has_value("user.name")
                        && event
                            .get_str("user.name")
                            .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                            .is_some_and(|i| i.is_some_and(|i| i > 0))
                };
                if _cond {
                    event.rename("user.name", "user.email")?;
                }
                let _cond = { !event.has_value("user.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("user.email") {
                            if let Some(input) = event.get_string("user.email") {
                                let mut remaining: &str = &input;
                                let mut captured: Vec<(&str, &str)> = Vec::new();
                                let matched = 'dissect: {
                                    let Some(pos) = remaining.find("@") else {
                                        break 'dissect false;
                                    };
                                    captured.push(("user.name", &remaining[..pos]));
                                    remaining = &remaining[pos..];
                                    let Some(rest) = remaining.strip_prefix("@") else {
                                        break 'dissect false;
                                    };
                                    remaining = rest;
                                    captured.push(("user.domain", remaining));
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
                }
                let _cond = { event.has_value("client.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("destination.ip") };
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
                let _cond = { event.has_value("server.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("server.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("source.ip") };
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
                let _cond = { event.has_value("teleport.audit.certificate.identity.client_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("teleport.audit.certificate.identity.client_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("host.hostname") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("client.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("destination.domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("destination.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("server.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("source.domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("source.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("url.domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("url.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("teleport.audit.app.public_address") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("teleport.audit.app.public_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.set("related.user", json!(""))?;
                let _cond = { event.has_value("user.name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("user.email") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("teleport.audit.resource.name")
                        && event
                            .get_str("event.action")
                            .is_some_and(|s| s.starts_with("user."))
                };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("teleport.audit.resource.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("process.user.name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("process.user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("teleport.audit.database.user") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("teleport.audit.database.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("teleport.audit.database.user_change.username") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("teleport.audit.database.user_change.username")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("teleport.audit.certificate.identity.user") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("teleport.audit.certificate.identity.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // Painless script
                // Source: if (ctx.teleport?.audit?.certificate?.identity?.logins != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.logins);\n}\nif (ctx.teleport?.audit?.certificate?.identity?.participants != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.participants);\n}\nif (ctx.teleport?.audit?.certificate?.identity?.database_users != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.database_users);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.teleport?.audit?.certificate?.identity?.logins != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.logins);\n}\nif (ctx.teleport?.audit?.certificate?.identity?.participants != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.participants);\n}\nif (ctx.teleport?.audit?.certificate?.identity?.database_users != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.database_users);\n}\n"#
                    ),
                )?;
                let _cond = { event.has_value("teleport.audit.certificate.identity.impersonator") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("teleport.audit.certificate.identity.impersonator")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("teleport.audit.server.labels.hostname") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("teleport.audit.server.labels.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "event-enrich"
                Ok(())
            })();

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
