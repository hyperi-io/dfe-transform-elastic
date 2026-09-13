// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The three `jamf_protect` captures, and what the grammar refuses.
//!
//! Every vendor script here is the CALL SITE's own literal, escapes and all --
//! a generated site holds the pipeline's folded YAML written back out as an
//! escaped string, so its newlines are two characters. A test written with
//! real newlines passes over the step that resolves them.
//!
//! The last few decline cases are CONSTRUCTED, and say so: they cover the
//! structural rules -- no parent creation, one member, an `else` arm -- that no
//! vendor script in the tree spells today.

use serde_json::json;

use super::*;
use crate::common::normalise;

/// Verbatim from `crates/dfe-transforms/src/filebeat/jamf_protect_alerts/default.rs`.
const FILE_CAPTURE: &str = r#"if (ctx.jamf_protect.alerts.input.related.files.size() > 0) {\n    def file = ctx.jamf_protect.alerts.input.related.files[0];\n\n    ctx.file = ctx.file ?: new HashMap();\n    \n    ctx.file.path = file.path;\n    ctx.file.size = file.size;\n    ctx.file.inode = String.valueOf(file.inode);\n    ctx.file.gid = String.valueOf(file.gid);\n    ctx.file.mode = String.valueOf(file.mode);\n    ctx.file.uid = String.valueOf(file.uid);\n    \n    ctx.file.hash = ctx.file.hash ?: new HashMap();\n    ctx.file.hash.sha1 = file.sha1hex;\n    ctx.file.hash.sha256 = file.sha256hex;\n    \n    ctx.file.code_signature = ctx.file.code_signature ?: new HashMap();\n    ctx.file.code_signature.signing_id = file.signingInfo?.appid; // Use safe navigation for nested objects\n    ctx.file.code_signature.status = file.signingInfo?.statusMessage;\n    ctx.file.code_signature.team_id = file.signingInfo?.teamid;\n}\n"#;

