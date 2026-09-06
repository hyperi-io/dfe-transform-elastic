#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Rank the unclaimed Painless scripts by what they would actually be worth.

Two artefacts answer different halves of "what should I work on next" and
nothing joined them, so the choice was a judgement call that got made badly:
ranking by how often a script is REACHED sent a delegate at three scripts worth
44 fields between them, because their sources were already at 100%.

    DFE_PAINLESS_UNHANDLED=<dump> cargo test -p dfe-transforms \\
        --test compat_corpus --quiet -- --nocapture > <run>
    scripts/next_targets.py <dump> <run>

Three joins, in order, and each one changes the answer:

1. `ran == 0` is a gap. A script with `ran 2, skipped 326` is a matcher working
   correctly and declining on events that lack its field -- the whole winlog
   `event.code` family reads that way, and counting them is what makes a
   reach-ranked list useless.
2. The owning module, found by searching the generated tree for the script's
   opening text, then that module's source in the corpus. Skips are REACH; the
   source's `fields wrong` is the ceiling on what the script can buy.
3. The ceiling is not the value. The last column names the fields the source is
   actually wrong on, so the reader can see whether the script writes any of
   them -- `filterMassive` ranks second by ceiling and is worth six fields,
   because servicenow's real debt is a long tail it never touches.

A fourth step stays manual, because no artefact here can answer it: check the
corpus CONTAINS the data the script keys on. One `rg -cl` over `testdata/compat`
settled the heaviest unclaimed script in the catalogue as worth exactly zero.
"""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
GENERATED = ROOT / "crates/dfe-transforms/src"

# `<source> events 12/34 (35%), fields 5/6 (83.3%), 7 extra, 0 errors`
SUMMARY = re.compile(
    r"^([a-z0-9_]+) +events (\d+)/(\d+) .*?fields (\d+)/(\d+) .*?(\d+) extra, (\d+) errors"
)
# `      wrong in    27, unlocks     4   some.field`
DETAIL = re.compile(r"^\s+wrong in\s+(\d+), unlocks\s+(\d+)\s+(\S+)$")


def read_corpus(path: pathlib.Path) -> dict[str, dict]:
    """Per-source scores and their wrong-field detail, from one run's output."""
    sources: dict[str, dict] = {}
    current: str | None = None
    for line in path.read_text(errors="replace").splitlines():
        if found := SUMMARY.match(line):
            name, events, total, ok, fields, extra, errors = found.groups()
            current = name
            sources[name] = {
                "events_missed": int(total) - int(events),
                "events_total": int(total),
                "fields_wrong": int(fields) - int(ok),
                "extra": int(extra),
                "errors": int(errors),
                "detail": [],
            }
        elif current and (found := DETAIL.match(line)):
            wrong, unlocks, field = found.groups()
            sources[current]["detail"].append((int(wrong), int(unlocks), field))
    return sources


def module_of(opening: str, texts: dict[pathlib.Path, str]) -> list[str]:
    """The generated modules holding a script, by its opening text.

    The dump stores the script as the pipeline wrote it and the generated file
    holds it escaped, so the needle is escaped the same way before searching.
    """
    needle = opening.replace("\n", "\\n").replace('"', '\\"')
    return sorted({path.parent.name for path, text in texts.items() if needle in text})


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("dump", type=pathlib.Path, help="DFE_PAINLESS_UNHANDLED json")
    parser.add_argument("run", type=pathlib.Path, help="a compat_corpus run's output")
    parser.add_argument(
        "--min-skips",
        type=int,
        default=40,
        help="ignore scripts reached fewer times than this (default 40)",
    )
    parser.add_argument("--top", type=int, default=20, help="rows to print")
    args = parser.parse_args()

    dump = json.loads(args.dump.read_text())
    sources = read_corpus(args.run)
    if not sources:
        print(f"no per-source lines in {args.run} -- is it a corpus run?", file=sys.stderr)
        return 2

    texts = {path: path.read_text(errors="replace") for path in GENERATED.rglob("*.rs")}

    rows = []
    seen: set[tuple[str, int]] = set()
    for script in dump.get("scripts", []):
        if script["ran"] or script["skipped"] < args.min_skips:
            continue
        for module in module_of(script["script"][:60], texts):
            # A module is `<source>_<stream>`, and the corpus keys on the
            # source, so the longest source name the module starts with wins.
            owner = max(
                (name for name in sources if module.startswith(name)),
                key=len,
                default=None,
            )
            if owner is None or (owner, script["skipped"]) in seen:
                continue
            seen.add((owner, script["skipped"]))
            score = sources[owner]
            if not score["fields_wrong"] and not score["events_missed"]:
                continue
            rows.append((score["fields_wrong"], score["events_missed"], owner, script))

    # By debt, then by reach. Keyed rather than tuple-sorted, or two rows with
    # the same debt fall through to comparing the script dicts.
    rows.sort(key=lambda row: (row[0], row[1], row[3]["skipped"]), reverse=True)
    print(
        f"{len(rows)} unclaimed scripts on sources with debt, "
        f"of {dump.get('distinct', '?')} distinct and {dump.get('never_ran', '?')} never run\n"
    )
    for wrong, missed, owner, script in rows[: args.top]:
        score = sources[owner]
        opening = " ".join(script["script"].split())[:72]
        print(f"{wrong:5} wrong  {missed:4} events  {owner:28} reached {script['skipped']}")
        print(f"        {opening}")
        top = ", ".join(f"{field} ({n})" for n, _, field in score["detail"][:4])
        print(f"        wrong on: {top}\n")

    print(
        "Before writing a matcher: check the corpus CONTAINS the data the script\n"
        "keys on, and that the fields it writes are in the wrong-on list above."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
