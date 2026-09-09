// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! What `painless_exec` costs per EVENT.
//!
//! The script is a literal at the call site and never changes, but the text is
//! re-normalised and re-scanned on every event. These measure that: a script
//! the first matcher claims, one that falls to the end of the ladder, and one
//! nothing matches -- the last being the worst case and, at ~45% of scripts,
//! the common one.
//!
//! Run with: `cargo bench -p dfe-painless`

use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use dfe_core::event::Event;
use dfe_painless::common::normalise;
use dfe_painless::plan::{
    PainlessPlan, painless_exec, painless_exec_params, painless_exec_plan,
    painless_exec_plan_params,
};
use serde_json::json;

/// Verbatim from `pipelines/crowdstrike/default.yml`, escapes and all -- the
/// generated modules pass the script exactly as it is emitted.
const SENTINEL: &str = "ctx.crowdstrike.event.entrySet().removeIf(entry -> \
                        params.values.contains(entry.getValue()));\\n";

/// Last rung of the text-only ladder.
const OKTA_TARGET: &str = "for (item in ctx.okta.target) {\\n  if (item.alternateId != null) \
                           {\\n    item.alternate_id = item.alternateId;\\n  }\\n}";

/// A run of guarded copies, which is what windows' `security_standard` ships
/// four hundred lines of -- one per winlog field, each in its own `!= null`
/// block. `KnownPattern::GuardedCopy` claims it, and the walk over the body is
/// the per-event cost this measures.
const GUARDED_COPIES: &str = "if (ctx.winlog.event_data.SubjectUserName != null) \
                              {\\n  ctx.user.name = ctx.winlog.event_data.SubjectUserName;\\n}\\n\
                              if (ctx.winlog.event_data.SubjectDomainName != null) \
                              {\\n  ctx.user.domain = ctx.winlog.event_data.SubjectDomainName;\\n}\
                              \\nif (ctx.winlog.event_data.TargetUserName != null) \
                              {\\n  ctx.user.target.name = ctx.winlog.event_data.TargetUserName;\
                              \\n}\\nif (ctx.winlog.event_data.IpAddress != null) \
                              {\\n  ctx.source.ip = ctx.winlog.event_data.IpAddress;\\n}\\n\
                              if (ctx.winlog.event_data.IpPort != null) \
                              {\\n  ctx.source.port = ctx.winlog.event_data.IpPort;\\n}\\n\
                              if (ctx.winlog.event_data.WorkstationName != null) \
                              {\\n  ctx.host.hostname = ctx.winlog.event_data.WorkstationName;\\n}\
                              \\nif (ctx.winlog.event_data.ProcessName != null) \
                              {\\n  ctx.process.executable = ctx.winlog.event_data.ProcessName;\
                              \\n}\\nif (ctx.winlog.event_data.LogonType != null) \
                              {\\n  ctx.winlog.logon.type = ctx.winlog.event_data.LogonType;\\n}\\n\
                              if (ctx.winlog.event_data.Status != null) \
                              {\\n  ctx.event.code = ctx.winlog.event_data.Status;\\n}\\n\
                              if (ctx.winlog.event_data.ServiceName != null) \
                              {\\n  ctx.service.name = ctx.winlog.event_data.ServiceName;\\n}\\n\
                              if (ctx.source.ip != null) {\\n  ctx.related.ip.add(ctx.source.ip);\
                              \\n}\\nif (ctx.user.name != null) \
                              {\\n  ctx.related.user.add(ctx.user.name);\\n}\\n";

/// zeek's connection stream: one field decided by a truth table over two
/// others, written as four sequential `if`s each ending in a `return`.
/// `KnownPattern::PairTable` claims it. The event below takes the LAST row, so
/// all four comparisons run -- the first row would measure one.
const PAIR_TABLE: &str = "if (ctx.zeek?.connection?.local_orig == null ||\\n    \
                          ctx.zeek?.connection?.local_resp == null) {\\n  return;\\n}\\n\
                          if (ctx.zeek.connection.local_orig == true &&\\n    \
                          ctx.zeek.connection.local_resp == true) {\\n  \
                          ctx.network.direction = \"internal\";\\n  return;\\n}\\n\
                          if (ctx.zeek.connection.local_orig == true &&\\n    \
                          ctx.zeek.connection.local_resp == false) {\\n  \
                          ctx.network.direction = \"outbound\";\\n  return;\\n}\\n\
                          if (ctx.zeek.connection.local_orig == false &&\\n    \
                          ctx.zeek.connection.local_resp == true) {\\n  \
                          ctx.network.direction = \"inbound\";\\n  return;\\n}\\n\
                          if (ctx.zeek.connection.local_orig == false &&\\n    \
                          ctx.zeek.connection.local_resp == false) {\\n  \
                          ctx.network.direction = \"external\";\\n  return;\\n}";

