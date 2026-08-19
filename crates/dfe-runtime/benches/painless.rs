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
use serde_json::json;

/// Verbatim from `pipelines/crowdstrike/default.yml`, escapes and all -- the
/// generated modules pass the script exactly as it is emitted.
const SENTINEL: &str = "ctx.crowdstrike.event.entrySet().removeIf(entry -> \
                        params.values.contains(entry.getValue()));\\n";

/// Last rung of the text-only ladder.
const OKTA_TARGET: &str = "for (item in ctx.okta.target) {\\n  if (item.alternateId != null) \
                           {\\n    item.alternate_id = item.alternateId;\\n  }\\n}";

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

criterion_group!(
    benches,
    bench_params_matcher,
    bench_text_matcher,
    bench_unhandled,
    bench_cached
);
criterion_main!(benches);
