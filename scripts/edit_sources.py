#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Append a proposed block to `sources.yaml`, or take named sources out of it.

`propose_sources.py` in the dev repo derives the block; this is what puts it in
and what takes it back out when a pipeline does not generate, because a
declaration is the statement that a source is onboarded.

Entries are separated by a blank line and any comment above one belongs to it,
which is what makes the blocks safe to drop whole.

    scripts/edit_sources.py add block.yaml
    scripts/edit_sources.py drop cribl_logs cribl_metrics
"""

import pathlib
import sys

SOURCES = pathlib.Path(__file__).resolve().parent.parent / "sources.yaml"


def add(block_path: str) -> None:
    """Append a proposal's body, which carries its own `sources:` header."""
    block = pathlib.Path(block_path).read_text(encoding="utf-8")
    body = block.split("sources:\n", 1)[1].rstrip("\n")
    current = SOURCES.read_text(encoding="utf-8").rstrip("\n")
    SOURCES.write_text(current + "\n\n" + body + "\n", encoding="utf-8", newline="\n")
    print(f"appended {len(body.splitlines())} lines")


def drop(names: list[str]) -> None:
    """Remove each named source, comment block and all."""
    wanted = set(names)
    kept, dropped = [], []
    for block in SOURCES.read_text(encoding="utf-8").split("\n\n"):
        keys = [
            line.strip().rstrip(":")
            for line in block.splitlines()
            if line.startswith("  ")
            and not line.startswith("   ")
            and line.rstrip().endswith(":")
        ]
        if len(keys) == 1 and keys[0] in wanted:
            dropped.append(keys[0])
            continue
        kept.append(block)

    SOURCES.write_text("\n\n".join(kept), encoding="utf-8", newline="\n")
    print(f"dropped {len(dropped)}: {', '.join(sorted(dropped))}")
    missing = sorted(wanted - set(dropped))
    if missing:
        print(f"NOT FOUND as a whole block: {', '.join(missing)}")


def main() -> int:
    match sys.argv[1:]:
        case ["add", block]:
            add(block)
        case ["drop", *names] if names:
            drop(names)
        case _:
            print(__doc__)
            return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
