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

`--classify <run>` answers a different question, from the run ALONE, and it
exists because the join above is blind to a whole failure class. Ranking
unclaimed scripts can only ever find a source whose debt has an unclaimed
script behind it. A source whose matcher IS claimed and derives the WRONG
policy never appears -- mysql_enterprise sat at 1/34 with its pruner taken by
a matcher that applied the inverse of it. So does a source whose debt is a
grok definition rather than a script: pfsense's 57 missing events were a
missing word boundary in `NONNEGINT`.

What the ratio of EXTRA to WRONG says, before any capture is opened:

    extra ~= 0        the field is genuinely never written
    extra ~= wrong    fields sit at the WRONG PATH -- each counts twice, once
                      missing where it belongs and once extra where it sits.
                      Go looking for a rename or key-fold that never ran
    extra >> wrong    we EMIT what Elasticsearch does not -- unpruned scratch,
                      or a writer firing where the vendor's declines

It names the CLASS and never the mechanism. beyondtrust_epm reads as a clean
95/95 misplacement and is NOT the `field_mappings` fold its sibling BeyondTrust
package uses. A source can also carry two causes at once, which is why the
mixed band exists rather than being forced into one of the other three.
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


# `ctx.a.b.c =` (not `==`), and the `ctx.a.b.put('c', ...)` member write.
ASSIGN = re.compile(r"ctx\.([A-Za-z0-9_.?]+)\s*=(?!=)")
PUT = re.compile(r"ctx\.([A-Za-z0-9_.?]+)\.put\(\s*['\"]([^'\"]+)['\"]")


def script_writes(script: str) -> set[str]:
    """The ctx paths a script assigns to, null-safe navigation stripped.

    Text, not a parser, so it is approximate on purpose -- it exists to
    INTERSECT with a wrong-field list, where a false path simply fails to
    match and a missed one costs a row nobody would have read anyway.
    """
    found = {m.group(1).replace("?", "").rstrip(".") for m in ASSIGN.finditer(script)}
    found |= {
        f"{m.group(1).replace('?', '').rstrip('.')}.{m.group(2)}"
        for m in PUT.finditer(script)
    }
    return {path for path in found if path}


def classify(wrong: int, extra: int) -> str:
    """The failure class a source's extra-to-wrong ratio points at.

    Bands rather than exact equality: a source carrying one dominant cause
    still picks up a stray field or two, and `beyondinsight` is 197 wrong
    against 163 extra -- mostly misplaced, with some never written at all.
    The floor of 2 keeps a tiny source out of the mixed band on one field.
    """
    if extra == 0:
        return "never-written"
    if abs(extra - wrong) <= max(2, round(wrong * 0.15)):
        return "misplaced"
    if extra > wrong:
        return "over-emitted"
    return "mixed"


def module_of(opening: str, texts: dict[pathlib.Path, str]) -> list[str]:
    """The generated modules holding a script, by its opening text.

    The dump stores the script as the pipeline wrote it and the generated file
    holds it escaped, so the needle is escaped the same way before searching.
    """
    needle = opening.replace("\n", "\\n").replace('"', '\\"')
    return sorted({path.parent.name for path, text in texts.items() if needle in text})


def best_unlocks(sources: dict[str, dict], top: int) -> None:
    """Sources ranked by the smallest set of fields that buys the most events.

    `fields wrong` is a ceiling and a long tail makes it a lie: ti_opencti is
    179 wrong, second-biggest of its class, and its whole detail list buys
    FOUR events.

    **`unlocks` is CUMULATIVE, not per-field.** The corpus builds the list by
    greedy set cover and the column means "events that pass once this path AND
    EVERYTHING ABOVE IT is fixed" (`compat_corpus.rs:1557`). So the unit is the
    PREFIX, never one line. gdacs reads `affected_area 0, class 0,
    polygon_label 40` -- three fields wrong in the same 40 events, and the 40
    belongs to fixing all three. Reporting it as one field overstates the
    result and understates the work.
    """
    rows = []
    for name, score in sources.items():
        detail = score["detail"]
        if not detail:
            continue
        # The list is already in greedy-cover order, so the prefix ending at
        # the best-unlocking line is the set that buys those events.
        cut = max(range(len(detail)), key=lambda index: detail[index][1])
        unlocks = detail[cut][1]
        if unlocks < 8:
            continue
        rows.append((unlocks, -(cut + 1), score["events_missed"], name, detail[: cut + 1], score))
    rows.sort(reverse=True, key=lambda row: (row[0], row[1], row[2], row[3]))

    print(f"-- fewest fields worth 8+ events, best first ({len(rows)} sources)")
    for unlocks, negative_needed, missed, name, prefix, score in rows[:top]:
        klass = classify(score["fields_wrong"], score["extra"])
        whole = " -- WHOLE SOURCE" if unlocks == missed else ""
        needed = -negative_needed
        plural = "" if needed == 1 else "s"
        print(
            f"{unlocks:6} unlocks {missed:5} missed  {needed} field{plural}  "
            f"{name}  [{klass}]{whole}"
        )
        for wrong, _, field in prefix:
            print(f"{'':22}{field}  (wrong in {wrong})")
    print()


