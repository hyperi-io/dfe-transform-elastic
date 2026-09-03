#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Rank skipped processors by the parity debt of the source they sit in.

A processor whose `if` condition did not transpile is emitted inside `if false`
and can never run, whatever the runtime learns to read. `skipped_processors.rs`
counts them; `tests/compat-baseline.json` says what each source is losing. On
their own neither says which generator gap to close first.

    scripts/skipped_debt.py
    scripts/skipped_debt.py --top 30

tanium is why this exists: ONE untranspiled condition,
`ctx.json['Match Details'] instanceof Map`, holds 1,968 fields across 54
events by putting the whole payload merge out of reach.

The join is by PACKAGE, taken from the generated directory name up to its
first underscore-separated word that the baseline knows. That is approximate --
a directory is `<package>_<data stream>` with no separator to tell them apart --
so treat the ranking as a worklist, never as an exact attribution.
"""

from __future__ import annotations

import argparse
import collections
import json
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
GENERATED = REPO / "crates/dfe-transforms/src"
BASELINE = REPO / "tests/compat-baseline.json"
MARKER = "// SKIPPED: condition not transpiled: "

CONDITION = re.compile(re.escape(MARKER) + r"(.*)$")


def skipped_by_directory() -> dict[str, list[str]]:
    """Every untranspiled condition, by the integration directory holding it."""
    found: dict[str, list[str]] = collections.defaultdict(list)
    for path in sorted(GENERATED.rglob("*.rs")):
        for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
            matched = CONDITION.search(line)
            if matched:
                found[path.parent.name].append(matched.group(1).strip())
    return found


def debt() -> dict[str, tuple[int, int, int]]:
    """Per source: fields wrong, events matched, events total."""
    baseline = json.loads(BASELINE.read_text(encoding="utf-8"))
    return {
        name: (row["fields_wrong"], row["events"], row["events_total"])
        for name, row in baseline["sources"].items()
    }


def package_of(directory: str, known: set[str]) -> str | None:
    """The baseline source a generated directory belongs to.

    `microsoft_dhcp_log` is package `microsoft_dhcp`, `tanium_threat_response`
    is `tanium`. Nothing in the name says where the package stops, so the
    LONGEST known prefix wins.
    """
    parts = directory.split("_")
    for end in range(len(parts), 0, -1):
        candidate = "_".join(parts[:end])
        if candidate in known:
            return candidate
    return None


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--top", type=int, default=20)
    args = parser.parse_args()

    if not GENERATED.is_dir():
        sys.exit(f"no generated tree at {GENERATED}")
    if not BASELINE.is_file():
        sys.exit(f"no baseline at {BASELINE}")

    skipped = skipped_by_directory()
    scores = debt()
    known = set(scores)

    ranked: dict[str, tuple[int, int, int, int]] = {}
    unattributed = 0
    for directory, conditions in skipped.items():
        package = package_of(directory, known)
        if package is None:
            unattributed += len(conditions)
            continue
        wrong, events, total = scores[package]
        markers, *_ = ranked.get(package, (0, 0, 0, 0))
        ranked[package] = (markers + len(conditions), wrong, events, total)

    total_markers = sum(len(v) for v in skipped.values())
    print(f"{total_markers} skipped processors over {len(skipped)} directories")
    print(f"{unattributed} sit in a source the corpus does not score\n")
    print(f"{'skipped':>8}  {'wrong':>7}  {'events':>12}  source")
    order = sorted(ranked.items(), key=lambda kv: -kv[1][1])
    for package, (markers, wrong, events, total) in order[: args.top]:
        print(f"{markers:>8}  {wrong:>7}  {events:>5}/{total:<6}  {package}")

    print("\nthe conditions themselves, worst source first:")
    for package, _ in order[:5]:
        for directory, conditions in sorted(skipped.items()):
            if package_of(directory, known) != package:
                continue
            for condition in conditions:
                print(f"  {package:<24} {condition}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
