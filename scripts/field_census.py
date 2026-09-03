#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""How wide a generated schema would be, per grouping level.

Schema generation groups by the registry's fourth tuple element,
`<package>.<data_stream>`, and the question that decides the shape of the
output is how many columns each level actually carries. This counts them off
the compat corpus's `expected.ndjson` - real Elasticsearch output, so it is
what a table would have to hold rather than what a mapping declares.

    scripts/field_census.py
    scripts/field_census.py --corpus /path/to/testdata/compat

The measurement that settled the design: 50,109 distinct leaf paths over 939
data streams, median stream 47 fields, widest package 4,573. A
"complete filebeat" table is therefore not a thing anyone deploys - the meta
schema is a shared vocabulary and the data stream is the deployable unit.

Re-run it when the corpus grows. The numbers move and the conclusion may not.
"""

from __future__ import annotations

import argparse
import json
import pathlib
import sys
from collections import Counter, defaultdict

DEFAULT_CORPUS = pathlib.Path(__file__).resolve().parent.parent / "testdata/compat"


def leaves(value, prefix: str = ""):
    """Every scalar leaf's dotted path.

    A list contributes its items' paths rather than an index, because a
    column holds the list and the index is not part of the field name.
    """
    if isinstance(value, dict):
        for key, sub in value.items():
            yield from leaves(sub, f"{prefix}.{key}" if prefix else key)
    elif isinstance(value, list):
        for item in value:
            yield from leaves(item, prefix)
    elif prefix:
        yield prefix


def census(corpus: pathlib.Path):
    """Distinct leaf paths per package and per data stream, and overall."""
    per_package: dict[str, set[str]] = defaultdict(set)
    per_stream: dict[str, set[str]] = defaultdict(set)
    everything: set[str] = set()
    events = 0

    for meta_path in sorted(corpus.rglob("meta.json")):
        expected = meta_path.parent / "expected.ndjson"
        if not expected.exists():
            continue
        meta = json.loads(meta_path.read_text(encoding="utf-8"))
        package = meta.get("package", "?")
        stream = f"{package}.{meta.get('data_stream', '?')}"

        for line in expected.read_text(encoding="utf-8").splitlines():
            if not line.strip():
                continue
            events += 1
            for path in leaves(json.loads(line)):
                everything.add(path)
                per_package[package].add(path)
                per_stream[stream].add(path)

    return events, everything, per_package, per_stream


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--corpus", type=pathlib.Path, default=DEFAULT_CORPUS)
    parser.add_argument("--top", type=int, default=10, help="widest packages to list")
    args = parser.parse_args()

    if not args.corpus.is_dir():
        sys.exit(
            f"no corpus at {args.corpus}.\n"
            "Generate it with `python3 scripts/compat.py generate --all`."
        )

    events, everything, per_package, per_stream = census(args.corpus)
    if not per_stream:
        sys.exit(f"{args.corpus} holds no readable captures")

    print(f"{events} events, {len(per_stream)} data streams, {len(per_package)} packages")
    print(f"every field in the corpus: {len(everything)} distinct paths")

    widths = Counter({name: len(paths) for name, paths in per_package.items()})
    print("\nwidest packages:")
    for name, width in widths.most_common(args.top):
        print(f"{width:6d}  {name}")

    streams = sorted(len(paths) for paths in per_stream.values())
    print(
        f"\ndata stream width: median {streams[len(streams) // 2]}, "
        f"max {streams[-1]}, min {streams[0]}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
