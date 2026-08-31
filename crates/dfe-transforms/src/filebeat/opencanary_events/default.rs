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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                event.rename("message", "event.original")?;
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("message");
                    Ok(())
                })();
            }

            let _cond = { event.get("json.message").is_some_and(|v| v.is_string()) };
            if _cond {
                parse_json_field(event, "json.message", "json")?;
            }

            let _cond = { !event.has_value("json") && event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            event.rename("json", "opencanary")?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("opencanary.utc_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSSSSS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "opencanary.utc_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("event.kind", json!("event"))?;
                Ok(())
            })();

            let _cond = {
                event.has_value("opencanary.logtype")
                    && event
                        .get_i64("opencanary.logtype")
                        .is_some_and(|n| n >= 2000)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.kind", json!("alert"))?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("event.category", Value::Array(vec![json!("configuration")]))?;
                Ok(())
            })();

            let _cond = {
                event.has_value("opencanary.logtype")
                    && event
                        .get_i64("opencanary.logtype")
                        .is_some_and(|n| n >= 2000)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("network"), json!("intrusion_detection")]),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("opencanary.logtype") };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            let _cond = {
                event.has_value("opencanary.logtype")
                    && event
                        .get_i64("opencanary.logtype")
                        .is_some_and(|n| n >= 2000)
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("connection")]))?;
            }

            let _cond = {
                event
                    .get("opencanary.logdata")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has_value("opencanary.logdata") {
                    event.rename("opencanary.logdata", "opencanary.logdata.msg.logdata")?;
                }
            }

            if let Some(v) = event
                .get("opencanary.logdata.msg.logdata")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = { event.has_value("opencanary.logtype") };
            if _cond {
                // Painless script
                // Source: String logType = ctx.opencanary.logtype.toString(); if (ctx.log == null) {\n  ctx.log = new HashMap();\n} if (params.get(logType) == null) {\n  ctx.log['logger'] = logType;\n} else {\n  ctx.log['logger'] = params.get(logType);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"String logType = ctx.opencanary.logtype.toString(); if (ctx.log == null) {\n  ctx.log = new HashMap();\n} if (params.get(logType) == null) {\n  ctx.log['logger'] = logType;\n} else {\n  ctx.log['logger'] = params.get(logType);\n}\n"#
                    ),
                    cached_params!(
                        "{\"1000\":\"LOG_BASE_BOOT\",\"1001\":\"LOG_BASE_MSG\",\"1002\":\"LOG_BASE_DEBUG\",\"1003\":\"LOG_BASE_ERROR\",\"1004\":\"LOG_BASE_PING\",\"1005\":\"LOG_BASE_CONFIG_SAVE\",\"1006\":\"LOG_BASE_EXAMPLE\",\"2000\":\"LOG_FTP_LOGIN_ATTEMPT\",\"2001\":\"LOG_FTP_AUTH_ATTEMPT_INITIATED\",\"3000\":\"LOG_HTTP_GET\",\"3001\":\"LOG_HTTP_POST_LOGIN_ATTEMPT\",\"3002\":\"LOG_HTTP_UNIMPLEMENTED_METHOD\",\"3003\":\"LOG_HTTP_REDIRECT\",\"4000\":\"LOG_SSH_NEW_CONNECTION\",\"4001\":\"LOG_SSH_REMOTE_VERSION_SENT\",\"4002\":\"LOG_SSH_LOGIN_ATTEMPT\",\"5000\":\"LOG_SMB_FILE_OPEN\",\"5001\":\"LOG_PORT_SYN\",\"5002\":\"LOG_PORT_NMAPOS\",\"5003\":\"LOG_PORT_NMAPNULL\",\"5004\":\"LOG_PORT_NMAPXMAS\",\"5005\":\"LOG_PORT_NMAPFIN\",\"6001\":\"LOG_TELNET_LOGIN_ATTEMPT\",\"6002\":\"LOG_TELNET_CONNECTION_MADE\",\"7001\":\"LOG_HTTPPROXY_LOGIN_ATTEMPT\",\"8001\":\"LOG_MYSQL_LOGIN_ATTEMPT\",\"9001\":\"LOG_MSSQL_LOGIN_SQLAUTH\",\"9002\":\"LOG_MSSQL_LOGIN_WINAUTH\",\"9003\":\"LOG_MYSQL_CONNECTION_MADE\",\"10001\":\"LOG_TFTP\",\"11001\":\"LOG_NTP_MONLIST\",\"12001\":\"LOG_VNC\",\"13001\":\"LOG_SNMP_CMD\",\"14001\":\"LOG_RDP\",\"15001\":\"LOG_SIP_REQUEST\",\"16001\":\"LOG_GIT_CLONE_REQUEST\",\"17001\":\"LOG_REDIS_COMMAND\",\"18001\":\"LOG_TCP_BANNER_CONNECTION_MADE\",\"18002\":\"LOG_TCP_BANNER_KEEP_ALIVE_CONNECTION_MADE\",\"18003\":\"LOG_TCP_BANNER_KEEP_ALIVE_SECRET_RECEIVED\",\"18004\":\"LOG_TCP_BANNER_KEEP_ALIVE_DATA_RECEIVED\",\"18005\":\"LOG_TCP_BANNER_DATA_RECEIVED\",\"19001\":\"LOG_LLMNR_QUERY_RESPONSE\",\"99000\":\"LOG_USER_0\",\"99001\":\"LOG_USER_1\",\"99002\":\"LOG_USER_2\",\"99003\":\"LOG_USER_3\",\"99004\":\"LOG_USER_4\",\"99005\":\"LOG_USER_5\",\"99006\":\"LOG_USER_6\",\"99007\":\"LOG_USER_7\",\"99008\":\"LOG_USER_8\",\"99009\":\"LOG_USER_9\"}"
                    ),
                )?;
            }

            let _cond = { !event.has_value("opencanary.logtype") };
            if _cond {
                event.set("log.logger", json!("LOG_BASE_ERROR"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.node_id") {
                    event.rename("opencanary.node_id", "opencanary.node.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.logdata.SKIN") {
                    event.rename("opencanary.logdata.SKIN", "opencanary.skin")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.logdata.ID") {
                    if let Some(val) = event.get("opencanary.logdata.ID") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "opencanary.logdata.ID".into(),
                                message,
                            }
                        })?;
                        event.set("opencanary.logdata.ID", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.logdata.TTL") {
                    if let Some(val) = event.get("opencanary.logdata.TTL") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "opencanary.logdata.TTL".into(),
                                message,
                            }
                        })?;
                        event.set("opencanary.logdata.TTL", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.logdata.URGP") {
                    if let Some(val) = event.get("opencanary.logdata.URGP") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "opencanary.logdata.URGP".into(),
                                message,
                            }
                        })?;
                        event.set("opencanary.logdata.URGP", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.logdata.WINDOW") {
                    if let Some(val) = event.get("opencanary.logdata.WINDOW") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "opencanary.logdata.WINDOW".into(),
                                message,
                            }
                        })?;
                        event.set("opencanary.logdata.WINDOW", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.get_str("log.logger") == Some("LOG_MSSQL_LOGIN_SQLAUTH") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.AppName") {
                        event
                            .rename("opencanary.logdata.AppName", "opencanary.mssql.client.app")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_MSSQL_LOGIN_SQLAUTH") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.HostName") {
                        event.rename(
                            "opencanary.logdata.HostName",
                            "opencanary.mssql.client.hostname",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_MSSQL_LOGIN_SQLAUTH") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.CltIntName") {
                        event.rename(
                            "opencanary.logdata.CltIntName",
                            "opencanary.mssql.client.interface_library",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_MSSQL_LOGIN_SQLAUTH") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.Database") {
                        event.rename("opencanary.logdata.Database", "opencanary.mssql.database")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_MSSQL_LOGIN_SQLAUTH") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("opencanary.logdata.ServerName");
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_REDIS_COMMAND") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.ARGS") {
                        event.rename("opencanary.logdata.ARGS", "opencanary.redis.args")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_REDIS_COMMAND") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.CMD") {
                        event.rename("opencanary.logdata.CMD", "opencanary.redis.command")?;
                    }
                    Ok(())
                })();
            }

            if let Some(v) = event
                .get("opencanary.logdata.AUDITACTION")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_SMB_FILE_OPEN") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.FILENAME") {
                        event.rename("opencanary.logdata.FILENAME", "opencanary.smb.filename")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_SMB_FILE_OPEN") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.SHARENAME") {
                        event
                            .rename("opencanary.logdata.SHARENAME", "opencanary.smb.share_name")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_SMB_FILE_OPEN") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.SMBARCH") {
                        event.rename("opencanary.logdata.SMBARCH", "opencanary.smb.smb_arch")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_SMB_FILE_OPEN") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.SMBVER") {
                        event.rename("opencanary.logdata.SMBVER", "opencanary.smb.smb_version")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_SMB_FILE_OPEN") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.STATUS") {
                        event.rename("opencanary.logdata.STATUS", "opencanary.smb.status")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("log.logger")
                    && (event.get_str("log.logger") == Some("LOG_SSH_NEW_CONNECTION")
                        || event.get_str("log.logger") == Some("LOG_SSH_LOGIN_ATTEMPT")
                        || event.get_str("log.logger") == Some("LOG_SSH_REMOTE_VERSION_SENT"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.LOCALVERSION") {
                        event.rename(
                            "opencanary.logdata.LOCALVERSION",
                            "opencanary.ssh.local_version",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("log.logger")
                    && (event.get_str("log.logger") == Some("LOG_SSH_NEW_CONNECTION")
                        || event.get_str("log.logger") == Some("LOG_SSH_LOGIN_ATTEMPT")
                        || event.get_str("log.logger") == Some("LOG_SSH_REMOTE_VERSION_SENT"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.REMOTEVERSION") {
                        event.rename(
                            "opencanary.logdata.REMOTEVERSION",
                            "opencanary.ssh.remote_version",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("log.logger")
                    && (event.get_str("log.logger")
                        == Some("LOG_TCP_BANNER_KEEP_ALIVE_CONNECTION_MADE")
                        || event.get_str("log.logger")
                            == Some("LOG_TCP_BANNER_KEEP_ALIVE_SECRET_RECEIVED")
                        || event.get_str("log.logger")
                            == Some("LOG_TCP_BANNER_KEEP_ALIVE_SECRET_RECEIVED"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.BANNER_ID") {
                        event.rename(
                            "opencanary.logdata.BANNER_ID",
                            "opencanary.tcp_banner.banner_id",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("log.logger")
                    && (event.get_str("log.logger")
                        == Some("LOG_TCP_BANNER_KEEP_ALIVE_CONNECTION_MADE")
                        || event.get_str("log.logger")
                            == Some("LOG_TCP_BANNER_KEEP_ALIVE_SECRET_RECEIVED")
                        || event.get_str("log.logger")
                            == Some("LOG_TCP_BANNER_KEEP_ALIVE_SECRET_RECEIVED"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.DATA") {
                        event.rename("opencanary.logdata.DATA", "opencanary.tcp_banner.data")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("log.logger")
                    && (event.get_str("log.logger")
                        == Some("LOG_TCP_BANNER_KEEP_ALIVE_CONNECTION_MADE")
                        || event.get_str("log.logger")
                            == Some("LOG_TCP_BANNER_KEEP_ALIVE_SECRET_RECEIVED")
                        || event.get_str("log.logger")
                            == Some("LOG_TCP_BANNER_KEEP_ALIVE_SECRET_RECEIVED"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.FUNCTION") {
                        event.rename(
                            "opencanary.logdata.FUNCTION",
                            "opencanary.tcp_banner.function",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("log.logger")
                    && (event.get_str("log.logger")
                        == Some("LOG_TCP_BANNER_KEEP_ALIVE_CONNECTION_MADE")
                        || event.get_str("log.logger")
                            == Some("LOG_TCP_BANNER_KEEP_ALIVE_SECRET_RECEIVED")
                        || event.get_str("log.logger")
                            == Some("LOG_TCP_BANNER_KEEP_ALIVE_SECRET_RECEIVED"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.SECRET_STRING") {
                        event.rename(
                            "opencanary.logdata.SECRET_STRING",
                            "opencanary.tcp_banner.secret_string",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_TFTP") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.FILENAME") {
                        event.rename("opencanary.logdata.FILENAME", "opencanary.tftp.filename")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_TFTP") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.MODE") {
                        event.rename("opencanary.logdata.MODE", "opencanary.tftp.node")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_TFTP") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.OPCODE") {
                        event.rename("opencanary.logdata.OPCODE", "opencanary.tftp.opcode")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_VNC") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.VNC Client Response") {
                        event.rename(
                            "opencanary.logdata.VNC Client Response",
                            "opencanary.vnc.client_response",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_VNC") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.VNC Password") {
                        event
                            .rename("opencanary.logdata.VNC Password", "opencanary.vnc.password")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("opencanary.vnc.password")
                    == Some("<Password was not in the common list>")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("opencanary.vnc.password");
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_VNC") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.VNC Server Challenge") {
                        event.rename(
                            "opencanary.logdata.VNC Server Challenge",
                            "opencanary.vnc.server_challenge",
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("log.logger") == Some("LOG_NTP_MONLIST") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("opencanary.logdata.NTP CMD") {
                        event.rename("opencanary.logdata.NTP CMD", "opencanary.ntp.cmd")?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("event.created", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("log.logger")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.provider", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("event.start", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.dst_port") {
                    if let Some(val) = event.get("opencanary.dst_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "opencanary.dst_port".into(),
                                message,
                            }
                        })?;
                        event.set("destination.port", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("opencanary.dst_host")
                    && event.get_str("opencanary.dst_host") != Some("")
                    && event.get_str("opencanary.dst_host") != Some("0.0.0.0")
            };
            if _cond {
                if let Some(v) = event
                    .get("opencanary.dst_host")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.address", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.address") {
                    if let Some(val) = event.get("destination.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.address".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_address",
                )?;
                if let Some(v) = event.get("destination.address").cloned() {
                    event.set("destination.domain", v)?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event
                .get("opencanary.logdata.LOCALNAME")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            if let Some(v) = event
                .get("opencanary.logdata.HOSTNAME")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.src_port") {
                    if let Some(val) = event.get("opencanary.src_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "opencanary.src_port".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("opencanary.src_host")
                    && event.get_str("opencanary.src_host") != Some("")
                    && event.get_str("opencanary.src_host") != Some("0.0.0.0")
            };
            if _cond {
                if let Some(v) = event
                    .get("opencanary.src_host")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.address", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_address")?;
                if let Some(v) = event.get("source.address").cloned() {
                    event.set("source.domain", v)?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.logdata.MAC") {
                    gsub_field(
                        event,
                        "opencanary.logdata.MAC",
                        "opencanary.logdata.MAC",
                        cached_regex!(":"),
                        "-",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.logdata.MAC") {
                    map_strings(
                        event,
                        "opencanary.logdata.MAC",
                        "opencanary.logdata.MAC",
                        str::to_uppercase,
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.logdata.MAC") {
                    if let Some(input) = event.get_string("opencanary.logdata.MAC") {
                        // Grok pattern: (?P<destination_mac>(?:(?:(?:[0-9A-F]{2}-){5}[0-9A-F]{2})))-(?P<source_mac>(?:(?:(?:[0-9A-F]{2}-){5}[0-9A-F]{2})))%{GREEDYDATA}
                        if !cached_grok_mapped!("(?P<destination_mac>(?:(?:(?:[0-9A-F]{2}-){5}[0-9A-F]{2})))-(?P<source_mac>(?:(?:(?:[0-9A-F]{2}-){5}[0-9A-F]{2})))%{GREEDYDATA}", [("destination_mac", "destination.mac"), ("source_mac", "source.mac")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                }
                Ok(())
            })();

            let _cond = {
                (!event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    })))
                    && event.has_value("source.mac")
                    && event.has_value("destination.mac")
            };
            if _cond {
                event.remove("opencanary.logdata.MAC");
            }

            if let Some(v) = event
                .get("opencanary.logdata.REMOTENAME")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            if let Some(v) = event
                .get("opencanary.logdata.PROTO")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.transport", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.ip") {
                    // Classify network direction against the internal network ranges
                    if let (Some(src), Some(dst)) = (
                        event.get_string("source.ip"),
                        event.get_string("destination.ip"),
                    ) {
                        let networks: Vec<&str> = vec![
                            "loopback",
                            "unicast",
                            "multicast",
                            "interface_local_multicast",
                            "link_local_unicast",
                            "link_local_multicast",
                            "private",
                            "unspecified",
                        ];
                        let direction = match (
                            ip_in_networks(&src, &networks),
                            ip_in_networks(&dst, &networks),
                        ) {
                            (true, false) => "outbound",
                            (false, true) => "inbound",
                            (true, true) => "internal",
                            (false, false) => "external",
                        };
                        event.set("network.direction", json!(direction))?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("opencanary.logdata.IN")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.ingress.interface.name", v)?;
            }

            if let Some(v) = event
                .get("opencanary.logdata.OUT")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.egress.interface.name", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("opencanary.logdata.PATH") {
                    uri_parts(event, "opencanary.logdata.PATH", "url", true, false)?;
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("opencanary.logdata.USERNAME")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("opencanary.logdata.USER")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("opencanary.logdata.UserName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("opencanary.logdata.DOMAIN")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            if event.has_value("opencanary.logdata.USERAGENT") {
                if let Some(ua_str) = event.get_string("opencanary.logdata.USERAGENT") {
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("redact_passwords"))
                        }
                        serde_json::Value::String(s) => s.contains("redact_passwords"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("opencanary.logdata.PASSWORD");
                    event.remove("opencanary.vnc.password");
                    Ok(())
                })();
            }

            // SKIPPED: condition not transpiled: ctx.tags != null && ctx.tags.contains("redact_passwords") && ctx.event?.original =~ /(?i)password\\*"\: *\\*"[^\\"]+?/
            #[allow(unreachable_code, unused_variables)]
            if false {
                if event.has_value("event.original") {
                    gsub_field(
                        event,
                        "event.original",
                        "event.original",
                        cached_regex!("((?i)password\\\\*\": *\\\\*\")(.+?)(\\\\*\\\" *[,}])"),
                        "$1<REDACTED>$3",
                    )?;
                }
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

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
                    event.remove("opencanary.utc_time");
                    event.remove("opencanary.local_time");
                    event.remove("opencanary.local_time_adjusted");
                    event.remove("opencanary.logdata.AUDITACTION");
                    event.remove("opencanary.logdata.DOMAIN");
                    event.remove("opencanary.logdata.HOSTNAME");
                    event.remove("opencanary.logdata.IN");
                    event.remove("opencanary.logdata.LOCALNAME");
                    event.remove("opencanary.logdata.OUT");
                    event.remove("opencanary.logdata.PATH");
                    event.remove("opencanary.logdata.PROTO");
                    event.remove("opencanary.logdata.REMOTENAME");
                    event.remove("opencanary.logdata.USERAGENT");
                    event.remove("opencanary.logdata.USERNAME");
                    event.remove("opencanary.logdata.USER");
                    event.remove("opencanary.logdata.UserName");
                    event.remove("opencanary.logdata.msg.logdata");
                    event.remove("opencanary.dst_host");
                    event.remove("opencanary.dst_port");
                    event.remove("opencanary.src_host");
                    event.remove("opencanary.src_port");
                    Ok(())
                })();
            }

            let _cond = { event.get_i64("source.port") == Some(-1) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("source.port");
                    Ok(())
                })();
            }

            let _cond = { event.get_i64("destination.port") == Some(-1) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("destination.port");
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("opencanary.logdata")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                foreach_array(event, "opencanary.logdata", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        map_strings(event, "_ingest._key", "_ingest._key", str::to_lowercase)?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'undefined') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'undefined') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
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
                        "Processor {} with tag {} in pipeline {} failed with message {}",
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
