// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Where the allocations go, per event, per transform.
//!
//! Timing says a transform is slow; it does not say WHY. An allocation count
//! does, because on this hot path almost every avoidable cost is a `String` or
//! a `Map` that did not need to exist -- a path joined with `format!`, a
//! `to_string` on a value that was only going to be compared, a clone of a
//! subtree the transform then overwrites. At the shipped batch size of 20,000
//! events, one stray allocation per event is 20,000 per batch.
//!
//! This is a REPORT, not a criterion bench: the numbers are exact rather than
//! sampled, so there is nothing to average. Run it with
//! `cargo bench -p dfe-transforms --bench allocations`.
//!
//! The counting allocator lives here and not in `transforms.rs` on purpose --
//! it adds two relaxed atomics to every allocation, which is small but not
//! nothing, and the throughput figures next door are meant to be clean.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use dfe_runtime::{Event, Transform};

/// Counts every allocation and the bytes it asked for.
///
/// Deallocation is not counted: what is being asked is how much work the
/// allocator was given, and a freed allocation cost exactly as much as one
/// that was kept.
struct Counting;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

/// Sampled call-site attribution, off unless [`profile_sites`] turned it on.
///
/// `perf` is unavailable on the build hosts (`perf_event_paranoid` is 4 and
/// lowering it needs root), so the profiler is built here instead. Capturing a
/// backtrace on EVERY allocation would be unusable -- symbol resolution costs
/// far more than the allocation it describes -- so one in `SAMPLE_EVERY` is
/// captured and the counts are scaled back up. The ranking is what matters,
/// and a 1-in-64 sample settles that long before it settles the absolute
/// numbers.
static PROFILING: AtomicBool = AtomicBool::new(false);
static SAMPLE_COUNTER: AtomicUsize = AtomicUsize::new(0);
const SAMPLE_EVERY: usize = 64;
static SITES: Mutex<Option<HashMap<String, usize>>> = Mutex::new(None);

thread_local! {
    /// Capturing a backtrace allocates, which would re-enter the hook and
    /// recurse until the stack is gone.
    static IN_HOOK: Cell<bool> = const { Cell::new(false) };
}

/// The innermost frame belonging to this project, as a symbol name.
///
/// Everything below it is `alloc`/`RawVec`/`serde_json` plumbing that says
/// nothing about which of OUR call sites asked for the memory.
fn attribute() -> Option<String> {
    let text = std::backtrace::Backtrace::force_capture().to_string();
    text.lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(_, symbol)| symbol.trim())
        // A trait method prints as `<Type as Trait>::method`, so the crate
        // name is not at the start -- matching on `starts_with` found nothing
        // at all once the optimiser had inlined the free functions into it.
        .find(|symbol| {
            (symbol.contains("dfe_runtime")
                || symbol.contains("dfe_transforms")
                || symbol.contains("dfe_parse"))
                && !symbol.contains("allocations")
        })
        // The generic instantiation tail (`::h9f3a...`) is noise.
        .map(|symbol| symbol.rsplit_once("::h").map_or(symbol, |(head, _)| head))
        .map(str::to_string)
}

