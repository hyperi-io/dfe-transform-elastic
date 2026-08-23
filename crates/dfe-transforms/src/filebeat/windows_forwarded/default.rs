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
            let _cond = {
                event.get("winlog.channel").is_some_and(|v| v.is_string())
                    && event
                        .get_str("winlog.channel")
                        .is_some_and(|s| s.to_lowercase() == "security")
                    && [
                        "Microsoft-Windows-Eventlog",
                        "Microsoft-Windows-Security-Auditing",
                    ]
                    .contains(&event.get_str("winlog.provider_name").unwrap_or(""))
            };
            if _cond {
                // Begin nested pipeline: "security_default"
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
                let _cond = { event.has_value("winlog.event_data") };
                if _cond {
                    // Painless script
                    // Source: ctx.winlog?.event_data?.entrySet().removeIf(entry -> [null, \"\", \"-\", \"{00000000-0000-0000-0000-000000000000}\"].contains(entry.getValue()))
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.winlog?.event_data?.entrySet().removeIf(entry -> [null, \"\", \"-\", \"{00000000-0000-0000-0000-000000000000}\"].contains(entry.getValue()))"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("winlog.provider_name")
                        && [
                            "Microsoft-Windows-Eventlog",
                            "Microsoft-Windows-Security-Auditing",
                        ]
                        .contains(&event.get_str("winlog.provider_name").unwrap_or(""))
                };
                if _cond {
                    // Begin nested pipeline: "security_standard"
                    // Painless script
                    // Source: if (ctx.event?.code == null || params.get(ctx.event.code) == null) {\n  return;\n}\nparams.get(ctx.event.code).forEach((k, v) -> {\n  if (v instanceof List) {\n    ctx.event[k] = new ArrayList(v);\n  } else {\n    ctx.event[k] = v;\n  }\n});
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null || params.get(ctx.event.code) == null) {\n  return;\n}\nparams.get(ctx.event.code).forEach((k, v) -> {\n  if (v instanceof List) {\n    ctx.event[k] = new ArrayList(v);\n  } else {\n    ctx.event[k] = v;\n  }\n});"#
                        ),
                        cached_params!(
                            "{\"1100\":{\"action\":\"logging-service-shutdown\",\"category\":[\"process\"],\"type\":[\"end\"]},\"1102\":{\"action\":\"audit-log-cleared\",\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"]},\"1104\":{\"action\":\"logging-full\",\"category\":[\"iam\"],\"type\":[\"admin\"]},\"1105\":{\"action\":\"auditlog-archieved\",\"category\":[\"iam\"],\"type\":[\"admin\"]},\"1108\":{\"action\":\"logging-processing-error\",\"category\":[\"iam\"],\"type\":[\"admin\"]},\"4610\":{\"action\":\"authentication-package-loaded\",\"category\":[\"configuration\"],\"type\":[\"access\"]},\"4611\":{\"action\":\"trusted-logon-process-registered\",\"category\":[\"configuration\"],\"type\":[\"change\"]},\"4614\":{\"action\":\"notification-package-loaded\",\"category\":[\"configuration\"],\"type\":[\"access\"]},\"4616\":{\"action\":\"system-time-changed\",\"category\":[\"configuration\"],\"type\":[\"change\"]},\"4622\":{\"action\":\"security-package-loaded\",\"category\":[\"configuration\"],\"type\":[\"access\"]},\"4624\":{\"action\":\"logged-in\",\"category\":[\"authentication\"],\"type\":[\"start\"]},\"4625\":{\"action\":\"logon-failed\",\"category\":[\"authentication\"],\"type\":[\"start\"]},\"4634\":{\"action\":\"logged-out\",\"category\":[\"authentication\"],\"type\":[\"end\"]},\"4647\":{\"action\":\"logged-out\",\"category\":[\"authentication\"],\"type\":[\"end\"]},\"4648\":{\"action\":\"logged-in-explicit\",\"category\":[\"authentication\"],\"type\":[\"start\"]},\"4657\":{\"action\":\"registry-value-modified\",\"category\":[\"registry\",\"configuration\"],\"type\":[\"change\"]},\"4662\":{\"action\":\"object-operation-performed\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"4670\":{\"action\":\"permissions-changed\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"4672\":{\"action\":\"logged-in-special\",\"category\":[\"iam\"],\"type\":[\"admin\"]},\"4673\":{\"action\":\"privileged-service-called\",\"category\":[\"iam\"],\"type\":[\"admin\"]},\"4674\":{\"action\":\"privileged-operation\",\"category\":[\"iam\"],\"type\":[\"admin\"]},\"4688\":{\"action\":\"created-process\",\"category\":[\"process\"],\"type\":[\"start\"]},\"4689\":{\"action\":\"exited-process\",\"category\":[\"process\"],\"type\":[\"end\"]},\"4697\":{\"action\":\"service-installed\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"4698\":{\"action\":\"scheduled-task-created\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\",\"admin\"]},\"4699\":{\"action\":\"scheduled-task-deleted\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"deletion\",\"admin\"]},\"4700\":{\"action\":\"scheduled-task-enabled\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"admin\"]},\"4701\":{\"action\":\"scheduled-task-disabled\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"admin\"]},\"4702\":{\"action\":\"scheduled-task-updated\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"admin\"]},\"4706\":{\"action\":\"domain-trust-added\",\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"4707\":{\"action\":\"domain-trust-removed\",\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"4713\":{\"action\":\"kerberos-policy-changed\",\"category\":[\"configuration\"],\"type\":[\"change\"]},\"4714\":{\"action\":\"encrypted-data-recovery-policy-changed\",\"category\":[\"configuration\"],\"type\":[\"change\"]},\"4715\":{\"action\":\"object-audit-policy-changed\",\"category\":[\"configuration\"],\"type\":[\"change\"]},\"4716\":{\"action\":\"trusted-domain-information-changed\",\"category\":[\"configuration\"],\"type\":[\"change\"]},\"4717\":{\"action\":\"system-security-access-granted\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"4718\":{\"action\":\"system-security-access-removed\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"deletion\"]},\"4719\":{\"action\":\"changed-audit-config\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"4720\":{\"action\":\"added-user-account\",\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"4722\":{\"action\":\"enabled-user-account\",\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"4723\":{\"action\":\"changed-password\",\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"4724\":{\"action\":\"reset-password\",\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"4725\":{\"action\":\"disabled-user-account\",\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]},\"4726\":{\"action\":\"deleted-user-account\",\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]},\"4727\":{\"action\":\"added-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"4728\":{\"action\":\"added-member-to-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4729\":{\"action\":\"removed-member-from-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4730\":{\"action\":\"deleted-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"4731\":{\"action\":\"added-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"4732\":{\"action\":\"added-member-to-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4733\":{\"action\":\"removed-member-from-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4734\":{\"action\":\"deleted-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"4735\":{\"action\":\"modified-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4737\":{\"action\":\"modified-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4738\":{\"action\":\"modified-user-account\",\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"4739\":{\"action\":\"domain-policy-changed\",\"category\":[\"configuration\"],\"type\":[\"change\"]},\"4740\":{\"action\":\"locked-out-user-account\",\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"4741\":{\"action\":\"added-computer-account\",\"category\":[\"iam\"],\"type\":[\"creation\",\"admin\"]},\"4742\":{\"action\":\"changed-computer-account\",\"category\":[\"iam\"],\"type\":[\"change\",\"admin\"]},\"4743\":{\"action\":\"deleted-computer-account\",\"category\":[\"iam\"],\"type\":[\"deletion\",\"admin\"]},\"4744\":{\"action\":\"added-distribution-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"4745\":{\"action\":\"changed-distribution-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4746\":{\"action\":\"added-member-to-distribution-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4747\":{\"action\":\"removed-member-from-distribution-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4748\":{\"action\":\"deleted-distribution-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"4749\":{\"action\":\"added-distribution-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"4750\":{\"action\":\"changed-distribution-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4751\":{\"action\":\"added-member-to-distribution-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4752\":{\"action\":\"removed-member-from-distribution-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4753\":{\"action\":\"deleted-distribution-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"4754\":{\"action\":\"added-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"4755\":{\"action\":\"modified-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4756\":{\"action\":\"added-member-to-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4757\":{\"action\":\"removed-member-from-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4758\":{\"action\":\"deleted-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"4759\":{\"action\":\"added-distribution-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"4760\":{\"action\":\"changed-distribution-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4761\":{\"action\":\"added-member-to-distribution-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4762\":{\"action\":\"removed-member-from-distribution-group\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4763\":{\"action\":\"deleted-distribution-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"4764\":{\"action\":\"type-changed-group-account\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"4767\":{\"action\":\"unlocked-user-account\",\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"4768\":{\"action\":\"kerberos-authentication-ticket-requested\",\"category\":[\"authentication\"],\"type\":[\"start\"]},\"4769\":{\"action\":\"kerberos-service-ticket-requested\",\"category\":[\"authentication\"],\"type\":[\"start\"]},\"4770\":{\"action\":\"kerberos-service-ticket-renewed\",\"category\":[\"authentication\"],\"type\":[\"start\"]},\"4771\":{\"action\":\"kerberos-preauth-failed\",\"category\":[\"authentication\"],\"type\":[\"start\"]},\"4776\":{\"action\":\"credential-validated\",\"category\":[\"authentication\"],\"type\":[\"start\"]},\"4778\":{\"action\":\"session-reconnected\",\"category\":[\"authentication\",\"session\"],\"type\":[\"start\"]},\"4779\":{\"action\":\"session-disconnected\",\"category\":[\"authentication\",\"session\"],\"type\":[\"end\"]},\"4781\":{\"action\":\"renamed-user-account\",\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"4797\":{\"action\":\"query-existence-of-blank-password\",\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"4798\":{\"action\":\"group-membership-enumerated\",\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"4799\":{\"action\":\"user-member-enumerated\",\"category\":[\"iam\"],\"type\":[\"group\",\"info\"]},\"4817\":{\"action\":\"object-audit-changed\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"4902\":{\"action\":\"user-audit-policy-created\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"creation\"]},\"4904\":{\"action\":\"security-event-source-added\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"4905\":{\"action\":\"security-event-source-removed\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"deletion\"]},\"4906\":{\"action\":\"crash-on-audit-changed\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"4907\":{\"action\":\"audit-setting-changed\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"4908\":{\"action\":\"special-group-table-changed\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"4912\":{\"action\":\"per-user-audit-policy-changed\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"4950\":{\"action\":\"windows-firewall-setting-changed\",\"category\":[\"configuration\"],\"type\":[\"change\"]},\"4954\":{\"action\":\"windows-firewall-group-policy-changed\",\"category\":[\"configuration\"],\"type\":[\"change\"]},\"4964\":{\"action\":\"logged-in-special\",\"category\":[\"iam\"],\"type\":[\"admin\",\"group\"]},\"5024\":{\"action\":\"windows-firewall-service-started\",\"category\":[\"process\"],\"type\":[\"start\"]},\"5025\":{\"action\":\"windows-firewall-service-stopped\",\"category\":[\"process\"],\"type\":[\"end\"]},\"5033\":{\"action\":\"windows-firewall-driver-started\",\"category\":[\"driver\"],\"type\":[\"start\"]},\"5034\":{\"action\":\"windows-firewall-driver-stopped\",\"category\":[\"driver\"],\"type\":[\"end\"]},\"5037\":{\"action\":\"windows-firewall-driver-error\",\"category\":[\"driver\"],\"type\":[\"end\"]},\"5136\":{\"action\":\"directory-service-object-modified\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"change\"]},\"5137\":{\"action\":\"directory-service-object-created\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"creation\"]},\"5140\":{\"action\":\"network-share-object-accessed\",\"category\":[\"network\",\"file\"],\"type\":[\"info\",\"access\"]},\"5141\":{\"action\":\"directory-service-object-deleted\",\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"deletion\"]},\"5145\":{\"action\":\"network-share-object-access-checked\",\"category\":[\"network\",\"file\"],\"type\":[\"info\",\"access\"]},\"5152\":{\"action\":\"windows-firewall-packet-drop\",\"category\":[\"network\"],\"type\":[\"connection\",\"info\",\"denied\"]},\"5156\":{\"action\":\"windows-firewall-connection\",\"category\":[\"network\"],\"type\":[\"connection\",\"info\",\"allowed\"]},\"5157\":{\"action\":\"windows-firewall-packet-block\",\"category\":[\"network\"],\"type\":[\"connection\",\"info\",\"denied\"]},\"5158\":{\"action\":\"windows-firewall-bind-local-port\",\"category\":[\"network\"],\"type\":[\"info\"]},\"5379\":{\"action\":\"credential-manager-credentials-were-read\",\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"5380\":{\"action\":\"vault-credential-find\",\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"5381\":{\"action\":\"vault-credentials-were-read\",\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"5382\":{\"action\":\"vault-credentials-were-read\",\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]}}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.winlog?.event_data?.LogonType == null) {\n  return;\n}\ndef t = params.get(ctx.winlog.event_data.LogonType);\nif (t == null) {\n  return;\n}\nif (ctx.winlog?.logon == null ) {\n  Map map = new HashMap();\n  ctx.winlog.put(\"logon\", map);\n}\nctx.winlog.logon.put(\"type\", t)
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data?.LogonType == null) {\n  return;\n}\ndef t = params.get(ctx.winlog.event_data.LogonType);\nif (t == null) {\n  return;\n}\nif (ctx.winlog?.logon == null ) {\n  Map map = new HashMap();\n  ctx.winlog.put(\"logon\", map);\n}\nctx.winlog.logon.put(\"type\", t)"#
                        ),
                        cached_params!(
                            "{\"10\":\"RemoteInteractive\",\"11\":\"CachedInteractive\",\"2\":\"Interactive\",\"3\":\"Network\",\"4\":\"Batch\",\"5\":\"Service\",\"7\":\"Unlock\",\"8\":\"NetworkCleartext\",\"9\":\"NewCredentials\"}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.winlog?.event_data == null) {\n  return;\n}\nLong newUacValue;\ntry {\n  newUacValue = Long.decode(ctx.winlog.event_data.NewUacValue.trim());\n} catch (Exception e) {\n  return;\n}\nArrayList uacResult = new ArrayList();\nfor (entry in params.entrySet()) {\n  Long flag = Long.decode(entry.getKey());\n  if ((newUacValue.longValue() & flag.longValue()) == flag.longValue()) {\n    uacResult.add(entry.getValue());\n  }\n}\nif (uacResult.length == 0) {\n  return;\n}\nctx.winlog.event_data.put(\"NewUACList\", uacResult);\nif (ctx.winlog.event_data.UserAccountControl == null || ctx.winlog.event_data.UserAccountControl == \"-\") {\n  return;\n}\nArrayList uac_array = new ArrayList();\nfor (elem in ctx.winlog.event_data.UserAccountControl.splitOnToken((String)((char)0x0a))) {\n  def trimmed = elem.replace(\"%%\",\"\").trim();\n  if (trimmed.length() > 0) {\n    uac_array.add(trimmed);\n  }\n}\nctx.winlog.event_data.UserAccountControl = uac_array;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data == null) {\n  return;\n}\nLong newUacValue;\ntry {\n  newUacValue = Long.decode(ctx.winlog.event_data.NewUacValue.trim());\n} catch (Exception e) {\n  return;\n}\nArrayList uacResult = new ArrayList();\nfor (entry in params.entrySet()) {\n  Long flag = Long.decode(entry.getKey());\n  if ((newUacValue.longValue() & flag.longValue()) == flag.longValue()) {\n    uacResult.add(entry.getValue());\n  }\n}\nif (uacResult.length == 0) {\n  return;\n}\nctx.winlog.event_data.put(\"NewUACList\", uacResult);\nif (ctx.winlog.event_data.UserAccountControl == null || ctx.winlog.event_data.UserAccountControl == \"-\") {\n  return;\n}\nArrayList uac_array = new ArrayList();\nfor (elem in ctx.winlog.event_data.UserAccountControl.splitOnToken((String)((char)0x0a))) {\n  def trimmed = elem.replace(\"%%\",\"\").trim();\n  if (trimmed.length() > 0) {\n    uac_array.add(trimmed);\n  }\n}\nctx.winlog.event_data.UserAccountControl = uac_array;"#
                        ),
                        cached_params!(
                            "{\"0x00000001\":\"USER_ACCOUNT_DISABLED\",\"0x00000002\":\"USER_HOME_DIRECTORY_REQUIRED\",\"0x00000004\":\"USER_PASSWORD_NOT_REQUIRED\",\"0x00000008\":\"USER_TEMP_DUPLICATE_ACCOUNT\",\"0x00000010\":\"USER_NORMAL_ACCOUNT\",\"0x00000020\":\"USER_MNS_LOGON_ACCOUNT\",\"0x00000040\":\"USER_INTERDOMAIN_TRUST_ACCOUNT\",\"0x00000080\":\"USER_WORKSTATION_TRUST_ACCOUNT\",\"0x00000100\":\"USER_SERVER_TRUST_ACCOUNT\",\"0x00000200\":\"USER_DONT_EXPIRE_PASSWORD\",\"0x00000400\":\"USER_ACCOUNT_AUTO_LOCKED\",\"0x00000800\":\"USER_ENCRYPTED_TEXT_PASSWORD_ALLOWED\",\"0x00001000\":\"USER_SMARTCARD_REQUIRED\",\"0x00002000\":\"USER_TRUSTED_FOR_DELEGATION\",\"0x00004000\":\"USER_NOT_DELEGATED\",\"0x00008000\":\"USER_USE_DES_KEY_ONLY\",\"0x00010000\":\"USER_DONT_REQUIRE_PREAUTH\",\"0x00020000\":\"USER_PASSWORD_EXPIRED\",\"0x00040000\":\"USER_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION\",\"0x00080000\":\"USER_NO_AUTH_DATA_REQUIRED\",\"0x00100000\":\"USER_PARTIAL_SECRETS_ACCOUNT\",\"0x00200000\":\"USER_USE_AES_KEYS\"}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.winlog?.event_data?.TicketOptions == null) {\n  return;\n}\nLong tOpts = Long.decode(ctx.winlog.event_data.TicketOptions);\nArrayList tDescs = new ArrayList();\nfor (entry in params.entrySet()) {\n  Long flag = Long.decode(entry.getKey());\n  if ((tOpts.longValue() & flag.longValue()) == flag.longValue()) {\n    tDescs.add(entry.getValue());\n  }\n}\nif (tDescs.length == 0) {\n  return;\n}\nctx.winlog.event_data.put(\"TicketOptionsDescription\", tDescs);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data?.TicketOptions == null) {\n  return;\n}\nLong tOpts = Long.decode(ctx.winlog.event_data.TicketOptions);\nArrayList tDescs = new ArrayList();\nfor (entry in params.entrySet()) {\n  Long flag = Long.decode(entry.getKey());\n  if ((tOpts.longValue() & flag.longValue()) == flag.longValue()) {\n    tDescs.add(entry.getValue());\n  }\n}\nif (tDescs.length == 0) {\n  return;\n}\nctx.winlog.event_data.put(\"TicketOptionsDescription\", tDescs);"#
                        ),
                        cached_params!(
                            "{\"0x00000001\":\"Validate\",\"0x00000002\":\"Renew\",\"0x00000008\":\"Enc-tkt-in-skey\",\"0x00000010\":\"Renewable-ok\",\"0x00000020\":\"Disable-transited-check\",\"0x00010000\":\"Name-canonicalize\",\"0x00020000\":\"Request-anonymous\",\"0x00040000\":\"Ok-as-delegate\",\"0x00080000\":\"Transited-policy-checked\",\"0x00100000\":\"Opt-hardware-auth\",\"0x00200000\":\"Pre-authent\",\"0x00400000\":\"Initial\",\"0x00800000\":\"Renewable\",\"0x01000000\":\"Invalid\",\"0x02000000\":\"Postdated\",\"0x04000000\":\"Allow-postdate\",\"0x08000000\":\"Proxy\",\"0x10000000\":\"Proxiable\",\"0x20000000\":\"Forwarded\",\"0x40000000\":\"Forwardable\"}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.winlog?.event_data?.TicketEncryptionType == null) {\n  return;\n}\nctx.winlog.event_data.put(\"TicketEncryptionTypeDescription\",\n                          params[ctx.winlog.event_data.TicketEncryptionType.toLowerCase()])
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data?.TicketEncryptionType == null) {\n  return;\n}\nctx.winlog.event_data.put(\"TicketEncryptionTypeDescription\",\n                          params[ctx.winlog.event_data.TicketEncryptionType.toLowerCase()])"#
                        ),
                        cached_params!(
                            "{\"0x1\":\"DES-CBC-CRC\",\"0x11\":\"AES128-CTS-HMAC-SHA1-96\",\"0x12\":\"AES256-CTS-HMAC-SHA1-96\",\"0x17\":\"RC4-HMAC\",\"0x18\":\"RC4-HMAC-EXP\",\"0x3\":\"DES-CBC-MD5\",\"0xffffffff\":\"FAIL\"}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.winlog?.event_data?.Status == null ||\n    ctx.event?.code == null ||\n    ![\"4768\", \"4769\", \"4770\", \"4771\"].contains(ctx.event.code)) {\n  return;\n}\nctx.winlog.event_data.put(\"StatusDescription\", params[ctx.winlog.event_data.Status]);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data?.Status == null ||\n    ctx.event?.code == null ||\n    ![\"4768\", \"4769\", \"4770\", \"4771\"].contains(ctx.event.code)) {\n  return;\n}\nctx.winlog.event_data.put(\"StatusDescription\", params[ctx.winlog.event_data.Status]);"#
                        ),
                        cached_params!(
                            "{\"0x0\":\"KDC_ERR_NONE\",\"0x1\":\"KDC_ERR_NAME_EXP\",\"0x10\":\"KDC_ERR_PADATA_TYPE_NOSUPP\",\"0x11\":\"KDC_ERR_TRTYPE_NO_SUPP\",\"0x12\":\"KDC_ERR_CLIENT_REVOKED\",\"0x13\":\"KDC_ERR_SERVICE_REVOKED\",\"0x14\":\"KDC_ERR_TGT_REVOKED\",\"0x15\":\"KDC_ERR_CLIENT_NOTYET\",\"0x16\":\"KDC_ERR_SERVICE_NOTYET\",\"0x17\":\"KDC_ERR_KEY_EXPIRED\",\"0x18\":\"KDC_ERR_PREAUTH_FAILED\",\"0x19\":\"KDC_ERR_PREAUTH_REQUIRED\",\"0x1A\":\"KDC_ERR_SERVER_NOMATCH\",\"0x1B\":\"KDC_ERR_MUST_USE_USER2USER\",\"0x1F\":\"KRB_AP_ERR_BAD_INTEGRITY\",\"0x2\":\"KDC_ERR_SERVICE_EXP\",\"0x20\":\"KRB_AP_ERR_TKT_EXPIRED\",\"0x21\":\"KRB_AP_ERR_TKT_NYV\",\"0x22\":\"KRB_AP_ERR_REPEAT\",\"0x23\":\"KRB_AP_ERR_NOT_US\",\"0x24\":\"KRB_AP_ERR_BADMATCH\",\"0x25\":\"KRB_AP_ERR_SKEW\",\"0x26\":\"KRB_AP_ERR_BADADDR\",\"0x27\":\"KRB_AP_ERR_BADVERSION\",\"0x28\":\"KRB_AP_ERR_MSG_TYPE\",\"0x29\":\"KRB_AP_ERR_MODIFIED\",\"0x2A\":\"KRB_AP_ERR_BADORDER\",\"0x2C\":\"KRB_AP_ERR_BADKEYVER\",\"0x2D\":\"KRB_AP_ERR_NOKEY\",\"0x2E\":\"KRB_AP_ERR_MUT_FAIL\",\"0x2F\":\"KRB_AP_ERR_BADDIRECTION\",\"0x3\":\"KDC_ERR_BAD_PVNO\",\"0x30\":\"KRB_AP_ERR_METHOD\",\"0x31\":\"KRB_AP_ERR_BADSEQ\",\"0x32\":\"KRB_AP_ERR_INAPP_CKSUM\",\"0x33\":\"KRB_AP_PATH_NOT_ACCEPTED\",\"0x34\":\"KRB_ERR_RESPONSE_TOO_BIG\",\"0x3C\":\"KRB_ERR_GENERIC\",\"0x3D\":\"KRB_ERR_FIELD_TOOLONG\",\"0x3E\":\"KDC_ERR_CLIENT_NOT_TRUSTED\",\"0x3F\":\"KDC_ERR_KDC_NOT_TRUSTED\",\"0x4\":\"KDC_ERR_C_OLD_MAST_KVNO\",\"0x40\":\"KDC_ERR_INVALID_SIG\",\"0x41\":\"KDC_ERR_KEY_TOO_WEAK\",\"0x42\":\"KRB_AP_ERR_USER_TO_USER_REQUIRED\",\"0x43\":\"KRB_AP_ERR_NO_TGT\",\"0x44\":\"KDC_ERR_WRONG_REALM\",\"0x5\":\"KDC_ERR_S_OLD_MAST_KVNO\",\"0x6\":\"KDC_ERR_C_PRINCIPAL_UNKNOWN\",\"0x7\":\"KDC_ERR_S_PRINCIPAL_UNKNOWN\",\"0x8\":\"KDC_ERR_PRINCIPAL_NOT_UNIQUE\",\"0x9\":\"KDC_ERR_NULL_KEY\",\"0xA\":\"KDC_ERR_CANNOT_POSTDATE\",\"0xB\":\"KDC_ERR_NEVER_VALID\",\"0xC\":\"KDC_ERR_POLICY\",\"0xD\":\"KDC_ERR_BADOPTION\",\"0xE\":\"KDC_ERR_ETYPE_NOTSUPP\",\"0xF\":\"KDC_ERR_SUMTYPE_NOSUPP\"}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.winlog?.event_data?.ServiceName != null) {\n  if (ctx.service == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"service\", hm);\n  }\n  ctx.service.put(\"name\", ctx.winlog.event_data.ServiceName);\n}\nif (ctx.winlog.event_data?.ServiceType != null) {\n  if (ctx.service == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"service\", hm);\n  }\n  ctx.service.put(\"type\", params[ctx.winlog.event_data.ServiceType]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data?.ServiceName != null) {\n  if (ctx.service == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"service\", hm);\n  }\n  ctx.service.put(\"name\", ctx.winlog.event_data.ServiceName);\n}\nif (ctx.winlog.event_data?.ServiceType != null) {\n  if (ctx.service == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"service\", hm);\n  }\n  ctx.service.put(\"type\", params[ctx.winlog.event_data.ServiceType]);\n}"#
                        ),
                        cached_params!(
                            "{\"0x1\":\"Kernel Driver\",\"0x10\":\"Win32 Own Process\",\"0x110\":\"Interactive Own Process\",\"0x120\":\"Interactive Share Process\",\"0x2\":\"File System Driver\",\"0x20\":\"Win32 Share Process\",\"0x8\":\"Recognizer Driver\"}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.winlog?.event_data?.SubcategoryGuid == null) {\n  return;\n}\ndef subCatGuid = ctx.winlog.event_data.SubcategoryGuid.replace(\"{\",\"\").replace(\"}\",\"\").toUpperCase();\nif (!params.containsKey(subCatGuid)) {\n  return;\n}\nctx.winlog.event_data.put(\"Category\", params[subCatGuid][1]);\nctx.winlog.event_data.put(\"SubCategory\", params[subCatGuid][0]);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data?.SubcategoryGuid == null) {\n  return;\n}\ndef subCatGuid = ctx.winlog.event_data.SubcategoryGuid.replace(\"{\",\"\").replace(\"}\",\"\").toUpperCase();\nif (!params.containsKey(subCatGuid)) {\n  return;\n}\nctx.winlog.event_data.put(\"Category\", params[subCatGuid][1]);\nctx.winlog.event_data.put(\"SubCategory\", params[subCatGuid][0]);"#
                        ),
                        cached_params!(
                            "{\"0CCE9210-69AE-11D9-BED3-505054503030\":[\"Security State Change\",\"System\"],\"0CCE9211-69AE-11D9-BED3-505054503030\":[\"Security System Extension\",\"System\"],\"0CCE9212-69AE-11D9-BED3-505054503030\":[\"System Integrity\",\"System\"],\"0CCE9213-69AE-11D9-BED3-505054503030\":[\"IPsec Driver\",\"System\"],\"0CCE9214-69AE-11D9-BED3-505054503030\":[\"Other System Events\",\"System\"],\"0CCE9215-69AE-11D9-BED3-505054503030\":[\"Logon\",\"Logon/Logoff\"],\"0CCE9216-69AE-11D9-BED3-505054503030\":[\"Logoff\",\"Logon/Logoff\"],\"0CCE9217-69AE-11D9-BED3-505054503030\":[\"Account Lockout\",\"Logon/Logoff\"],\"0CCE9218-69AE-11D9-BED3-505054503030\":[\"IPsec Main Mode\",\"Logon/Logoff\"],\"0CCE9219-69AE-11D9-BED3-505054503030\":[\"IPsec Quick Mode\",\"Logon/Logoff\"],\"0CCE921A-69AE-11D9-BED3-505054503030\":[\"IPsec Extended Mode\",\"Logon/Logoff\"],\"0CCE921B-69AE-11D9-BED3-505054503030\":[\"Special Logon\",\"Logon/Logoff\"],\"0CCE921C-69AE-11D9-BED3-505054503030\":[\"Other Logon/Logoff Events\",\"Logon/Logoff\"],\"0CCE921D-69AE-11D9-BED3-505054503030\":[\"File System\",\"Object Access\"],\"0CCE921E-69AE-11D9-BED3-505054503030\":[\"Registry\",\"Object Access\"],\"0CCE921F-69AE-11D9-BED3-505054503030\":[\"Kernel Object\",\"Object Access\"],\"0CCE9220-69AE-11D9-BED3-505054503030\":[\"SAM\",\"Object Access\"],\"0CCE9221-69AE-11D9-BED3-505054503030\":[\"Certification Services\",\"Object Access\"],\"0CCE9222-69AE-11D9-BED3-505054503030\":[\"Application Generated\",\"Object Access\"],\"0CCE9223-69AE-11D9-BED3-505054503030\":[\"Handle Manipulation\",\"Object Access\"],\"0CCE9224-69AE-11D9-BED3-505054503030\":[\"File Share\",\"Object Access\"],\"0CCE9225-69AE-11D9-BED3-505054503030\":[\"Filtering Platform Packet Drop\",\"Object Access\"],\"0CCE9226-69AE-11D9-BED3-505054503030\":[\"Filtering Platform Connection \",\"Object Access\"],\"0CCE9227-69AE-11D9-BED3-505054503030\":[\"Other Object Access Events\",\"Object Access\"],\"0CCE9228-69AE-11D9-BED3-505054503030\":[\"Sensitive Privilege Use\",\"Privilege Use\"],\"0CCE9229-69AE-11D9-BED3-505054503030\":[\"Non Sensitive Privilege Use\",\"Privilege Use\"],\"0CCE922A-69AE-11D9-BED3-505054503030\":[\"Other Privilege Use Events\",\"Privilege Use\"],\"0CCE922B-69AE-11D9-BED3-505054503030\":[\"Process Creation\",\"Detailed Tracking\"],\"0CCE922C-69AE-11D9-BED3-505054503030\":[\"Process Termination\",\"Detailed Tracking\"],\"0CCE922D-69AE-11D9-BED3-505054503030\":[\"DPAPI Activity\",\"Detailed Tracking\"],\"0CCE922E-69AE-11D9-BED3-505054503030\":[\"RPC Events\",\"Detailed Tracking\"],\"0CCE922F-69AE-11D9-BED3-505054503030\":[\"Audit Policy Change\",\"Policy Change\"],\"0CCE9230-69AE-11D9-BED3-505054503030\":[\"Authentication Policy Change\",\"Policy Change\"],\"0CCE9231-69AE-11D9-BED3-505054503030\":[\"Authorization Policy Change\",\"Policy Change\"],\"0CCE9232-69AE-11D9-BED3-505054503030\":[\"MPSSVC Rule-Level Policy Change\",\"Policy Change\"],\"0CCE9233-69AE-11D9-BED3-505054503030\":[\"Filtering Platform Policy Change\",\"Policy Change\"],\"0CCE9234-69AE-11D9-BED3-505054503030\":[\"Other Policy Change Events\",\"Policy Change\"],\"0CCE9235-69AE-11D9-BED3-505054503030\":[\"User Account Management\",\"Account Management\"],\"0CCE9236-69AE-11D9-BED3-505054503030\":[\"Computer Account Management\",\"Account Management\"],\"0CCE9237-69AE-11D9-BED3-505054503030\":[\"Security Group Management\",\"Account Management\"],\"0CCE9238-69AE-11D9-BED3-505054503030\":[\"Distribution Group Management\",\"Account Management\"],\"0CCE9239-69AE-11D9-BED3-505054503030\":[\"Application Group Management\",\"Account Management\"],\"0CCE923A-69AE-11D9-BED3-505054503030\":[\"Other Account Management Events\",\"Account Management\"],\"0CCE923B-69AE-11D9-BED3-505054503030\":[\"Directory Service Access\",\"Account Management\"],\"0CCE923C-69AE-11D9-BED3-505054503030\":[\"Directory Service Changes\",\"Account Management\"],\"0CCE923D-69AE-11D9-BED3-505054503030\":[\"Directory Service Replication\",\"Account Management\"],\"0CCE923E-69AE-11D9-BED3-505054503030\":[\"Detailed Directory Service Replication\",\"Account Management\"],\"0CCE923F-69AE-11D9-BED3-505054503030\":[\"Credential Validation\",\"Account Logon\"],\"0CCE9240-69AE-11D9-BED3-505054503030\":[\"Kerberos Service Ticket Operations\",\"Account Logon\"],\"0CCE9241-69AE-11D9-BED3-505054503030\":[\"Other Account Logon Events\",\"Account Logon\"],\"0CCE9242-69AE-11D9-BED3-505054503030\":[\"Kerberos Authentication Service\",\"Account Logon\"],\"0CCE9243-69AE-11D9-BED3-505054503030\":[\"Network Policy Server\",\"Logon/Logoff\"],\"0CCE9244-69AE-11D9-BED3-505054503030\":[\"Detailed File Share\",\"Object Access\"],\"0CCE9245-69AE-11D9-BED3-505054503030\":[\"Removable Storage\",\"Object Access\"],\"0CCE9246-69AE-11D9-BED3-505054503030\":[\"Central Policy Staging\",\"Object Access\"],\"0CCE9247-69AE-11D9-BED3-505054503030\":[\"User / Device Claims\",\"Logon/Logoff\"],\"0CCE9248-69AE-11D9-BED3-505054503030\":[\"Plug and Play Events\",\"Detailed Tracking\"],\"0CCE9249-69AE-11D9-BED3-505054503030\":[\"Group Membership\",\"Logon/Logoff\"]}"
                        ),
                    )?;
                    // Painless script
                    // Source: def split(String s) {\n  def f = new ArrayList();\n  int last = 0;\n  for (; last < s.length() && Character.isWhitespace(s.charAt(last)); last++) {}\n  for (def i = last; i < s.length(); i++) {\n    if (!Character.isWhitespace(s.charAt(i))) {\n      continue;\n    }\n    f.add(s.substring(last, i));\n    for (; i < s.length() && Character.isWhitespace(s.charAt(i)); i++) {}\n    last = i;\n  }\n  f.add(s.substring(last));\n  return f;\n}\nif (ctx.winlog?.event_data?.FailureReason != null) {\n  def code = ctx.winlog.event_data.FailureReason.replace(\"%%\",\"\");\n  def desc = params.descriptions[code];\n  if (desc == null) {\n    desc = code;\n  }\n  if (desc != null) {\n    if (ctx.winlog?.logon == null ) {\n      HashMap hm = new HashMap();\n      ctx.winlog.put(\"logon\", hm);\n    }\n    if (ctx.winlog?.logon?.failure == null) {\n      HashMap hm = new HashMap();\n      ctx.winlog.logon.put(\"failure\", hm);\n    }\n    ctx.winlog.logon.failure.put(\"reason\", desc);\n  }\n}\nif (ctx.winlog?.event_data?.AuditPolicyChanges != null) {\n  ArrayList policyChanges = null;\n  if (ctx.winlog.event_data.AuditPolicyChanges instanceof String) {\n    String[] tokens = ctx.winlog.event_data.AuditPolicyChanges.splitOnToken(\",\");\n    policyChanges = new ArrayList(Arrays.asList(tokens));\n  } else if (ctx.winlog.event_data.AuditPolicyChanges instanceof ArrayList) {\n    policyChanges = ctx.winlog.event_data.AuditPolicyChanges;\n  }\n  ArrayList results = new ArrayList();\n  for (elem in policyChanges) {\n    def code = elem.replace(\"%%\",\"\").trim();\n    if (params.descriptions.containsKey(code)) {\n      results.add(params.descriptions[code]);\n    } else {\n      results.add(code);\n    }\n  }\n  if (results.length > 0) {\n    ctx.winlog.event_data.put(\"AuditPolicyChangesDescription\", results);\n  }\n}\nif (ctx.winlog?.event_data?.AccessList != null) {\n  ArrayList codes = new ArrayList();\n  ArrayList results = new ArrayList();\n  ArrayList accessList = null;\n  if (ctx.winlog.event_data.AccessList instanceof String) {\n    accessList = split(ctx.winlog.event_data.AccessList);\n  } else if (ctx.winlog.event_data.AccessList instanceof ArrayList) {\n    accessList = ctx.winlog.event_data.AccessList;\n  }\n  for (elem in accessList) {\n    def code = elem.replace(\"%%\",\"\").trim();\n    if (code != \"\") {\n      codes.add(code);\n    }\n    if (params.descriptions.containsKey(code)) {\n      results.add(params.descriptions[code]);\n    } else {\n      results.add(code);\n    }\n  }\n  if (codes.length > 0) {\n    ctx.winlog.event_data.AccessList = codes;\n  }\n  if (results.length > 0) {\n    ctx.winlog.event_data.put(\"AccessListDescription\", results);\n  }\n}\nif (ctx.winlog?.event_data?.Direction != null) {\n  def code = ctx.winlog.event_data.Direction.replace(\"%%\",\"\").trim();\n  if (params.descriptions.containsKey(code)) {\n    ctx.winlog.event_data.put(\"DirectionDescription\", params.descriptions[code]);\n  }\n}\nif (ctx.winlog?.event_data?.LayerName != null) {\n  def code = ctx.winlog.event_data.LayerName.replace(\"%%\",\"\").trim();\n  if (params.descriptions.containsKey(code)) {\n    ctx.winlog.event_data.put(\"LayerNameDescription\", params.descriptions[code]);\n  }\n}\nif (ctx.winlog?.event_data?.AccessMask != null) {\n  ArrayList list = new ArrayList();\n  long lAccessMask;\n  ArrayList accessMask = null;\n  if (ctx.winlog.event_data.AccessMask instanceof String) {\n    accessMask = split(ctx.winlog.event_data.AccessMask);\n  } else if (ctx.winlog.event_data.AccessMask instanceof ArrayList) {\n    accessMask = ctx.winlog.event_data.AccessMask;\n  }\n  for (elem in accessMask) {\n    if (elem.length() == 0) {\n      continue;\n    }\n    def code = elem.replace(\"%%\",\"\").trim();\n    if (params.descriptions.containsKey(code)) {\n      list.add(params.descriptions[code]);\n    } else {\n      list.add(code);\n      if (params.reversed_descriptions.containsKey(code))\n        code = params.reversed_descriptions[code][0];\n    }\n    try {\n      def longCode = Long.decode(code).longValue();\n      lAccessMask |= longCode;\n    } catch (Exception e) {}\n  }\n  if (list.length > 0) {\n    ctx.winlog.event_data.put(\"AccessMask\", list);\n  }\n\n  ArrayList desc = new ArrayList();\n  def[] w = new def[] { null };\n  for (long b = 0; b < 32; b++) {\n    long flag = 1L << b;\n    if ((lAccessMask & flag) == flag) {\n      w[0] = flag;\n      def fDesc = params.AccessMaskDescriptions[String.format(\"0x%08X\", w)];\n      if (fDesc != null) {\n        desc.add(fDesc);\n      }\n    }\n  }\n  if (desc.length > 0) {\n    ctx.winlog.event_data.put(\"AccessMaskDescription\", desc);\n  }\n  ArrayList results = new ArrayList();\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def split(String s) {\n  def f = new ArrayList();\n  int last = 0;\n  for (; last < s.length() && Character.isWhitespace(s.charAt(last)); last++) {}\n  for (def i = last; i < s.length(); i++) {\n    if (!Character.isWhitespace(s.charAt(i))) {\n      continue;\n    }\n    f.add(s.substring(last, i));\n    for (; i < s.length() && Character.isWhitespace(s.charAt(i)); i++) {}\n    last = i;\n  }\n  f.add(s.substring(last));\n  return f;\n}\nif (ctx.winlog?.event_data?.FailureReason != null) {\n  def code = ctx.winlog.event_data.FailureReason.replace(\"%%\",\"\");\n  def desc = params.descriptions[code];\n  if (desc == null) {\n    desc = code;\n  }\n  if (desc != null) {\n    if (ctx.winlog?.logon == null ) {\n      HashMap hm = new HashMap();\n      ctx.winlog.put(\"logon\", hm);\n    }\n    if (ctx.winlog?.logon?.failure == null) {\n      HashMap hm = new HashMap();\n      ctx.winlog.logon.put(\"failure\", hm);\n    }\n    ctx.winlog.logon.failure.put(\"reason\", desc);\n  }\n}\nif (ctx.winlog?.event_data?.AuditPolicyChanges != null) {\n  ArrayList policyChanges = null;\n  if (ctx.winlog.event_data.AuditPolicyChanges instanceof String) {\n    String[] tokens = ctx.winlog.event_data.AuditPolicyChanges.splitOnToken(\",\");\n    policyChanges = new ArrayList(Arrays.asList(tokens));\n  } else if (ctx.winlog.event_data.AuditPolicyChanges instanceof ArrayList) {\n    policyChanges = ctx.winlog.event_data.AuditPolicyChanges;\n  }\n  ArrayList results = new ArrayList();\n  for (elem in policyChanges) {\n    def code = elem.replace(\"%%\",\"\").trim();\n    if (params.descriptions.containsKey(code)) {\n      results.add(params.descriptions[code]);\n    } else {\n      results.add(code);\n    }\n  }\n  if (results.length > 0) {\n    ctx.winlog.event_data.put(\"AuditPolicyChangesDescription\", results);\n  }\n}\nif (ctx.winlog?.event_data?.AccessList != null) {\n  ArrayList codes = new ArrayList();\n  ArrayList results = new ArrayList();\n  ArrayList accessList = null;\n  if (ctx.winlog.event_data.AccessList instanceof String) {\n    accessList = split(ctx.winlog.event_data.AccessList);\n  } else if (ctx.winlog.event_data.AccessList instanceof ArrayList) {\n    accessList = ctx.winlog.event_data.AccessList;\n  }\n  for (elem in accessList) {\n    def code = elem.replace(\"%%\",\"\").trim();\n    if (code != \"\") {\n      codes.add(code);\n    }\n    if (params.descriptions.containsKey(code)) {\n      results.add(params.descriptions[code]);\n    } else {\n      results.add(code);\n    }\n  }\n  if (codes.length > 0) {\n    ctx.winlog.event_data.AccessList = codes;\n  }\n  if (results.length > 0) {\n    ctx.winlog.event_data.put(\"AccessListDescription\", results);\n  }\n}\nif (ctx.winlog?.event_data?.Direction != null) {\n  def code = ctx.winlog.event_data.Direction.replace(\"%%\",\"\").trim();\n  if (params.descriptions.containsKey(code)) {\n    ctx.winlog.event_data.put(\"DirectionDescription\", params.descriptions[code]);\n  }\n}\nif (ctx.winlog?.event_data?.LayerName != null) {\n  def code = ctx.winlog.event_data.LayerName.replace(\"%%\",\"\").trim();\n  if (params.descriptions.containsKey(code)) {\n    ctx.winlog.event_data.put(\"LayerNameDescription\", params.descriptions[code]);\n  }\n}\nif (ctx.winlog?.event_data?.AccessMask != null) {\n  ArrayList list = new ArrayList();\n  long lAccessMask;\n  ArrayList accessMask = null;\n  if (ctx.winlog.event_data.AccessMask instanceof String) {\n    accessMask = split(ctx.winlog.event_data.AccessMask);\n  } else if (ctx.winlog.event_data.AccessMask instanceof ArrayList) {\n    accessMask = ctx.winlog.event_data.AccessMask;\n  }\n  for (elem in accessMask) {\n    if (elem.length() == 0) {\n      continue;\n    }\n    def code = elem.replace(\"%%\",\"\").trim();\n    if (params.descriptions.containsKey(code)) {\n      list.add(params.descriptions[code]);\n    } else {\n      list.add(code);\n      if (params.reversed_descriptions.containsKey(code))\n        code = params.reversed_descriptions[code][0];\n    }\n    try {\n      def longCode = Long.decode(code).longValue();\n      lAccessMask |= longCode;\n    } catch (Exception e) {}\n  }\n  if (list.length > 0) {\n    ctx.winlog.event_data.put(\"AccessMask\", list);\n  }\n\n  ArrayList desc = new ArrayList();\n  def[] w = new def[] { null };\n  for (long b = 0; b < 32; b++) {\n    long flag = 1L << b;\n    if ((lAccessMask & flag) == flag) {\n      w[0] = flag;\n      def fDesc = params.AccessMaskDescriptions[String.format(\"0x%08X\", w)];\n      if (fDesc != null) {\n        desc.add(fDesc);\n      }\n    }\n  }\n  if (desc.length > 0) {\n    ctx.winlog.event_data.put(\"AccessMaskDescription\", desc);\n  }\n  ArrayList results = new ArrayList();\n}"#
                        ),
                        cached_params!(
                            "{\"AccessMaskDescriptions\":{\"0x00000001\":\"Create Child\",\"0x00000002\":\"Delete Child\",\"0x00000004\":\"List Contents\",\"0x00000008\":\"SELF\",\"0x00000010\":\"Read Property\",\"0x00000020\":\"Write Property\",\"0x00000040\":\"Delete Treee\",\"0x00000080\":\"List Object\",\"0x00000100\":\"Control Access\",\"0x0000FFFF\":\"SPECIFIC_RIGHTS_ALL\",\"0x00010000\":\"DELETE\",\"0x00020000\":\"READ_CONTROL\",\"0x00040000\":\"WRITE_DAC\",\"0x00080000\":\"WRITE_OWNER\",\"0x00100000\":\"SYNCHRONIZE\",\"0x001F0000\":\"STANDARD_RIGHTS_ALL\",\"0x00F00000\":\"STANDARD_RIGHTS_REQUIRED\",\"0x01000000\":\"ADS_RIGHT_ACCESS_SYSTEM_SECURITY\",\"0x10000000\":\"ADS_RIGHT_GENERIC_ALL\",\"0x20000000\":\"ADS_RIGHT_GENERIC_EXECUTE\",\"0x40000000\":\"ADS_RIGHT_GENERIC_WRITE\",\"0x80000000\":\"ADS_RIGHT_GENERIC_READ\"},\"descriptions\":{\"12288\":\"Security State Change\",\"12289\":\"Security System Extension\",\"12290\":\"System Integrity\",\"12291\":\"IPsec Driver\",\"12292\":\"Other System Events\",\"12544\":\"Logon\",\"12545\":\"Logoff\",\"12546\":\"Account Lockout\",\"12547\":\"IPsec Main Mode\",\"12548\":\"Special Logon\",\"12549\":\"IPsec Quick Mode\",\"12550\":\"IPsec Extended Mode\",\"12551\":\"Other Logon/Logoff Events\",\"12552\":\"Network Policy Server\",\"12553\":\"User / Device Claims\",\"12554\":\"Group Membership\",\"12800\":\"File System\",\"12801\":\"Registry\",\"12802\":\"Kernel Object\",\"12803\":\"SAM\",\"12804\":\"Other Object Access Events\",\"12805\":\"Certification Services\",\"12806\":\"Application Generated\",\"12807\":\"Handle Manipulation\",\"12808\":\"File Share\",\"12809\":\"Filtering Platform Packet Drop\",\"12810\":\"Filtering Platform Connection\",\"12811\":\"Detailed File Share\",\"12812\":\"Removable Storage\",\"12813\":\"Central Policy Staging\",\"13056\":\"Sensitive Privilege Use\",\"13057\":\"Non Sensitive Privilege Use\",\"13058\":\"Other Privilege Use Events\",\"13312\":\"Process Creation\",\"13313\":\"Process Termination\",\"13314\":\"DPAPI Activity\",\"13315\":\"RPC Events\",\"13316\":\"Plug and Play Events\",\"13317\":\"Token Right Adjusted Events\",\"13568\":\"Audit Policy Change\",\"13569\":\"Authentication Policy Change\",\"13570\":\"Authorization Policy Change\",\"13571\":\"MPSSVC Rule-Level Policy Change\",\"13572\":\"Filtering Platform Policy Change\",\"13573\":\"Other Policy Change Events\",\"13824\":\"User Account Management\",\"13825\":\"Computer Account Management\",\"13826\":\"Security Group Management\",\"13827\":\"Distribution Group Management\",\"13828\":\"Application Group Management\",\"13829\":\"Other Account Management Events\",\"14080\":\"Directory Service Access\",\"14081\":\"Directory Service Changes\",\"14082\":\"Directory Service Replication\",\"14083\":\"Detailed Directory Service Replication\",\"14336\":\"Credential Validation\",\"14337\":\"Kerberos Service Ticket Operations\",\"14338\":\"Other Account Logon Events\",\"14339\":\"Kerberos Authentication Service\",\"14592\":\"Inbound\",\"14593\":\"Outbound\",\"14594\":\"Forward\",\"14595\":\"Bidirectional\",\"14596\":\"IP Packet\",\"14597\":\"Transport\",\"14598\":\"Forward\",\"14599\":\"Stream\",\"14600\":\"Datagram Data\",\"14601\":\"ICMP Error\",\"14602\":\"MAC 802.3\",\"14603\":\"MAC Native\",\"14604\":\"vSwitch\",\"14608\":\"Resource Assignment\",\"14609\":\"Listen\",\"14610\":\"Receive/Accept\",\"14611\":\"Connect\",\"14612\":\"Flow Established\",\"14614\":\"Resource Release\",\"14615\":\"Endpoint Closure\",\"14616\":\"Connect Redirect\",\"14617\":\"Bind Redirect\",\"14624\":\"Stream Packet\",\"14640\":\"ICMP Echo-Request\",\"14641\":\"vSwitch Ingress\",\"14642\":\"vSwitch Egress\",\"14672\":\"<Binary>\",\"14673\":\"[NULL]\",\"14674\":\"Value Added\",\"14675\":\"Value Deleted\",\"14676\":\"Active Directory Domain Services\",\"14677\":\"Active Directory Lightweight Directory Services\",\"14678\":\"Yes\",\"14679\":\"No\",\"14680\":\"Value Added With Expiration Time\",\"14681\":\"Value Deleted With Expiration Time\",\"14688\":\"Value Auto Deleted With Expiration Time\",\"1536\":\"Unused message ID\",\"1537\":\"DELETE\",\"1538\":\"READ_CONTROL\",\"1539\":\"WRITE_DAC\",\"1540\":\"WRITE_OWNER\",\"1541\":\"SYNCHRONIZE\",\"1542\":\"ACCESS_SYS_SEC\",\"1543\":\"MAX_ALLOWED\",\"1552\":\"Unknown specific access (bit 0)\",\"1553\":\"Unknown specific access (bit 1)\",\"1554\":\"Unknown specific access (bit 2)\",\"1555\":\"Unknown specific access (bit 3)\",\"1556\":\"Unknown specific access (bit 4)\",\"1557\":\"Unknown specific access (bit 5)\",\"1558\":\"Unknown specific access (bit 6)\",\"1559\":\"Unknown specific access (bit 7)\",\"1560\":\"Unknown specific access (bit 8)\",\"1561\":\"Unknown specific access (bit 9)\",\"1562\":\"Unknown specific access (bit 10)\",\"1563\":\"Unknown specific access (bit 11)\",\"1564\":\"Unknown specific access (bit 12)\",\"1565\":\"Unknown specific access (bit 13)\",\"1566\":\"Unknown specific access (bit 14)\",\"1567\":\"Unknown specific access (bit 15)\",\"1601\":\"Not used\",\"1603\":\"Assign Primary Token Privilege\",\"1604\":\"Lock Memory Privilege\",\"1605\":\"Increase Memory Quota Privilege\",\"1606\":\"Unsolicited Input Privilege\",\"1607\":\"Trusted Computer Base Privilege\",\"1608\":\"Security Privilege\",\"1609\":\"Take Ownership Privilege\",\"1610\":\"Load/Unload Driver Privilege\",\"1611\":\"Profile System Privilege\",\"1612\":\"Set System Time Privilege\",\"1613\":\"Profile Single Process Privilege\",\"1614\":\"Increment Base Priority Privilege\",\"1615\":\"Create Pagefile Privilege\",\"1616\":\"Create Permanent Object Privilege\",\"1617\":\"Backup Privilege\",\"1618\":\"Restore From Backup Privilege\",\"1619\":\"Shutdown System Privilege\",\"1620\":\"Debug Privilege\",\"1621\":\"View or Change Audit Log Privilege\",\"1622\":\"Change Hardware Environment Privilege\",\"1623\":\"Change Notify (and Traverse) Privilege\",\"1624\":\"Remotely Shut System Down Privilege\",\"16384\":\"Add\",\"16385\":\"Delete\",\"16386\":\"Boot-time\",\"16387\":\"Persistent\",\"16388\":\"Not persistent\",\"16389\":\"Block\",\"16390\":\"Permit\",\"16391\":\"Callout\",\"16392\":\"MD5\",\"16393\":\"SHA-1\",\"16394\":\"SHA-256\",\"16395\":\"AES-GCM 128\",\"16396\":\"AES-GCM 192\",\"16397\":\"AES-GCM 256\",\"16398\":\"DES\",\"16399\":\"3DES\",\"16400\":\"AES-128\",\"16401\":\"AES-192\",\"16402\":\"AES-256\",\"16403\":\"Transport\",\"16404\":\"Tunnel\",\"16405\":\"Responder\",\"16406\":\"Initiator\",\"16407\":\"AES-GMAC 128\",\"16408\":\"AES-GMAC 192\",\"16409\":\"AES-GMAC 256\",\"16416\":\"AuthNoEncap Transport\",\"16896\":\"Enable WMI Account\",\"16897\":\"Execute Method\",\"16898\":\"Full Write\",\"16899\":\"Partial Write\",\"16900\":\"Provider Write\",\"16901\":\"Remote Access\",\"16902\":\"Subscribe\",\"16903\":\"Publish\",\"1792\":\"<value changed\",\"1793\":\"<value not set>\",\"1794\":\"<never>\",\"1795\":\"Enabled\",\"1796\":\"Disabled\",\"1797\":\"All\",\"1798\":\"None\",\"1799\":\"Audit Policy query/set API Operation\",\"1800\":\"<Value change auditing for this registry type is not supported>\",\"1801\":\"Granted by\",\"1802\":\"Denied by\",\"1803\":\"Denied by Integrity Policy check\",\"1804\":\"Granted by Ownership\",\"1805\":\"Not granted\",\"1806\":\"Granted by NULL DACL\",\"1807\":\"Denied by Empty DACL\",\"1808\":\"Granted by NULL Security Descriptor\",\"1809\":\"Unknown or unchecked\",\"1810\":\"Not granted due to missing\",\"1811\":\"Granted by ACE on parent folder\",\"1812\":\"Denied by ACE on parent folder\",\"1813\":\"Granted by Central Access Rule\",\"1814\":\"NOT Granted by Central Access Rule\",\"1815\":\"Granted by parent folder's Central Access Rule\",\"1816\":\"NOT Granted by parent folder's Central Access Rule\",\"1817\":\"Unknown Type\",\"1818\":\"String\",\"1819\":\"Unsigned 64-bit Integer\",\"1820\":\"64-bit Integer\",\"1821\":\"FQBN\",\"1822\":\"Blob\",\"1823\":\"Sid\",\"1824\":\"Boolean\",\"1825\":\"TRUE\",\"1826\":\"FALSE\",\"1827\":\"Invalid\",\"1828\":\"an ACE too long to display\",\"1829\":\"a Security Descriptor too long to display\",\"1830\":\"Not granted to AppContainers\",\"1831\":\"...\",\"1832\":\"Identification\",\"1833\":\"Impersonation\",\"1840\":\"Delegation\",\"1841\":\"Denied by Process Trust Label ACE\",\"1842\":\"Yes\",\"1843\":\"No\",\"1844\":\"System\",\"1845\":\"Not Available\",\"1846\":\"Default\",\"1847\":\"DisallowMmConfig\",\"1848\":\"Off\",\"1849\":\"Auto\",\"1872\":\"REG_NONE\",\"1873\":\"REG_SZ\",\"1874\":\"REG_EXPAND_SZ\",\"1875\":\"REG_BINARY\",\"1876\":\"REG_DWORD\",\"1877\":\"REG_DWORD_BIG_ENDIAN\",\"1878\":\"REG_LINK\",\"1879\":\"REG_MULTI_SZ (New lines are replaced with *. A * is replaced with **)\",\"1880\":\"REG_RESOURCE_LIST\",\"1881\":\"REG_FULL_RESOURCE_DESCRIPTOR\",\"1882\":\"REG_RESOURCE_REQUIREMENTS_LIST\",\"1883\":\"REG_QWORD\",\"1904\":\"New registry value created\",\"1905\":\"Existing registry value modified\",\"1906\":\"Registry value deleted\",\"1920\":\"Sunday\",\"1921\":\"Monday\",\"1922\":\"Tuesday\",\"1923\":\"Wednesday\",\"1924\":\"Thursday\",\"1925\":\"Friday\",\"1926\":\"Saturday\",\"1936\":\"TokenElevationTypeDefault (1)\",\"1937\":\"TokenElevationTypeFull (2)\",\"1938\":\"TokenElevationTypeLimited (3)\",\"2048\":\"Account Enabled\",\"2049\":\"Home Directory Required' - Disabled\",\"2050\":\"Password Not Required' - Disabled\",\"2051\":\"Temp Duplicate Account' - Disabled\",\"2052\":\"Normal Account' - Disabled\",\"2053\":\"MNS Logon Account' - Disabled\",\"2054\":\"Interdomain Trust Account' - Disabled\",\"2055\":\"Workstation Trust Account' - Disabled\",\"2056\":\"Server Trust Account' - Disabled\",\"2057\":\"Don't Expire Password' - Disabled\",\"2058\":\"Account Unlocked\",\"2059\":\"Encrypted Text Password Allowed' - Disabled\",\"2060\":\"Smartcard Required' - Disabled\",\"2061\":\"Trusted For Delegation' - Disabled\",\"2062\":\"Not Delegated' - Disabled\",\"2063\":\"Use DES Key Only' - Disabled\",\"2064\":\"Don't Require Preauth' - Disabled\",\"2065\":\"Password Expired' - Disabled\",\"2066\":\"Trusted To Authenticate For Delegation' - Disabled\",\"2067\":\"Exclude Authorization Information' - Disabled\",\"2068\":\"Undefined UserAccountControl Bit 20' - Disabled\",\"2069\":\"Protect Kerberos Service Tickets with AES Keys' - Disabled\",\"2070\":\"Undefined UserAccountControl Bit 22' - Disabled\",\"2071\":\"Undefined UserAccountControl Bit 23' - Disabled\",\"2072\":\"Undefined UserAccountControl Bit 24' - Disabled\",\"2073\":\"Undefined UserAccountControl Bit 25' - Disabled\",\"2074\":\"Undefined UserAccountControl Bit 26' - Disabled\",\"2075\":\"Undefined UserAccountControl Bit 27' - Disabled\",\"2076\":\"Undefined UserAccountControl Bit 28' - Disabled\",\"2077\":\"Undefined UserAccountControl Bit 29' - Disabled\",\"2078\":\"Undefined UserAccountControl Bit 30' - Disabled\",\"2079\":\"Undefined UserAccountControl Bit 31' - Disabled\",\"2080\":\"Account Disabled\",\"2081\":\"Home Directory Required' - Enabled\",\"2082\":\"Password Not Required' - Enabled\",\"2083\":\"Temp Duplicate Account' - Enabled\",\"2084\":\"Normal Account' - Enabled\",\"2085\":\"MNS Logon Account' - Enabled\",\"2086\":\"Interdomain Trust Account' - Enabled\",\"2087\":\"Workstation Trust Account' - Enabled\",\"2088\":\"Server Trust Account' - Enabled\",\"2089\":\"Don't Expire Password' - Enabled\",\"2090\":\"Account Locked\",\"2091\":\"Encrypted Text Password Allowed' - Enabled\",\"2092\":\"Smartcard Required' - Enabled\",\"2093\":\"Trusted For Delegation' - Enabled\",\"2094\":\"Not Delegated' - Enabled\",\"2095\":\"Use DES Key Only' - Enabled\",\"2096\":\"Don't Require Preauth' - Enabled\",\"2097\":\"Password Expired' - Enabled\",\"2098\":\"Trusted To Authenticate For Delegation' - Enabled\",\"2099\":\"Exclude Authorization Information' - Enabled\",\"2100\":\"Undefined UserAccountControl Bit 20' - Enabled\",\"2101\":\"Protect Kerberos Service Tickets with AES Keys' - Enabled\",\"2102\":\"Undefined UserAccountControl Bit 22' - Enabled\",\"2103\":\"Undefined UserAccountControl Bit 23' - Enabled\",\"2104\":\"Undefined UserAccountControl Bit 24' - Enabled\",\"2105\":\"Undefined UserAccountControl Bit 25' - Enabled\",\"2106\":\"Undefined UserAccountControl Bit 26' - Enabled\",\"2107\":\"Undefined UserAccountControl Bit 27' - Enabled\",\"2108\":\"Undefined UserAccountControl Bit 28' - Enabled\",\"2109\":\"Undefined UserAccountControl Bit 29' - Enabled\",\"2110\":\"Undefined UserAccountControl Bit 30' - Enabled\",\"2111\":\"Undefined UserAccountControl Bit 31' - Enabled\",\"2304\":\"An Error occured during Logon.\",\"2305\":\"The specified user account has expired.\",\"2306\":\"The NetLogon component is not active.\",\"2307\":\"Account locked out.\",\"2308\":\"The user has not been granted the requested logon type at this machine.\",\"2309\":\"The specified account's password has expired.\",\"2310\":\"Account currently disabled.\",\"2311\":\"Account logon time restriction violation.\",\"2312\":\"User not allowed to logon at this computer.\",\"2313\":\"Unknown user name or bad password.\",\"2314\":\"Domain sid inconsistent.\",\"2315\":\"Smartcard logon is required and was not used.\",\"2432\":\"Not Available.\",\"2436\":\"Random number generator failure.\",\"2437\":\"Random number generation failed FIPS-140 pre-hash check.\",\"2438\":\"Failed to zero secret data.\",\"2439\":\"Key failed pair wise consistency check.\",\"2448\":\"Failed to unprotect persistent cryptographic key.\",\"2449\":\"Key export checks failed.\",\"2450\":\"Validation of public key failed.\",\"2451\":\"Signature verification failed.\",\"2456\":\"Open key file.\",\"2457\":\"Delete key file.\",\"2458\":\"Read persisted key from file.\",\"2459\":\"Write persisted key to file.\",\"2464\":\"Export of persistent cryptographic key.\",\"2465\":\"Import of persistent cryptographic key.\",\"2480\":\"Open Key.\",\"2481\":\"Create Key.\",\"2482\":\"Delete Key.\",\"2483\":\"Encrypt.\",\"2484\":\"Decrypt.\",\"2485\":\"Sign hash.\",\"2486\":\"Secret agreement.\",\"2487\":\"Domain settings\",\"2488\":\"Local settings\",\"2489\":\"Add provider.\",\"2490\":\"Remove provider.\",\"2491\":\"Add context.\",\"2492\":\"Remove context.\",\"2493\":\"Add function.\",\"2494\":\"Remove function.\",\"2495\":\"Add function provider.\",\"2496\":\"Remove function provider.\",\"2497\":\"Add function property.\",\"2498\":\"Remove function property.\",\"2499\":\"Machine key.\",\"2500\":\"User key.\",\"2501\":\"Key Derivation.\",\"279\":\"Undefined Access (no effect) Bit 7\",\"4352\":\"Device Access Bit 0\",\"4353\":\"Device Access Bit 1\",\"4354\":\"Device Access Bit 2\",\"4355\":\"Device Access Bit 3\",\"4356\":\"Device Access Bit 4\",\"4357\":\"Device Access Bit 5\",\"4358\":\"Device Access Bit 6\",\"4359\":\"Device Access Bit 7\",\"4360\":\"Device Access Bit 8\",\"4361\":\"Undefined Access (no effect) Bit 9\",\"4362\":\"Undefined Access (no effect) Bit 10\",\"4363\":\"Undefined Access (no effect) Bit 11\",\"4364\":\"Undefined Access (no effect) Bit 12\",\"4365\":\"Undefined Access (no effect) Bit 13\",\"4366\":\"Undefined Access (no effect) Bit 14\",\"4367\":\"Undefined Access (no effect) Bit 15\",\"4368\":\"Query directory\",\"4369\":\"Traverse\",\"4370\":\"Create object in directory\",\"4371\":\"Create sub-directory\",\"4372\":\"Undefined Access (no effect) Bit 4\",\"4373\":\"Undefined Access (no effect) Bit 5\",\"4374\":\"Undefined Access (no effect) Bit 6\",\"4375\":\"Undefined Access (no effect) Bit 7\",\"4376\":\"Undefined Access (no effect) Bit 8\",\"4377\":\"Undefined Access (no effect) Bit 9\",\"4378\":\"Undefined Access (no effect) Bit 10\",\"4379\":\"Undefined Access (no effect) Bit 11\",\"4380\":\"Undefined Access (no effect) Bit 12\",\"4381\":\"Undefined Access (no effect) Bit 13\",\"4382\":\"Undefined Access (no effect) Bit 14\",\"4383\":\"Undefined Access (no effect) Bit 15\",\"4384\":\"Query event state\",\"4385\":\"Modify event state\",\"4386\":\"Undefined Access (no effect) Bit 2\",\"4387\":\"Undefined Access (no effect) Bit 3\",\"4388\":\"Undefined Access (no effect) Bit 4\",\"4389\":\"Undefined Access (no effect) Bit 5\",\"4390\":\"Undefined Access (no effect) Bit 6\",\"4391\":\"Undefined Access (no effect) Bit 7\",\"4392\":\"Undefined Access (no effect) Bit 8\",\"4393\":\"Undefined Access (no effect) Bit 9\",\"4394\":\"Undefined Access (no effect) Bit 10\",\"4395\":\"Undefined Access (no effect) Bit 11\",\"4396\":\"Undefined Access (no effect) Bit 12\",\"4397\":\"Undefined Access (no effect) Bit 13\",\"4398\":\"Undefined Access (no effect) Bit 14\",\"4399\":\"Undefined Access (no effect) Bit 15\",\"4416\":\"ReadData (or ListDirectory)\",\"4417\":\"WriteData (or AddFile)\",\"4418\":\"AppendData (or AddSubdirectory or CreatePipeInstance)\",\"4419\":\"ReadEA\",\"4420\":\"WriteEA\",\"4421\":\"Execute/Traverse\",\"4422\":\"DeleteChild\",\"4423\":\"ReadAttributes\",\"4424\":\"WriteAttributes\",\"4425\":\"Undefined Access (no effect) Bit 9\",\"4426\":\"Undefined Access (no effect) Bit 10\",\"4427\":\"Undefined Access (no effect) Bit 11\",\"4428\":\"Undefined Access (no effect) Bit 12\",\"4429\":\"Undefined Access (no effect) Bit 13\",\"4430\":\"Undefined Access (no effect) Bit 14\",\"4431\":\"Undefined Access (no effect) Bit 15\",\"4432\":\"Query key value\",\"4433\":\"Set key value\",\"4434\":\"Create sub-key\",\"4435\":\"Enumerate sub-keys\",\"4436\":\"Notify about changes to keys\",\"4437\":\"Create Link\",\"4438\":\"Undefined Access (no effect) Bit 6\",\"4439\":\"Undefined Access (no effect) Bit 7\",\"4440\":\"Enable 64(or 32) bit application to open 64 bit key\",\"4441\":\"Enable 64(or 32) bit application to open 32 bit key\",\"4442\":\"Undefined Access (no effect) Bit 10\",\"4443\":\"Undefined Access (no effect) Bit 11\",\"4444\":\"Undefined Access (no effect) Bit 12\",\"4445\":\"Undefined Access (no effect) Bit 13\",\"4446\":\"Undefined Access (no effect) Bit 14\",\"4447\":\"Undefined Access (no effect) Bit 15\",\"4448\":\"Query mutant state\",\"4449\":\"Undefined Access (no effect) Bit 1\",\"4450\":\"Undefined Access (no effect) Bit 2\",\"4451\":\"Undefined Access (no effect) Bit 3\",\"4452\":\"Undefined Access (no effect) Bit 4\",\"4453\":\"Undefined Access (no effect) Bit 5\",\"4454\":\"Undefined Access (no effect) Bit 6\",\"4455\":\"Undefined Access (no effect) Bit 7\",\"4456\":\"Undefined Access (no effect) Bit 8\",\"4457\":\"Undefined Access (no effect) Bit 9\",\"4458\":\"Undefined Access (no effect) Bit 10\",\"4459\":\"Undefined Access (no effect) Bit 11\",\"4460\":\"Undefined Access (no effect) Bit 12\",\"4461\":\"Undefined Access (no effect) Bit 13\",\"4462\":\"Undefined Access (no effect) Bit 14\",\"4463\":\"Undefined Access (no effect) Bit 15\",\"4464\":\"Communicate using port\",\"4465\":\"Undefined Access (no effect) Bit 1\",\"4466\":\"Undefined Access (no effect) Bit 2\",\"4467\":\"Undefined Access (no effect) Bit 3\",\"4468\":\"Undefined Access (no effect) Bit 4\",\"4469\":\"Undefined Access (no effect) Bit 5\",\"4470\":\"Undefined Access (no effect) Bit 6\",\"4471\":\"Undefined Access (no effect) Bit 7\",\"4472\":\"Undefined Access (no effect) Bit 8\",\"4473\":\"Undefined Access (no effect) Bit 9\",\"4474\":\"Undefined Access (no effect) Bit 10\",\"4475\":\"Undefined Access (no effect) Bit 11\",\"4476\":\"Undefined Access (no effect) Bit 12\",\"4477\":\"Undefined Access (no effect) Bit 13\",\"4478\":\"Undefined Access (no effect) Bit 14\",\"4479\":\"Undefined Access (no effect) Bit 15\",\"4480\":\"Force process termination\",\"4481\":\"Create new thread in process\",\"4482\":\"Set process session ID\",\"4483\":\"Perform virtual memory operation\",\"4484\":\"Read from process memory\",\"4485\":\"Write to process memory\",\"4486\":\"Duplicate handle into or out of process\",\"4487\":\"Create a subprocess of process\",\"4488\":\"Set process quotas\",\"4489\":\"Set process information\",\"4490\":\"Query process information\",\"4491\":\"Set process termination port\",\"4492\":\"Undefined Access (no effect) Bit 12\",\"4493\":\"Undefined Access (no effect) Bit 13\",\"4494\":\"Undefined Access (no effect) Bit 14\",\"4495\":\"Undefined Access (no effect) Bit 15\",\"4496\":\"Control profile\",\"4497\":\"Undefined Access (no effect) Bit 1\",\"4498\":\"Undefined Access (no effect) Bit 2\",\"4499\":\"Undefined Access (no effect) Bit 3\",\"4500\":\"Undefined Access (no effect) Bit 4\",\"4501\":\"Undefined Access (no effect) Bit 5\",\"4502\":\"Undefined Access (no effect) Bit 6\",\"4503\":\"Undefined Access (no effect) Bit 7\",\"4504\":\"Undefined Access (no effect) Bit 8\",\"4505\":\"Undefined Access (no effect) Bit 9\",\"4506\":\"Undefined Access (no effect) Bit 10\",\"4507\":\"Undefined Access (no effect) Bit 11\",\"4508\":\"Undefined Access (no effect) Bit 12\",\"4509\":\"Undefined Access (no effect) Bit 13\",\"4510\":\"Undefined Access (no effect) Bit 14\",\"4511\":\"Undefined Access (no effect) Bit 15\",\"4512\":\"Query section state\",\"4513\":\"Map section for write\",\"4514\":\"Map section for read\",\"4515\":\"Map section for execute\",\"4516\":\"Extend size\",\"4517\":\"Undefined Access (no effect) Bit 5\",\"4518\":\"Undefined Access (no effect) Bit 6\",\"4519\":\"Undefined Access (no effect) Bit 7\",\"4520\":\"Undefined Access (no effect) Bit 8\",\"4521\":\"Undefined Access (no effect) Bit 9\",\"4522\":\"Undefined Access (no effect) Bit 10\",\"4523\":\"Undefined Access (no effect) Bit 11\",\"4524\":\"Undefined Access (no effect) Bit 12\",\"4525\":\"Undefined Access (no effect) Bit 13\",\"4526\":\"Undefined Access (no effect) Bit 14\",\"4527\":\"Undefined Access (no effect) Bit 15\",\"4528\":\"Query semaphore state\",\"4529\":\"Modify semaphore state\",\"4530\":\"Undefined Access (no effect) Bit 2\",\"4531\":\"Undefined Access (no effect) Bit 3\",\"4532\":\"Undefined Access (no effect) Bit 4\",\"4533\":\"Undefined Access (no effect) Bit 5\",\"4534\":\"Undefined Access (no effect) Bit 6\",\"4535\":\"Undefined Access (no effect) Bit 7\",\"4536\":\"Undefined Access (no effect) Bit 8\",\"4537\":\"Undefined Access (no effect) Bit 9\",\"4538\":\"Undefined Access (no effect) Bit 10\",\"4539\":\"Undefined Access (no effect) Bit 11\",\"4540\":\"Undefined Access (no effect) Bit 12\",\"4541\":\"Undefined Access (no effect) Bit 13\",\"4542\":\"Undefined Access (no effect) Bit 14\",\"4543\":\"Undefined Access (no effect) Bit 15\",\"4544\":\"Use symbolic link\",\"4545\":\"Undefined Access (no effect) Bit 1\",\"4546\":\"Undefined Access (no effect) Bit 2\",\"4547\":\"Undefined Access (no effect) Bit 3\",\"4548\":\"Undefined Access (no effect) Bit 4\",\"4549\":\"Undefined Access (no effect) Bit 5\",\"4550\":\"Undefined Access (no effect) Bit 6\",\"4551\":\"Undefined Access (no effect) Bit 7\",\"4552\":\"Undefined Access (no effect) Bit 8\",\"4553\":\"Undefined Access (no effect) Bit 9\",\"4554\":\"Undefined Access (no effect) Bit 10\",\"4555\":\"Undefined Access (no effect) Bit 11\",\"4556\":\"Undefined Access (no effect) Bit 12\",\"4557\":\"Undefined Access (no effect) Bit 13\",\"4558\":\"Undefined Access (no effect) Bit 14\",\"4559\":\"Undefined Access (no effect) Bit 15\",\"4560\":\"Force thread termination\",\"4561\":\"Suspend or resume thread\",\"4562\":\"Send an alert to thread\",\"4563\":\"Get thread context\",\"4564\":\"Set thread context\",\"4565\":\"Set thread information\",\"4566\":\"Query thread information\",\"4567\":\"Assign a token to the thread\",\"4568\":\"Cause thread to directly impersonate another thread\",\"4569\":\"Directly impersonate this thread\",\"4570\":\"Undefined Access (no effect) Bit 10\",\"4571\":\"Undefined Access (no effect) Bit 11\",\"4572\":\"Undefined Access (no effect) Bit 12\",\"4573\":\"Undefined Access (no effect) Bit 13\",\"4574\":\"Undefined Access (no effect) Bit 14\",\"4575\":\"Undefined Access (no effect) Bit 15\",\"4576\":\"Query timer state\",\"4577\":\"Modify timer state\",\"4578\":\"Undefined Access (no effect) Bit 2\",\"4579\":\"Undefined Access (no effect) Bit 3\",\"4580\":\"Undefined Access (no effect) Bit 4\",\"4581\":\"Undefined Access (no effect) Bit 5\",\"4582\":\"Undefined Access (no effect) Bit 6\",\"4584\":\"Undefined Access (no effect) Bit 8\",\"4585\":\"Undefined Access (no effect) Bit 9\",\"4586\":\"Undefined Access (no effect) Bit 10\",\"4587\":\"Undefined Access (no effect) Bit 11\",\"4588\":\"Undefined Access (no effect) Bit 12\",\"4589\":\"Undefined Access (no effect) Bit 13\",\"4590\":\"Undefined Access (no effect) Bit 14\",\"4591\":\"Undefined Access (no effect) Bit 15\",\"4592\":\"AssignAsPrimary\",\"4593\":\"Duplicate\",\"4594\":\"Impersonate\",\"4595\":\"Query\",\"4596\":\"QuerySource\",\"4597\":\"AdjustPrivileges\",\"4598\":\"AdjustGroups\",\"4599\":\"AdjustDefaultDacl\",\"4600\":\"AdjustSessionID\",\"4601\":\"Undefined Access (no effect) Bit 9\",\"4602\":\"Undefined Access (no effect) Bit 10\",\"4603\":\"Undefined Access (no effect) Bit 11\",\"4604\":\"Undefined Access (no effect) Bit 12\",\"4605\":\"Undefined Access (no effect) Bit 13\",\"4606\":\"Undefined Access (no effect) Bit 14\",\"4607\":\"Undefined Access (no effect) Bit 15\",\"4608\":\"Create instance of object type\",\"4609\":\"Undefined Access (no effect) Bit 1\",\"4610\":\"Undefined Access (no effect) Bit 2\",\"4611\":\"Undefined Access (no effect) Bit 3\",\"4612\":\"Undefined Access (no effect) Bit 4\",\"4613\":\"Undefined Access (no effect) Bit 5\",\"4614\":\"Undefined Access (no effect) Bit 6\",\"4615\":\"Undefined Access (no effect) Bit 7\",\"4616\":\"Undefined Access (no effect) Bit 8\",\"4617\":\"Undefined Access (no effect) Bit 9\",\"4618\":\"Undefined Access (no effect) Bit 10\",\"4619\":\"Undefined Access (no effect) Bit 11\",\"4620\":\"Undefined Access (no effect) Bit 12\",\"4621\":\"Undefined Access (no effect) Bit 13\",\"4622\":\"Undefined Access (no effect) Bit 14\",\"4623\":\"Undefined Access (no effect) Bit 15\",\"4864\":\"Query State\",\"4865\":\"Modify State\",\"5120\":\"Channel read message\",\"5121\":\"Channel write message\",\"5122\":\"Channel query information\",\"5123\":\"Channel set information\",\"5124\":\"Undefined Access (no effect) Bit 4\",\"5125\":\"Undefined Access (no effect) Bit 5\",\"5126\":\"Undefined Access (no effect) Bit 6\",\"5127\":\"Undefined Access (no effect) Bit 7\",\"5128\":\"Undefined Access (no effect) Bit 8\",\"5129\":\"Undefined Access (no effect) Bit 9\",\"5130\":\"Undefined Access (no effect) Bit 10\",\"5131\":\"Undefined Access (no effect) Bit 11\",\"5132\":\"Undefined Access (no effect) Bit 12\",\"5133\":\"Undefined Access (no effect) Bit 13\",\"5134\":\"Undefined Access (no effect) Bit 14\",\"5135\":\"Undefined Access (no effect) Bit 15\",\"5136\":\"Assign process\",\"5137\":\"Set Attributes\",\"5138\":\"Query Attributes\",\"5139\":\"Terminate Job\",\"5140\":\"Set Security Attributes\",\"5141\":\"Undefined Access (no effect) Bit 5\",\"5142\":\"Undefined Access (no effect) Bit 6\",\"5143\":\"Undefined Access (no effect) Bit 7\",\"5144\":\"Undefined Access (no effect) Bit 8\",\"5145\":\"Undefined Access (no effect) Bit 9\",\"5146\":\"Undefined Access (no effect) Bit 10\",\"5147\":\"Undefined Access (no effect) Bit 11\",\"5148\":\"Undefined Access (no effect) Bit 12\",\"5149\":\"Undefined Access (no effect) Bit 13\",\"5150\":\"Undefined Access (no effect) Bit 14\",\"5151\":\"Undefined Access (no effect) Bit 15\",\"5376\":\"ConnectToServer\",\"5377\":\"ShutdownServer\",\"5378\":\"InitializeServer\",\"5379\":\"CreateDomain\",\"5380\":\"EnumerateDomains\",\"5381\":\"LookupDomain\",\"5382\":\"Undefined Access (no effect) Bit 6\",\"5383\":\"Undefined Access (no effect) Bit 7\",\"5384\":\"Undefined Access (no effect) Bit 8\",\"5385\":\"Undefined Access (no effect) Bit 9\",\"5386\":\"Undefined Access (no effect) Bit 10\",\"5387\":\"Undefined Access (no effect) Bit 11\",\"5388\":\"Undefined Access (no effect) Bit 12\",\"5389\":\"Undefined Access (no effect) Bit 13\",\"5390\":\"Undefined Access (no effect) Bit 14\",\"5391\":\"Undefined Access (no effect) Bit 15\",\"5392\":\"ReadPasswordParameters\",\"5393\":\"WritePasswordParameters\",\"5394\":\"ReadOtherParameters\",\"5395\":\"WriteOtherParameters\",\"5396\":\"CreateUser\",\"5397\":\"CreateGlobalGroup\",\"5398\":\"CreateLocalGroup\",\"5399\":\"GetLocalGroupMembership\",\"5400\":\"ListAccounts\",\"5401\":\"LookupIDs\",\"5402\":\"AdministerServer\",\"5403\":\"Undefined Access (no effect) Bit 11\",\"5404\":\"Undefined Access (no effect) Bit 12\",\"5405\":\"Undefined Access (no effect) Bit 13\",\"5406\":\"Undefined Access (no effect) Bit 14\",\"5407\":\"Undefined Access (no effect) Bit 15\",\"5408\":\"ReadInformation\",\"5409\":\"WriteAccount\",\"5410\":\"AddMember\",\"5411\":\"RemoveMember\",\"5412\":\"ListMembers\",\"5413\":\"Undefined Access (no effect) Bit 5\",\"5414\":\"Undefined Access (no effect) Bit 6\",\"5415\":\"Undefined Access (no effect) Bit 7\",\"5416\":\"Undefined Access (no effect) Bit 8\",\"5417\":\"Undefined Access (no effect) Bit 9\",\"5418\":\"Undefined Access (no effect) Bit 10\",\"5419\":\"Undefined Access (no effect) Bit 11\",\"5420\":\"Undefined Access (no effect) Bit 12\",\"5421\":\"Undefined Access (no effect) Bit 13\",\"5422\":\"Undefined Access (no effect) Bit 14\",\"5423\":\"Undefined Access (no effect) Bit 15\",\"5424\":\"AddMember\",\"5425\":\"RemoveMember\",\"5426\":\"ListMembers\",\"5427\":\"ReadInformation\",\"5428\":\"WriteAccount\",\"5429\":\"Undefined Access (no effect) Bit 5\",\"5430\":\"Undefined Access (no effect) Bit 6\",\"5431\":\"Undefined Access (no effect) Bit 7\",\"5432\":\"Undefined Access (no effect) Bit 8\",\"5433\":\"Undefined Access (no effect) Bit 9\",\"5434\":\"Undefined Access (no effect) Bit 10\",\"5435\":\"Undefined Access (no effect) Bit 11\",\"5436\":\"Undefined Access (no effect) Bit 12\",\"5437\":\"Undefined Access (no effect) Bit 13\",\"5438\":\"Undefined Access (no effect) Bit 14\",\"5439\":\"Undefined Access (no effect) Bit 15\",\"5440\":\"ReadGeneralInformation\",\"5441\":\"ReadPreferences\",\"5442\":\"WritePreferences\",\"5443\":\"ReadLogon\",\"5444\":\"ReadAccount\",\"5445\":\"WriteAccount\",\"5446\":\"ChangePassword (with knowledge of old password)\",\"5447\":\"SetPassword (without knowledge of old password)\",\"5448\":\"ListGroups\",\"5449\":\"ReadGroupMembership\",\"5450\":\"ChangeGroupMembership\",\"5451\":\"Undefined Access (no effect) Bit 11\",\"5452\":\"Undefined Access (no effect) Bit 12\",\"5453\":\"Undefined Access (no effect) Bit 13\",\"5454\":\"Undefined Access (no effect) Bit 14\",\"5455\":\"Undefined Access (no effect) Bit 15\",\"5632\":\"View non-sensitive policy information\",\"5633\":\"View system audit requirements\",\"5634\":\"Get sensitive policy information\",\"5635\":\"Modify domain trust relationships\",\"5636\":\"Create special accounts (for assignment of user rights)\",\"5637\":\"Create a secret object\",\"5638\":\"Create a privilege\",\"5639\":\"Set default quota limits\",\"5640\":\"Change system audit requirements\",\"5641\":\"Administer audit log attributes\",\"5642\":\"Enable/Disable LSA\",\"5643\":\"Lookup Names/SIDs\",\"5648\":\"Change secret value\",\"5649\":\"Query secret value\",\"5650\":\"Undefined Access (no effect) Bit 2\",\"5651\":\"Undefined Access (no effect) Bit 3\",\"5652\":\"Undefined Access (no effect) Bit 4\",\"5653\":\"Undefined Access (no effect) Bit 5\",\"5654\":\"Undefined Access (no effect) Bit 6\",\"5655\":\"Undefined Access (no effect) Bit 7\",\"5656\":\"Undefined Access (no effect) Bit 8\",\"5657\":\"Undefined Access (no effect) Bit 9\",\"5658\":\"Undefined Access (no effect) Bit 10\",\"5659\":\"Undefined Access (no effect) Bit 11\",\"5660\":\"Undefined Access (no effect) Bit 12\",\"5661\":\"Undefined Access (no effect) Bit 13\",\"5662\":\"Undefined Access (no effect) Bit 14\",\"5663\":\"Undefined Access (no effect) Bit 15\",\"5664\":\"Query trusted domain name/SID\",\"5665\":\"Retrieve the controllers in the trusted domain\",\"5666\":\"Change the controllers in the trusted domain\",\"5667\":\"Query the Posix ID offset assigned to the trusted domain\",\"5668\":\"Change the Posix ID offset assigned to the trusted domain\",\"5669\":\"Undefined Access (no effect) Bit 5\",\"5670\":\"Undefined Access (no effect) Bit 6\",\"5671\":\"Undefined Access (no effect) Bit 7\",\"5672\":\"Undefined Access (no effect) Bit 8\",\"5673\":\"Undefined Access (no effect) Bit 9\",\"5674\":\"Undefined Access (no effect) Bit 10\",\"5675\":\"Undefined Access (no effect) Bit 11\",\"5676\":\"Undefined Access (no effect) Bit 12\",\"5677\":\"Undefined Access (no effect) Bit 13\",\"5678\":\"Undefined Access (no effect) Bit 14\",\"5679\":\"Undefined Access (no effect) Bit 15\",\"5680\":\"Query account information\",\"5681\":\"Change privileges assigned to account\",\"5682\":\"Change quotas assigned to account\",\"5683\":\"Change logon capabilities assigned to account\",\"5684\":\"Change the Posix ID offset assigned to the accounted domain\",\"5685\":\"Undefined Access (no effect) Bit 5\",\"5686\":\"Undefined Access (no effect) Bit 6\",\"5687\":\"Undefined Access (no effect) Bit 7\",\"5688\":\"Undefined Access (no effect) Bit 8\",\"5689\":\"Undefined Access (no effect) Bit 9\",\"5690\":\"Undefined Access (no effect) Bit 10\",\"5691\":\"Undefined Access (no effect) Bit 11\",\"5692\":\"Undefined Access (no effect) Bit 12\",\"5693\":\"Undefined Access (no effect) Bit 13\",\"5694\":\"Undefined Access (no effect) Bit 14\",\"5695\":\"Undefined Access (no effect) Bit 15\",\"5696\":\"KeyedEvent Wait\",\"5697\":\"KeyedEvent Wake\",\"5698\":\"Undefined Access (no effect) Bit 2\",\"5699\":\"Undefined Access (no effect) Bit 3\",\"5700\":\"Undefined Access (no effect) Bit 4\",\"5701\":\"Undefined Access (no effect) Bit 5\",\"5702\":\"Undefined Access (no effect) Bit 6\",\"5703\":\"Undefined Access (no effect) Bit 7\",\"5704\":\"Undefined Access (no effect) Bit 8\",\"5705\":\"Undefined Access (no effect) Bit 9\",\"5706\":\"Undefined Access (no effect) Bit 10\",\"5707\":\"Undefined Access (no effect) Bit 11\",\"5708\":\"Undefined Access (no effect) Bit 12\",\"5709\":\"Undefined Access (no effect) Bit 13\",\"5710\":\"Undefined Access (no effect) Bit 14\",\"5711\":\"Undefined Access (no effect) Bit 15\",\"6656\":\"Enumerate desktops\",\"6657\":\"Read attributes\",\"6658\":\"Access Clipboard\",\"6659\":\"Create desktop\",\"6660\":\"Write attributes\",\"6661\":\"Access global atoms\",\"6662\":\"Exit windows\",\"6663\":\"Unused Access Flag\",\"6664\":\"Include this windowstation in enumerations\",\"6665\":\"Read screen\",\"6672\":\"Read Objects\",\"6673\":\"Create window\",\"6674\":\"Create menu\",\"6675\":\"Hook control\",\"6676\":\"Journal (record)\",\"6677\":\"Journal (playback)\",\"6678\":\"Include this desktop in enumerations\",\"6679\":\"Write objects\",\"6680\":\"Switch to this desktop\",\"6912\":\"Administer print server\",\"6913\":\"Enumerate printers\",\"6930\":\"Full Control\",\"6931\":\"Print\",\"6948\":\"Administer Document\",\"7168\":\"Connect to service controller\",\"7169\":\"Create a new service\",\"7170\":\"Enumerate services\",\"7171\":\"Lock service database for exclusive access\",\"7172\":\"Query service database lock state\",\"7173\":\"Set last-known-good state of service database\",\"7184\":\"Query service configuration information\",\"7185\":\"Set service configuration information\",\"7186\":\"Query status of service\",\"7187\":\"Enumerate dependencies of service\",\"7188\":\"Start the service\",\"7189\":\"Stop the service\",\"7190\":\"Pause or continue the service\",\"7191\":\"Query information from service\",\"7192\":\"Issue service-specific control commands\",\"7424\":\"DDE Share Read\",\"7425\":\"DDE Share Write\",\"7426\":\"DDE Share Initiate Static\",\"7427\":\"DDE Share Initiate Link\",\"7428\":\"DDE Share Request\",\"7429\":\"DDE Share Advise\",\"7430\":\"DDE Share Poke\",\"7431\":\"DDE Share Execute\",\"7432\":\"DDE Share Add Items\",\"7433\":\"DDE Share List Items\",\"7680\":\"Create Child\",\"7681\":\"Delete Child\",\"7682\":\"List Contents\",\"7683\":\"Write Self\",\"7684\":\"Read Property\",\"7685\":\"Write Property\",\"7686\":\"Delete Tree\",\"7687\":\"List Object\",\"7688\":\"Control Access\",\"7689\":\"Undefined Access (no effect) Bit 9\",\"7690\":\"Undefined Access (no effect) Bit 10\",\"7691\":\"Undefined Access (no effect) Bit 11\",\"7692\":\"Undefined Access (no effect) Bit 12\",\"7693\":\"Undefined Access (no effect) Bit 13\",\"7694\":\"Undefined Access (no effect) Bit 14\",\"7695\":\"Undefined Access (no effect) Bit 15\",\"7936\":\"Audit Set System Policy\",\"7937\":\"Audit Query System Policy\",\"7938\":\"Audit Set Per User Policy\",\"7939\":\"Audit Query Per User Policy\",\"7940\":\"Audit Enumerate Users\",\"7941\":\"Audit Set Options\",\"7942\":\"Audit Query Options\",\"8064\":\"Port sharing (read)\",\"8065\":\"Port sharing (write)\",\"8096\":\"Default credentials\",\"8097\":\"Credentials manager\",\"8098\":\"Fresh credentials\",\"8192\":\"Kerberos\",\"8193\":\"Preshared key\",\"8194\":\"Unknown authentication\",\"8195\":\"DES\",\"8196\":\"3DES\",\"8197\":\"MD5\",\"8198\":\"SHA1\",\"8199\":\"Local computer\",\"8200\":\"Remote computer\",\"8201\":\"No state\",\"8202\":\"Sent first (SA) payload\",\"8203\":\"Sent second (KE) payload\",\"8204\":\"Sent third (ID) payload\",\"8205\":\"Initiator\",\"8206\":\"Responder\",\"8207\":\"No state\",\"8208\":\"Sent first (SA) payload\",\"8209\":\"Sent final payload\",\"8210\":\"Complete\",\"8211\":\"Unknown\",\"8212\":\"Transport\",\"8213\":\"Tunnel\",\"8214\":\"IKE/AuthIP DoS prevention mode started\",\"8215\":\"IKE/AuthIP DoS prevention mode stopped\",\"8216\":\"Enabled\",\"8217\":\"Not enabled\",\"8218\":\"No state\",\"8219\":\"Sent first (EM attributes) payload\",\"8220\":\"Sent second (SSPI) payload\",\"8221\":\"Sent third (hash) payload\",\"8222\":\"IKEv1\",\"8223\":\"AuthIP\",\"8224\":\"Anonymous\",\"8225\":\"NTLM V2\",\"8226\":\"CGA\",\"8227\":\"Certificate\",\"8228\":\"SSL\",\"8229\":\"None\",\"8230\":\"DH group 1\",\"8231\":\"DH group 2\",\"8232\":\"DH group 14\",\"8233\":\"DH group ECP 256\",\"8234\":\"DH group ECP 384\",\"8235\":\"AES-128\",\"8236\":\"AES-192\",\"8237\":\"AES-256\",\"8238\":\"Certificate ECDSA P256\",\"8239\":\"Certificate ECDSA P384\",\"8240\":\"SSL ECDSA P256\",\"8241\":\"SSL ECDSA P384\",\"8242\":\"SHA 256\",\"8243\":\"SHA 384\",\"8244\":\"IKEv2\",\"8245\":\"EAP payload sent\",\"8246\":\"Authentication payload sent\",\"8247\":\"EAP\",\"8248\":\"DH group 24\",\"8272\":\"System\",\"8273\":\"Logon/Logoff\",\"8274\":\"Object Access\",\"8275\":\"Privilege Use\",\"8276\":\"Detailed Tracking\",\"8277\":\"Policy Change\",\"8278\":\"Account Management\",\"8279\":\"DS Access\",\"8280\":\"Account Logon\",\"8448\":\"Success removed\",\"8449\":\"Success Added\",\"8450\":\"Failure removed\",\"8451\":\"Failure Added\",\"8452\":\"Success include removed\",\"8453\":\"Success include added\",\"8454\":\"Success exclude removed\",\"8455\":\"Success exclude added\",\"8456\":\"Failure include removed\",\"8457\":\"Failure include added\",\"8458\":\"Failure exclude removed\",\"8459\":\"Failure exclude added\"},\"reversed_descriptions\":{\"...\":[\"1831\"],\"3DES\":[\"8196\",\"16399\"],\"64-bit Integer\":[\"1820\"],\"<Binary>\":[\"14672\"],\"<Value change auditing for this registry type is not supported>\":[\"1800\"],\"<never>\":[\"1794\"],\"<value changed\":[\"1792\"],\"<value not set>\":[\"1793\"],\"ACCESS_SYS_SEC\":[\"1542\"],\"AES-128\":[\"16400\",\"8235\"],\"AES-192\":[\"8236\",\"16401\"],\"AES-256\":[\"16402\",\"8237\"],\"AES-GCM 128\":[\"16395\"],\"AES-GCM 192\":[\"16396\"],\"AES-GCM 256\":[\"16397\"],\"AES-GMAC 128\":[\"16407\"],\"AES-GMAC 192\":[\"16408\"],\"AES-GMAC 256\":[\"16409\"],\"Access Clipboard\":[\"6658\"],\"Access global atoms\":[\"6661\"],\"Account Disabled\":[\"2080\"],\"Account Enabled\":[\"2048\"],\"Account Locked\":[\"2090\"],\"Account Lockout\":[\"12546\"],\"Account Logon\":[\"8280\"],\"Account Management\":[\"8278\"],\"Account Unlocked\":[\"2058\"],\"Account currently disabled.\":[\"2310\"],\"Account locked out.\":[\"2307\"],\"Account logon time restriction violation.\":[\"2311\"],\"Active Directory Domain Services\":[\"14676\"],\"Active Directory Lightweight Directory Services\":[\"14677\"],\"Add\":[\"16384\"],\"Add context.\":[\"2491\"],\"Add function property.\":[\"2497\"],\"Add function provider.\":[\"2495\"],\"Add function.\":[\"2493\"],\"Add provider.\":[\"2489\"],\"AddMember\":[\"5410\",\"5424\"],\"AdjustDefaultDacl\":[\"4599\"],\"AdjustGroups\":[\"4598\"],\"AdjustPrivileges\":[\"4597\"],\"AdjustSessionID\":[\"4600\"],\"Administer Document\":[\"6948\"],\"Administer audit log attributes\":[\"5641\"],\"Administer print server\":[\"6912\"],\"AdministerServer\":[\"5402\"],\"All\":[\"1797\"],\"An Error occured during Logon.\":[\"2304\"],\"Anonymous\":[\"8224\"],\"AppendData (or AddSubdirectory or CreatePipeInstance)\":[\"4418\"],\"Application Generated\":[\"12806\"],\"Application Group Management\":[\"13828\"],\"Assign Primary Token Privilege\":[\"1603\"],\"Assign a token to the thread\":[\"4567\"],\"Assign process\":[\"5136\"],\"AssignAsPrimary\":[\"4592\"],\"Audit Enumerate Users\":[\"7940\"],\"Audit Policy Change\":[\"13568\"],\"Audit Policy query/set API Operation\":[\"1799\"],\"Audit Query Options\":[\"7942\"],\"Audit Query Per User Policy\":[\"7939\"],\"Audit Query System Policy\":[\"7937\"],\"Audit Set Options\":[\"7941\"],\"Audit Set Per User Policy\":[\"7938\"],\"Audit Set System Policy\":[\"7936\"],\"AuthIP\":[\"8223\"],\"AuthNoEncap Transport\":[\"16416\"],\"Authentication Policy Change\":[\"13569\"],\"Authentication payload sent\":[\"8246\"],\"Authorization Policy Change\":[\"13570\"],\"Auto\":[\"1849\"],\"Backup Privilege\":[\"1617\"],\"Bidirectional\":[\"14595\"],\"Bind Redirect\":[\"14617\"],\"Blob\":[\"1822\"],\"Block\":[\"16389\"],\"Boolean\":[\"1824\"],\"Boot-time\":[\"16386\"],\"CGA\":[\"8226\"],\"Callout\":[\"16391\"],\"Cause thread to directly impersonate another thread\":[\"4568\"],\"Central Policy Staging\":[\"12813\"],\"Certificate\":[\"8227\"],\"Certificate ECDSA P256\":[\"8238\"],\"Certificate ECDSA P384\":[\"8239\"],\"Certification Services\":[\"12805\"],\"Change Hardware Environment Privilege\":[\"1622\"],\"Change Notify (and Traverse) Privilege\":[\"1623\"],\"Change logon capabilities assigned to account\":[\"5683\"],\"Change privileges assigned to account\":[\"5681\"],\"Change quotas assigned to account\":[\"5682\"],\"Change secret value\":[\"5648\"],\"Change system audit requirements\":[\"5640\"],\"Change the Posix ID offset assigned to the accounted domain\":[\"5684\"],\"Change the Posix ID offset assigned to the trusted domain\":[\"5668\"],\"Change the controllers in the trusted domain\":[\"5666\"],\"ChangeGroupMembership\":[\"5450\"],\"ChangePassword (with knowledge of old password)\":[\"5446\"],\"Channel query information\":[\"5122\"],\"Channel read message\":[\"5120\"],\"Channel set information\":[\"5123\"],\"Channel write message\":[\"5121\"],\"Communicate using port\":[\"4464\"],\"Complete\":[\"8210\"],\"Computer Account Management\":[\"13825\"],\"Connect\":[\"14611\"],\"Connect Redirect\":[\"14616\"],\"Connect to service controller\":[\"7168\"],\"ConnectToServer\":[\"5376\"],\"Control Access\":[\"7688\"],\"Control profile\":[\"4496\"],\"Create Child\":[\"7680\"],\"Create Key.\":[\"2481\"],\"Create Link\":[\"4437\"],\"Create Pagefile Privilege\":[\"1615\"],\"Create Permanent Object Privilege\":[\"1616\"],\"Create a new service\":[\"7169\"],\"Create a privilege\":[\"5638\"],\"Create a secret object\":[\"5637\"],\"Create a subprocess of process\":[\"4487\"],\"Create desktop\":[\"6659\"],\"Create instance of object type\":[\"4608\"],\"Create menu\":[\"6674\"],\"Create new thread in process\":[\"4481\"],\"Create object in directory\":[\"4370\"],\"Create special accounts (for assignment of user rights)\":[\"5636\"],\"Create sub-directory\":[\"4371\"],\"Create sub-key\":[\"4434\"],\"Create window\":[\"6673\"],\"CreateDomain\":[\"5379\"],\"CreateGlobalGroup\":[\"5397\"],\"CreateLocalGroup\":[\"5398\"],\"CreateUser\":[\"5396\"],\"Credential Validation\":[\"14336\"],\"Credentials manager\":[\"8097\"],\"DDE Share Add Items\":[\"7432\"],\"DDE Share Advise\":[\"7429\"],\"DDE Share Execute\":[\"7431\"],\"DDE Share Initiate Link\":[\"7427\"],\"DDE Share Initiate Static\":[\"7426\"],\"DDE Share List Items\":[\"7433\"],\"DDE Share Poke\":[\"7430\"],\"DDE Share Read\":[\"7424\"],\"DDE Share Request\":[\"7428\"],\"DDE Share Write\":[\"7425\"],\"DELETE\":[\"1537\"],\"DES\":[\"16398\",\"8195\"],\"DH group 1\":[\"8230\"],\"DH group 14\":[\"8232\"],\"DH group 2\":[\"8231\"],\"DH group 24\":[\"8248\"],\"DH group ECP 256\":[\"8233\"],\"DH group ECP 384\":[\"8234\"],\"DPAPI Activity\":[\"13314\"],\"DS Access\":[\"8279\"],\"Datagram Data\":[\"14600\"],\"Debug Privilege\":[\"1620\"],\"Decrypt.\":[\"2484\"],\"Default\":[\"1846\"],\"Default credentials\":[\"8096\"],\"Delegation\":[\"1840\"],\"Delete\":[\"16385\"],\"Delete Child\":[\"7681\"],\"Delete Key.\":[\"2482\"],\"Delete Tree\":[\"7686\"],\"Delete key file.\":[\"2457\"],\"DeleteChild\":[\"4422\"],\"Denied by\":[\"1802\"],\"Denied by ACE on parent folder\":[\"1812\"],\"Denied by Empty DACL\":[\"1807\"],\"Denied by Integrity Policy check\":[\"1803\"],\"Denied by Process Trust Label ACE\":[\"1841\"],\"Detailed Directory Service Replication\":[\"14083\"],\"Detailed File Share\":[\"12811\"],\"Detailed Tracking\":[\"8276\"],\"Device Access Bit 0\":[\"4352\"],\"Device Access Bit 1\":[\"4353\"],\"Device Access Bit 2\":[\"4354\"],\"Device Access Bit 3\":[\"4355\"],\"Device Access Bit 4\":[\"4356\"],\"Device Access Bit 5\":[\"4357\"],\"Device Access Bit 6\":[\"4358\"],\"Device Access Bit 7\":[\"4359\"],\"Device Access Bit 8\":[\"4360\"],\"Directly impersonate this thread\":[\"4569\"],\"Directory Service Access\":[\"14080\"],\"Directory Service Changes\":[\"14081\"],\"Directory Service Replication\":[\"14082\"],\"Disabled\":[\"1796\"],\"DisallowMmConfig\":[\"1847\"],\"Distribution Group Management\":[\"13827\"],\"Domain settings\":[\"2487\"],\"Domain sid inconsistent.\":[\"2314\"],\"Don't Expire Password' - Disabled\":[\"2057\"],\"Don't Expire Password' - Enabled\":[\"2089\"],\"Don't Require Preauth' - Disabled\":[\"2064\"],\"Don't Require Preauth' - Enabled\":[\"2096\"],\"Duplicate\":[\"4593\"],\"Duplicate handle into or out of process\":[\"4486\"],\"EAP\":[\"8247\"],\"EAP payload sent\":[\"8245\"],\"Enable 64(or 32) bit application to open 32 bit key\":[\"4441\"],\"Enable 64(or 32) bit application to open 64 bit key\":[\"4440\"],\"Enable WMI Account\":[\"16896\"],\"Enable/Disable LSA\":[\"5642\"],\"Enabled\":[\"1795\",\"8216\"],\"Encrypt.\":[\"2483\"],\"Encrypted Text Password Allowed' - Disabled\":[\"2059\"],\"Encrypted Text Password Allowed' - Enabled\":[\"2091\"],\"Endpoint Closure\":[\"14615\"],\"Enumerate dependencies of service\":[\"7187\"],\"Enumerate desktops\":[\"6656\"],\"Enumerate printers\":[\"6913\"],\"Enumerate services\":[\"7170\"],\"Enumerate sub-keys\":[\"4435\"],\"EnumerateDomains\":[\"5380\"],\"Exclude Authorization Information' - Disabled\":[\"2067\"],\"Exclude Authorization Information' - Enabled\":[\"2099\"],\"Execute Method\":[\"16897\"],\"Execute/Traverse\":[\"4421\"],\"Existing registry value modified\":[\"1905\"],\"Exit windows\":[\"6662\"],\"Export of persistent cryptographic key.\":[\"2464\"],\"Extend size\":[\"4516\"],\"FALSE\":[\"1826\"],\"FQBN\":[\"1821\"],\"Failed to unprotect persistent cryptographic key.\":[\"2448\"],\"Failed to zero secret data.\":[\"2438\"],\"Failure Added\":[\"8451\"],\"Failure exclude added\":[\"8459\"],\"Failure exclude removed\":[\"8458\"],\"Failure include added\":[\"8457\"],\"Failure include removed\":[\"8456\"],\"Failure removed\":[\"8450\"],\"File Share\":[\"12808\"],\"File System\":[\"12800\"],\"Filtering Platform Connection\":[\"12810\"],\"Filtering Platform Packet Drop\":[\"12809\"],\"Filtering Platform Policy Change\":[\"13572\"],\"Flow Established\":[\"14612\"],\"Force process termination\":[\"4480\"],\"Force thread termination\":[\"4560\"],\"Forward\":[\"14598\",\"14594\"],\"Fresh credentials\":[\"8098\"],\"Friday\":[\"1925\"],\"Full Control\":[\"6930\"],\"Full Write\":[\"16898\"],\"Get sensitive policy information\":[\"5634\"],\"Get thread context\":[\"4563\"],\"GetLocalGroupMembership\":[\"5399\"],\"Granted by\":[\"1801\"],\"Granted by ACE on parent folder\":[\"1811\"],\"Granted by Central Access Rule\":[\"1813\"],\"Granted by NULL DACL\":[\"1806\"],\"Granted by NULL Security Descriptor\":[\"1808\"],\"Granted by Ownership\":[\"1804\"],\"Granted by parent folder's Central Access Rule\":[\"1815\"],\"Group Membership\":[\"12554\"],\"Handle Manipulation\":[\"12807\"],\"Home Directory Required' - Disabled\":[\"2049\"],\"Home Directory Required' - Enabled\":[\"2081\"],\"Hook control\":[\"6675\"],\"ICMP Echo-Request\":[\"14640\"],\"ICMP Error\":[\"14601\"],\"IKE/AuthIP DoS prevention mode started\":[\"8214\"],\"IKE/AuthIP DoS prevention mode stopped\":[\"8215\"],\"IKEv1\":[\"8222\"],\"IKEv2\":[\"8244\"],\"IP Packet\":[\"14596\"],\"IPsec Driver\":[\"12291\"],\"IPsec Extended Mode\":[\"12550\"],\"IPsec Main Mode\":[\"12547\"],\"IPsec Quick Mode\":[\"12549\"],\"Identification\":[\"1832\"],\"Impersonate\":[\"4594\"],\"Impersonation\":[\"1833\"],\"Import of persistent cryptographic key.\":[\"2465\"],\"Inbound\":[\"14592\"],\"Include this desktop in enumerations\":[\"6678\"],\"Include this windowstation in enumerations\":[\"6664\"],\"Increase Memory Quota Privilege\":[\"1605\"],\"Increment Base Priority Privilege\":[\"1614\"],\"InitializeServer\":[\"5378\"],\"Initiator\":[\"8205\",\"16406\"],\"Interdomain Trust Account' - Disabled\":[\"2054\"],\"Interdomain Trust Account' - Enabled\":[\"2086\"],\"Invalid\":[\"1827\"],\"Issue service-specific control commands\":[\"7192\"],\"Journal (playback)\":[\"6677\"],\"Journal (record)\":[\"6676\"],\"Kerberos\":[\"8192\"],\"Kerberos Authentication Service\":[\"14339\"],\"Kerberos Service Ticket Operations\":[\"14337\"],\"Kernel Object\":[\"12802\"],\"Key Derivation.\":[\"2501\"],\"Key export checks failed.\":[\"2449\"],\"Key failed pair wise consistency check.\":[\"2439\"],\"KeyedEvent Wait\":[\"5696\"],\"KeyedEvent Wake\":[\"5697\"],\"List Contents\":[\"7682\"],\"List Object\":[\"7687\"],\"ListAccounts\":[\"5400\"],\"ListGroups\":[\"5448\"],\"ListMembers\":[\"5412\",\"5426\"],\"Listen\":[\"14609\"],\"Load/Unload Driver Privilege\":[\"1610\"],\"Local computer\":[\"8199\"],\"Local settings\":[\"2488\"],\"Lock Memory Privilege\":[\"1604\"],\"Lock service database for exclusive access\":[\"7171\"],\"Logoff\":[\"12545\"],\"Logon\":[\"12544\"],\"Logon/Logoff\":[\"8273\"],\"Lookup Names/SIDs\":[\"5643\"],\"LookupDomain\":[\"5381\"],\"LookupIDs\":[\"5401\"],\"MAC 802.3\":[\"14602\"],\"MAC Native\":[\"14603\"],\"MAX_ALLOWED\":[\"1543\"],\"MD5\":[\"16392\",\"8197\"],\"MNS Logon Account' - Disabled\":[\"2053\"],\"MNS Logon Account' - Enabled\":[\"2085\"],\"MPSSVC Rule-Level Policy Change\":[\"13571\"],\"Machine key.\":[\"2499\"],\"Map section for execute\":[\"4515\"],\"Map section for read\":[\"4514\"],\"Map section for write\":[\"4513\"],\"Modify State\":[\"4865\"],\"Modify domain trust relationships\":[\"5635\"],\"Modify event state\":[\"4385\"],\"Modify semaphore state\":[\"4529\"],\"Modify timer state\":[\"4577\"],\"Monday\":[\"1921\"],\"NOT Granted by Central Access Rule\":[\"1814\"],\"NOT Granted by parent folder's Central Access Rule\":[\"1816\"],\"NTLM V2\":[\"8225\"],\"Network Policy Server\":[\"12552\"],\"New registry value created\":[\"1904\"],\"No\":[\"14679\",\"1843\"],\"No state\":[\"8207\",\"8218\",\"8201\"],\"Non Sensitive Privilege Use\":[\"13057\"],\"None\":[\"1798\",\"8229\"],\"Normal Account' - Disabled\":[\"2052\"],\"Normal Account' - Enabled\":[\"2084\"],\"Not Available\":[\"1845\"],\"Not Available.\":[\"2432\"],\"Not Delegated' - Disabled\":[\"2062\"],\"Not Delegated' - Enabled\":[\"2094\"],\"Not enabled\":[\"8217\"],\"Not granted\":[\"1805\"],\"Not granted due to missing\":[\"1810\"],\"Not granted to AppContainers\":[\"1830\"],\"Not persistent\":[\"16388\"],\"Not used\":[\"1601\"],\"Notify about changes to keys\":[\"4436\"],\"Object Access\":[\"8274\"],\"Off\":[\"1848\"],\"Open Key.\":[\"2480\"],\"Open key file.\":[\"2456\"],\"Other Account Logon Events\":[\"14338\"],\"Other Account Management Events\":[\"13829\"],\"Other Logon/Logoff Events\":[\"12551\"],\"Other Object Access Events\":[\"12804\"],\"Other Policy Change Events\":[\"13573\"],\"Other Privilege Use Events\":[\"13058\"],\"Other System Events\":[\"12292\"],\"Outbound\":[\"14593\"],\"Partial Write\":[\"16899\"],\"Password Expired' - Disabled\":[\"2065\"],\"Password Expired' - Enabled\":[\"2097\"],\"Password Not Required' - Disabled\":[\"2050\"],\"Password Not Required' - Enabled\":[\"2082\"],\"Pause or continue the service\":[\"7190\"],\"Perform virtual memory operation\":[\"4483\"],\"Permit\":[\"16390\"],\"Persistent\":[\"16387\"],\"Plug and Play Events\":[\"13316\"],\"Policy Change\":[\"8277\"],\"Port sharing (read)\":[\"8064\"],\"Port sharing (write)\":[\"8065\"],\"Preshared key\":[\"8193\"],\"Print\":[\"6931\"],\"Privilege Use\":[\"8275\"],\"Process Creation\":[\"13312\"],\"Process Termination\":[\"13313\"],\"Profile Single Process Privilege\":[\"1613\"],\"Profile System Privilege\":[\"1611\"],\"Protect Kerberos Service Tickets with AES Keys' - Disabled\":[\"2069\"],\"Protect Kerberos Service Tickets with AES Keys' - Enabled\":[\"2101\"],\"Provider Write\":[\"16900\"],\"Publish\":[\"16903\"],\"Query\":[\"4595\"],\"Query Attributes\":[\"5138\"],\"Query State\":[\"4864\"],\"Query account information\":[\"5680\"],\"Query directory\":[\"4368\"],\"Query event state\":[\"4384\"],\"Query information from service\":[\"7191\"],\"Query key value\":[\"4432\"],\"Query mutant state\":[\"4448\"],\"Query process information\":[\"4490\"],\"Query secret value\":[\"5649\"],\"Query section state\":[\"4512\"],\"Query semaphore state\":[\"4528\"],\"Query service configuration information\":[\"7184\"],\"Query service database lock state\":[\"7172\"],\"Query status of service\":[\"7186\"],\"Query the Posix ID offset assigned to the trusted domain\":[\"5667\"],\"Query thread information\":[\"4566\"],\"Query timer state\":[\"4576\"],\"Query trusted domain name/SID\":[\"5664\"],\"QuerySource\":[\"4596\"],\"READ_CONTROL\":[\"1538\"],\"REG_BINARY\":[\"1875\"],\"REG_DWORD\":[\"1876\"],\"REG_DWORD_BIG_ENDIAN\":[\"1877\"],\"REG_EXPAND_SZ\":[\"1874\"],\"REG_FULL_RESOURCE_DESCRIPTOR\":[\"1881\"],\"REG_LINK\":[\"1878\"],\"REG_MULTI_SZ (New lines are replaced with *. A * is replaced with **)\":[\"1879\"],\"REG_NONE\":[\"1872\"],\"REG_QWORD\":[\"1883\"],\"REG_RESOURCE_LIST\":[\"1880\"],\"REG_RESOURCE_REQUIREMENTS_LIST\":[\"1882\"],\"REG_SZ\":[\"1873\"],\"RPC Events\":[\"13315\"],\"Random number generation failed FIPS-140 pre-hash check.\":[\"2437\"],\"Random number generator failure.\":[\"2436\"],\"Read Objects\":[\"6672\"],\"Read Property\":[\"7684\"],\"Read attributes\":[\"6657\"],\"Read from process memory\":[\"4484\"],\"Read persisted key from file.\":[\"2458\"],\"Read screen\":[\"6665\"],\"ReadAccount\":[\"5444\"],\"ReadAttributes\":[\"4423\"],\"ReadData (or ListDirectory)\":[\"4416\"],\"ReadEA\":[\"4419\"],\"ReadGeneralInformation\":[\"5440\"],\"ReadGroupMembership\":[\"5449\"],\"ReadInformation\":[\"5427\",\"5408\"],\"ReadLogon\":[\"5443\"],\"ReadOtherParameters\":[\"5394\"],\"ReadPasswordParameters\":[\"5392\"],\"ReadPreferences\":[\"5441\"],\"Receive/Accept\":[\"14610\"],\"Registry\":[\"12801\"],\"Registry value deleted\":[\"1906\"],\"Remote Access\":[\"16901\"],\"Remote computer\":[\"8200\"],\"Remotely Shut System Down Privilege\":[\"1624\"],\"Removable Storage\":[\"12812\"],\"Remove context.\":[\"2492\"],\"Remove function property.\":[\"2498\"],\"Remove function provider.\":[\"2496\"],\"Remove function.\":[\"2494\"],\"Remove provider.\":[\"2490\"],\"RemoveMember\":[\"5425\",\"5411\"],\"Resource Assignment\":[\"14608\"],\"Resource Release\":[\"14614\"],\"Responder\":[\"16405\",\"8206\"],\"Restore From Backup Privilege\":[\"1618\"],\"Retrieve the controllers in the trusted domain\":[\"5665\"],\"SAM\":[\"12803\"],\"SHA 256\":[\"8242\"],\"SHA 384\":[\"8243\"],\"SHA-1\":[\"16393\"],\"SHA-256\":[\"16394\"],\"SHA1\":[\"8198\"],\"SSL\":[\"8228\"],\"SSL ECDSA P256\":[\"8240\"],\"SSL ECDSA P384\":[\"8241\"],\"SYNCHRONIZE\":[\"1541\"],\"Saturday\":[\"1926\"],\"Secret agreement.\":[\"2486\"],\"Security Group Management\":[\"13826\"],\"Security Privilege\":[\"1608\"],\"Security State Change\":[\"12288\"],\"Security System Extension\":[\"12289\"],\"Send an alert to thread\":[\"4562\"],\"Sensitive Privilege Use\":[\"13056\"],\"Sent final payload\":[\"8209\"],\"Sent first (EM attributes) payload\":[\"8219\"],\"Sent first (SA) payload\":[\"8208\",\"8202\"],\"Sent second (KE) payload\":[\"8203\"],\"Sent second (SSPI) payload\":[\"8220\"],\"Sent third (ID) payload\":[\"8204\"],\"Sent third (hash) payload\":[\"8221\"],\"Server Trust Account' - Disabled\":[\"2056\"],\"Server Trust Account' - Enabled\":[\"2088\"],\"Set Attributes\":[\"5137\"],\"Set Security Attributes\":[\"5140\"],\"Set System Time Privilege\":[\"1612\"],\"Set default quota limits\":[\"5639\"],\"Set key value\":[\"4433\"],\"Set last-known-good state of service database\":[\"7173\"],\"Set process information\":[\"4489\"],\"Set process quotas\":[\"4488\"],\"Set process session ID\":[\"4482\"],\"Set process termination port\":[\"4491\"],\"Set service configuration information\":[\"7185\"],\"Set thread context\":[\"4564\"],\"Set thread information\":[\"4565\"],\"SetPassword (without knowledge of old password)\":[\"5447\"],\"Shutdown System Privilege\":[\"1619\"],\"ShutdownServer\":[\"5377\"],\"Sid\":[\"1823\"],\"Sign hash.\":[\"2485\"],\"Signature verification failed.\":[\"2451\"],\"Smartcard Required' - Disabled\":[\"2060\"],\"Smartcard Required' - Enabled\":[\"2092\"],\"Smartcard logon is required and was not used.\":[\"2315\"],\"Special Logon\":[\"12548\"],\"Start the service\":[\"7188\"],\"Stop the service\":[\"7189\"],\"Stream\":[\"14599\"],\"Stream Packet\":[\"14624\"],\"String\":[\"1818\"],\"Subscribe\":[\"16902\"],\"Success Added\":[\"8449\"],\"Success exclude added\":[\"8455\"],\"Success exclude removed\":[\"8454\"],\"Success include added\":[\"8453\"],\"Success include removed\":[\"8452\"],\"Success removed\":[\"8448\"],\"Sunday\":[\"1920\"],\"Suspend or resume thread\":[\"4561\"],\"Switch to this desktop\":[\"6680\"],\"System\":[\"1844\",\"8272\"],\"System Integrity\":[\"12290\"],\"TRUE\":[\"1825\"],\"Take Ownership Privilege\":[\"1609\"],\"Temp Duplicate Account' - Disabled\":[\"2051\"],\"Temp Duplicate Account' - Enabled\":[\"2083\"],\"Terminate Job\":[\"5139\"],\"The NetLogon component is not active.\":[\"2306\"],\"The specified account's password has expired.\":[\"2309\"],\"The specified user account has expired.\":[\"2305\"],\"The user has not been granted the requested logon type at this machine.\":[\"2308\"],\"Thursday\":[\"1924\"],\"Token Right Adjusted Events\":[\"13317\"],\"TokenElevationTypeDefault (1)\":[\"1936\"],\"TokenElevationTypeFull (2)\":[\"1937\"],\"TokenElevationTypeLimited (3)\":[\"1938\"],\"Transport\":[\"14597\",\"16403\",\"8212\"],\"Traverse\":[\"4369\"],\"Trusted Computer Base Privilege\":[\"1607\"],\"Trusted For Delegation' - Disabled\":[\"2061\"],\"Trusted For Delegation' - Enabled\":[\"2093\"],\"Trusted To Authenticate For Delegation' - Disabled\":[\"2066\"],\"Trusted To Authenticate For Delegation' - Enabled\":[\"2098\"],\"Tuesday\":[\"1922\"],\"Tunnel\":[\"16404\",\"8213\"],\"Undefined Access (no effect) Bit 1\":[\"4609\",\"4545\",\"4497\",\"4465\",\"4449\"],\"Undefined Access (no effect) Bit 10\":[\"4554\",\"4618\",\"4378\",\"5418\",\"4474\",\"7690\",\"5690\",\"4442\",\"4522\",\"4458\",\"4602\",\"5658\",\"5434\",\"5146\",\"5706\",\"4426\",\"5386\",\"4362\",\"4538\",\"4570\",\"4586\",\"5674\",\"4506\",\"4394\",\"5130\"],\"Undefined Access (no effect) Bit 11\":[\"4587\",\"5435\",\"5691\",\"5675\",\"4603\",\"4379\",\"5451\",\"5387\",\"5707\",\"4619\",\"7691\",\"4395\",\"4459\",\"4427\",\"4571\",\"4363\",\"4539\",\"5403\",\"4443\",\"5147\",\"4523\",\"5131\",\"4475\",\"4555\",\"4507\",\"5419\",\"5659\"],\"Undefined Access (no effect) Bit 12\":[\"5660\",\"4364\",\"4620\",\"5708\",\"4540\",\"4428\",\"4524\",\"5148\",\"5420\",\"4508\",\"5404\",\"5452\",\"4380\",\"4460\",\"4604\",\"5436\",\"4492\",\"4396\",\"4556\",\"7692\",\"5676\",\"4588\",\"4476\",\"4572\",\"4444\",\"5132\",\"5692\",\"5388\"],\"Undefined Access (no effect) Bit 13\":[\"5149\",\"5437\",\"4477\",\"5389\",\"4525\",\"4557\",\"5421\",\"4605\",\"4541\",\"4461\",\"5677\",\"5693\",\"4509\",\"4621\",\"4589\",\"4381\",\"5405\",\"4429\",\"4445\",\"4573\",\"5661\",\"4397\",\"5709\",\"4365\",\"5453\",\"7693\",\"4493\",\"5133\"],\"Undefined Access (no effect) Bit 14\":[\"4510\",\"4366\",\"4606\",\"4462\",\"4558\",\"5694\",\"4446\",\"5710\",\"5390\",\"5438\",\"4478\",\"4398\",\"4382\",\"4590\",\"5150\",\"5454\",\"5134\",\"5678\",\"7694\",\"5662\",\"4526\",\"4622\",\"5422\",\"4574\",\"4542\",\"4494\",\"4430\",\"5406\"],\"Undefined Access (no effect) Bit 15\":[\"4399\",\"5679\",\"4447\",\"5391\",\"5407\",\"5135\",\"4559\",\"4591\",\"5663\",\"5439\",\"4511\",\"4431\",\"4495\",\"5151\",\"4607\",\"7695\",\"4623\",\"4575\",\"4543\",\"4479\",\"5455\",\"4367\",\"4383\",\"5695\",\"5423\",\"5711\",\"4527\",\"4463\"],\"Undefined Access (no effect) Bit 2\":[\"4450\",\"4498\",\"4466\",\"5698\",\"4386\",\"5650\",\"4610\",\"4578\",\"4530\",\"4546\"],\"Undefined Access (no effect) Bit 3\":[\"4451\",\"5699\",\"4579\",\"5651\",\"4467\",\"4387\",\"4547\",\"4611\",\"4531\",\"4499\"],\"Undefined Access (no effect) Bit 4\":[\"4372\",\"5652\",\"5124\",\"4468\",\"4580\",\"4548\",\"4500\",\"4452\",\"4532\",\"5700\",\"4612\",\"4388\"],\"Undefined Access (no effect) Bit 5\":[\"5669\",\"5701\",\"5653\",\"4517\",\"4453\",\"4469\",\"4501\",\"5125\",\"4549\",\"4533\",\"4581\",\"5429\",\"5685\",\"4373\",\"5413\",\"4389\",\"4613\",\"5141\"],\"Undefined Access (no effect) Bit 6\":[\"5654\",\"4534\",\"4502\",\"4390\",\"5414\",\"5382\",\"4550\",\"4582\",\"4518\",\"4614\",\"4438\",\"4454\",\"4374\",\"5126\",\"4470\",\"5430\",\"5702\",\"5670\",\"5686\",\"5142\"],\"Undefined Access (no effect) Bit 7\":[\"4519\",\"4455\",\"5143\",\"4375\",\"5703\",\"4471\",\"5383\",\"5415\",\"4391\",\"5687\",\"5431\",\"5655\",\"4551\",\"5127\",\"4503\",\"4439\",\"5671\",\"279\",\"4535\",\"4615\"],\"Undefined Access (no effect) Bit 8\":[\"5144\",\"4376\",\"5656\",\"4552\",\"4472\",\"4504\",\"4456\",\"5128\",\"4392\",\"4616\",\"4536\",\"4584\",\"4520\",\"5432\",\"5384\",\"5672\",\"5416\",\"5704\",\"5688\"],\"Undefined Access (no effect) Bit 9\":[\"5433\",\"5145\",\"4361\",\"4457\",\"4601\",\"4537\",\"4585\",\"4393\",\"4521\",\"5657\",\"5673\",\"4553\",\"7689\",\"5385\",\"4425\",\"4505\",\"4377\",\"5689\",\"5417\",\"5705\",\"4617\",\"5129\",\"4473\"],\"Undefined UserAccountControl Bit 20' - Disabled\":[\"2068\"],\"Undefined UserAccountControl Bit 20' - Enabled\":[\"2100\"],\"Undefined UserAccountControl Bit 22' - Disabled\":[\"2070\"],\"Undefined UserAccountControl Bit 22' - Enabled\":[\"2102\"],\"Undefined UserAccountControl Bit 23' - Disabled\":[\"2071\"],\"Undefined UserAccountControl Bit 23' - Enabled\":[\"2103\"],\"Undefined UserAccountControl Bit 24' - Disabled\":[\"2072\"],\"Undefined UserAccountControl Bit 24' - Enabled\":[\"2104\"],\"Undefined UserAccountControl Bit 25' - Disabled\":[\"2073\"],\"Undefined UserAccountControl Bit 25' - Enabled\":[\"2105\"],\"Undefined UserAccountControl Bit 26' - Disabled\":[\"2074\"],\"Undefined UserAccountControl Bit 26' - Enabled\":[\"2106\"],\"Undefined UserAccountControl Bit 27' - Disabled\":[\"2075\"],\"Undefined UserAccountControl Bit 27' - Enabled\":[\"2107\"],\"Undefined UserAccountControl Bit 28' - Disabled\":[\"2076\"],\"Undefined UserAccountControl Bit 28' - Enabled\":[\"2108\"],\"Undefined UserAccountControl Bit 29' - Disabled\":[\"2077\"],\"Undefined UserAccountControl Bit 29' - Enabled\":[\"2109\"],\"Undefined UserAccountControl Bit 30' - Disabled\":[\"2078\"],\"Undefined UserAccountControl Bit 30' - Enabled\":[\"2110\"],\"Undefined UserAccountControl Bit 31' - Disabled\":[\"2079\"],\"Undefined UserAccountControl Bit 31' - Enabled\":[\"2111\"],\"Unknown\":[\"8211\"],\"Unknown Type\":[\"1817\"],\"Unknown authentication\":[\"8194\"],\"Unknown or unchecked\":[\"1809\"],\"Unknown specific access (bit 0)\":[\"1552\"],\"Unknown specific access (bit 1)\":[\"1553\"],\"Unknown specific access (bit 10)\":[\"1562\"],\"Unknown specific access (bit 11)\":[\"1563\"],\"Unknown specific access (bit 12)\":[\"1564\"],\"Unknown specific access (bit 13)\":[\"1565\"],\"Unknown specific access (bit 14)\":[\"1566\"],\"Unknown specific access (bit 15)\":[\"1567\"],\"Unknown specific access (bit 2)\":[\"1554\"],\"Unknown specific access (bit 3)\":[\"1555\"],\"Unknown specific access (bit 4)\":[\"1556\"],\"Unknown specific access (bit 5)\":[\"1557\"],\"Unknown specific access (bit 6)\":[\"1558\"],\"Unknown specific access (bit 7)\":[\"1559\"],\"Unknown specific access (bit 8)\":[\"1560\"],\"Unknown specific access (bit 9)\":[\"1561\"],\"Unknown user name or bad password.\":[\"2313\"],\"Unsigned 64-bit Integer\":[\"1819\"],\"Unsolicited Input Privilege\":[\"1606\"],\"Unused Access Flag\":[\"6663\"],\"Unused message ID\":[\"1536\"],\"Use DES Key Only' - Disabled\":[\"2063\"],\"Use DES Key Only' - Enabled\":[\"2095\"],\"Use symbolic link\":[\"4544\"],\"User / Device Claims\":[\"12553\"],\"User Account Management\":[\"13824\"],\"User key.\":[\"2500\"],\"User not allowed to logon at this computer.\":[\"2312\"],\"Validation of public key failed.\":[\"2450\"],\"Value Added\":[\"14674\"],\"Value Added With Expiration Time\":[\"14680\"],\"Value Auto Deleted With Expiration Time\":[\"14688\"],\"Value Deleted\":[\"14675\"],\"Value Deleted With Expiration Time\":[\"14681\"],\"View non-sensitive policy information\":[\"5632\"],\"View or Change Audit Log Privilege\":[\"1621\"],\"View system audit requirements\":[\"5633\"],\"WRITE_DAC\":[\"1539\"],\"WRITE_OWNER\":[\"1540\"],\"Wednesday\":[\"1923\"],\"Workstation Trust Account' - Disabled\":[\"2055\"],\"Workstation Trust Account' - Enabled\":[\"2087\"],\"Write Property\":[\"7685\"],\"Write Self\":[\"7683\"],\"Write attributes\":[\"6660\"],\"Write objects\":[\"6679\"],\"Write persisted key to file.\":[\"2459\"],\"Write to process memory\":[\"4485\"],\"WriteAccount\":[\"5409\",\"5445\",\"5428\"],\"WriteAttributes\":[\"4424\"],\"WriteData (or AddFile)\":[\"4417\"],\"WriteEA\":[\"4420\"],\"WriteOtherParameters\":[\"5395\"],\"WritePasswordParameters\":[\"5393\"],\"WritePreferences\":[\"5442\"],\"Yes\":[\"1842\",\"14678\"],\"[NULL]\":[\"14673\"],\"a Security Descriptor too long to display\":[\"1829\"],\"an ACE too long to display\":[\"1828\"],\"vSwitch\":[\"14604\"],\"vSwitch Egress\":[\"14642\"],\"vSwitch Ingress\":[\"14641\"]}}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.winlog?.event_data?.Status == null ||\n    ctx.event?.code == null ||\n    ![\"4625\", \"4776\"].contains(ctx.event.code)) {\n  return;\n}\nif (params.containsKey(ctx.winlog.event_data.Status)) {\n  if (ctx.winlog?.logon == null ) {\n      HashMap hm = new HashMap();\n      ctx.winlog.put(\"logon\", hm);\n  }\n  if (ctx.winlog?.logon?.failure == null) {\n      HashMap hm = new HashMap();\n      ctx.winlog.logon.put(\"failure\", hm);\n  }\n  ctx.winlog.logon.failure.put(\"status\", params[ctx.winlog.event_data.Status]);\n}\nif (ctx.winlog?.event_data?.SubStatus == null || !params.containsKey(ctx.winlog.event_data.SubStatus)) {\n  return;\n}\nif (ctx.winlog?.logon == null ) {\n  HashMap hm = new HashMap();\n  ctx.winlog.put(\"logon\", hm);\n}\nif (ctx.winlog?.logon?.failure == null) {\n  HashMap hm = new HashMap();\n  ctx.winlog.logon.put(\"failure\", hm);\n}\nctx.winlog.logon.failure.put(\"sub_status\", params[ctx.winlog.event_data.SubStatus]);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data?.Status == null ||\n    ctx.event?.code == null ||\n    ![\"4625\", \"4776\"].contains(ctx.event.code)) {\n  return;\n}\nif (params.containsKey(ctx.winlog.event_data.Status)) {\n  if (ctx.winlog?.logon == null ) {\n      HashMap hm = new HashMap();\n      ctx.winlog.put(\"logon\", hm);\n  }\n  if (ctx.winlog?.logon?.failure == null) {\n      HashMap hm = new HashMap();\n      ctx.winlog.logon.put(\"failure\", hm);\n  }\n  ctx.winlog.logon.failure.put(\"status\", params[ctx.winlog.event_data.Status]);\n}\nif (ctx.winlog?.event_data?.SubStatus == null || !params.containsKey(ctx.winlog.event_data.SubStatus)) {\n  return;\n}\nif (ctx.winlog?.logon == null ) {\n  HashMap hm = new HashMap();\n  ctx.winlog.put(\"logon\", hm);\n}\nif (ctx.winlog?.logon?.failure == null) {\n  HashMap hm = new HashMap();\n  ctx.winlog.logon.put(\"failure\", hm);\n}\nctx.winlog.logon.failure.put(\"sub_status\", params[ctx.winlog.event_data.SubStatus]);"#
                        ),
                        cached_params!(
                            "{\"0x0\":\"Status OK.\",\"0xc000005e\":\"There are currently no logon servers available to service the logon request.\",\"0xc0000064\":\"User logon with misspelled or bad user account\",\"0xc000006a\":\"User logon with misspelled or bad password\",\"0xc000006d\":\"This is either due to a bad username or authentication information\",\"0xc000006e\":\"Unknown user name or bad password.\",\"0xc000006f\":\"User logon outside authorized hours\",\"0xc0000070\":\"User logon from unauthorized workstation\",\"0xc0000071\":\"User logon with expired password\",\"0xc0000072\":\"User logon to account disabled by administrator\",\"0xc00000dc\":\"Indicates the Sam Server was in the wrong state to perform the desired operation.\",\"0xc0000133\":\"Clocks between DC and other computer too far out of sync\",\"0xc000015b\":\"The user has not been granted the requested logon type (aka logon right) at this machine\",\"0xc000018c\":\"The logon request failed because the trust relationship between the primary domain and the trusted domain failed.\",\"0xc0000192\":\"An attempt was made to logon, but the Netlogon service was not started.\",\"0xc0000193\":\"User logon with expired account\",\"0xc0000224\":\"User is required to change password at next logon\",\"0xc0000225\":\"Evidently a bug in Windows and not a risk\",\"0xc0000234\":\"User logon with account locked\",\"0xc00002ee\":\"Failure Reason: An Error occurred during Logon\",\"0xc0000371\":\"The local account store does not contain secret material for the specified account\",\"0xc0000413\":\"Logon Failure: The machine you are logging onto is protected by an authentication firewall. The specified account is not allowed to authenticate to the machine.\"}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.winlog?.event_data?.TdoType == null) {\n  return;\n}\nif (!params.containsKey(ctx.winlog.event_data.TdoType)) {\n  return;\n}\nctx.winlog.put(\"trustType\", params[ctx.winlog.event_data.TdoType]);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data?.TdoType == null) {\n  return;\n}\nif (!params.containsKey(ctx.winlog.event_data.TdoType)) {\n  return;\n}\nctx.winlog.put(\"trustType\", params[ctx.winlog.event_data.TdoType]);"#
                        ),
                        cached_params!(
                            "{\"1\":\"TRUST_TYPE_DOWNLEVEL\",\"2\":\"TRUST_TYPE_UPLEVEL\",\"3\":\"TRUST_TYPE_MIT\",\"4\":\"TRUST_TYPE_DCE\"}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.winlog?.event_data?.TdoDirection == null) {\n  return;\n}\nif (!params.containsKey(ctx.winlog.event_data.TdoDirection)) {\n  return;\n}\nctx.winlog.put(\"trustDirection\", params[ctx.winlog.event_data.TdoDirection]);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data?.TdoDirection == null) {\n  return;\n}\nif (!params.containsKey(ctx.winlog.event_data.TdoDirection)) {\n  return;\n}\nctx.winlog.put(\"trustDirection\", params[ctx.winlog.event_data.TdoDirection]);"#
                        ),
                        cached_params!(
                            "{\"0\":\"TRUST_DIRECTION_DISABLED\",\"1\":\"TRUST_DIRECTION_INBOUND\",\"2\":\"TRUST_DIRECTION_OUTBOUND\",\"3\":\"TRUST_DIRECTION_BIDIRECTIONAL\"}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.winlog?.event_data?.TdoAttributes == null) {\n  return;\n}\nif (!params.containsKey(ctx.winlog.event_data.TdoAttributes)) {\n  return;\n}\nctx.winlog.put(\"trustAttribute\", params[ctx.winlog.event_data.TdoAttributes]);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data?.TdoAttributes == null) {\n  return;\n}\nif (!params.containsKey(ctx.winlog.event_data.TdoAttributes)) {\n  return;\n}\nctx.winlog.put(\"trustAttribute\", params[ctx.winlog.event_data.TdoAttributes]);"#
                        ),
                        cached_params!(
                            "{\"0\":\"UNDEFINED\",\"1\":\"TRUST_ATTRIBUTE_NON_TRANSITIVE\",\"1024\":\"TRUST_ATTRIBUTE_PIM_TRUST\",\"128\":\"TRUST_ATTRIBUTE_USES_RC4_ENCRYPTION\",\"16\":\"TRUST_ATTRIBUTE_CROSS_ORGANIZATION\",\"2\":\"TRUST_ATTRIBUTE_UPLEVEL_ONLY\",\"32\":\"TRUST_ATTRIBUTE_WITHIN_FOREST\",\"4\":\"TRUST_ATTRIBUTE_QUARANTINED_DOMAIN\",\"512\":\"TRUST_ATTRIBUTE_CROSS_ORGANIZATION_NO_TGT_DELEGATION\",\"64\":\"TRUST_ATTRIBUTE_TREAT_AS_EXTERNAL\",\"8\":\"TRUST_ATTRIBUTE_FOREST_TRANSITIVE\"}"
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.event?.code == null ||\n    ![\"4778\", \"4779\"].contains(ctx.event.code)) {\n  return;\n}\n//AccountName to user.name and related.user\nif (ctx.winlog?.event_data?.AccountName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  ctx.user.put(\"name\", ctx.winlog.event_data.AccountName);\n  if (!ctx.related.user.contains(ctx.winlog.event_data.AccountName)) {\n    ctx.related.user.add(ctx.winlog.event_data.AccountName);\n  }\n}\n\n//AccountDomain to user.domain\nif (ctx.winlog?.event_data?.AccountDomain != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  ctx.user.put(\"domain\", ctx.winlog.event_data.AccountDomain);\n}\n\n//ClientAddress to source.ip and related.ip\nif (ctx.winlog?.event_data?.ClientAddress != null &&\n    ctx.winlog.event_data.ClientAddress != \"-\" &&\n    ctx.winlog.event_data.ClientAddress != \"Unknown\") {\n  // Correct invalid IP address \"LOCAL\"\n  if (ctx.winlog.event_data.ClientAddress == \"LOCAL\") {\n    ctx.winlog.event_data.ClientAddress = \"127.0.0.1\";\n  }\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.ip == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"ip\", al);\n  }\n  ctx.source.put(\"ip\", ctx.winlog.event_data.ClientAddress);\n  if (!ctx.related.ip.contains(ctx.winlog.event_data.ClientAddress)) {\n    ctx.related.ip.add(ctx.winlog.event_data.ClientAddress);\n  }\n}\n\n//ClientName to source.domain\nif (ctx.winlog?.event_data?.ClientName != null) {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  ctx.source.put(\"domain\", ctx.winlog.event_data.ClientName);\n}\n\n//LogonID to winlog.logon.id\nif (ctx.winlog?.event_data?.LogonID != null) {\n  if (ctx.winlog?.logon == null) {\n    HashMap hm = new HashMap();\n    ctx.winlog.put(\"logon\", hm);\n  }\n  ctx.winlog.logon.put(\"id\", ctx.winlog.event_data.LogonID);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null ||\n    ![\"4778\", \"4779\"].contains(ctx.event.code)) {\n  return;\n}\n//AccountName to user.name and related.user\nif (ctx.winlog?.event_data?.AccountName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  ctx.user.put(\"name\", ctx.winlog.event_data.AccountName);\n  if (!ctx.related.user.contains(ctx.winlog.event_data.AccountName)) {\n    ctx.related.user.add(ctx.winlog.event_data.AccountName);\n  }\n}\n\n//AccountDomain to user.domain\nif (ctx.winlog?.event_data?.AccountDomain != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  ctx.user.put(\"domain\", ctx.winlog.event_data.AccountDomain);\n}\n\n//ClientAddress to source.ip and related.ip\nif (ctx.winlog?.event_data?.ClientAddress != null &&\n    ctx.winlog.event_data.ClientAddress != \"-\" &&\n    ctx.winlog.event_data.ClientAddress != \"Unknown\") {\n  // Correct invalid IP address \"LOCAL\"\n  if (ctx.winlog.event_data.ClientAddress == \"LOCAL\") {\n    ctx.winlog.event_data.ClientAddress = \"127.0.0.1\";\n  }\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.ip == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"ip\", al);\n  }\n  ctx.source.put(\"ip\", ctx.winlog.event_data.ClientAddress);\n  if (!ctx.related.ip.contains(ctx.winlog.event_data.ClientAddress)) {\n    ctx.related.ip.add(ctx.winlog.event_data.ClientAddress);\n  }\n}\n\n//ClientName to source.domain\nif (ctx.winlog?.event_data?.ClientName != null) {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  ctx.source.put(\"domain\", ctx.winlog.event_data.ClientName);\n}\n\n//LogonID to winlog.logon.id\nif (ctx.winlog?.event_data?.LogonID != null) {\n  if (ctx.winlog?.logon == null) {\n    HashMap hm = new HashMap();\n    ctx.winlog.put(\"logon\", hm);\n  }\n  ctx.winlog.logon.put(\"id\", ctx.winlog.event_data.LogonID);\n}"#
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.event?.code == null ||\n    ![\"4624\", \"4625\", \"4634\", \"4647\", \"4648\", \"4768\", \"4769\", \"4770\",\n      \"4771\", \"4776\", \"4964\"].contains(ctx.event.code)) {\n  return;\n}\n\ndef targetUserId = ctx.winlog?.event_data?.TargetUserSid;\nif (targetUserId == null) {\n  targetUserId = ctx.winlog?.event_data?.TargetSid;\n}\n\n//TargetUserSid to user.id or user.target.id\nif (targetUserId != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.user?.id == null) {\n    ctx.user.put(\"id\", targetUserId);\n  } else {\n    if (ctx.user?.target == null) {\n      HashMap hm = new HashMap();\n      ctx.user.put(\"target\", hm);\n    }\n    ctx.user.target.put(\"id\", targetUserId);\n  }\n}\n\n//TargetUserName to related.user and user.name or user.target.name\nif (ctx.winlog?.event_data?.TargetUserName != null) {\n  def tun = ctx.winlog.event_data.TargetUserName.splitOnToken(\"@\");\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.user?.name == null) {\n    ctx.user.put(\"name\", tun[0]);\n  } else {\n    if (ctx.user?.target == null) {\n      HashMap hm = new HashMap();\n      ctx.user.put(\"target\", hm);\n    }\n    ctx.user.target.put(\"name\", tun[0]);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  if (!ctx.related.user.contains(tun[0])) {\n    ctx.related.user.add(tun[0]);\n  }\n}\n//TargetUserDomain to user.domain or user.target.domain\nif (ctx.winlog?.event_data?.TargetDomainName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.user?.domain == null) {\n    ctx.user.put(\"domain\", ctx.winlog.event_data.TargetDomainName);\n  } else {\n    if (ctx.user?.target == null){\n      HashMap hm = new HashMap();\n      ctx.user.put(\"target\", hm);\n    }\n    ctx.user.target.put(\"domain\", ctx.winlog.event_data.TargetDomainName);\n  }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null ||\n    ![\"4624\", \"4625\", \"4634\", \"4647\", \"4648\", \"4768\", \"4769\", \"4770\",\n      \"4771\", \"4776\", \"4964\"].contains(ctx.event.code)) {\n  return;\n}\n\ndef targetUserId = ctx.winlog?.event_data?.TargetUserSid;\nif (targetUserId == null) {\n  targetUserId = ctx.winlog?.event_data?.TargetSid;\n}\n\n//TargetUserSid to user.id or user.target.id\nif (targetUserId != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.user?.id == null) {\n    ctx.user.put(\"id\", targetUserId);\n  } else {\n    if (ctx.user?.target == null) {\n      HashMap hm = new HashMap();\n      ctx.user.put(\"target\", hm);\n    }\n    ctx.user.target.put(\"id\", targetUserId);\n  }\n}\n\n//TargetUserName to related.user and user.name or user.target.name\nif (ctx.winlog?.event_data?.TargetUserName != null) {\n  def tun = ctx.winlog.event_data.TargetUserName.splitOnToken(\"@\");\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.user?.name == null) {\n    ctx.user.put(\"name\", tun[0]);\n  } else {\n    if (ctx.user?.target == null) {\n      HashMap hm = new HashMap();\n      ctx.user.put(\"target\", hm);\n    }\n    ctx.user.target.put(\"name\", tun[0]);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  if (!ctx.related.user.contains(tun[0])) {\n    ctx.related.user.add(tun[0]);\n  }\n}\n//TargetUserDomain to user.domain or user.target.domain\nif (ctx.winlog?.event_data?.TargetDomainName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.user?.domain == null) {\n    ctx.user.put(\"domain\", ctx.winlog.event_data.TargetDomainName);\n  } else {\n    if (ctx.user?.target == null){\n      HashMap hm = new HashMap();\n      ctx.user.put(\"target\", hm);\n    }\n    ctx.user.target.put(\"domain\", ctx.winlog.event_data.TargetDomainName);\n  }\n}"#
                        ),
                    )?;
                    let _cond = { event.has_value("winlog.event_data.MemberName") };
                    if _cond {
                        if event.has_value("winlog.event_data.MemberName") {
                            if let Some(s) = event.get_string("winlog.event_data.MemberName") {
                                let parts: Vec<Value> = cached_regex!("(?<!\\\\),")
                                    .split(&s)
                                    .into_iter()
                                    .map(|p| json!(p))
                                    .collect();
                                event.set("_temp.MemberNameParts", Value::Array(parts))?;
                            }
                        }
                    }
                    // Painless script
                    // Source: if (ctx.event?.code == null ||\n    ![\"4727\", \"4728\", \"4729\", \"4730\", \"4731\", \"4732\", \"4733\", \"4734\", \"4735\",\n      \"4737\", \"4744\", \"4745\", \"4746\", \"4747\", \"4748\", \"4749\", \"4750\", \"4751\",\n      \"4752\", \"4753\", \"4754\", \"4755\", \"4756\", \"4757\", \"4758\", \"4759\", \"4760\",\n      \"4761\", \"4762\", \"4763\", \"4764\", \"4799\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx._temp?.MemberNameParts != null) {\n  def memberNameParts = ctx._temp.MemberNameParts;\n  def memberName = memberNameParts[0].replace(\"CN=\",\"\").replace(\"cn=\",\"\");\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.user?.target == null){\n    HashMap hm = new HashMap();\n    ctx.user.put(\"target\", hm);\n  }\n  ctx.user.target.put(\"name\", memberName);\n  if (!ctx.related.user.contains(memberName)) {\n    ctx.related.user.add(memberName);\n  }\n  if (memberNameParts.length >= 4) {\n    def domain = memberNameParts[3].replace(\"DC=\", \"\").replace(\"dc=\", \"\");\n    ctx.user.target.put(\"domain\", domain);\n  }\n}\nif (ctx.winlog?.event_data?.TargetUserSid != null) {\n  if (ctx.group == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"group\", hm);\n  }\n  ctx.group.put(\"id\", ctx.winlog.event_data.TargetUserSid);\n}\nif (ctx.winlog?.event_data?.TargetSid != null) {\n  if (ctx.group == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"group\", hm);\n  }\n  ctx.group.put(\"id\", ctx.winlog.event_data.TargetSid);\n}\nif (ctx.winlog?.event_data?.TargetUserName != null) {\n  if (ctx.group == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"group\", hm);\n  }\n  ctx.group.put(\"name\", ctx.winlog.event_data.TargetUserName);\n}\nif (ctx.winlog?.event_data?.TargetDomainName != null) {\n  if (ctx.group == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"group\", hm);\n  }\n  def domain = ctx.winlog.event_data.TargetDomainName.replace(\"DC=\", \"\").replace(\"dc=\", \"\");\n  ctx.group.put(\"domain\", domain);\n}\nif (ctx.user?.target != null) {\n  if (ctx.user?.target?.group == null) {\n    HashMap hm = new HashMap();\n    ctx.user.target.put(\"group\", hm);\n  }\n  if (ctx.group?.id != null) {\n    ctx.user.target.group.put(\"id\", ctx.group.id);\n  } \n  if (ctx.group?.name != null) {\n    ctx.user.target.group.put(\"name\", ctx.group.name);\n  } \n  if (ctx.group?.domain != null) {\n    ctx.user.target.group.put(\"domain\", ctx.group.domain);\n  }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null ||\n    ![\"4727\", \"4728\", \"4729\", \"4730\", \"4731\", \"4732\", \"4733\", \"4734\", \"4735\",\n      \"4737\", \"4744\", \"4745\", \"4746\", \"4747\", \"4748\", \"4749\", \"4750\", \"4751\",\n      \"4752\", \"4753\", \"4754\", \"4755\", \"4756\", \"4757\", \"4758\", \"4759\", \"4760\",\n      \"4761\", \"4762\", \"4763\", \"4764\", \"4799\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx._temp?.MemberNameParts != null) {\n  def memberNameParts = ctx._temp.MemberNameParts;\n  def memberName = memberNameParts[0].replace(\"CN=\",\"\").replace(\"cn=\",\"\");\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.user?.target == null){\n    HashMap hm = new HashMap();\n    ctx.user.put(\"target\", hm);\n  }\n  ctx.user.target.put(\"name\", memberName);\n  if (!ctx.related.user.contains(memberName)) {\n    ctx.related.user.add(memberName);\n  }\n  if (memberNameParts.length >= 4) {\n    def domain = memberNameParts[3].replace(\"DC=\", \"\").replace(\"dc=\", \"\");\n    ctx.user.target.put(\"domain\", domain);\n  }\n}\nif (ctx.winlog?.event_data?.TargetUserSid != null) {\n  if (ctx.group == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"group\", hm);\n  }\n  ctx.group.put(\"id\", ctx.winlog.event_data.TargetUserSid);\n}\nif (ctx.winlog?.event_data?.TargetSid != null) {\n  if (ctx.group == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"group\", hm);\n  }\n  ctx.group.put(\"id\", ctx.winlog.event_data.TargetSid);\n}\nif (ctx.winlog?.event_data?.TargetUserName != null) {\n  if (ctx.group == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"group\", hm);\n  }\n  ctx.group.put(\"name\", ctx.winlog.event_data.TargetUserName);\n}\nif (ctx.winlog?.event_data?.TargetDomainName != null) {\n  if (ctx.group == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"group\", hm);\n  }\n  def domain = ctx.winlog.event_data.TargetDomainName.replace(\"DC=\", \"\").replace(\"dc=\", \"\");\n  ctx.group.put(\"domain\", domain);\n}\nif (ctx.user?.target != null) {\n  if (ctx.user?.target?.group == null) {\n    HashMap hm = new HashMap();\n    ctx.user.target.put(\"group\", hm);\n  }\n  if (ctx.group?.id != null) {\n    ctx.user.target.group.put(\"id\", ctx.group.id);\n  } \n  if (ctx.group?.name != null) {\n    ctx.user.target.group.put(\"name\", ctx.group.name);\n  } \n  if (ctx.group?.domain != null) {\n    ctx.user.target.group.put(\"domain\", ctx.group.domain);\n  }\n}"#
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.event?.code == null ||\n    ![\"4741\", \"4742\", \"4743\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.event_data?.TargetSid != null) {\n  if (ctx.winlog?.computerObject == null) {\n    HashMap hm = new HashMap();\n    ctx.winlog.put(\"computerObject\", hm);\n  }\n  ctx.winlog.computerObject.put(\"id\", ctx.winlog.event_data.TargetSid);\n}\nif (ctx.winlog?.event_data?.TargetUserName != null) {\n  if (ctx.winlog?.computerObject == null) {\n    HashMap hm = new HashMap();\n    ctx.winlog.put(\"computerObject\", hm);\n  }\n  ctx.winlog.computerObject.put(\"name\", ctx.winlog.event_data.TargetUserName);\n}\nif (ctx.winlog?.event_data?.TargetDomainName != null) {\n  if (ctx.winlog?.computerObject == null) {\n    HashMap hm = new HashMap();\n    ctx.winlog.put(\"computerObject\", hm);\n  }\n  ctx.winlog.computerObject.put(\"domain\", ctx.winlog.event_data.TargetDomainName);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null ||\n    ![\"4741\", \"4742\", \"4743\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.event_data?.TargetSid != null) {\n  if (ctx.winlog?.computerObject == null) {\n    HashMap hm = new HashMap();\n    ctx.winlog.put(\"computerObject\", hm);\n  }\n  ctx.winlog.computerObject.put(\"id\", ctx.winlog.event_data.TargetSid);\n}\nif (ctx.winlog?.event_data?.TargetUserName != null) {\n  if (ctx.winlog?.computerObject == null) {\n    HashMap hm = new HashMap();\n    ctx.winlog.put(\"computerObject\", hm);\n  }\n  ctx.winlog.computerObject.put(\"name\", ctx.winlog.event_data.TargetUserName);\n}\nif (ctx.winlog?.event_data?.TargetDomainName != null) {\n  if (ctx.winlog?.computerObject == null) {\n    HashMap hm = new HashMap();\n    ctx.winlog.put(\"computerObject\", hm);\n  }\n  ctx.winlog.computerObject.put(\"domain\", ctx.winlog.event_data.TargetDomainName);\n}"#
                        ),
                    )?;
                    let _cond = {
                        event.has_value("event.code")
                            && ["4634", "4647", "4964"]
                                .contains(&event.get_str("event.code").unwrap_or(""))
                    };
                    if _cond {
                        if let Some(v) = event.get("winlog.event_data.TargetLogonId").cloned() {
                            event.set("winlog.logon.id", v)?;
                        }
                    }
                    // Painless script
                    // Source: if (ctx.event?.code == null ||\n    ![\"4648\", \"4657\", \"4662\", \"4670\", \"4672\", \"4673\", \"4674\", \"4688\", \"4689\", \"4697\",\n      \"4698\", \"4699\", \"4700\", \"4701\", \"4702\", \"4706\", \"4707\", \"4713\", \"4716\", \"4717\",\n      \"4718\", \"4719\", \"4720\", \"4722\", \"4723\", \"4724\", \"4725\", \"4726\", \"4727\", \"4728\",\n      \"4729\", \"4730\", \"4731\", \"4732\", \"4733\", \"4734\", \"4735\", \"4737\", \"4738\", \"4739\",\n      \"4740\", \"4741\", \"4742\", \"4743\", \"4744\", \"4745\", \"4746\", \"4747\", \"4748\", \"4749\",\n      \"4750\", \"4751\", \"4752\", \"4753\", \"4754\", \"4755\", \"4756\", \"4757\", \"4758\", \"4759\",\n      \"4760\", \"4761\", \"4762\", \"4763\", \"4764\", \"4767\", \"4781\", \"4797\", \"4798\", \"4799\",\n      \"4817\", \"4904\", \"4905\", \"4907\", \"4912\", \"5136\", \"5140\", \"5145\", \"5379\", \"5380\",\n      \"5381\", \"5382\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.event_data?.SubjectUserSid != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  ctx.user.put(\"id\", ctx.winlog.event_data.SubjectUserSid);\n}\nif (ctx.winlog?.event_data?.SubjectUserName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  ctx.user.put(\"name\", ctx.winlog.event_data.SubjectUserName);\n  if (!ctx.related.user.contains(ctx.winlog.event_data.SubjectUserName)) {\n    ctx.related.user.add(ctx.winlog.event_data.SubjectUserName);\n  }\n}\nif (ctx.winlog?.event_data?.SubjectDomainName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  ctx.user.put(\"domain\", ctx.winlog.event_data.SubjectDomainName);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null ||\n    ![\"4648\", \"4657\", \"4662\", \"4670\", \"4672\", \"4673\", \"4674\", \"4688\", \"4689\", \"4697\",\n      \"4698\", \"4699\", \"4700\", \"4701\", \"4702\", \"4706\", \"4707\", \"4713\", \"4716\", \"4717\",\n      \"4718\", \"4719\", \"4720\", \"4722\", \"4723\", \"4724\", \"4725\", \"4726\", \"4727\", \"4728\",\n      \"4729\", \"4730\", \"4731\", \"4732\", \"4733\", \"4734\", \"4735\", \"4737\", \"4738\", \"4739\",\n      \"4740\", \"4741\", \"4742\", \"4743\", \"4744\", \"4745\", \"4746\", \"4747\", \"4748\", \"4749\",\n      \"4750\", \"4751\", \"4752\", \"4753\", \"4754\", \"4755\", \"4756\", \"4757\", \"4758\", \"4759\",\n      \"4760\", \"4761\", \"4762\", \"4763\", \"4764\", \"4767\", \"4781\", \"4797\", \"4798\", \"4799\",\n      \"4817\", \"4904\", \"4905\", \"4907\", \"4912\", \"5136\", \"5140\", \"5145\", \"5379\", \"5380\",\n      \"5381\", \"5382\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.event_data?.SubjectUserSid != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  ctx.user.put(\"id\", ctx.winlog.event_data.SubjectUserSid);\n}\nif (ctx.winlog?.event_data?.SubjectUserName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  ctx.user.put(\"name\", ctx.winlog.event_data.SubjectUserName);\n  if (!ctx.related.user.contains(ctx.winlog.event_data.SubjectUserName)) {\n    ctx.related.user.add(ctx.winlog.event_data.SubjectUserName);\n  }\n}\nif (ctx.winlog?.event_data?.SubjectDomainName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  ctx.user.put(\"domain\", ctx.winlog.event_data.SubjectDomainName);\n}"#
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.event?.code == null ||\n    ![\"4670\", \"4720\", \"4722\", \"4723\", \"4724\", \"4725\",\n      \"4726\", \"4738\", \"4740\", \"4767\", \"4798\", \"4817\",\n      \"4907\", \"4797\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n}\nif (ctx.user?.target == null) {\n    HashMap hm = new HashMap();\n    ctx.user.put(\"target\", hm);\n}\ndef userId = ctx.winlog?.event_data?.TargetSid;\nif (userId != null && userId != \"\" && userId != \"-\") ctx.user.target.id = userId;\ndef userName = ctx.winlog?.event_data?.TargetUserName;\nif (userName != null && userName != \"\" && userName != \"-\") {\n  ctx.user.target.name = userName;\n  def parts = userName.splitOnToken(\"@\");\n  if (parts.length > 1) {\n    ctx.user.target.name = parts[0];\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  if (!ctx.related.user.contains(ctx.user.target.name)) {\n    ctx.related.user.add(ctx.user.target.name);\n  }\n}\ndef userDomain = ctx.winlog?.event_data?.TargetDomainName;\nif (userDomain != null && userDomain != \"\" && userDomain != \"-\") ctx.user.target.domain = userDomain;\nif (ctx.user?.target != null && ctx.user.target.size() == 0) ctx.user.remove(\"target\");
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null ||\n    ![\"4670\", \"4720\", \"4722\", \"4723\", \"4724\", \"4725\",\n      \"4726\", \"4738\", \"4740\", \"4767\", \"4798\", \"4817\",\n      \"4907\", \"4797\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n}\nif (ctx.user?.target == null) {\n    HashMap hm = new HashMap();\n    ctx.user.put(\"target\", hm);\n}\ndef userId = ctx.winlog?.event_data?.TargetSid;\nif (userId != null && userId != \"\" && userId != \"-\") ctx.user.target.id = userId;\ndef userName = ctx.winlog?.event_data?.TargetUserName;\nif (userName != null && userName != \"\" && userName != \"-\") {\n  ctx.user.target.name = userName;\n  def parts = userName.splitOnToken(\"@\");\n  if (parts.length > 1) {\n    ctx.user.target.name = parts[0];\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  if (!ctx.related.user.contains(ctx.user.target.name)) {\n    ctx.related.user.add(ctx.user.target.name);\n  }\n}\ndef userDomain = ctx.winlog?.event_data?.TargetDomainName;\nif (userDomain != null && userDomain != \"\" && userDomain != \"-\") ctx.user.target.domain = userDomain;\nif (ctx.user?.target != null && ctx.user.target.size() == 0) ctx.user.remove(\"target\");"#
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.event?.code == null ||\n    ![\"4648\", \"4688\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n}\nif (ctx.user?.effective == null) {\n    HashMap hm = new HashMap();\n    ctx.user.put(\"effective\", hm);\n}\ndef userId = ctx.winlog?.event_data?.TargetUserSid;\nif (userId != null && userId != \"\" && userId != \"-\") ctx.user.effective.id = userId;\ndef userName = ctx.winlog?.event_data?.TargetUserName;\nif (userName != null && userName != \"\" && userName != \"-\") {\n  ctx.user.effective.name = userName;\n  def parts = userName.splitOnToken(\"@\");\n  if (parts.length > 1) {\n    ctx.user.effective.name = parts[0];\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  if (!ctx.related.user.contains(ctx.user.effective.name)) {\n    ctx.related.user.add(ctx.user.effective.name);\n  }\n}\ndef userDomain = ctx.winlog?.event_data?.TargetDomainName;\nif (userDomain != null && userDomain != \"\" && userDomain != \"-\") ctx.user.effective.domain = userDomain;\nif (ctx.user?.effective != null && ctx.user.effective.size() == 0) ctx.user.remove(\"effective\");
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null ||\n    ![\"4648\", \"4688\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n}\nif (ctx.user?.effective == null) {\n    HashMap hm = new HashMap();\n    ctx.user.put(\"effective\", hm);\n}\ndef userId = ctx.winlog?.event_data?.TargetUserSid;\nif (userId != null && userId != \"\" && userId != \"-\") ctx.user.effective.id = userId;\ndef userName = ctx.winlog?.event_data?.TargetUserName;\nif (userName != null && userName != \"\" && userName != \"-\") {\n  ctx.user.effective.name = userName;\n  def parts = userName.splitOnToken(\"@\");\n  if (parts.length > 1) {\n    ctx.user.effective.name = parts[0];\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  if (!ctx.related.user.contains(ctx.user.effective.name)) {\n    ctx.related.user.add(ctx.user.effective.name);\n  }\n}\ndef userDomain = ctx.winlog?.event_data?.TargetDomainName;\nif (userDomain != null && userDomain != \"\" && userDomain != \"-\") ctx.user.effective.domain = userDomain;\nif (ctx.user?.effective != null && ctx.user.effective.size() == 0) ctx.user.remove(\"effective\");"#
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.event?.code == null ||\n    ![\"1102\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.user_data?.SubjectUserSid != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  ctx.user.put(\"id\", ctx.winlog.user_data.SubjectUserSid);\n}\nif (ctx.winlog?.user_data?.SubjectUserName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  ctx.user.put(\"name\", ctx.winlog.user_data.SubjectUserName);\n  if (!ctx.related.user.contains(ctx.winlog.user_data.SubjectUserName)) {\n    ctx.related.user.add(ctx.winlog.user_data.SubjectUserName);\n  }\n}\nif (ctx.winlog?.user_data?.SubjectDomainName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  ctx.user.put(\"domain\", ctx.winlog.user_data.SubjectDomainName);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null ||\n    ![\"1102\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.user_data?.SubjectUserSid != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  ctx.user.put(\"id\", ctx.winlog.user_data.SubjectUserSid);\n}\nif (ctx.winlog?.user_data?.SubjectUserName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  ctx.user.put(\"name\", ctx.winlog.user_data.SubjectUserName);\n  if (!ctx.related.user.contains(ctx.winlog.user_data.SubjectUserName)) {\n    ctx.related.user.add(ctx.winlog.user_data.SubjectUserName);\n  }\n}\nif (ctx.winlog?.user_data?.SubjectDomainName != null) {\n  if (ctx.user == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"user\", hm);\n  }\n  ctx.user.put(\"domain\", ctx.winlog.user_data.SubjectDomainName);\n}"#
                        ),
                    )?;
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event.get("winlog.event_data.SubjectLogonId").cloned() {
                            event.set("winlog.logon.id", v)?;
                        }
                        Ok(())
                    })();
                    let _cond = {
                        event.has_value("event.code")
                            && ["1102"].contains(&event.get_str("event.code").unwrap_or(""))
                    };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if let Some(v) = event.get("winlog.user_data.SubjectLogonId").cloned() {
                                event.set("winlog.logon.id", v)?;
                            }
                            Ok(())
                        })();
                    }
                    // Painless script
                    // Source: if (ctx.event?.code == null ||\n    ![\"1100\", \"1102\", \"1104\", \"1105\", \"1108\", \"4624\", \"4648\", \"4625\",\n      \"4670\", \"4673\", \"4674\", \"4689\", \"4697\", \"4719\", \"4720\", \"4722\",\n      \"4723\", \"4724\", \"4725\", \"4726\", \"4727\", \"4728\", \"4729\", \"4730\",\n      \"4731\", \"4732\", \"4733\", \"4734\", \"4735\", \"4737\", \"4738\", \"4740\",\n      \"4741\", \"4742\", \"4743\", \"4744\", \"4745\", \"4746\", \"4747\", \"4748\",\n      \"4749\", \"4750\", \"4751\", \"4752\", \"4753\", \"4754\", \"4755\", \"4756\",\n      \"4757\", \"4758\", \"4759\", \"4760\", \"4761\", \"4762\", \"4763\", \"4764\",\n      \"4767\", \"4768\", \"4769\", \"4770\", \"4771\", \"4798\", \"4799\", \"4817\",\n      \"4904\", \"4905\", \"4907\", \"4912\", \"5140\", \"5145\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.event_data?.ProcessId != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  if (ctx.winlog.event_data.ProcessId instanceof String) {\n    Long pid = Long.decode(ctx.winlog.event_data.ProcessId);\n    ctx.process.put(\"pid\", pid.longValue());\n  } else {\n    ctx.process.put(\"pid\", ctx.winlog.event_data.ProcessId);\n  }\n  ctx.winlog.event_data.remove(\"ProcessId\");\n}\nif (ctx.winlog?.event_data?.ProcessName != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  ctx.process.put(\"executable\", ctx.winlog.event_data.ProcessName);\n  ctx.winlog.event_data.remove(\"ProcessName\");\n}\nif (ctx.winlog?.event_data?.IpAddress != null &&\n    ctx.winlog.event_data.IpAddress != \"-\") {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  ctx.source.put(\"ip\", ctx.winlog.event_data.IpAddress);\n  ctx.winlog.event_data.remove(\"IpAddress\");\n}\nif (ctx.winlog?.event_data?.IpPort != null && ctx.winlog.event_data.IpPort != \"-\") {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  ctx.source.put(\"port\", Long.decode(ctx.winlog.event_data.IpPort));\n  ctx.winlog.event_data.remove(\"IpPort\");\n}\nif (ctx.winlog?.event_data?.WorkstationName != null) {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  ctx.source.put(\"domain\", ctx.winlog.event_data.WorkstationName);\n  ctx.winlog.event_data.remove(\"WorkstationName\");\n}\nif (ctx.winlog?.event_data?.ClientAddress != null &&\n    ctx.winlog.event_data.ClientAddress != \"-\" &&\n    ctx.winlog.event_data.ClientAddress != \"Unknown\") {\n  // Correct invalid IP address \"LOCAL\"\n  if (ctx.winlog.event_data.ClientAddress == \"LOCAL\") {\n    ctx.winlog.event_data.ClientAddress = \"127.0.0.1\";\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  ctx.related.put(\"ip\", ctx.winlog.event_data.ClientAddress);\n  ctx.winlog.event_data.remove(\"ClientAddress\");\n}\nif (ctx.process?.name == null && ctx.process?.executable != null) {\n  def parts = ctx.process.executable.splitOnToken(\"\\\\\");\n  ctx.process.put(\"name\", parts[-1]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null ||\n    ![\"1100\", \"1102\", \"1104\", \"1105\", \"1108\", \"4624\", \"4648\", \"4625\",\n      \"4670\", \"4673\", \"4674\", \"4689\", \"4697\", \"4719\", \"4720\", \"4722\",\n      \"4723\", \"4724\", \"4725\", \"4726\", \"4727\", \"4728\", \"4729\", \"4730\",\n      \"4731\", \"4732\", \"4733\", \"4734\", \"4735\", \"4737\", \"4738\", \"4740\",\n      \"4741\", \"4742\", \"4743\", \"4744\", \"4745\", \"4746\", \"4747\", \"4748\",\n      \"4749\", \"4750\", \"4751\", \"4752\", \"4753\", \"4754\", \"4755\", \"4756\",\n      \"4757\", \"4758\", \"4759\", \"4760\", \"4761\", \"4762\", \"4763\", \"4764\",\n      \"4767\", \"4768\", \"4769\", \"4770\", \"4771\", \"4798\", \"4799\", \"4817\",\n      \"4904\", \"4905\", \"4907\", \"4912\", \"5140\", \"5145\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.event_data?.ProcessId != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  if (ctx.winlog.event_data.ProcessId instanceof String) {\n    Long pid = Long.decode(ctx.winlog.event_data.ProcessId);\n    ctx.process.put(\"pid\", pid.longValue());\n  } else {\n    ctx.process.put(\"pid\", ctx.winlog.event_data.ProcessId);\n  }\n  ctx.winlog.event_data.remove(\"ProcessId\");\n}\nif (ctx.winlog?.event_data?.ProcessName != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  ctx.process.put(\"executable\", ctx.winlog.event_data.ProcessName);\n  ctx.winlog.event_data.remove(\"ProcessName\");\n}\nif (ctx.winlog?.event_data?.IpAddress != null &&\n    ctx.winlog.event_data.IpAddress != \"-\") {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  ctx.source.put(\"ip\", ctx.winlog.event_data.IpAddress);\n  ctx.winlog.event_data.remove(\"IpAddress\");\n}\nif (ctx.winlog?.event_data?.IpPort != null && ctx.winlog.event_data.IpPort != \"-\") {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  ctx.source.put(\"port\", Long.decode(ctx.winlog.event_data.IpPort));\n  ctx.winlog.event_data.remove(\"IpPort\");\n}\nif (ctx.winlog?.event_data?.WorkstationName != null) {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  ctx.source.put(\"domain\", ctx.winlog.event_data.WorkstationName);\n  ctx.winlog.event_data.remove(\"WorkstationName\");\n}\nif (ctx.winlog?.event_data?.ClientAddress != null &&\n    ctx.winlog.event_data.ClientAddress != \"-\" &&\n    ctx.winlog.event_data.ClientAddress != \"Unknown\") {\n  // Correct invalid IP address \"LOCAL\"\n  if (ctx.winlog.event_data.ClientAddress == \"LOCAL\") {\n    ctx.winlog.event_data.ClientAddress = \"127.0.0.1\";\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  ctx.related.put(\"ip\", ctx.winlog.event_data.ClientAddress);\n  ctx.winlog.event_data.remove(\"ClientAddress\");\n}\nif (ctx.process?.name == null && ctx.process?.executable != null) {\n  def parts = ctx.process.executable.splitOnToken(\"\\\\\");\n  ctx.process.put(\"name\", parts[-1]);\n}"#
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.event?.code == null ||\n    ![\"5152\", \"5156\", \"5157\", \"5158\"].contains(ctx.event.code)) {\n  return;\n}\n\n// DestAddress to destination.ip and related.ip\nif (ctx.winlog?.event_data?.DestAddress != null &&\n    ctx.winlog.event_data.DestAddress != \"-\") {\n  if (ctx.destination == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"destination\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.ip == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"ip\", al);\n  }\n  ctx.destination.put(\"ip\", ctx.winlog.event_data.DestAddress);\n  if (!ctx.related.ip.contains(ctx.winlog.event_data.DestAddress)) {\n    ctx.related.ip.add(ctx.winlog.event_data.DestAddress);\n  }\n  ctx.winlog.event_data.remove(\"DestAddress\");\n}\n\n// SourceAddress to source.ip and related.ip\nif (ctx.winlog?.event_data?.SourceAddress != null &&\n    ctx.winlog.event_data.SourceAddress != \"-\") {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.ip == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"ip\", al);\n  }\n  ctx.source.put(\"ip\", ctx.winlog.event_data.SourceAddress);\n  if (!ctx.related.ip.contains(ctx.winlog.event_data.SourceAddress)) {\n    ctx.related.ip.add(ctx.winlog.event_data.SourceAddress);\n  }\n  ctx.winlog.event_data.remove(\"SourceAddress\");\n}\n// DestPort to destination.port\nif (ctx.winlog?.event_data?.DestPort != null && ctx.winlog.event_data.DestPort != \"-\") {\n  if (ctx.destination == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"destination\", hm);\n  }\n  ctx.destination.put(\"port\", Long.decode(ctx.winlog.event_data.DestPort));\n  ctx.winlog.event_data.remove(\"DestPort\");\n}\n// SourcePort to source.port\nif (ctx.winlog?.event_data?.SourcePort != null && ctx.winlog.event_data.SourcePort != \"-\") {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  ctx.source.put(\"port\", Long.decode(ctx.winlog.event_data.SourcePort));\n  ctx.winlog.event_data.remove(\"SourcePort\");\n}\n// Protocol to network.iana_number of type keyword\nif (ctx.winlog?.event_data?.Protocol != null && ctx.winlog.event_data.Protocol != \"-\") {\n  if (ctx.network == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"network\", hm);\n  }\n  ctx.network.put(\"iana_number\", ctx.winlog.event_data.Protocol);\n}\n// Application to process.executable and process.name\nif (ctx.winlog?.event_data?.Application != null && ctx.winlog?.event_data?.Application != \"-\") {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  ctx.process.put(\"executable\", ctx.winlog.event_data.Application);\n  ctx.winlog.event_data.remove(\"Application\");\n}\nif (ctx.process?.name == null && ctx.process?.executable != null) {\n  def parts = ctx.process.executable.splitOnToken(\"\\\\\");\n  ctx.process.put(\"name\", parts[-1]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null ||\n    ![\"5152\", \"5156\", \"5157\", \"5158\"].contains(ctx.event.code)) {\n  return;\n}\n\n// DestAddress to destination.ip and related.ip\nif (ctx.winlog?.event_data?.DestAddress != null &&\n    ctx.winlog.event_data.DestAddress != \"-\") {\n  if (ctx.destination == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"destination\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.ip == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"ip\", al);\n  }\n  ctx.destination.put(\"ip\", ctx.winlog.event_data.DestAddress);\n  if (!ctx.related.ip.contains(ctx.winlog.event_data.DestAddress)) {\n    ctx.related.ip.add(ctx.winlog.event_data.DestAddress);\n  }\n  ctx.winlog.event_data.remove(\"DestAddress\");\n}\n\n// SourceAddress to source.ip and related.ip\nif (ctx.winlog?.event_data?.SourceAddress != null &&\n    ctx.winlog.event_data.SourceAddress != \"-\") {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.ip == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"ip\", al);\n  }\n  ctx.source.put(\"ip\", ctx.winlog.event_data.SourceAddress);\n  if (!ctx.related.ip.contains(ctx.winlog.event_data.SourceAddress)) {\n    ctx.related.ip.add(ctx.winlog.event_data.SourceAddress);\n  }\n  ctx.winlog.event_data.remove(\"SourceAddress\");\n}\n// DestPort to destination.port\nif (ctx.winlog?.event_data?.DestPort != null && ctx.winlog.event_data.DestPort != \"-\") {\n  if (ctx.destination == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"destination\", hm);\n  }\n  ctx.destination.put(\"port\", Long.decode(ctx.winlog.event_data.DestPort));\n  ctx.winlog.event_data.remove(\"DestPort\");\n}\n// SourcePort to source.port\nif (ctx.winlog?.event_data?.SourcePort != null && ctx.winlog.event_data.SourcePort != \"-\") {\n  if (ctx.source == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"source\", hm);\n  }\n  ctx.source.put(\"port\", Long.decode(ctx.winlog.event_data.SourcePort));\n  ctx.winlog.event_data.remove(\"SourcePort\");\n}\n// Protocol to network.iana_number of type keyword\nif (ctx.winlog?.event_data?.Protocol != null && ctx.winlog.event_data.Protocol != \"-\") {\n  if (ctx.network == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"network\", hm);\n  }\n  ctx.network.put(\"iana_number\", ctx.winlog.event_data.Protocol);\n}\n// Application to process.executable and process.name\nif (ctx.winlog?.event_data?.Application != null && ctx.winlog?.event_data?.Application != \"-\") {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  ctx.process.put(\"executable\", ctx.winlog.event_data.Application);\n  ctx.winlog.event_data.remove(\"Application\");\n}\nif (ctx.process?.name == null && ctx.process?.executable != null) {\n  def parts = ctx.process.executable.splitOnToken(\"\\\\\");\n  ctx.process.put(\"name\", parts[-1]);\n}"#
                        ),
                    )?;
                    // Painless script
                    // Source: if (ctx.event?.code == null ||\n    ![\"4688\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.event_data?.NewProcessId != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  if (ctx.winlog.event_data.NewProcessId instanceof String) {\n    Long pid = Long.decode(ctx.winlog.event_data.NewProcessId);\n    ctx.process.put(\"pid\", pid.longValue());\n  } else {\n    ctx.process.put(\"pid\", ctx.winlog.event_data.NewProcessId);\n  }\n  ctx.winlog.event_data.remove(\"NewProcessId\");\n}\nif (ctx.winlog?.event_data?.NewProcessName != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  ctx.process.put(\"executable\", ctx.winlog.event_data.NewProcessName);\n  ctx.winlog.event_data.remove(\"NewProcessName\");\n}\nif (ctx.winlog?.event_data?.ParentProcessName != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  if (ctx.process?.parent == null) {\n    HashMap hm = new HashMap();\n    ctx.process.put(\"parent\", hm);\n  }\n  ctx.process.parent.put(\"executable\", ctx.winlog.event_data.ParentProcessName);\n  ctx.winlog.event_data.remove(\"ParentProcessName\");\n}\nif (ctx.process?.name == null && ctx.process?.executable != null) {\n  def parts = ctx.process.executable.splitOnToken(\"\\\\\");\n  ctx.process.put(\"name\", parts[-1]);\n}\nif (ctx.process?.parent?.name == null && ctx.process?.parent?.executable != null) {\n  def parts = ctx.process.parent.executable.splitOnToken(\"\\\\\");\n  ctx.process.parent.put(\"name\", parts[-1]);\n}\nif (ctx.winlog?.event_data?.ProcessId != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  if (ctx.process?.parent == null) {\n    HashMap hm = new HashMap();\n    ctx.process.put(\"parent\", hm);\n  }\n  if (ctx.winlog.event_data.ProcessId instanceof String) {\n    Long pid = Long.decode(ctx.winlog.event_data.ProcessId);\n    ctx.process.parent.put(\"pid\", pid.longValue());\n  } else {\n    ctx.process.parent.put(\"pid\", ctx.winlog.event_data.ProcessId);\n  }\n}\nif (ctx.winlog?.event_data?.CommandLine != null) {\n  int start = 0;\n  int end = 0;\n  boolean in_quote = false;\n  ArrayList al = new ArrayList();\n  for (int i = 0; i < ctx.winlog.event_data.CommandLine.length(); i++) {\n    end = i;\n    if (Character.compare(ctx.winlog.event_data.CommandLine.charAt(i), \"\\\"\".charAt(0)) == 0) {\n      if (in_quote) {\n        in_quote = false;\n      } else {\n        in_quote = true;\n      }\n    }\n    if (Character.isWhitespace(ctx.winlog.event_data.CommandLine.charAt(i)) && !in_quote) {\n      al.add(ctx.winlog.event_data.CommandLine.substring(start, end));\n      start = i + 1;\n    }\n    if (i == ctx.winlog.event_data.CommandLine.length() - 1) {\n      al.add(ctx.winlog.event_data.CommandLine.substring(start, end + 1));\n    }\n  }\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  ctx.process.put(\"args\", al);\n  ctx.process.put(\"command_line\", ctx.winlog.event_data.CommandLine);\n  ctx.process.put(\"args_count\", al.size());\n}\nif ((ctx.winlog?.event_data?.TargetUserName != null) &&\n    (!ctx.winlog.event_data.TargetUserName.equals(\"-\"))) {\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  if (!ctx.related.user.contains(ctx.winlog.event_data.TargetUserName)) {\n    ctx.related.user.add(ctx.winlog.event_data.TargetUserName);\n  }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event?.code == null ||\n    ![\"4688\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.event_data?.NewProcessId != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  if (ctx.winlog.event_data.NewProcessId instanceof String) {\n    Long pid = Long.decode(ctx.winlog.event_data.NewProcessId);\n    ctx.process.put(\"pid\", pid.longValue());\n  } else {\n    ctx.process.put(\"pid\", ctx.winlog.event_data.NewProcessId);\n  }\n  ctx.winlog.event_data.remove(\"NewProcessId\");\n}\nif (ctx.winlog?.event_data?.NewProcessName != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  ctx.process.put(\"executable\", ctx.winlog.event_data.NewProcessName);\n  ctx.winlog.event_data.remove(\"NewProcessName\");\n}\nif (ctx.winlog?.event_data?.ParentProcessName != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  if (ctx.process?.parent == null) {\n    HashMap hm = new HashMap();\n    ctx.process.put(\"parent\", hm);\n  }\n  ctx.process.parent.put(\"executable\", ctx.winlog.event_data.ParentProcessName);\n  ctx.winlog.event_data.remove(\"ParentProcessName\");\n}\nif (ctx.process?.name == null && ctx.process?.executable != null) {\n  def parts = ctx.process.executable.splitOnToken(\"\\\\\");\n  ctx.process.put(\"name\", parts[-1]);\n}\nif (ctx.process?.parent?.name == null && ctx.process?.parent?.executable != null) {\n  def parts = ctx.process.parent.executable.splitOnToken(\"\\\\\");\n  ctx.process.parent.put(\"name\", parts[-1]);\n}\nif (ctx.winlog?.event_data?.ProcessId != null) {\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  if (ctx.process?.parent == null) {\n    HashMap hm = new HashMap();\n    ctx.process.put(\"parent\", hm);\n  }\n  if (ctx.winlog.event_data.ProcessId instanceof String) {\n    Long pid = Long.decode(ctx.winlog.event_data.ProcessId);\n    ctx.process.parent.put(\"pid\", pid.longValue());\n  } else {\n    ctx.process.parent.put(\"pid\", ctx.winlog.event_data.ProcessId);\n  }\n}\nif (ctx.winlog?.event_data?.CommandLine != null) {\n  int start = 0;\n  int end = 0;\n  boolean in_quote = false;\n  ArrayList al = new ArrayList();\n  for (int i = 0; i < ctx.winlog.event_data.CommandLine.length(); i++) {\n    end = i;\n    if (Character.compare(ctx.winlog.event_data.CommandLine.charAt(i), \"\\\"\".charAt(0)) == 0) {\n      if (in_quote) {\n        in_quote = false;\n      } else {\n        in_quote = true;\n      }\n    }\n    if (Character.isWhitespace(ctx.winlog.event_data.CommandLine.charAt(i)) && !in_quote) {\n      al.add(ctx.winlog.event_data.CommandLine.substring(start, end));\n      start = i + 1;\n    }\n    if (i == ctx.winlog.event_data.CommandLine.length() - 1) {\n      al.add(ctx.winlog.event_data.CommandLine.substring(start, end + 1));\n    }\n  }\n  if (ctx.process == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"process\", hm);\n  }\n  ctx.process.put(\"args\", al);\n  ctx.process.put(\"command_line\", ctx.winlog.event_data.CommandLine);\n  ctx.process.put(\"args_count\", al.size());\n}\nif ((ctx.winlog?.event_data?.TargetUserName != null) &&\n    (!ctx.winlog.event_data.TargetUserName.equals(\"-\"))) {\n  if (ctx.related == null) {\n    HashMap hm = new HashMap();\n    ctx.put(\"related\", hm);\n  }\n  if (ctx.related?.user == null) {\n    ArrayList al = new ArrayList();\n    ctx.related.put(\"user\", al);\n  }\n  if (!ctx.related.user.contains(ctx.winlog.event_data.TargetUserName)) {\n    ctx.related.user.add(ctx.winlog.event_data.TargetUserName);\n  }\n}"#
                        ),
                    )?;
                    let _cond = {
                        event.has_value("event.code")
                            && ["4624", "4648", "4797", "5379", "5380", "5381", "5382"]
                                .contains(&event.get_str("event.code").unwrap_or(""))
                            && event.has_value("winlog.event_data.SubjectUserName")
                            && event.get_str("winlog.event_data.SubjectUserName") != Some("-")
                    };
                    if _cond {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("winlog.event_data.SubjectUserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    let _cond = {
                        event.has_value("event.code")
                            && [
                                "4688", "4720", "4722", "4723", "4724", "4725", "4726", "4738",
                                "4740", "4767", "4797", "4798",
                            ]
                            .contains(&event.get_str("event.code").unwrap_or(""))
                            && event.has_value("winlog.event_data.TargetUserName")
                            && event.get_str("winlog.event_data.TargetUserName") != Some("-")
                    };
                    if _cond {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("winlog.event_data.TargetUserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    let _cond = {
                        event.has_value("event.code")
                            && ["4672", "4673", "4674", "4741", "4742", "4743"]
                                .contains(&event.get_str("event.code").unwrap_or(""))
                            && event.has_value("winlog.event_data.PrivilegeList")
                    };
                    if _cond {
                        if let Some(s) = event.get_string("winlog.event_data.PrivilegeList") {
                            let parts: Vec<Value> = cached_regex!("\\s+")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            event.set("winlog.event_data.PrivilegeList", Value::Array(parts))?;
                        }
                    }
                    if let Some(v) = event
                        .get("winlog.event_data.OldTargetUserName")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.target.name", v)?;
                    }
                    if let Some(v) = event
                        .get("winlog.event_data.NewTargetUserName")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.changes.name", v)?;
                    }
                    let _cond = {
                        event.has_value("winlog.event_data.NewTargetUserName")
                            && event.get_str("winlog.event_data.NewTargetUserName") != Some("-")
                    };
                    if _cond {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("winlog.event_data.NewTargetUserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    let _cond = {
                        event.has_value("winlog.event_data.OldTargetUserName")
                            && event.get_str("winlog.event_data.OldTargetUserName") != Some("-")
                    };
                    if _cond {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("winlog.event_data.OldTargetUserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    let _cond = {
                        event.get_str("event.code") == Some("5136")
                            && event.has_value("winlog.event_data.OperationType")
                    };
                    if _cond {
                        if let Some(v) = event.get("winlog.event_data.OperationType").cloned() {
                            event.set("event.reason", v)?;
                        }
                    }
                    let _cond = {
                        event.get_str("event.code") == Some("5136")
                            && event.has_value("winlog.event_data.ObjectDN")
                    };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            // Painless script
                            // Source: String objectDN = ctx.winlog.event_data.ObjectDN.toString();\nString objectClass = ctx.winlog?.event_data?.ObjectClass != null ? ctx.winlog.event_data.ObjectClass.toString().toLowerCase() : \"\";\nint cnStart = objectDN.toLowerCase().indexOf(\"cn=\");\nif (cnStart < 0) return;\nint valueStart = cnStart + 3;\nint dnLen = objectDN.length();\nStringBuilder cn = new StringBuilder();\nfor (int i = valueStart; i < dnLen; i++) {\n  char c = objectDN.charAt(i);\n  if (c == 92 && i + 1 < dnLen) { // backslash escape\n    char n = objectDN.charAt(++i);\n    int d1 = Character.digit(n, 16);\n    if (d1 >= 0 && i + 1 < dnLen) { // hex escape\n      int d2 = Character.digit(objectDN.charAt(i + 1), 16);\n      if (d2 >= 0) {\n        cn.append((char)(d1 * 16 + d2));\n        i++;\n        continue;\n      }\n    }\n    cn.append(n); // simple escaped char\n    continue;\n  }\n  if (c == 44) break; // unescaped comma ends CN\n  cn.append(c);\n}\nString cnValue = cn.toString().trim();\nif (cnValue.length() == 0) return;\nif (objectClass.contains(\"user\")) {\n  if (ctx.user == null) ctx.put(\"user\", new HashMap());\n  if (ctx.user.target == null) ctx.user.put(\"target\", new HashMap());\n  ctx.user.target.put(\"name\", cnValue);\n  if (ctx.related == null) ctx.put(\"related\", new HashMap());\n  if (ctx.related.user == null) ctx.related.put(\"user\", new ArrayList());\n  if (!ctx.related.user.contains(cnValue)) ctx.related.user.add(cnValue);\n} else if (objectClass.contains(\"group\")) {\n  if (ctx.group == null) ctx.put(\"group\", new HashMap());\n  ctx.group.put(\"name\", cnValue);\n  if (ctx.related == null) ctx.put(\"related\", new HashMap());\n  if (ctx.related.user == null) ctx.related.put(\"user\", new ArrayList());\n  if (!ctx.related.user.contains(cnValue)) ctx.related.user.add(cnValue);\n} else if (objectClass.contains(\"computer\") && (ctx.host == null || ctx.host.name == null)) {\n  if (ctx.host == null) ctx.put(\"host\", new HashMap());\n  ctx.host.put(\"name\", cnValue);\n}
                            // TODO: Transpile Painless to Rust (2.2.3)
                            painless_exec_plan(
                                event,
                                cached_painless!(
                                    r#"String objectDN = ctx.winlog.event_data.ObjectDN.toString();\nString objectClass = ctx.winlog?.event_data?.ObjectClass != null ? ctx.winlog.event_data.ObjectClass.toString().toLowerCase() : \"\";\nint cnStart = objectDN.toLowerCase().indexOf(\"cn=\");\nif (cnStart < 0) return;\nint valueStart = cnStart + 3;\nint dnLen = objectDN.length();\nStringBuilder cn = new StringBuilder();\nfor (int i = valueStart; i < dnLen; i++) {\n  char c = objectDN.charAt(i);\n  if (c == 92 && i + 1 < dnLen) { // backslash escape\n    char n = objectDN.charAt(++i);\n    int d1 = Character.digit(n, 16);\n    if (d1 >= 0 && i + 1 < dnLen) { // hex escape\n      int d2 = Character.digit(objectDN.charAt(i + 1), 16);\n      if (d2 >= 0) {\n        cn.append((char)(d1 * 16 + d2));\n        i++;\n        continue;\n      }\n    }\n    cn.append(n); // simple escaped char\n    continue;\n  }\n  if (c == 44) break; // unescaped comma ends CN\n  cn.append(c);\n}\nString cnValue = cn.toString().trim();\nif (cnValue.length() == 0) return;\nif (objectClass.contains(\"user\")) {\n  if (ctx.user == null) ctx.put(\"user\", new HashMap());\n  if (ctx.user.target == null) ctx.user.put(\"target\", new HashMap());\n  ctx.user.target.put(\"name\", cnValue);\n  if (ctx.related == null) ctx.put(\"related\", new HashMap());\n  if (ctx.related.user == null) ctx.related.put(\"user\", new ArrayList());\n  if (!ctx.related.user.contains(cnValue)) ctx.related.user.add(cnValue);\n} else if (objectClass.contains(\"group\")) {\n  if (ctx.group == null) ctx.put(\"group\", new HashMap());\n  ctx.group.put(\"name\", cnValue);\n  if (ctx.related == null) ctx.put(\"related\", new HashMap());\n  if (ctx.related.user == null) ctx.related.put(\"user\", new ArrayList());\n  if (!ctx.related.user.contains(cnValue)) ctx.related.user.add(cnValue);\n} else if (objectClass.contains(\"computer\") && (ctx.host == null || ctx.host.name == null)) {\n  if (ctx.host == null) ctx.put(\"host\", new HashMap());\n  ctx.host.put(\"name\", cnValue);\n}"#
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = {
                        event.has_value("winlog.event_data.ObjectName")
                            && (["4657"].contains(&event.get_str("event.code").unwrap_or(""))
                                || (["4656", "4658", "4660", "4661", "4662", "4663"]
                                    .contains(&event.get_str("event.code").unwrap_or(""))
                                    && (event.get_str("winlog.event_data.ObjectType")
                                        == Some("Key")
                                        || (event
                                            .get("winlog.event_data.ObjectName")
                                            .is_some_and(|v| v.is_string())
                                            && event
                                                .get_str("winlog.event_data.ObjectName")
                                                .is_some_and(|s| {
                                                    s.to_uppercase().starts_with("\\REGISTRY\\")
                                                })))))
                    };
                    if _cond {
                        if let Some(v) = event.get("winlog.event_data.ObjectName").cloned() {
                            event.set("registry.path", v)?;
                        }
                    }
                    if event.has_value("winlog.event_data.SidList") {
                        if let Some(s) = event.get_string("winlog.event_data.SidList") {
                            let re = cached_regex!("\\s+");
                            let replaced = re.replace_all(&s, " ").into_owned();
                            event.set("winlog.event_data.SidList", replaced)?;
                        }
                    }
                    // Painless script
                    // Source: ArrayList translatePermissionMask(def mask, def params) {\n  ArrayList al = new ArrayList();\n  Long permCode = Long.decode(mask);\n  for (entry in params.PermsFlags.entrySet()) {\n    Long permFlag = Long.decode(entry.getKey());\n    if ((permCode.longValue() & permFlag.longValue()) == permFlag.longValue()) {\n      al.add(entry.getValue());\n    }\n  }\n  if (al.length == 0) {\n    al.add(mask);\n  }\n  return al;\n}\n\nHashMap translateACL(def dacl, def params) {\n  def aceArray = dacl.splitOnToken(\";\");\n  HashMap hm = new HashMap();\n\n  if (aceArray.length >= 6 ) {\n    hm.put(\"grantee\", translateSID(aceArray[5], params));\n  }\n\n  if (aceArray.length >= 1) {\n    hm.put(\"type\", params.AceTypes[aceArray[0]]);\n  }\n\n  if (aceArray.length >= 3) {\n    if (aceArray[2].startsWith(\"0x\")) {\n      hm.put(\"perms\", translatePermissionMask(aceArray[2], params));\n    } else {\n      ArrayList al = new ArrayList();\n      Pattern permPattern = /.{1,2}/;\n      Matcher permMatcher = permPattern.matcher(aceArray[2]);\n      while (permMatcher.find()) {\n        al.add(params.PermissionDescription[permMatcher.group(0)]);\n      }\n      hm.put(\"perms\", al);\n    }\n  }\n  return hm;\n}\nString translateSID(def sid, def params) {\n  if (!params.AccountSIDDescription.containsKey(sid)) {\n    if (sid.startsWith(\"S-1-5-21\")) {\n      Pattern uidPattern = /[0-9]{1,5}$/;\n      Matcher uidMatcher = uidPattern.matcher(sid);\n      if (uidMatcher.find()) {\n        return params.DomainSpecificSID[uidMatcher.group(0)];\n      }\n      return sid;\n    }\n    return sid;\n  }\n  return params.AccountSIDDescription[sid];\n}\n\nvoid enrichSDDL(def sddlStr, def Sd, def params, def ctx) {\n  Pattern sdOwnerPattern = /^O\\:[A-Z]{2}/;\n  Matcher sdOwnerMatcher = sdOwnerPattern.matcher(sddlStr);\n  if (sdOwnerMatcher.find()) {\n    ctx.winlog.event_data.put(Sd + \"Owner\", translateSID(sdOwnerMatcher.group(0), params));\n  }\n\n  Pattern sdGroupPattern = /^G\\:[A-Z]{2}/;\n  Matcher sdGroupMatcher = sdGroupPattern.matcher(sddlStr);\n  if (sdGroupMatcher.find()) {\n    ctx.winlog.event_data.put(Sd + \"Group\", translateSID(sdGroupMatcher.group(0), params));\n  }\n\n  Pattern sdDaclPattern = /(D:([A-Z]*(\\(.*\\))*))/;\n  Matcher sdDaclMatcher = sdDaclPattern.matcher(sddlStr);\n  if (sdDaclMatcher.find()) {\n    Pattern dacListPattern = /\\([^*\\)]*\\)/;\n    Matcher dacListMatcher = dacListPattern.matcher(sdDaclMatcher.group(1));\n    for (def i = 0; dacListMatcher.find(); i++) {\n      def newDacl = translateACL(dacListMatcher.group(0).replace(\"(\",\"\").replace(\")\",\"\"), params);\n      ctx.winlog.event_data.put(Sd + \"Dacl\" + i.toString(), newDacl['grantee'] + \" :\" + newDacl['type'] + \" (\" + newDacl['perms'] + \")\");\n      if ([\"Administrator\", \"Guest\", \"KRBTGT\"].contains(newDacl['grantee'])) {\n        if (ctx.related == null) {\n          HashMap hm = new HashMap();\n          ctx.put(\"related\", hm);\n        }\n        if (ctx.related?.user == null) {\n          ArrayList al = new ArrayList();\n          ctx.related.put(\"user\", al);\n        }\n        if (!ctx.related.user.contains(newDacl['grantee'])) {\n          ctx.related.user.add(newDacl['grantee']);\n        }\n      }\n    }\n  }\n\n  Pattern sdSaclPattern = /(S:([A-Z]*(\\(.*\\))*))?$/;\n  Matcher sdSaclMatcher = sdSaclPattern.matcher(sddlStr);\n  if (sdSaclMatcher.find()) {\n    Pattern sacListPattern = /\\([^*\\)]*\\)/;\n    Matcher sacListMatcher = sacListPattern.matcher(sdSaclMatcher.group(0));\n    for (def i = 0; sacListMatcher.find(); i++) {\n      def newSacl = translateACL(sacListMatcher.group(0).replace(\"(\",\"\").replace(\")\",\"\"), params);\n      ctx.winlog.event_data.put(Sd + \"Sacl\" + i.toString(), newSacl['grantee'] + \" :\" + newSacl['type'] + \" (\" + newSacl['perms'] + \")\");\n      if ([\"Administrator\", \"Guest\", \"KRBTGT\"].contains(newSacl['grantee'])) {\n        if (ctx.related == null) {\n          HashMap hm = new HashMap();\n          ctx.put(\"related\", hm);\n        }\n        if (ctx.related?.user == null) {\n          ArrayList al = new ArrayList();\n          ctx.related.put(\"user\", al);\n        }\n        if (!ctx.related.user.contains(newSacl['grantee'])) {\n          ctx.related.user.add(newSacl['grantee']);\n        }\n      }\n    }\n  }\n}\n\nvoid splitSidList(def sids, def params, def ctx) {\n  ArrayList al = new ArrayList();\n  def sidsArray = sids.splitOnToken(\" \");\n  ArrayList sidList = new ArrayList(Arrays.asList(sidsArray));\n  ctx.winlog.event_data.put(\"SidList\", sidList);\n  for (def i = 0; i < sidList.length; i++ ) {\n    al.add(translateSID(sidList[i].replace(\"%\", \"\").replace(\"{\", \"\").replace(\"}\", \"\").replace(\" \",\"\"), params));\n  }\n  ctx.winlog.event_data.put(\"SidListDesc\", al);\n}\nif (ctx.winlog?.event_data?.RemoteMachineID != null) {  \n  ctx.winlog.event_data.put(\"RemoteMachineDescription\", params.AccountSIDDescription[ctx.winlog.event_data.RemoteMachineID]);\n}\nif (ctx.winlog?.event_data?.RemoteUserID != null) {  \n  ctx.winlog.event_data.put(\"RemoteUserDescription\", params.AccountSIDDescription[ctx.winlog.event_data.RemoteUserID]);\n}\nif (ctx.event?.code == null ||\n    ![\"4670\", \"4817\", \"4907\", \"4908\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.event_data?.OldSd != null) {\n  enrichSDDL(ctx.winlog.event_data.OldSd, \"OldSd\", params, ctx);\n}\nif (ctx.winlog?.event_data?.NewSd != null) {\n  enrichSDDL(ctx.winlog.event_data.NewSd, \"NewSd\", params, ctx);\n}\nif (ctx.winlog?.event_data?.SidList != null) {\n  splitSidList(ctx.winlog.event_data.SidList, params, ctx);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ArrayList translatePermissionMask(def mask, def params) {\n  ArrayList al = new ArrayList();\n  Long permCode = Long.decode(mask);\n  for (entry in params.PermsFlags.entrySet()) {\n    Long permFlag = Long.decode(entry.getKey());\n    if ((permCode.longValue() & permFlag.longValue()) == permFlag.longValue()) {\n      al.add(entry.getValue());\n    }\n  }\n  if (al.length == 0) {\n    al.add(mask);\n  }\n  return al;\n}\n\nHashMap translateACL(def dacl, def params) {\n  def aceArray = dacl.splitOnToken(\";\");\n  HashMap hm = new HashMap();\n\n  if (aceArray.length >= 6 ) {\n    hm.put(\"grantee\", translateSID(aceArray[5], params));\n  }\n\n  if (aceArray.length >= 1) {\n    hm.put(\"type\", params.AceTypes[aceArray[0]]);\n  }\n\n  if (aceArray.length >= 3) {\n    if (aceArray[2].startsWith(\"0x\")) {\n      hm.put(\"perms\", translatePermissionMask(aceArray[2], params));\n    } else {\n      ArrayList al = new ArrayList();\n      Pattern permPattern = /.{1,2}/;\n      Matcher permMatcher = permPattern.matcher(aceArray[2]);\n      while (permMatcher.find()) {\n        al.add(params.PermissionDescription[permMatcher.group(0)]);\n      }\n      hm.put(\"perms\", al);\n    }\n  }\n  return hm;\n}\nString translateSID(def sid, def params) {\n  if (!params.AccountSIDDescription.containsKey(sid)) {\n    if (sid.startsWith(\"S-1-5-21\")) {\n      Pattern uidPattern = /[0-9]{1,5}$/;\n      Matcher uidMatcher = uidPattern.matcher(sid);\n      if (uidMatcher.find()) {\n        return params.DomainSpecificSID[uidMatcher.group(0)];\n      }\n      return sid;\n    }\n    return sid;\n  }\n  return params.AccountSIDDescription[sid];\n}\n\nvoid enrichSDDL(def sddlStr, def Sd, def params, def ctx) {\n  Pattern sdOwnerPattern = /^O\\:[A-Z]{2}/;\n  Matcher sdOwnerMatcher = sdOwnerPattern.matcher(sddlStr);\n  if (sdOwnerMatcher.find()) {\n    ctx.winlog.event_data.put(Sd + \"Owner\", translateSID(sdOwnerMatcher.group(0), params));\n  }\n\n  Pattern sdGroupPattern = /^G\\:[A-Z]{2}/;\n  Matcher sdGroupMatcher = sdGroupPattern.matcher(sddlStr);\n  if (sdGroupMatcher.find()) {\n    ctx.winlog.event_data.put(Sd + \"Group\", translateSID(sdGroupMatcher.group(0), params));\n  }\n\n  Pattern sdDaclPattern = /(D:([A-Z]*(\\(.*\\))*))/;\n  Matcher sdDaclMatcher = sdDaclPattern.matcher(sddlStr);\n  if (sdDaclMatcher.find()) {\n    Pattern dacListPattern = /\\([^*\\)]*\\)/;\n    Matcher dacListMatcher = dacListPattern.matcher(sdDaclMatcher.group(1));\n    for (def i = 0; dacListMatcher.find(); i++) {\n      def newDacl = translateACL(dacListMatcher.group(0).replace(\"(\",\"\").replace(\")\",\"\"), params);\n      ctx.winlog.event_data.put(Sd + \"Dacl\" + i.toString(), newDacl['grantee'] + \" :\" + newDacl['type'] + \" (\" + newDacl['perms'] + \")\");\n      if ([\"Administrator\", \"Guest\", \"KRBTGT\"].contains(newDacl['grantee'])) {\n        if (ctx.related == null) {\n          HashMap hm = new HashMap();\n          ctx.put(\"related\", hm);\n        }\n        if (ctx.related?.user == null) {\n          ArrayList al = new ArrayList();\n          ctx.related.put(\"user\", al);\n        }\n        if (!ctx.related.user.contains(newDacl['grantee'])) {\n          ctx.related.user.add(newDacl['grantee']);\n        }\n      }\n    }\n  }\n\n  Pattern sdSaclPattern = /(S:([A-Z]*(\\(.*\\))*))?$/;\n  Matcher sdSaclMatcher = sdSaclPattern.matcher(sddlStr);\n  if (sdSaclMatcher.find()) {\n    Pattern sacListPattern = /\\([^*\\)]*\\)/;\n    Matcher sacListMatcher = sacListPattern.matcher(sdSaclMatcher.group(0));\n    for (def i = 0; sacListMatcher.find(); i++) {\n      def newSacl = translateACL(sacListMatcher.group(0).replace(\"(\",\"\").replace(\")\",\"\"), params);\n      ctx.winlog.event_data.put(Sd + \"Sacl\" + i.toString(), newSacl['grantee'] + \" :\" + newSacl['type'] + \" (\" + newSacl['perms'] + \")\");\n      if ([\"Administrator\", \"Guest\", \"KRBTGT\"].contains(newSacl['grantee'])) {\n        if (ctx.related == null) {\n          HashMap hm = new HashMap();\n          ctx.put(\"related\", hm);\n        }\n        if (ctx.related?.user == null) {\n          ArrayList al = new ArrayList();\n          ctx.related.put(\"user\", al);\n        }\n        if (!ctx.related.user.contains(newSacl['grantee'])) {\n          ctx.related.user.add(newSacl['grantee']);\n        }\n      }\n    }\n  }\n}\n\nvoid splitSidList(def sids, def params, def ctx) {\n  ArrayList al = new ArrayList();\n  def sidsArray = sids.splitOnToken(\" \");\n  ArrayList sidList = new ArrayList(Arrays.asList(sidsArray));\n  ctx.winlog.event_data.put(\"SidList\", sidList);\n  for (def i = 0; i < sidList.length; i++ ) {\n    al.add(translateSID(sidList[i].replace(\"%\", \"\").replace(\"{\", \"\").replace(\"}\", \"\").replace(\" \",\"\"), params));\n  }\n  ctx.winlog.event_data.put(\"SidListDesc\", al);\n}\nif (ctx.winlog?.event_data?.RemoteMachineID != null) {  \n  ctx.winlog.event_data.put(\"RemoteMachineDescription\", params.AccountSIDDescription[ctx.winlog.event_data.RemoteMachineID]);\n}\nif (ctx.winlog?.event_data?.RemoteUserID != null) {  \n  ctx.winlog.event_data.put(\"RemoteUserDescription\", params.AccountSIDDescription[ctx.winlog.event_data.RemoteUserID]);\n}\nif (ctx.event?.code == null ||\n    ![\"4670\", \"4817\", \"4907\", \"4908\"].contains(ctx.event.code)) {\n  return;\n}\nif (ctx.winlog?.event_data?.OldSd != null) {\n  enrichSDDL(ctx.winlog.event_data.OldSd, \"OldSd\", params, ctx);\n}\nif (ctx.winlog?.event_data?.NewSd != null) {\n  enrichSDDL(ctx.winlog.event_data.NewSd, \"NewSd\", params, ctx);\n}\nif (ctx.winlog?.event_data?.SidList != null) {\n  splitSidList(ctx.winlog.event_data.SidList, params, ctx);\n}"#
                        ),
                        cached_params!(
                            "{\"AccountSIDDescription\":{\"AN\":\"Anonymous logon\",\"AO\":\"Account operators\",\"AU\":\"Authenticated users\",\"BA\":\"Built-in administrators\",\"BG\":\"Built-in guests\",\"BO\":\"Backup operators\",\"BU\":\"Built-in users\",\"CA\":\"Certificate server administrators\",\"CG\":\"Creator group\",\"CO\":\"Creator owner\",\"DA\":\"Domain administrators\",\"DC\":\"Domain computers\",\"DD\":\"Domain controllers\",\"DG\":\"Domain guests\",\"DU\":\"Domain users\",\"EA\":\"Enterprise administrators\",\"ED\":\"Enterprise domain controllers\",\"IU\":\"Interactively logged-on user\",\"LA\":\"Local administrator\",\"LG\":\"Local guest\",\"LS\":\"Local service account\",\"NO\":\"Network configuration operators\",\"NS\":\"Network service account\",\"NU\":\"Network logon user\",\"PA\":\"Group Policy administrators\",\"PO\":\"Printer operators\",\"PS\":\"Personal self\",\"PU\":\"Power users\",\"RC\":\"Restricted code\",\"RD\":\"Terminal server users\",\"RE\":\"Replicator\",\"RS\":\"RAS servers group\",\"RU\":\"Alias to allow previous Windows 2000\",\"S-1-0\":\"Null Authority\",\"S-1-0-0\":\"Nobody\",\"S-1-1\":\"World Authority\",\"S-1-1-0\":\"Everyone\",\"S-1-16-0\":\"Untrusted Mandatory Level\",\"S-1-16-12288\":\"High Mandatory Level\",\"S-1-16-16384\":\"System Mandatory Level\",\"S-1-16-20480\":\"Protected Process Mandatory Level\",\"S-1-16-28672\":\"Secure Process Mandatory Level\",\"S-1-16-4096\":\"Low Mandatory Level\",\"S-1-16-8192\":\"Medium Mandatory Level\",\"S-1-16-8448\":\"Medium Plus Mandatory Level\",\"S-1-2\":\"Local Authority\",\"S-1-2-0\":\"Local\",\"S-1-2-1\":\"Console Logon\",\"S-1-3\":\"Creator Authority\",\"S-1-3-0\":\"Creator Owner\",\"S-1-3-1\":\"Creator Group\",\"S-1-3-2\":\"Creator Owner Server\",\"S-1-3-3\":\"Creator Group Server\",\"S-1-3-4\":\"Owner Rights\",\"S-1-4\":\"Non-unique Authority\",\"S-1-5\":\"NT Authority\",\"S-1-5-1\":\"Dialup\",\"S-1-5-10\":\"Principal Self\",\"S-1-5-11\":\"Authenticated Users\",\"S-1-5-12\":\"Restricted Code\",\"S-1-5-13\":\"Terminal Server Users\",\"S-1-5-14\":\"Remote Interactive Logon\",\"S-1-5-15\":\"This Organization\",\"S-1-5-17\":\"This Organization\",\"S-1-5-18\":\"Local System\",\"S-1-5-19\":\"NT Authority\",\"S-1-5-2\":\"Network\",\"S-1-5-20\":\"NT Authority\",\"S-1-5-3\":\"Batch\",\"S-1-5-32-544\":\"Administrators\",\"S-1-5-32-545\":\"Users\",\"S-1-5-32-546\":\"Guests\",\"S-1-5-32-547\":\"Power Users\",\"S-1-5-32-548\":\"Account Operators\",\"S-1-5-32-549\":\"Server Operators\",\"S-1-5-32-550\":\"Print Operators\",\"S-1-5-32-551\":\"Backup Operators\",\"S-1-5-32-552\":\"Replicators\",\"S-1-5-32-554\":\"Builtin\\\\Pre-Windows 2000 Compatible Access\",\"S-1-5-32-555\":\"Builtin\\\\Remote Desktop Users\",\"S-1-5-32-556\":\"Builtin\\\\Network Configuration Operators\",\"S-1-5-32-557\":\"Builtin\\\\Incoming Forest Trust Builders\",\"S-1-5-32-558\":\"Builtin\\\\Performance Monitor Users\",\"S-1-5-32-559\":\"Builtin\\\\Performance Log Users\",\"S-1-5-32-560\":\"Builtin\\\\Windows Authorization Access Group\",\"S-1-5-32-561\":\"Builtin\\\\Terminal Server License Servers\",\"S-1-5-32-562\":\"Builtin\\\\Distributed COM Users\",\"S-1-5-32-569\":\"Builtin\\\\Cryptographic Operators\",\"S-1-5-32-573\":\"Builtin\\\\Event Log Readers\",\"S-1-5-32-574\":\"Builtin\\\\Certificate Service DCOM Access\",\"S-1-5-32-575\":\"Builtin\\\\RDS Remote Access Servers\",\"S-1-5-32-576\":\"Builtin\\\\RDS Endpoint Servers\",\"S-1-5-32-577\":\"Builtin\\\\RDS Management Servers\",\"S-1-5-32-578\":\"Builtin\\\\Hyper-V Administrators\",\"S-1-5-32-579\":\"Builtin\\\\Access Control Assistance Operators\",\"S-1-5-32-580\":\"Builtin\\\\Remote Management Users\",\"S-1-5-32-582\":\"Storage Replica Administrators\",\"S-1-5-4\":\"Interactive\",\"S-1-5-5-X-Y\":\"Logon Session\",\"S-1-5-6\":\"Service\",\"S-1-5-64-10\":\"NTLM Authentication\",\"S-1-5-64-14\":\"SChannel Authentication\",\"S-1-5-64-21\":\"Digest Authentication\",\"S-1-5-7\":\"Anonymous\",\"S-1-5-8\":\"Proxy\",\"S-1-5-80\":\"NT Service\",\"S-1-5-80-0\":\"All Services\",\"S-1-5-83-0\":\"NT Virtual Machine\\\\Virtual Machines\",\"S-1-5-9\":\"Enterprise Domain Controllers\",\"S-1-5-90-0\":\"Windows Manager\\\\Windows Manager Group\",\"SA\":\"Schema administrators\",\"SO\":\"Server operators\",\"SU\":\"Service logon user\",\"SY\":\"Local system\",\"WD\":\"Everyone\"},\"AceTypes\":{\"A\":\"Access Allowed\",\"AL\":\"System Alarm\",\"AU\":\"System Audit\",\"D\":\"Access Denied\",\"ML\":\"System Mandatory Label\",\"OA\":\"Object Access Allowed\",\"OD\":\"Object Access Denied\",\"OL\":\"System Object Alarm\",\"OU\":\"System Object Audit\",\"SP\":\"Central Policy ID\"},\"DomainSpecificSID\":{\"498\":\"Enterprise Read-only Domain Controllers\",\"500\":\"Administrator\",\"501\":\"Guest\",\"502\":\"KRBTGT\",\"512\":\"Domain Admins\",\"513\":\"Domain Users\",\"514\":\"Domain Guests\",\"515\":\"Domain Computers\",\"516\":\"Domain Controllers\",\"517\":\"Cert Publishers\",\"518\":\"Schema Admins\",\"519\":\"Enterprise Admins\",\"520\":\"Group Policy Creator Owners\",\"521\":\"Read-only Domain Controllers\",\"522\":\"Cloneable Domain Controllers\",\"526\":\"Key Admins\",\"527\":\"Enterprise Key Admins\",\"553\":\"RAS and IAS Servers\",\"571\":\"Allowed RODC Password Replication Group\",\"572\":\"Denied RODC Password Replication Group\"},\"PermissionDescription\":{\"CC\":\"Create All Child Objects\",\"CR\":\"All Extended Rights\",\"DC\":\"Delete All Child Objects\",\"DT\":\"Delete Subtree\",\"FA\":\"File All Access\",\"FR\":\"File Generic Read\",\"FW\":\"FILE GENERIC WRITE\",\"FX\":\"FILE GENERIC EXECUTE\",\"GA\":\"Generic All\",\"GR\":\"Generic Read\",\"GW\":\"Generic Write\",\"GX\":\"Generic Execute\",\"KA\":\"KEY ALL ACCESS\",\"KR\":\"KEY READ\",\"KW\":\"KEY WRITE\",\"KX\":\"KEY EXECUTE\",\"LC\":\"List Contents\",\"LO\":\"List Object\",\"RC\":\"Read Permissions\",\"RP\":\"Read All Properties\",\"SD\":\"Delete\",\"SW\":\"All Validated\",\"WD\":\"Modify Permissions\",\"WO\":\"Modify Owner\",\"WP\":\"Write All Properties\"},\"PermsFlags\":{\"0x00010000\":\"Delete\",\"0x00020000\":\"Read Control\",\"0x00040000\":\"Write DACL\",\"0x00080000\":\"Write Owner\",\"0x00100000\":\"Syncronize\",\"0x01000000\":\"Access System Security\",\"0x02000000\":\"Maximum Allowed\",\"0x10000000\":\"Generic All\",\"0x20000000\":\"Generic Execute\",\"0x4000000\":\"Generic Write\",\"0x80000000\":\"Generic Read\"}}"
                        ),
                    )?;
                    let _cond = {
                        event.has_value("network.iana_number")
                            && !event.has_value("network.transport")
                    };
                    if _cond {
                        // Painless script
                        // Source: if (ctx.network?.iana_number == null) {\n  return;\n} def t = params.get(ctx.network.iana_number); if (t == null) {\n  return;\n} ctx.network.put(\"transport\", t)\n
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan_params(
                            event,
                            cached_painless!(
                                r#"if (ctx.network?.iana_number == null) {\n  return;\n} def t = params.get(ctx.network.iana_number); if (t == null) {\n  return;\n} ctx.network.put(\"transport\", t)\n"#
                            ),
                            cached_params!(
                                "{\"1\":\"icmp\",\"12\":\"pup\",\"17\":\"udp\",\"2\":\"igmp\",\"27\":\"rdp\",\"28\":\"irtp\",\"33\":\"dccp\",\"35\":\"idpr\",\"4\":\"ipv4\",\"41\":\"ipv6\",\"43\":\"ipv6-route\",\"44\":\"ipv6-frag\",\"46\":\"rsvp\",\"47\":\"gre\",\"50\":\"esp\",\"58\":\"ipv6-icmp\",\"59\":\"ipv6-nonxt\",\"6\":\"tcp\",\"60\":\"ipv6-opts\",\"8\":\"egp\",\"9\":\"igp\"}"
                            ),
                        )?;
                    }
                    let _cond = {
                        event.has_value("event.code")
                            && ["5140", "5142", "5145"]
                                .contains(&event.get_str("event.code").unwrap_or(""))
                    };
                    if _cond {
                        // Painless script
                        // Source: if (ctx.file == null) { ctx.file = new HashMap(); }\nString rel = \"\";\nif (ctx.winlog?.event_data?.RelativeTargetName != null) {\n  rel = ctx.winlog.event_data.RelativeTargetName;\n}\nString share = \"\";\nif (ctx.winlog?.event_data?.ShareLocalPath != null) {\n  share = ctx.winlog.event_data.ShareLocalPath;\n}\nif (share.startsWith(\"\\\\??\\\\\")) {\n  share = share.substring(4);\n} else if (share.startsWith(\"\\\\?\\\\\")) {\n  share = share.substring(3);\n}\nif (share.endsWith(\"\\\\\")) {\n  share = share.substring(0, share.length() - 1);\n}\nif (rel.startsWith(\"\\\\\")) {\n  rel = rel.substring(1);\n}\nif (rel != \"\") {\n  String path = share + \"\\\\\" + rel;\n  ctx.file.put(\"path\", path);\n  int lastSep = path.lastIndexOf('\\\\');\n  if (lastSep >= 0) {\n    ctx.file.put(\"name\", path.substring(lastSep + 1));\n    ctx.file.put(\"directory\", path.substring(0, lastSep));\n  } else {\n    ctx.file.put(\"directory\", share);\n  }\n} else {\n  ctx.file.put(\"directory\", share);\n}\nString shareName = \"\";\nif (ctx.winlog?.event_data?.ShareName != null) {\n  shareName = ctx.winlog.event_data.ShareName;\n}\nif (shareName != \"\" && rel != \"\" && [\"5140\", \"5145\"].contains(ctx.event.code)) {\n  ctx.file.put(\"target_path\", shareName + \"\\\\\" + rel);\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"if (ctx.file == null) { ctx.file = new HashMap(); }\nString rel = \"\";\nif (ctx.winlog?.event_data?.RelativeTargetName != null) {\n  rel = ctx.winlog.event_data.RelativeTargetName;\n}\nString share = \"\";\nif (ctx.winlog?.event_data?.ShareLocalPath != null) {\n  share = ctx.winlog.event_data.ShareLocalPath;\n}\nif (share.startsWith(\"\\\\??\\\\\")) {\n  share = share.substring(4);\n} else if (share.startsWith(\"\\\\?\\\\\")) {\n  share = share.substring(3);\n}\nif (share.endsWith(\"\\\\\")) {\n  share = share.substring(0, share.length() - 1);\n}\nif (rel.startsWith(\"\\\\\")) {\n  rel = rel.substring(1);\n}\nif (rel != \"\") {\n  String path = share + \"\\\\\" + rel;\n  ctx.file.put(\"path\", path);\n  int lastSep = path.lastIndexOf('\\\\');\n  if (lastSep >= 0) {\n    ctx.file.put(\"name\", path.substring(lastSep + 1));\n    ctx.file.put(\"directory\", path.substring(0, lastSep));\n  } else {\n    ctx.file.put(\"directory\", share);\n  }\n} else {\n  ctx.file.put(\"directory\", share);\n}\nString shareName = \"\";\nif (ctx.winlog?.event_data?.ShareName != null) {\n  shareName = ctx.winlog.event_data.ShareName;\n}\nif (shareName != \"\" && rel != \"\" && [\"5140\", \"5145\"].contains(ctx.event.code)) {\n  ctx.file.put(\"target_path\", shareName + \"\\\\\" + rel);\n}"#
                            ),
                        )?;
                    }
                    let _cond = { event.has_value("file.name") };
                    if _cond {
                        // Painless script
                        // Source: def extIdx = ctx.file.name.lastIndexOf(\".\");\nif (extIdx > -1) {\n    ctx.file.extension = ctx.file.name.substring(extIdx+1);\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"def extIdx = ctx.file.name.lastIndexOf(\".\");\nif (extIdx > -1) {\n    ctx.file.extension = ctx.file.name.substring(extIdx+1);\n}"#
                            ),
                        )?;
                    }
                    if event.has("winlog.event_data.DirectionDescription") {
                        event.rename(
                            "winlog.event_data.DirectionDescription",
                            "network.direction",
                        )?;
                    }
                    if event.has_value("network.direction") {
                        if let Some(s) = event.get_string("network.direction") {
                            let lowered = s.to_lowercase();
                            event.set("network.direction", lowered)?;
                        }
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("source.ip") {
                            // Community ID v1 hash
                            if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                                event.get_string("source.ip"),
                                event.get_string("destination.ip"),
                                event
                                    .get_as_string("network.iana_number")
                                    .or_else(|| event.get_as_string("network.transport")),
                            ) {
                                let icmp = matches!(
                                    protocol.to_ascii_lowercase().as_str(),
                                    "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                                );
                                let (src_field, dst_field) = if icmp {
                                    ("icmp.type", "icmp.code")
                                } else {
                                    ("source.port", "destination.port")
                                };
                                let src_port =
                                    u16::try_from(event.get_as_i64(src_field).unwrap_or(0))
                                        .unwrap_or(0);
                                let dst_port =
                                    u16::try_from(event.get_as_i64(dst_field).unwrap_or(0))
                                        .unwrap_or(0);
                                match community_id_v1(
                                    &src_ip, &dst_ip, src_port, dst_port, &protocol,
                                ) {
                                    Ok(cid) => event.set("network.community_id", cid)?,
                                    Err(message) => {
                                        return Err(TransformError::ParseError {
                                            path: "network.community_id".into(),
                                            message,
                                        });
                                    }
                                }
                            }
                        }
                        Ok(())
                    })();
                    event.remove("_temp");
                    // End nested pipeline: "security_standard"
                }
                let _cond = {
                    event.has_value("winlog")
                        && event.get_str("winlog.provider_name")
                            == Some("Microsoft-Windows-Security-Auditing")
                        && event.has_value("event")
                        && event
                            .get("winlog._tmp.scheduled_task")
                            .is_some_and(|v| v.is_object())
                        && ((["4698", "4699", "4700", "4701"]
                            .contains(&event.get_str("event.code").unwrap_or(""))
                            && event
                                .get("winlog._tmp.scheduled_task.task_content")
                                .is_some_and(|v| v.is_object()))
                            || (event.get_str("event.code") == Some("4702")
                                && event
                                    .get("winlog._tmp.scheduled_task.task_content_new")
                                    .is_some_and(|v| v.is_object())))
                };
                if _cond {
                    // Begin nested pipeline: "security_scheduled_task"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: def asMap(def value) {\n  return value instanceof Map ? value : null;\n}\n\nArrayList asList(def value) {\n  ArrayList values = new ArrayList();\n  if (value == null) {\n    return values;\n  }\n  if (value instanceof List) {\n    for (def item : value) {\n      values.add(item);\n    }\n    return values;\n  }\n  values.add(value);\n  return values;\n}\n\nString text(def value) {\n  if (value == null) {\n    return null;\n  }\n  def raw = value;\n  if (value instanceof Map) {\n    raw = value.get(\"#text\");\n  }\n  if (raw == null) {\n    return null;\n  }\n  String s = raw.toString();\n  return s.length() == 0 ? null : s;\n}\n\ndef boolValue(def value) {\n  String s = text(value);\n  if (s == null) {\n    return null;\n  }\n  String v = s.toLowerCase();\n  if (v.equals(\"true\") || v.equals(\"1\")) {\n    return true;\n  }\n  if (v.equals(\"false\") || v.equals(\"0\")) {\n    return false;\n  }\n  return null;\n}\n\nvoid putText(HashMap target, String field, def source, String key) {\n  if (!(source instanceof Map)) {\n    return;\n  }\n  String value = text(source.get(key));\n  if (value != null) {\n    target.put(field, value);\n  }\n}\n\nvoid putBool(HashMap target, String field, def source, String key) {\n  if (!(source instanceof Map)) {\n    return;\n  }\n  def value = boolValue(source.get(key));\n  if (value != null) {\n    target.put(field, value);\n  }\n}\n\nvoid putBoolDefault(HashMap target, String field, def source, String key, boolean defaultValue) {\n  if (!(source instanceof Map) || !source.containsKey(key)) {\n    target.put(field, defaultValue);\n    return;\n  }\n  def value = boolValue(source.get(key));\n  if (value != null) {\n    target.put(field, value);\n  }\n}\n\nString escapeDefinitionValue(String value) {\n  String hex = \"0123456789ABCDEF\";\n  StringBuilder escaped = new StringBuilder();\n  for (int i = 0; i < value.length(); i++) {\n    int code = value.charAt(i);\n    // Escape %, |, =, C0 control characters, and DEL.\n    if (code == 37 || code == 124 || code == 61 || code < 32 || code == 127) {\n      escaped.append(\"%\");\n      escaped.append(hex.charAt((code >> 4) & 15));\n      escaped.append(hex.charAt(code & 15));\n    } else {\n      escaped.append(value.charAt(i));\n    }\n  }\n  return escaped.toString();\n}\n\nString canonicalDefinition(HashMap values, def keys) {\n  StringBuilder definition = new StringBuilder(\"v=1\");\n  for (def keyValue : keys) {\n    String key = keyValue.toString();\n    definition.append(\"|\");\n    definition.append(key);\n    definition.append(\"=\");\n    if (!values.containsKey(key) || values.get(key) == null) {\n      continue;\n    }\n    definition.append(escapeDefinitionValue(values.get(key).toString()));\n  }\n  return definition.toString();\n}\n\nArrayList normalizePrincipals(def task) {\n  def principalsBlock = asMap(task.get(\"principals\"));\n  if (principalsBlock == null) {\n    return null;\n  }\n\n  ArrayList principals = new ArrayList();\n  for (def principalValue : asList(principalsBlock.get(\"principal\"))) {\n    def principalBlock = asMap(principalValue);\n    if (principalBlock == null) {\n      continue;\n    }\n\n    HashMap principal = new HashMap();\n    putText(principal, \"id\", principalBlock, \"id\");\n\n    String userId = text(principalBlock.get(\"userid\"));\n    if (userId != null) {\n      HashMap user = new HashMap();\n      user.put(\"identifier\", userId);\n      principal.put(\"user\", user);\n    }\n\n    String groupId = text(principalBlock.get(\"groupid\"));\n    if (groupId != null) {\n      HashMap group = new HashMap();\n      group.put(\"identifier\", groupId);\n      principal.put(\"group\", group);\n    }\n\n    String logonType = text(principalBlock.get(\"logontype\"));\n    if (logonType != null) {\n      HashMap logon = new HashMap();\n      logon.put(\"type\", logonType);\n      principal.put(\"logon\", logon);\n    }\n\n    putText(principal, \"run_level\", principalBlock, \"runlevel\");\n    if (principal.size() > 0) {\n      principals.add(principal);\n    }\n  }\n  return principals;\n}\n\nHashMap normalizeSettings(def task) {\n  def settingsBlock = asMap(task.get(\"settings\"));\n  if (settingsBlock == null) {\n    return null;\n  }\n\n  HashMap settings = new HashMap();\n  putBool(settings, \"enabled\", settingsBlock, \"enabled\");\n  putBool(settings, \"hidden\", settingsBlock, \"hidden\");\n  return settings;\n}\n\nArrayList normalizeTriggers(def task, def triggerKeys, def triggerTypes, def definitionKeys) {\n  def triggersBlock = asMap(task.get(\"triggers\"));\n  if (triggersBlock == null) {\n    return null;\n  }\n\n  ArrayList triggers = new ArrayList();\n  // decode_xml groups child elements by name. Same-name elements remain ordered\n  // lists, but cross-type XML order is not available after agent-side decoding.\n  for (def triggerKey : triggerKeys) {\n    String type = triggerTypes.get(triggerKey);\n    for (def triggerValue : asList(triggersBlock.get(triggerKey))) {\n      def triggerBlock = asMap(triggerValue);\n      // Empty XML trigger elements decode as empty scalar strings. Ignore\n      // non-empty scalars because they cannot represent a valid trigger.\n      if (triggerBlock == null && text(triggerValue) != null) {\n        continue;\n      }\n\n      HashMap trigger = new HashMap();\n      trigger.put(\"type\", type);\n      // Task Scheduler defaults an omitted trigger Enabled element to true.\n      putBoolDefault(trigger, \"enabled\", triggerBlock, \"enabled\", true);\n\n      // decode_xml represents empty elements (for example, <BootTrigger />)\n      // as scalar strings. Their presence still defines a trigger, even when\n      // there are no optional fields to extract from a map.\n      if (triggerBlock != null) {\n        def repetitionBlock = asMap(triggerBlock.get(\"repetition\"));\n        if (repetitionBlock != null) {\n          HashMap repetition = new HashMap();\n          putText(repetition, \"interval\", repetitionBlock, \"interval\");\n          putText(repetition, \"duration\", repetitionBlock, \"duration\");\n          putBool(repetition, \"stop_at_duration_end\", repetitionBlock, \"stopatdurationend\");\n          if (repetition.size() > 0) {\n            trigger.put(\"repetition\", repetition);\n          }\n        }\n      }\n      HashMap definitionValues = new HashMap();\n      definitionValues.put(\"type\", trigger.get(\"type\"));\n      definitionValues.put(\"enabled\", trigger.get(\"enabled\"));\n      def repetition = asMap(trigger.get(\"repetition\"));\n      if (repetition != null) {\n        for (def key : [\"interval\", \"duration\", \"stop_at_duration_end\"]) {\n          if (repetition.containsKey(key)) {\n            definitionValues.put(key, repetition.get(key));\n          }\n        }\n      }\n      trigger.put(\"definition\", canonicalDefinition(definitionValues, definitionKeys));\n      triggers.add(trigger);\n    }\n  }\n  return triggers;\n}\n\nArrayList normalizeActions(def task, def actionKeys, def actionTypes, def definitionKeys) {\n  def actionsBlock = asMap(task.get(\"actions\"));\n  if (actionsBlock == null) {\n    return null;\n  }\n\n  ArrayList actions = new ArrayList();\n  String context = text(actionsBlock.get(\"context\"));\n  // Use a stable type order because decode_xml exposes actions by element name.\n  // Repeated same-name actions are still emitted in their decoded list order.\n  for (def actionKey : actionKeys) {\n    String type = actionTypes.get(actionKey);\n    for (def actionValue : asList(actionsBlock.get(actionKey))) {\n      def actionBlock = asMap(actionValue);\n      if (actionBlock == null) {\n        continue;\n      }\n\n      HashMap action = new HashMap();\n      action.put(\"type\", type);\n      if (context != null) {\n        action.put(\"context\", context);\n      }\n\n      if (type.equals(\"exec\")) {\n        putText(action, \"command\", actionBlock, \"command\");\n        putText(action, \"arguments\", actionBlock, \"arguments\");\n        putText(action, \"working_directory\", actionBlock, \"workingdirectory\");\n      } else if (type.equals(\"com_handler\")) {\n        putText(action, \"class_id\", actionBlock, \"classid\");\n      }\n      action.put(\"definition\", canonicalDefinition(action, definitionKeys));\n      actions.add(action);\n    }\n  }\n  return actions;\n}\n\nif (ctx.event == null || ctx.winlog == null || !(ctx.winlog._tmp?.scheduled_task instanceof Map)) {\n  return;\n}\n\nString code = ctx.event.code == null ? null : ctx.event.code.toString();\nif (code == null || ![\"4698\", \"4699\", \"4700\", \"4701\", \"4702\"].contains(code)) {\n  return;\n}\n\ndef tmp = ctx.winlog._tmp.scheduled_task;\ndef decoded = code.equals(\"4702\") ? tmp.task_content_new : tmp.task_content;\ndef decodedMap = asMap(decoded);\nif (decodedMap == null) {\n  return;\n}\n\n// decode_xml lowercases names and uses local element/attribute names.\ndef task = asMap(decodedMap.get(\"task\"));\nif (task == null) {\n  task = decodedMap;\n}\n\nHashMap normalized = new HashMap();\n\ndef registrationInfo = asMap(task.get(\"registrationinfo\"));\nputText(normalized, \"uri\", registrationInfo, \"uri\");\n\nArrayList principals = normalizePrincipals(task);\nif (principals != null && principals.size() > 0) {\n  normalized.put(\"principals\", principals);\n}\n\nHashMap settings = normalizeSettings(task);\nif (settings != null && settings.size() > 0) {\n  normalized.put(\"settings\", settings);\n}\n\nArrayList triggers = normalizeTriggers(\n  task,\n  params.trigger_keys,\n  params.trigger_types,\n  params.trigger_definition_keys\n);\nif (triggers != null && triggers.size() > 0) {\n  normalized.put(\"triggers\", triggers);\n}\n\nArrayList actions = normalizeActions(\n  task,\n  params.action_keys,\n  params.action_types,\n  params.action_definition_keys\n);\nif (actions != null && actions.size() > 0) {\n  normalized.put(\"actions\", actions);\n}\n\n// Publish only after every section has normalized successfully.\nif (normalized.size() > 0) {\n  ctx.winlog.scheduled_task = normalized;\n}\n
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan_params(
                            event,
                            cached_painless!(
                                r##"def asMap(def value) {\n  return value instanceof Map ? value : null;\n}\n\nArrayList asList(def value) {\n  ArrayList values = new ArrayList();\n  if (value == null) {\n    return values;\n  }\n  if (value instanceof List) {\n    for (def item : value) {\n      values.add(item);\n    }\n    return values;\n  }\n  values.add(value);\n  return values;\n}\n\nString text(def value) {\n  if (value == null) {\n    return null;\n  }\n  def raw = value;\n  if (value instanceof Map) {\n    raw = value.get(\"#text\");\n  }\n  if (raw == null) {\n    return null;\n  }\n  String s = raw.toString();\n  return s.length() == 0 ? null : s;\n}\n\ndef boolValue(def value) {\n  String s = text(value);\n  if (s == null) {\n    return null;\n  }\n  String v = s.toLowerCase();\n  if (v.equals(\"true\") || v.equals(\"1\")) {\n    return true;\n  }\n  if (v.equals(\"false\") || v.equals(\"0\")) {\n    return false;\n  }\n  return null;\n}\n\nvoid putText(HashMap target, String field, def source, String key) {\n  if (!(source instanceof Map)) {\n    return;\n  }\n  String value = text(source.get(key));\n  if (value != null) {\n    target.put(field, value);\n  }\n}\n\nvoid putBool(HashMap target, String field, def source, String key) {\n  if (!(source instanceof Map)) {\n    return;\n  }\n  def value = boolValue(source.get(key));\n  if (value != null) {\n    target.put(field, value);\n  }\n}\n\nvoid putBoolDefault(HashMap target, String field, def source, String key, boolean defaultValue) {\n  if (!(source instanceof Map) || !source.containsKey(key)) {\n    target.put(field, defaultValue);\n    return;\n  }\n  def value = boolValue(source.get(key));\n  if (value != null) {\n    target.put(field, value);\n  }\n}\n\nString escapeDefinitionValue(String value) {\n  String hex = \"0123456789ABCDEF\";\n  StringBuilder escaped = new StringBuilder();\n  for (int i = 0; i < value.length(); i++) {\n    int code = value.charAt(i);\n    // Escape %, |, =, C0 control characters, and DEL.\n    if (code == 37 || code == 124 || code == 61 || code < 32 || code == 127) {\n      escaped.append(\"%\");\n      escaped.append(hex.charAt((code >> 4) & 15));\n      escaped.append(hex.charAt(code & 15));\n    } else {\n      escaped.append(value.charAt(i));\n    }\n  }\n  return escaped.toString();\n}\n\nString canonicalDefinition(HashMap values, def keys) {\n  StringBuilder definition = new StringBuilder(\"v=1\");\n  for (def keyValue : keys) {\n    String key = keyValue.toString();\n    definition.append(\"|\");\n    definition.append(key);\n    definition.append(\"=\");\n    if (!values.containsKey(key) || values.get(key) == null) {\n      continue;\n    }\n    definition.append(escapeDefinitionValue(values.get(key).toString()));\n  }\n  return definition.toString();\n}\n\nArrayList normalizePrincipals(def task) {\n  def principalsBlock = asMap(task.get(\"principals\"));\n  if (principalsBlock == null) {\n    return null;\n  }\n\n  ArrayList principals = new ArrayList();\n  for (def principalValue : asList(principalsBlock.get(\"principal\"))) {\n    def principalBlock = asMap(principalValue);\n    if (principalBlock == null) {\n      continue;\n    }\n\n    HashMap principal = new HashMap();\n    putText(principal, \"id\", principalBlock, \"id\");\n\n    String userId = text(principalBlock.get(\"userid\"));\n    if (userId != null) {\n      HashMap user = new HashMap();\n      user.put(\"identifier\", userId);\n      principal.put(\"user\", user);\n    }\n\n    String groupId = text(principalBlock.get(\"groupid\"));\n    if (groupId != null) {\n      HashMap group = new HashMap();\n      group.put(\"identifier\", groupId);\n      principal.put(\"group\", group);\n    }\n\n    String logonType = text(principalBlock.get(\"logontype\"));\n    if (logonType != null) {\n      HashMap logon = new HashMap();\n      logon.put(\"type\", logonType);\n      principal.put(\"logon\", logon);\n    }\n\n    putText(principal, \"run_level\", principalBlock, \"runlevel\");\n    if (principal.size() > 0) {\n      principals.add(principal);\n    }\n  }\n  return principals;\n}\n\nHashMap normalizeSettings(def task) {\n  def settingsBlock = asMap(task.get(\"settings\"));\n  if (settingsBlock == null) {\n    return null;\n  }\n\n  HashMap settings = new HashMap();\n  putBool(settings, \"enabled\", settingsBlock, \"enabled\");\n  putBool(settings, \"hidden\", settingsBlock, \"hidden\");\n  return settings;\n}\n\nArrayList normalizeTriggers(def task, def triggerKeys, def triggerTypes, def definitionKeys) {\n  def triggersBlock = asMap(task.get(\"triggers\"));\n  if (triggersBlock == null) {\n    return null;\n  }\n\n  ArrayList triggers = new ArrayList();\n  // decode_xml groups child elements by name. Same-name elements remain ordered\n  // lists, but cross-type XML order is not available after agent-side decoding.\n  for (def triggerKey : triggerKeys) {\n    String type = triggerTypes.get(triggerKey);\n    for (def triggerValue : asList(triggersBlock.get(triggerKey))) {\n      def triggerBlock = asMap(triggerValue);\n      // Empty XML trigger elements decode as empty scalar strings. Ignore\n      // non-empty scalars because they cannot represent a valid trigger.\n      if (triggerBlock == null && text(triggerValue) != null) {\n        continue;\n      }\n\n      HashMap trigger = new HashMap();\n      trigger.put(\"type\", type);\n      // Task Scheduler defaults an omitted trigger Enabled element to true.\n      putBoolDefault(trigger, \"enabled\", triggerBlock, \"enabled\", true);\n\n      // decode_xml represents empty elements (for example, <BootTrigger />)\n      // as scalar strings. Their presence still defines a trigger, even when\n      // there are no optional fields to extract from a map.\n      if (triggerBlock != null) {\n        def repetitionBlock = asMap(triggerBlock.get(\"repetition\"));\n        if (repetitionBlock != null) {\n          HashMap repetition = new HashMap();\n          putText(repetition, \"interval\", repetitionBlock, \"interval\");\n          putText(repetition, \"duration\", repetitionBlock, \"duration\");\n          putBool(repetition, \"stop_at_duration_end\", repetitionBlock, \"stopatdurationend\");\n          if (repetition.size() > 0) {\n            trigger.put(\"repetition\", repetition);\n          }\n        }\n      }\n      HashMap definitionValues = new HashMap();\n      definitionValues.put(\"type\", trigger.get(\"type\"));\n      definitionValues.put(\"enabled\", trigger.get(\"enabled\"));\n      def repetition = asMap(trigger.get(\"repetition\"));\n      if (repetition != null) {\n        for (def key : [\"interval\", \"duration\", \"stop_at_duration_end\"]) {\n          if (repetition.containsKey(key)) {\n            definitionValues.put(key, repetition.get(key));\n          }\n        }\n      }\n      trigger.put(\"definition\", canonicalDefinition(definitionValues, definitionKeys));\n      triggers.add(trigger);\n    }\n  }\n  return triggers;\n}\n\nArrayList normalizeActions(def task, def actionKeys, def actionTypes, def definitionKeys) {\n  def actionsBlock = asMap(task.get(\"actions\"));\n  if (actionsBlock == null) {\n    return null;\n  }\n\n  ArrayList actions = new ArrayList();\n  String context = text(actionsBlock.get(\"context\"));\n  // Use a stable type order because decode_xml exposes actions by element name.\n  // Repeated same-name actions are still emitted in their decoded list order.\n  for (def actionKey : actionKeys) {\n    String type = actionTypes.get(actionKey);\n    for (def actionValue : asList(actionsBlock.get(actionKey))) {\n      def actionBlock = asMap(actionValue);\n      if (actionBlock == null) {\n        continue;\n      }\n\n      HashMap action = new HashMap();\n      action.put(\"type\", type);\n      if (context != null) {\n        action.put(\"context\", context);\n      }\n\n      if (type.equals(\"exec\")) {\n        putText(action, \"command\", actionBlock, \"command\");\n        putText(action, \"arguments\", actionBlock, \"arguments\");\n        putText(action, \"working_directory\", actionBlock, \"workingdirectory\");\n      } else if (type.equals(\"com_handler\")) {\n        putText(action, \"class_id\", actionBlock, \"classid\");\n      }\n      action.put(\"definition\", canonicalDefinition(action, definitionKeys));\n      actions.add(action);\n    }\n  }\n  return actions;\n}\n\nif (ctx.event == null || ctx.winlog == null || !(ctx.winlog._tmp?.scheduled_task instanceof Map)) {\n  return;\n}\n\nString code = ctx.event.code == null ? null : ctx.event.code.toString();\nif (code == null || ![\"4698\", \"4699\", \"4700\", \"4701\", \"4702\"].contains(code)) {\n  return;\n}\n\ndef tmp = ctx.winlog._tmp.scheduled_task;\ndef decoded = code.equals(\"4702\") ? tmp.task_content_new : tmp.task_content;\ndef decodedMap = asMap(decoded);\nif (decodedMap == null) {\n  return;\n}\n\n// decode_xml lowercases names and uses local element/attribute names.\ndef task = asMap(decodedMap.get(\"task\"));\nif (task == null) {\n  task = decodedMap;\n}\n\nHashMap normalized = new HashMap();\n\ndef registrationInfo = asMap(task.get(\"registrationinfo\"));\nputText(normalized, \"uri\", registrationInfo, \"uri\");\n\nArrayList principals = normalizePrincipals(task);\nif (principals != null && principals.size() > 0) {\n  normalized.put(\"principals\", principals);\n}\n\nHashMap settings = normalizeSettings(task);\nif (settings != null && settings.size() > 0) {\n  normalized.put(\"settings\", settings);\n}\n\nArrayList triggers = normalizeTriggers(\n  task,\n  params.trigger_keys,\n  params.trigger_types,\n  params.trigger_definition_keys\n);\nif (triggers != null && triggers.size() > 0) {\n  normalized.put(\"triggers\", triggers);\n}\n\nArrayList actions = normalizeActions(\n  task,\n  params.action_keys,\n  params.action_types,\n  params.action_definition_keys\n);\nif (actions != null && actions.size() > 0) {\n  normalized.put(\"actions\", actions);\n}\n\n// Publish only after every section has normalized successfully.\nif (normalized.size() > 0) {\n  ctx.winlog.scheduled_task = normalized;\n}\n"##
                            ),
                            cached_params!(
                                "{\"action_definition_keys\":[\"type\",\"context\",\"command\",\"arguments\",\"working_directory\",\"class_id\"],\"action_keys\":[\"exec\",\"comhandler\"],\"action_types\":{\"comhandler\":\"com_handler\",\"exec\":\"exec\"},\"trigger_definition_keys\":[\"type\",\"enabled\",\"interval\",\"duration\",\"stop_at_duration_end\"],\"trigger_keys\":[\"boottrigger\",\"wnfstatechangetrigger\",\"timetrigger\",\"calendartrigger\",\"logontrigger\",\"registrationtrigger\",\"sessionstatechangetrigger\",\"eventtrigger\",\"idletrigger\"],\"trigger_types\":{\"boottrigger\":\"boot\",\"calendartrigger\":\"calendar\",\"eventtrigger\":\"event\",\"idletrigger\":\"idle\",\"logontrigger\":\"logon\",\"registrationtrigger\":\"registration\",\"sessionstatechangetrigger\":\"session_state_change\",\"timetrigger\":\"time\",\"wnfstatechangetrigger\":\"wnf_state_change\"}}"
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "normalize_decoded_scheduled_task_xml",
                        )?;
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "tags",
                                json!("scheduled_task_normalization_failed"),
                            )?;
                            Ok(())
                        })();
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // End nested pipeline: "security_scheduled_task"
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("winlog._tmp.scheduled_task");
                    Ok(())
                })();
                let _cond = {
                    event.get("winlog._tmp").is_some_and(|v| v.is_object()) && event.get("winlog._tmp").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("winlog._tmp");
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("winlog.provider_name")
                        == Some("Microsoft-Windows-Security-Auditing")
                        && event
                            .get("winlog.event_data")
                            .is_some_and(|v| v.is_object())
                        && event.has_value("winlog.event_data.TaskName")
                        && event.has_value("event.code")
                        && ["4698", "4699", "4700", "4701", "4702"]
                            .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    if let Some(v) = event
                        .get("winlog.event_data.TaskName")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("winlog.scheduled_task.name", v)?;
                    }
                }
                if event.has_value("source.ip") {
                    if let Some(s) = event.get_string("source.ip") {
                        let re = cached_regex!(
                            "^\\[?::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)(?:\\](?::[0-9]+)?)?$"
                        );
                        let replaced = re.replace_all(&s, "$1").into_owned();
                        event.set("source.ip", replaced)?;
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
                if event.has("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                let _cond =
                    { event.has_value("source.ip") && event.get_str("source.ip") != Some("-") };
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
                    // on_failure: 3 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("winlog.time_created") {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["ISO8601"], None, None)
                            {
                                event.set("@timestamp", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "time_created_date")?;
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.remove("winlog.time_created").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "winlog.time_created".into(),
                                });
                            }
                            Ok(())
                        })();
                        event.append(
                            "error.message",
                            json!(format!(
                                "fail-{}",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        return Err(TransformError::ParseError {
                            path: "_fail".into(),
                            message: (format!(
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
                            ))
                            .to_string(),
                        });
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
                let _cond = {
                    event.has_value("winlog.event_data") && event.get("winlog.event_data").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("winlog.event_data");
                        Ok(())
                    })();
                }
                // End nested pipeline: "security_default"
            }

            let _cond = {
                event.get("winlog.channel").is_some_and(|v| v.is_string())
                    && event
                        .get_str("winlog.channel")
                        .is_some_and(|s| s.to_lowercase() == "windows powershell")
            };
            if _cond {
                // Begin nested pipeline: "powershell"
                let _cond = { event.get_str("winlog.event_id") == Some("800") };
                if _cond {
                    if let Some(kv_str) = event.get_string("winlog.event_data.param2") {
                        for pair in kv_str.split("\n\t") {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "winlog.event_data.param2".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                let key = key.trim_matches(|c| "\n\t".contains(c));
                                let value = value.trim_matches(|c| "\n\t".contains(c));
                                if !key.is_empty() {
                                    event.set(&format!("winlog.event_data.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                let _cond = {
                    event.get_str("winlog.event_id") != Some("800")
                        && event.has_value("winlog.event_data.param3")
                };
                if _cond {
                    // Painless script
                    // Source: def p = ctx.winlog?.event_data[params[\"field\"]];\n// Define the pattern that will match all keys\ndef pat = /(^|(^[\\n]?))?\\t([^\\s\\W]+)=/m;\ndef m = pat.matcher(p);\n\n// we position ourselves in the first matching key\nm.find();\ndef key = m.group(3).trim();\ndef previousEnd = m.end();\n\n// while new keys are found, we add everything between one key and the next\n// as the value, regardless of its contents\nwhile(m.find())\n{\n    ctx.winlog.event_data[key] = p.substring(previousEnd, m.start()).trim();\n    previousEnd = m.end();\n    key = m.group(3).trim();\n}\n\n// add remaining value\nctx.winlog.event_data[key] = p.substring(previousEnd).trim();
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def p = ctx.winlog?.event_data[params[\"field\"]];\n// Define the pattern that will match all keys\ndef pat = /(^|(^[\\n]?))?\\t([^\\s\\W]+)=/m;\ndef m = pat.matcher(p);\n\n// we position ourselves in the first matching key\nm.find();\ndef key = m.group(3).trim();\ndef previousEnd = m.end();\n\n// while new keys are found, we add everything between one key and the next\n// as the value, regardless of its contents\nwhile(m.find())\n{\n    ctx.winlog.event_data[key] = p.substring(previousEnd, m.start()).trim();\n    previousEnd = m.end();\n    key = m.group(3).trim();\n}\n\n// add remaining value\nctx.winlog.event_data[key] = p.substring(previousEnd).trim();"#
                        ),
                        cached_params!("{\"field\":\"param3\"}"),
                    )?;
                }
                event.set("ecs.version", json!("8.17.0"))?;
                let _cond = { event.has_value("winlog.event_data") };
                if _cond {
                    // Painless script
                    // Source: ctx.winlog?.event_data?.entrySet().removeIf(entry -> [null, \"\", \"-\", \"{00000000-0000-0000-0000-000000000000}\"].contains(entry.getValue()))
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.winlog?.event_data?.entrySet().removeIf(entry -> [null, \"\", \"-\", \"{00000000-0000-0000-0000-000000000000}\"].contains(entry.getValue()))"#
                        ),
                    )?;
                }
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
                    // on_failure: 3 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("winlog.time_created") {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["ISO8601"], None, None)
                            {
                                event.set("@timestamp", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "time_created_date")?;
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.remove("winlog.time_created").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "winlog.time_created".into(),
                                });
                            }
                            Ok(())
                        })();
                        event.append(
                            "error.message",
                            json!(format!(
                                "fail-{}",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        return Err(TransformError::ParseError {
                            path: "_fail".into(),
                            message: (format!(
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
                            ))
                            .to_string(),
                        });
                    }
                }
                event.set("event.kind", json!("event"))?;
                event.set(
                    "event.code",
                    json!(
                        event
                            .get("winlog.event_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.category", Value::Array(vec![json!("process")]))?;
                let _cond = { event.get_str("event.code") == Some("400") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = { event.get_str("event.code") == Some("403") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = { !event.has_value("event.type") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.SequenceNumber") {
                        if let Some(val) = event.get("winlog.event_data.SequenceNumber") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.SequenceNumber".into(),
                                    message,
                                }
                            })?;
                            event.set("event.sequence", converted)?;
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
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
                    Ok(())
                })();
                let _cond = { event.get_str("winlog.event_data.HostId") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.HostId") {
                            event.rename("winlog.event_data.HostId", "process.entity_id")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.HostApplication") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.HostApplication") {
                            event.rename(
                                "winlog.event_data.HostApplication",
                                "process.command_line",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.HostName") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.HostName") {
                            event.rename("winlog.event_data.HostName", "process.title")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("winlog.event_data.UserId") };
                if _cond {
                    if let Some(s) = event.get_string("winlog.event_data.UserId") {
                        let parts: Vec<Value> = cached_regex!("\\\\")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        event.set("_temp.user_parts", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("_temp.user_parts") && event.get("_temp.user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        let v = json!(
                            event
                                .get("_temp.user_parts.0")
                                .map_or_else(String::new, template_to_string)
                        );
                        if !painless_is_empty_value(&v) {
                            event.set("user.domain", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("_temp.user_parts") && event.get("_temp.user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        let v = json!(
                            event
                                .get("_temp.user_parts.1")
                                .map_or_else(String::new, template_to_string)
                        );
                        if !painless_is_empty_value(&v) {
                            event.set("user.name", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("user.name") };
                if _cond {
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
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data._MemberUserName") {
                        event.rename("winlog.event_data._MemberUserName", "user.name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data._MemberDomain") {
                        event.rename("winlog.event_data._MemberDomain", "user.domain")?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("winlog.event_data._MemberAccountType") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.roles",
                            json!(
                                event
                                    .get("winlog.event_data._MemberAccountType")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("user.roles")
                        && event.has_value("winlog.event_data._MemberAccountType")
                        && event.get("user.roles").is_some_and(|v| {
                            match (v, event.get("winlog.event_data._MemberAccountType")) {
                                (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
                                _ => false,
                            }
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("winlog.event_data._MemberAccountType");
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.NewEngineState") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.NewEngineState") {
                            event.rename(
                                "winlog.event_data.NewEngineState",
                                "powershell.engine.new_state",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.PreviousEngineState") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.PreviousEngineState") {
                            event.rename(
                                "winlog.event_data.PreviousEngineState",
                                "powershell.engine.previous_state",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.NewProviderState") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.NewProviderState") {
                            event.rename(
                                "winlog.event_data.NewProviderState",
                                "powershell.provider.new_state",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.ProviderName") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ProviderName") {
                            event.rename(
                                "winlog.event_data.ProviderName",
                                "powershell.provider.name",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.DetailTotal") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.DetailTotal") {
                            if let Some(val) = event.get("winlog.event_data.DetailTotal") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "winlog.event_data.DetailTotal".into(),
                                        message,
                                    }
                                })?;
                                event.set("powershell.total", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.DetailSequence") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.DetailSequence") {
                            if let Some(val) = event.get("winlog.event_data.DetailSequence") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "winlog.event_data.DetailSequence".into(),
                                        message,
                                    }
                                })?;
                                event.set("powershell.sequence", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.EngineVersion") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.EngineVersion") {
                            event.rename(
                                "winlog.event_data.EngineVersion",
                                "powershell.engine.version",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.PipelineId") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.PipelineId") {
                            event
                                .rename("winlog.event_data.PipelineId", "powershell.pipeline_id")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.RunspaceId") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.RunspaceId") {
                            event
                                .rename("winlog.event_data.RunspaceId", "powershell.runspace_id")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.HostVersion") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.HostVersion") {
                            event.rename(
                                "winlog.event_data.HostVersion",
                                "powershell.process.executable_version",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.CommandLine") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.CommandLine") {
                            event.rename(
                                "winlog.event_data.CommandLine",
                                "powershell.command.value",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.CommandPath") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.CommandPath") {
                            event.rename(
                                "winlog.event_data.CommandPath",
                                "powershell.command.path",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.CommandName") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.CommandName") {
                            event.rename(
                                "winlog.event_data.CommandName",
                                "powershell.command.name",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.CommandType") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.CommandType") {
                            event.rename(
                                "winlog.event_data.CommandType",
                                "powershell.command.type",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") == Some("800") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.param3") {
                            if let Some(s) = event.get_string("winlog.event_data.param3") {
                                let parts: Vec<Value> = s.split("\n").map(|p| json!(p)).collect();
                                event.set("winlog.event_data.param3", Value::Array(parts))?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") == Some("800") };
                if _cond {
                    // Painless script
                    // Source: def parseRawDetail(String raw) {\n    Pattern detailRegex = /^([^:(]+)\\(([^)]+)\\)\\:\\s*(.+)?$/;\n    Pattern parameterBindingRegex = /name\\=(.+);\\s*value\\=(.+)$/;\n\n    def matcher = detailRegex.matcher(raw);\n    if (!matcher.matches()) {\n        return [\"value\": raw];\n    }\n    def matches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        matches.add(matcher.group(i));\n    }\n    \n    if (matches.length != 4) {\n        return [\"value\": raw];\n    }                \n    \n    if (matches[1] != \"ParameterBinding\") {\n        return [\n            \"type\": matches[1], \n            \"related_command\": matches[2], \n            \"value\": matches[3]\n        ];\n    }\n\n    matcher = parameterBindingRegex.matcher(matches[3]);\n    if (!matcher.matches()) {\n        return [\"value\": matches[4]];\n    }\n    def nameValMatches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        nameValMatches.add(matcher.group(i));\n    }\n    if (nameValMatches.length !== 3) {\n        return [\"value\": matches[3]];\n    }\n\n    return [\n        \"type\": matches[1],\n        \"related_command\": matches[2],\n        \"name\": nameValMatches[1],\n        \"value\": nameValMatches[2]\n    ];\n}\n\nif (ctx._temp == null) {\n    ctx._temp = new HashMap();\n}\n\nif (ctx._temp.details == null) {\n    ctx._temp.details = new ArrayList();\n}\n\ndef values = ctx.winlog?.event_data[params[\"field\"]];\nif (values != null && values.length > 0) {\n    for (v in values) {\n        ctx._temp.details.add(parseRawDetail(v));\n    }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def parseRawDetail(String raw) {\n    Pattern detailRegex = /^([^:(]+)\\(([^)]+)\\)\\:\\s*(.+)?$/;\n    Pattern parameterBindingRegex = /name\\=(.+);\\s*value\\=(.+)$/;\n\n    def matcher = detailRegex.matcher(raw);\n    if (!matcher.matches()) {\n        return [\"value\": raw];\n    }\n    def matches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        matches.add(matcher.group(i));\n    }\n    \n    if (matches.length != 4) {\n        return [\"value\": raw];\n    }                \n    \n    if (matches[1] != \"ParameterBinding\") {\n        return [\n            \"type\": matches[1], \n            \"related_command\": matches[2], \n            \"value\": matches[3]\n        ];\n    }\n\n    matcher = parameterBindingRegex.matcher(matches[3]);\n    if (!matcher.matches()) {\n        return [\"value\": matches[4]];\n    }\n    def nameValMatches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        nameValMatches.add(matcher.group(i));\n    }\n    if (nameValMatches.length !== 3) {\n        return [\"value\": matches[3]];\n    }\n\n    return [\n        \"type\": matches[1],\n        \"related_command\": matches[2],\n        \"name\": nameValMatches[1],\n        \"value\": nameValMatches[2]\n    ];\n}\n\nif (ctx._temp == null) {\n    ctx._temp = new HashMap();\n}\n\nif (ctx._temp.details == null) {\n    ctx._temp.details = new ArrayList();\n}\n\ndef values = ctx.winlog?.event_data[params[\"field\"]];\nif (values != null && values.length > 0) {\n    for (v in values) {\n        ctx._temp.details.add(parseRawDetail(v));\n    }\n}"#
                        ),
                        cached_params!("{\"field\":\"param3\"}"),
                    )?;
                }
                let _cond = {
                    event.has_value("_temp.details") && event.get("_temp.details").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
                };
                if _cond {
                    event.rename("_temp.details", "powershell.command.invocation_details")?;
                }
                let _cond = {
                    event.has_value("process.command_line")
                        && event.get_str("process.command_line") != Some("")
                };
                if _cond {
                    // Painless script
                    // Source: // appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string cmd into next\n// argument and command line remainder.\ndef readNextArg(String cmd) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; cmd.length() > 0; cmd = cmd.substring(1)) {\n        def c = cmd.charAt(0);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"rest\": cmd.substring(1)\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && cmd.length() > 1 && cmd.charAt(1) == (char)'\"') {\n                    b.append(c);\n                    cmd = cmd.substring(1);\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"rest\": ''\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String cmd) {\n    def args = new ArrayList();\n    while (cmd.length() > 0) {\n        if (cmd.charAt(0) == (char)' ' || cmd.charAt(0) == (char)0x09) {\n            cmd = cmd.substring(1);\n            continue;\n        }\n        def next = readNextArg(cmd);\n        cmd = next.rest;\n        args.add(next.arg);\n    }\n    return args;\n}\n\nctx.process.args = commandLineToArgv(ctx.process.command_line);\nctx.process.args_count = ctx.process.args.length;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"// appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string cmd into next\n// argument and command line remainder.\ndef readNextArg(String cmd) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; cmd.length() > 0; cmd = cmd.substring(1)) {\n        def c = cmd.charAt(0);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"rest\": cmd.substring(1)\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && cmd.length() > 1 && cmd.charAt(1) == (char)'\"') {\n                    b.append(c);\n                    cmd = cmd.substring(1);\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"rest\": ''\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String cmd) {\n    def args = new ArrayList();\n    while (cmd.length() > 0) {\n        if (cmd.charAt(0) == (char)' ' || cmd.charAt(0) == (char)0x09) {\n            cmd = cmd.substring(1);\n            continue;\n        }\n        def next = readNextArg(cmd);\n        cmd = next.rest;\n        args.add(next.arg);\n    }\n    return args;\n}\n\nctx.process.args = commandLineToArgv(ctx.process.command_line);\nctx.process.args_count = ctx.process.args.length;"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("winlog.event_data.ScriptName")
                        && event
                            .get_as_string("winlog.event_data.ScriptName")
                            .is_some_and(|s| s.len() > 1)
                };
                if _cond {
                    // Painless script
                    // Source: def path = ctx.winlog.event_data.ScriptName;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def path = ctx.winlog.event_data.ScriptName;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}"#
                        ),
                    )?;
                }
                let _cond = { event.get_str("winlog.event_data.ScriptName") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ScriptName") {
                            event.rename("winlog.event_data.ScriptName", "file.path")?;
                        }
                        Ok(())
                    })();
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("_temp");
                    event.remove("winlog.event_data.param1");
                    event.remove("winlog.event_data.param2");
                    event.remove("winlog.event_data.param3");
                    event.remove("winlog.event_data.SequenceNumber");
                    event.remove("winlog.event_data.DetailTotal");
                    event.remove("winlog.event_data.DetailSequence");
                    event.remove("winlog.event_data.UserId");
                    event.remove("winlog.time_created");
                    event.remove("winlog.level");
                    Ok(())
                })();
                let _cond = {
                    event.has_value("winlog.event_data") && event.get("winlog.event_data").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("winlog.event_data");
                        Ok(())
                    })();
                }
                // End nested pipeline: "powershell"
            }

            let _cond = {
                event.get("winlog.channel").is_some_and(|v| v.is_string())
                    && event.get_str("winlog.channel").is_some_and(|s| {
                        s.to_lowercase() == "microsoft-windows-powershell/operational"
                    })
            };
            if _cond {
                // Begin nested pipeline: "powershell_operational"
                let _cond = { event.get_str("winlog.event_id") == Some("4103") };
                if _cond {
                    if let Some(kv_str) = event.get_string("winlog.event_data.ContextInfo") {
                        for pair in cached_regex!("\\n(?!\\n)\\s+").split(&kv_str).into_iter() {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = ({
                                let parts = cached_regex!("[:=]").splitn(&pair, 2);
                                match (parts.first(), parts.get(1)) {
                                    (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                    _ => None,
                                }
                            }) else {
                                return Err(TransformError::ParseError {
                                    path: "winlog.event_data.ContextInfo".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                let key = key.trim_matches(|c| " \n\t".contains(c));
                                let value = value.trim_matches(|c| " \n\t".contains(c));
                                if !key.is_empty() {
                                    event.set(&format!("winlog.event_data.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                let _cond = { event.has_value("winlog.event_data") };
                if _cond {
                    // Painless script
                    // Source: def newEventData = new HashMap();\nfor (entry in ctx.winlog.event_data.entrySet()) {\n  def newKey = /\\s/.matcher(entry.getKey().toString()).replaceAll(\"\");\n  newEventData.put(newKey, entry.getValue());\n}\nctx.winlog.event_data = newEventData;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def newEventData = new HashMap();\nfor (entry in ctx.winlog.event_data.entrySet()) {\n  def newKey = /\\s/.matcher(entry.getKey().toString()).replaceAll(\"\");\n  newEventData.put(newKey, entry.getValue());\n}\nctx.winlog.event_data = newEventData;"#
                        ),
                    )?;
                }
                event.set("ecs.version", json!("8.17.0"))?;
                let _cond = { event.has_value("winlog.event_data") };
                if _cond {
                    // Painless script
                    // Source: ctx.winlog?.event_data?.entrySet().removeIf(entry -> [null, \"\", \"-\", \"{00000000-0000-0000-0000-000000000000}\"].contains(entry.getValue()))
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.winlog?.event_data?.entrySet().removeIf(entry -> [null, \"\", \"-\", \"{00000000-0000-0000-0000-000000000000}\"].contains(entry.getValue()))"#
                        ),
                    )?;
                }
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
                    // on_failure: 3 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("winlog.time_created") {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["ISO8601"], None, None)
                            {
                                event.set("@timestamp", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "time_created_date")?;
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.remove("winlog.time_created").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "winlog.time_created".into(),
                                });
                            }
                            Ok(())
                        })();
                        event.append(
                            "error.message",
                            json!(format!(
                                "fail-{}",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        return Err(TransformError::ParseError {
                            path: "_fail".into(),
                            message: (format!(
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
                            ))
                            .to_string(),
                        });
                    }
                }
                event.set("event.kind", json!("event"))?;
                event.set(
                    "event.code",
                    json!(
                        event
                            .get("winlog.event_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.category", Value::Array(vec![json!("process")]))?;
                let _cond = { event.get_str("event.code") == Some("4105") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = { event.get_str("event.code") == Some("4106") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = { !event.has_value("event.type") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.SequenceNumber") {
                        if let Some(val) = event.get("winlog.event_data.SequenceNumber") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.SequenceNumber".into(),
                                    message,
                                }
                            })?;
                            event.set("event.sequence", converted)?;
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
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
                    Ok(())
                })();
                let _cond = { event.get_str("winlog.event_data.HostID") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.HostID") {
                            event.rename("winlog.event_data.HostID", "process.entity_id")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.HostApplication") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.HostApplication") {
                            event.rename(
                                "winlog.event_data.HostApplication",
                                "process.command_line",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.HostName") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.HostName") {
                            event.rename("winlog.event_data.HostName", "process.title")?;
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.process.pid")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.pid", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.user.identifier")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.id", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.user.domain")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.domain", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.user.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("user.name") };
                if _cond {
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
                }
                let _cond = { event.has_value("user.domain") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("user.domain")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("winlog.event_data.ConnectedUser") };
                if _cond {
                    if let Some(s) = event.get_string("winlog.event_data.ConnectedUser") {
                        let parts: Vec<Value> = cached_regex!("\\\\")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        event.set("_temp.connected_user_parts", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("_temp.connected_user_parts") && event.get("_temp.connected_user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        let v = json!(
                            event
                                .get("_temp.connected_user_parts.0")
                                .map_or_else(String::new, template_to_string)
                        );
                        if !painless_is_empty_value(&v) {
                            event.set("source.user.domain", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("_temp.connected_user_parts") && event.get("_temp.connected_user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        let v = json!(
                            event
                                .get("_temp.connected_user_parts.1")
                                .map_or_else(String::new, template_to_string)
                        );
                        if !painless_is_empty_value(&v) {
                            event.set("source.user.name", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.user.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("source.user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.user") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("user.domain") {
                            event.rename("user.domain", "destination.user.domain")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.user") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("user.name") {
                            event.rename("user.name", "destination.user.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.user") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("source.user.domain")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("user.domain", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.user") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("source.user.name")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("user.name", v)?;
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data._MemberUserName") {
                        event.rename("winlog.event_data._MemberUserName", "user.name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data._MemberDomain") {
                        event.rename("winlog.event_data._MemberDomain", "user.domain")?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("winlog.event_data._MemberAccountType") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.roles",
                            json!(
                                event
                                    .get("winlog.event_data._MemberAccountType")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("user.roles")
                        && event.has_value("winlog.event_data._MemberAccountType")
                        && event.get("user.roles").is_some_and(|v| {
                            match (v, event.get("winlog.event_data._MemberAccountType")) {
                                (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
                                _ => false,
                            }
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("winlog.event_data._MemberAccountType");
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.MessageNumber") {
                        if let Some(val) = event.get("winlog.event_data.MessageNumber") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.MessageNumber".into(),
                                    message,
                                }
                            })?;
                            event.set("powershell.sequence", converted)?;
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("winlog.event_data.MessageTotal") {
                        if let Some(val) = event.get("winlog.event_data.MessageTotal") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "winlog.event_data.MessageTotal".into(),
                                    message,
                                }
                            })?;
                            event.set("powershell.total", converted)?;
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.get_str("winlog.event_data.ShellID") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ShellID") {
                            event.rename("winlog.event_data.ShellID", "powershell.id")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.EngineVersion") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.EngineVersion") {
                            event.rename(
                                "winlog.event_data.EngineVersion",
                                "powershell.engine.version",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.PipelineID") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.PipelineID") {
                            event
                                .rename("winlog.event_data.PipelineID", "powershell.pipeline_id")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.RunspaceID") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.RunspaceID") {
                            event
                                .rename("winlog.event_data.RunspaceID", "powershell.runspace_id")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.RunspaceId") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.RunspaceId") {
                            event
                                .rename("winlog.event_data.RunspaceId", "powershell.runspace_id")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.HostVersion") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.HostVersion") {
                            event.rename(
                                "winlog.event_data.HostVersion",
                                "powershell.process.executable_version",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.CommandLine") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.CommandLine") {
                            event.rename(
                                "winlog.event_data.CommandLine",
                                "powershell.command.value",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.CommandPath") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.CommandPath") {
                            event.rename(
                                "winlog.event_data.CommandPath",
                                "powershell.command.path",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.CommandName") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.CommandName") {
                            event.rename(
                                "winlog.event_data.CommandName",
                                "powershell.command.name",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.CommandType") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.CommandType") {
                            event.rename(
                                "winlog.event_data.CommandType",
                                "powershell.command.type",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.ScriptBlockId") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ScriptBlockId") {
                            event.rename(
                                "winlog.event_data.ScriptBlockId",
                                "powershell.file.script_block_id",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("winlog.event_data.ScriptBlockText") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ScriptBlockText") {
                            event.rename(
                                "winlog.event_data.ScriptBlockText",
                                "powershell.file.script_block_text",
                            )?;
                        }
                        Ok(())
                    })();
                }
                if event.has_value("powershell.file.script_block_text") {
                    if let Some(s) = event.get_string("powershell.file.script_block_text") {
                        let trimmed = s.trim().to_string();
                        event.set("powershell.file.script_block_text", trimmed)?;
                    }
                }
                if event.has_value("powershell.file.script_block_text") {
                    if let Some(s) = event.get_string("powershell.file.script_block_text") {
                        let re = cached_regex!("\\s");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("_temp.script_block_no_space", replaced)?;
                    }
                }
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("_temp.script_block_no_space") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set(
                            "powershell.file.script_block_hash",
                            json!(fingerprint_default(&values)),
                        )?;
                    }
                }
                if event.has_value("powershell.file.script_block_text") {
                    if let Some(s) = event.get_string("powershell.file.script_block_text") {
                        let re = cached_regex!("(?s)# SIG # Begin signature block.+");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("_temp.script_block_no_signature", replaced)?;
                    }
                }
                let _cond = { event.has_value("_temp.script_block_no_signature") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // Painless script
                        // Source: // Entropy Variance from: https://github.com/elastic/toutoumomoma/blob/be287c9c0d0e435572e3889a6584199983c688f0/toutoumomoma.go#L326-L363.\nString script = ctx._temp.script_block_no_signature;\n\nint length = script.length();\nif (length == 0) {\n  return;\n}\n\n// Skip signature only scripts:\n// - Inspect only line 2 (line 1 can be truncated mid-signature).\n// - Match \"# \" + base64-ish content at the fixed signature line length (64 chars).\nint lf = 10;    // '\\n'\nint cr = 13;    // '\\r'\nint hash = 35;  // '#'\nint space = 32; // ' '\nint sigLineLen = 64; // Content length, excluding \"# \" prefix.\n\nint firstLineEnd = -1;\nfor (int idx = 0; idx < length; idx++) {\n  if (script.charAt(idx) == lf) {\n    firstLineEnd = idx;\n    break;\n  }\n}\n\nif (firstLineEnd > 0) {\n  int secondLineStart = firstLineEnd + 1;\n  if (secondLineStart < length) {\n    int secondLineEnd = length;\n    for (int idx = secondLineStart; idx < length; idx++) {\n      if (script.charAt(idx) == lf) {\n        secondLineEnd = idx;\n        break;\n      }\n    }\n\n    if (secondLineEnd > secondLineStart && script.charAt(secondLineEnd - 1) == cr) {\n      secondLineEnd--;\n    }\n\n    if (secondLineStart < secondLineEnd && script.charAt(secondLineStart) == hash) {\n      int contentStart = secondLineStart + 1;\n      if (contentStart < secondLineEnd && script.charAt(contentStart) == space) {\n        contentStart++;\n      }\n      int lineLen = secondLineEnd - contentStart;\n      if (lineLen == sigLineLen) {\n        boolean base64Line = true;\n        for (int i = contentStart; i < secondLineEnd; i++) {\n          int c = (int) script.charAt(i);\n          if (!((c >= 65 && c <= 90) || (c >= 97 && c <= 122) ||\n                (c >= 48 && c <= 57) || c == 43 || c == 47 || c == 61)) {\n            base64Line = false;\n            break;\n          }\n        }\n        if (base64Line) {\n          return;\n        }\n      }\n    }\n  }\n}\n\nscript = java.text.Normalizer.normalize(script, java.text.Normalizer.Form.NFC);\n\nlength = script.length();\nif (length == 0) {\n  return;\n}\n\nint[] counts = new int[65536];\nint[] seen = new int[length];\nint seenCount = 0;\nint uniqueSymbols = 0;\nfor (int i = 0; i < length; i++) {\n    int ch = script.charAt(i);\n    if (counts[ch] == 0) {\n        counts[ch] = 1;\n        seen[seenCount++] = ch;\n        uniqueSymbols++;\n    } else {\n        counts[ch]++;\n    }\n}\n\ndouble invLog2 = 1.0 / Math.log(2.0);\ndouble entropy = 0.0;\ndouble surprisalVar = 0.0;\ndouble pSum = 0.0;\n\nfor (int i = 0; i < seenCount; i++) {\n    int code = seen[i];\n    double cnt = (double) counts[code];\n    double p = cnt / (double) length;\n    double l2p = Math.log(p) * invLog2;\n\n    pSum += p;\n    double tmp = entropy;\n    entropy = tmp + (p / pSum) * (l2p - tmp);\n    surprisalVar += p * (l2p - tmp) * (l2p - entropy);\n}\n\nsurprisalVar = Math.max(0.0, surprisalVar);\ndouble surprisalSd = Math.sqrt(surprisalVar);\ndouble entropyBits = -entropy;\nif (entropyBits == -0.0) {\n    entropyBits = 0.0;\n}\n\ndouble normalizedEntropy = 0.0;\nif (length > 1) {\n    double maxEntropy = Math.log((double) length) * invLog2; // max bits if every character is unique\n    normalizedEntropy = entropyBits / maxEntropy;           // scale 0..1 against script length\n    if (normalizedEntropy < 0.0) normalizedEntropy = 0.0;\n    else if (normalizedEntropy > 1.0) normalizedEntropy = 1.0;\n}\n\nctx.powershell.file.script_block_entropy_bits = entropyBits;\nctx.powershell.file.script_block_entropy_normalized = normalizedEntropy;\nctx.powershell.file.script_block_surprisal_stdev = surprisalSd;\nctx.powershell.file.script_block_length = length;\nctx.powershell.file.script_block_unique_symbols = uniqueSymbols;
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r##"// Entropy Variance from: https://github.com/elastic/toutoumomoma/blob/be287c9c0d0e435572e3889a6584199983c688f0/toutoumomoma.go#L326-L363.\nString script = ctx._temp.script_block_no_signature;\n\nint length = script.length();\nif (length == 0) {\n  return;\n}\n\n// Skip signature only scripts:\n// - Inspect only line 2 (line 1 can be truncated mid-signature).\n// - Match \"# \" + base64-ish content at the fixed signature line length (64 chars).\nint lf = 10;    // '\\n'\nint cr = 13;    // '\\r'\nint hash = 35;  // '#'\nint space = 32; // ' '\nint sigLineLen = 64; // Content length, excluding \"# \" prefix.\n\nint firstLineEnd = -1;\nfor (int idx = 0; idx < length; idx++) {\n  if (script.charAt(idx) == lf) {\n    firstLineEnd = idx;\n    break;\n  }\n}\n\nif (firstLineEnd > 0) {\n  int secondLineStart = firstLineEnd + 1;\n  if (secondLineStart < length) {\n    int secondLineEnd = length;\n    for (int idx = secondLineStart; idx < length; idx++) {\n      if (script.charAt(idx) == lf) {\n        secondLineEnd = idx;\n        break;\n      }\n    }\n\n    if (secondLineEnd > secondLineStart && script.charAt(secondLineEnd - 1) == cr) {\n      secondLineEnd--;\n    }\n\n    if (secondLineStart < secondLineEnd && script.charAt(secondLineStart) == hash) {\n      int contentStart = secondLineStart + 1;\n      if (contentStart < secondLineEnd && script.charAt(contentStart) == space) {\n        contentStart++;\n      }\n      int lineLen = secondLineEnd - contentStart;\n      if (lineLen == sigLineLen) {\n        boolean base64Line = true;\n        for (int i = contentStart; i < secondLineEnd; i++) {\n          int c = (int) script.charAt(i);\n          if (!((c >= 65 && c <= 90) || (c >= 97 && c <= 122) ||\n                (c >= 48 && c <= 57) || c == 43 || c == 47 || c == 61)) {\n            base64Line = false;\n            break;\n          }\n        }\n        if (base64Line) {\n          return;\n        }\n      }\n    }\n  }\n}\n\nscript = java.text.Normalizer.normalize(script, java.text.Normalizer.Form.NFC);\n\nlength = script.length();\nif (length == 0) {\n  return;\n}\n\nint[] counts = new int[65536];\nint[] seen = new int[length];\nint seenCount = 0;\nint uniqueSymbols = 0;\nfor (int i = 0; i < length; i++) {\n    int ch = script.charAt(i);\n    if (counts[ch] == 0) {\n        counts[ch] = 1;\n        seen[seenCount++] = ch;\n        uniqueSymbols++;\n    } else {\n        counts[ch]++;\n    }\n}\n\ndouble invLog2 = 1.0 / Math.log(2.0);\ndouble entropy = 0.0;\ndouble surprisalVar = 0.0;\ndouble pSum = 0.0;\n\nfor (int i = 0; i < seenCount; i++) {\n    int code = seen[i];\n    double cnt = (double) counts[code];\n    double p = cnt / (double) length;\n    double l2p = Math.log(p) * invLog2;\n\n    pSum += p;\n    double tmp = entropy;\n    entropy = tmp + (p / pSum) * (l2p - tmp);\n    surprisalVar += p * (l2p - tmp) * (l2p - entropy);\n}\n\nsurprisalVar = Math.max(0.0, surprisalVar);\ndouble surprisalSd = Math.sqrt(surprisalVar);\ndouble entropyBits = -entropy;\nif (entropyBits == -0.0) {\n    entropyBits = 0.0;\n}\n\ndouble normalizedEntropy = 0.0;\nif (length > 1) {\n    double maxEntropy = Math.log((double) length) * invLog2; // max bits if every character is unique\n    normalizedEntropy = entropyBits / maxEntropy;           // scale 0..1 against script length\n    if (normalizedEntropy < 0.0) normalizedEntropy = 0.0;\n    else if (normalizedEntropy > 1.0) normalizedEntropy = 1.0;\n}\n\nctx.powershell.file.script_block_entropy_bits = entropyBits;\nctx.powershell.file.script_block_entropy_normalized = normalizedEntropy;\nctx.powershell.file.script_block_surprisal_stdev = surprisalSd;\nctx.powershell.file.script_block_length = length;\nctx.powershell.file.script_block_unique_symbols = uniqueSymbols;"##
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") == Some("4103") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.Payload") {
                            if let Some(s) = event.get_string("winlog.event_data.Payload") {
                                let parts: Vec<Value> = s.split("\n").map(|p| json!(p)).collect();
                                event.set("winlog.event_data.Payload", Value::Array(parts))?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") == Some("4103") };
                if _cond {
                    // Painless script
                    // Source: def parseRawDetail(String raw) {\n    Pattern detailRegex = /^([^(]+)\\(([^)]+)\\)\\:\\s*(.+)?$/;\n    Pattern parameterBindingRegex = /name\\=(.+);\\s*value\\=(.+)$/;\n\n    def matcher = detailRegex.matcher(raw);\n    if (!matcher.matches()) {\n        return [\"value\": raw];\n    }\n    def matches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        matches.add(matcher.group(i));\n    }\n    \n    if (matches.length != 4) {\n        return [\"value\": raw];\n    }                \n    \n    if (matches[1] != \"ParameterBinding\") {\n        return [\n            \"type\": matches[1], \n            \"related_command\": matches[2], \n            \"value\": matches[3]\n        ];\n    }\n\n    matcher = parameterBindingRegex.matcher(matches[3]);\n    if (!matcher.matches()) {\n        return [\"value\": matches[4]];\n    }\n    def nameValMatches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        nameValMatches.add(matcher.group(i));\n    }\n    if (nameValMatches.length !== 3) {\n        return [\"value\": matches[3]];\n    }\n\n    return [\n        \"type\": matches[1],\n        \"related_command\": matches[2],\n        \"name\": nameValMatches[1],\n        \"value\": nameValMatches[2]\n    ];\n}\n\nif (ctx._temp == null) {\n    ctx._temp = new HashMap();\n}\n\nif (ctx._temp.details == null) {\n    ctx._temp.details = new ArrayList();\n}\n\ndef values = ctx.winlog?.event_data[params[\"field\"]];\nif (values != null && values.length > 0) {\n    for (v in values) {\n        ctx._temp.details.add(parseRawDetail(v));\n    }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def parseRawDetail(String raw) {\n    Pattern detailRegex = /^([^(]+)\\(([^)]+)\\)\\:\\s*(.+)?$/;\n    Pattern parameterBindingRegex = /name\\=(.+);\\s*value\\=(.+)$/;\n\n    def matcher = detailRegex.matcher(raw);\n    if (!matcher.matches()) {\n        return [\"value\": raw];\n    }\n    def matches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        matches.add(matcher.group(i));\n    }\n    \n    if (matches.length != 4) {\n        return [\"value\": raw];\n    }                \n    \n    if (matches[1] != \"ParameterBinding\") {\n        return [\n            \"type\": matches[1], \n            \"related_command\": matches[2], \n            \"value\": matches[3]\n        ];\n    }\n\n    matcher = parameterBindingRegex.matcher(matches[3]);\n    if (!matcher.matches()) {\n        return [\"value\": matches[4]];\n    }\n    def nameValMatches = new ArrayList();\n    for (def i = 0; i <= matcher.groupCount(); i++) {\n        nameValMatches.add(matcher.group(i));\n    }\n    if (nameValMatches.length !== 3) {\n        return [\"value\": matches[3]];\n    }\n\n    return [\n        \"type\": matches[1],\n        \"related_command\": matches[2],\n        \"name\": nameValMatches[1],\n        \"value\": nameValMatches[2]\n    ];\n}\n\nif (ctx._temp == null) {\n    ctx._temp = new HashMap();\n}\n\nif (ctx._temp.details == null) {\n    ctx._temp.details = new ArrayList();\n}\n\ndef values = ctx.winlog?.event_data[params[\"field\"]];\nif (values != null && values.length > 0) {\n    for (v in values) {\n        ctx._temp.details.add(parseRawDetail(v));\n    }\n}"#
                        ),
                        cached_params!("{\"field\":\"Payload\"}"),
                    )?;
                }
                let _cond = {
                    event.has_value("_temp.details") && event.get("_temp.details").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
                };
                if _cond {
                    event.rename("_temp.details", "powershell.command.invocation_details")?;
                }
                let _cond = {
                    event.has_value("process.command_line")
                        && event.get_str("process.command_line") != Some("")
                };
                if _cond {
                    // Painless script
                    // Source: // appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string cmd into next\n// argument and command line remainder.\ndef readNextArg(String cmd) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; cmd.length() > 0; cmd = cmd.substring(1)) {\n        def c = cmd.charAt(0);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"rest\": cmd.substring(1)\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && cmd.length() > 1 && cmd.charAt(1) == (char)'\"') {\n                    b.append(c);\n                    cmd = cmd.substring(1);\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"rest\": ''\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String cmd) {\n    def args = new ArrayList();\n    while (cmd.length() > 0) {\n        if (cmd.charAt(0) == (char)' ' || cmd.charAt(0) == (char)0x09) {\n            cmd = cmd.substring(1);\n            continue;\n        }\n        def next = readNextArg(cmd);\n        cmd = next.rest;\n        args.add(next.arg);\n    }\n    return args;\n}\n\nctx.process.args = commandLineToArgv(ctx.process.command_line);\nctx.process.args_count = ctx.process.args.length;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"// appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string cmd into next\n// argument and command line remainder.\ndef readNextArg(String cmd) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; cmd.length() > 0; cmd = cmd.substring(1)) {\n        def c = cmd.charAt(0);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"rest\": cmd.substring(1)\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && cmd.length() > 1 && cmd.charAt(1) == (char)'\"') {\n                    b.append(c);\n                    cmd = cmd.substring(1);\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"rest\": ''\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String cmd) {\n    def args = new ArrayList();\n    while (cmd.length() > 0) {\n        if (cmd.charAt(0) == (char)' ' || cmd.charAt(0) == (char)0x09) {\n            cmd = cmd.substring(1);\n            continue;\n        }\n        def next = readNextArg(cmd);\n        cmd = next.rest;\n        args.add(next.arg);\n    }\n    return args;\n}\n\nctx.process.args = commandLineToArgv(ctx.process.command_line);\nctx.process.args_count = ctx.process.args.length;"#
                        ),
                    )?;
                }
                let _cond = { event.get_str("winlog.event_data.Path") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.Path") {
                            event
                                .rename("winlog.event_data.Path", "winlog.event_data.ScriptName")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.ScriptName")
                        && event
                            .get_as_string("winlog.event_data.ScriptName")
                            .is_some_and(|s| s.len() > 1)
                };
                if _cond {
                    // Painless script
                    // Source: def path = ctx.winlog.event_data.ScriptName;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def path = ctx.winlog.event_data.ScriptName;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}"#
                        ),
                    )?;
                }
                let _cond = { event.get_str("winlog.event_data.ScriptName") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ScriptName") {
                            event.rename("winlog.event_data.ScriptName", "file.path")?;
                        }
                        Ok(())
                    })();
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("_temp");
                    event.remove("winlog.event_data.SequenceNumber");
                    event.remove("winlog.event_data.User");
                    event.remove("winlog.event_data.ConnectedUser");
                    event.remove("winlog.event_data.ContextInfo");
                    event.remove("winlog.event_data.Severity");
                    event.remove("winlog.event_data.MessageTotal");
                    event.remove("winlog.event_data.MessageNumber");
                    event.remove("winlog.event_data.Payload");
                    event.remove("winlog.time_created");
                    event.remove("winlog.level");
                    Ok(())
                })();
                let _cond = {
                    event.has_value("winlog.event_data") && event.get("winlog.event_data").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("winlog.event_data");
                        Ok(())
                    })();
                }
                // End nested pipeline: "powershell_operational"
            }

            let _cond = {
                event.get("winlog.channel").is_some_and(|v| v.is_string())
                    && event
                        .get_str("winlog.channel")
                        .is_some_and(|s| s.to_lowercase() == "microsoft-windows-sysmon/operational")
            };
            if _cond {
                // Begin nested pipeline: "sysmon_operational"
                event.set("ecs.version", json!("8.17.0"))?;
                let _cond = { event.has_value("winlog.event_data") };
                if _cond {
                    // Painless script
                    // Source: ctx.winlog?.event_data?.entrySet().removeIf(entry -> [null, \"\", \"-\", \"{00000000-0000-0000-0000-000000000000}\"].contains(entry.getValue()))
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.winlog?.event_data?.entrySet().removeIf(entry -> [null, \"\", \"-\", \"{00000000-0000-0000-0000-000000000000}\"].contains(entry.getValue()))"#
                        ),
                    )?;
                }
                let _cond = { event.get_str("winlog.level") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.level") {
                            event.rename("winlog.level", "log.level")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("winlog.time_created") };
                if _cond {
                    // on_failure: 3 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("winlog.time_created") {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["ISO8601"], None, None)
                            {
                                event.set("event.created", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "time_created_date")?;
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.remove("winlog.time_created").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "winlog.time_created".into(),
                                });
                            }
                            Ok(())
                        })();
                        event.append(
                            "error.message",
                            json!(format!(
                                "fail-{}",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        return Err(TransformError::ParseError {
                            path: "_fail".into(),
                            message: (format!(
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
                            ))
                            .to_string(),
                        });
                    }
                }
                let _cond = { event.has_value("winlog.event_data.UtcTime") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("winlog.event_data.UtcTime") {
                            if let Some(parsed) = parse_date_out(
                                &date_str,
                                &["yyyy-MM-dd HH:mm:ss.SSS"],
                                Some("UTC"),
                                None,
                            ) {
                                event.set("@timestamp", parsed)?;
                            }
                        }
                        Ok(())
                    })();
                }
                event.set("event.kind", json!("event"))?;
                event.set(
                    "event.code",
                    json!(
                        event
                            .get("winlog.event_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                // Painless script
                // Source: if (ctx.event?.code == null || params.get(ctx.event.code) == null) {\n  return;\n}\ndef hm = new HashMap(params[ctx.event.code]);\nhm.forEach((k, v) -> ctx.event[k] = v);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.event?.code == null || params.get(ctx.event.code) == null) {\n  return;\n}\ndef hm = new HashMap(params[ctx.event.code]);\nhm.forEach((k, v) -> ctx.event[k] = v);"#
                    ),
                    cached_params!(
                        "{\"1\":{\"action\":\"Process creation\",\"category\":[\"process\"],\"type\":[\"start\"]},\"10\":{\"action\":\"ProcessAccess\",\"category\":[\"process\"],\"type\":[\"access\"]},\"11\":{\"action\":\"FileCreate\",\"category\":[\"file\"],\"type\":[\"creation\"]},\"12\":{\"action\":\"RegistryEvent (Object create and delete)\",\"category\":[\"configuration\",\"registry\"],\"type\":[\"change\"]},\"13\":{\"action\":\"RegistryEvent (Value Set)\",\"category\":[\"configuration\",\"registry\"],\"type\":[\"change\"]},\"14\":{\"action\":\"RegistryEvent (Key and Value Rename)\",\"category\":[\"configuration\",\"registry\"],\"type\":[\"change\"]},\"15\":{\"action\":\"FileCreateStreamHash\",\"category\":[\"file\"],\"type\":[\"access\"]},\"16\":{\"action\":\"ServiceConfigurationChange\",\"category\":[\"configuration\"],\"type\":[\"change\"]},\"17\":{\"action\":\"PipeEvent (Pipe Created)\",\"category\":[\"file\"],\"type\":[\"creation\"]},\"18\":{\"action\":\"PipeEvent (Pipe Connected)\",\"category\":[\"file\"],\"type\":[\"access\"]},\"19\":{\"action\":\"WmiEvent (WmiEventFilter activity detected)\",\"category\":[\"process\"],\"type\":[\"info\"]},\"2\":{\"action\":\"A process changed a file creation time\",\"category\":[\"file\"],\"type\":[\"change\"]},\"20\":{\"action\":\"WmiEvent (WmiEventConsumer activity detected)\",\"category\":[\"process\"],\"type\":[\"change\"]},\"21\":{\"action\":\"WmiEvent (WmiEventConsumerToFilter activity detected)\",\"category\":[\"process\"],\"type\":[\"access\"]},\"22\":{\"action\":\"DNSEvent (DNS query)\",\"category\":[\"network\"],\"type\":[\"connection\",\"protocol\",\"info\"]},\"23\":{\"action\":\"FileDelete (File Delete archived)\",\"category\":[\"file\"],\"type\":[\"deletion\"]},\"24\":{\"action\":\"ClipboardChange (New content in the clipboard)\",\"type\":[\"change\"]},\"25\":{\"action\":\"ProcessTampering (Process image change)\",\"category\":[\"process\"],\"type\":[\"change\"]},\"255\":{\"action\":\"Error\",\"category\":[\"process\"],\"outcome\":[\"failure\"]},\"26\":{\"action\":\"FileDeleteDetected (File Delete logged)\",\"category\":[\"file\"],\"type\":[\"deletion\"]},\"27\":{\"action\":\"FileBlockExecutable\",\"category\":[\"file\"],\"outcome\":[\"failure\"],\"type\":[\"creation\"]},\"28\":{\"action\":\"FileBlockShredding\",\"category\":[\"file\"],\"type\":[\"deletion\"]},\"29\":{\"action\":\"FileExecutableDetected\",\"category\":[\"file\"],\"type\":[\"creation\"]},\"3\":{\"action\":\"Network connection\",\"category\":[\"network\"],\"type\":[\"start\",\"connection\",\"protocol\"]},\"4\":{\"action\":\"Sysmon service state changed\",\"category\":[\"process\"],\"type\":[\"change\"]},\"5\":{\"action\":\"Process terminated\",\"category\":[\"process\"],\"type\":[\"end\"]},\"6\":{\"action\":\"Driver loaded\",\"category\":[\"driver\"],\"type\":[\"start\"]},\"7\":{\"action\":[\"Image loaded\",\"load\"],\"category\":[\"process\",\"library\"],\"type\":[\"change\",\"start\"]},\"8\":{\"action\":\"CreateRemoteThread\",\"category\":[\"process\"],\"type\":[\"change\"]},\"9\":{\"action\":\"RawAccessRead\",\"category\":[\"process\"],\"type\":[\"access\"]}}"
                    ),
                )?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
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
                    Ok(())
                })();
                let _cond = {
                    event.get_str("event.code") == Some("255")
                        && event.has_value("winlog.event_data.ID")
                        && event.get_str("winlog.event_data.ID") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ID") {
                            event.rename("winlog.event_data.ID", "error.code")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.RuleName")
                        && event.get_str("winlog.event_data.RuleName") != Some("")
                        && event.get_str("winlog.event_data.RuleName") != Some("-")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.RuleName") {
                            event.rename("winlog.event_data.RuleName", "rule.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("event.code") == Some("25")
                        && event.has_value("winlog.event_data.Type")
                        && event.get_str("winlog.event_data.Type") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.Type") {
                            event.rename("winlog.event_data.Type", "message")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.Hash")
                        && event.get_str("winlog.event_data.Hash") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.Hash") {
                            event.rename("winlog.event_data.Hash", "winlog.event_data.Hashes")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("winlog.event_data.Hashes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("winlog.event_data.Hashes") {
                            for pair in kv_str.split(",") {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=") else {
                                    return Err(TransformError::ParseError {
                                        path: "winlog.event_data.Hashes".into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    if !key.is_empty() {
                                        event.set(&format!("_temp.hashes.{}", key), value)?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("_temp.hashes") };
                if _cond {
                    // Painless script
                    // Source: def hashIsEmpty(String hash) {\n  if (hash == \"\") {\n    return true;\n  }\n  \n  Pattern emptyHashRegex = /^0*$/;\n  def matcher = emptyHashRegex.matcher(hash);\n  \n  return matcher.matches(); \n}\n\ndef hashes = new HashMap();\ndef related = [\n  \"hash\": new ArrayList()\n];\nfor (entry in ctx._temp.hashes.entrySet()) {\n  def key = entry.getKey().toString().toLowerCase();\n  def value = entry.getValue().toString().toLowerCase();\n\n  if (hashIsEmpty(value)) {\n    continue;\n  }\n\n  hashes[key] = value;\n  related.hash.add(value);\n}\n\nctx._temp.hashes = hashes;\nif (related.hash.length > 0) {\n  ctx.related = related;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def hashIsEmpty(String hash) {\n  if (hash == \"\") {\n    return true;\n  }\n  \n  Pattern emptyHashRegex = /^0*$/;\n  def matcher = emptyHashRegex.matcher(hash);\n  \n  return matcher.matches(); \n}\n\ndef hashes = new HashMap();\ndef related = [\n  \"hash\": new ArrayList()\n];\nfor (entry in ctx._temp.hashes.entrySet()) {\n  def key = entry.getKey().toString().toLowerCase();\n  def value = entry.getValue().toString().toLowerCase();\n\n  if (hashIsEmpty(value)) {\n    continue;\n  }\n\n  hashes[key] = value;\n  related.hash.add(value);\n}\n\nctx._temp.hashes = hashes;\nif (related.hash.length > 0) {\n  ctx.related = related;\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("_temp.hashes")
                        && ["1", "23", "24", "25", "26"]
                            .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.rename("_temp.hashes", "process.hash")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("process.hash.imphash") {
                        event.rename("process.hash.imphash", "process.pe.imphash")?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("winlog.event_data.ProcessGuid")
                        && event.get_str("winlog.event_data.ProcessGuid") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ProcessGuid") {
                            event.rename("winlog.event_data.ProcessGuid", "process.entity_id")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.ProcessId")
                        && event.get_str("winlog.event_data.ProcessId") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.ProcessId") {
                            if let Some(val) = event.get("winlog.event_data.ProcessId") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "winlog.event_data.ProcessId".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.pid", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.Image")
                        && event.get_str("winlog.event_data.Image") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.Image") {
                            event.rename("winlog.event_data.Image", "process.executable")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.SourceProcessGuid")
                        && event.get_str("winlog.event_data.SourceProcessGuid") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.SourceProcessGuid") {
                            event.rename(
                                "winlog.event_data.SourceProcessGuid",
                                "process.entity_id",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.SourceProcessGUID")
                        && event.get_str("winlog.event_data.SourceProcessGUID") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.SourceProcessGUID") {
                            event.rename(
                                "winlog.event_data.SourceProcessGUID",
                                "process.entity_id",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.SourceProcessId")
                        && event.get_str("winlog.event_data.SourceProcessId") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.SourceProcessId") {
                            if let Some(val) = event.get("winlog.event_data.SourceProcessId") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "winlog.event_data.SourceProcessId".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.pid", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.SourceThreadId")
                        && event.get_str("winlog.event_data.SourceThreadId") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.SourceThreadId") {
                            if let Some(val) = event.get("winlog.event_data.SourceThreadId") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "winlog.event_data.SourceThreadId".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.thread.id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.SourceImage")
                        && event.get_str("winlog.event_data.SourceImage") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.SourceImage") {
                            event.rename("winlog.event_data.SourceImage", "process.executable")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.Destination")
                        && event.get_str("winlog.event_data.Destination") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.Destination") {
                            event.rename("winlog.event_data.Destination", "process.executable")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.CommandLine")
                        && event.get_str("winlog.event_data.CommandLine") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.CommandLine") {
                            event
                                .rename("winlog.event_data.CommandLine", "process.command_line")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.CurrentDirectory")
                        && event.get_str("winlog.event_data.CurrentDirectory") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.CurrentDirectory") {
                            event.rename(
                                "winlog.event_data.CurrentDirectory",
                                "process.working_directory",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.ParentProcessGuid")
                        && event.get_str("winlog.event_data.ParentProcessGuid") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ParentProcessGuid") {
                            event.rename(
                                "winlog.event_data.ParentProcessGuid",
                                "process.parent.entity_id",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.ParentProcessId")
                        && event.get_str("winlog.event_data.ParentProcessId") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.ParentProcessId") {
                            if let Some(val) = event.get("winlog.event_data.ParentProcessId") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "winlog.event_data.ParentProcessId".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.parent.pid", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.ParentImage")
                        && event.get_str("winlog.event_data.ParentImage") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ParentImage") {
                            event.rename(
                                "winlog.event_data.ParentImage",
                                "process.parent.executable",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.ParentCommandLine")
                        && event.get_str("winlog.event_data.ParentCommandLine") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ParentCommandLine") {
                            event.rename(
                                "winlog.event_data.ParentCommandLine",
                                "process.parent.command_line",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("event.code") != Some("7")
                        && event.has_value("winlog.event_data.OriginalFileName")
                        && event.get_str("winlog.event_data.OriginalFileName") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.OriginalFileName") {
                            event.rename(
                                "winlog.event_data.OriginalFileName",
                                "process.pe.original_file_name",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") != Some("7") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("winlog.event_data.Company")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("process.pe.company", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") != Some("7") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("winlog.event_data.Description")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("process.pe.description", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") != Some("7") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("winlog.event_data.FileVersion")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("process.pe.file_version", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") != Some("7") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("winlog.event_data.Product")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("process.pe.product", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    (event.has_value("process.command_line")
                        && event.get_str("process.command_line") != Some(""))
                        || (event.has_value("process.parent.command_line")
                            && event.get_str("process.parent.command_line") != Some(""))
                };
                if _cond {
                    // Painless script
                    // Source: // appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string cmd into next\n// argument and command line remainder.\ndef readNextArg(String cmd) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; cmd.length() > 0; cmd = cmd.substring(1)) {\n        def c = cmd.charAt(0);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"rest\": cmd.substring(1)\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && cmd.length() > 1 && cmd.charAt(1) == (char)'\"') {\n                    b.append(c);\n                    cmd = cmd.substring(1);\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"rest\": ''\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String cmd) {\n    def args = new ArrayList();\n    while (cmd.length() > 0) {\n        if (cmd.charAt(0) == (char)' ' || cmd.charAt(0) == (char)0x09) {\n            cmd = cmd.substring(1);\n            continue;\n        }\n        def next = readNextArg(cmd);\n        cmd = next.rest;\n        args.add(next.arg);\n    }\n    return args;\n}\n\ndef cmd = ctx.process?.command_line;\nif (cmd != null && cmd != \"\") {\n  ctx.process.args = commandLineToArgv(cmd);\n  ctx.process.args_count = ctx.process.args.length;\n}\n\ndef parentCmd = ctx.process?.parent?.command_line;\nif (parentCmd != null && parentCmd != \"\") {\n  ctx.process.parent.args = commandLineToArgv(parentCmd);\n  ctx.process.parent.args_count = ctx.process.parent.args.length;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"// appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string cmd into next\n// argument and command line remainder.\ndef readNextArg(String cmd) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; cmd.length() > 0; cmd = cmd.substring(1)) {\n        def c = cmd.charAt(0);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"rest\": cmd.substring(1)\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && cmd.length() > 1 && cmd.charAt(1) == (char)'\"') {\n                    b.append(c);\n                    cmd = cmd.substring(1);\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"rest\": ''\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String cmd) {\n    def args = new ArrayList();\n    while (cmd.length() > 0) {\n        if (cmd.charAt(0) == (char)' ' || cmd.charAt(0) == (char)0x09) {\n            cmd = cmd.substring(1);\n            continue;\n        }\n        def next = readNextArg(cmd);\n        cmd = next.rest;\n        args.add(next.arg);\n    }\n    return args;\n}\n\ndef cmd = ctx.process?.command_line;\nif (cmd != null && cmd != \"\") {\n  ctx.process.args = commandLineToArgv(cmd);\n  ctx.process.args_count = ctx.process.args.length;\n}\n\ndef parentCmd = ctx.process?.parent?.command_line;\nif (parentCmd != null && parentCmd != \"\") {\n  ctx.process.parent.args = commandLineToArgv(parentCmd);\n  ctx.process.parent.args_count = ctx.process.parent.args.length;\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    (event.has_value("process.executable")
                        && event
                            .get_as_string("process.executable")
                            .is_some_and(|s| s.len() > 1))
                        || (event.has_value("process.parent.executable")
                            && event
                                .get_as_string("process.parent.executable")
                                .is_some_and(|s| s.len() > 1))
                };
                if _cond {
                    // Painless script
                    // Source: def getProcessName(def path) {\n  def idx = path.lastIndexOf(\"\\\\\");\n  if (idx > -1) {\n      return path.substring(idx+1);\n  }\n  return \"\";\n}\n\ndef cmd = ctx.process?.executable;\nif (cmd != null && cmd != \"\" && ctx.process?.name == null) {\n  def name = getProcessName(cmd);\n  if (name != \"\") {\n    ctx.process.name = name;\n  }\n}\n\ndef parentCmd = ctx.process?.parent?.executable;\nif (parentCmd != null && parentCmd != \"\" && ctx.process?.parent?.name == null) {\n  def name = getProcessName(parentCmd);\n  if (name != \"\") {\n    ctx.process.parent.name = name;\n  }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def getProcessName(def path) {\n  def idx = path.lastIndexOf(\"\\\\\");\n  if (idx > -1) {\n      return path.substring(idx+1);\n  }\n  return \"\";\n}\n\ndef cmd = ctx.process?.executable;\nif (cmd != null && cmd != \"\" && ctx.process?.name == null) {\n  def name = getProcessName(cmd);\n  if (name != \"\") {\n    ctx.process.name = name;\n  }\n}\n\ndef parentCmd = ctx.process?.parent?.executable;\nif (parentCmd != null && parentCmd != \"\" && ctx.process?.parent?.name == null) {\n  def name = getProcessName(parentCmd);\n  if (name != \"\") {\n    ctx.process.parent.name = name;\n  }\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("_temp.hashes")
                        && ["6", "7", "15", "26", "29"]
                            .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    if let Some(v) = event.get("_temp.hashes").cloned() {
                        event.set("file.hash", v)?;
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("file.hash.imphash") {
                        event.rename("file.hash.imphash", "file.pe.imphash")?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("winlog.event_data.TargetFilename")
                        && event.get_str("winlog.event_data.TargetFilename") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.TargetFilename") {
                            event.rename("winlog.event_data.TargetFilename", "file.path")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.Device")
                        && event.get_str("winlog.event_data.Device") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.Device") {
                            event.rename("winlog.event_data.Device", "file.path")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.PipeName")
                        && event.get_str("winlog.event_data.PipeName") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.PipeName") {
                            event.rename("winlog.event_data.PipeName", "file.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.ImageLoaded")
                        && event.get_str("winlog.event_data.ImageLoaded") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.ImageLoaded") {
                            event.rename("winlog.event_data.ImageLoaded", "file.path")?;
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.event_data.Signature")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.code_signature.subject_name", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.event_data.SignatureStatus")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.code_signature.status", v)?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.get_str("event.code") == Some("7")
                        && event.has_value("winlog.event_data.OriginalFileName")
                        && event.get_str("winlog.event_data.OriginalFileName") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.OriginalFileName") {
                            event.rename(
                                "winlog.event_data.OriginalFileName",
                                "file.pe.original_file_name",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") == Some("7") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("winlog.event_data.Company")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("file.pe.company", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") == Some("7") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("winlog.event_data.Description")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("file.pe.description", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") == Some("7") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("winlog.event_data.FileVersion")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("file.pe.file_version", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") == Some("7") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("winlog.event_data.Product")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("file.pe.product", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.Signed")
                        && event.get_str("winlog.event_data.Signed") == Some("true")
                };
                if _cond {
                    event.set("file.code_signature.trusted", json!(true))?;
                }
                let _cond = {
                    event.has_value("winlog.event_data.Signed")
                        && event.get_str("winlog.event_data.Signed") != Some("true")
                };
                if _cond {
                    event.set("file.code_signature.trusted", json!(false))?;
                }
                let _cond = {
                    event.has_value("winlog.event_data.SignatureStatus")
                        && event.get_str("winlog.event_data.SignatureStatus") == Some("Valid")
                };
                if _cond {
                    event.set("file.code_signature.valid", json!(true))?;
                }
                let _cond = {
                    event.has_value("file.path")
                        && event
                            .get_as_string("file.path")
                            .is_some_and(|s| s.len() > 1)
                };
                if _cond {
                    // Painless script
                    // Source: def path = ctx.file.path;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def path = ctx.file.path;\ndef idx = path.lastIndexOf(\"\\\\\");\nif (idx > -1) {\n    if (ctx.file == null) {\n        ctx.file = new HashMap();\n    }\n    ctx.file.name = path.substring(idx+1);\n    ctx.file.directory = path.substring(0, idx);\n\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}"#
                        ),
                    )?;
                }
                let _cond = { event.get_str("event.code") == Some("7") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("file.name")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("dll.name", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") == Some("7") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("file.path")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("dll.path", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") == Some("7") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("winlog.event_data.Signature")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("dll.code_signature.subject_name", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("event.code") == Some("7")
                        && event.get_str("winlog.event_data.Signed") == Some("true")
                };
                if _cond {
                    event.set("dll.code_signature.trusted", json!(true))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("7")
                        && event.has_value("winlog.event_data.Signed")
                        && event.get_str("winlog.event_data.Signed") == Some("false")
                };
                if _cond {
                    event.set("dll.code_signature.trusted", json!(false))?;
                }
                let _cond = { event.get_str("event.code") == Some("7") };
                if _cond {
                    // Painless script
                    // Source: if (ctx.winlog?.event_data?.Signed == 'true') {\n  ctx.dll.code_signature.status = \"trusted\";\n} else if (ctx.winlog?.event_data?.SignatureStatus == \"Unavailable\") {\n  ctx.dll.code_signature.status = \"Unavailable\";\n} else if (ctx.winlog?.event_data?.Signed instanceof String && ctx.winlog.event_data.Signed.startsWith(\"failed:\")) {\n  ctx.dll.code_signature.status = ctx.winlog.event_data.Signed;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.winlog?.event_data?.Signed == 'true') {\n  ctx.dll.code_signature.status = \"trusted\";\n} else if (ctx.winlog?.event_data?.SignatureStatus == \"Unavailable\") {\n  ctx.dll.code_signature.status = \"Unavailable\";\n} else if (ctx.winlog?.event_data?.Signed instanceof String && ctx.winlog.event_data.Signed.startsWith(\"failed:\")) {\n  ctx.dll.code_signature.status = ctx.winlog.event_data.Signed;\n}"#
                        ),
                    )?;
                }
                let _cond =
                    { event.has_value("_temp.hashes") && event.get_str("event.code") == Some("7") };
                if _cond {
                    if let Some(v) = event.get("_temp.hashes").cloned() {
                        event.set("dll.hash", v)?;
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("dll.hash.imphash") {
                        event.rename("dll.hash.imphash", "dll.pe.imphash")?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.get_str("event.code") == Some("7")
                        && event.has_value("file.pe.original_file_name")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event.get("file.pe.original_file_name").cloned() {
                            event.set("dll.pe.original_file_name", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.Protocol")
                        && event.get_str("winlog.event_data.Protocol") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.Protocol") {
                            event.rename("winlog.event_data.Protocol", "network.transport")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("event.code") != Some("22")
                        && event.has_value("winlog.event_data.DestinationPortName")
                        && event.get_str("winlog.event_data.DestinationPortName") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.DestinationPortName") {
                            event.rename(
                                "winlog.event_data.DestinationPortName",
                                "network.protocol",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("event.code") != Some("22")
                        && event.has_value("winlog.event_data.SourcePortName")
                        && event.get_str("winlog.event_data.SourcePortName") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.SourcePortName") {
                            event.rename("winlog.event_data.SourcePortName", "network.protocol")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.code") == Some("22") };
                if _cond {
                    event.set("network.protocol", json!("dns"))?;
                }
                let _cond = {
                    event.has_value("winlog.event_data.SourceIp")
                        && event.get_str("winlog.event_data.SourceIp") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.SourceIp") {
                            if let Some(val) = event.get("winlog.event_data.SourceIp") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "winlog.event_data.SourceIp".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.ip", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.SourceHostname")
                        && event.get_str("winlog.event_data.SourceHostname") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.SourceHostname") {
                            event.rename("winlog.event_data.SourceHostname", "source.domain")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.SourcePort")
                        && event.get_str("winlog.event_data.SourcePort") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.SourcePort") {
                            if let Some(val) = event.get("winlog.event_data.SourcePort") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "winlog.event_data.SourcePort".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.DestinationIp")
                        && event.get_str("winlog.event_data.DestinationIp") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.DestinationIp") {
                            if let Some(val) = event.get("winlog.event_data.DestinationIp") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "winlog.event_data.DestinationIp".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.ip", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.DestinationHostname")
                        && event.get_str("winlog.event_data.DestinationHostname") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.DestinationHostname") {
                            event.rename(
                                "winlog.event_data.DestinationHostname",
                                "destination.domain",
                            )?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.DestinationPort")
                        && event.get_str("winlog.event_data.DestinationPort") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.DestinationPort") {
                            if let Some(val) = event.get("winlog.event_data.DestinationPort") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "winlog.event_data.DestinationPort".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.QueryName")
                        && event.get_str("winlog.event_data.QueryName") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.QueryName") {
                            event.rename("winlog.event_data.QueryName", "dns.question.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.Initiated")
                        && event.get_str("winlog.event_data.Initiated") == Some("true")
                };
                if _cond {
                    event.set("network.direction", json!("egress"))?;
                }
                let _cond = {
                    event.has_value("winlog.event_data.Initiated")
                        && event.get_str("winlog.event_data.Initiated") == Some("false")
                };
                if _cond {
                    event.set("network.direction", json!("ingress"))?;
                }
                let _cond = {
                    event.has_value("winlog.event_data.SourceIsIpv6")
                        && event.get_str("winlog.event_data.SourceIsIpv6") == Some("false")
                };
                if _cond {
                    event.set("network.type", json!("ipv4"))?;
                }
                let _cond = {
                    event.has_value("winlog.event_data.SourceIsIpv6")
                        && event.get_str("winlog.event_data.SourceIsIpv6") == Some("true")
                };
                if _cond {
                    event.set("network.type", json!("ipv6"))?;
                }
                let _cond = {
                    event.has_value("winlog.event_data.QueryResults")
                        && event.get_str("winlog.event_data.QueryResults") != Some("")
                };
                if _cond {
                    // Painless script
                    // Source: def results = /;/.split(ctx.winlog.event_data.QueryResults);\ndef answers = new ArrayList();\ndef ips = new ArrayList();\ndef relatedHosts = new ArrayList();\nfor (def i = 0; i < results.length; i++) {\n  def answer = results[i];\n  if (answer == \"\") {\n    continue;\n  }\n\n  if (answer.startsWith(\"type:\")) {\n    def parts = /\\s+/.split(answer);\n    if (parts.length < 2) {\n      throw new Exception(\"unexpected QueryResult format\");\n    }\n    if (parts.length == 3) {\n      answers.add([\n        \"type\": params[parts[1]],\n        \"data\": parts[2]\n      ]);\n      relatedHosts.add(parts[2]);\n    } else {\n      answers.add([\n        \"type\": params[parts[1]]\n      ]);\n    }\n  } else {\n    ips.add(answer);\n  }\n}\n\nif (answers.length > 0) {\n  ctx.dns.answers = answers;\n}\nif (ips.length > 0) {\n  ctx.dns.resolved_ip = ips;\n}\nif (relatedHosts.length > 0) {\n  if (ctx.related == null) {\n    ctx.related = new HashMap();\n  }\n  ctx.related.hosts = relatedHosts;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def results = /;/.split(ctx.winlog.event_data.QueryResults);\ndef answers = new ArrayList();\ndef ips = new ArrayList();\ndef relatedHosts = new ArrayList();\nfor (def i = 0; i < results.length; i++) {\n  def answer = results[i];\n  if (answer == \"\") {\n    continue;\n  }\n\n  if (answer.startsWith(\"type:\")) {\n    def parts = /\\s+/.split(answer);\n    if (parts.length < 2) {\n      throw new Exception(\"unexpected QueryResult format\");\n    }\n    if (parts.length == 3) {\n      answers.add([\n        \"type\": params[parts[1]],\n        \"data\": parts[2]\n      ]);\n      relatedHosts.add(parts[2]);\n    } else {\n      answers.add([\n        \"type\": params[parts[1]]\n      ]);\n    }\n  } else {\n    ips.add(answer);\n  }\n}\n\nif (answers.length > 0) {\n  ctx.dns.answers = answers;\n}\nif (ips.length > 0) {\n  ctx.dns.resolved_ip = ips;\n}\nif (relatedHosts.length > 0) {\n  if (ctx.related == null) {\n    ctx.related = new HashMap();\n  }\n  ctx.related.hosts = relatedHosts;\n}"#
                        ),
                        cached_params!(
                            "{\"1\":\"A\",\"10\":\"NULL\",\"100\":\"UINFO\",\"101\":\"UID\",\"102\":\"GID\",\"103\":\"UNSPEC\",\"11\":\"WKS\",\"12\":\"PTR\",\"13\":\"HINFO\",\"14\":\"MINFO\",\"15\":\"MX\",\"16\":\"TXT\",\"17\":\"RP\",\"18\":\"AFSDB\",\"19\":\"X25\",\"2\":\"NS\",\"20\":\"ISDN\",\"21\":\"RT\",\"22\":\"NSAP\",\"23\":\"NSAPPTR\",\"24\":\"SIG\",\"248\":\"ADDRS\",\"249\":\"TKEY\",\"25\":\"KEY\",\"250\":\"TSIG\",\"251\":\"IXFR\",\"252\":\"AXFR\",\"253\":\"MAILB\",\"254\":\"MAILA\",\"255\":\"ANY\",\"26\":\"PX\",\"27\":\"GPOS\",\"28\":\"AAAA\",\"29\":\"LOC\",\"3\":\"MD\",\"30\":\"NXT\",\"31\":\"EID\",\"32\":\"NIMLOC\",\"33\":\"SRV\",\"34\":\"ATMA\",\"35\":\"NAPTR\",\"36\":\"KX\",\"37\":\"CERT\",\"38\":\"A6\",\"39\":\"DNAME\",\"4\":\"MF\",\"40\":\"SINK\",\"41\":\"OPT\",\"43\":\"DS\",\"46\":\"RRSIG\",\"47\":\"NSEC\",\"48\":\"DNSKEY\",\"49\":\"DHCID\",\"5\":\"CNAME\",\"6\":\"SOA\",\"65281\":\"WINS\",\"65282\":\"WINSR\",\"7\":\"MB\",\"8\":\"MG\",\"9\":\"MR\"}"
                        ),
                    )?;
                }
                let _cond = { event.get("dns.answers").is_some_and(|v| v.is_array()) };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "dns.answers", |event| {
                            if let Some(s) = event.get_string("_ingest._value") {
                                let re = cached_regex!(
                                    "^\\[?::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)(?:\\](?::[0-9]+)?)?$"
                                );
                                let replaced = re.replace_all(&s, "$1").into_owned();
                                event.set("_ingest._value", replaced)?;
                            }
                            Ok(())
                        })?;
                        Ok(())
                    })();
                }
                let _cond = { event.get("dns.resolved_ip").is_some_and(|v| v.is_array()) };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "dns.resolved_ip", |event| {
                            if let Some(s) = event.get_string("_ingest._value") {
                                let re = cached_regex!(
                                    "^\\[?::ffff:([0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+)(?:\\](?::[0-9]+)?)?$"
                                );
                                let replaced = re.replace_all(&s, "$1").into_owned();
                                event.set("_ingest._value", replaced)?;
                            }
                            Ok(())
                        })?;
                        Ok(())
                    })();
                }
                if event.has_value("dns.resolved_ip") {
                    if let Some(Value::Array(items)) = event.get("dns.resolved_ip").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value", converted)?;
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
                                }
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        event.set("dns.resolved_ip", Value::Array(out))?;
                    }
                }
                let _cond = { event.has_value("dns.resolved_ip") };
                if _cond {
                    // Painless script
                    // Source: if (ctx.dns.answers == null) {\n  ctx.dns.answers = new ArrayList();\n}\nfor (def i = 0; i < ctx.dns.resolved_ip.length; i++) {\n  def ip = ctx.dns.resolved_ip[i];\n  if (ip == null) {\n    ctx.dns.resolved_ip.remove(i);\n    continue;\n  }\n\n  // Synthesize record type based on IP address type.\n  def type = \"A\";\n  if (ip.indexOf(\":\") != -1) {\n    type = \"AAAA\";\n  }\n  ctx.dns.answers.add([\n    \"type\": type,\n    \"data\": ip\n  ]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.dns.answers == null) {\n  ctx.dns.answers = new ArrayList();\n}\nfor (def i = 0; i < ctx.dns.resolved_ip.length; i++) {\n  def ip = ctx.dns.resolved_ip[i];\n  if (ip == null) {\n    ctx.dns.resolved_ip.remove(i);\n    continue;\n  }\n\n  // Synthesize record type based on IP address type.\n  def type = \"A\";\n  if (ip.indexOf(\":\") != -1) {\n    type = \"AAAA\";\n  }\n  ctx.dns.answers.add([\n    \"type\": type,\n    \"data\": ip\n  ]);\n}"#
                        ),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("dns.question.name") {
                        if let Some(domain_str) = event.get_string("dns.question.name") {
                            let domain = domain_str.to_string();
                            event.set("dns.question.domain", json!(domain.clone()))?;
                            // Public suffix list lookup for registered domain extraction
                            if let Some(rd) = registered_domain_lookup(&domain) {
                                if let Some(registered) = rd.registered_domain {
                                    event
                                        .set("dns.question.registered_domain", json!(registered))?;
                                }
                                event.set(
                                    "dns.question.top_level_domain",
                                    json!(rd.top_level_domain),
                                )?;
                                if let Some(sub) = rd.subdomain {
                                    event.set("dns.question.subdomain", json!(sub))?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("dns.question.name")
                        && event.get_str("dns.question.name") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("dns.question.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("dns.question.domain");
                    Ok(())
                })();
                if event.has_value("dns.resolved_ip") {
                    foreach_array(event, "dns.resolved_ip", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.ip",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Community ID v1 hash
                    if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                        event.get_string("source.ip"),
                        event.get_string("destination.ip"),
                        event
                            .get_as_string("network.iana_number")
                            .or_else(|| event.get_as_string("network.transport")),
                    ) {
                        let icmp = matches!(
                            protocol.to_ascii_lowercase().as_str(),
                            "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                        );
                        let (src_field, dst_field) = if icmp {
                            ("icmp.type", "icmp.code")
                        } else {
                            ("source.port", "destination.port")
                        };
                        let src_port =
                            u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                        let dst_port =
                            u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                        match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                            Ok(cid) => event.set("network.community_id", cid)?,
                            Err(message) => {
                                return Err(TransformError::ParseError {
                                    path: "network.community_id".into(),
                                    message,
                                });
                            }
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("winlog.user.identifier")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.id", v)?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("winlog.event_data.User") };
                if _cond {
                    if let Some(s) = event.get_string("winlog.event_data.User") {
                        let parts: Vec<Value> = cached_regex!("\\\\")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        event.set("_temp.user_parts", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("_temp.user_parts") && event.get("_temp.user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        let v = json!(
                            event
                                .get("_temp.user_parts.0")
                                .map_or_else(String::new, template_to_string)
                        );
                        if !painless_is_empty_value(&v) {
                            event.set("user.domain", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("_temp.user_parts") && event.get("_temp.user_parts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 2)
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        let v = json!(
                            event
                                .get("_temp.user_parts.1")
                                .map_or_else(String::new, template_to_string)
                        );
                        if !painless_is_empty_value(&v) {
                            event.set("user.name", v)?;
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data._MemberUserName") {
                        event.rename("winlog.event_data._MemberUserName", "user.name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("winlog.event_data._MemberDomain") {
                        event.rename("winlog.event_data._MemberDomain", "user.domain")?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("winlog.event_data._MemberAccountType") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.roles",
                            json!(
                                event
                                    .get("winlog.event_data._MemberAccountType")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("user.roles")
                        && event.has_value("winlog.event_data._MemberAccountType")
                        && event.get("user.roles").is_some_and(|v| {
                            match (v, event.get("winlog.event_data._MemberAccountType")) {
                                (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                                (
                                    serde_json::Value::String(s),
                                    Some(serde_json::Value::String(n)),
                                ) => s.contains(n.as_str()),
                                _ => false,
                            }
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("winlog.event_data._MemberAccountType");
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.QueryStatus")
                        && event.get_str("winlog.event_data.QueryStatus") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has("winlog.event_data.QueryStatus") {
                            event.rename("winlog.event_data.QueryStatus", "sysmon.dns.status")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("sysmon.dns.status")
                        && event.get_str("sysmon.dns.status") != Some("")
                };
                if _cond {
                    // Painless script
                    // Source: def status = params[ctx.sysmon.dns.status];\nif (status != null) {\n  ctx.sysmon.dns.status = status;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def status = params[ctx.sysmon.dns.status];\nif (status != null) {\n  ctx.sysmon.dns.status = status;\n}"#
                        ),
                        cached_params!(
                            "{\"0\":\"SUCCESS\",\"10054\":\"WSAECONNRESET\",\"10055\":\"WSAENOBUFS\",\"10060\":\"WSAETIMEDOUT\",\"1214\":\"ERROR_INVALID_NETNAME\",\"1223\":\"ERROR_CANCELLED\",\"123\":\"ERROR_INVALID_NAME\",\"13\":\"ERROR_INVALID_DATA\",\"14\":\"ERROR_OUTOFMEMORY\",\"1460\":\"ERROR_TIMEOUT\",\"4312\":\"ERROR_OBJECT_NOT_FOUND\",\"5\":\"ERROR_ACCESS_DENIED\",\"8\":\"ERROR_NOT_ENOUGH_MEMORY\",\"9001\":\"DNS_ERROR_RCODE_FORMAT_ERROR\",\"9002\":\"DNS_ERROR_RCODE_SERVER_FAILURE\",\"9003\":\"DNS_ERROR_RCODE_NAME_ERROR\",\"9004\":\"DNS_ERROR_RCODE_NOT_IMPLEMENTED\",\"9005\":\"DNS_ERROR_RCODE_REFUSED\",\"9006\":\"DNS_ERROR_RCODE_YXDOMAIN\",\"9007\":\"DNS_ERROR_RCODE_YXRRSET\",\"9008\":\"DNS_ERROR_RCODE_NXRRSET\",\"9009\":\"DNS_ERROR_RCODE_NOTAUTH\",\"9010\":\"DNS_ERROR_RCODE_NOTZONE\",\"9016\":\"DNS_ERROR_RCODE_BADSIG\",\"9017\":\"DNS_ERROR_RCODE_BADKEY\",\"9018\":\"DNS_ERROR_RCODE_BADTIME\",\"9101\":\"DNS_ERROR_KEYMASTER_REQUIRED\",\"9102\":\"DNS_ERROR_NOT_ALLOWED_ON_SIGNED_ZONE\",\"9103\":\"DNS_ERROR_NSEC3_INCOMPATIBLE_WITH_RSA_SHA1\",\"9104\":\"DNS_ERROR_NOT_ENOUGH_SIGNING_KEY_DESCRIPTORS\",\"9105\":\"DNS_ERROR_UNSUPPORTED_ALGORITHM\",\"9106\":\"DNS_ERROR_INVALID_KEY_SIZE\",\"9107\":\"DNS_ERROR_SIGNING_KEY_NOT_ACCESSIBLE\",\"9108\":\"DNS_ERROR_KSP_DOES_NOT_SUPPORT_PROTECTION\",\"9109\":\"DNS_ERROR_UNEXPECTED_DATA_PROTECTION_ERROR\",\"9110\":\"DNS_ERROR_UNEXPECTED_CNG_ERROR\",\"9111\":\"DNS_ERROR_UNKNOWN_SIGNING_PARAMETER_VERSION\",\"9112\":\"DNS_ERROR_KSP_NOT_ACCESSIBLE\",\"9113\":\"DNS_ERROR_TOO_MANY_SKDS\",\"9114\":\"DNS_ERROR_INVALID_ROLLOVER_PERIOD\",\"9115\":\"DNS_ERROR_INVALID_INITIAL_ROLLOVER_OFFSET\",\"9116\":\"DNS_ERROR_ROLLOVER_IN_PROGRESS\",\"9117\":\"DNS_ERROR_STANDBY_KEY_NOT_PRESENT\",\"9118\":\"DNS_ERROR_NOT_ALLOWED_ON_ZSK\",\"9119\":\"DNS_ERROR_NOT_ALLOWED_ON_ACTIVE_SKD\",\"9120\":\"DNS_ERROR_ROLLOVER_ALREADY_QUEUED\",\"9121\":\"DNS_ERROR_NOT_ALLOWED_ON_UNSIGNED_ZONE\",\"9122\":\"DNS_ERROR_BAD_KEYMASTER\",\"9123\":\"DNS_ERROR_INVALID_SIGNATURE_VALIDITY_PERIOD\",\"9124\":\"DNS_ERROR_INVALID_NSEC3_ITERATION_COUNT\",\"9125\":\"DNS_ERROR_DNSSEC_IS_DISABLED\",\"9126\":\"DNS_ERROR_INVALID_XML\",\"9127\":\"DNS_ERROR_NO_VALID_TRUST_ANCHORS\",\"9128\":\"DNS_ERROR_ROLLOVER_NOT_POKEABLE\",\"9129\":\"DNS_ERROR_NSEC3_NAME_COLLISION\",\"9130\":\"DNS_ERROR_NSEC_INCOMPATIBLE_WITH_NSEC3_RSA_SHA1\",\"9501\":\"DNS_INFO_NO_RECORDS\",\"9502\":\"DNS_ERROR_BAD_PACKET\",\"9503\":\"DNS_ERROR_NO_PACKET\",\"9504\":\"DNS_ERROR_RCODE\",\"9505\":\"DNS_ERROR_UNSECURE_PACKET\",\"9506\":\"DNS_REQUEST_PENDING\",\"9551\":\"DNS_ERROR_INVALID_TYPE\",\"9552\":\"DNS_ERROR_INVALID_IP_ADDRESS\",\"9553\":\"DNS_ERROR_INVALID_PROPERTY\",\"9554\":\"DNS_ERROR_TRY_AGAIN_LATER\",\"9555\":\"DNS_ERROR_NOT_UNIQUE\",\"9556\":\"DNS_ERROR_NON_RFC_NAME\",\"9557\":\"DNS_STATUS_FQDN\",\"9558\":\"DNS_STATUS_DOTTED_NAME\",\"9559\":\"DNS_STATUS_SINGLE_PART_NAME\",\"9560\":\"DNS_ERROR_INVALID_NAME_CHAR\",\"9561\":\"DNS_ERROR_NUMERIC_NAME\",\"9562\":\"DNS_ERROR_NOT_ALLOWED_ON_ROOT_SERVER\",\"9563\":\"DNS_ERROR_NOT_ALLOWED_UNDER_DELEGATION\",\"9564\":\"DNS_ERROR_CANNOT_FIND_ROOT_HINTS\",\"9565\":\"DNS_ERROR_INCONSISTENT_ROOT_HINTS\",\"9566\":\"DNS_ERROR_DWORD_VALUE_TOO_SMALL\",\"9567\":\"DNS_ERROR_DWORD_VALUE_TOO_LARGE\",\"9568\":\"DNS_ERROR_BACKGROUND_LOADING\",\"9569\":\"DNS_ERROR_NOT_ALLOWED_ON_RODC\",\"9570\":\"DNS_ERROR_NOT_ALLOWED_UNDER_DNAME\",\"9571\":\"DNS_ERROR_DELEGATION_REQUIRED\",\"9572\":\"DNS_ERROR_INVALID_POLICY_TABLE\",\"9573\":\"DNS_ERROR_ADDRESS_REQUIRED\",\"9601\":\"DNS_ERROR_ZONE_DOES_NOT_EXIST\",\"9602\":\"DNS_ERROR_NO_ZONE_INFO\",\"9603\":\"DNS_ERROR_INVALID_ZONE_OPERATION\",\"9604\":\"DNS_ERROR_ZONE_CONFIGURATION_ERROR\",\"9605\":\"DNS_ERROR_ZONE_HAS_NO_SOA_RECORD\",\"9606\":\"DNS_ERROR_ZONE_HAS_NO_NS_RECORDS\",\"9607\":\"DNS_ERROR_ZONE_LOCKED\",\"9608\":\"DNS_ERROR_ZONE_CREATION_FAILED\",\"9609\":\"DNS_ERROR_ZONE_ALREADY_EXISTS\",\"9610\":\"DNS_ERROR_AUTOZONE_ALREADY_EXISTS\",\"9611\":\"DNS_ERROR_INVALID_ZONE_TYPE\",\"9612\":\"DNS_ERROR_SECONDARY_REQUIRES_MASTER_IP\",\"9613\":\"DNS_ERROR_ZONE_NOT_SECONDARY\",\"9614\":\"DNS_ERROR_NEED_SECONDARY_ADDRESSES\",\"9615\":\"DNS_ERROR_WINS_INIT_FAILED\",\"9616\":\"DNS_ERROR_NEED_WINS_SERVERS\",\"9617\":\"DNS_ERROR_NBSTAT_INIT_FAILED\",\"9618\":\"DNS_ERROR_SOA_DELETE_INVALID\",\"9619\":\"DNS_ERROR_FORWARDER_ALREADY_EXISTS\",\"9620\":\"DNS_ERROR_ZONE_REQUIRES_MASTER_IP\",\"9621\":\"DNS_ERROR_ZONE_IS_SHUTDOWN\",\"9622\":\"DNS_ERROR_ZONE_LOCKED_FOR_SIGNING\",\"9651\":\"DNS_ERROR_PRIMARY_REQUIRES_DATAFILE\",\"9652\":\"DNS_ERROR_INVALID_DATAFILE_NAME\",\"9653\":\"DNS_ERROR_DATAFILE_OPEN_FAILURE\",\"9654\":\"DNS_ERROR_FILE_WRITEBACK_FAILED\",\"9655\":\"DNS_ERROR_DATAFILE_PARSING\",\"9701\":\"DNS_ERROR_RECORD_DOES_NOT_EXIST\",\"9702\":\"DNS_ERROR_RECORD_FORMAT\",\"9703\":\"DNS_ERROR_NODE_CREATION_FAILED\",\"9704\":\"DNS_ERROR_UNKNOWN_RECORD_TYPE\",\"9705\":\"DNS_ERROR_RECORD_TIMED_OUT\",\"9706\":\"DNS_ERROR_NAME_NOT_IN_ZONE\",\"9707\":\"DNS_ERROR_CNAME_LOOP\",\"9708\":\"DNS_ERROR_NODE_IS_CNAME\",\"9709\":\"DNS_ERROR_CNAME_COLLISION\",\"9710\":\"DNS_ERROR_RECORD_ONLY_AT_ZONE_ROOT\",\"9711\":\"DNS_ERROR_RECORD_ALREADY_EXISTS\",\"9712\":\"DNS_ERROR_SECONDARY_DATA\",\"9713\":\"DNS_ERROR_NO_CREATE_CACHE_DATA\",\"9714\":\"DNS_ERROR_NAME_DOES_NOT_EXIST\",\"9715\":\"DNS_WARNING_PTR_CREATE_FAILED\",\"9716\":\"DNS_WARNING_DOMAIN_UNDELETED\",\"9717\":\"DNS_ERROR_DS_UNAVAILABLE\",\"9718\":\"DNS_ERROR_DS_ZONE_ALREADY_EXISTS\",\"9719\":\"DNS_ERROR_NO_BOOTFILE_IF_DS_ZONE\",\"9720\":\"DNS_ERROR_NODE_IS_DNAME\",\"9721\":\"DNS_ERROR_DNAME_COLLISION\",\"9722\":\"DNS_ERROR_ALIAS_LOOP\",\"9751\":\"DNS_INFO_AXFR_COMPLETE\",\"9752\":\"DNS_ERROR_AXFR\",\"9753\":\"DNS_INFO_ADDED_LOCAL_WINS\",\"9801\":\"DNS_STATUS_CONTINUE_NEEDED\",\"9851\":\"DNS_ERROR_NO_TCPIP\",\"9852\":\"DNS_ERROR_NO_DNS_SERVERS\",\"9901\":\"DNS_ERROR_DP_DOES_NOT_EXIST\",\"9902\":\"DNS_ERROR_DP_ALREADY_EXISTS\",\"9903\":\"DNS_ERROR_DP_NOT_ENLISTED\",\"9904\":\"DNS_ERROR_DP_ALREADY_ENLISTED\",\"9905\":\"DNS_ERROR_DP_NOT_AVAILABLE\",\"9906\":\"DNS_ERROR_DP_FSMO_ERROR\",\"9911\":\"DNS_ERROR_RRL_NOT_ENABLED\",\"9912\":\"DNS_ERROR_RRL_INVALID_WINDOW_SIZE\",\"9913\":\"DNS_ERROR_RRL_INVALID_IPV4_PREFIX\",\"9914\":\"DNS_ERROR_RRL_INVALID_IPV6_PREFIX\",\"9915\":\"DNS_ERROR_RRL_INVALID_TC_RATE\",\"9916\":\"DNS_ERROR_RRL_INVALID_LEAK_RATE\",\"9917\":\"DNS_ERROR_RRL_LEAK_RATE_LESSTHAN_TC_RATE\",\"9921\":\"DNS_ERROR_VIRTUALIZATION_INSTANCE_ALREADY_EXISTS\",\"9922\":\"DNS_ERROR_VIRTUALIZATION_INSTANCE_DOES_NOT_EXIST\",\"9923\":\"DNS_ERROR_VIRTUALIZATION_TREE_LOCKED\",\"9924\":\"DNS_ERROR_INVAILD_VIRTUALIZATION_INSTANCE_NAME\",\"9925\":\"DNS_ERROR_DEFAULT_VIRTUALIZATION_INSTANCE\",\"9951\":\"DNS_ERROR_ZONESCOPE_ALREADY_EXISTS\",\"9952\":\"DNS_ERROR_ZONESCOPE_DOES_NOT_EXIST\",\"9953\":\"DNS_ERROR_DEFAULT_ZONESCOPE\",\"9954\":\"DNS_ERROR_INVALID_ZONESCOPE_NAME\",\"9955\":\"DNS_ERROR_NOT_ALLOWED_WITH_ZONESCOPES\",\"9956\":\"DNS_ERROR_LOAD_ZONESCOPE_FAILED\",\"9957\":\"DNS_ERROR_ZONESCOPE_FILE_WRITEBACK_FAILED\",\"9958\":\"DNS_ERROR_INVALID_SCOPE_NAME\",\"9959\":\"DNS_ERROR_SCOPE_DOES_NOT_EXIST\",\"9960\":\"DNS_ERROR_DEFAULT_SCOPE\",\"9961\":\"DNS_ERROR_INVALID_SCOPE_OPERATION\",\"9962\":\"DNS_ERROR_SCOPE_LOCKED\",\"9963\":\"DNS_ERROR_SCOPE_ALREADY_EXISTS\",\"9971\":\"DNS_ERROR_POLICY_ALREADY_EXISTS\",\"9972\":\"DNS_ERROR_POLICY_DOES_NOT_EXIST\",\"9973\":\"DNS_ERROR_POLICY_INVALID_CRITERIA\",\"9974\":\"DNS_ERROR_POLICY_INVALID_SETTINGS\",\"9975\":\"DNS_ERROR_CLIENT_SUBNET_IS_ACCESSED\",\"9976\":\"DNS_ERROR_CLIENT_SUBNET_DOES_NOT_EXIST\",\"9977\":\"DNS_ERROR_CLIENT_SUBNET_ALREADY_EXISTS\",\"9978\":\"DNS_ERROR_SUBNET_DOES_NOT_EXIST\",\"9979\":\"DNS_ERROR_SUBNET_ALREADY_EXISTS\",\"9980\":\"DNS_ERROR_POLICY_LOCKED\",\"9981\":\"DNS_ERROR_POLICY_INVALID_WEIGHT\",\"9982\":\"DNS_ERROR_POLICY_INVALID_NAME\",\"9983\":\"DNS_ERROR_POLICY_MISSING_CRITERIA\",\"9984\":\"DNS_ERROR_INVALID_CLIENT_SUBNET_NAME\",\"9985\":\"DNS_ERROR_POLICY_PROCESSING_ORDER_INVALID\",\"9986\":\"DNS_ERROR_POLICY_SCOPE_MISSING\",\"9987\":\"DNS_ERROR_POLICY_SCOPE_NOT_ALLOWED\",\"9988\":\"DNS_ERROR_SERVERSCOPE_IS_REFERENCED\",\"9989\":\"DNS_ERROR_ZONESCOPE_IS_REFERENCED\",\"9990\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_CLIENT_SUBNET\",\"9991\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_TRANSPORT_PROTOCOL\",\"9992\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_NETWORK_PROTOCOL\",\"9993\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_INTERFACE\",\"9994\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_FQDN\",\"9995\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_QUERY_TYPE\",\"9996\":\"DNS_ERROR_POLICY_INVALID_CRITERIA_TIME_OF_DAY\"}"
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("winlog.event_data.Archived")
                        && event.get_str("winlog.event_data.Archived") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.Archived") {
                            if let Some(val) = event.get("winlog.event_data.Archived") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "winlog.event_data.Archived".into(),
                                            message,
                                        }
                                    })?;
                                event.set("sysmon.file.archived", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("winlog.event_data.IsExecutable")
                        && event.get_str("winlog.event_data.IsExecutable") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("winlog.event_data.IsExecutable") {
                            if let Some(val) = event.get("winlog.event_data.IsExecutable") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "winlog.event_data.IsExecutable".into(),
                                            message,
                                        }
                                    })?;
                                event.set("sysmon.file.is_executable", converted)?;
                            }
                        }
                        Ok(())
                    })();
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
                let _cond =
                    { event.has_value("user.name") && event.get_str("user.name") != Some("") };
                if _cond {
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
                }
                let _cond =
                    { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
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
                let _cond = {
                    event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
                };
                if _cond {
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
                }
                let _cond = {
                    event.has_value("winlog.event_data.TargetObject")
                        && event.get_str("winlog.event_data.TargetObject") != Some("")
                        && ["12", "13", "14"].contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    // Painless script
                    // Source: ctx.registry = new HashMap();\nPattern qwordRegex = /(?i)QWORD \\(((0x[0-9A-F]{8})-(0x[0-9A-F]{8}))\\)/;\nPattern dwordRegex = /(?i)DWORD \\((0x[0-9A-F]{8})\\)/;\nPattern binDataRegex = /Binary Data/;\n\ndef path = ctx.winlog.event_data.TargetObject;\nctx.registry.path = path;\n\ndef pathTokens = Arrays.asList(/\\\\/.split(path));\ndef hive = params[pathTokens[0]];\nif (hive != null) {\n  ctx.registry.hive = hive;\n  if (pathTokens.length > 1) {\n    ctx.registry.key = pathTokens.subList(1, pathTokens.length).join(\"\\\\\");\n  }\n}\n\ndef value = pathTokens[pathTokens.length - 1];\nctx.registry.value = value;\n\ndef data = ctx.winlog?.event_data?.Details;\nif (data != null && data != \"\") {\n  def prefixLen = 2; // to remove 0x prefix\n  def dataValue = \"\";\n  def dataType = \"\";\n  def matcher = qwordRegex.matcher(data);\n  if (matcher.matches()) {\n    def parsedHighByte = Long.parseLong(matcher.group(2).substring(prefixLen), 16);\n    def parsedLowByte = Long.parseLong(matcher.group(3).substring(prefixLen), 16);\n    if (!Double.isNaN(parsedHighByte) && !Double.isNaN(parsedLowByte)) {\n      dataType = \"SZ_QWORD\";\n      dataValue = Long.toString(((parsedHighByte << 8) + parsedLowByte));\n      ctx.registry.data = [\n        \"strings\": [dataValue],\n        \"type\": dataType\n      ];\n    }\n    return;\n  }\n\n  matcher = dwordRegex.matcher(data);\n  if (matcher.matches()) {\n    def parsedValue = Long.parseLong(matcher.group(1).substring(prefixLen), 16);\n    if (!Double.isNaN(parsedValue)) {\n      dataType = \"SZ_DWORD\";\n      dataValue = Long.toString(parsedValue);\n      ctx.registry.data = [\n        \"strings\": [dataValue],\n        \"type\": dataType\n      ];\n    }\n    return;\n  }\n\n  matcher = binDataRegex.matcher(data);\n  if (matcher.matches()) {\n    // Data type could be REG_BINARY or REG_MULTI_SZ\n    ctx.registry.data = [\n      \"strings\": [data],\n      \"type\": \"REG_BINARY\"\n    ];\n    return;\n  }\n\n  // REG_SZ or REG_EXPAND_SZ\n  ctx.registry.data = [\n    \"strings\": [data],\n    \"type\": \"REG_SZ\"\n  ];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.registry = new HashMap();\nPattern qwordRegex = /(?i)QWORD \\(((0x[0-9A-F]{8})-(0x[0-9A-F]{8}))\\)/;\nPattern dwordRegex = /(?i)DWORD \\((0x[0-9A-F]{8})\\)/;\nPattern binDataRegex = /Binary Data/;\n\ndef path = ctx.winlog.event_data.TargetObject;\nctx.registry.path = path;\n\ndef pathTokens = Arrays.asList(/\\\\/.split(path));\ndef hive = params[pathTokens[0]];\nif (hive != null) {\n  ctx.registry.hive = hive;\n  if (pathTokens.length > 1) {\n    ctx.registry.key = pathTokens.subList(1, pathTokens.length).join(\"\\\\\");\n  }\n}\n\ndef value = pathTokens[pathTokens.length - 1];\nctx.registry.value = value;\n\ndef data = ctx.winlog?.event_data?.Details;\nif (data != null && data != \"\") {\n  def prefixLen = 2; // to remove 0x prefix\n  def dataValue = \"\";\n  def dataType = \"\";\n  def matcher = qwordRegex.matcher(data);\n  if (matcher.matches()) {\n    def parsedHighByte = Long.parseLong(matcher.group(2).substring(prefixLen), 16);\n    def parsedLowByte = Long.parseLong(matcher.group(3).substring(prefixLen), 16);\n    if (!Double.isNaN(parsedHighByte) && !Double.isNaN(parsedLowByte)) {\n      dataType = \"SZ_QWORD\";\n      dataValue = Long.toString(((parsedHighByte << 8) + parsedLowByte));\n      ctx.registry.data = [\n        \"strings\": [dataValue],\n        \"type\": dataType\n      ];\n    }\n    return;\n  }\n\n  matcher = dwordRegex.matcher(data);\n  if (matcher.matches()) {\n    def parsedValue = Long.parseLong(matcher.group(1).substring(prefixLen), 16);\n    if (!Double.isNaN(parsedValue)) {\n      dataType = \"SZ_DWORD\";\n      dataValue = Long.toString(parsedValue);\n      ctx.registry.data = [\n        \"strings\": [dataValue],\n        \"type\": dataType\n      ];\n    }\n    return;\n  }\n\n  matcher = binDataRegex.matcher(data);\n  if (matcher.matches()) {\n    // Data type could be REG_BINARY or REG_MULTI_SZ\n    ctx.registry.data = [\n      \"strings\": [data],\n      \"type\": \"REG_BINARY\"\n    ];\n    return;\n  }\n\n  // REG_SZ or REG_EXPAND_SZ\n  ctx.registry.data = [\n    \"strings\": [data],\n    \"type\": \"REG_SZ\"\n  ];\n}"#
                        ),
                        cached_params!(
                            "{\"HKCC\":\"HKCC\",\"HKCR\":\"HKCR\",\"HKCU\":\"HKCU\",\"HKDD\":\"HKDD\",\"HKEY_CLASSES_ROOT\":\"HKCR\",\"HKEY_CURRENT_CONFIG\":\"HKCC\",\"HKEY_CURRENT_USER\":\"HKCU\",\"HKEY_DYN_DATA\":\"HKDD\",\"HKEY_LOCAL_MACHINE\":\"HKLM\",\"HKEY_PERFORMANCE_DATA\":\"HKPD\",\"HKEY_USERS\":\"HKU\",\"HKLM\":\"HKLM\",\"HKPD\":\"HKPD\",\"HKU\":\"HKU\"}"
                        ),
                    )?;
                }
                let _cond = { event.has_value("winlog.event_data.TargetProcessGuid") };
                if _cond {
                    event.rename(
                        "winlog.event_data.TargetProcessGuid",
                        "winlog.event_data.TargetProcessGUID",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("_temp");
                    event.remove("winlog.event_data.ProcessId");
                    event.remove("winlog.event_data.ParentProcessId");
                    event.remove("winlog.event_data.SourceProcessId");
                    event.remove("winlog.event_data.SourceThreadId");
                    event.remove("winlog.event_data.SourceIp");
                    event.remove("winlog.event_data.SourcePort");
                    event.remove("winlog.event_data.SourcePortName");
                    event.remove("winlog.event_data.DestinationIp");
                    event.remove("winlog.event_data.DestinationPort");
                    event.remove("winlog.event_data.DestinationPortName");
                    event.remove("winlog.event_data.RuleName");
                    event.remove("winlog.event_data.User");
                    event.remove("winlog.event_data.Initiated");
                    event.remove("winlog.event_data.SourceIsIpv6");
                    event.remove("winlog.event_data.DestinationIsIpv6");
                    event.remove("winlog.event_data.QueryStatus");
                    event.remove("winlog.event_data.Archived");
                    event.remove("winlog.event_data.IsExecutable");
                    event.remove("winlog.event_data.QueryResults");
                    event.remove("winlog.event_data.UtcTime");
                    event.remove("winlog.event_data.Hash");
                    event.remove("winlog.event_data.Hashes");
                    event.remove("winlog.event_data.TargetObject");
                    event.remove("winlog.event_data.Details");
                    event.remove("winlog.time_created");
                    event.remove("winlog.level");
                    Ok(())
                })();
                let _cond = {
                    event.has_value("winlog.event_data") && event.get("winlog.event_data").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("winlog.event_data");
                        Ok(())
                    })();
                }
                // End nested pipeline: "sysmon_operational"
            }

            if !event.has("host.os.type") {
                event.set("host.os.type", json!("windows"))?;
            }

            if !event.has("host.os.family") {
                event.set("host.os.family", json!("windows"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("winlog.event_data._MemberUserName") {
                    event.rename("winlog.event_data._MemberUserName", "user.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("winlog.event_data._MemberDomain") {
                    event.rename("winlog.event_data._MemberDomain", "user.domain")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("winlog.event_data._MemberAccountType") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "user.roles",
                        json!(
                            event
                                .get("winlog.event_data._MemberAccountType")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("user.roles")
                    && event.has_value("winlog.event_data._MemberAccountType")
                    && event.get("user.roles").is_some_and(|v| {
                        match (v, event.get("winlog.event_data._MemberAccountType")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("winlog.event_data._MemberAccountType");
                    Ok(())
                })();
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
                            .map_or_else(String::new, template_to_string)
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
