// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Every `compiled_painless!` call site in the generated tree, read back into
//! the plan it builds.

use std::path::{Path, PathBuf};

use dfe_runtime::painless_plan::PainlessPlan;

/// The macro every generated call site wraps its plan in.
const SITE: &str = "compiled_painless!(";

/// Below this the scanner has stopped seeing the tree rather than the tree
/// having shrunk.
pub const MIN_SITES: usize = 1_600;

/// One generated call site.
#[allow(
    dead_code,
    reason = "each test binary reads its own subset of the fields"
)]
pub struct Site {
    pub file: String,
    pub line: usize,
    pub hash: String,
    pub plan: PainlessPlan,
}

/// The `src/` of every `dfe-transforms` crate in the workspace.
pub fn generated_roots() -> Vec<PathBuf> {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut roots: Vec<PathBuf> = std::fs::read_dir(&crates)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.starts_with("dfe-transforms"))
        })
        .map(|entry| entry.path().join("src"))
        .collect();
    roots.sort();
    roots
}

/// Every `.rs` file under `dir`, depth first.
fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The index of the first byte at or after `at` that is not whitespace.
fn skip_space(src: &str, at: usize) -> usize {
    at + src[at..].len() - src[at..].trim_start().len()
}

/// The raw-string literal starting at `at`, and the index just past it.
///
/// The closing run of `#` has to match the opening one, or a plan holding `"#`
/// would end early.
fn raw_literal(src: &str, at: usize) -> Option<(&str, usize)> {
    let rest = src.get(at..)?.strip_prefix('r')?;
    let hashes = rest.len() - rest.trim_start_matches('#').len();
    let body_at = at + 1 + hashes + 1;
    if src.as_bytes().get(body_at - 1) != Some(&b'"') {
        return None;
    }
    let close = format!("\"{}", "#".repeat(hashes));
    let end = src[body_at..].find(&close)? + body_at;
    Some((&src[body_at..end], end + close.len()))
}

/// The hash, plan and optional text of the site whose arguments start at `at`,
/// and the index just past the last of them.
fn arguments(src: &str, at: usize) -> Option<(&str, &str, Option<&str>, usize)> {
    let at = skip_space(src, at);
    let hash = src.get(at..)?.strip_prefix('"')?.get(..64)?;
    let comma = skip_space(src, at + 66);
    if src.as_bytes().get(comma) != Some(&b',') {
        return None;
    }
    let (plan, end) = raw_literal(src, skip_space(src, comma + 1))?;
    let after = skip_space(src, end);
    if src.as_bytes().get(after) == Some(&b',')
        && let Some((text, end)) = raw_literal(src, skip_space(src, after + 1))
    {
        return Some((hash, plan, Some(text), end));
    }
    Some((hash, plan, None, end))
}

/// Every call site in the generated tree, in file order.
///
/// # Panics
///
/// Panics on a site whose arguments do not read back, which is a generator
/// defect, and on a scan that found fewer than [`MIN_SITES`].
pub fn sites() -> Vec<Site> {
    let mut found = Vec::new();
    for root in generated_roots() {
        let mut files = Vec::new();
        rust_files(&root, &mut files);
        files.sort();
        for file in files {
            let Ok(src) = std::fs::read_to_string(&file) else {
                continue;
            };
            let name = file.display().to_string();
            let mut from = 0;
            while let Some(hit) = src[from..].find(SITE) {
                let after = from + hit + SITE.len();
                let Some((hash, plan, text, end)) = arguments(&src, after) else {
                    panic!("{name}: compiled_painless! at byte {after} does not read back");
                };
                found.push(Site {
                    file: name.clone(),
                    line: src[..after].matches('\n').count() + 1,
                    hash: hash.to_string(),
                    plan: PainlessPlan::compiled(hash, plan, text),
                });
                from = end;
            }
        }
    }
    assert!(
        found.len() >= MIN_SITES,
        "found only {} call sites -- the scanner is blind, not the tree empty",
        found.len()
    );
    found
}