/// Verbatim from `crates/dfe-transforms/src/filebeat/jamf_protect_alerts/default.rs`.
const PROCESS_CAPTURE: &str = r#"if (ctx.jamf_protect?.alerts?.input?.related?.processes != null && ctx.jamf_protect.alerts.input.related.processes.size() > 0) {\n    def process = ctx.jamf_protect.alerts.input.related.processes[0];\n    def binary = ctx.jamf_protect.alerts.input.related.binaries[0];\n    \n    ctx.process = ctx.process ?: new HashMap();                          \n    ctx.process.name = process.name;\n    ctx.process.executable = process.path;\n    ctx.process.pid = process.pid;\n    if (process.containsKey('startTimestamp')) {\n        ctx.process.start = Instant.ofEpochSecond(process.startTimestamp).toString();\n    }\n    if (process?.exitCode != null) {\n        ctx.process.exit_code = process.exitCode;\n    }\n    ctx.process.args = process.args ?: new ArrayList();\n    ctx.process.entity_id = process.uuid;\n\n    ctx.process.parent = new HashMap();    \n    ctx.process.parent.pid = process.responsiblePID;\n\n    ctx.process.user = new HashMap();\n    ctx.process.user.id = process.uid.toString();\n\n    ctx.process.group_leader = new HashMap();\n    if (process?.pgid != null) {\n        ctx.process.group_leader.pid = process.pgid;\n    }\n    ctx.process.group_leader.group = new HashMap();\n    if (process?.gid != null) {\n        ctx.process.group_leader.group.id = process.gid.toString();\n    }\n\n    ctx.process.real_user = new HashMap();\n    if (process?.ruid != null) {\n        ctx.process.real_user.id = process.ruid.toString();\n    }               \n\n    ctx.process.real_group = new HashMap();\n    if (process?.rgid != null) {\n        ctx.process.real_group.id = process.rgid.toString();\n    }      \n\n    ctx.process.hash = ctx.process.hash ?: new HashMap();\n    if (binary?.sha1hex != null) {\n        ctx.process.hash.sha1 = binary.sha1hex;\n    }\n    if (binary?.sha256hex != null) {\n        ctx.process.hash.sha256 = binary.sha256hex;\n    }\n    \n    ctx.process.code_signature = ctx.process.code_signature ?: new HashMap();\n    if (process.signingInfo?.appid != null) {\n        ctx.process.code_signature.signing_id = process.signingInfo.appid;\n    }\n    if (process.signingInfo?.statusMessage != null) {\n        ctx.process.code_signature.status = process.signingInfo.statusMessage;\n    }\n    if (process?.signingInfo?.teamid != null) {\n        ctx.process.code_signature.team_id = process.signingInfo.teamid;\n    }\n\n    // Mapping out the parent process\n    if (ctx.jamf_protect.alerts.input.related.processes.size() > 1) {\n    def parentProcess = ctx.jamf_protect.alerts.input.related.processes[1];\n\n    ctx.process.parent = new HashMap();\n    ctx.process.parent.name = parentProcess.name;\n    ctx.process.parent.pid = parentProcess.pid;\n    ctx.process.parent.executable = parentProcess.path;\n    ctx.process.parent.entity_id = parentProcess.uuid;\n\n    if (parentProcess.containsKey('startTimestamp')) {\n        ctx.process.parent.start = Instant.ofEpochSecond(parentProcess.startTimestamp).toString();\n    }\n\n    ctx.process.parent.user = new HashMap();\n    if (parentProcess?.uid != null) {\n        ctx.process.parent.user.id = parentProcess.uid.toString();\n    }      \n\n    ctx.process.parent.real_user = new HashMap();\n    if (parentProcess?.ruid != null) {\n        ctx.process.parent.real_user.id = parentProcess.ruid.toString();\n    }               \n\n    ctx.process.parent.real_group = new HashMap();\n    if (parentProcess?.rgid != null) {\n        ctx.process.parent.real_group.id = parentProcess.rgid.toString();\n    }\n\n    ctx.process.parent.code_signature = ctx.process.parent.code_signature ?: new HashMap();\n    if (parentProcess.signingInfo?.appid != null) {\n        ctx.process.parent.code_signature.signing_id = parentProcess.signingInfo.appid;\n    }\n    if (parentProcess.signingInfo?.statusMessage != null) {\n        ctx.process.parent.code_signature.status = parentProcess.signingInfo.statusMessage;\n    }\n    if (parentProcess?.signingInfo?.teamid != null) {\n        ctx.process.parent.code_signature.team_id = parentProcess.signingInfo.teamid;\n    }\n\n    }\n\n    // Mapping out the process group leader, which can be the same as parent\n    def processGroupLeader = ctx.jamf_protect.alerts.input.related.processes[ctx.jamf_protect.alerts.input.related.processes.size() - 1];\n    ctx.process.group_leader = new HashMap();\n    ctx.process.group_leader.name = processGroupLeader.name;\n    ctx.process.group_leader.pid = processGroupLeader.pid;\n    ctx.process.group_leader.executable = processGroupLeader.path;\n\n    if (processGroupLeader.containsKey('startTimestamp')) {\n        ctx.process.group_leader.start = Instant.ofEpochSecond(processGroupLeader.startTimestamp).toString();\n    }\n\n    ctx.process.group_leader.user = new HashMap();\n    if (processGroupLeader?.uid != null) {\n        ctx.process.group_leader.user.id = processGroupLeader.uid.toString();\n    }      \n\n    ctx.process.group_leader.real_user = new HashMap();\n    if (processGroupLeader?.ruid != null) {\n        ctx.process.group_leader.real_user.id = processGroupLeader.ruid.toString();\n    }               \n\n    ctx.process.group_leader.real_group = new HashMap();\n    if (processGroupLeader?.rgid != null) {\n        ctx.process.group_leader.real_group.id = processGroupLeader.rgid.toString();\n    }\n}\n"#;

/// Verbatim from `crates/dfe-transforms/src/filebeat/jamf_protect_alerts/default.rs`.
const GROUP_CAPTURE: &str = r#"if (ctx.jamf_protect?.alerts?.input?.related?.groups != null && ctx.jamf_protect.alerts.input.related.groups.size() > 0) {\n    def group = ctx.jamf_protect.alerts.input.related.groups[0];\n    \n        ctx.group = ctx.group ?: new HashMap();\n        \n        ctx.group.name = group.name;\n        ctx.group.id = group.gid.toString();\n}\n"#;