/// Record the site that asked for this allocation, if this one is sampled.
fn sample_site() {
    if !PROFILING.load(Ordering::Relaxed) {
        return;
    }
    if !SAMPLE_COUNTER
        .fetch_add(1, Ordering::Relaxed)
        .is_multiple_of(SAMPLE_EVERY)
    {
        return;
    }
    IN_HOOK.with(|guard| {
        if guard.get() {
            return;
        }
        guard.set(true);
        if let Some(site) = attribute()
            && let Ok(mut sites) = SITES.lock()
            && let Some(sites) = sites.as_mut()
        {
            *sites.entry(site).or_insert(0) += 1;
        }
        guard.set(false);
    });
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        sample_site();
        // SAFETY: the layout is the caller's, forwarded unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: the pointer and layout are the caller's, forwarded unchanged.
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(new_size.saturating_sub(layout.size()), Ordering::Relaxed);
        sample_site();
        // SAFETY: the pointer and layout are the caller's, forwarded unchanged.
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

/// Run `work` with call-site sampling on, and return the ranked sites.
///
/// The counts are sample counts scaled by [`SAMPLE_EVERY`], so they are an
/// estimate of allocations per site rather than an exact figure. Ranking is
/// the point; if two sites are within a sample of each other, they are the
/// same size as far as this says.
fn profile_sites<T>(work: impl FnOnce() -> T) -> Vec<(String, usize)> {
    if let Ok(mut sites) = SITES.lock() {
        *sites = Some(HashMap::new());
    }
    SAMPLE_COUNTER.store(0, Ordering::Relaxed);
    PROFILING.store(true, Ordering::Relaxed);
    let _ = work();
    PROFILING.store(false, Ordering::Relaxed);

    let mut ranked: Vec<(String, usize)> = SITES
        .lock()
        .ok()
        .and_then(|mut sites| sites.take())
        .unwrap_or_default()
        .into_iter()
        .map(|(site, count)| (site, count * SAMPLE_EVERY))
        .collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    ranked
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Allocations and bytes charged while `work` ran.
fn measure<T>(work: impl FnOnce() -> T) -> (T, usize, usize) {
    let allocations = ALLOCATIONS.load(Ordering::Relaxed);
    let bytes = BYTES.load(Ordering::Relaxed);
    let out = work();
    (
        out,
        ALLOCATIONS.load(Ordering::Relaxed) - allocations,
        BYTES.load(Ordering::Relaxed) - bytes,
    )
}

/// Beats-shaped events from a committed fixture, the same shape
/// `transforms.rs` builds so the two reports describe the same work.
///
/// A line that already carries `message` is used as it stands; anything else
/// is the raw vendor payload and gets wrapped. Wrapping an already-wrapped
/// line hands the transform a `message` holding the TEXT of a JSON object, so
/// nothing matches and the report describes a no-op.
fn fixture_events(relative: &str) -> Vec<Event> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(relative);
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture {} is committed: {e}", path.display()));

    raw.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| Event::new(beat_shaped(line)))
        .collect()
}

/// One fixture line as the document a transform is handed.
fn beat_shaped(line: &str) -> serde_json::Value {
    match serde_json::from_str::<serde_json::Value>(line) {
        Ok(value) if value.get("message").is_some() => value,
        _ => serde_json::json!({ "message": line }),
    }
}

/// A transform that changed nothing is not slow, it is not running.
///
/// This guard exists because two of these benches spent their whole life
/// measuring a no-op: the fixture was double-wrapped, no grok matched, and the
/// figures looked like a fast transform rather than an absent one.
fn assert_did_work(name: &str, before: &Event, after: &Event) {
    assert_ne!(
        serde_json::to_string(before.as_value()).unwrap_or_default(),
        serde_json::to_string(after.as_value()).unwrap_or_default(),
        "{name}: the transform left the event untouched, so this measures nothing"
    );
}

/// What reading the fixture line as JSON costs on its own.
///
/// For a source whose payload IS JSON that is the vendor document; for a
/// syslog source it is the Beats envelope around the line. Either way the tree
/// has to be built before a transform can do anything, so separating it says
/// how much of a source's total is the document and how much is the
/// transform's own work -- without it, a big payload and a wasteful transform
/// look identical.
///
/// Returns `None` where a line does not parse at all, rather than reporting
/// the cost of a failed parse as a floor and inventing a ratio.
fn payload_floor(fixture: &str) -> Option<(usize, usize, usize)> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(fixture);
    let raw = std::fs::read_to_string(&path).expect("the fixture is committed");
    let lines: Vec<&str> = raw.lines().filter(|l| !l.trim().is_empty()).collect();
    let count = lines.len();

    let (parsed, allocations, bytes) = measure(|| {
        let mut parsed = Vec::with_capacity(count);
        for line in &lines {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(line) {
                parsed.push(value);
            }
        }
        parsed
    });
    (parsed.len() == count).then_some((count, allocations, bytes))
}

