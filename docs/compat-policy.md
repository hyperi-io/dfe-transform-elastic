# compat policy -- what counts as a difference

The ratchet that holds per-source scores, the rule sections in
`tests/compare-policy.yaml` that decide which differences are defects, and the divergences kept
on purpose. The how-to that produces the corpus is [compat.md](compat.md), and what parity
means in the first place is [parity.md](parity.md).

## The ratchet

`tests/compat-baseline.json` is the ratchet the corpus test asserts, and the
test prints the exact line for every source that improved.
`scripts/raise_baseline.py` applies them:

```bash
cargo test -p dfe-transforms --test compat_corpus -- --nocapture > /tmp/run.txt
python3 scripts/raise_baseline.py /tmp/run.txt
```

It REFUSES any line that would lower a score. A fall needs a stated reason and
is never mechanical.

## The ratchet refuses to be skipped quietly

The baseline is asserted only when the corpus on disk was captured at the
`integrations_sha` and engine version the baseline names. A score against
different pipelines is a different measurement, so ratcheting one against the
other would fail for the wrong reason.

Until 2026-09-03 that mismatch PRINTED and returned, and the run still exited
0. Regenerate the corpus at a new sha, forget to update the baseline, and
parity was unenforced with no signal, the message lost among thousands of
per-fixture lines. It now FAILS instead, naming the two shas.

`DFE_COMPAT_ALLOW_UNRATCHETED=1` is the acknowledgement, for the one
legitimate case: a regeneration in progress, where the corpus has moved and
the baseline has not caught up yet. It leaves parity unenforced for that run,
which is why it has to be typed.

A corpus that is ABSENT is a different thing and still skips silently -- the
corpus is gitignored, so a fresh clone has none.

## The second ratchet: patterns that never apply

`never_ran` in `tests/compat-baseline.json` holds the count of scripts that
bound a pattern and never once ran it. Read the number there rather than here:
a figure restated in prose is stale the next time the ratchet moves.

This is the class the work in early September kept finding: a ladder arm with a
bare `contains()` trigger claims a script its runner then declines on every
event. The static census counts it as covered and the Painless coverage floor
reads 100%, because both measure whether a pattern MATCHED. Only running the
corpus shows whether it applied.

It is not a defect count. A script can legitimately never run because no event
in the corpus carries its source field, so read it with
`scripts/pattern_reach.py`, which joins the reach data to call sites, and never
on its own.