/// zeek's DNS stream: one member lifted out of every record of a list into a
/// list of its own, and dropped from the record it came from.
/// `KnownPattern::HoistMember` claims it. The cost scales with the list, so
/// the event below carries several answers rather than one.
const HOIST_MEMBER: &str = "def answers = ctx.dns.answers; def iplist = new ArrayList(); \
                            for (def i = 0; i < ctx.dns.answers.length; i++) {\\n  \
                            if (answers[i].containsKey(\"tmpip\")) {\\n    \
                            iplist.add(answers[i].tmpip);\\n    \
                            answers[i].remove(\"tmpip\");\\n  }\\n} \
                            ctx.dns.resolved_ip = iplist;";

/// panw_cortex_xdr's MITRE list, cut on one separator and fanned out to two
/// deduped lists. `KnownPattern::SplitFanOut` claims it. The cost scales with
/// the list and with the dedup scan, so the event below carries the four
/// techniques and the repeated tactic its own stream sends.
const SPLIT_FAN_OUT: &str = "void addTechnique(def ctx, def x, def y) {\\n  \
                             if (ctx.threat == null) {\\n    ctx.threat = new HashMap();\\n  }\\n  \
                             if (ctx.threat.technique == null) {\\n    \
                             ctx.threat.technique = new HashMap();\\n  }\\n  \
                             if (ctx.threat.technique.id == null) {\\n    \
                             ctx.threat.technique.id = new ArrayList();\\n  }\\n  \
                             if (ctx.threat.technique.name == null) {\\n    \
                             ctx.threat.technique.name = new ArrayList();\\n  }\\n  \
                             if (!ctx.threat.technique.id.contains(x)) {\\n    \
                             ctx.threat.technique.id.add(x);\\n  }\\n  \
                             if (!ctx.threat.technique.name.contains(y)) {\\n    \
                             ctx.threat.technique.name.add(y);\\n  }\\n}\\n\
                             for (mitre_technique in \
                             ctx.panw_cortex.xdr.mitre_technique_id_and_name) {\\n  \
                             addTechnique(ctx, mitre_technique.splitOnToken(' - ')[0], \
                             mitre_technique.splitOnToken(' - ')[1]);\\n}";

/// ti_threatq's sources, giving up two members in one walk with an allow-list
/// on the second. `KnownPattern::GatherMembers` claims it. The cost scales with
/// the record count and is paid once per column, so the event carries several.
const GATHER_MEMBERS: &str = "def ecsTlps = ['WHITE', 'GREEN', 'AMBER', 'RED', 'CLEAR', 'AMBER+STRICT'];\\n\
     def providers = new ArrayList();\\ndef tlps = new ArrayList();\\n\
     for (source in ctx.threatq.sources) {\\n  if (source == null) {\\n    return;\\n  }\\n  \
     if (source.containsKey(\"name\") && source[\"name\"] != null) {\\n    \
     providers.add(source[\"name\"]);\\n  }\\n  \
     if (source.containsKey(\"tlp_name\") && source[\"tlp_name\"] != null && \
     ecsTlps.contains(source[\"tlp_name\"])) {\\n    tlps.add(source[\"tlp_name\"]);\\n  }\\n}\\n\
     if (tlps.size() > 0) {\\n  if (ctx.threat.indicator.marking == null) {\\n    \
     ctx.threat.indicator.marking = new HashMap();\\n  }\\n  \
     ctx.threat.indicator.marking.tlp = tlps;\\n}\\nif (providers.size() > 0) {\\n  \
     if (ctx.threat.indicator.provider == null) {\\n    \
     ctx.threat.indicator.provider = new HashMap();\\n  }\\n  \
     ctx.threat.indicator.provider = providers;\\n}";

/// claude_code's classification: two literal arms, then a params table.
/// `ParamsPattern::ArmedTable` claims it. Benched on a name the TABLE answers,
/// which is the branch the arms fall through to and the one that carries the
/// lookup.
const ARMED_TABLE: &str = "def name = ctx.event_name; ctx.event = ctx.event ?: new HashMap(); \
                           ctx.event.kind = 'event'; \
                           if (name == 'user_prompt' || name == 'skill_activated') {\\n  \
                           return;\\n} if (name == 'tool_decision') {\\n  \
                           ctx.event.category = ['iam'];\\n  ctx.event.type = ['info'];\\n  \
                           return;\\n} def entry = params.categories.getOrDefault(name, null); \
                           if (entry != null) {\\n  ctx.event.category = entry.category;\\n  \
                           ctx.event.type = entry.type;\\n} else {\\n  \
                           ctx.event.category = ['host'];\\n  ctx.event.type = ['info'];\\n}\\n";

