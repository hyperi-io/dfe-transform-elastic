#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Rank the patterns that BIND a script and then never apply it.

Two tests measure different halves of the same question and neither answers it
alone. `painless_binding` is static: it says what a script MATCHES, and a pattern
whose runner declines still matches. `compat_corpus` is dynamic: it says what
actually ran. A pattern that claims a script and then writes nothing looks
covered from everywhere except here.

    cargo test -p dfe-transforms --test painless_binding      # DFE_BINDING_DUMP
    cargo test -p dfe-transforms --test compat_corpus         # DFE_PAINLESS_UNHANDLED
    scripts/pattern_reach.py

Both dumps are written by env var, so the tests stay silent by default:

    DFE_BINDING_DUMP=<path> ...
    DFE_PAINLESS_UNHANDLED=<path> ...

A script counted here has `ran == 0` across the WHOLE corpus, so it is not a
pattern declining on the events that lack its source field -- that is ordinary
and is excluded. Scripts with no capture at all are excluded too: absence of
evidence is reported separately rather than counted as a defect.
"""

from __future__ import annotations

import argparse
import json
import pathlib
import sys
from collections import Counter

ROOT = pathlib.Path(__file__).resolve().parent.parent
DEFAULT_BINDING = ROOT / ".hyperi-ai/tmp/binding-dump.json"
DEFAULT_REACH = ROOT / ".hyperi-ai/tmp/unhandled.json"

# The census truncates a script to this many characters; the reach dump keeps
# more. Comparing on the shorter run is what lets the two be joined at all.
KEY = 60


def load(path: pathlib.Path, what: str) -> dict:
    if not path.is_file():
        sys.exit(
            f"no {what} at {path}.\n"
            "Run the two tests with DFE_BINDING_DUMP and DFE_PAINLESS_UNHANDLED set."
        )
    return json.loads(path.read_text(encoding="utf-8"))


def pattern_of(binding: str) -> str:
    """The family name, without the payload the census prints after it."""
    return binding.split("(")[0].split(" ")[0]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--binding", type=pathlib.Path, default=DEFAULT_BINDING)
    parser.add_argument("--reach", type=pathlib.Path, default=DEFAULT_REACH)
    parser.add_argument(
        "--solo",
        action="store_true",
        help="only scripts with ONE binding, so nothing ahead of it could have handled them",
    )
    args = parser.parse_args()

    census = load(args.binding, "binding census")
    reach = load(args.reach, "runtime reach dump")

    bound = {
        script["head"][:KEY]: script
        for script in census["scripts"]
        if script["binding"]
    }

    claimed: Counter[str] = Counter()
    sites: Counter[str] = Counter()
    unbound_skips = 0
    sometimes = 0

    for row in reach["scripts"]:
        if row["ran"]:
            sometimes += 1
            continue
        script = bound.get(row["script"][:KEY])
        if script is None:
            unbound_skips += row["skipped"]
            continue
        if args.solo and len(script["binding"]) > 1:
            continue
        name = pattern_of(script["binding"][0])
        claimed[name] += row["skipped"]
        sites[name] += len(script["uses"])

    print(f"{reach['distinct']} distinct scripts ran, {reach['never_ran']} never applied")
    print(f"  bound by a pattern:  {sum(claimed.values())} invocations, {sum(sites.values())} sites")
    print(f"  honestly unbound:  {unbound_skips} invocations")
    print(f"  ran and sometimes declined (ordinary, excluded): {sometimes}")

    if args.solo:
        print("\n--solo: nothing else binds these, so the claim is the only one")
    print("\nnever applied, by the pattern that claimed them:")
    for name, invocations in claimed.most_common():
        print(f"{invocations:8d} invocations  {sites[name]:5d} sites  {name}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