def report_classes(run: pathlib.Path, top: int) -> int:
    """Every source with debt, by failure class, from a run alone."""
    sources = read_corpus(run)
    if not sources:
        print(f"no per-source lines in {run} -- is it a corpus run?", file=sys.stderr)
        return 2

    best_unlocks(sources, top)

    rows = [
        (score["fields_wrong"], score["events_missed"], score["extra"], name)
        for name, score in sources.items()
        if score["fields_wrong"] or score["events_missed"]
    ]
    rows.sort(reverse=True)

    grouped: dict[str, list[tuple[int, int, int, str]]] = {}
    for row in rows:
        grouped.setdefault(classify(row[0], row[2]), []).append(row)

    tally = ", ".join(f"{name} {len(found)}" for name, found in sorted(grouped.items()))
    print(f"{len(rows)} sources with debt -- {tally}\n")

    # Misplacement first: it names a cause rather than a symptom, and a rename
    # that never ran is one matcher for a whole subtree.
    order = ["misplaced", "never-written", "mixed", "over-emitted"]
    for name in order:
        found = grouped.get(name)
        if not found:
            continue
        print(f"-- {name}")
        for wrong, missed, extra, source in found[:top]:
            print(f"{wrong:6} wrong {extra:6} extra {missed:5} events  {source}")
        print()

    print(
        "The class names the mechanism NOWHERE. It says where to look:\n"
        "  misplaced     -> a rename or key-fold that never ran\n"
        "  never-written -> a writer that never fires\n"
        "  over-emitted  -> scratch we do not prune, or a writer firing too widely\n"
        "A source can carry two causes, which is what the mixed band is."
    )
    return 0


def report_claimed(dump: dict, sources: dict[str, dict], top: int) -> int:
    """Scripts claimed on EVERY event that write a field their source is wrong on.

    The inverse band to the default mode. `ran > 0, skipped 0` means a matcher
    recognised and ran the script every time it was reached, which reads like
    success -- `painless_stats::record_handled` says RECOGNISED AND RUN and
    nothing about the output. checkpoint_email sits there `ran 18, skipped 0`
    with its field wrong in all 18.

    The band alone is noise: 459 scripts are in it, most working while their
    source's debt is elsewhere. The signal is the INTERSECTION with the
    wrong-field list, which cuts it to 43.
    """
    texts = {path: path.read_text(errors="replace") for path in GENERATED.rglob("*.rs")}
    rows = []
    seen: set[tuple[str, tuple[str, ...]]] = set()
    for script in dump.get("scripts", []):
        if script["ran"] == 0 or script["skipped"] != 0:
            continue
        target = script_writes(script["script"])
        if not target:
            continue
        for module in module_of(script["script"][:60], texts):
            owner = max(
                (name for name in sources if module.startswith(name)), key=len, default=None
            )
            if owner is None:
                continue
            score = sources[owner]
            hit = target & {field for _, _, field in score["detail"]}
            if not hit:
                continue
            key = (owner, tuple(sorted(hit)))
            if key in seen:
                continue
            seen.add(key)
            worst = max(n for n, _, field in score["detail"] if field in hit)
            # `unlocks` is CUMULATIVE, so the honest event count for this
            # script's own fields is the BEST unlocks among them -- never the
            # `wrong in` count, which is a field tally and reads far larger.
            unlocks = max(u for _, u, field in score["detail"] if field in hit)
            rows.append((worst, unlocks, owner, sorted(hit), script))

    # By events bought, then by fields damaged: `wrong in` is a field tally, so
    # six fields each wrong in 22 can still be worth four events.
    rows.sort(reverse=True, key=lambda row: (row[1], row[0], row[2]))
    print(f"{len(rows)} claimed scripts writing a field their source is WRONG on\n")
    for worst, unlocks, owner, hit, script in rows[:top]:
        score = sources[owner]
        klass = classify(score["fields_wrong"], score["extra"])
        whole = "  -- WHOLE SOURCE" if unlocks and unlocks == score["events_missed"] else ""
        print(
            f"wrong in {worst:4}  unlocks {unlocks:4}  {owner:28} ran {script['ran']:4} "
            f"[{klass}] {score['events_missed']} missed{whole}"
        )
        print(f"        writes: {', '.join(hit)}")
        print(f"        {' '.join(script['script'].split())[:92]}\n")

    print(
        "`ran > 0` means CLAIMED, never CORRECT. A matcher here recognised the\n"
        "script and ran it, and the field it writes is still wrong -- so read the\n"
        "matcher's output, not its reach.\n"
        "`wrong in` is a FIELD tally and is always the bigger number. `unlocks`\n"
        "is the event count, and it is cumulative -- WHOLE SOURCE is marked only\n"
        "where this script's own fields carry the whole gap."
    )
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "dump", type=pathlib.Path, nargs="?", help="DFE_PAINLESS_UNHANDLED json"
    )
    parser.add_argument(
        "run", type=pathlib.Path, nargs="?", help="a compat_corpus run's output"
    )
    parser.add_argument(
        "--classify",
        type=pathlib.Path,
        metavar="RUN",
        help="classify every source with debt by its extra-to-wrong ratio, "
        "from a run alone -- finds the sources the script join cannot see",
    )
    parser.add_argument(
        "--min-skips",
        type=int,
        default=40,
        help="ignore scripts reached fewer times than this (default 40)",
    )
    parser.add_argument(
        "--claimed",
        action="store_true",
        help="the INVERSE band: scripts a matcher claims on every event that "
        "still write a field their source is wrong on",
    )
    parser.add_argument("--top", type=int, default=20, help="rows to print")
    args = parser.parse_args()

    if args.classify:
        return report_classes(args.classify, args.top)

    if args.dump is None or args.run is None:
        parser.error("both <dump> and <run> are required without --classify")

    dump = json.loads(args.dump.read_text())
    sources = read_corpus(args.run)
    if not sources:
        print(f"no per-source lines in {args.run} -- is it a corpus run?", file=sys.stderr)
        return 2

    if args.claimed:
        return report_claimed(dump, sources, args.top)

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