`DFE_PAINLESS_UNHANDLED=<path>` writes the per-script catalogue. The catalogue
is now on for every run: its write lock is taken once per script EXECUTION,
about 70,500 times over the whole corpus, and the run measured 18.6s against
17.7-19.8s with it off. [Choosing which unclaimed script to work
next](compat.md#choosing-which-unclaimed-script-to-work-next) is what to do with it.

## The ratchet does not gate EXTRA fields

`fields_wrong` counts missing and mismatched fields only. A field we emit that
Elasticsearch does not is counted separately as `fields_extra`, printed in the
run and compared against nothing.

An event still needs an empty diff to count as matched, so extras are gated at
EVENT granularity -- but on a source already scoring zero matched events, which
is most of the ones carrying real debt, nothing checks them at all. The run
prints the current figure; it is not restated here, for the same reason.

Extras are the best single signal that a source is emitting a RAW shape rather
than the parsed one. axonius carried 2,313 of them on 3,705 fields, all of the
form `<base>.event.data.<name>` where Elasticsearch had `<base>.<name>` -- one
unread hoist, and reading it took the source from 11% of events to 94%.

So **record the extras figure before and after any change to the pattern
ladder**. A pattern that stops claiming a script hands it to whatever sits
below, and if
that writes a field Elastic never emits, the totals hold while only the extras
move -- which the ratchet passes green. That has happened once and the change
was reverted; it was caught by having written the previous number down and by
nothing else.

## What matters, and what does not

Byte equality is not the goal. `tests/compare-policy.yaml` is the single
definition of which differences are defects, read by this tool and by the Rust
comparison harness. Every rule carries a reason, and every run reports what it
excluded. Adding an entry HIDES a difference, which is why the reason is
mandatory and why the section a rule lands in is a decision rather than a
filing choice:

| Section | What it claims | What follows from it |
|---|---|---|
| `nondeterministic` | the value varies run to run and carries no information | removed at capture, never compared at all |
| `not_emitted` | Elastic or Beats plumbing DFE does not produce | never a defect |
| `known_different` | both sides produce it, differently | DEBT. Each entry should eventually go away and the count is expected to FALL |
| `corrected` | Elasticsearch is wrong and we are right | permanent. It never goes away, and matching the bug would have scored better |
| `unordered` | the field's value is a SET | the same members in another sequence is equality, not a difference |

`known_different` and `corrected` read alike and mean opposite things, and the
test is whether the difference is ours to close. An array NOT named in
`unordered` is compared in order, deliberately.

## Scope a rule to its source, or it blinds every other one

A rule may carry `sources: [name, ...]` and then holds for those sources alone.
Omit it only where the reason is a fact about the FIELD -- `event.ingested` is
write time everywhere. An unscoped quirk is the expensive mistake: `event.kind`
and `error.message` were once excluded corpus-wide for one source's date
failure, so a transform emitting `pipeline_error` anywhere else did not register
as a difference at all. `unordered` is never scoped, because a set is a fact
about the field.

The two readers differ. `crates/dfe-runtime/src/testutil/policy.rs` honours
`sources` and chains the four rule sections into ONE skip set, keeping no column
breakdown -- a skipped path is simply not compared, whichever section it came
from. `scripts/compat.py` keeps them apart to build its audit columns and does
not read `sources` yet, so every rule applies globally, the wider reading of the
two. And a comparison that does not know its source cannot honour scoping, so
every scoped rule applies to it, which is how the committed-fixture tests run.

## The audit's four columns

Every run reports what it excluded:

```text
excluded 156 field differences by 5 rules:
      93x  source.geo    DB-IP Lite against MaxMind GeoLite2.
      24x  tags          Beats and test-harness tagging. Not vendor data.
```

What it still counts is sorted into four columns, which is where the sections
above land:

- **real** -- the only one that means something is wrong. Everything the policy
  did not explain.
- **order** -- `unordered`, where the members match and the sequence does not.
  An undeclared array in a different order is still real.
- **meta** -- `not_emitted`.
- **enrich** -- `known_different` and `corrected` together. `nondeterministic`
  reaches no column, because it is dropped before the comparison.

Each fixture's own `dynamic_fields` and `numeric_keyword_fields` are honoured
on top of the policy. A source reports `CURRENT` when `real` is zero, and the
`clean` count is the events with no real difference.

**The corpus test runs with GeoIP OFF** (`geoip_global::disable`). Skipping the
geo fields is not enough, because the enrichment has side effects that ARE
compared: gcp/vpcflow renames `source.as.asn` onto `source.as.number`, and an
ASN hit DB-IP Lite has where MaxMind's GeoLite2 does not makes that rename land
on an occupied target -- which fails the document and skips the twenty-nine
removes behind it, costing 262 events. Comparing against output built from a
database we do not have means running without one. The fixture floors still
exercise enrichment, over the Elastic fixtures kept outside this repository and the samples in `tests/fixtures/unencumbered/`.

## Correct beats bug-compatible

The corpus is the reference for what a pipeline MEANS, not a specification of
what Elasticsearch does in every case. Where its output is wrong, we emit the
right value and record the divergence -- we do not reproduce the bug.

Two live examples. Elasticsearch 9.2.2 overflows a 2,826 ms duration to
`event.duration: -1468967296` where the correct value is 2,826,000,000, because
it multiplies into a 32-bit int. And symantec_endpoint_security carries 51
events with a non-JSON `event.original`, so both engines fail the same `json`
processor and report it in their own words -- the failure agrees and only the
wording is the runtime's own.

Both are recorded in `tests/compare-policy.yaml` with the reason attached, so
the difference is excluded from scoring and stays visible to anyone reading it.
A divergence taken this way is a decision with a name on it, and it is never
the quiet option: matching the bug would have scored better.

### Most of Elasticsearch's own failures never reach a policy entry

`compat_corpus.rs` drops any expected document whose `event.kind` is
`pipeline_error`, or that carries a bare `_compat_error` object, BEFORE
comparing -- counting it as `events_unanswered`, because scoring against a
failed run counts our correct output as a miss. 405 captured events carry an
Elasticsearch-side `error.message` and that gate excludes 336 of them.

So a policy entry is needed only where the failed document is still SCORED,
which means Elasticsearch failed a processor and its `on_failure` left
`event.kind` as `event`. That is why cisco_nexus needs no entry in this
harness despite 27 unparseable timestamps -- all 27 set `pipeline_error` and
the source scores 45/45. Its entries in `compare-policy.yaml` still do work
for `scripts/compat.py`, which applies the rules globally.

**Read the scorer before concluding an Elasticsearch failure costs anything.**
A scan of `testdata/` cannot see this gate, and sizing a prize off the captures
alone over-counts it by roughly six times.