/// kolide's issue lifecycle: two params tables and a deadline compared against
/// the event's own timestamp. `ParamsPattern::IssueLifecycle` claims it.
/// Benched on the document that takes every branch -- a row miss, a domain
/// append, and both timestamps parsed.
const ISSUE_LIFECYCLE: &str = "def action = ctx.event.action;\\ndef m = params.exact.get(action);\\n\
     if (m != null) {\\n  ctx.event.kind = m.kind;\\n  \
     ctx.event.category = new ArrayList(m.category);\\n} else {\\n  \
     ctx.event.kind = 'event';\\n  ctx.event.category = ['configuration'];\\n}\\n\\n\
     if (ctx.rule?.id != null) {\\n  def domain = params.check_category.get(ctx.rule.id);\\n  \
     if (domain != null && !ctx.event.category.contains(domain)) {\\n    \
     ctx.event.category.add(domain);\\n  }\\n}\\n\\n\
     boolean pendingBlock = (ctx.kolide?.issues?.blocks_device_at != null);\\n\
     boolean blocked = false;\\nif (pendingBlock && ctx['@timestamp'] != null) {\\n  \
     ZonedDateTime blockAt = ZonedDateTime.parse(ctx.kolide.issues.blocks_device_at);\\n  \
     ZonedDateTime eventTime = ZonedDateTime.parse(ctx['@timestamp']);\\n  \
     blocked = !blockAt.isAfter(eventTime);\\n}\\n\
     boolean resolved = (ctx.kolide?.issues?.resolved_at != null) || \
     (action == 'issues.resolved');\\n\
     ctx.event.type = (pendingBlock || resolved) ? ['change'] : ['creation'];\\n\\n\
     ctx._tmp = ctx._tmp == null ? [:] : ctx._tmp;\\nctx._tmp.blocked = blocked;\\n\\n\
     if (action == 'issue') {\\n  if (resolved) {\\n    ctx.event.action = 'resolved';\\n  } \
     else if (blocked) {\\n    ctx.event.action = 'blocked';\\n  } else if (pendingBlock) {\\n    \
     ctx.event.action = 'will_be_blocked';\\n  }\\n}";

/// Nothing matches this, so every matcher's scan runs before it is counted.
const UNHANDLED: &str = "def splitUnquoted(String input, String sep) {\\n  def tokens = [];\\n  \
                         def startPosition = 0;\\n  boolean inQuotes = false;\\n  for (int i = 0; \
                         i < input.length(); ++i) {\\n    if (input.charAt(i) == (char)34) {\\n   \
                         inQuotes = !inQuotes;\\n    } else if (!inQuotes && \
                         sep.indexOf(input.charAt(i)) >= 0) {\\n      \
                         tokens.add(input.substring(startPosition, i));\\n      startPosition = i \
                         + 1;\\n    }\\n  }\\n  return tokens;\\n}";

fn sentinel_event() -> Event {
    Event::new(json!({
        "crowdstrike": { "event": { "keep": "v", "empty": "", "dash": "-", "zero": 0 } },
    }))
}

/// Building the event is NOT part of what is being measured -- it costs more
/// than the dispatch does, and would swamp the comparison.
fn bench_params_matcher(c: &mut Criterion) {
    let params = json!({ "values": [null, "", "-", "N/A", "NA", 0] });
    c.bench_function("painless_exec/params_first_match", |b| {
        b.iter_batched_ref(
            sentinel_event,
            |event| painless_exec_params(event, black_box(SENTINEL), black_box(&params)),
            BatchSize::SmallInput,
        );
    });
}

fn bench_text_matcher(c: &mut Criterion) {
    c.bench_function("painless_exec/text_last_match", |b| {
        b.iter_batched_ref(
            || Event::new(json!({ "okta": { "target": [] } })),
            |event| painless_exec(event, black_box(OKTA_TARGET)),
            BatchSize::SmallInput,
        );
    });
}

fn bench_unhandled(c: &mut Criterion) {
    c.bench_function("painless_exec/unhandled", |b| {
        b.iter_batched_ref(
            || Event::new(json!({ "message": "x" })),
            |event| painless_exec(event, black_box(UNHANDLED)),
            BatchSize::SmallInput,
        );
    });
}

