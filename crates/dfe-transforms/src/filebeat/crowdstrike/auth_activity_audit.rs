// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `auth_activity_audit` pipeline.
pub struct AuthActivityAudit;

impl Transform for AuthActivityAudit {
    fn name(&self) -> &str {
        "auth_activity_audit"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("event.kind", json!("event"))?;

        let cond = {
            event.has("crowdstrike.event.OperationName")
                && !(["twoFactorAuthenticate", "userAuthenticate"].contains(
                    &event
                        .get_str("crowdstrike.event.OperationName")
                        .unwrap_or(""),
                ))
        };
        if cond {
            event.append("event.category", json!("iam"))?;
        }

        let cond = {
            event.has("crowdstrike.event.OperationName")
                && ["twoFactorAuthenticate", "userAuthenticate"].contains(
                    &event
                        .get_str("crowdstrike.event.OperationName")
                        .unwrap_or(""),
                )
        };
        if cond {
            event.append("event.category", json!("authentication"))?;
        }

        let cond = {
            event.has("crowdstrike.event.OperationName")
                && [
                    "activateUser",
                    "changePassword",
                    "confirmResetPassword",
                    "deactivateUser",
                    "grantUserRoles",
                    "grantCustomerSubscriptions",
                    "revokeUserRoles",
                    "revokeCustomerSubscriptions",
                    "updateUser",
                    "updateUserRoles",
                ]
                .contains(
                    &event
                        .get_str("crowdstrike.event.OperationName")
                        .unwrap_or(""),
                )
        };
        if cond {
            event.append("event.type", json!("user"))?;
        }

        let cond = {
            event.has("crowdstrike.event.OperationName")
                && [
                    "activateUser",
                    "changePassword",
                    "confirmResetPassword",
                    "deactivateUser",
                    "grantUserRoles",
                    "grantCustomerSubscriptions",
                    "revokeUserRoles",
                    "revokeCustomerSubscriptions",
                    "updateUser",
                    "updateUserRoles",
                ]
                .contains(
                    &event
                        .get_str("crowdstrike.event.OperationName")
                        .unwrap_or(""),
                )
        };
        if cond {
            event.append("event.type", json!("change"))?;
        }

        let cond = {
            event.has("crowdstrike.event.OperationName")
                && event.get_str("crowdstrike.event.OperationName") == Some("createUser")
        };
        if cond {
            event.append("event.type", json!("creation"))?;
        }

        let cond = {
            event.has("crowdstrike.event.OperationName")
                && event.get_str("crowdstrike.event.OperationName") == Some("deleteUser")
        };
        if cond {
            event.append("event.type", json!("deletion"))?;
        }

        if event.has("crowdstrike.event.UserId") {
            event.rename("crowdstrike.event.UserId", "user.name")?;
        }

        let cond = { event.has("crowdstrike.event.OperationName") };
        if cond {
            event.append(
                "event.action",
                event
                    .get("crowdstrike.event.OperationName")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { !event.has("event.action") };
        if cond {
            event.append("event.action", json!("AuthActivityAuditEvent"))?;
        }

        let cond = { event.get_bool("crowdstrike.event.Success") == Some(true) };
        if cond {
            event.set("event.outcome", json!("success"))?;
        }

        let cond = { event.get_bool("crowdstrike.event.Success") == Some(false) };
        if cond {
            event.set("event.outcome", json!("failure"))?;
        }

        let cond = { !event.has("event.outcome") };
        if cond {
            event.set("event.outcome", json!("unknown"))?;
        }

        if event.has("crowdstrike.event.ServiceName") {
            event.rename("crowdstrike.event.ServiceName", "message")?;
        }

        if event.has("crowdstrike.event.UserIp") {
            event.rename("crowdstrike.event.UserIp", "source.ip")?;
        }

        Ok(TransformResult::Continue)
    }
}
