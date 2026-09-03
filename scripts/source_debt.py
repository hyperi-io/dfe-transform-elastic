#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Rank the parity debt, and say what SHAPE each source's failure is.

The debt is concentrated: 330 sources hold ~28,000 wrong fields and the worst
15 hold 60% of them, while the unbound Painless sites spread over 625 scripts
at 1.3 each. So the leverage is in a handful of sources, each with ONE
structural cause, and finding that cause starts with a number the run already
prints.

    scripts/source_debt.py                # the ranking, worst first
    scripts/source_debt.py axonius        # diagnose one source

The classification is the point. Reading four sources by hand produced three
signatures, and the EXTRAS count separates them before a single diff line is
read:

- **raw shape** -- many extras. Every field is missing at one path and extra
  at a longer one, so an unread structural move (a hoist, a merge, a rename)
  is putting the whole payload one level down. axonius: 2,313 extras on 3,705
  fields, one unread hoist, 11% of events to 94%.
- **nothing emitted** -- near-zero extras with most fields missing. Something
  upstream never runs at all. tanium: one condition the GENERATOR could not
  transpile, so the body sits inside `if false` and no runtime shape reaches
  it. Check `scripts/skipped_debt.py`.
- **block not lifted** -- some extras, most fields right, whole groups absent.
  A nested payload that one script should fan out. google_workspace: a
  name/value parameter list, five streams.
"""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
BASELINE = REPO / "tests/compat-baseline.json"

TOTAL = re.compile(
    r"TOTAL\s+events (?P<events>\d+)/(?P<events_total>\d+).*?"
    r"fields (?P<fields>\d+)/(?P<fields_total>\d+).*?"
    r"(?P<extra>\d+) extra,\s*(?P<errors>\d+) errors"
)
REACH = re.compile(
    r"(?P<handled>\d+) handled, (?P<skipped>\d+) skipped across "
    r"(?P<distinct>\d+) distinct scripts, (?P<never>\d+) of which NEVER ran"
)


def debt() -> list[tuple[int, int, int, str]]:
    """Every source as (fields wrong, events matched, events total, name)."""
    baseline = json.loads(BASELINE.read_text(encoding="utf-8"))
    return sorted(
        (
            (row["fields_wrong"], row["events"], row["events_total"], name)
            for name, row in baseline["sources"].items()
        ),
        reverse=True,
    )


def classify(fields: int, fields_total: int, extra: int) -> tuple[str, str]:
    """Name the failure's shape, and what to do about it.

    Thresholds are read off the four sources diagnosed by hand, and are meant
    to point rather than to decide: the diff is still the evidence.
    """
    missing = fields_total - fields
    if missing == 0:
        return ("clean", "Nothing wrong to explain.")
    correct = fields / max(fields_total, 1)

    # Extras first, because they are the loudest signal: emitting far more than
    # Elasticsearch means the payload is landing somewhere else entirely.
    if extra >= missing // 2:
        return (
            "raw shape",
            "Fields are MISSING at one path and EXTRA at a longer one: an "
            "unread hoist, merge or rename is leaving the payload a level down. "
            "Find the one script that moves it.",
        )
    # Then how much of the event is right at all. Mostly right with groups
    # absent is a different problem from mostly absent.
    if correct >= 0.8:
        return (
            "block not lifted",
            "Most of each event is right and whole GROUPS of fields are absent: "
            "a nested payload one script should fan out or lift.",
        )
    return (
        "nothing emitted",
        "Most of the event is missing with nothing extra, so a step upstream "
        "never runs. Check scripts/skipped_debt.py -- a condition the GENERATOR "
        "could not transpile puts its whole body inside `if false`.",
    )


def diagnose(source: str) -> int:
    command = [
        "cargo", "test", "-p", "dfe-transforms", "--test", "compat_corpus",
        "--", "--nocapture",
    ]
    env = {"DFE_COMPAT_ONLY": source, "DFE_COMPAT_DETAIL": source}
    print(f"  scoring {source} alone", file=sys.stderr)
    done = subprocess.run(
        command,
        cwd=REPO,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        env={**dict(__import__("os").environ), **env},
        check=False,
    )
    text = done.stdout + done.stderr

    total = TOTAL.search(text)
    if not total:
        print(f"{source}: the run printed no TOTAL -- is it in the corpus?")
        return 1
    fields = int(total["fields"])
    fields_total = int(total["fields_total"])
    extra = int(total["extra"])
    shape, advice = classify(fields, fields_total, extra)

    print(f"\n{source}")
    print(f"  events   {total['events']}/{total['events_total']}")
    print(f"  fields   {fields}/{fields_total}  ({fields_total - fields} missing)")
    print(f"  extra    {extra}")
    print(f"  errors   {total['errors']}")
    if reach := REACH.search(text):
        print(
            f"  scripts  {reach['distinct']} distinct, {reach['never']} never ran, "
            f"{reach['skipped']} invocations skipped"
        )
    print(f"\n  SHAPE: {shape}\n  {advice}")

    blockers = [line for line in text.splitlines() if "wrong in" in line]
    if blockers:
        print("\n  the fields blocking the most events:")
        for line in blockers[-6:]:
            print(f"  {line.strip()}")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("source", nargs="?", help="diagnose this source")
    parser.add_argument("--top", type=int, default=15)
    args = parser.parse_args()

    if args.source:
        return diagnose(args.source)

    rows = debt()
    total = sum(row[0] for row in rows)
    top = sum(row[0] for row in rows[: args.top])
    print(f"{len(rows)} sources, {total} fields wrong")
    print(f"the worst {args.top} hold {top} of them ({top * 100 // max(total, 1)}%)\n")
    print(f"{'wrong':>7}  {'events':>13}  source")
    for wrong, events, events_total, name in rows[: args.top]:
        print(f"{wrong:>7}  {events:>5}/{events_total:<7}  {name}")
    print("\nDiagnose one with: scripts/source_debt.py <source>")
    return 0


if __name__ == "__main__":
    sys.exit(main())
