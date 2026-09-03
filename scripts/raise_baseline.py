#!/usr/bin/env python3
"""Apply the baseline raises `compat_corpus` prints.

The corpus test already computes the exact line to paste for every source that
improved. Pasting them by hand is a `sd` per source per run, and a typo there
lowers a ratchet instead of raising it.

Runs the corpus itself, or reads a run it is given:

    raise_baseline.py --run
    raise_baseline.py /tmp/run.txt

`--run` is the one that leaves no step for a person: the paste was the last
place a number reached the repo by hand. The writer stays here rather than in
the test, because two writers over one file format drift the way two walks over
one grammar do.

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
import subprocess
import sys

BASELINE = pathlib.Path(__file__).resolve().parent.parent / "tests/compat-baseline.json"

ENTRY = (
    r'"(?P<source>[\w]+)":\s*\{\s*"events":\s*(?P<events>\d+),\s*'
    r'"events_total":\s*(?P<total>\d+),\s*"fields_wrong":\s*(?P<wrong>\d+)\s*\},?\s*$'
)
LINE = re.compile(rf"^\s*{ENTRY}")
RESIZE = re.compile(rf"^RESIZE\s+{ENTRY}")
NEW = re.compile(rf"^NEW\s+{ENTRY}")

# The reach line the corpus run prints, so `never_ran` ratchets the same way
# every per-source score does -- written by this tool, never by hand.
REACH = re.compile(r"across \d+ distinct scripts, (?P<never>\d+) of which NEVER ran")


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


def corpus_run() -> str:
    """Run the corpus test and return everything it printed.

    A non-zero exit is not a reason to stop: the run prints its raises before
    it asserts, and a source that regressed is exactly when the others' gains
    still need recording.
    """
    command = [
        "cargo",
        "test",
        "-p",
        "dfe-transforms",
        "--test",
        "compat_corpus",
        "--",
        "--nocapture",
    ]
    print(f"  running {' '.join(command)}", file=sys.stderr)
    done = subprocess.run(
        command,
        cwd=BASELINE.parent.parent,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    return done.stdout + done.stderr


def main() -> int:
    flags = {"--resize", "--new", "--run"}
    argv = [a for a in sys.argv[1:] if a not in flags]
    if "--run" in sys.argv:
        text = corpus_run()
    elif argv:
        text = pathlib.Path(argv[0]).read_text(encoding="utf-8", errors="replace")
    else:
        text = sys.stdin.read()

    proposed = scores_in(text, LINE)
    resized = scores_in(text, RESIZE) if "--resize" in sys.argv else {}
    fresh = scores_in(text, NEW) if "--new" in sys.argv else {}
    reached = REACH.search(text)
    # A run that scored the corpus always prints its reach. Not finding it
    # means the line was reworded and this stopped ratcheting never_ran --
    # silently, which is the failure mode the corpus gate itself had.
    if "painless runtime reach" in text and not reached:
        print("the reach line changed shape -- REACH here no longer reads it", file=sys.stderr)
        return 1
    if not proposed and not resized and not fresh and not reached:
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

    for source, scores in sorted(fresh.items()):
        if source in baseline["sources"]:
            refused.append(f"{source}: already written down, so it is not new")
            continue
        baseline["sources"][source] = scores
        raised.append(
            f"{source}: FIRST SCORE {scores['events']}/{scores['events_total']} events, "
            f"{scores['fields_wrong']} fields wrong"
        )

    if reached:
        never = int(reached["never"])
        old = baseline.get("never_ran")
        if old is not None and never > old:
            refused.append(f"never_ran: {old} -> {never}")
        else:
            baseline["never_ran"] = never
            if old != never:
                raised.append(f"never_ran: {old} -> {never} scripts")

    # Written by hand rather than json.dump: the file keeps one source per
    # line, which a pretty-printer would explode into five.
    # Sorted, so a new source lands where it belongs and a diff shows only the
    # line that changed.
    body = ",\n".join(
        f'    "{name}": {{ "events": {s["events"]}, '
        f'"events_total": {s["events_total"]}, "fields_wrong": {s["fields_wrong"]} }}'
        for name, s in sorted(baseline["sources"].items())
    )
    # The head is rebuilt from the parsed keys rather than copied, so a value
    # this tool sets -- `never_ran` -- actually reaches the file. Insertion
    # order is the file's own, because `json.loads` keeps it.
    head = "".join(
        f"  {json.dumps(key)}: {json.dumps(value)},\n"
        for key, value in baseline.items()
        if key != "sources"
    )
    BASELINE.write_text(
        f'{{\n{head}  "sources": {{\n{body}\n  }}\n}}\n', encoding="utf-8", newline="\n"
    )

    for line in raised:
        print(f"  raised {line}")
    for line in refused:
        print(f"  REFUSED, this would lower a ratchet -- {line}")
    if not raised and not refused:
        print("  every ratchet already holds today's score")
    return 1 if refused else 0


if __name__ == "__main__":
    sys.exit(main())
