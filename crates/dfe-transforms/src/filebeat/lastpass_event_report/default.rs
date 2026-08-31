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
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "json")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.Action") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.Data") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.IP_Address") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.Time") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.Username") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.Action") {
                event.rename("json.Action", "lastpass.event_report.action")?;
            }

            if let Some(v) = event
                .get("lastpass.event_report.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("login verification email sent")),
                        serde_json::Value::String(s) => s.contains("login verification email sent"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("log in"))
                        }
                        serde_json::Value::String(s) => s.contains("log in"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("login to admin console")),
                        serde_json::Value::String(s) => s.contains("login to admin console"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("saml login"))
                        }
                        serde_json::Value::String(s) => s.contains("saml login"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("multifactor enabled"))
                        }
                        serde_json::Value::String(s) => s.contains("multifactor enabled"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("failed login attempt"))
                        }
                        serde_json::Value::String(s) => s.contains("failed login attempt"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set(
                    "event.category",
                    Value::Array(vec![json!("authentication")]),
                )?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("add policy"))
                        }
                        serde_json::Value::String(s) => s.contains("add policy"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("delete policy"))
                        }
                        serde_json::Value::String(s) => s.contains("delete policy"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("enterprise api secret regenerated")),
                        serde_json::Value::String(s) => {
                            s.contains("enterprise api secret regenerated")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("configuration")]))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("make admin"))
                        }
                        serde_json::Value::String(s) => s.contains("make admin"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("master password changed")),
                        serde_json::Value::String(s) => s.contains("master password changed"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("master password reset by super admin")),
                        serde_json::Value::String(s) => {
                            s.contains("master password reset by super admin")
                        }
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("require password change")),
                        serde_json::Value::String(s) => s.contains("require password change"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account created")),
                        serde_json::Value::String(s) => s.contains("employee account created"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("created lastpass account")),
                        serde_json::Value::String(s) => s.contains("created lastpass account"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account deleted")),
                        serde_json::Value::String(s) => s.contains("employee account deleted"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("remove admin"))
                        }
                        serde_json::Value::String(s) => s.contains("remove admin"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("create group"))
                        }
                        serde_json::Value::String(s) => s.contains("create group"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("adding user to group"))
                        }
                        serde_json::Value::String(s) => s.contains("adding user to group"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("iam")]))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("log in"))
                        }
                        serde_json::Value::String(s) => s.contains("log in"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("login to admin console")),
                        serde_json::Value::String(s) => s.contains("login to admin console"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("saml login"))
                        }
                        serde_json::Value::String(s) => s.contains("saml login"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("open secure note"))
                        }
                        serde_json::Value::String(s) => s.contains("open secure note"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("failed login attempt"))
                        }
                        serde_json::Value::String(s) => s.contains("failed login attempt"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("start")]))?;
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("make admin"))
                    }
                    serde_json::Value::String(s) => s.contains("make admin"),
                    _ => false,
                })
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("admin")]))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("enterprise api secret regenerated")),
                        serde_json::Value::String(s) => {
                            s.contains("enterprise api secret regenerated")
                        }
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("update folder permissions")),
                        serde_json::Value::String(s) => s.contains("update folder permissions"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("master password changed")),
                        serde_json::Value::String(s) => s.contains("master password changed"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("master password reset by super admin")),
                        serde_json::Value::String(s) => {
                            s.contains("master password reset by super admin")
                        }
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("edit secure note"))
                        }
                        serde_json::Value::String(s) => s.contains("edit secure note"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("renamed shared folder")),
                        serde_json::Value::String(s) => s.contains("renamed shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("move to shared folder")),
                        serde_json::Value::String(s) => s.contains("move to shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("move from shared folder")),
                        serde_json::Value::String(s) => s.contains("move from shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("limit shared folder"))
                        }
                        serde_json::Value::String(s) => s.contains("limit shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("update folder permissions")),
                        serde_json::Value::String(s) => s.contains("update folder permissions"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("change")]))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account created")),
                        serde_json::Value::String(s) => s.contains("employee account created"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("created lastpass account")),
                        serde_json::Value::String(s) => s.contains("created lastpass account"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("created shared folder")),
                        serde_json::Value::String(s) => s.contains("created shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("add secure note"))
                        }
                        serde_json::Value::String(s) => s.contains("add secure note"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("site added"))
                        }
                        serde_json::Value::String(s) => s.contains("site added"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("add to shared folder"))
                        }
                        serde_json::Value::String(s) => s.contains("add to shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("add policy"))
                        }
                        serde_json::Value::String(s) => s.contains("add policy"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("creation")]))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("deleted sites"))
                        }
                        serde_json::Value::String(s) => s.contains("deleted sites"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("delete policy"))
                        }
                        serde_json::Value::String(s) => s.contains("delete policy"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("deleted shared folder")),
                        serde_json::Value::String(s) => s.contains("deleted shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account deleted")),
                        serde_json::Value::String(s) => s.contains("employee account deleted"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("remove admin"))
                        }
                        serde_json::Value::String(s) => s.contains("remove admin"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("removed from shared folder")),
                        serde_json::Value::String(s) => s.contains("removed from shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("delete shared sites"))
                        }
                        serde_json::Value::String(s) => s.contains("delete shared sites"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("deleted sites"))
                        }
                        serde_json::Value::String(s) => s.contains("deleted sites"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("deletion")]))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("create group"))
                        }
                        serde_json::Value::String(s) => s.contains("create group"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("adding user to group"))
                        }
                        serde_json::Value::String(s) => s.contains("adding user to group"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set(
                    "event.type",
                    Value::Array(vec![json!("group"), json!("creation")]),
                )?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("login verification email sent")),
                        serde_json::Value::String(s) => s.contains("login verification email sent"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("get shared folder data")),
                        serde_json::Value::String(s) => s.contains("get shared folder data"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("get user data"))
                        }
                        serde_json::Value::String(s) => s.contains("get user data"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("employee invited"))
                        }
                        serde_json::Value::String(s) => s.contains("employee invited"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("reporting"))
                        }
                        serde_json::Value::String(s) => s.contains("reporting"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("require password change")),
                        serde_json::Value::String(s) => s.contains("require password change"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("multifactor enabled"))
                        }
                        serde_json::Value::String(s) => s.contains("multifactor enabled"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("login verification email sent")),
                        serde_json::Value::String(s) => s.contains("login verification email sent"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("log in"))
                        }
                        serde_json::Value::String(s) => s.contains("log in"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("login to admin console")),
                        serde_json::Value::String(s) => s.contains("login to admin console"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("saml login"))
                        }
                        serde_json::Value::String(s) => s.contains("saml login"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("open secure note"))
                        }
                        serde_json::Value::String(s) => s.contains("open secure note"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("make admin"))
                        }
                        serde_json::Value::String(s) => s.contains("make admin"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("enterprise api secret regenerated")),
                        serde_json::Value::String(s) => {
                            s.contains("enterprise api secret regenerated")
                        }
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("update folder permissions")),
                        serde_json::Value::String(s) => s.contains("update folder permissions"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("master password changed")),
                        serde_json::Value::String(s) => s.contains("master password changed"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("master password reset by super admin")),
                        serde_json::Value::String(s) => {
                            s.contains("master password reset by super admin")
                        }
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account created")),
                        serde_json::Value::String(s) => s.contains("employee account created"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("created lastpass account")),
                        serde_json::Value::String(s) => s.contains("created lastpass account"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("created shared folder")),
                        serde_json::Value::String(s) => s.contains("created shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("add secure note"))
                        }
                        serde_json::Value::String(s) => s.contains("add secure note"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("site added"))
                        }
                        serde_json::Value::String(s) => s.contains("site added"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("add to shared folder"))
                        }
                        serde_json::Value::String(s) => s.contains("add to shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("add policy"))
                        }
                        serde_json::Value::String(s) => s.contains("add policy"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("deleted sites"))
                        }
                        serde_json::Value::String(s) => s.contains("deleted sites"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("delete policy"))
                        }
                        serde_json::Value::String(s) => s.contains("delete policy"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("deleted shared folder")),
                        serde_json::Value::String(s) => s.contains("deleted shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account deleted")),
                        serde_json::Value::String(s) => s.contains("employee account deleted"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("remove admin"))
                        }
                        serde_json::Value::String(s) => s.contains("remove admin"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("create group"))
                        }
                        serde_json::Value::String(s) => s.contains("create group"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("adding user to group"))
                        }
                        serde_json::Value::String(s) => s.contains("adding user to group"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("get shared folder data")),
                        serde_json::Value::String(s) => s.contains("get shared folder data"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("get user data"))
                        }
                        serde_json::Value::String(s) => s.contains("get user data"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("employee invited"))
                        }
                        serde_json::Value::String(s) => s.contains("employee invited"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("edit secure note"))
                        }
                        serde_json::Value::String(s) => s.contains("edit secure note"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("renamed shared folder")),
                        serde_json::Value::String(s) => s.contains("renamed shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("move to shared folder")),
                        serde_json::Value::String(s) => s.contains("move to shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("move from shared folder")),
                        serde_json::Value::String(s) => s.contains("move from shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("limit shared folder"))
                        }
                        serde_json::Value::String(s) => s.contains("limit shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("removed from shared folder")),
                        serde_json::Value::String(s) => s.contains("removed from shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("update folder permissions")),
                        serde_json::Value::String(s) => s.contains("update folder permissions"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("delete shared sites"))
                        }
                        serde_json::Value::String(s) => s.contains("delete shared sites"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("multifactor enabled"))
                        }
                        serde_json::Value::String(s) => s.contains("multifactor enabled"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("deleted sites"))
                        }
                        serde_json::Value::String(s) => s.contains("deleted sites"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("failed login attempt"))
                    }
                    serde_json::Value::String(s) => s.contains("failed login attempt"),
                    _ => false,
                })
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("require password change")),
                        serde_json::Value::String(s) => s.contains("require password change"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("reporting"))
                        }
                        serde_json::Value::String(s) => s.contains("reporting"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = { event.has_value("_conf.tz_offset") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("_conf.tz_offset", "event.timezone")?;
                    Ok(())
                })();
            }

            if !event.has("event.timezone") {
                event.set("event.timezone", json!("US/Eastern"))?;
            }

            let _cond = {
                event.has_value("json.Time")
                    && event.get_str("json.Time") != Some("")
                    && event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd HH:mm:ss",
                                "yyyy-MM-dd HH:mm:ss ZZZZ",
                                "yyyy-MM-dd HH:mm:ssZZZZ",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("lastpass.event_report.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.Time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            if let Some(v) = event
                .get("lastpass.event_report.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = { event.get_str("json.IP_Address") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IP_Address") {
                        if let Some(val) = event.get("json.IP_Address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IP_Address".into(),
                                    message,
                                }
                            })?;
                            event.set("lastpass.event_report.ip", converted)?;
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
                                .map_or_else(String::new, template_to_string)
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

            if let Some(v) = event
                .get("lastpass.event_report.ip")
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

            if event.has_value("json.Username") {
                event.rename("json.Username", "lastpass.event_report.user_name")?;
            }

            if event.has_value("json.Data") {
                event.rename("json.Data", "lastpass.event_report.data.original")?;
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("deleted sites"))
                    }
                    serde_json::Value::String(s) => s.contains("deleted sites"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("lastpass.event_report.data.original") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set(
                            "lastpass.event_report.data.deleted_site",
                            Value::Array(parts),
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("employee invited"))
                        }
                        serde_json::Value::String(s) => s.contains("employee invited"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account created")),
                        serde_json::Value::String(s) => s.contains("employee account created"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account deleted")),
                        serde_json::Value::String(s) => s.contains("employee account deleted"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("lastpass.event_report.data.original") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("lastpass.event_report.data.user_email", Value::Array(parts))?;
                    }
                    Ok(())
                })();
            }

            // Painless script
            // Source: if (ctx.event?.action?.contains('limit shared folder') == true) {\n  int indx = ctx.lastpass.event_report.data.original.lastIndexOf(' ');\n  String str = ctx.lastpass.event_report.data.original.substring(0,indx)+ ',' + ctx.lastpass.event_report.data.original.substring(indx+1);\n  ctx._temp = str;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.event?.action?.contains('limit shared folder') == true) {\n  int indx = ctx.lastpass.event_report.data.original.lastIndexOf(' ');\n  String str = ctx.lastpass.event_report.data.original.substring(0,indx)+ ',' + ctx.lastpass.event_report.data.original.substring(indx+1);\n  ctx._temp = str;\n}\n"#
                ),
            )?;

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("log in")),
                    serde_json::Value::String(s) => s.contains("log in"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^%{GREEDYDATA:lastpass.event_report.data.login_site}$
                        if !cached_grok!("^%{GREEDYDATA:lastpass.event_report.data.login_site}$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("saml login"))
                    }
                    serde_json::Value::String(s) => s.contains("saml login"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^%{GREEDYDATA:lastpass.event_report.data.saml_login}$
                        if !cached_grok!("^%{GREEDYDATA:lastpass.event_report.data.saml_login}$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("failed login attempt"))
                    }
                    serde_json::Value::String(s) => s.contains("failed login attempt"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^%{GREEDYDATA:lastpass.event_report.data.failed_login}$
                        if !cached_grok!("^%{GREEDYDATA:lastpass.event_report.data.failed_login}$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("login to admin console")),
                        serde_json::Value::String(s) => s.contains("login to admin console"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("make admin"))
                        }
                        serde_json::Value::String(s) => s.contains("make admin"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("remove admin"))
                        }
                        serde_json::Value::String(s) => s.contains("remove admin"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("require password change")),
                        serde_json::Value::String(s) => s.contains("require password change"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("master password reset by super admin")),
                        serde_json::Value::String(s) => {
                            s.contains("master password reset by super admin")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^%{EMAILADDRESS:lastpass.event_report.data.user_email}$
                        if !cached_grok!("^%{EMAILADDRESS:lastpass.event_report.data.user_email}$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("site added"))
                    }
                    serde_json::Value::String(s) => s.contains("site added"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^%{GREEDYDATA:lastpass.event_report.data.added_site}$
                        if !cached_grok!("^%{GREEDYDATA:lastpass.event_report.data.added_site}$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("created shared folder")),
                        serde_json::Value::String(s) => s.contains("created shared folder"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("deleted shared folder")),
                        serde_json::Value::String(s) => s.contains("deleted shared folder"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^%{DATA:lastpass.event_report.data.shared_folder_name}$
                        if !cached_grok!("^%{DATA:lastpass.event_report.data.shared_folder_name}$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("add secure note"))
                        }
                        serde_json::Value::String(s) => s.contains("add secure note"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("open secure note"))
                        }
                        serde_json::Value::String(s) => s.contains("open secure note"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^Secure Note\\s+\\(%{DATA:lastpass.event_report.data.secure_note}\\)$
                        // Grok pattern: ^Secure Note\\s+\\(%{DATA:lastpass.event_report.data.secure_note}\\)\\s+from\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^Secure Note\\s+\\(%{DATA:lastpass.event_report.data.secure_note}\\)$"
                                ),
                                cached_grok!(
                                    "^Secure Note\\s+\\(%{DATA:lastpass.event_report.data.secure_note}\\)\\s+from\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("edit secure note"))
                    }
                    serde_json::Value::String(s) => s.contains("edit secure note"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^Secure Note\\s+\\(%{DATA:lastpass.event_report.data.secure_note}\\)$
                        if !cached_grok!(
                            "^Secure Note\\s+\\(%{DATA:lastpass.event_report.data.secure_note}\\)$"
                        )
                        .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("add to shared folder"))
                    }
                    serde_json::Value::String(s) => s.contains("add to shared folder"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^'%{DATA:lastpass.event_report.data.shared_folder_name}'\\s+'%{EMAILADDRESS:lastpass.event_report.data.user_email}'$
                        if !cached_grok!("^'%{DATA:lastpass.event_report.data.shared_folder_name}'\\s+'%{EMAILADDRESS:lastpass.event_report.data.user_email}'$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("create group"))
                    }
                    serde_json::Value::String(s) => s.contains("create group"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^'%{DATA:lastpass.event_report.data.group_name}'$
                        if !cached_grok!("^'%{DATA:lastpass.event_report.data.group_name}'$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("adding user to group"))
                    }
                    serde_json::Value::String(s) => s.contains("adding user to group"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^%{EMAILADDRESS:lastpass.event_report.data.user_email}\\s+\\-\\s+%{DATA:lastpass.event_report.data.group_name}$
                        if !cached_grok!("^%{EMAILADDRESS:lastpass.event_report.data.user_email}\\s+\\-\\s+%{DATA:lastpass.event_report.data.group_name}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("created lastpass account")),
                    serde_json::Value::String(s) => s.contains("created lastpass account"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^%{EMAILADDRESS:lastpass.event_report.data.user_email}\\s*-Shared-\\s*%{DATA:lastpass.event_report.data.shared_folder_name}$
                        if !cached_grok!("^%{EMAILADDRESS:lastpass.event_report.data.user_email}\\s*-Shared-\\s*%{DATA:lastpass.event_report.data.shared_folder_name}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("update folder permissions")),
                    serde_json::Value::String(s) => s.contains("update folder permissions"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^'%{DATA:lastpass.event_report.data.shared_folder_name}'\\s+'%{EMAILADDRESS:lastpass.event_report.data.user_email}'\\s+'Read only:%{DATA:lastpass.event_report.data.shared_folder_user_permissions.read_only}\\s+Admin:%{DATA:lastpass.event_report.data.shared_folder_user_permissions.admin}\\s+Hide PW:%{DATA:lastpass.event_report.data.shared_folder_user_permissions.hide_password}'$
                        if !cached_grok!("^'%{DATA:lastpass.event_report.data.shared_folder_name}'\\s+'%{EMAILADDRESS:lastpass.event_report.data.user_email}'\\s+'Read only:%{DATA:lastpass.event_report.data.shared_folder_user_permissions.read_only}\\s+Admin:%{DATA:lastpass.event_report.data.shared_folder_user_permissions.admin}\\s+Hide PW:%{DATA:lastpass.event_report.data.shared_folder_user_permissions.hide_password}'$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("renamed shared folder")),
                    serde_json::Value::String(s) => s.contains("renamed shared folder"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^'%{DATA:lastpass.event_report.data.shared_folder_name}'\\s+'%{DATA:lastpass.event_report.data.renamed_shared_folder_name}'$
                        if !cached_grok!("^'%{DATA:lastpass.event_report.data.shared_folder_name}'\\s+'%{DATA:lastpass.event_report.data.renamed_shared_folder_name}'$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("move to shared folder")),
                    serde_json::Value::String(s) => s.contains("move to shared folder"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^\\s+to\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$
                        // Grok pattern: ^%{GREEDYDATA:lastpass.event_report.data.site}\\s+to\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^\\s+to\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$"
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:lastpass.event_report.data.site}\\s+to\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("move from shared folder")),
                    serde_json::Value::String(s) => s.contains("move from shared folder"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^ from INVALID SHARED FOLDER$
                        // Grok pattern: ^\\s+from\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$
                        // Grok pattern: ^%{GREEDYDATA:lastpass.event_report.data.site}\\s+from\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$
                        if !extract_first_match(
                            &[
                                cached_grok!("^ from INVALID SHARED FOLDER$"),
                                cached_grok!(
                                    "^\\s+from\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$"
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:lastpass.event_report.data.site}\\s+from\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("delete shared sites"))
                    }
                    serde_json::Value::String(s) => s.contains("delete shared sites"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^\\s+from\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$
                        // Grok pattern: ^%{GREEDYDATA:lastpass.event_report.data.deleted_site}\\s+from\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^\\s+from\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$"
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:lastpass.event_report.data.deleted_site}\\s+from\\s+%{DATA:lastpass.event_report.data.shared_folder_name}$"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("limit shared folder"))
                    }
                    serde_json::Value::String(s) => s.contains("limit shared folder"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp") {
                        // Grok pattern: ^%{DATA:lastpass.event_report.data.shared_folder_name},%{EMAILADDRESS:lastpass.event_report.data.user_email}$
                        if !cached_grok!("^%{DATA:lastpass.event_report.data.shared_folder_name},%{EMAILADDRESS:lastpass.event_report.data.user_email}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get("event.action").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("removed from shared folder")),
                    serde_json::Value::String(s) => s.contains("removed from shared folder"),
                    _ => false,
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("lastpass.event_report.data.original") {
                        // Grok pattern: ^'%{DATA:lastpass.event_report.data.shared_folder_name}'\\s+'%{EMAILADDRESS:lastpass.event_report.data.user_email}'$
                        if !cached_grok!("^'%{DATA:lastpass.event_report.data.shared_folder_name}'\\s+'%{EMAILADDRESS:lastpass.event_report.data.user_email}'$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            if let Some(v) = event
                .get("lastpass.event_report.data.group_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.group.name", v)?;
            }

            let _cond = {
                event.has_value("event.action")
                    && (event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("employee invited"))
                        }
                        serde_json::Value::String(s) => s.contains("employee invited"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account created")),
                        serde_json::Value::String(s) => s.contains("employee account created"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account deleted")),
                        serde_json::Value::String(s) => s.contains("employee account deleted"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("lastpass.event_report.data.user_email") {
                        foreach_array(event, "lastpass.event_report.data.user_email", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "user.email",
                                    json!(
                                        event
                                            .get("_ingest._value")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                !event.has_value("event.action")
                    || !(event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("employee invited"))
                        }
                        serde_json::Value::String(s) => s.contains("employee invited"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account created")),
                        serde_json::Value::String(s) => s.contains("employee account created"),
                        _ => false,
                    }) || event.get("event.action").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("employee account deleted")),
                        serde_json::Value::String(s) => s.contains("employee account deleted"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "user.email",
                        json!(
                            event
                                .get("lastpass.event_report.data.user_email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "user.email",
                    json!(
                        event
                            .get("lastpass.event_report.user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("user.email") {
                    foreach_array(event, "user.email", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                }
                Ok(())
            })();

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

            event.remove("json");
            event.remove("_temp");

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
                    event.remove("lastpass.event_report.time");
                    event.remove("lastpass.event_report.action");
                    event.remove("lastpass.event_report.ip");
                    event.remove("lastpass.event_report.user_name");
                    event.remove("lastpass.event_report.data.user_email");
                    event.remove("lastpass.event_report.data.group_name");
                    Ok(())
                })();
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
