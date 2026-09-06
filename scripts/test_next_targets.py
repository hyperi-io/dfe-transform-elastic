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

import contextlib
import io
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


class BestUnlocks(unittest.TestCase):
    """The unlocks ranking, against the same verbatim run."""

    def setUp(self) -> None:
        handle = tempfile.NamedTemporaryFile("w", suffix=".txt", delete=False)
        handle.write(ReadCorpus.RUN)
        handle.close()
        self.path = Path(handle.name)
        self.sources = next_targets.read_corpus(self.path)

    def tearDown(self) -> None:
        self.path.unlink()

    def test_the_best_field_is_the_one_unlocking_most_not_the_one_wrong_most(
        self,
    ) -> None:
        """Two fields, the bigger `wrong in` count worth fewer events.

        This is the whole reason the column exists: ti_opencti's top field is
        wrong in 31 and unlocks 4, so a `wrong in` sort picks the wrong one.
        """
        detail = [(31, 4, "opencti.indicator.invalid_or_revoked_from"), (8, 19, "related.hosts")]
        best = max(detail, key=lambda found: found[1])
        self.assertEqual(best[2], "related.hosts")

    def test_unlocks_is_cumulative_so_the_unit_is_the_prefix(self) -> None:
        """gdacs, verbatim: three fields wrong in the same 40 events.

        `compat_corpus.rs:1557` builds this by greedy set cover and the column
        means "once this path AND everything above it is fixed". Reporting the
        40 against `polygon_label` alone would overstate the result and
        understate the work -- it is three fields, and F69 traces all three to
        one script.
        """
        run = (
            "gdacs                events 2/42 (5%), fields 1931/2051 (94.1%),"
            " 0 extra, 0 errors\n"
            "      wrong in    40, unlocks     0   gdacs.affected_area\n"
            "      wrong in    40, unlocks     0   gdacs.class\n"
            "      wrong in    40, unlocks    40   gdacs.polygon_label\n"
        )
        handle = tempfile.NamedTemporaryFile("w", suffix=".txt", delete=False)
        handle.write(run)
        handle.close()
        path = Path(handle.name)
        try:
            sources = next_targets.read_corpus(path)
            held = io.StringIO()
            with contextlib.redirect_stdout(held):
                next_targets.best_unlocks(sources, top=5)
            printed = held.getvalue()
            self.assertIn("3 fields", printed)
            self.assertIn("gdacs.affected_area", printed)
            self.assertIn("gdacs.class", printed)
            self.assertNotIn("1 field ", printed)
        finally:
            path.unlink()

    def test_a_source_with_no_detail_lines_is_skipped(self) -> None:
        # github parses as a source and carries no detail, so it must not
        # reach `max()` on an empty list. Output captured so a passing suite
        # stays readable.
        self.assertEqual(self.sources["github"]["detail"], [])
        held = io.StringIO()
        with contextlib.redirect_stdout(held):
            next_targets.best_unlocks(self.sources, top=5)
        self.assertIn("gigamon", held.getvalue())
        self.assertNotIn("github", held.getvalue())

    def test_unlocks_equal_to_missed_is_the_whole_source(self) -> None:
        # gigamon, before its matcher landed: 29 missed, one field worth 29.
        score = self.sources["gigamon"]
        self.assertEqual(score["events_missed"], 29)
        self.assertEqual(max(score["detail"], key=lambda found: found[1])[1], 29)


class Classify(unittest.TestCase):
    """The bands, pinned to the real sources that defined each one.

    Every number here is from the 2026-09-07 run, so a band that drifts shows
    up as a source changing class rather than as an abstract threshold moving.
    """

    def test_no_extra_means_the_field_is_never_written(self) -> None:
        # gdacs: three fields one script never writes.
        self.assertEqual(next_targets.classify(120, 0), "never-written")

    def test_equal_counts_mean_the_fields_sit_at_the_wrong_path(self) -> None:
        # beyondtrust_epm, the exact 95/95 that named the band.
        self.assertEqual(next_targets.classify(95, 95), "misplaced")

    def test_a_near_miss_is_still_misplacement(self) -> None:
        # cloudflare 208/187 and gitlab 189/176 are one cause each, not two.
        self.assertEqual(next_targets.classify(208, 187), "misplaced")
        self.assertEqual(next_targets.classify(189, 176), "misplaced")

    def test_far_more_extra_than_wrong_is_over_emission(self) -> None:
        # mysql_enterprise: we keep the empty strings the vendor prunes.
        self.assertEqual(next_targets.classify(35, 106), "over-emitted")

    def test_a_large_shortfall_is_mixed_rather_than_forced(self) -> None:
        # beyondinsight carries TWO causes -- a fold and a drop -- and saying
        # "misplaced" would hide the second one.
        self.assertEqual(next_targets.classify(197, 163), "mixed")

    def test_a_tiny_source_does_not_land_in_mixed_on_one_field(self) -> None:
        # The floor of 2: without it, 3 wrong against 1 extra reads as mixed.
        self.assertEqual(next_targets.classify(3, 1), "misplaced")

    def test_the_same_root_cause_can_land_in_two_bands(self) -> None:
        """oracle and mysql_enterprise share F66's inverted drop policy.

        The band follows whichever half of the inversion dominates the counts,
        so it points at a symptom and never at a mechanism. This is the
        property that stops the classifier being read as a diagnosis.
        """
        self.assertEqual(next_targets.classify(76, 78), "misplaced")
        self.assertEqual(next_targets.classify(35, 106), "over-emitted")


if __name__ == "__main__":
    unittest.main()
