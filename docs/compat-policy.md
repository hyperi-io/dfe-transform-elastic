# compat policy -- what counts as a difference

`tests/compare-policy.yaml` is the one definition of which differences between our output and Elasticsearch's are defects. The comparison harness reads it through `crates/dfe-runtime/src/testutil/policy.rs`, which is why it lives here beside the runtime. The known limits are in [compat.md](compat.md), and what parity means is [parity.md](parity.md).

Byte equality is not the goal. Every rule has a `why:` that says why the difference is not a defect, because adding a rule hides that difference from every run. The section a rule lands in decides what follows:

| Section | What it claims | What follows from it |
|---|---|---|
| `nondeterministic` | The value varies run to run and says nothing | Removed before comparison, never compared |
| `not_emitted` | Elastic or Beats plumbing DFE does not produce | Never a defect |
| `known_different` | Both sides produce it, differently | Debt. Each entry should go, and the count should fall |
| `corrected` | Elasticsearch is wrong and we are right | Permanent. Matching the bug would have scored better |
| `unordered` | The field's value is a set | The same members in another order are equal |

`known_different` and `corrected` read alike and mean opposite things. The test is whether the difference is ours to close. An array not listed in `unordered` is compared in order.

Scope a rule to its source, or it blinds every other one. A rule with `sources: [name, ...]` holds for those sources alone. Leave `sources` off where the reason is a fact about the field itself: `event.ingested` is write time everywhere.

An unscoped quirk is the expensive mistake. `event.kind` and `error.message` were once excluded everywhere for one source's date failure, so a transform emitting `pipeline_error` for any other source did not register as a difference. `unordered` is never scoped, because a set is a fact about the field.

`policy.rs` honours `sources` and chains the four rule sections into one skip set, so a skipped path is not compared whichever section it came from. A comparison that does not know its source cannot honour scoping, so every scoped rule applies to it.

## Correct beats bug-compatible

Elasticsearch's output is the reference for what a pipeline means, not a specification of what it does in every case. Where its output is wrong, we emit the right value and record the divergence in `corrected` rather than reproduce the bug.

Elasticsearch 9.2.2 multiplies milliseconds into a 32-bit int, so azure's 2,826 ms duration becomes `event.duration: -1468967296` where the correct value is 2,826,000,000. symantec_endpoint_security has events with a non-JSON `event.original`, so both engines fail the same `json` processor and report it in different words: the failure agrees and the wording differs. Both rules name the bug in their `why:`, so the difference is left out of the score and stays visible to anyone reading the file.
