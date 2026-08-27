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

A source whose event TOTAL moved is a different measurement, so neither its
event floor nor its field count carries across and the ratchet cannot compare
them. The test marks those `RESIZE` and this only rewrites them under
`--resize`, which accepts whatever they now score.
"""

from __future__ import annotations

import json
import pathlib
import re
import sys

BASELINE = pathlib.Path(__file__).resolve().parent.parent / "tests/compat-baseline.json"

ENTRY = (
    r'"(?P<source>[\w]+)":\s*\{\s*"events":\s*(?P<events>\d+),\s*'
    r'"events_total":\s*(?P<total>\d+),\s*"fields_wrong":\s*(?P<wrong>\d+)\s*\},?\s*$'
)
LINE = re.compile(rf"^\s*{ENTRY}")
RESIZE = re.compile(rf"^RESIZE\s+{ENTRY}")


def scores_in(text: str, pattern: re.Pattern[str]) -> dict[str, dict[str, int]]:
    found: dict[str, dict[str, int]] = {}
    for line in text.splitlines():
        match = pattern.match(line)
        if match:
            found[match["source"]] = {
                "events": int(match["events"]),
                "events_total": int(match["total"]),
                "fields_wrong": int(match["wrong"]),
            }
    return found


def main() -> int:
    argv = [a for a in sys.argv[1:] if a != "--resize"]
    resizing = "--resize" in sys.argv
    text = (
        pathlib.Path(argv[0]).read_text(encoding="utf-8", errors="replace")
        if argv
        else sys.stdin.read()
    )

    proposed = scores_in(text, LINE)
    resized = scores_in(text, RESIZE) if resizing else {}
    if not proposed and not resized:
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

    # No ratchet check: the old entry counted a different set of events, so
    # there is nothing here to compare against.
    for source, scores in sorted(resized.items()):
        old = baseline["sources"].get(source)
        baseline["sources"][source] = scores
        raised.append(
            f"{source}: RESIZED from {old['events_total'] if old else 0} events to "
            f"{scores['events_total']} -- {scores['events']} matched, "
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
