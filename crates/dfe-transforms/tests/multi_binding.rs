// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scripts that bind to MORE THAN ONE pattern, across the whole generated tree.
//!
//! Dispatch runs the params pattern, then the text patterns in order, and stops
//! at the first that returns `true` (`dfe-painless/src/plan.rs`,
//! `painless_exec_plan_params`). A runner that cannot decline therefore shadows
//! every pattern after it.
//!
//! Scanning for runners that never return `false` is the WRONG search -- about
//! 170 do, and it is harmless where the runner is its script's only binding.
//! The hazard needs BOTH halves, so this counts the other one.
//!
//! A `Bespoke` claim is stripped before the count, because the floor watches
//! ladder contention and a hand transcription consulted ahead of the ladder
//! competes with nothing -- what remains is the ladder's own search space.
//!
//! The plans are read back out of the generated modules, one per distinct
//! script hash.

mod sites;

use std::collections::BTreeMap;

/// Scripts binding to more than one pattern.
///
/// Not a defect count: where the first pattern declines, dispatch falls
/// through and the second runs. The number is the search space for
/// shadowing, and it moves when the ladder gains or loses an arm or a
/// regeneration changes which scripts reach the ladder.
///
/// A rise means a script now resolves to two matchers: check whether the
/// first returns `true` without doing the whole job before moving the floor.
const MULTI_BINDING: usize = 56;

#[test]
fn the_shadowing_search_space_has_not_grown() {
    let plans: BTreeMap<String, Vec<String>> = sites::sites()
        .into_iter()
        .map(|site| (site.hash, site.plan.binding()))
        .collect();
    let mut multi: Vec<(usize, String)> = plans
        .into_iter()
        .filter_map(|(hash, binding)| {
            let names: Vec<String> = binding
                .into_iter()
                .filter(|name| !name.starts_with("Bespoke("))
                .collect();
            (names.len() > 1).then(|| {
                let heads: Vec<&str> = names
                    .iter()
                    .map(|name| name.split(['(', ' ', '{']).next().unwrap_or_default())
                    .collect();
                (names.len(), format!("{} <- {hash}", heads.join(" + ")))
            })
        })
        .collect();
    multi.sort();

    let report: Vec<&String> = multi.iter().map(|(_, line)| line).collect();
    assert_eq!(
        multi.len(),
        MULTI_BINDING,
        "multi-binding scripts moved from {MULTI_BINDING}:\n{report:#?}"
    );
}
