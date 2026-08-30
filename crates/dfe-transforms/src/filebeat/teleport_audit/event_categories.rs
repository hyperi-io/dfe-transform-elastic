// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `event_categories` pipeline.
pub struct EventCategories;

impl Transform for EventCategories {
    fn name(&self) -> &str {
        "event_categories"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("event.kind", json!("event"))?;

        let _cond = { event.get_str("event.code").is_some_and(|s| s.ends_with("E")) || event.get_str("event.code").is_some_and(|s| s.ends_with("W")) };
        if _cond {
        event.set("event.outcome", Value::Array(vec![json!("failure")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("access_list.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("configuration"), json!("iam")]))?;
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

        let _cond = { event.get_str("event.action") == Some("access_list.member.delete_all_members") };
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

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("access_request.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("session"), json!("iam")]))?;
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

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("app.")) && !(event.get_str("event.action").is_some_and(|s| s.starts_with("app.session."))) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("process"), json!("configuration")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("app.session.")) };
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

        let _cond = { event.get_str("event.action") == Some("app.session.chunk") || event.get_str("event.action") == Some("app.session.dynamodb.request") };
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

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("auth")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("authentication")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("auth") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("info")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("auth_preference.update") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("change")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("billing.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("configuration"), json!("iam")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("billing.create_card") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("billing.delete_card") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("deletion")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("billing.update_card") || event.get_str("event.action") == Some("billing.update_info") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("change")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("bot.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("process"), json!("host")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("bot.create") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("start")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("bot.delete") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("end")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("bot.join") || event.get_str("event.action") == Some("bot.update") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("info")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("cert.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("configuration"), json!("iam")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("cert.create") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("client.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("network")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("client.disconnect") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("end")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("cluster_networking_config.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("configuration")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("cluster_networking_config.update") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("change")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("db.")) };
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

        let _cond = { event.get_str("event.action") == Some("db.session.cassandra.batch") || event.get_str("event.action") == Some("db.session.cassandra.execute") || event.get_str("event.action") == Some("db.session.cassandra.prepare") || event.get_str("event.action") == Some("db.session.cassandra.register") || event.get_str("event.action") == Some("db.session.dynamodb.request") || event.get_str("event.action") == Some("db.session.elasticsearch.request") || event.get_str("event.action") == Some("db.session.opensearch.request") || event.get_str("event.action") == Some("db.session.permissions.update") || event.get_str("event.action") == Some("db.session.postgres.function") || event.get_str("event.action") == Some("db.session.postgres.statements.bind") || event.get_str("event.action") == Some("db.session.postgres.statements.close") || event.get_str("event.action") == Some("db.session.postgres.statements.execute") || event.get_str("event.action") == Some("db.session.postgres.statements.parse") || event.get_str("event.action") == Some("db.session.query") || event.get_str("event.action") == Some("db.session.spanner.rpc") || event.get_str("event.action") == Some("db.session.sqlserver.rpc_request") || event.get_str("event.action") == Some("db.session.start") || event.get_str("event.action") == Some("db.session.user.create") || event.get_str("event.action") == Some("db.session.user.deactivate") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("info")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("db.session.end") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("info")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("db.session.malformed_packet") || event.get_str("event.action") == Some("db.session.query.failed") };
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

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("desktop.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("host"), json!("file")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("desktop.clipboard.receive") || event.get_str("event.action") == Some("desktop.clipboard.send") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("change")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("desktop.directory.read") || event.get_str("event.action") == Some("desktop.directory.share") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("access")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("desktop.directory.write") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation"), json!("deletion")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("device.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("configuration"), json!("iam")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("device.authenticate.confirm") || event.get_str("event.action") == Some("device.authenticate") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("info")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("device.create") || event.get_str("event.action") == Some("device.token.create") || event.get_str("event.action") == Some("device.webtoken.create") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("device.delete") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("deletion")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("device.enroll") || event.get_str("event.action") == Some("device.update") || event.get_str("event.action") == Some("device.token.spent") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("change")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("exec")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("process"), json!("host")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("exec") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("start")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("external_audit_storage.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("configuration")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("external_audit_storage.disable") || event.get_str("event.action") == Some("external_audit_storage.enable") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("change")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("github.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("authentication"), json!("iam")]))?;
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

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("instance.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("network"), json!("host")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("instance.join") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("start")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("join_token.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("session"), json!("iam")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("join_token.create") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("kube.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("process"), json!("host")]))?;
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

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("lock.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("configuration"), json!("iam")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("lock.created") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("lock.deleted") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("deletion")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("login_rule.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("authentication"), json!("iam")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("login_rule.create") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("login_rule.delete") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("deletion")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("mfa.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("authentication"), json!("iam")]))?;
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

        let _cond = { event.get_str("event.action") == Some("mfa_auth_challenge.validate") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("end")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("oidc.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("authentication"), json!("iam")]))?;
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

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("okta.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("iam")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("okta.assignment")) || event.get_str("event.action").is_some_and(|s| s.starts_with("okta.")) && event.get_str("event.action").is_some_and(|s| s.ends_with(".update")) };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("change")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("okta.sync") || event.get_str("event.action") == Some("okta.access_list.sync") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("change")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("port")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("network")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("port") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("start")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("privilege_token.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("session"), json!("iam")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("privilege_token.create") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("recovery_")) || event.get_str("event.action").is_some_and(|s| s.starts_with("reset_password_token.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("session"), json!("iam")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("recovery_code.generated") || event.get_str("event.action") == Some("recovery_token.create") || event.get_str("event.action") == Some("reset_password_token.create") };
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

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("role.")) };
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

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("saml.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("authentication"), json!("iam")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("saml.created") || event.get_str("event.action") == Some("saml.idp.service.provider.create") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("saml.deleted") || event.get_str("event.action") == Some("saml.idp.service.provider.delete") || event.get_str("event.action") == Some("saml.idp.service.provider.delete_all") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("deletion")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("saml.updated") || event.get_str("event.action") == Some("saml.idp.service.provider.update") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("change")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("saml.idp.auth") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("info")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("scp.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("file")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("secreports.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("threat")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("secreports.audit.query.run") || event.get_str("event.action") == Some("secreports.report.run") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("indicator")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("session.")) };
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

        let _cond = { event.get_str("event.action") == Some("session.connect") || event.get_str("event.action") == Some("session.command") || event.get_str("event.action") == Some("session.data") || event.get_str("event.action") == Some("session.disk") || event.get_str("event.action") == Some("session.join") || event.get_str("event.action") == Some("session.network") || event.get_str("event.action") == Some("session.leave") || event.get_str("event.action") == Some("session.process_exit") || event.get_str("event.action") == Some("session.recording.access") || event.get_str("event.action") == Some("session.upload") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("info")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("session.rejected") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("denied")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("session_recording_config.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("configuration")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("session_recording_config.update") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("change")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("sftp")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("file"), json!("network")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("spiffe.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("iam"), json!("process")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("spiffe.svid.issued") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("ssm.")) };
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

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("trusted_cluster.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("network"), json!("host")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("trusted_cluster.create") || event.get_str("event.action") == Some("trusted_cluster_token.create") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("trusted_cluster.delete") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("deletion")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("user.")) && event.get_str("event.action") != Some("user.login") };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("iam")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("user.login") };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("authentication")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("user.create") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("creation"), json!("user")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("user.delete") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("deletion"), json!("user")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("user.password_change") || event.get_str("event.action") == Some("user.update") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("change"), json!("user")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("user.login") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("start")]))?;
        }

        let _cond = { event.get_str("event.action").is_some_and(|s| s.starts_with("windows.")) };
        if _cond {
        event.set("event.category", Value::Array(vec![json!("host"), json!("session")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("windows.desktop.session.start") };
        if _cond {
        event.set("event.type", Value::Array(vec![json!("start")]))?;
        }

        let _cond = { event.get_str("event.action") == Some("windows.desktop.session.end") };
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

        Ok(TransformResult::Continue)
    }
}