/// Verbatim from `crates/dfe-transforms/src/filebeat/jamf_protect_alerts/default.rs`.
const USERS_CAPTURE: &str = r#"if (ctx.jamf_protect?.alerts?.input?.related?.users != null && ctx.jamf_protect.alerts.input.related.users.size() > 0) {\n    ArrayList userNames = new ArrayList();\n\n    for (def user : ctx.jamf_protect.alerts.input.related.users) {\n        if (user.containsKey('name') && user['name'] != null) {\n            userNames.add(user['name']);\n        }\n    }\n    if (userNames.size() > 0) {\n        ctx.related = ctx.related ?: new HashMap();\n        ctx.related.user = userNames;\n    }\n}  \n"#;

/// Verbatim from `crates/dfe-transforms/src/filebeat/crowdstrike_alert/automated_lead.rs`.
const CROWDSTRIKE_INDICATOR: &str = r#"def indicator = ctx.crowdstrike.alert.threatgraph_indicators[0];\nif (indicator.host_id != null && indicator.host_id != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.id = indicator.host_id;\n}\nif (indicator.hostname != null && indicator.hostname != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.name = indicator.hostname;\n}\nif (indicator.process_id != null && indicator.process_id != '') {\n  ctx.process = ctx.process ?: [:];\n  ctx.process.entity_id = indicator.process_id;\n}"#;

/// Verbatim from `crates/dfe-transforms/src/filebeat/lyve_cloud_audit/audit_lc.rs`.
const LYVE_CLOUD_CLIENT_IP: &str =
    r#"ctx.client = new HashMap(); ctx.client[\"ip\"] = ctx.related.ip[-1];"#;

/// Verbatim from `crates/dfe-transforms/src/filebeat/sysdig_event/default.rs`.
const SYSDIG_PARSE_DATE: &str = r#"def parseDate(def rawtimestamp) {\n  long timestamp;\n  if (rawtimestamp instanceof String) {\n    timestamp = Long.parseLong(rawtimestamp);\n  } else if (rawtimestamp instanceof long) {\n    timestamp = (long) rawtimestamp;\n  }\n  if (String.valueOf(timestamp).length() == 19) {\n    long epoch = timestamp / 1000000000L;\n    long seconds = timestamp % 1000000000L;\n    return Instant.ofEpochSecond(epoch, seconds).atZone(ZoneOffset.UTC);\n  }\n  return '';\n} if (ctx.json?.timestamp != null) {\n  ctx.json.timestamp = parseDate(ctx.json.timestamp);\n} if (ctx.sysdig?.event?.content?.fields?.proc?.pid_ts != null) {\n  ctx.sysdig.event.content.fields.proc.pid_ts = parseDate(ctx.sysdig.event.content.fields.proc.pid_ts);\n} if (ctx.sysdig?.event?.content?.fields?.proc?.ppid_ts != null) {\n  ctx.sysdig.event.content.fields.proc.ppid_ts = parseDate(ctx.sysdig.event.content.fields.proc.ppid_ts);\n}\n"#;

/// Verbatim from `crates/dfe-transforms/src/filebeat/gitlab_api/default.rs`.
const GITLAB_API_PARAMS: &str = r#"def keyValuePairs = [];\nfor (item in ctx.gitlab.api.params) {\n  def key = item.key;\n  def value = item.value;\n  def keyValueObject = [key: value];\n  keyValuePairs.add(keyValueObject)\n}\nctx.gitlab.api.params = keyValuePairs;\n"#;

/// A parse of the call site's text, resolved the way the ladder resolves it.
fn parse(script: &str) -> ElementMapping {
    parse_element_mapping(&normalise(script))
        .unwrap_or_else(|| panic!("declined a capture it should read: {script}"))
}

fn declines(script: &str) -> bool {
    parse_element_mapping(&normalise(script)).is_none()
}

