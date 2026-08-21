#!/usr/bin/env python3
"""Apply the baseline raises `compat_corpus` prints.

The corpus test already computes the exact line to paste for every source that
improved. Pasting them by hand is a `sd` per source per run, and a typo there
lowers a ratchet instead of raising it.

Reads the test's output on stdin, or from a file:

    cargo test ... -- --nocapture > /tmp/run.txt
    raise_baseline.py /tmp/run.txt

Refuses any line that would LOWER a score: a fall needs a stated reason and is
never mechanical.
"""

from __future__ import annotations

import json
import pathlib
import re
import sys

BASELINE = pathlib.Path(__file__).resolve().parent.parent / "tests/compat-baseline.json"

LINE = re.compile(
    r'^\s*"(?P<source>[\w]+)":\s*\{\s*"events":\s*(?P<events>\d+),\s*'
    r'"events_total":\s*(?P<total>\d+),\s*"fields_wrong":\s*(?P<wrong>\d+)\s*\},?\s*$'
)


def main() -> int:
    text = (
        pathlib.Path(sys.argv[1]).read_text(encoding="utf-8", errors="replace")
        if len(sys.argv) > 1
        else sys.stdin.read()
    )

    proposed: dict[str, dict[str, int]] = {}
    for line in text.splitlines():
        match = LINE.match(line)
        if match:
            proposed[match["source"]] = {
                "events": int(match["events"]),
                "events_total": int(match["total"]),
                "fields_wrong": int(match["wrong"]),
            }
    if not proposed:
        print("nothing to raise")
        return 0

    baseline = json.loads(BASELINE.read_text(encoding="utf-8"))
    raised, refused = [], []
    for source, scores in sorted(proposed.items()):
        old = baseline["sources"].get(source)
        if old and (
            scores["events"] < old["events"] or scores["fields_wrong"] > old["fields_wrong"]
        ):
            refused.append(f"{source}: {old} -> {scores}")
            continue
        baseline["sources"][source] = scores
        raised.append(
            f"{source}: {scores['events']}/{scores['events_total']} events, "
            f"{scores['fields_wrong']} fields wrong"
        )

    # Written by hand rather than json.dump: the file keeps one source per
    # line, which a pretty-printer would explode into five.
    body = ",\n".join(
        f'    "{name}": {{ "events": {s["events"]}, '
        f'"events_total": {s["events_total"]}, "fields_wrong": {s["fields_wrong"]} }}'
        for name, s in baseline["sources"].items()
    )
    head = BASELINE.read_text(encoding="utf-8").split('  "sources": {')[0]
    BASELINE.write_text(
        f'{head}  "sources": {{\n{body}\n  }}\n}}\n', encoding="utf-8", newline="\n"
    )

    for line in raised:
        print(f"  raised {line}")
    for line in refused:
        print(f"  REFUSED, this would lower a ratchet -- {line}")
    return 1 if refused else 0


if __name__ == "__main__":
    sys.exit(main())
