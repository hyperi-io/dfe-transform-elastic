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
            if event.has_value("event.code") {
                if let Some(val) = event.get("event.code") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "event.code".into(),
                            message,
                        }
                    })?;
                    event.set("event.code", converted)?;
                }
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: %{DATA:winlog.event_data.Message}\\|%{GREEDYDATA:kvpairs}
                if !cached_grok!("%{DATA:winlog.event_data.Message}\\|%{GREEDYDATA:kvpairs}")
                    .extract_into(&input, event)?
                {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            if let Some(kv_str) = event.get_string("kvpairs") {
                for pair in cached_regex!("\\|").split(&kv_str).into_iter() {
                    if pair.trim().is_empty() {
                        continue;
                    }
                    let Some((key, value)) = pair.split_once("=") else {
                        return Err(TransformError::ParseError {
                            path: "kvpairs".into(),
                            message: format!("does not contain value_split: {pair}"),
                        });
                    };
                    {
                        if !key.is_empty() {
                            kv_put(event, &format!("winlog.event_data.{}", key), value)?;
                        }
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("kvpairs");
                Ok(())
            })();

            if event.has_value("winlog.event_data.ClientIPs") {
                if let Some(s) = event.get_string("winlog.event_data.ClientIPs") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("winlog.event_data.ClientIPs", Value::Array(parts))?;
                }
            }

            if event.has_value("winlog.event_data.FailedTargets") {
                if let Some(s) = event.get_string("winlog.event_data.FailedTargets") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("winlog.event_data.FailedTargets", Value::Array(parts))?;
                }
            }

            // Painless script
            // Source: if (ctx?.winlog?.event_id == null) {\n  return;\n}\ndef t = params.get(ctx.winlog.event_id);\nif (t == null) {\n  return;\n}\nctx.winlog.put(\"symbolic_id\", t)
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx?.winlog?.event_id == null) {\n  return;\n}\ndef t = params.get(ctx.winlog.event_id);\nif (t == null) {\n  return;\n}\nctx.winlog.put(\"symbolic_id\", t)"#
                ),
                cached_params!(
                    "{\"1\":\"AUTH_CHAIN_FAILURE\",\"2\":\"AUTH_CHAIN_SUCCESS\",\"3\":\"USER_LOGIN_LOCKOUT\",\"4\":\"DB_COMMIT_SUSPEND\",\"5\":\"DB_COMMIT_RESUME\",\"6\":\"DB_REPLICATION_CONN_FAILURE\",\"7\":\"DB_REPLICATION_CONN_RESTORED\",\"8\":\"DB_REPLICATION_TRANS_FAILURE\",\"9\":\"DB_QUEUE_INSERT_FAILURE\",\"10\":\"DB_FAILED_PROC_RECORDED\",\"11\":\"PAMSA_ORCHESTRATION_START_FAILURE\",\"12\":\"PAMSA_ORCHESTRATION_END_FAILURE\",\"13\":\"UPDATE_RESOURCE_FAILURE\",\"14\":\"GSET_CHECKIN_FAILURE\",\"15\":\"GSET_CHECKIN_PARTIAL\",\"16\":\"GSET_CHECKIN_SUCCESS\",\"17\":\"GSET_CHECKOUT_SUCCESS\",\"18\":\"GSET_CHECKOUT_FAILURE\",\"19\":\"GSET_CHECKOUT_PARTIAL\",\"20\":\"PWD_CHECKOUT_SUCCESS\",\"21\":\"PWD_CHECKOUT_FAILURE\",\"22\":\"PWD_CHECKIN_SUCCESS\",\"23\":\"PWD_CHECKIN_FAILURE\",\"24\":\"WSTN_VIEW_PASSWORD_SUCCESS\",\"25\":\"WSTN_VIEW_PASSWORD_FAILURE\",\"26\":\"WSTN_VIEW_PASSWORD_HIS_SUCCESS\",\"27\":\"WSTN_VIEW_PASSWORD_HIS_FAILURE\",\"28\":\"ADMIN_ENABLE_ADMIN\",\"29\":\"ADMIN_ENABLE_USER\",\"30\":\"ADMIN_DISABLE_ADMIN\",\"31\":\"ADMIN_DISABLE_USER\",\"32\":\"ADMIN_UNLOCK_ADMIN\",\"33\":\"ADMIN_UNLOCK_USER\",\"34\":\"SMON_SESSION_START\",\"35\":\"SMON_SESSION_END\",\"36\":\"SMON_ADMIN_SESS_TERM_REQ\",\"37\":\"PSUPDATE_START\",\"38\":\"PSUPDATE_FINISH\",\"39\":\"IDAPI_LOGIN_SUCCESS\",\"40\":\"IDAPI_LOGIN_FAILURE\",\"41\":\"MAQ_CHECKIN_FAILURE\",\"42\":\"MAQ_CHECKIN_SUCCESS\",\"43\":\"MAQ_CHECKOUT_FAILURE\",\"44\":\"MAQ_CHECKOUT_SUCCESS\",\"45\":\"TARGET_DEPLOYMENT_FAILURE\",\"46\":\"TARGET_DEPLOYMENT_SUCCESS\",\"47\":\"OPERATION_IMPORT_TARGET\",\"48\":\"WSTN_ADD_WSTN_SUCCESS\",\"49\":\"WSTN_ADD_WSTN_FAILURE\",\"50\":\"IDWFM_EVENT_ABORT\",\"51\":\"IDWFM_EVENT_FAILURE\",\"52\":\"USER_QA_ADD_SUCCESS\",\"53\":\"USER_QA_ADD_FAILURE\",\"54\":\"USER_QA_UPDATE_SUCCESS\",\"55\":\"USER_QA_UPDATE_FAILURE\",\"56\":\"USER_QA_DELETE_SUCCESS\",\"57\":\"ADMIN_QA_ADD_SUCCESS\",\"58\":\"ADMIN_QA_ADD_FAILURE\",\"59\":\"ADMIN_QA_UPDATE_SUCCESS\",\"60\":\"ADMIN_QA_UPDATE_FAILURE\",\"61\":\"ADMIN_QA_DELETE_SUCCESS\",\"62\":\"USER_PW_RESET_START\",\"63\":\"USER_PW_RESET_SUCCESS\",\"64\":\"USER_PW_RESET_FAILURE\",\"65\":\"ADMIN_PW_RESET_START\",\"66\":\"ADMIN_PW_RESET_SUCCESS\",\"67\":\"ADMIN_PW_RESET_FAILURE\",\"68\":\"USER_ACCT_UNLOCK_START\",\"69\":\"USER_ACCT_UNLOCK_SUCCESS\",\"70\":\"USER_ACCT_UNLOCK_FAILURE\",\"71\":\"ADMIN_ACCT_UNLOCK_START\",\"72\":\"ADMIN_ACCT_UNLOCK_SUCCESS\",\"73\":\"ADMIN_ACCT_UNLOCK_FAILURE\",\"74\":\"DB_REPLICATION_WATERMARK_WARN\",\"75\":\"USER_ALIAS_ALREADY_CLAIMED\",\"76\":\"ADMIN_ALIAS_ALREADY_CLAIMED\",\"77\":\"CONNECTOR_TIMEOUT\",\"78\":\"FILE_REPLICATION_FAILURE\",\"79\":\"IDPM_GROUP_SUCCESS\",\"80\":\"IDPM_GROUP_FAILURE\",\"81\":\"WF_REQUEST_BATCH_APPROVED\",\"82\":\"WF_REQUEST_BATCH_REJECTED\",\"83\":\"WF_REQUEST_BATCH_CANCELED\",\"84\":\"WF_REQUEST_BATCH_REVOKED\",\"85\":\"WF_REQUEST_BATCH_PROCESSED\",\"86\":\"DID_REGISTER_SUCCESS\",\"87\":\"DID_REGISTER_FAILURE\",\"88\":\"DID_UPDATE_SUCCESS\",\"89\":\"DID_SEND_SUCCESS\",\"90\":\"USER_IDENTIFY_SUCCESS\",\"91\":\"USER_IDENTIFY_FAILURE\",\"92\":\"USER_LOGIN_SUCCESS\",\"93\":\"USER_LOGIN_FAILURE\",\"94\":\"FEDIDP_IDENTIFY_SUCCESS\",\"95\":\"FEDIDP_IDENTIFY_FAILURE\",\"96\":\"FEDIDP_AUTH_SUCCESS\",\"97\":\"FEDIDP_AUTH_FAILURE\",\"98\":\"DB_STORED_PROC_FAILURE\",\"99\":\"ADMIN_CRED_FAILURE\",\"100\":\"ADMIN_CRED_SUCCESS\",\"101\":\"FEDIDP_SSO_SESSION_CREATE\",\"102\":\"FEDIDP_SSO_SESSION_DESTROY\",\"103\":\"PAM_CHECKOUT_SUCCESS\",\"104\":\"PAM_CHECKOUT_PARTIAL\",\"105\":\"PAM_CHECKOUT_FAILURE\",\"106\":\"PAM_CHECKIN_SUCCESS\",\"107\":\"PAM_CHECKIN_PARTIAL\",\"108\":\"PAM_CHECKIN_FAILURE\",\"109\":\"PAM_CHECKOUT_EXPIRY\",\"110\":\"PAM_CHECKOUT_LIMIT_REACHED\",\"111\":\"PAM_CHECKOUT_OPERATION_SUCCESS\",\"112\":\"PAM_CHECKOUT_OPERATION_FAILURE\",\"113\":\"PAM_CHECKIN_OPERATION_SUCCESS\",\"114\":\"PAM_CHECKIN_OPERATION_FAILURE\",\"115\":\"FEDSP_SAMLAUTH_ASR_FAILURE\",\"116\":\"FEDSP_SAMLAUTH_ASR_SUCCESS\",\"117\":\"FEDSP_SAMLAUTH_ISSUED\",\"118\":\"DB_REPLICATION_QUEUE_DELAY_PAST_THRESHOLD\",\"119\":\"USER_HDD_RECOVERY_SUCCESS\",\"120\":\"USER_HDD_RECOVERY_FAILURE\",\"121\":\"USER_MOBILE_DEVICE_REGISTRATION\"}"
                ),
            )?;

            // Painless script
            // Source: if (ctx?.winlog?.event_id == null) {\n  return;\n}\ndef t = params.get(ctx.winlog.event_id);\nif (t == null) {\n  return;\n}\nif (ctx?.winlog?.event_data == null ) {\n  Map map = new HashMap();\n  ctx.winlog.put(\"event_data\", map);\n}\nctx.winlog.event_data.put(\"Description\", t)
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx?.winlog?.event_id == null) {\n  return;\n}\ndef t = params.get(ctx.winlog.event_id);\nif (t == null) {\n  return;\n}\nif (ctx?.winlog?.event_data == null ) {\n  Map map = new HashMap();\n  ctx.winlog.put(\"event_data\", map);\n}\nctx.winlog.event_data.put(\"Description\", t)"#
                ),
                cached_params!(
                    "{\"1\":\"User failed to authenticate\",\"2\":\"User successfully authenticated\",\"3\":\"User lockout triggered\",\"4\":\"Database commits suspended, replication queue full\",\"5\":\"Database commits resuming\",\"6\":\"Connectivity to replica database lost\",\"7\":\"Connectivity to replica database restored\",\"8\":\"Failed to replicate database transaction\",\"9\":\"Failed to insert data into database replication queue\",\"10\":\"ed to run stored procedure on replica server\",\"11\":\"Subscriber orchestration failed to start\",\"12\":\"Subscriber orchestration completed with failures\",\"13\":\"Failed to update subscriber password\",\"14\":\"Failed to check-in managed group set\",\"15\":\"Failed to fully check-in managed group set, some memberships were not revoked\",\"16\":\"Managed group set successfully checked in\",\"17\":\"Managed group set successfully checked out\",\"18\":\"Failed to check out managed group set\",\"19\":\"Managed group set partially checked out, some memberships were not granted\",\"20\":\"Managed account password successfully checked out\",\"21\":\"Failed to check-out managed account password\",\"22\":\"Managed account password successfully checked in\",\"23\":\"Failed to check-in managed account password\",\"24\":\"Managed account password viewed\",\"25\":\"Failed to view managed account password\",\"26\":\"Historical managed account password viewed\",\"27\":\"Failed to view historical managed account password\",\"28\":\"Administrative profile enabled\",\"29\":\"User profile enabled\",\"30\":\"Administrative profile disabled\",\"31\":\"User profile disabled\",\"32\":\"Administrative profile unlocked\",\"33\":\"User profile unlocked\",\"34\":\"Privileged access session recording started\",\"35\":\"Privileged access session recording ended\",\"36\":\"Privileged access session termination requested by administrator\",\"37\":\"Nightly discovery process started\",\"38\":\"Nightly discovery process finished\",\"39\":\"API login succeeded\",\"40\":\"API login failure\",\"41\":\"Failed to check in system and account query based access\",\"42\":\"Succeeded in checking in system and account query based access\",\"43\":\"Failed to check out system and account query based access\",\"44\":\"Succeeded in checking out system and account query based access\",\"45\":\"Target deployment finished with a failure.\",\"46\":\"Successfully finished target deployment.\",\"47\":\"Successfully imported a single target.\",\"48\":\"Successfully finished target deployment.\",\"49\":\"Target deployment finished with a failure.\",\"50\":\"Workflow manager aborted event processing.\",\"51\":\"Workflow manager failed to process event.\",\"52\":\"Security question successfully added.\",\"53\":\"Failed to add security question.\",\"54\":\"Security question successfully updated.\",\"55\":\"Failed to update security question.\",\"56\":\"Security question successfully deleted.\",\"57\":\"Security question successfully added.\",\"58\":\"Failed to add security question.\",\"59\":\"Security question successfully updated.\",\"60\":\"Failed to update security question.\",\"61\":\"Security question successfully deleted.\",\"62\":\"Self-service password reset started.\",\"63\":\"Self-service password reset successful.\",\"64\":\"Self-service password reset failed.\",\"65\":\"Help-desk assisted password reset started.\",\"66\":\"Help-desk assisted password reset successful.\",\"67\":\"Help-desk assisted password reset failed.\",\"68\":\"Self-service account unlock started.\",\"69\":\"Self-service account unlock successful.\",\"70\":\"Self-service account unlock failed.\",\"71\":\"Help-desk assisted account unlock started.\",\"72\":\"Help-desk assisted account unlock successful.\",\"73\":\"Help-desk assisted password reset failed.\",\"74\":\"Database replication watermark hit.\",\"75\":\"User attempted to claim alias that is already claimed.\",\"76\":\"Admin attempted to assign alias that is already claimed.\",\"77\":\"Connector timed out while performing operation.\",\"78\":\"Error occured during file replication to remote nodes.\",\"79\":\"All passwords successfully synchronized.\",\"80\":\"One or more passwords failed to be synchronized.\",\"81\":\"Workflow request has been approved.\",\"82\":\"Workflow request has been rejected.\",\"83\":\"Workflow request has been canceled.\",\"84\":\"Workflow request has been revoked.\",\"85\":\"Workflow request has been processed.\",\"86\":\"Successfully registered Digital ID.\",\"87\":\"Failed to register Digital ID.\",\"88\":\"Successfully updated Digital ID.\",\"89\":\"Digital ID successfully downloaded.\",\"90\":\"User successfully identified\",\"91\":\"Failed to identify user.\",\"92\":\"User successfully logged in.\",\"93\":\"User failed to log in.\",\"94\":\"Federated authn request successfully parsed.\",\"95\":\"Federated authn request failed to be parsed.\",\"96\":\"Federated assertion successfully generated.\",\"97\":\"Federated assertion failed to be generated.\",\"98\":\"Failed to execute stored procedure.\",\"99\":\"Target creation failure: Could not establish credentials.\",\"100\":\"Target creation successful: Credentials set successfully.\",\"101\":\"New federated SSO session created.\",\"102\":\"Federated SSO session terminated.\",\"103\":\"Generic access check-out successful.\",\"104\":\"Generic access check-out partially successful.\",\"105\":\"Generic access check-out failed.\",\"106\":\"Generic access check-in successful.\",\"107\":\"Generic access check-in partially successful.\",\"108\":\"Generic access check-in failed.\",\"109\":\"Generic access check-out expired.\",\"110\":\"Generic access check-out cannot be performed because it would exceed the check-out limit of one of its targets.\",\"111\":\"An operation run as part of a generic access check-out succeeded.\",\"112\":\"An operation run as part of a generic access check-out failed.\",\"113\":\"An operation run as part of a generic access check-in succeeded.\",\"114\":\"An operation run as part of a generic access check-in failed.\",\"115\":\"Failed to validate a SAML assertion.\",\"116\":\"Successfully validated a SAML assertion.\",\"117\":\"Issued SAML AuthNRequest.\",\"118\":\"Database replication queue delay exceeded configured threshold.\",\"119\":\"Self-service encrypted drive recovery successful.\",\"120\":\"Self-service encrypted drive recovery failure.\",\"121\":\"Self-service mobile device registration.\"}"
                ),
            )?;

            if event.has_value("winlog.record_id") {
                if let Some(val) = event.get("winlog.record_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "winlog.record_id".into(),
                            message,
                        }
                    })?;
                    event.set("winlog.record_id", converted)?;
                }
            }

            if event.has_value("winlog.event_id") {
                if let Some(val) = event.get("winlog.event_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "winlog.event_id".into(),
                            message,
                        }
                    })?;
                    event.set("winlog.event_id", converted)?;
                }
            }

            if event.has_value("winlog.event_data.DelayThreshold") {
                if let Some(val) = event.get("winlog.event_data.DelayThreshold") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "winlog.event_data.DelayThreshold".into(),
                            message,
                        }
                    })?;
                    event.set("winlog.event_data.DelayThreshold", converted)?;
                }
            }

            if event.has_value("winlog.event_data.QueueDelay") {
                if let Some(val) = event.get("winlog.event_data.QueueDelay") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "winlog.event_data.QueueDelay".into(),
                            message,
                        }
                    })?;
                    event.set("winlog.event_data.QueueDelay", converted)?;
                }
            }

            if event.has_value("winlog.event_data.QueueSize") {
                if let Some(val) = event.get("winlog.event_data.QueueSize") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "winlog.event_data.QueueSize".into(),
                            message,
                        }
                    })?;
                    event.set("winlog.event_data.QueueSize", converted)?;
                }
            }

            if event.has_value("winlog.event_data.Runtime") {
                if let Some(val) = event.get("winlog.event_data.Runtime") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "winlog.event_data.Runtime".into(),
                            message,
                        }
                    })?;
                    event.set("winlog.event_data.Runtime", converted)?;
                }
            }

            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { event.get_str("winlog.level") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.level")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("log.level", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("winlog.time_created") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("winlog.time_created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "winlog.time_created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            event.remove("winlog.event_data.Value1");
            event.remove("winlog.event_data.Value2");
            event.remove("winlog.event_data.Value3");
            event.remove("winlog.event_data.Value4");
            event.remove("winlog.event_data.Value5");
            event.remove("winlog.event_data.Value6");
            event.remove("winlog.event_data.Value7");
            event.remove("winlog.event_data.Value8");
            event.remove("winlog.event_data.Value9");

            if event.has_value("error.code") {
                if let Some(val) = event.get("error.code") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "error.code".into(),
                            message,
                        }
                    })?;
                    event.set("error.code", converted)?;
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
