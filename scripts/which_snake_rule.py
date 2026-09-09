#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Decide which SnakeRule reproduces a source's captured key spellings.

A vendor pipeline that snake-cases its keys picks one of several incompatible
conventions, and the wrong one shows up as equal MISSING and EXTRA counts --
every field present, none under the key Elasticsearch emitted. Reading the
helper body tells you which convention the vendor MEANT; this tells you which
one its output actually agrees with.

Pairs every leaf of the parsed input against the leaves Elasticsearch emitted
under a namespace, converts each path segment under each rule in
`crates/dfe-painless/src/helpers.rs`, and reports how many keys each
reproduces. A rule scoring 100% IS the vendor's rule. Where none does, the
misses are what a new rule has to explain -- and they are usually one
condition, not a new convention.

    python3 scripts/which_snake_rule.py \\
        testdata/compat/beyondtrust_epm/audit/test-audit beyondtrust_epm.audit
"""

import argparse
import json
import pathlib
import re
import sys


def on_word_break(s):
    """Underscore only where a lowercase is followed by an uppercase."""
    out, prev_lower = [], False
    for ch in s:
        if ch.isupper():
            if prev_lower:
                out.append("_")
            out.append(ch.lower())
        else:
            out.append(ch)
        prev_lower = ch.islower()
    return "".join(out)


def before_every_upper(s):
    """Underscore before every uppercase after the first."""
    out, first = [], True
    for ch in s:
        if ch.isupper():
            if not first:
                out.append("_")
            out.append(ch.lower())
        else:
            out.append(ch)
        first = False
    return "".join(out)


def after_non_upper(s):
    """Underscore before an uppercase whose predecessor was not uppercase."""
    out, prev_upper, first = [], False, True
    for ch in s:
        if ch.isupper():
            if not first and not prev_upper:
                out.append("_")
            out.append(ch.lower())
        else:
            out.append(ch)
        prev_upper, first = ch.isupper(), False
    return "".join(out)


def _acronym_run(s, lowercase_ends_run_only, guard_double_separator):
    """The run-counter family. The two flags are the conditions that separate
    its members; see `acronym_run` and `acronym_run_strict` below."""
    if not any(c.isupper() for c in s[1:]):
        return s.lower()
    out, run, first = [], 0, True
    for ch in s:
        if ch.isupper():
            already = guard_double_separator and out and out[-1] in ("_", "-")
            if run == 0 and not first and not already:
                out.append("_")
            run += 1
            out.append(ch.lower())
        else:
            ends = ch.islower() if lowercase_ends_run_only else True
            if run > 1 and ends:
                tail = out.pop()
                out.append("_")
                out.append(tail)
            run = 0
            out.append(ch)
        first = False
    return "".join(out)


def acronym_run(s):
    """Separator before the LAST uppercase of a run: `HTTPServer` -> `http_server`."""
    return _acronym_run(s, lowercase_ends_run_only=False, guard_double_separator=False)


def acronym_run_strict(s):
    """`acronym_run` with the two conditions the captures require.

    The run's separator moves back only when a lowercase LETTER follows, so a
    run ended by `_`, `-` or end-of-string keeps its acronym whole; and no
    separator is inserted where one already sits. Without the first,
    `IT_Administrators` becomes `i_t_administrators` -- the same corruption that
    turned tanium's `Computer IP` into `computer _i_p`. Without the second,
    `Data_Protection_Policy` becomes `data__protection__policy`.
    """
    return _acronym_run(s, lowercase_ends_run_only=True, guard_double_separator=True)


def camel_break(s, eat_underscore):
    """The regex `_?([a-z])([A-Z]+)` several packages write instead of a walk."""
    pattern = r"_?([a-z])([A-Z]+)" if eat_underscore else r"([a-z])([A-Z]+)"
    return re.sub(pattern, r"\1_\2", s).lower()


RULES = {
    "OnWordBreak": on_word_break,
    "BeforeEveryUpper": before_every_upper,
    "AfterNonUpper": after_non_upper,
    "AcronymRun": acronym_run,
    "AcronymRunStrict": acronym_run_strict,
    "CamelBreak": lambda s: camel_break(s, True),
    "CamelBreakKeepingUnderscore": lambda s: camel_break(s, False),
}


def leaves(obj, prefix=""):
    if isinstance(obj, dict):
        for key, value in obj.items():
            yield from leaves(value, f"{prefix}.{key}" if prefix else key)
    else:
        yield prefix


def load(fixture):
    base = pathlib.Path(fixture)
    first = (base / "input.ndjson").read_text(encoding="utf-8", errors="replace").splitlines()[0]
    inp = json.loads(first)
    exp = json.loads(
        (base / "expected.ndjson").read_text(encoding="utf-8", errors="replace").splitlines()[0]
    )
    message = inp.get("message")
    return (json.loads(message) if isinstance(message, str) else message), exp


CORPUS = pathlib.Path(__file__).resolve().parent.parent / "testdata" / "compat"


def _score(parsed, target, convert):
    want = set(leaves(target))
    if not want:
        return None
    got = {".".join(convert(seg) for seg in path.split(".")) for path in leaves(parsed)}
    return len(want & got), len(want)


def sweep(left, right, engaged_at=0.5):
    """Compare two rules across the whole corpus.

    Answers the question a single fixture cannot: is a proposed condition a FIX
    to an existing rule, or a new convention? A rule that never scores lower
    anywhere is a fix.
    """
    better, worse, skipped, same = [], [], 0, 0
    for fixture in sorted(CORPUS.glob("*/*/*")):
        if not (fixture / "expected.ndjson").exists():
            continue
        source = fixture.parts[len(CORPUS.parts)]
        try:
            parsed, expected = load(fixture)
        except (json.JSONDecodeError, IndexError, UnicodeDecodeError, KeyError):
            continue
        if not isinstance(parsed, dict) or not isinstance(expected, dict):
            continue
        target = expected.get(source)
        if not isinstance(target, dict):
            continue

        # A stream often nests one level further (beyondtrust_epm.audit) while
        # the input leaves carry no such prefix. Take the level that scores best.
        candidates = [target]
        candidates.extend(v for v in target.values() if isinstance(v, dict))
        target = max(candidates, key=lambda c: (_score(parsed, c, RULES[left]) or (0, 1))[0])

        a = _score(parsed, target, RULES[left])
        b = _score(parsed, target, RULES[right])
        if not a or not b or a[1] < 5:
            continue
        if max(a[0], b[0]) / a[1] < engaged_at:
            skipped += 1
            continue
        if b[0] > a[0]:
            better.append((source, fixture.name, a, b))
        elif b[0] < a[0]:
            worse.append((source, fixture.name, a, b))
        else:
            same += 1

    print(f"not snake-cased, skipped ({left} under {engaged_at:.0%}): {skipped}")
    print(f"a rule engaged and the two agree:                {same}\n")
    print(f"{right} SCORES HIGHER on {len(better)}:")
    for src, fx, a, b in better:
        print(f"  {src}/{fx}: {a[0]}/{a[1]} -> {b[0]}/{b[1]}")
    print(f"\n{right} SCORES LOWER on {len(worse)}:")
    for src, fx, a, b in worse:
        print(f"  {src}/{fx}: {a[0]}/{a[1]} -> {b[0]}/{b[1]}")
    if not worse:
        print("  (none -- it is a fix to the incumbent, not a separate convention)")
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("fixture", nargs="?", help="a testdata/compat/<source>/<stream>/<fixture> directory")
    parser.add_argument("namespace", nargs="?", help="dotted path in the expected doc, e.g. beyondtrust_epm.audit")
    parser.add_argument("--show", type=int, default=12, help="how many misses to print per rule")
    parser.add_argument(
        "--sweep",
        nargs=2,
        metavar=("INCUMBENT", "CANDIDATE"),
        help="compare two rules across the whole corpus instead of one fixture",
    )
    args = parser.parse_args(argv)

    if args.sweep:
        left, right = args.sweep
        for name in (left, right):
            if name not in RULES:
                parser.error(f"unknown rule {name!r}; choose from {', '.join(RULES)}")
        return sweep(left, right)

    if not args.fixture or not args.namespace:
        parser.error("fixture and namespace are required unless --sweep is given")

    parsed, expected = load(args.fixture)
    target = expected
    for part in args.namespace.split("."):
        if not isinstance(target, dict):
            print(f"{args.namespace} is not a map in the expected document", file=sys.stderr)
            return 2
        target = target.get(part, {})

    want = set(leaves(target))
    src = sorted(leaves(parsed))
    if not want:
        print(f"nothing captured under {args.namespace}", file=sys.stderr)
        return 2

    print(f"{len(src)} input leaves, {len(want)} captured leaves under {args.namespace}\n")
    exact = []
    for name, convert in RULES.items():
        got = {".".join(convert(seg) for seg in path.split(".")) for path in src}
        hit = want & got
        print(f"{name:<28} reproduces {len(hit):>4}/{len(want)}")
        if len(hit) == len(want):
            exact.append(name)
            continue
        for miss in sorted(want - got)[: args.show]:
            print(f"{'':28}   misses {miss}")

    print()
    if exact:
        print("EXACT: " + ", ".join(exact))
    else:
        print("No rule reproduces the capture. The misses above are what a new condition must explain.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
