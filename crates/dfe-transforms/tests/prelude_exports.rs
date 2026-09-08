// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Every runtime symbol the generated tree names must be reachable through the
//! prelude.
//!
//! A generated module imports exactly one thing -- `use dfe_runtime::prelude::*`
//! -- so an arm whose `direct_call` emits a type or function the prelude never
//! re-exported cannot compile. The prelude already claims this invariant in a
//! comment ("Same functions the ladder dispatches to, so the two paths cannot
//! diverge") and nothing enforced it.
//!
//! It is invisible until a source carrying that arm is REGENERATED, which is
//! why it survived: regeneration had been selective, `gdacs_events` had not
//! been touched since `CoerceBoolean` and `JoinPresentFields` were added, and
//! the first whole-tree regeneration broke the build on all four names at once.
//!
//! Static, so it costs milliseconds where compiling this crate costs minutes --
//! and it fails with the missing NAMES rather than a compiler error in a
//! generated file nobody wrote.
//!
//! The first test reads what the TREE names, so a pattern whose arm has never
//! been regenerated is invisible to it. The second closes that by asking the
//! RUNTIME instead: every `pub fn <name>(event: &mut Event, pattern: &<Type>)`
//! is a runner by construction, so both its names belong in the prelude whether
//! or not a generated file mentions them yet. It found 13 such runners missing,
//! of which only two had surfaced.
//!
//! Reading the `direct_call` bodies was tried and abandoned -- telling a Rust
//! string literal from code needs a real scanner, and a regex over quote pairs
//! reports two dozen names that are neither emitted nor missing. A signature
//! match has none of that ambiguity.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use regex::Regex;

/// The walk has to actually reach both trees.
///
/// Properties of this test, not of the code under test: reading zero symbols
/// is indistinguishable from scanning zero files, and that is how a scanner
/// passes for a fortnight while measuring nothing.
const MIN_PUBLIC_SYMBOLS: usize = 60;
const MIN_FILES_SCANNED: usize = 2_500;
/// Five modules write a `direct_call` today. A floor rather than the count, so
/// adding a sixth does not fail this and deleting four does.
const MIN_EMITTING_MODULES: usize = 4;
/// 35 runners carry the signature today, across seven runtime modules.
const MIN_RUNNERS: usize = 30;

fn runtime_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../dfe-runtime/src")
}

fn generated_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn every_named_runtime_symbol_is_in_the_prelude() {
    let runtime = runtime_src();

    // Every file that writes a `direct_call`, discovered rather than listed --
    // a named list cannot see a pattern module added beside it, and the first
    // sign of one would be a regeneration that does not compile. Anything else
    // the generator emits is a macro or a `codegen_api` helper, which reach the
    // call site by their own path.
    // Anchored at column zero: an indented `pub fn` is an inherent method such
    // as `new`, reached through its type rather than named by the prelude.
    let declaration = Regex::new(r"(?m)^pub (?:fn|struct|enum) ([A-Za-z_][A-Za-z0-9_]*)")
        .expect("declaration regex");
    let mut public = BTreeSet::new();
    let mut emitting = Vec::new();
    for entry in std::fs::read_dir(&runtime)
        .expect("read the runtime source")
        .flatten()
    {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let text = read(&path);
        if !text.contains("fn direct_call") {
            continue;
        }
        for caps in declaration.captures_iter(&text) {
            public.insert(caps[1].to_owned());
        }
        emitting.push(text);
    }
    assert!(
        emitting.len() >= MIN_EMITTING_MODULES,
        "found {} runtime modules writing a `direct_call`, expected at least \
         {MIN_EMITTING_MODULES} -- the walk is not reaching them",
        emitting.len()
    );
    assert!(
        public.len() >= MIN_PUBLIC_SYMBOLS,
        "scanned {} public runtime symbols, expected at least {MIN_PUBLIC_SYMBOLS} -- \
         the walk is not reaching the runtime source",
        public.len()
    );

    let prelude = read(&runtime.join("prelude.rs"));
    let word = Regex::new(r"\b([A-Za-z_][A-Za-z0-9_]*)\b").expect("word regex");
    let exported: BTreeSet<String> = word
        .captures_iter(&prelude)
        .map(|caps| caps[1].to_owned())
        .collect();

    // A call, or a type used as a constructor, in a generated module.
    let call = Regex::new(r"\b([a-z_][a-z0-9_]*)\s*\(").expect("call regex");
    let constructed = Regex::new(r"\b([A-Z][A-Za-z0-9_]*)::").expect("type regex");

    let mut files = Vec::new();
    rust_files(&generated_src(), &mut files);
    assert!(
        files.len() >= MIN_FILES_SCANNED,
        "scanned {} generated files, expected at least {MIN_FILES_SCANNED}",
        files.len()
    );

    let mut used = BTreeSet::new();
    for path in &files {
        let text = read(path);
        for caps in call.captures_iter(&text) {
            used.insert(caps[1].to_owned());
        }
        for caps in constructed.captures_iter(&text) {
            used.insert(caps[1].to_owned());
        }
    }

    let missing: Vec<&String> = public
        .iter()
        .filter(|name| used.contains(*name) && !exported.contains(*name))
        .collect();

    assert!(
        missing.is_empty(),
        "the generated tree names {} runtime symbol(s) the prelude does not export: {missing:?}\n  \
         A generated module imports only `dfe_runtime::prelude::*`, so these cannot resolve. \
         Add them to the `painless_common` re-export in crates/dfe-runtime/src/prelude.rs.",
        missing.len()
    );
}