/// The same three with the escapes already resolved, which is what
/// `cached_script!` hands the matcher from the second event onward.
fn bench_cached(c: &mut Criterion) {
    let params = json!({ "values": [null, "", "-", "N/A", "NA", 0] });
    let sentinel = normalise(SENTINEL).into_owned();
    let okta = normalise(OKTA_TARGET).into_owned();
    let unhandled = normalise(UNHANDLED).into_owned();

    c.bench_function("painless_exec/params_first_match_cached", |b| {
        b.iter_batched_ref(
            sentinel_event,
            |event| painless_exec_params(event, black_box(&sentinel), black_box(&params)),
            BatchSize::SmallInput,
        );
    });
    c.bench_function("painless_exec/text_last_match_cached", |b| {
        b.iter_batched_ref(
            || Event::new(json!({ "okta": { "target": [] } })),
            |event| painless_exec(event, black_box(&okta)),
            BatchSize::SmallInput,
        );
    });
    c.bench_function("painless_exec/unhandled_cached", |b| {
        b.iter_batched_ref(
            || Event::new(json!({ "message": "x" })),
            |event| painless_exec(event, black_box(&unhandled)),
            BatchSize::SmallInput,
        );
    });
}

/// The same three through a [`PainlessPlan`], which is what `cached_painless!`
/// hands the runtime: dispatch decided once, so per event only the matchers
/// the text triggers run -- and for an unhandled script, nothing at all.
fn bench_planned(c: &mut Criterion) {
    let params = json!({ "values": [null, "", "-", "N/A", "NA", 0] });
    let sentinel = PainlessPlan::new(SENTINEL);
    let okta = PainlessPlan::new(OKTA_TARGET);
    let unhandled = PainlessPlan::new(UNHANDLED);

    c.bench_function("painless_exec/params_first_match_planned", |b| {
        b.iter_batched_ref(
            sentinel_event,
            |event| painless_exec_plan_params(event, black_box(&sentinel), black_box(&params)),
            BatchSize::SmallInput,
        );
    });
    c.bench_function("painless_exec/text_last_match_planned", |b| {
        b.iter_batched_ref(
            || Event::new(json!({ "okta": { "target": [] } })),
            |event| painless_exec_plan(event, black_box(&okta)),
            BatchSize::SmallInput,
        );
    });
    c.bench_function("painless_exec/unhandled_planned", |b| {
        b.iter_batched_ref(
            || Event::new(json!({ "message": "x" })),
            |event| painless_exec_plan(event, black_box(&unhandled)),
            BatchSize::SmallInput,
        );
    });
}

/// The guarded-literal walk, through the plan the runtime actually holds.
///
/// Half the guarded fields are present, so half the branches are taken and half
/// fall through -- a body where every guard failed would measure the test and
/// none of the writes.
fn bench_guarded_copies(c: &mut Criterion) {
    let plan = PainlessPlan::new(GUARDED_COPIES);
    // The measurement is worthless if an earlier matcher claims the script, and
    // the ladder is long enough that reading it is not proof.
    assert!(
        plan.binding().iter().any(|b| b.starts_with("GuardedCopy")),
        "the guarded-copy bench no longer measures GuardedCopy: {:?}",
        plan.binding(),
    );

    c.bench_function("painless_exec/guarded_copies_planned", |b| {
        b.iter_batched_ref(
            || {
                Event::new(json!({
                    "winlog": { "event_data": {
                        "SubjectUserName": "svc-backup",
                        "SubjectDomainName": "CORP",
                        "IpAddress": "10.4.7.21",
                        "IpPort": "49512",
                        "WorkstationName": "WKS-114",
                        "LogonType": "3",
                    } },
                }))
            },
            |event| painless_exec_plan(event, black_box(&plan)),
            BatchSize::SmallInput,
        );
    });
}

/// The truth table, through the plan the runtime holds.
fn bench_pair_table(c: &mut Criterion) {
    let plan = PainlessPlan::new(PAIR_TABLE);
    assert!(
        plan.binding().iter().any(|b| b.starts_with("PairTable")),
        "the pair-table bench no longer measures PairTable: {:?}",
        plan.binding(),
    );

    c.bench_function("painless_exec/pair_table_planned", |b| {
        b.iter_batched_ref(
            || {
                Event::new(json!({
                    "zeek": { "connection": { "local_orig": false, "local_resp": false } },
                    "network": { "transport": "tcp" },
                }))
            },
            |event| painless_exec_plan(event, black_box(&plan)),
            BatchSize::SmallInput,
        );
    });
}

