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
//! Run with: `cargo bench -p dfe-runtime`

use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use dfe_runtime::codegen_api::{painless_exec, painless_exec_params};
use dfe_runtime::event::Event;
use dfe_runtime::painless_common::normalise;
use dfe_runtime::painless_plan::{PainlessPlan, painless_exec_plan, painless_exec_plan_params};
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

criterion_group!(
    benches,
    bench_params_matcher,
    bench_text_matcher,
    bench_unhandled,
    bench_cached,
    bench_planned,
    bench_guarded_copies
);
criterion_main!(benches);
