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

    // What the runtime offers publicly from the two files a `direct_call` can
    // name. Anything else it emits is a macro or a `codegen_api` helper, which
    // reach the call site by their own path.
    let declaration = Regex::new(r"(?m)^pub (?:fn|struct|enum) ([A-Za-z_][A-Za-z0-9_]*)")
        .expect("declaration regex");
    let mut public = BTreeSet::new();
    for name in ["painless_common.rs", "painless_params.rs"] {
        let text = read(&runtime.join(name));
        for caps in declaration.captures_iter(&text) {
            public.insert(caps[1].to_owned());
        }
    }
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
