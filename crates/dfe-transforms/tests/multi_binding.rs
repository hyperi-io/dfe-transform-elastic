// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scripts that bind to MORE THAN ONE pattern, across the whole generated tree.
//!
//! Dispatch runs the params pattern, then the text patterns in order, and stops
//! at the first that returns `true` (`painless_plan.rs:126-146`). A runner that
//! cannot decline therefore shadows every pattern after it. gdacs lost
//! `BranchCopies` that way for as long as the source existed: its
//! `geo_point_from_coordinates` wrote the centroid, returned `true`
//! unconditionally, and the two field copies behind it never ran.
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

/// Scripts binding to more than one pattern, as of 2026-09-07.
///
/// An arm whose every claimant binds to it ALONE does not move this, so a
/// ladder edit is not on its own a reason to expect the number to change.
///
/// **Not a defect count.** Where the first pattern DECLINES, dispatch falls
/// through and the second runs correctly -- 20 of these are
/// `SentinelRemoval + SentinelRemovalLiteral`, and the params half declines on
/// a script carrying no `params` block. The number is the SEARCH SPACE for
/// shadowing, cut from 2,456 call sites to this.
///
/// A FLOOR, not a ratchet: it moves when the ladder gains or loses an arm, or
/// when a regeneration changes which scripts still reach the ladder at all --
/// not on ordinary parity work. A RISE means a script that used to resolve to
/// one matcher now resolves to two: check whether the first returns `true`
/// without doing the whole job, which is how gdacs lost `BranchCopies`.
///
/// 63 to 59 with the flat-subscript regeneration, which resolved twelve more
/// scripts to their runners at generation time and so took them out of this
/// search space entirely. Distinct ladder scripts 1,200 to 1,188 on the same
/// run.
///
/// 59 to 60 with the `ArmedTable` arm, which claude_code and claude_cowork
/// share one script for. It shadows `GuardedCopy` deliberately: that pattern
/// reads the literal arms correctly, then cannot resolve the params row and
/// takes the else branch for every key the table carries.
///
/// 60 to 61 with the `IssueLifecycle` arm, kolide's issues stream. It shadows
/// `GuardedCopy` for the same reason: every guard there is over a boolean local
/// and resolves to `Never`, so `event.type` was never written at all.
///
/// 61 to 59 when `FirstElement` stopped reading a path out of script text. Its
/// take slices between the last `ctx.` and a `[0];` with neither anchored to a
/// statement, so ece's action script and sentinel_one's asset select both fell
/// out of the search space they should never have been in.
///
/// 59 to 60 with the `LabelledScore` arm, `beyondtrust_isi`'s severity. It
/// shadows `GuardedCopy` deliberately and shadows nothing: every guard in that
/// script is over a local the evaluator resolves to `Never`, so the `Program`
/// ran none of its three writes and the source scored 0 of 4 events.
///
/// 60 to 58 when `GuardedCopy`'s single-copy fallback was gated on a BARE-PATH
/// value. Both losses are a second binding that could only ever have written a
/// wrong value: azure's `ctx.event.duration = ctx.event.duration * params.<n>`
/// read the last `ctx.` path out of the multiply and bound as a copy of the
/// field onto itself, and ti_anomali's itype lookup bound as a copy of
/// `json.itype` over the whole `ctx.threat` map. `Scale` and `LookupNormalise`
/// shadowed both and always return true, so neither ever ran -- this is the
/// search space losing two entries that were never reachable, not a matcher
/// going away.
///
/// 58 to 57 when the `DropEmpty` arm learnt the iterative stack spelling of the
/// recursive prune. hackerone's was claimed by `CollectingLadder` AND
/// `CollectMapValues`, neither of which prunes anything, and it now binds the
/// one matcher that does.
///
/// 57 to 58 with the `LabelledKeyRename` arm, varonis's CEF label rename. It
/// shadows `FirstContainedMember` and shadows nothing: that is a params matcher
/// and the call site passes no `params` block, so dispatch skips it and this
/// arm is the only one that can run. The source scored 0 of 2 events with 26
/// fields Elasticsearch does not emit, and 2 of 2 with none once it bound.
///
/// 58 to 56 when the scan moved to the compiled plans, which it reads at every
/// site, once per script hash. okta's two risk-behaviour scripts left the
/// search space when `RemapKeysThroughTable` stopped claiming them with script
/// text as its target.
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
