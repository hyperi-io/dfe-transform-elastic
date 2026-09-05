// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `login` pipeline.
pub struct Login;

impl Transform for Login {
    fn name(&self) -> &str {
        "login"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        let _cond = {
            event.get("message").is_some_and(|v| match v {
                serde_json::Value::Array(a) => {
                    a.iter().any(|x| x.as_str() == Some("Authenticated user"))
                }
                serde_json::Value::String(s) => s.contains("Authenticated user"),
                _ => false,
            })
        };
        if _cond {
            if let Some(input) = event.get_string("message") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find(" user ") else {
                        break 'dissect false;
                    };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" user ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
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
                } else {
                    return Err(TransformError::ParseError {
                        path: "message".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        let _cond = {
            event.has_value("message")
                && (event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("logged in as"))
                    }
                    serde_json::Value::String(s) => s.contains("logged in as"),
                    _ => false,
                }) && event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("User ")),
                    serde_json::Value::String(s) => s.contains("User "),
                    _ => false,
                }))
        };
        if _cond {
            if let Some(input) = event.get_string("message") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find("User ") else {
                        break 'dissect false;
                    };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("User ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find("@") else {
                        break 'dissect false;
                    };
                    captured.push(("user.name", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("@") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" ") else {
                        break 'dissect false;
                    };
                    captured.push(("client.ip", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" as ") else {
                        break 'dissect false;
                    };
                    captured.push(("_tmp.event", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" as ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    captured.push(("user_agent.original", remaining));
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "message".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        let _cond = {
            event.has_value("message")
                && (event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("logged in as"))
                    }
                    serde_json::Value::String(s) => s.contains("logged in as"),
                    _ => false,
                }) && event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("Shared secret"))
                    }
                    serde_json::Value::String(s) => s.contains("Shared secret"),
                    _ => false,
                }))
        };
        if _cond {
            if let Some(input) = event.get_string("message") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find(" Shared secret from ") else {
                        break 'dissect false;
                    };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" Shared secret from ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" ") else {
                        break 'dissect false;
                    };
                    captured.push(("client.ip", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" as ") else {
                        break 'dissect false;
                    };
                    captured.push(("_tmp.event", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" as ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    captured.push(("user_agent.original", remaining));
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "message".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        let _cond = {
            event.has_value("message")
                && (event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("logged in as"))
                    }
                    serde_json::Value::String(s) => s.contains("logged in as"),
                    _ => false,
                }) && event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("SSL thumbprint"))
                    }
                    serde_json::Value::String(s) => s.contains("SSL thumbprint"),
                    _ => false,
                }))
        };
        if _cond {
            if let Some(input) = event.get_string("message") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find(" SSL thumbprint logged in as ") else {
                        break 'dissect false;
                    };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" SSL thumbprint logged in as ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    captured.push(("user_agent.original", remaining));
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "message".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        let _cond = {
            event.has_value("message")
                && (event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("logged in as"))
                    }
                    serde_json::Value::String(s) => s.contains("logged in as"),
                    _ => false,
                }) && event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("Shared secret"))
                    }
                    serde_json::Value::String(s) => s.contains("Shared secret"),
                    _ => false,
                }))
        };
        if _cond {
            event.set("user.name", json!("shared_secret_login"))?;
        }

        let _cond = {
            event.has_value("message")
                && (event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("logged in as"))
                    }
                    serde_json::Value::String(s) => s.contains("logged in as"),
                    _ => false,
                }) && event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("SSL thumbprint"))
                    }
                    serde_json::Value::String(s) => s.contains("SSL thumbprint"),
                    _ => false,
                }))
        };
        if _cond {
            event.set("user.name", json!("ssl_thumbprint_login"))?;
        }

        let _cond = {
            event.has_value("message")
                && (event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("logged in as"))
                    }
                    serde_json::Value::String(s) => s.contains("logged in as"),
                    _ => false,
                }) && !(event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("Shared secret"))
                    }
                    serde_json::Value::String(s) => s.contains("Shared secret"),
                    _ => false,
                })) && !(event.get("message").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("User ")),
                    serde_json::Value::String(s) => s.contains("User "),
                    _ => false,
                })))
        };
        if _cond {
            event.set("_tmp.event", json!("logged in"))?;
        }

        let _cond = {
            event.get("message").is_some_and(|v| match v {
                serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("logged out")),
                serde_json::Value::String(s) => s.contains("logged out"),
                _ => false,
            })
        };
        if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("User ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("User ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("@") else {
                            break 'dissect false;
                        };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("@") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("client.ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" out (login time: ") else {
                            break 'dissect false;
                        };
                        captured.push(("_tmp.event", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" out (login time: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", number ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.start", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", number ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(": ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(": ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", ") else {
                            break 'dissect false;
                        };
                        captured.push(("vsphere.log.api.invocations", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(": ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(": ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("user_agent.original", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
        }

        let _cond = {
            event.get("message").is_some_and(|v| match v {
                serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("logged out")),
                serde_json::Value::String(s) => s.contains("logged out"),
                _ => false,
            })
        };
        if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("User {Name: ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("User {Name: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Domain: ") else {
                            break 'dissect false;
                        };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Domain: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("} ") else {
                            break 'dissect false;
                        };
                        captured.push(("user.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("} ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
        }

        let _cond = {
            event.get("message").is_some_and(|v| match v {
                serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Failed login")),
                serde_json::Value::String(s) => s.contains("Failed login"),
                _ => false,
            })
        };
        if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("[") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("] [] [") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] [] [") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("] [") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("] [") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" login ") else {
                            break 'dissect false;
                        };
                        captured.push(("event.outcome", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" login ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("client.ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
        }

        let _cond = {
            event
                .get_str("message")
                .is_some_and(|s| s.starts_with("Received"))
        };
        if _cond {
            if let Some(input) = event.get_string("message") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(rest) = remaining.strip_prefix("Received ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" from ") else {
                        break 'dissect false;
                    };
                    captured.push(("_tmp.status", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" from ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" port ") else {
                        break 'dissect false;
                    };
                    captured.push(("client.ip", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" port ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(":") else {
                        break 'dissect false;
                    };
                    captured.push(("client.port", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(":") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(": ") else {
                        break 'dissect false;
                    };
                    captured.push(("destination.port", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(": ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "message".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        let _cond = {
            event
                .get_str("message")
                .is_some_and(|s| s.starts_with("Disconnected"))
        };
        if _cond {
            if let Some(input) = event.get_string("message") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find(" from ") else {
                        break 'dissect false;
                    };
                    captured.push(("_tmp.status", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" from ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" port ") else {
                        break 'dissect false;
                    };
                    captured.push(("client.ip", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" port ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" ") else {
                        break 'dissect false;
                    };
                    captured.push(("client.port", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "message".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        let _cond = {
            event
                .get_str("message")
                .is_some_and(|s| s.starts_with("Connection"))
        };
        if _cond {
            if let Some(input) = event.get_string("message") {
                // Grok pattern: Connection %{DATA:_tmp.status} by %{IPORHOST:client.ip} port %{POSINT:client.port}( %{GREEDYDATA})?$
                if !cached_grok!("Connection %{DATA:_tmp.status} by %{IPORHOST:client.ip} port %{POSINT:client.port}( %{GREEDYDATA})?$").extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
        }

        let _cond = {
            event.get("message").is_some_and(|v| match v {
                serde_json::Value::Array(a) => {
                    a.iter().any(|x| x.as_str() == Some("Logged in user:"))
                }
                serde_json::Value::String(s) => s.contains("Logged in user:"),
                _ => false,
            })
        };
        if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" Logged in user: \"") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Logged in user: \"") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("\\") else {
                            break 'dissect false;
                        };
                        captured.push(("user.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("\\") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("\"") else {
                            break 'dissect false;
                        };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("\"") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
        }

        let _cond = {
            event.get("message").is_some_and(|v| match v {
                serde_json::Value::Array(a) => a
                    .iter()
                    .any(|x| x.as_str() == Some("logged in successfully")),
                serde_json::Value::String(s) => s.contains("logged in successfully"),
                _ => false,
            })
        };
        if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" User {Name: ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" User {Name: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", Domain: ") else {
                            break 'dissect false;
                        };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", Domain: ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("} ") else {
                            break 'dissect false;
                        };
                        captured.push(("user.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("} ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();
        }

        let _cond = {
            event.get("user.name").is_some_and(|v| match v {
                serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("\\")),
                serde_json::Value::String(s) => s.contains("\\"),
                _ => false,
            })
        };
        if _cond {
            if let Some(input) = event.get_string("user.name") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find("\\") else {
                        break 'dissect false;
                    };
                    captured.push(("user.domain", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\\") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    captured.push(("user.name", remaining));
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "user.name".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }
        }

        let _cond = {
            !event.has_value("event.outcome")
                && (event.has_value("user.name") || event.has_value("user.domain"))
        };
        if _cond {
            event.set("event.outcome", json!("success"))?;
        }

        // SKIPPED: condition not transpiled: ctx.event?.outcome?.toLowerCase()?.startsWith('f') ?: false
        #[allow(unreachable_code, unused_variables)]
        if false {
            event.set("event.outcome", json!("failure"))?;
        }

        // SKIPPED: condition not transpiled: ctx.user_agent?.original != null && (ctx.user_agent.original.contains(')') || ctx.user_agent.original.contains(']'))
        #[allow(unreachable_code, unused_variables)]
        if false {
            if let Some(input) = event.get_string("user_agent.original") {
                // Grok pattern: %{DATA:user_agent.original}(?:\\]|\\)+)
                if !cached_grok!("%{DATA:user_agent.original}(?:\\]|\\)+)")
                    .extract_into(&input, event)?
                {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
        }

        let _cond = { event.has_value("user_agent.original") };
        if _cond {
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

        let _cond = { event.has_value("event.start") };
        if _cond {
            event.set(
                "event.end",
                json!(
                    event
                        .get("@timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("event.start") };
        if _cond {
            if let Some(date_str) = event.get_as_string("event.start") {
                match parse_date_out(&date_str, &["EEEE, dd MMMM, yyyy hh:mm:ss a"], None, None) {
                    Some(parsed) => event.set("event.start", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "event.start".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
        }

        let _cond = { event.has_value("event.end") && event.has_value("event.start") };
        if _cond {
            // Painless script
            // Source: ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\n ZonedDateTime end = ZonedDateTime.parse(ctx.event.end);\n ctx.event.duration= ChronoUnit.NANOS.between(start, end);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\n ZonedDateTime end = ZonedDateTime.parse(ctx.event.end);\n ctx.event.duration= ChronoUnit.NANOS.between(start, end);"#
                ),
            )?;
        }

        let _cond = {
            event
                .get_str("message")
                .is_some_and(|s| s.to_lowercase().contains("logged in"))
        };
        if _cond {
            event.set("event.action", json!("login"))?;
        }

        let _cond = {
            event
                .get_str("message")
                .is_some_and(|s| s.to_lowercase().contains("logged out"))
        };
        if _cond {
            event.set("event.action", json!("logout"))?;
        }

        event.append("event.type", json!("info"))?;

        event.append("event.category", json!("authentication"))?;

        let _cond = { event.has_value("client.port") };
        if _cond {
            if let Some(val) = event.get("client.port") {
                let converted =
                    convert_value(val, "long").map_err(|message| TransformError::ParseError {
                        path: "client.port".into(),
                        message,
                    })?;
                event.set("client.port", converted)?;
            }
        }

        let _cond = { event.has_value("destination.port") };
        if _cond {
            if let Some(val) = event.get("destination.port") {
                let converted =
                    convert_value(val, "long").map_err(|message| TransformError::ParseError {
                        path: "destination.port".into(),
                        message,
                    })?;
                event.set("destination.port", converted)?;
            }
        }

        let _cond = {
            event
                .get("vsphere.log.api.invocations")
                .is_some_and(|v| v.is_string())
                && event
                    .get("vsphere.log.api.invocations")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    })
        };
        if _cond {
            gsub_field(
                event,
                "vsphere.log.api.invocations",
                "vsphere.log.api.invocations",
                cached_regex!(",|\\."),
                "",
            )?;
        }

        let _cond = { event.has_value("vsphere.log.api.invocations") };
        if _cond {
            if let Some(val) = event.get("vsphere.log.api.invocations") {
                let converted =
                    convert_value(val, "long").map_err(|message| TransformError::ParseError {
                        path: "vsphere.log.api.invocations".into(),
                        message,
                    })?;
                event.set("vsphere.log.api.invocations", converted)?;
            }
        }

        let _cond = { event.has_value("client.ip") };
        if _cond {
            if let Some(v) = event.get("client.ip").cloned() {
                event.set("source.ip", v)?;
            }
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

        Ok(TransformResult::Continue)
    }
}