/// The member hoist, through the plan the runtime holds.
fn bench_hoist_member(c: &mut Criterion) {
    let plan = PainlessPlan::new(HOIST_MEMBER);
    assert!(
        plan.binding().iter().any(|b| b.starts_with("HoistMember")),
        "the hoist bench no longer measures HoistMember: {:?}",
        plan.binding(),
    );

    c.bench_function("painless_exec/hoist_member_planned", |b| {
        b.iter_batched_ref(
            || {
                Event::new(json!({
                    "dns": { "answers": [
                        { "data": "a.example.com", "ttl": 60, "tmpip": "10.0.0.1" },
                        { "data": "b.example.com", "ttl": 120 },
                        { "data": "c.example.com", "ttl": 30, "tmpip": "10.0.0.2" },
                        { "data": "d.example.com", "ttl": 300, "tmpip": "10.0.0.3" },
                    ] },
                }))
            },
            |event| painless_exec_plan(event, black_box(&plan)),
            BatchSize::SmallInput,
        );
    });
}

/// The split fan-out, through the plan the runtime holds.
fn bench_split_fan_out(c: &mut Criterion) {
    let plan = PainlessPlan::new(SPLIT_FAN_OUT);
    assert!(
        plan.binding().iter().any(|b| b.starts_with("SplitFanOut")),
        "the fan-out bench no longer measures SplitFanOut: {:?}",
        plan.binding(),
    );

    c.bench_function("painless_exec/split_fan_out_planned", |b| {
        b.iter_batched_ref(
            || {
                Event::new(json!({
                    "panw_cortex": { "xdr": { "mitre_technique_id_and_name": [
                        "T1018 - Remote System Discovery",
                        "T1082 - System Information Discovery",
                        "T1016 - System Network Configuration Discovery",
                        "T1007 - System Service Discovery",
                        "T1018 - Remote System Discovery",
                    ] } },
                }))
            },
            |event| painless_exec_plan(event, black_box(&plan)),
            BatchSize::SmallInput,
        );
    });
}

/// The member gather, through the plan the runtime holds.
fn bench_gather_members(c: &mut Criterion) {
    let plan = PainlessPlan::new(GATHER_MEMBERS);
    assert!(
        plan.binding()
            .iter()
            .any(|b| b.starts_with("GatherMembers")),
        "the gather bench no longer measures GatherMembers: {:?}",
        plan.binding(),
    );

    c.bench_function("painless_exec/gather_members_planned", |b| {
        b.iter_batched_ref(
            || {
                Event::new(json!({
                    "threatq": { "sources": [
                        { "name": "TAXII Feed", "tlp_name": "AMBER" },
                        { "name": "Internal Research" },
                        { "name": "Partner Feed", "tlp_name": "PURPLE" },
                        { "name": "Vendor Feed", "tlp_name": "GREEN" },
                    ] },
                }))
            },
            |event| painless_exec_plan(event, black_box(&plan)),
            BatchSize::SmallInput,
        );
    });
}

/// The armed table, through the plan the runtime holds.
fn bench_armed_table(c: &mut Criterion) {
    let plan = PainlessPlan::new(ARMED_TABLE);
    assert!(
        plan.binding().iter().any(|b| b.starts_with("ArmedTable")),
        "the armed-table bench no longer measures ArmedTable: {:?}",
        plan.binding(),
    );
    let params = json!({ "categories": {
        "tool_result": { "category": ["process"], "type": ["info"] },
        "api_request": { "category": ["api"], "type": ["info"] },
        "api_refusal": { "category": ["api"], "type": ["denied"] },
        "mcp_server_connection": { "category": ["network"], "type": ["connection"] },
        "hook_execution_start": { "category": ["process"], "type": ["start"] },
    }});

    c.bench_function("painless_exec/armed_table_planned", |b| {
        b.iter_batched_ref(
            || Event::new(json!({ "event_name": "mcp_server_connection" })),
            |event| painless_exec_plan_params(event, black_box(&plan), black_box(&params)),
            BatchSize::SmallInput,
        );
    });
}

/// The issue lifecycle, through the plan the runtime holds.
fn bench_issue_lifecycle(c: &mut Criterion) {
    let plan = PainlessPlan::new(ISSUE_LIFECYCLE);
    assert!(
        plan.binding()
            .iter()
            .any(|b| b.starts_with("IssueLifecycle")),
        "the lifecycle bench no longer measures IssueLifecycle: {:?}",
        plan.binding(),
    );
    let params = json!({
        "exact": {},
        "check_category": { "20": "malware", "41": "vulnerability" },
    });

    c.bench_function("painless_exec/issue_lifecycle_planned", |b| {
        b.iter_batched_ref(
            || {
                Event::new(json!({
                    "event": { "action": "issue" },
                    "@timestamp": "2026-06-16T00:00:00Z",
                    "rule": { "id": "20" },
                    "kolide": { "issues": { "blocks_device_at": "2026-06-16T00:00:00.000Z" } },
                }))
            },
            |event| painless_exec_plan_params(event, black_box(&plan), black_box(&params)),
            BatchSize::SmallInput,
        );
    });
}