/// One source's report line.
fn report(name: &str, fixture: &str, transform: &dyn Transform) {
    let events = fixture_events(fixture);
    let count = events.len();
    assert!(count > 0, "{name}: fixture produced no events");

    // A warm pass first: every `OnceLock` in the transform -- grok patterns,
    // params tables, Painless plans -- allocates on first touch, and charging
    // that to the first event would put the whole compile cost on one line.
    let mut warm = events.clone();
    for event in &mut warm {
        let _ = transform.transform(event);
    }
    assert_did_work(name, &events[0], &warm[0]);

    let mut batch = events;
    let (emitted, allocations, bytes) = measure(|| {
        let mut emitted = 0usize;
        for event in &mut batch {
            if transform.transform(event).is_ok() {
                emitted += 1;
            }
        }
        emitted
    });

    #[allow(clippy::cast_precision_loss)]
    let per_event = allocations as f64 / count as f64;
    #[allow(clippy::cast_precision_loss)]
    let bytes_per_event = bytes as f64 / count as f64;

    let floor = match payload_floor(fixture) {
        #[allow(clippy::cast_precision_loss)]
        Some((_, floor_allocations, floor_bytes)) => format!(
            "{:>8.1} {:>6.1}x {:>9.0}",
            floor_allocations as f64 / count as f64,
            per_event / (floor_allocations as f64 / count as f64).max(1.0),
            floor_bytes as f64 / count as f64,
        ),
        None => format!("{:>8} {:>7} {:>9}", "-", "-", "-"),
    };

    println!(
        "{name:<16} {count:>4}  {per_event:>8.1} {floor}  {bytes_per_event:>9.0}  \
         ({emitted} emitted)"
    );
}

/// Where one source's allocations come from, ranked.
fn sites(name: &str, fixture: &str, transform: &dyn Transform, top: usize) {
    let events = fixture_events(fixture);
    let count = events.len();

    // Warm, so the one-off pattern and plan builds are not in the sample.
    let mut warm = events.clone();
    for event in &mut warm {
        let _ = transform.transform(event);
    }

    // Enough repeats that a 1-in-64 sample sees the small sites too.
    const PASSES: usize = 40;
    let ranked = profile_sites(|| {
        for _ in 0..PASSES {
            let mut batch = events.clone();
            for event in &mut batch {
                let _ = transform.transform(event);
            }
        }
    });

    println!("\n{name} -- allocations per event by innermost dfe frame:");
    if ranked.is_empty() {
        println!("  (no frames resolved; the bench profile needs debuginfo)");
        return;
    }
    for (site, allocations) in ranked.iter().take(top) {
        #[allow(clippy::cast_precision_loss)]
        let per_event = *allocations as f64 / (count * PASSES) as f64;
        println!("  {per_event:>7.1}  {site}");
    }
}

fn main() {
    println!(
        "Allocations charged per event, after a warm pass. `payload` is what \
         parsing\nthe fixture line as JSON costs on its own, so the ratio is \
         how much the transform adds.\n"
    );
    println!(
        "{:<16} {:>4}  {:>8} {:>8} {:>7} {:>9}  {:>9}",
        "source", "n", "allocs", "payload", "ratio", "payload B", "bytes"
    );

    const SOURCES: [(&str, &str); 3] = [
        ("okta", "okta/system/test-okta-system-events.log"),
        ("cisco_meraki", "cisco/meraki/logs/test-events.log"),
        ("fortinet", "fortinet/fortigate/test-fortinet.log"),
    ];

    let transforms: [&dyn Transform; 3] = [
        &dfe_transforms::filebeat::okta::default::Default,
        &dfe_transforms::filebeat::cisco_meraki::default::Default,
        &dfe_transforms::filebeat::fortinet::default::Default,
    ];

    for ((name, fixture), transform) in SOURCES.iter().zip(transforms) {
        report(name, fixture, transform);
    }

    println!(
        "\nA `serde_json` document is itself allocation-heavy, so the floor is \
         not zero.\nWhat matters is the DELTA when a call site stops building a \
         string it did not need."
    );

    // Sampled attribution. Off by default: resolving symbols is slow enough
    // that it doubles the run, and the totals above are the number to watch
    // day to day.
    if std::env::var_os("DFE_ALLOC_SITES").is_none() {
        println!("\nSet DFE_ALLOC_SITES=1 for a ranked per-call-site breakdown.");
        return;
    }
    if std::env::var_os("DFE_ALLOC_RAW").is_some() {
        println!("\nOne raw backtrace, as the attribution sees it:\n");
        println!("{}", std::backtrace::Backtrace::force_capture());
    }
    for ((name, fixture), transform) in SOURCES.iter().zip(transforms) {
        sites(name, fixture, transform, 15);
    }
}
