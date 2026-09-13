#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Tests for the failure-pattern classifier in source_debt.py.

Every case here is a source diagnosed BY HAND first, with the cause then found
and fixed. They are the evidence the thresholds are drawn from, so a change to
those thresholds has to keep answering them the same way.

Run with: python3 -m unittest discover -s scripts -p 'test_*.py'
"""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import source_debt


class Classify(unittest.TestCase):
    """The three patterns, on the numbers the four worked sources printed."""

    def pattern(self, fields: int, total: int, extra: int) -> str:
        return source_debt.classify(fields, total, extra)[0]

    def test_axonius_reads_as_a_raw_shape(self) -> None:
        """2,313 extras on 3,705 fields: one unread hoist, 11% of events."""
        self.assertEqual(self.pattern(1352, 3705, 2313), "raw shape")

    def test_tanium_reads_as_nothing_emitted(self) -> None:
        """One extra and 83% of fields missing: the generator gated the body."""
        self.assertEqual(self.pattern(408, 2376, 1), "nothing emitted")

    def test_google_workspace_reads_as_a_block_not_lifted(self) -> None:
        """88% right with whole `drive.*` groups absent: a fan-out nobody read."""
        self.assertEqual(self.pattern(14106, 15991, 15), "block not lifted")

    def test_jamf_protect_reads_as_nothing_emitted(self) -> None:
        """Half the fields missing, 9 of 13 scripts never running."""
        self.assertEqual(self.pattern(1331, 2720, 34), "nothing emitted")

    def test_a_source_with_nothing_wrong_is_clean(self) -> None:
        self.assertEqual(self.pattern(2376, 2376, 0), "clean")

    def test_extras_outrank_correctness(self) -> None:
        """A mostly-right source drowning in extras is still a raw shape.

        Emitting far more than Elasticsearch is the loudest signal there is,
        whatever fraction of the rest happens to line up.
        """
        self.assertEqual(self.pattern(900, 1000, 80), "raw shape")

    def test_every_pattern_carries_advice(self) -> None:
        for fields, total, extra in [(1352, 3705, 2313), (408, 2376, 1), (14106, 15991, 15)]:
            _, advice = source_debt.classify(fields, total, extra)
            self.assertTrue(advice.strip(), "a pattern with no advice names nothing")


if __name__ == "__main__":
    unittest.main()