/// servicenow's first two processors, verbatim from the generated call sites
/// in `crates/dfe-transforms/src/filebeat/servicenow_event/default.rs`.
///
/// The copy runs first and creates the entry the wrap then wraps, which is what
/// makes `servicenow.event.timestamp_field.value` resolve downstream.
const SN_TIMESTAMP_FIELD: &str = r#"def obj = ctx.servicenow.event; if (obj.containsKey(ctx._conf.timestamp_field)) {\n    ctx.servicenow.event.timestamp_field = obj.get(ctx._conf.timestamp_field);\n}"#;

const SN_WRAP: &str = r#"for (def entry: ctx.servicenow.event.entrySet()) {\n  if (entry.getKey() == 'table_name') {\n    continue;\n  }\n  def v = entry.getValue();\n  if (v instanceof Map) {\n    continue;\n  }\n  Map n = [:];\n  if (ctx._conf.data_has_display_values == \"true\") {\n    n.display_value = v;\n  } else {\n    n.value = v;\n  }\n  entry.setValue(n);\n}\n"#;

/// The 40 columns of a servicenow asset record, from `test-event-aws.log`.
///
/// A slice rather than a `json!` literal: a map this wide exceeds rustc's macro
/// recursion limit on its own, the same way the o365 operation table does.
const SERVICENOW_COLUMNS: &[(&str, &str)] = &[
    ("table_name", "alm_hardware"),
    ("parent", ""),
    ("skip_sync", "false"),
    ("product_instance_id", ""),
    ("residual_date", "2024-09-10"),
    ("residual", "509.95"),
    ("sys_updated_on", "2024-09-10 08:15:50"),
    ("request_line", ""),
    ("resold_value", "0"),
    ("sys_updated_by", "system"),
    ("due_in", ""),
    ("model_category", "81feb9c137101000deeabfc8bcbe5dc4"),
    ("sys_created_on", "2023-08-31 18:16:40"),
    ("sys_domain", "global"),
    ("disposal_reason", ""),
    ("model", "46bbf3cba9fe1981000545a67695b505"),
    ("install_date", "2023-05-02 07:00:00"),
    ("gl_account", ""),
    ("invoice_number", ""),
    ("sys_created_by", "admin"),
    ("warranty_expiration", ""),
    ("asset_tag", "P1000241"),
    ("depreciated_amount", "190.04"),
    ("substatus", ""),
    ("pre_allocated", "false"),
    ("owned_by", ""),
    ("checked_out", ""),
    ("display_name", "P1000241 - Gateway DX Series"),
    ("sys_domain_path", "/"),
    ("asset_function", ""),
    ("delivery_date", ""),
    ("retirement_date", ""),
    ("model_component_id", ""),
    ("beneficiary", ""),
    ("install_status", "1"),
    ("cost_center", "7fb1cc99c0a80a6d30c04574d14c0acf"),
    ("supported_by", ""),
    ("assigned", "2023-08-01 08:00:00"),
    ("sys_class_name", "alm_hardware"),
    ("sys_id", "0196612a37c4200044e0bfc8bcbe5d3a"),
];

/// A servicenow asset record, at the width the vendor sends.
///
/// The wrap's cost is per ENTRY, so the width is the measurement: a two-key
/// event would measure the dispatch and nothing else.
fn servicenow_event() -> serde_json::Value {
    let columns: serde_json::Map<String, serde_json::Value> = SERVICENOW_COLUMNS
        .iter()
        .map(|(key, value)| ((*key).to_owned(), json!(value)))
        .collect();
    json!({
        "_conf": { "timestamp_field": "sys_updated_on" },
        "servicenow": { "event": columns },
    })
}

/// The one-key wrap over a real record's width, through the plan.
fn bench_wrap_entries(c: &mut Criterion) {
    let plan = PainlessPlan::new(SN_WRAP);
    assert!(
        plan.binding().iter().any(|b| b.starts_with("WrapEntries")),
        "the wrap bench no longer measures WrapEntries: {:?}",
        plan.binding(),
    );

    c.bench_function("painless_exec/wrap_entries_planned", |b| {
        b.iter_batched_ref(
            || Event::new(servicenow_event()),
            |event| painless_exec_plan(event, black_box(&plan)),
            BatchSize::SmallInput,
        );
    });
}