/// One related file, as the corpus ships it -- an unsigned app bundle whose
/// hashes are empty strings.
fn one_file() -> Event {
    Event::new(json!({
        "jamf_protect": { "alerts": { "input": { "related": { "files": [ {
            "path": "/Applications/Setapp.app",
            "size": 96,
            "inode": 22894871,
            "gid": 20,
            "mode": 16877,
            "uid": 501,
            "sha1hex": "",
            "sha256hex": "",
            "signingInfo": {
                "appid": "com.setapp.DesktopClient",
                "statusMessage": "No error.",
                "teamid": "MEHY5QF425"
            }
        } ] } } } }
    }))
}

#[test]
fn the_file_capture_writes_every_member_including_the_coerced_ones() {
    let pattern = parse(FILE_CAPTURE);
    let mut event = one_file();
    assert!(element_mapping(&mut event, &pattern));

    assert_eq!(
        event.get("file.path"),
        Some(&json!("/Applications/Setapp.app"))
    );
    assert_eq!(event.get("file.size"), Some(&json!(96)));
    // `String.valueOf` renders the number, so these are strings and the raw
    // members are not.
    assert_eq!(event.get("file.inode"), Some(&json!("22894871")));
    assert_eq!(event.get("file.gid"), Some(&json!("20")));
    assert_eq!(event.get("file.mode"), Some(&json!("16877")));
    assert_eq!(event.get("file.uid"), Some(&json!("501")));
    // Empty strings survive here; the pipeline's own drop-empty takes them.
    assert_eq!(event.get("file.hash.sha1"), Some(&json!("")));
    assert_eq!(event.get("file.hash.sha256"), Some(&json!("")));
    assert_eq!(
        event.get("file.code_signature.signing_id"),
        Some(&json!("com.setapp.DesktopClient"))
    );
    assert_eq!(
        event.get("file.code_signature.status"),
        Some(&json!("No error."))
    );
    assert_eq!(
        event.get("file.code_signature.team_id"),
        Some(&json!("MEHY5QF425"))
    );
}

#[test]
fn an_absent_member_lands_as_the_null_painless_assigns() {
    let pattern = parse(FILE_CAPTURE);
    let mut event = Event::new(json!({
        "jamf_protect": { "alerts": { "input": { "related": { "files": [ {
            "path": "/tmp/x"
        } ] } } } }
    }));
    assert!(element_mapping(&mut event, &pattern));
    // A bare assignment of an absent member.
    assert_eq!(event.get("file.size"), Some(&json!(null)));
    // Java's `String.valueOf(null)` is the four-character word, which the
    // vendor's drop-empty keeps -- so this is what Elasticsearch stores too.
    assert_eq!(event.get("file.inode"), Some(&json!("null")));
    // Safe navigation through an absent `signingInfo` reaches nothing.
    assert_eq!(event.get("file.code_signature.status"), Some(&json!(null)));
}

#[test]
fn an_empty_file_list_writes_nothing_and_still_claims_the_script() {
    let pattern = parse(FILE_CAPTURE);
    let mut event = Event::new(json!({
        "jamf_protect": { "alerts": { "input": { "related": { "files": [] } } } }
    }));
    assert!(element_mapping(&mut event, &pattern));
    assert_eq!(event.get("file"), None);
}

/// Three processes and one binary: the current process, its parent, and a
/// group leader that is neither.
fn three_processes() -> Event {
    Event::new(json!({
        "jamf_protect": { "alerts": { "input": { "related": {
            "binaries": [ { "sha1hex": "aa11", "sha256hex": "bb22" } ],
            "processes": [
                {
                    "name": "child", "path": "/bin/child", "pid": 10,
                    "uid": 0, "gid": 0, "pgid": 8, "ruid": 0, "rgid": 0,
                    "uuid": "u-child", "responsiblePID": 77,
                    "args": ["/bin/child"], "startTimestamp": 1700566364,
                    "signingInfo": {
                        "appid": "id.child",
                        "statusMessage": "No error.",
                        "teamid": "T1"
                    }
                },
                {
                    "name": "parent", "path": "/bin/parent", "pid": 9,
                    "uid": 501, "ruid": 501, "rgid": 20,
                    "uuid": "u-parent", "startTimestamp": 1700566000,
                    "signingInfo": {
                        "appid": "id.parent",
                        "statusMessage": "No error.",
                        "teamid": "T2"
                    }
                },
                {
                    "name": "leader", "path": "/bin/leader", "pid": 1,
                    "uid": 0, "ruid": 0, "rgid": 0
                }
            ]
        } } } }
    }))
}

