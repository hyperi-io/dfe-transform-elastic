// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Per-event transform throughput, measured on committed fixtures.
//!
//! The number that matters is events per second through one transform, because
//! that is what a 20,000-event batch multiplies. Anything a transform does per
//! event -- converting a grok pattern, compiling a regex, allocating a map --
//! is paid 20,000 times per batch and shows up here.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use dfe_runtime::{Event, Transform};

/// Beats-shaped events from a committed fixture: the raw vendor payload as a
/// STRING in `message`, which is what the transforms are written for.
fn fixture_events(relative: &str) -> Vec<Event> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(relative);
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture {} is committed: {e}", path.display()));

    raw.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let beat = serde_json::json!({ "message": line });
            Event::new(beat)
        })
        .collect()
}

/// One pass of `transform` over every event.
///
/// Takes the events by value because a transform MUTATES them, so each
/// iteration needs a fresh copy. The copying is done in criterion's setup
/// closure, not here -- cloning a `serde_json::Value` is not cheap, and timing
/// it alongside the transform would flatter or bury whatever we are measuring.
fn run_pass(transform: &dyn Transform, mut events: Vec<Event>) -> usize {
    let mut emitted = 0;
    for event in &mut events {
        if transform.transform(event).is_ok() {
            emitted += 1;
        }
    }
    emitted
}

/// Benchmark one transform over one fixture.
///
/// The transform structs are named `Default`, which shadows the trait, so each
/// caller spells out the path rather than importing it.
fn bench_source(c: &mut Criterion, name: &str, fixture: &str, transform: &dyn Transform) {
    let events = fixture_events(fixture);
    assert!(!events.is_empty(), "{name}: fixture produced no events");

    let mut group = c.benchmark_group(name);
    group.throughput(criterion::Throughput::Elements(events.len() as u64));
    group.bench_function("default", |b| {
        b.iter_batched(
            || events.clone(),
            |batch| black_box(run_pass(transform, batch)),
            criterion::BatchSize::SmallInput,
        );
    });
    group.finish();
}

/// Mostly JSON field moves -- three grok sites. The floor for what a transform
/// costs when grok is not the work.
fn okta(c: &mut Criterion) {
    bench_source(
        c,
        "okta",
        "okta/system/test-okta-system-events.log",
        &dfe_transforms::filebeat::okta::default::Default,
    );
}

/// The grok-heaviest transform in the tree: 30 pattern sites in `default.rs`
/// alone, over syslog lines that have to be parsed rather than read.
fn cisco_meraki(c: &mut Criterion) {
    bench_source(
        c,
        "cisco_meraki",
        "cisco/meraki/logs/test-events.log",
        &dfe_transforms::filebeat::cisco_meraki::default::Default,
    );
}

/// Syslog with a CSV-ish key-value body, and nine grok sites.
fn fortinet(c: &mut Criterion) {
    bench_source(
        c,
        "fortinet",
        "fortinet/fortigate/test-fortinet.log",
        &dfe_transforms::filebeat::fortinet::default::Default,
    );
}

/// The change itself, isolated from everything else a transform does.
///
/// `per_event` is what every grok site used to do on every event: expand the
/// pattern to a regex string, then build the DFA. `cached` is what it does
/// now. Same pattern, same result, and the ratio between them is the reason
/// the transforms above got faster.
fn grok_compilation(c: &mut Criterion) {
    // A representative pattern: two typed captures and a literal separator.
    const PATTERN: &str = "^%{IPV4:_temp.src_ip}:%{PORT:sport}$";

    let mut group = c.benchmark_group("grok_compilation");

    group.bench_function("per_event", |b| {
        b.iter(|| {
            let (expanded, field_map) =
                dfe_runtime::codegen_api::grok_to_regex_with_map(black_box(PATTERN));
            #[allow(clippy::expect_used)]
            let re = regex::Regex::new(&expanded).expect("the pattern compiles");
            black_box((re, field_map))
        });
    });

    group.bench_function("cached", |b| {
        b.iter(|| black_box(dfe_runtime::grok_cache::grok(black_box(PATTERN))));
    });

    group.finish();
}

/// The dotted-path accessors, which a transform calls dozens of times per
/// event. Whatever these cost is multiplied by that count and then by the
/// batch size, so they are worth knowing to the nanosecond.
fn event_paths(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_paths");

    group.bench_function("set_nested", |b| {
        let mut event = Event::new(serde_json::json!({}));
        b.iter(|| {
            let _ = event.set(black_box("source.geo.country_iso_code"), black_box("AU"));
        });
    });

    group.bench_function("set_flat", |b| {
        let mut event = Event::new(serde_json::json!({}));
        b.iter(|| {
            let _ = event.set(black_box("message"), black_box("hello"));
        });
    });

    let populated = Event::new(serde_json::json!({
        "source": { "geo": { "country_iso_code": "AU" } },
        "message": "hello",
    }));

    group.bench_function("get_nested", |b| {
        b.iter(|| black_box(populated.get_str(black_box("source.geo.country_iso_code"))));
    });

    group.bench_function("get_missing", |b| {
        b.iter(|| black_box(populated.get_str(black_box("destination.geo.city_name"))));
    });

    group.finish();
}