/// The dynamically-keyed copy that runs before it.
fn bench_key_named_by_field(c: &mut Criterion) {
    let plan = PainlessPlan::new(SN_TIMESTAMP_FIELD);
    assert!(
        plan.binding()
            .iter()
            .any(|b| b.starts_with("KeyNamedByField")),
        "the copy bench no longer measures KeyNamedByField: {:?}",
        plan.binding(),
    );

    c.bench_function("painless_exec/key_named_by_field_planned", |b| {
        b.iter_batched_ref(
            || Event::new(servicenow_event()),
            |event| painless_exec_plan(event, black_box(&plan)),
            BatchSize::SmallInput,
        );
    });
}

/// tetragon's lift, verbatim from its generated call site.
const TETRAGON_LIFT: &str = r#"void run(Map map) {\n  for (def k : map?.cilium_tetragon?.log?.keySet()) {\n    if (k == \"process_exec\" ||\n        k == \"process_exit\" ||\n        k == \"process_kprobe\") {\n      if (map?._tmp_ == null) {\n        map[\"_tmp_\"] = new HashMap();\n      }\n      map[\"_tmp_\"][\"process\"] = map.cilium_tetragon.log[k].process;\n    }\n\n    if (k == \"process_exec\" ||\n        k == \"process_exit\" ||\n        k == \"process_kprobe\") {\n      if (map?._tmp_ == null) {\n        map[\"_tmp_\"] = new HashMap();\n      }\n      map[\"_tmp_\"][\"parent\"] = map.cilium_tetragon.log[k].parent;\n    }\n  }\n}\n\nrun(ctx);\n"#;

/// zerofox's root prune, verbatim from its generated call site.
const ZEROFOX_PRUNE: &str = r#"ctx?.zerofox?.entrySet().removeIf(entry -> entry.getValue() == null || entry.getValue().equals(\"\") || (entry.getValue() instanceof List && entry.getValue().length == 0) || (entry.getValue() instanceof Map && entry.getValue().size() == 0));"#;

/// A tetragon `process_exec` event, at the width the vendor sends.
///
/// The lift clones two subtrees and the aliasing is per RENAME, so the members
/// carry their real field counts rather than a token key apiece.
fn tetragon_event() -> serde_json::Value {
    json!({ "cilium_tetragon": { "log": {
        "node_name": "kind-control-plane",
        "process_exec": {
            "process": {
                "exec_id": "a2luZC1jb250cm9sLXBsYW5lOjY5NjM5MjAwMDAwMDAwOjIyNDM5NQ==",
                "pid": 224_395, "uid": 0, "cwd": "/",
                "binary": "/usr/local/bin/local-path-provisioner",
                "arguments": "--debug start --config /etc/config/config.json",
                "flags": "procFS auid rootcwd",
                "start_time": "2024-10-18T22:12:37.150989227Z",
                "auid": 4_294_967_295i64,
                "pod": {
                    "namespace": "local-path-storage",
                    "name": "local-path-provisioner-57c5987fd4-vd668",
                    "container": {
                        "id": "containerd://aac7662884b96b0351a3529cf7c38311a48b7468",
                        "name": "local-path-provisioner",
                        "image": { "id": "sha256:282f619d10d4", "name": "docker.io/kindest/x" },
                        "start_time": "2024-10-18T22:12:37Z", "pid": 1
                    },
                    "workload": "local-path-provisioner", "workload_kind": "Deployment"
                },
                "tid": 224_395
            },
            "parent": {
                "exec_id": "a2luZC1jb250cm9sLXBsYW5lOjY5NjM3MTYwMDAwMDAwOjIyMzk2NQ==",
                "pid": 223_965, "uid": 0,
                "cwd": "/run/containerd/io.containerd.runtime.v2.task/k8s.io/6aa99f632d",
                "binary": "/usr/local/bin/containerd-shim-runc-v2",
                "arguments": "-namespace k8s.io -address /run/containerd/containerd.sock",
                "flags": "procFS auid",
                "start_time": "2024-10-18T22:12:35.110989185Z",
                "auid": 4_294_967_295i64, "tid": 223_965
            }
        },
        "time": "2024-10-18T22:12:37.150989102Z"
    } } })
}

