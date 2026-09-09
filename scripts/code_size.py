#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Measure the hand-written code and how much of it repeats.

The recurring optimisation pass asks two questions -- where is the maintenance
weight, and can abstraction remove any of it -- and both have to be measured
before anything is refactored. Reports lines per crate, the largest files, and
the share of lines sitting inside a repeated block.

Generated modules under `crates/dfe-transforms/src` are reported but excluded
from the duplication scan: they repeat by construction, and their size is the
generator's to change, not a refactor's.

    python3 scripts/code_size.py
    python3 scripts/code_size.py --window 6
"""

import argparse
import hashlib
import pathlib
import sys
from collections import defaultdict

ROOT = pathlib.Path(__file__).resolve().parent.parent
GENERATED = ROOT / "crates/dfe-transforms/src"
HAND = [
    "crates/dfe-core/src",
    "crates/dfe-painless/src",
    "crates/dfe-parse/src",
    "crates/dfe-runtime/src",
    "src",
]


def count(path):
    return len(path.read_text(encoding="utf-8", errors="replace").splitlines())


def significant(line):
    stripped = line.strip()
    if not stripped or stripped.startswith("//"):
        return None
    return " ".join(stripped.split())


def report_size():
    files = list(GENERATED.rglob("*.rs"))
    generated = sum(count(f) for f in files)
    print(f"generated  {generated:>10,} lines  {len(files):>5,} files  (crates/dfe-transforms/src)")

    total = 0
    per_crate = []
    for name in HAND:
        directory = ROOT / name
        if not directory.is_dir():
            continue
        lines = sum(count(f) for f in directory.rglob("*.rs"))
        per_crate.append((lines, name))
        total += lines
    print()
    for lines, name in sorted(per_crate, reverse=True):
        share = 100 * lines / total if total else 0
        print(f"  {lines:>8,}  {share:>5.1f}%  {name}")
    print(f"  {total:>8,}  100.0%  hand-written total")
    return total


def report_largest(limit):
    rows = []
    for name in HAND:
        directory = ROOT / name
        if not directory.is_dir():
            continue
        for f in directory.rglob("*.rs"):
            rows.append((count(f), f.relative_to(ROOT)))
    rows.sort(reverse=True)
    print(f"\nlargest {limit} hand-written files:")
    for lines, path in rows[:limit]:
        print(f"  {lines:>8,}  {path}")


def report_duplication(window):
    bodies = {}
    for name in HAND:
        directory = ROOT / name
        if not directory.is_dir():
            continue
        for f in sorted(directory.rglob("*.rs")):
            rows = [n for n in (significant(l) for l in
                                f.read_text(encoding="utf-8", errors="replace").splitlines()) if n]
            bodies[f] = rows

    seen = defaultdict(list)
    for f, rows in bodies.items():
        for i in range(len(rows) - window + 1):
            chunk = rows[i : i + window]
            if len(set(chunk)) < 4:
                continue  # a run of near-identical closing braces is not duplication
            key = hashlib.blake2b("\n".join(chunk).encode("utf-8"), digest_size=16).hexdigest()
            seen[key].append((f, i))

    repeated = {k: v for k, v in seen.items() if len(v) > 1}
    significant_lines = sum(len(r) for r in bodies.values())
    covered = set()
    for occurrences in repeated.values():
        for f, i in occurrences:
            covered.update((f, j) for j in range(i, i + window))

    share = 100 * len(covered) / significant_lines if significant_lines else 0
    print(f"\n{significant_lines:,} significant hand-written lines")
    print(f"{len(repeated):,} distinct {window}-line blocks occur more than once")
    print(f"{len(covered):,} lines sit inside a repeated block ({share:.1f}%)")

    ranked = sorted(repeated.items(), key=lambda kv: -len(kv[1]))
    if ranked:
        print("\nmost-repeated blocks:")
        for _, occurrences in ranked[:8]:
            names = defaultdict(int)
            for f, _ in occurrences:
                names[f.name] += 1
            spread = ", ".join(f"{n}x {nm}" for nm, n in sorted(names.items(), key=lambda kv: -kv[1])[:3])
            print(f"  {len(occurrences):>4} copies  [{spread}]")
    print(
        "\nA low share is the expected result and has been the measured one: the weight is\n"
        "one reader per vendor script spelling, which does not repeat. Cutting it means\n"
        "the generator transpiling more, not a refactor."
    )


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--window", type=int, default=10, help="block size for the duplication scan")
    parser.add_argument("--largest", type=int, default=10, help="how many large files to list")
    args = parser.parse_args(argv)

    report_size()
    report_largest(args.largest)
    report_duplication(args.window)
    return 0


if __name__ == "__main__":
    sys.exit(main())
