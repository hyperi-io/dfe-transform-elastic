// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//! Every `.rs` file under `src` is declared by a `mod` line reachable from
//! `lib.rs`.
//!
//! The generator writes a module per sub-pipeline beside a `default.rs` that
//! already inlines it, and a file no `mod` line declares is compiled by
//! nothing. Every other gate in this directory walks the disk, so such a file
//! still feeds its literals into the floors and the ratchets, and a
//! regeneration dry run diffs it as a changed source. 653 of them sat here
//! once, carrying 556 painless literals and 62 of the dead-branch sites the
//! ratchet counted.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use regex::Regex;

/// The walk has to reach the tree: about 2,210 declared files today.
///
/// A property of this test, not of the code under test: a walk that follows
/// nothing reports no orphans and passes.
const MIN_DECLARED: usize = 1_900;

/// A `mod name;` line, with the `#[path = "..."]` attribute it may carry.
const MOD_LINE: &str = r#"(?m)^\s*(?:#\[path\s*=\s*"(?P<path>[^"]+)"\]\s*)?(?:pub(?:\([^)]*\))?\s+)?mod\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*;"#;

fn source_root() -> PathBuf {
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

/// Where a file's `mod name;` lines resolve: the directory itself for a
/// `mod.rs` or `lib.rs`, `<stem>/` for any other file.
fn child_dir(file: &Path) -> Option<PathBuf> {
    let parent = file.parent()?;
    let name = file.file_name()?;
    if name == "mod.rs" || name == "lib.rs" {
        Some(parent.to_path_buf())
    } else {
        Some(parent.join(file.file_stem()?))
    }
}

/// The files one module file declares, resolved the way rustc resolves them.
fn declared(file: &Path, mod_line: &Regex) -> Vec<PathBuf> {
    let Ok(text) = std::fs::read_to_string(file) else {
        return Vec::new();
    };
    let (Some(parent), Some(base)) = (file.parent(), child_dir(file)) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for caps in mod_line.captures_iter(&text) {
        let Some(name) = caps.name("name") else {
            continue;
        };
        let candidates = match caps.name("path") {
            Some(path) => vec![parent.join(path.as_str())],
            None => vec![
                base.join(format!("{}.rs", name.as_str())),
                base.join(name.as_str()).join("mod.rs"),
            ],
        };
        if let Some(found) = candidates.into_iter().find(|c| c.is_file()) {
            out.push(found);
        }
    }
    out
}

#[test]
fn every_module_file_is_declared() {
    let mod_line = Regex::new(MOD_LINE).expect("the mod-line pattern compiles");
    let root = source_root()
        .canonicalize()
        .expect("the generated tree exists");

    let mut reached = BTreeSet::new();
    let mut stack = vec![root.join("lib.rs")];
    while let Some(file) = stack.pop() {
        let Ok(file) = file.canonicalize() else {
            continue;
        };
        if !reached.insert(file.clone()) {
            continue;
        }
        stack.extend(declared(&file, &mod_line));
    }
    println!("{} module file(s) reached from lib.rs", reached.len());
    assert!(
        reached.len() >= MIN_DECLARED,
        "the walk reached {} files, under the {MIN_DECLARED} floor -- it is not \
         following the mod declarations, so it would report no orphans either",
        reached.len()
    );

    let mut on_disk = Vec::new();
    rust_files(&root, &mut on_disk);
    let orphans: Vec<String> = on_disk
        .iter()
        .filter_map(|path| path.canonicalize().ok())
        .filter(|path| !reached.contains(path))
        .map(|path| {
            path.strip_prefix(&root)
                .map_or_else(|_| path.clone(), Path::to_path_buf)
                .display()
                .to_string()
        })
        .collect();
    assert!(
        orphans.is_empty(),
        "{} file(s) under src are declared by no mod line and compiled by nothing:\n{}",
        orphans.len(),
        orphans.join("\n")
    );
}
