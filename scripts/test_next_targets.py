#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Tests for the corpus reader in next_targets.py.

The two regexes are the whole tool: a summary line that does not parse drops a
source silently, and a detail line that does not parse costs the wrong-on
column that makes the ranking honest. Both are matched against verbatim lines
from a real run rather than lines written to suit them.

Run with: python3 -m unittest discover -s scripts -p 'test_*.py'
"""

from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import next_targets


class ReadCorpus(unittest.TestCase):
    """Verbatim output from a `compat_corpus` run, 2026-09-06."""

    RUN = (
        "gigamon              events 49/78 (62%), fields 3577/3606 (99.2%), 0 extra, 0 errors\n"
        "      wrong in    29, unlocks    29   dns.question.subdomain\n"
        "github               events 216/237 (91%), fields 3318/3355 (98.9%), 1 extra, 0 errors\n"
        "cisco_nexus          events 45/45 (100%), fields 1007/1007 (100.0%), 0 extra,"
        " 0 errors, 27 elastic-errored\n"
        "TOTAL                events 23025/25032 (92%), fields 653769/662116 (98.7%),"
        " 5478 extra, 3 errors, 119 elastic-errored\n"
    )

    def setUp(self) -> None:
        handle = tempfile.NamedTemporaryFile("w", suffix=".txt", delete=False)
        handle.write(self.RUN)
        handle.close()
        self.path = Path(handle.name)
        self.sources = next_targets.read_corpus(self.path)

    def tearDown(self) -> None:
        self.path.unlink()

    def test_the_debt_is_the_difference_not_the_score(self) -> None:
        gigamon = self.sources["gigamon"]
        self.assertEqual(gigamon["events_missed"], 29)
        self.assertEqual(gigamon["fields_wrong"], 29)

    def test_the_detail_lines_attach_to_their_source(self) -> None:
        self.assertEqual(
            self.sources["gigamon"]["detail"],
            [(29, 29, "dns.question.subdomain")],
        )
        # A source with no detail printed still parses, with an empty list.
        self.assertEqual(self.sources["github"]["detail"], [])

    def test_a_trailing_elastic_errored_count_does_not_break_the_line(self) -> None:
        """27 of cisco_nexus's events are ones Elasticsearch itself failed.

        The count is appended after `errors` on some lines and not others, and
        a regex anchored to the end would drop every source that carries one.
        """
        self.assertIn("cisco_nexus", self.sources)
        self.assertEqual(self.sources["cisco_nexus"]["fields_wrong"], 0)

    def test_the_total_row_is_not_a_source(self) -> None:
        """The lowercase anchor keeps the rollup out of the source map.

        `TOTAL` carries the same columns as a source and would otherwise rank
        first on every run, with a debt that is every source's added together.
        Widening the pattern to `\\w+` would do it.
        """
        self.assertNotIn("TOTAL", self.sources)
        self.assertEqual(len(self.sources), 3)


class ModuleOf(unittest.TestCase):
    """The needle is escaped the way the generated file holds it."""

    def test_a_newline_and_a_quote_are_escaped_before_searching(self) -> None:
        opening = 'if (ctx.a == null) {\n  ctx.b = "x";\n}'
        held = {
            Path("filebeat/some_source/default.rs"): (
                'cached_painless!(r#"if (ctx.a == null) {\\n  ctx.b = \\"x\\";\\n}"#)'
            )
        }
        self.assertEqual(next_targets.module_of(opening, held), ["some_source"])

    def test_a_script_no_module_holds_resolves_to_nothing(self) -> None:
        self.assertEqual(next_targets.module_of("def unmatched = 1;", {}), [])


if __name__ == "__main__":
    unittest.main()
