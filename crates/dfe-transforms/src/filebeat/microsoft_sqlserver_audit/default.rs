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

            let _cond = { event.get_str("winlog.event_id") == Some("33205") };
            if _cond {
                gsub_field(
                    event,
                    "winlog.event_data.param1",
                    "winlog.event_data.param1",
                    cached_regex!("(?m)^\\.$"),
                    "",
                )?;
            }

            if let Some(input) = event.get_string("winlog.event_data.param1") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find("statement:") else {
                        break 'dissect false;
                    };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("statement:") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find("\nadditional_information:") else {
                        break 'dissect false;
                    };
                    captured.push(("_temp.stmt", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\nadditional_information:") else {
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
                        path: "winlog.event_data.param1".into(),
                        message: "dissect pattern did not match".into(),
                    });
                }
            }

            gsub_field(
                event,
                "winlog.event_data.param1",
                "winlog.event_data.param1",
                cached_regex!("statement:(.*\\s)*(?=additional_information:)"),
                "",
            )?;

            if let Some(kv_str) = event.get_string("winlog.event_data.param1") {
                for pair in cached_regex!("\\n").split(&kv_str).into_iter() {
                    if pair.trim().is_empty() {
                        continue;
                    }
                    let Some((key, value)) = pair.split_once(":") else {
                        return Err(TransformError::ParseError {
                            path: "winlog.event_data.param1".into(),
                            message: format!("does not contain value_split: {pair}"),
                        });
                    };
                    {
                        let key = key.trim_matches(|c| "\\n".contains(c));
                        let value = value.trim_matches(|c| "\\n".contains(c));
                        if !key.is_empty() {
                            kv_put(event, &format!("sqlserver.audit.{}", key), value)?;
                        }
                    }
                }
            }

            if let Some(v) = event.get("_temp.stmt").cloned() {
                event.set("sqlserver.audit.statement", v)?;
            }

            let _cond = { event.get_str("winlog.log.level") != Some("") };
            if _cond {
                if let Some(v) = event
                    .get("winlog.log.level")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("log.level", v)?;
                }
            }

            if let Some(date_str) = event.get_as_string("sqlserver.audit.event_time") {
                match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSSSSSS"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "sqlserver.audit.event_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "")?;
            }

            if event.has_value("host.mac") {
                gsub_field(
                    event,
                    "host.mac",
                    "host.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("database"))?;

            if event.has_value("sqlserver.audit.action_id") {
                map_strings(
                    event,
                    "sqlserver.audit.action_id",
                    "sqlserver.audit.action_id",
                    |s| s.trim().to_string(),
                )?;
            }

            if event.has_value("sqlserver.audit.class_type") {
                map_strings(
                    event,
                    "sqlserver.audit.class_type",
                    "sqlserver.audit.class_type",
                    |s| s.trim().to_string(),
                )?;
            }

            if event.has_value("sqlserver.audit.action_id") {
                map_strings(
                    event,
                    "sqlserver.audit.action_id",
                    "sqlserver.audit.action_id",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("sqlserver.audit.class_type") {
                map_strings(
                    event,
                    "sqlserver.audit.class_type",
                    "sqlserver.audit.class_type",
                    str::to_uppercase,
                )?;
            }

            // Painless script
            // Source: def actionIdKey = ctx.sqlserver.audit.action_id;\ndef actions = params.get('actions');\ndef classTypes = params.get('classtypes');\n// handle class type\n// overwrite the abbreviated key with its value\ndef ct = classTypes.get(ctx.sqlserver.audit.class_type);\nif (ct != null) {\n  ctx.sqlserver.audit.class_type = ct;\n}\n// error case - for unhandled action ids\ndef actionData = actions.get(actionIdKey);\nif (actionData == null) {\n  ctx.event.action = 'unknown-' + actionIdKey.toLowerCase();\n  ctx.event.type = ['info'];\n  return;\n}\n// overwrite the action id with its actual value\nctx.sqlserver.audit.action_id = actionData.get('value');\n// event.type\ndef actionType = actionData.get('type');\nif (actionType != null) {\n  ctx.event.type = new ArrayList(actionType);\n}\n// event.category\ndef actionCategory = actionData.get('category');\nif (actionCategory != null) {\n  for (def c : actionCategory) {\n    ctx.event.category.add(c);\n  }\n}\n// event.action\ndef action = actionData.get('action');\nif (action != null) {\n  ctx.event.action = action;\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def actionIdKey = ctx.sqlserver.audit.action_id;\ndef actions = params.get('actions');\ndef classTypes = params.get('classtypes');\n// handle class type\n// overwrite the abbreviated key with its value\ndef ct = classTypes.get(ctx.sqlserver.audit.class_type);\nif (ct != null) {\n  ctx.sqlserver.audit.class_type = ct;\n}\n// error case - for unhandled action ids\ndef actionData = actions.get(actionIdKey);\nif (actionData == null) {\n  ctx.event.action = 'unknown-' + actionIdKey.toLowerCase();\n  ctx.event.type = ['info'];\n  return;\n}\n// overwrite the action id with its actual value\nctx.sqlserver.audit.action_id = actionData.get('value');\n// event.type\ndef actionType = actionData.get('type');\nif (actionType != null) {\n  ctx.event.type = new ArrayList(actionType);\n}\n// event.category\ndef actionCategory = actionData.get('category');\nif (actionCategory != null) {\n  for (def c : actionCategory) {\n    ctx.event.category.add(c);\n  }\n}\n// event.action\ndef action = actionData.get('action');\nif (action != null) {\n  ctx.event.action = action;\n}"#
                ),
                cached_params!(
                    "{\"classtypes\":{\"DB\":\"DATABASE\",\"OB\":\"OBJECT\",\"TY\":\"TYPE\",\"SC\":\"SCHEMA\",\"SX\":\"XML SCHEMA COLLECTION\",\"AS\":\"ASSEMBLY\",\"US\":\"USER\",\"RL\":\"ROLE\",\"AR\":\"APPLICATION ROLE\",\"MT\":\"MESSAGE TYPE\",\"CT\":\"CONTRACT\",\"SV\":\"SERVICE\",\"BN\":\"REMOTE SERVICE BINDING\",\"RT\":\"ROUTE\",\"FC\":\"FULLTEXT CATALOG\",\"FL\":\"FULLTEXT STOPLIST\",\"FP\":\"SEARCH PROPERTY LIST\",\"SK\":\"SYMMETRIC KEY\",\"CR\":\"CERTIFICATE\",\"AK\":\"ASYMMETRIC KEY\",\"DC\":\"DATABASE SCOPED CREDENTIAL\",\"EL\":\"EXTERNAL LIBRARY\",\"LA\":\"EXTERNAL LANGUAGE\",\"SR\":\"SERVER\",\"EP\":\"ENDPOINT\",\"SG\":\"SERVER ROLE\",\"AG\":\"AVAILABILITY GROUP\",\"LX\":\"LOGIN\",\"CK\":\"COLUMN ENCRYPTION KEY\",\"CM\":\"COLUMN MASTER KEY\",\"DA\":\"DATABASE AUDIT SPECIFICATION\",\"DU\":\"AUDIT\",\"DS\":\"DATABASE SCOPED CONFIGURATION\",\"DR\":\"DATABASE SCOPED RESOURCE GOVERNOR\",\"DN\":\"EVENT NOTIFICATION DATABASE\",\"DT\":\"TRIGGER DATABASE\",\"MK\":\"MASTER KEY\",\"DK\":\"DATABASE ENCRYPTION KEY\",\"ON\":\"EVENT NOTIFICATION OBJECT\",\"PF\":\"PARTITION FUNCTION\",\"PR\":\"BROKER PRIORITY\",\"PS\":\"PARTITION SCHEME\",\"DE\":\"DATABASE EVENT SESSION\",\"AQ\":\"ADHOC QUERY\",\"AF\":\"AGGREGATE\",\"AP\":\"Undocumented\",\"C\":\"CHECK CONSTRAINT\",\"D\":\"DEFAULT\",\"EC\":\"EDGE CONSTRAINT\",\"EN\":\"EVENT NOTIFICATION\",\"F\":\"FOREIGN KEY CONSTRAINT\",\"FS\":\"FUNCTION SCALAR ASSEMBLY\",\"FT\":\"FUNCTION TABLE-VALUED ASSEMBLY\",\"FN\":\"FUNCTION SCALAR SQL\",\"IX\":\"INDEX\",\"IF\":\"FUNCTION TABLE-VALUED INLINE SQL\",\"IS\":\"FUNCTION SCALAR INLINE SQL\",\"IT\":\"INTERNAL TABLE\",\"PQ\":\"PREPARED ADHOC QUERY\",\"PK\":\"PRIMARY KEY\",\"P\":\"STORED PROCEDURE\",\"PC\":\"STORED PROCEDURE ASSEMBLY\",\"RF\":\"STORED PROCEDURE REPLICATION FILTER\",\"R\":\"RULE\",\"SP\":\"SECURITY POLICY\",\"SO\":\"SEQUENCE OBJECT\",\"ST\":\"STATISTICS\",\"SQ\":\"QUEUE\",\"SN\":\"SYNONYM\",\"S\":\"TABLE SYSTEM\",\"TF\":\"FUNCTION TABLE-VALUED SQL\",\"TA\":\"TRIGGER ASSEMBLY\",\"TR\":\"TRIGGER\",\"UQ\":\"UNIQUE CONSTRAINT\",\"U\":\"TABLE\",\"V\":\"VIEW\",\"X\":\"STORED PROCEDURE EXTENDED\",\"XR\":\"XREL TREE\",\"AU\":\"ASYMMETRIC KEY USER\",\"CU\":\"CERTIFICATE USER\",\"GU\":\"GROUP USER\",\"SU\":\"SQL USER\",\"WU\":\"WINDOWS USER\",\"XU\":\"EXTERNAL USER\",\"PU\":\"EXTERNAL GROUP USER\",\"A\":\"SERVER AUDIT\",\"CD\":\"CREDENTIAL\",\"CP\":\"CRYPTOGRAPHIC PROVIDER\",\"ED\":\"EXTERNAL DATA SOURCE\",\"EF\":\"EXTERNAL FILE FORMAT\",\"RG\":\"RESOURCE GOVERNOR\",\"SA\":\"SERVER AUDIT SPECIFICATION\",\"SD\":\"EVENT NOTIFICATION SERVER\",\"T\":\"TRIGGER SERVER\",\"SE\":\"EVENT SESSION\",\"CO\":\"SERVER CONFIG\",\"AL\":\"ASYMMETRIC KEY LOGIN\",\"CL\":\"CERTIFICATE LOGIN\",\"SL\":\"SQL LOGIN\",\"WG\":\"WINDOWS GROUP\",\"WL\":\"WINDOWS LOGIN\",\"ER\":\"EXTERNAL RESOURCE POOL\",\"EX\":\"EXTERNAL SCRIPT QUERY\",\"PL\":\"EXTERNAL GROUP LOGIN\",\"XL\":\"EXTERNAL LOGIN\"},\"actions\":{\"ACDO\":{\"value\":\"DATABASE_OBJECT_ACCESS_GROUP\",\"type\":[\"access\"],\"action\":\"database-object-accessed\"},\"ACO\":{\"value\":\"SCHEMA_OBJECT_ACCESS_GROUP\",\"type\":[\"access\"],\"action\":\"schema-object-permission-checked\"},\"ADBO\":{\"value\":\"BULK ADMIN\",\"type\":[\"change\"],\"action\":\"bulk-admin-operation\"},\"ADDP\":{\"value\":\"DATABASE_ROLE_MEMBER_CHANGE_GROUP\",\"type\":[\"admin\",\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"login-changed-from-database-role\"},\"ADFR\":{\"value\":\"ADD FEATURE RESTRICTION\",\"type\":[\"info\"],\"action\":\"add-feature-restriction\"},\"ADSC\":{\"value\":\"ADD SENSITIVITY CLASSIFICATION\",\"type\":[\"change\"],\"action\":\"add-sensitivity-classification-to-db-columns\"},\"ADSP\":{\"value\":\"SERVER_ROLE_MEMBER_CHANGE_GROUP\",\"type\":[\"admin\",\"change\",\"user\"],\"category\":[\"iam\"],\"action\":\"login-changed-from-server-role\"},\"AL\":{\"value\":\"ALTER\",\"type\":[\"change\"],\"action\":\"alter-object\"},\"ALCN\":{\"value\":\"ALTER CONNECTION\",\"type\":[\"change\",\"connection\"],\"category\":[\"network\"],\"action\":\"alter-connection\"},\"ALRS\":{\"value\":\"ALTER RESOURCES\",\"type\":[\"change\"],\"action\":\"alter-resources\"},\"ALSS\":{\"value\":\"ALTER SERVER STATE\",\"type\":[\"change\"],\"action\":\"alter-server-state\"},\"ALST\":{\"value\":\"ALTER SETTINGS\",\"type\":[\"change\"],\"category\":[\"configuration\"],\"action\":\"alter-settings\"},\"ALTR\":{\"value\":\"ALTER TRACE\",\"type\":[\"change\"],\"action\":\"alter-trace\"},\"APRL\":{\"value\":\"ADD MEMBER\",\"type\":[\"change\"],\"action\":\"add-member\"},\"AS\":{\"value\":\"ACCESS\",\"type\":[\"access\"],\"action\":\"access-object\"},\"AUSC\":{\"value\":\"AUDIT SESSION CHANGED\",\"type\":[\"change\"],\"action\":\"audit-session-changed\"},\"AUSF\":{\"value\":\"AUDIT SHUTDOWN ON FAILURE\",\"type\":[\"error\"],\"action\":\"audit-write-failed-database-shutdown\"},\"AUTH\":{\"value\":\"AUTHENTICATE\",\"type\":[\"info\"],\"action\":\"authenticate\"},\"BA\":{\"value\":\"BACKUP\",\"type\":[\"info\"],\"action\":\"database-backup-executed\"},\"BAL\":{\"value\":\"BACKUP LOG\",\"type\":[\"info\"],\"action\":\"transaction-log-backup-executed\"},\"BCM\":{\"value\":\"BATCH COMPLETED\",\"type\":[\"info\"],\"action\":\"transact-sql-batch-completed\"},\"BCMG\":{\"value\":\"BATCH_COMPLETED_GROUP\",\"type\":[\"info\"],\"action\":\"batch-text-stored-proc-or-txn-mgmt-op-ended\"},\"BRDB\":{\"value\":\"BACKUP_RESTORE_GROUP\",\"type\":[\"admin\"],\"action\":\"backup-or-restore-command-issued\"},\"BST\":{\"value\":\"BATCH STARTED\",\"type\":[\"info\"],\"action\":\"transact-sql-batch-started\"},\"BSTG\":{\"value\":\"BATCH_STARTED_GROUP\",\"type\":[\"info\"],\"action\":\"batch-text-stored-proc-txn-mgmt-op-started\"},\"C2OF\":{\"value\":\"TRACE AUDIT C2OFF\",\"type\":[\"change\"],\"action\":\"c2-audit-mode-server-config-off\"},\"C2ON\":{\"value\":\"TRACE AUDIT C2ON\",\"type\":[\"info\"],\"action\":\"c2-audit-mode-server-config-on\"},\"CCLG\":{\"value\":\"CHANGE LOGIN CREDENTIAL\",\"type\":[\"change\"],\"action\":\"change-login-credential\"},\"CMLG\":{\"value\":\"CREDENTIAL MAP TO LOGIN\",\"type\":[\"change\"],\"action\":\"credential-mapped-to-sql-server-login\"},\"CNAU\":{\"value\":\"AUDIT_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"audit-or-audit-spec-changed\"},\"CO\":{\"value\":\"CONNECT\",\"type\":[\"info\"],\"action\":\"connect\"},\"CP\":{\"value\":\"CHECKPOINT\",\"type\":[\"info\"],\"action\":\"checkpoint-created\"},\"CR\":{\"value\":\"CREATE\",\"type\":[\"info\"],\"action\":\"create\"},\"DABO\":{\"value\":\"DATABASE BULK ADMIN\",\"type\":[\"change\"],\"action\":\"database-bulk-admin\"},\"DAGF\":{\"value\":\"FAILED_DATABASE_AUTHENTICATION_GROUP\",\"type\":[\"error\"],\"action\":\"principal-login-failed\"},\"DAGL\":{\"value\":\"DATABASE_LOGOUT_GROUP\",\"type\":[\"info\",\"end\"],\"category\":[\"session\"],\"action\":\"contained-database-user-logout\"},\"DAGS\":{\"value\":\"SUCCESSFUL_DATABASE_AUTHENTICATION_GROUP\",\"type\":[\"info\",\"start\"],\"category\":[\"session\"],\"action\":\"principal-login-to-contained-database-successful\"},\"DBAF\":{\"value\":\"DATABASE AUTHENTICATION FAILED\",\"type\":[\"error\"],\"action\":\"database-authentication-failed\"},\"DBAS\":{\"value\":\"DATABASE AUTHENTICATION SUCCEEDED\",\"type\":[\"access\",\"info\"],\"action\":\"database-authentication-succeeded\"},\"DBCC\":{\"value\":\"DBCC\",\"type\":[\"change\"],\"category\":[\"configuration\"],\"action\":\"principal-issued-dbcc-command\"},\"DBCG\":{\"value\":\"DBCC_GROUP\",\"type\":[\"change\"],\"category\":[\"configuration\"],\"action\":\"principal-issued-dbcc-command\"},\"DBL\":{\"value\":\"DATABASE LOGOUT\",\"type\":[\"end\"],\"category\":[\"session\"],\"action\":\"database-logout\"},\"D\":{\"value\":\"DENY\",\"type\":[\"info\"],\"action\":\"permission-denied-to-principal\"},\"DL\":{\"value\":\"DELETE\",\"type\":[\"change\"],\"action\":\"delete\"},\"DPRL\":{\"value\":\"DROP MEMBER\",\"type\":[\"info\"],\"action\":\"drop-security-account-from-role\"},\"DR\":{\"value\":\"DROP\",\"type\":[\"change\"],\"action\":\"drop-object\"},\"DRFR\":{\"value\":\"DROP FEATURE RESTRICTION\",\"type\":[\"change\"],\"action\":\"drop-feature-restriction\"},\"DRSC\":{\"value\":\"DROP SENSITIVITY CLASSIFICATION\",\"type\":[\"change\"],\"action\":\"drop-sensitivity-classification-from-db-columns\"},\"DWC\":{\"value\":\"DENY WITH CASCADE\",\"type\":[\"change\"],\"action\":\"permission-denied-with-cascade\"},\"EX\":{\"value\":\"EXECUTE\",\"type\":[\"info\"],\"action\":\"execute-stored-proc-or-function\"},\"FRCG\":{\"value\":\"FEATURE_RESTRICTION_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"feature-restriction-changed\"},\"FT\":{\"value\":\"FULLTEXT\",\"type\":[\"info\"],\"action\":\"fulltext-event-occurred\"},\"FTG\":{\"value\":\"FULLTEXT_GROUP\",\"type\":[\"info\"],\"action\":\"fulltext-event-occurred\"},\"G\":{\"value\":\"GRANT\",\"type\":[\"info\"],\"action\":\"grant-permission-to-principal\"},\"GRDB\":{\"value\":\"DATABASE_PERMISSION_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"grant-revoke-or-deny-permission\"},\"GRDO\":{\"value\":\"DATABASE_OBJECT_PERMISSION_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"grant-revoke-or-deny-permission-on-schema-or-assemblies\"},\"GRO\":{\"value\":\"SCHEMA_OBJECT_PERMISSION_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"grant-revoke-or-deny-permission-on-schema-objects\"},\"GRSO\":{\"value\":\"SERVER_OBJECT_PERMISSION_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"grant-revoke-or-deny-permission-on-server-objects\"},\"GRSV\":{\"value\":\"SERVER_PERMISSION_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"grant-revoke-or-deny-permission-issued-in-server-scope\"},\"GWG\":{\"value\":\"GRANT WITH GRANT\",\"type\":[\"info\"],\"action\":\"grant-with-grant-issued-to-principal\"},\"IMDP\":{\"value\":\"DATABASE_PRINCIPAL_IMPERSONATION_GROUP\",\"type\":[\"info\"],\"action\":\"database-user-impersonation-occurred\"},\"IMP\":{\"value\":\"IMPERSONATE\",\"type\":[\"info\"],\"action\":\"database-user-impersonation-occurred\"},\"IMSP\":{\"value\":\"SERVER_PRINCIPAL_IMPERSONATION_GROUP\",\"type\":[\"user\"],\"action\":\"server-login-impersonation-occurred\"},\"IN\":{\"value\":\"INSERT\",\"type\":[\"info\"],\"action\":\"insert\"},\"LGB\":{\"value\":\"BROKER LOGIN\",\"type\":[\"info\"],\"action\":\"service-broker-transport-security-event\"},\"LGBG\":{\"value\":\"BROKER_LOGIN_GROUP\",\"type\":[\"info\"],\"action\":\"service-broker-transport-security-event\"},\"LGDA\":{\"value\":\"DISABLE\",\"type\":[\"change\"],\"action\":\"disable\"},\"LGDB\":{\"value\":\"CHANGE DEFAULT DATABASE\",\"type\":[\"change\"],\"action\":\"change-default-database\"},\"LGEA\":{\"value\":\"ENABLE\",\"type\":[\"info\"],\"action\":\"enable\"},\"LGFL\":{\"value\":\"FAILED_LOGIN_GROUP\",\"type\":[\"error\"],\"category\":[\"authentication\"],\"action\":\"principal-login-failed\"},\"LGGG\":{\"value\":\"GLOBAL_TRANSACTIONS_LOGIN_GROUP\",\"type\":[\"info\"],\"action\":\"global-transactions-login\"},\"LGG\":{\"value\":\"GLOBAL TRANSACTIONS LOGIN\",\"type\":[\"info\"],\"action\":\"global-transactions-login\"},\"LGIF\":{\"value\":\"LOGIN FAILED\",\"type\":[\"error\"],\"category\":[\"authentication\"],\"action\":\"login-failed\"},\"LGIS\":{\"value\":\"LOGIN SUCCEEDED\",\"type\":[\"info\",\"start\"],\"category\":[\"session\"],\"action\":\"login-succeeded\"},\"LGLG\":{\"value\":\"CHANGE DEFAULT LANGUAGE\",\"type\":[\"change\"],\"action\":\"change-default-language\"},\"LGM\":{\"value\":\"DATABASE MIRRORING LOGIN\",\"type\":[\"info\"],\"action\":\"database-mirroring-transport-security-event\"},\"LGMG\":{\"value\":\"DATABASE_MIRRORING_LOGIN_GROUP\",\"type\":[\"info\"],\"action\":\"database-mirroring-transport-security-event\"},\"LGNM\":{\"value\":\"NAME CHANGE\",\"type\":[\"change\"],\"action\":\"name-change\"},\"LGO\":{\"value\":\"LOGOUT\",\"type\":[\"end\"],\"category\":[\"session\"],\"action\":\"logout\"},\"LGSD\":{\"value\":\"SUCCESSFUL_LOGIN_GROUP\",\"type\":[\"info\",\"start\"],\"category\":[\"session\"],\"action\":\"user-login-succeeded\"},\"LGSG\":{\"value\":\"STORAGE_LOGIN_GROUP\",\"type\":[\"info\"],\"action\":\"storage-login\"},\"LGS\":{\"value\":\"STORAGE LOGIN\",\"type\":[\"info\"],\"action\":\"storage-login\"},\"LO\":{\"value\":\"LOGOUT_GROUP\",\"type\":[\"info\",\"end\"],\"category\":[\"session\"],\"action\":\"user-logout-succeeded\"},\"MNDB\":{\"value\":\"DATABASE_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"database-created-altered-or-dropped\"},\"MNDO\":{\"value\":\"DATABASE_OBJECT_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"database-object-created-altered-or-dropped\"},\"MNDP\":{\"value\":\"DATABASE_PRINCIPAL_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"principals-created-altered-or-dropped\"},\"MNO\":{\"value\":\"SCHEMA_OBJECT_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"schema-object-create-alter-or-dropped\"},\"MNSO\":{\"value\":\"SERVER_OBJECT_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"server-object-create-alter-or-dropped\"},\"MNSP\":{\"value\":\"SERVER_PRINCIPAL_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"server-principal-create-alter-or-dropped\"},\"NMLG\":{\"value\":\"NO CREDENTIAL MAP TO LOGIN\",\"type\":[\"error\"],\"action\":\"no-credential-map-to-login\"},\"OPDB\":{\"value\":\"DATABASE_OPERATION_GROUP\",\"type\":[\"info\"],\"action\":\"db-checkpoint-or-subscribe-query-notification-executed\"},\"OP\":{\"value\":\"OPEN\",\"type\":[\"access\"],\"action\":\"open\"},\"OPSV\":{\"value\":\"SERVER_OPERATION_GROUP\",\"type\":[\"change\"],\"action\":\"alter-settings-resources-or-external-access\"},\"PWAR\":{\"value\":\"APPLICATION_ROLE_CHANGE_PASSWORD_GROUP\",\"type\":[\"change\"],\"action\":\"password-changed-for-application-role\"},\"PWC\":{\"value\":\"CHANGE PASSWORD\",\"type\":[\"change\"],\"action\":\"password-changed-for-application-role\"},\"PWCG\":{\"value\":\"LOGIN_CHANGE_PASSWORD_GROUP\",\"type\":[\"change\"],\"action\":\"login-password-changed-via-alter-or-sp-password\"},\"PWCS\":{\"value\":\"CHANGE OWN PASSWORD\",\"type\":[\"change\"],\"action\":\"change-own-password\"},\"PWEX\":{\"value\":\"PASSWORD EXPIRATION\",\"type\":[\"info\"],\"action\":\"password-expired\"},\"PWMC\":{\"value\":\"MUST CHANGE PASSWORD\",\"type\":[\"info\"],\"action\":\"must-change-password\"},\"PWPL\":{\"value\":\"PASSWORD POLICY\",\"type\":[\"info\"],\"action\":\"password-policy\"},\"PWR\":{\"value\":\"RESET PASSWORD\",\"type\":[\"change\"],\"action\":\"reset-password\"},\"PWRS\":{\"value\":\"RESET OWN PASSWORD\",\"type\":[\"change\"],\"action\":\"reset-own-password\"},\"PWU\":{\"value\":\"UNLOCK ACCOUNT\",\"type\":[\"change\"],\"action\":\"unlock-sql-server-login-account\"},\"RCM\":{\"value\":\"RPC COMPLETED\",\"type\":[\"end\"],\"category\":[\"network\"],\"action\":\"rpc-completed\"},\"RC\":{\"value\":\"RECEIVE\",\"type\":[\"access\"],\"action\":\"retrieve-message-from-queue\"},\"RF\":{\"value\":\"REFERENCES\",\"type\":[\"info\"],\"action\":\"references\"},\"R\":{\"value\":\"REVOKE\",\"type\":[\"change\"],\"action\":\"remove-granted-or-denied-permission\"},\"RS\":{\"value\":\"RESTORE\",\"type\":[\"change\"],\"action\":\"restore-database-backup\"},\"RST\":{\"value\":\"RPC STARTED\",\"type\":[\"start\"],\"category\":[\"network\"],\"action\":\"rpc-started\"},\"RWC\":{\"value\":\"REVOKE WITH CASCADE\",\"type\":[\"change\"],\"action\":\"revoke-granted-or-denied-permission-with-cascade\"},\"RWG\":{\"value\":\"REVOKE WITH GRANT\",\"type\":[\"change\"],\"action\":\"revoke-with-grant\"},\"SCCG\":{\"value\":\"SENSITIVITY_CLASSIFICATION_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"sensitivity-classification-changed\"},\"SL\":{\"value\":\"SELECT\",\"type\":[\"access\"],\"action\":\"select\"},\"SN\":{\"value\":\"SEND\",\"type\":[\"access\"],\"action\":\"send-message-to-queue\"},\"SPLN\":{\"value\":\"SHOW PLAN\",\"type\":[\"info\"],\"action\":\"show-plan\"},\"STSV\":{\"value\":\"SERVER_STATE_CHANGE_GROUP\",\"type\":[\"info\"],\"action\":\"server-service-state-changed\"},\"SUQN\":{\"value\":\"SUBSCRIBE QUERY NOTIFICATION\",\"type\":[\"info\"],\"action\":\"subscribe-query-notification\"},\"SVCN\":{\"value\":\"SERVER CONTINUE\",\"type\":[\"change\"],\"action\":\"server-service-state-changed-to-continue\"},\"SVPD\":{\"value\":\"SERVER PAUSED\",\"type\":[\"change\"],\"action\":\"server-service-state-changed-to-paused\"},\"SVSD\":{\"value\":\"SERVER SHUTDOWN\",\"type\":[\"change\"],\"action\":\"server-service-state-changed-to-shutdown\"},\"SVSR\":{\"value\":\"SERVER STARTED\",\"type\":[\"change\"],\"action\":\"server-service-state-changed-to-start\"},\"TASA\":{\"value\":\"TRACE AUDIT START\",\"type\":[\"info\"],\"action\":\"trace-audit-start\"},\"TASP\":{\"value\":\"TRACE AUDIT STOP\",\"type\":[\"info\"],\"action\":\"trace-audit-stop\"},\"TODB\":{\"value\":\"DATABASE_OWNERSHIP_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"permission-check-performed-to-change-database-owner\"},\"TODO\":{\"value\":\"DATABASE_OBJECT_OWNERSHIP_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"database-object-owner-changed\"},\"TOO\":{\"value\":\"SCHEMA_OBJECT_OWNERSHIP_CHANGE_GROUP\",\"type\":[\"info\"],\"action\":\"permission-check-performed-to-change-schema-object\"},\"TOSO\":{\"value\":\"SERVER_OBJECT_OWNERSHIP_CHANGE_GROUP\",\"type\":[\"change\"],\"action\":\"server-scoped-object-owner-changed\"},\"TO\":{\"value\":\"TAKE OWNERSHIP\",\"type\":[\"info\"],\"action\":\"take-ownership\"},\"TRBC\":{\"value\":\"TRANSACTION BEGIN COMPLETED\",\"type\":[\"info\"],\"action\":\"transaction-begin-completed\"},\"TRBS\":{\"value\":\"TRANSACTION BEGIN STARTING\",\"type\":[\"info\"],\"action\":\"transaction-begin-starting\"},\"TRCC\":{\"value\":\"TRANSACTION COMMIT COMPLETED\",\"type\":[\"info\"],\"action\":\"transaction-commit-completed\"},\"TRCG\":{\"value\":\"TRACE_CHANGE_GROUP\",\"type\":[\"info\"],\"action\":\"permission-checked-for-alter-trace\"},\"TRCS\":{\"value\":\"TRANSACTION COMMIT STARTING\",\"type\":[\"info\"],\"action\":\"transaction-commit-starting\"},\"TRGC\":{\"value\":\"TRANSACTION PROPAGATE COMPLETED\",\"type\":[\"info\"],\"action\":\"transaction-propogation-completed\"},\"TRGS\":{\"value\":\"TRANSACTION PROPAGATE STARTING\",\"type\":[\"info\"],\"action\":\"transaction-propogation-starting\"},\"TRO\":{\"value\":\"TRANSFER\",\"type\":[\"info\"],\"action\":\"data-transfer\"},\"TRPC\":{\"value\":\"TRANSACTION PROMOTE COMPLETED\",\"type\":[\"info\"],\"action\":\"local-to-distributed-transaction-promote-completed\"},\"TRPS\":{\"value\":\"TRANSACTION PROMOTE STARTING\",\"type\":[\"info\"],\"action\":\"local-to-distributed-transaction-promote-starting\"},\"TRRC\":{\"value\":\"TRANSACTION ROLLBACK COMPLETED\",\"type\":[\"info\"],\"action\":\"transaction-rollback-completed\"},\"TRRS\":{\"value\":\"TRANSACTION ROLLBACK STARTING\",\"type\":[\"info\"],\"action\":\"transaction-rollback-starting\"},\"TRSC\":{\"value\":\"TRANSACTION SAVEPOINT COMPLETED\",\"type\":[\"info\"],\"action\":\"transaction-savepoint-completed\"},\"TRSS\":{\"value\":\"TRANSACTION SAVEPOINT STARTING\",\"type\":[\"info\"],\"action\":\"transaction-savepoint-starting\"},\"TXBG\":{\"value\":\"TRANSACTION BEGIN\",\"type\":[\"info\"],\"action\":\"transaction-begin\"},\"TXCG\":{\"value\":\"TRANSACTION_COMMIT_GROUP\",\"type\":[\"info\"],\"action\":\"transaction-commit-group-event\"},\"TXCM\":{\"value\":\"TRANSACTION COMMIT\",\"type\":[\"info\"],\"action\":\"transaction-commit\"},\"TXGG\":{\"value\":\"TRANSACTION_BEGIN_GROUP\",\"type\":[\"info\"],\"action\":\"transaction-begin-group-event\"},\"TXRB\":{\"value\":\"TRANSACTION ROLLBACK\",\"type\":[\"info\"],\"action\":\"transaction-rollback\"},\"TXRG\":{\"value\":\"TRANSACTION_ROLLBACK_GROUP\",\"type\":[\"info\"],\"action\":\"transaction-rollback-group\"},\"TX\":{\"value\":\"TRANSACTION_GROUP\",\"type\":[\"info\"],\"action\":\"transaction-event-occurred\"},\"UCGP\":{\"value\":\"USER_CHANGE_PASSWORD_GROUP\",\"type\":[\"change\"],\"action\":\"password-of-contained-database-user-changed\"},\"UDAG\":{\"value\":\"USER_DEFINED_AUDIT_GROUP\",\"type\":[\"info\"],\"action\":\"user-defined-audit-event-sp-audit-write\"},\"UDAU\":{\"value\":\"USER DEFINED AUDIT\",\"type\":[\"info\"],\"action\":\"user-defined-audit-event-sp-audit-write\"},\"UNDG\":{\"value\":\"STATEMENT_ROLLBACK_GROUP\",\"type\":[\"info\"],\"action\":\"statement-rollback-group\"},\"UNDO\":{\"value\":\"STATEMENT ROLLBACK\",\"type\":[\"info\"],\"action\":\"statement-rollback\"},\"UP\":{\"value\":\"UPDATE\",\"type\":[\"change\"],\"action\":\"update\"},\"USAF\":{\"value\":\"CHANGE USERS LOGIN AUTO\",\"type\":[\"change\"],\"action\":\"change-users-login-auto\"},\"USLG\":{\"value\":\"CHANGE USERS LOGIN\",\"type\":[\"change\"],\"action\":\"change-users-login\"},\"USTC\":{\"value\":\"COPY PASSWORD\",\"type\":[\"info\"],\"action\":\"password-copied\"},\"VDST\":{\"value\":\"VIEW DATABASE STATE\",\"type\":[\"info\"],\"action\":\"view-database-state\"},\"VSST\":{\"value\":\"VIEW SERVER STATE\",\"type\":[\"info\"],\"action\":\"view-server-state\"},\"VWCT\":{\"value\":\"VIEW CHANGETRACKING\",\"type\":[\"info\"],\"action\":\"view-change-tracking\"},\"VW\":{\"value\":\"VIEW\",\"type\":[\"info\"],\"action\":\"view\"},\"XA\":{\"value\":\"EXTERNAL ACCESS ASSEMBLY\",\"type\":[\"access\"],\"category\":[\"network\",\"registry\"],\"action\":\"external-access-assembly\"},\"XU\":{\"value\":\"UNSAFE ASSEMBLY\",\"type\":[\"access\"],\"action\":\"unsafe-assembly\"}}}"
                ),
            )?;

            if event.has_value("sqlserver.audit.sequence_number") {
                if let Some(val) = event.get("sqlserver.audit.sequence_number") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "sqlserver.audit.sequence_number".into(),
                            message,
                        }
                    })?;
                    event.set("sqlserver.audit.sequence_number", converted)?;
                }
            }

            if event.has_value("sqlserver.audit.succeeded") {
                if let Some(val) = event.get("sqlserver.audit.succeeded") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "sqlserver.audit.succeeded".into(),
                            message,
                        }
                    })?;
                    event.set("sqlserver.audit.succeeded", converted)?;
                }
            }

            if event.has_value("sqlserver.audit.affected_rows") {
                if let Some(val) = event.get("sqlserver.audit.affected_rows") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "sqlserver.audit.affected_rows".into(),
                            message,
                        }
                    })?;
                    event.set("sqlserver.audit.affected_rows", converted)?;
                }
            }

            if event.has_value("sqlserver.audit.response_rows") {
                if let Some(val) = event.get("sqlserver.audit.response_rows") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "sqlserver.audit.response_rows".into(),
                            message,
                        }
                    })?;
                    event.set("sqlserver.audit.response_rows", converted)?;
                }
            }

            if event.has_value("sqlserver.audit.is_column_permission") {
                if let Some(val) = event.get("sqlserver.audit.is_column_permission") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "sqlserver.audit.is_column_permission".into(),
                            message,
                        }
                    })?;
                    event.set("sqlserver.audit.is_column_permission", converted)?;
                }
            }

            // Painless script
            // Source: def v = ctx?.sqlserver?.audit?.duration_milliseconds;\nif (v != null) {\n    ctx.event.duration = Long.parseLong(v) * 1000000;\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def v = ctx?.sqlserver?.audit?.duration_milliseconds;\nif (v != null) {\n    ctx.event.duration = Long.parseLong(v) * 1000000;\n}"#
                ),
            )?;

            if event.has_value("winlog.process") {
                event.rename("winlog.process", "process")?;
            }

            let _cond = {
                event.has_value("sqlserver.audit.server_principal_name")
                    && event
                        .get("sqlserver.audit.server_principal_name")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        })
            };
            if _cond {
                if event.has_value("sqlserver.audit.server_principal_name") {
                    if let Some(input) = event.get_string("sqlserver.audit.server_principal_name") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("\\") else {
                                break 'dissect false;
                            };
                            captured.push(("_temp.domain", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("_temp.username", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "sqlserver.audit.server_principal_name".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("sqlserver.audit.server_principal_name")
                    && !(event
                        .get("sqlserver.audit.server_principal_name")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        }))
            };
            if _cond {
                if let Some(v) = event.get("sqlserver.audit.server_principal_name").cloned() {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { event.has_value("_temp.username") };
            if _cond {
                if let Some(v) = event.get("_temp.username").cloned() {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { event.has_value("_temp.domain") };
            if _cond {
                if let Some(v) = event.get("_temp.domain").cloned() {
                    event.set("user.domain", v)?;
                }
            }

            if let Some(v) = event.get("sqlserver.audit.server_principal_sid").cloned() {
                event.set("user.id", v)?;
            }

            if let Some(v) = event
                .get("sqlserver.audit.target_server_principal_name")
                .cloned()
            {
                event.set("user.target.name", v)?;
            }

            if let Some(v) = event
                .get("sqlserver.audit.target_server_principal_sid")
                .cloned()
            {
                event.set("user.target.id", v)?;
            }

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

            event.remove("_temp");
            event.remove("winlog.event_data.param1");
            event.remove("sqlserver.audit.event_time");
            event.remove("sqlserver.audit.additional_information");
            event.remove("sqlserver.audit.duration_milliseconds");
            event.remove("sqlserver.audit.server_principal_name");
            event.remove("sqlserver.audit.server_principal_sid");
            event.remove("sqlserver.audit.target_server_principal_name");
            event.remove("sqlserver.audit.target_server_principal_sid");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
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
                event.set(
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