#[test]
fn the_process_capture_writes_the_current_process_and_its_binary() {
    let pattern = parse(PROCESS_CAPTURE);
    let mut event = three_processes();
    assert!(element_mapping(&mut event, &pattern));

    assert_eq!(event.get("process.name"), Some(&json!("child")));
    assert_eq!(event.get("process.executable"), Some(&json!("/bin/child")));
    assert_eq!(event.get("process.pid"), Some(&json!(10)));
    assert_eq!(event.get("process.entity_id"), Some(&json!("u-child")));
    assert_eq!(event.get("process.args"), Some(&json!(["/bin/child"])));
    assert_eq!(event.get("process.user.id"), Some(&json!("0")));
    assert_eq!(event.get("process.real_user.id"), Some(&json!("0")));
    assert_eq!(event.get("process.real_group.id"), Some(&json!("0")));
    // Off `binaries[0]`, not the process.
    assert_eq!(event.get("process.hash.sha1"), Some(&json!("aa11")));
    assert_eq!(event.get("process.hash.sha256"), Some(&json!("bb22")));
    assert_eq!(
        event.get("process.code_signature.signing_id"),
        Some(&json!("id.child"))
    );
    assert_eq!(
        event.get("process.code_signature.team_id"),
        Some(&json!("T1"))
    );
    // `Instant.ofEpochSecond(long).toString()` -- seconds precision, `Z`.
    assert_eq!(
        event.get("process.start"),
        Some(&json!("2023-11-21T11:32:44Z"))
    );
    // The guard declines: this process carries no exit code.
    assert_eq!(event.get("process.exit_code"), None);
}

#[test]
fn the_second_process_replaces_the_parent_built_from_the_responsible_pid() {
    let pattern = parse(PROCESS_CAPTURE);
    let mut event = three_processes();
    assert!(element_mapping(&mut event, &pattern));

    // `ctx.process.parent = new HashMap();` inside the size() > 1 branch
    // REPLACES the map holding `responsiblePID`, so 77 is gone.
    assert_eq!(event.get("process.parent.pid"), Some(&json!(9)));
    assert_eq!(event.get("process.parent.name"), Some(&json!("parent")));
    assert_eq!(
        event.get("process.parent.executable"),
        Some(&json!("/bin/parent"))
    );
    assert_eq!(
        event.get("process.parent.entity_id"),
        Some(&json!("u-parent"))
    );
    assert_eq!(
        event.get("process.parent.start"),
        Some(&json!("2023-11-21T11:26:40Z"))
    );
    assert_eq!(event.get("process.parent.user.id"), Some(&json!("501")));
    assert_eq!(
        event.get("process.parent.real_user.id"),
        Some(&json!("501"))
    );
    assert_eq!(
        event.get("process.parent.real_group.id"),
        Some(&json!("20"))
    );
    assert_eq!(
        event.get("process.parent.code_signature.team_id"),
        Some(&json!("T2"))
    );
}

#[test]
fn the_last_process_replaces_the_group_leader_and_takes_its_group_with_it() {
    let pattern = parse(PROCESS_CAPTURE);
    let mut event = three_processes();
    assert!(element_mapping(&mut event, &pattern));

    assert_eq!(
        event.get("process.group_leader.name"),
        Some(&json!("leader"))
    );
    assert_eq!(event.get("process.group_leader.pid"), Some(&json!(1)));
    assert_eq!(
        event.get("process.group_leader.executable"),
        Some(&json!("/bin/leader"))
    );
    assert_eq!(event.get("process.group_leader.user.id"), Some(&json!("0")));
    assert_eq!(
        event.get("process.group_leader.real_user.id"),
        Some(&json!("0"))
    );
    assert_eq!(
        event.get("process.group_leader.real_group.id"),
        Some(&json!("0"))
    );
    // The leader carries no start timestamp, so the guarded write is skipped.
    assert_eq!(event.get("process.group_leader.start"), None);
    // The FIRST group_leader map held `pid` from `pgid` and `group.id` from
    // `gid`. `= new HashMap()` replaced it, and Elasticsearch emits no
    // `group_leader.group` for the same reason.
    assert_eq!(event.get("process.group_leader.group"), None);
}