/// Every matcher runner and its pattern type reaches the prelude, whether or
/// not the generated tree names one today.
///
/// The test above reads what the TREE names, so a matcher whose `direct_call`
/// has never been regenerated is invisible to it -- which is how four symbols
/// broke the first whole-tree regeneration and two more broke the next. This
/// asks the RUNTIME instead. A `pub fn <name>(event: &mut Event, pattern:
/// &<Type>)` is a runner by construction, and the prelude's own comment already
/// claims it re-exports every one of them; the export list was a hand-kept
/// subset, which is the defect.
///
/// A signature match, not a scan for names inside string literals -- that
/// reader was tried and abandoned for reporting two dozen names that were
/// neither emitted nor missing.
#[test]
fn every_matcher_runner_reaches_the_prelude() {
    let runtime = runtime_src();
    let runner = Regex::new(
        r"(?m)^pub fn ([a-z_][a-z0-9_]*)\(event: &mut Event, pattern: &([A-Za-z][A-Za-z0-9_]*)\)",
    )
    .expect("runner regex");

    let mut runners: Vec<(String, String)> = Vec::new();
    for entry in std::fs::read_dir(&runtime)
        .expect("read the runtime source")
        .flatten()
    {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        for caps in runner.captures_iter(&read(&path)) {
            runners.push((caps[1].to_owned(), caps[2].to_owned()));
        }
    }
    assert!(
        runners.len() >= MIN_RUNNERS,
        "found {} matcher runners, expected at least {MIN_RUNNERS} -- the walk is \
         not reaching the runtime source",
        runners.len()
    );

    let prelude = read(&runtime.join("prelude.rs"));
    let word = Regex::new(r"\b([A-Za-z_][A-Za-z0-9_]*)\b").expect("word regex");
    let exported: BTreeSet<String> = word
        .captures_iter(&prelude)
        .map(|caps| caps[1].to_owned())
        .collect();

    let missing: Vec<String> = runners
        .iter()
        .flat_map(|(function, kind)| [function, kind])
        .filter(|name| !exported.contains(*name))
        .cloned()
        .collect();

    assert!(
        missing.is_empty(),
        "{} runner symbol(s) are not re-exported by the prelude: {missing:?}\n  \
         A generated module imports only `dfe_runtime::prelude::*`, so the first \
         regeneration of a source using one of these will not compile. Add them to \
         crates/dfe-runtime/src/prelude.rs.",
        missing.len()
    );
}