/// A zerofox alert at the width the vendor sends, half of it empty.
///
/// Built from a slice rather than a `json!` literal: the map is wide enough
/// that the macro's recursion is worth avoiding, and the prune's cost is per
/// ENTRY, so the width IS the measurement.
fn zerofox_event() -> serde_json::Value {
    const FILLED: &[(&str, &str)] = &[
        ("alert_type", "email"),
        ("status", "open"),
        ("severity", "3"),
        ("rule_name", "Impersonation"),
        ("network", "twitter"),
        ("offending_content_url", "https://example.invalid/x"),
        ("content_created_at", "2024-10-18T22:12:37Z"),
        ("last_modified", "2024-10-18T22:14:00Z"),
        ("timestamp", "2024-10-18T22:12:37Z"),
        ("id", "1234567"),
    ];
    const BLANK: &[&str] = &[
        "asset_term",
        "assignee",
        "entity_term",
        "protected_account",
        "protected_locations",
        "darkweb_term",
        "business_network",
        "protected_social_object",
        "notes",
        "entity_account",
        "entity_email_receiver_id",
    ];

    let mut zerofox = serde_json::Map::new();
    for (key, value) in FILLED {
        zerofox.insert((*key).to_owned(), json!(value));
    }
    for key in BLANK {
        zerofox.insert((*key).to_owned(), json!(""));
    }
    zerofox.insert("reviews".to_owned(), json!([]));
    zerofox.insert("content_actions".to_owned(), json!([]));
    zerofox.insert("tags".to_owned(), json!([]));
    zerofox.insert("metadata".to_owned(), json!({}));
    zerofox.insert("logs".to_owned(), serde_json::Value::Null);

    json!({ "zerofox": zerofox })
}

/// The variant-key lift, whose members are now aliased rather than copied.
fn bench_member_from_variant_key(c: &mut Criterion) {
    let plan = PainlessPlan::new(TETRAGON_LIFT);
    assert!(
        plan.binding()
            .iter()
            .any(|b| b.starts_with("MemberFromVariantKey")),
        "the lift bench no longer measures MemberFromVariantKey: {:?}",
        plan.binding(),
    );

    c.bench_function("painless_exec/member_from_variant_key_planned", |b| {
        b.iter_batched_ref(
            || Event::new(tetragon_event()),
            |event| painless_exec_plan(event, black_box(&plan)),
            BatchSize::SmallInput,
        );
    });
}

/// The renames the lift feeds, which are what the aliasing costs: each one
/// mirrors its removal onto the variant subtree.
fn bench_aliased_renames(c: &mut Criterion) {
    const RENAMES: &[(&str, &str)] = &[
        ("_tmp_.process.arguments", "process.args"),
        ("_tmp_.process.binary", "process.executable"),
        ("_tmp_.process.cwd", "process.working_directory"),
        ("_tmp_.process.pid", "process.pid"),
        ("_tmp_.process.exec_id", "process.entity_id"),
        ("_tmp_.process.tid", "process.thread.id"),
        ("_tmp_.process.uid", "process.user.id"),
        ("_tmp_.process.start_time", "process.start"),
        ("_tmp_.parent.arguments", "process.parent.args"),
        ("_tmp_.parent.binary", "process.parent.executable"),
        ("_tmp_.parent.cwd", "process.parent.working_directory"),
        ("_tmp_.parent.pid", "process.parent.pid"),
        ("_tmp_.parent.exec_id", "process.parent.entity_id"),
        ("_tmp_.parent.tid", "process.parent.thread.id"),
        ("_tmp_.parent.uid", "process.parent.user.id"),
        ("_tmp_.parent.start_time", "process.parent.start"),
    ];
    let plan = PainlessPlan::new(TETRAGON_LIFT);

    c.bench_function("painless_exec/aliased_renames", |b| {
        b.iter_batched_ref(
            || {
                let mut event = Event::new(tetragon_event());
                painless_exec_plan(&mut event, &plan);
                event
            },
            |event| {
                for (from, to) in RENAMES {
                    let _ = event.rename(black_box(from), black_box(to));
                }
                event.remove("_tmp_")
            },
            BatchSize::SmallInput,
        );
    });
}

/// The shallow prune over a real alert's width.
fn bench_empty_arm_prune(c: &mut Criterion) {
    let plan = PainlessPlan::new(ZEROFOX_PRUNE);
    assert!(
        plan.binding()
            .iter()
            .any(|b| b.starts_with("SentinelRemovalLiteral")),
        "the prune bench no longer measures SentinelRemovalLiteral: {:?}",
        plan.binding(),
    );

    c.bench_function("painless_exec/empty_arm_prune_planned", |b| {
        b.iter_batched_ref(
            || Event::new(zerofox_event()),
            |event| painless_exec_plan(event, black_box(&plan)),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    bench_params_matcher,
    bench_text_matcher,
    bench_unhandled,
    bench_cached,
    bench_planned,
    bench_guarded_copies,
    bench_pair_table,
    bench_hoist_member,
    bench_split_fan_out,
    bench_gather_members,
    bench_armed_table,
    bench_issue_lifecycle,
    bench_wrap_entries,
    bench_key_named_by_field,
    bench_member_from_variant_key,
    bench_aliased_renames,
    bench_empty_arm_prune
);
criterion_main!(benches);