#[test]
fn a_single_process_is_its_own_group_leader_and_keeps_the_responsible_pid() {
    let pattern = parse(PROCESS_CAPTURE);
    let mut event = Event::new(json!({
        "jamf_protect": { "alerts": { "input": { "related": {
            "binaries": [ { "sha1hex": "aa11", "sha256hex": "bb22" } ],
            "processes": [ {
                "name": "solo", "path": "/bin/solo", "pid": 5,
                "uid": 0, "ruid": 0, "rgid": 0, "responsiblePID": 5,
                "exitCode": 3
            } ]
        } } } }
    }));
    assert!(element_mapping(&mut event, &pattern));

    // No second process, so the parent keeps what the responsible pid put there.
    assert_eq!(event.get("process.parent.pid"), Some(&json!(5)));
    assert_eq!(event.get("process.parent.name"), None);
    // `size() - 1` on a one-item list is element zero.
    assert_eq!(event.get("process.group_leader.name"), Some(&json!("solo")));
    // The guard holds this time.
    assert_eq!(event.get("process.exit_code"), Some(&json!(3)));
    // `process.args = process.args ?: new ArrayList()` with no args.
    assert_eq!(event.get("process.args"), Some(&json!([])));
}

#[test]
fn the_group_capture_writes_the_name_and_the_coerced_gid() {
    let pattern = parse(GROUP_CAPTURE);
    let mut event = Event::new(json!({
        "jamf_protect": { "alerts": { "input": { "related": {
            "groups": [ { "gid": 20, "name": "staff", "uuid": "Z41L97RNJT14" } ]
        } } } }
    }));
    assert!(element_mapping(&mut event, &pattern));
    assert_eq!(event.get("group.name"), Some(&json!("staff")));
    assert_eq!(event.get("group.id"), Some(&json!("20")));
}

#[test]
fn an_existing_parent_map_survives_the_null_coalescing_creation() {
    let pattern = parse(GROUP_CAPTURE);
    let mut event = Event::new(json!({
        "group": { "domain": "local" },
        "jamf_protect": { "alerts": { "input": { "related": {
            "groups": [ { "gid": 0, "name": "wheel" } ]
        } } } }
    }));
    assert!(element_mapping(&mut event, &pattern));
    assert_eq!(event.get("group.domain"), Some(&json!("local")));
    assert_eq!(event.get("group.id"), Some(&json!("0")));
}

// -- What the grammar refuses --------------------------------------------

#[test]
fn the_fourth_jamf_capture_is_a_loop_and_is_declined() {
    // Same source, same "capture the related X" description, and a different
    // pattern: it walks the whole list into a new one rather than taking an
    // element. Claiming it would write nothing for `related.user`.
    assert!(declines(USERS_CAPTURE));
}

#[test]
fn crowdstrikes_indexed_copies_are_left_to_their_own_reader() {
    // A `def` binding, a map creation and member writes -- the same skeleton --
    // but the creation is `[:]` and every guard also tests against the empty
    // string. `IndexedFieldCopies` reads it and this must not.
    assert!(declines(CROWDSTRIKE_INDICATOR));
}

#[test]
fn a_last_element_take_with_no_binding_is_declined() {
    assert!(declines(LYVE_CLOUD_CLIENT_IP));
}

#[test]
fn a_script_declaring_a_helper_is_declined() {
    assert!(declines(SYSDIG_PARSE_DATE));
}

#[test]
fn a_script_walking_the_whole_list_is_declined() {
    assert!(declines(GITLAB_API_PARAMS));
}

#[test]
fn a_script_that_does_not_build_its_parent_is_declined() {
    // Painless throws on `ctx.file.path = ...` with `ctx.file` absent, so this
    // is not a script that ever ran.
    let script = r"def file = ctx.a.related.files[0];\nctx.file.path = file.path;\nctx.file.size = file.size;\n";
    assert!(declines(script));
}