/// Does `dfe-parse` actually beat a COMPILED regex?
///
/// The design rests on "grok/regex is the #1 hot-path bottleneck, replace it
/// with native parsers". That was written when every regex was rebuilt per
/// event. Now that they are compiled once, the premise deserves a measurement
/// rather than an assumption -- so this runs the same two jobs both ways.
fn native_vs_regex(c: &mut Criterion) {
    const ADDRESS: &str = "192.168.1.100";
    const PAIR: &str = "10.0.0.7:443";

    let mut group = c.benchmark_group("native_vs_regex");

    group.bench_function("ipv4/regex", |b| {
        let compiled = dfe_runtime::grok_cache::grok("^%{IPV4:source.ip}$");
        b.iter(|| black_box(compiled.regex.captures(black_box(ADDRESS))));
    });

    group.bench_function("ipv4/native", |b| {
        b.iter(|| black_box(dfe_parse::ip::parse_ipv4(black_box(ADDRESS))));
    });

    group.bench_function("ip_port/regex", |b| {
        let compiled = dfe_runtime::grok_cache::grok("^%{IPV4:_temp.src_ip}:%{PORT:sport}$");
        b.iter(|| black_box(compiled.regex.captures(black_box(PAIR))));
    });

    group.bench_function("ip_port/native", |b| {
        b.iter(|| {
            // The composite the grok expresses: address, literal colon, port.
            let parsed = dfe_parse::ip::parse_ipv4(black_box(PAIR)).and_then(|(rest, ip)| {
                let rest = rest.strip_prefix(':').unwrap_or(rest);
                dfe_parse::numeric::parse_port(rest).map(|(tail, port)| (tail, ip, port))
            });
            black_box(parsed)
        });
    });

    group.finish();
}

/// Looking a pattern UP, with every worker doing it at once.
///
/// The service runs one transform thread per partition, and each of them hits
/// the same cache on every grok site of every event. A single-threaded figure
/// misses the actual cost of `RwLock`: readers share one reader-count atomic,
/// so the line bounces between cores under exactly the load the service runs.
///
/// `map` is the shared cache. `site_local` is what a per-call-site `OnceLock`
/// costs -- one relaxed atomic load, no shared line, no hash of the pattern.
fn grok_lookup_contended(c: &mut Criterion) {
    use std::sync::OnceLock;
    use std::sync::atomic::{AtomicBool, Ordering};

    const PATTERN: &str = "^%{IPV4:_temp.src_ip}:%{PORT:sport}$";
    const LOOKUPS: usize = 20_000;
    static SITE: OnceLock<&'static dfe_runtime::grok_cache::CompiledGrok> = OnceLock::new();

    // Warm both so neither pays first-touch inside the measurement.
    let _ = dfe_runtime::grok_cache::grok(PATTERN);
    let _ = SITE.get_or_init(|| dfe_runtime::grok_cache::grok(PATTERN));

    let mut group = c.benchmark_group("grok_lookup_contended");
    // One batch of lookups per iteration, per thread.
    group.throughput(criterion::Throughput::Elements(LOOKUPS as u64));

    for threads in [1_usize, 4, 8] {
        group.bench_function(format!("map/{threads}t"), |b| {
            b.iter_custom(|iters| {
                spawn_and_time(threads, iters, || {
                    black_box(dfe_runtime::grok_cache::grok(black_box(PATTERN)));
                })
            });
        });

        group.bench_function(format!("site_local/{threads}t"), |b| {
            b.iter_custom(|iters| {
                spawn_and_time(threads, iters, || {
                    black_box(SITE.get());
                })
            });
        });
    }

    group.finish();

    // Keeps the helpers below out of the reader's way above.
    fn spawn_and_time(
        threads: usize,
        iters: u64,
        work: impl Fn() + Send + Sync + Copy,
    ) -> std::time::Duration {
        let go = AtomicBool::new(false);
        std::thread::scope(|scope| {
            let handles: Vec<_> = (0..threads)
                .map(|_| {
                    let go = &go;
                    scope.spawn(move || {
                        while !go.load(Ordering::Acquire) {
                            std::hint::spin_loop();
                        }
                        for _ in 0..iters {
                            for _ in 0..LOOKUPS {
                                work();
                            }
                        }
                    })
                })
                .collect();

            // Start the clock only once every thread is spinning on the gate,
            // so thread spawn-up is not counted as lookup time.
            let start = std::time::Instant::now();
            go.store(true, Ordering::Release);
            for handle in handles {
                let _ = handle.join();
            }
            start.elapsed()
        })
    }
}

criterion_group!(
    benches,
    grok_compilation,
    grok_lookup_contended,
    native_vs_regex,
    event_paths,
    okta,
    cisco_meraki,
    fortinet
);
criterion_main!(benches);