#[test]
fn a_single_member_take_stays_with_the_first_element_reader() {
    let script = r"def file = ctx.a.related.files[0];\nctx.file = ctx.file ?: new HashMap();\nctx.file.path = file.path;\n";
    assert!(declines(script));
}

#[test]
fn an_else_arm_is_declined_rather_than_half_read() {
    let script = r"def file = ctx.a.related.files[0];\nctx.file = ctx.file ?: new HashMap();\nif (file.path != null) {\n  ctx.file.path = file.path;\n} else {\n  ctx.file.path = file.fallback;\n}\nctx.file.size = file.size;\n";
    assert!(declines(script));
}

#[test]
fn a_subscript_counted_off_another_list_is_declined() {
    let script = r"def item = ctx.a.left[ctx.a.right.size() - 1];\nctx.out = ctx.out ?: new HashMap();\nctx.out.name = item.name;\nctx.out.id = item.id;\n";
    assert!(declines(script));
}

#[test]
fn a_default_this_cannot_name_is_declined() {
    let script = r"def item = ctx.a.list[0];\nctx.out = ctx.out ?: new HashMap();\nctx.out.name = item.name;\nctx.out.tags = item.tags ?: new LinkedHashSet();\n";
    assert!(declines(script));
}

#[test]
fn trailing_text_the_grammar_cannot_read_declines_the_whole_script() {
    let script = r"def item = ctx.a.list[0];\nctx.out = ctx.out ?: new HashMap();\nctx.out.name = item.name;\nctx.out.id = item.id;\nctx.out.entrySet().removeIf(e -> e.getValue() == null);\n";
    assert!(declines(script));
}

/// Verbatim from `crates/dfe-transforms/src/filebeat/azure_auditlogs/default.rs`,
/// the one script `ArrayToIndexedObject` exists for.
const AZURE_TARGET_RESOURCES: &str = r#"if (ctx.azure.auditlogs.properties?.targetResources != null) {\n  ctx.azure.auditlogs.properties.target_resources = new HashMap();\n  for (def i = 0; i < ctx.azure.auditlogs.properties.targetResources.length; i++) {\n    String index = String.valueOf(i);\n    ctx.azure.auditlogs.properties.target_resources[index] = new HashMap();\n    if(ctx.azure.auditlogs.properties.targetResources[i].displayName != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].display_name = ctx.azure.auditlogs.properties.targetResources[i].displayName;\n    }\n    ctx.azure.auditlogs.properties.target_resources[index].id = ctx.azure.auditlogs.properties.targetResources[i].id;\n    ctx.azure.auditlogs.properties.target_resources[index].type = ctx.azure.auditlogs.properties.targetResources[i].type;\n    if (ctx.azure.auditlogs.properties.targetResources[i].ipAddress != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].ip_address = ctx.azure.auditlogs.properties.targetResources[i].ipAddress;\n    }\n    if (ctx.azure.auditlogs.properties.targetResources[i].userPrincipalName != null) {\n      ctx.azure.auditlogs.properties.target_resources[index].user_principal_name = ctx.azure.auditlogs.properties.targetResources[i].userPrincipalName;\n    }\n    ctx.azure.auditlogs.properties.target_resources[index].modified_properties = new HashMap();\n    for (def j = 0; j < ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties.length; j++) {\n      String n = String.valueOf(j);\n      ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n] = new HashMap();\n\n      ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].display_name = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].displayName;\n      \n      if (ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].newValue != null) {\n        ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].new_value = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].newValue;\n      }\n      if (ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].oldValue != null) {\n        ctx.azure.auditlogs.properties.target_resources[index].modified_properties[n].old_value = ctx.azure.auditlogs.properties.targetResources[i].modifiedProperties[j].oldValue;\n      }\n    }\n  }\n  ctx.azure.auditlogs.properties.remove('targetResources');\n}"#;

#[test]
fn azures_rekeyed_array_is_declined() {
    // A loop, and a subscripted target the grammar has no statement for.
    assert!(declines(AZURE_TARGET_RESOURCES));
}
