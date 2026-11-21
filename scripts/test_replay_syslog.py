#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Tests for the date and time restamp replay_syslog.py applies at send time.

Run with: python3 -m unittest discover -s scripts -p 'test_*.py'
"""

import re
import sys
import unittest
from datetime import UTC, datetime, timedelta, timezone
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import replay_syslog

# A send-time stamp as it appears in a fortinet body, once restamped.
STAMP = re.compile(r"date=(?P<day>\d{4}-\d{2}-\d{2}) time=(?P<clock>\d{2}:\d{2}:\d{2})")

# (case, fixture line, expected output with the send-time stamp masked to <day> and <clock>)
WRAP_CASES = (
    (
        "fortinet without a PRI gets a header and a fresh stamp",
        "Aug 18 12:15:35 192.168.250.1 date=2016-08-18 time=12:15:35 type=traffic",
        "<134>Aug 18 12:15:35 192.168.250.1 date=<day> time=<clock> type=traffic",
    ),
    (
        "a line with no date or time gets only its header",
        "Aug 18 12:15:35 192.168.250.1 action=close",
        "<134>Aug 18 12:15:35 192.168.250.1 action=close",
    ),
    (
        "a line that already has a PRI keeps it and gets a fresh stamp",
        "<189>Aug 18 12:15:35 192.168.250.1 date=2016-08-18 time=12:15:35 type=traffic",
        "<189>Aug 18 12:15:35 192.168.250.1 date=<day> time=<clock> type=traffic",
    ),
)


class WrapStampsTheBody(unittest.TestCase):
    """wrap() stamps the send time into the body and leaves the rest of the line alone."""

    def test_each_case_keeps_its_header_and_restamps_only_the_fields(self) -> None:
        for case, line, expected in WRAP_CASES:
            with self.subTest(case):
                before = datetime.now(UTC).replace(microsecond=0)
                out = replay_syslog.wrap(line)
                after = datetime.now(UTC)
                stamp = STAMP.search(out)
                if stamp:
                    sent = datetime.strptime(
                        f"{stamp['day']} {stamp['clock']}", "%Y-%m-%d %H:%M:%S"
                    ).replace(tzinfo=UTC)
                    self.assertLessEqual(before, sent)
                    self.assertLessEqual(sent, after)
                masked = STAMP.sub("date=<day> time=<clock>", out)
                self.assertEqual(masked, expected)


class RestampFields(unittest.TestCase):
    """restamp() writes the send time into date= and time=, in UTC."""

    def test_fields_take_the_send_time_converted_to_utc(self) -> None:
        """A send time of 02:30 on 21 Nov at UTC+11 is 15:30 on 20 Nov in UTC."""
        sydney = timezone(timedelta(hours=11))
        now = datetime(2026, 11, 21, 2, 30, 5, tzinfo=sydney)
        self.assertEqual(
            replay_syslog.restamp("date=2016-08-18 time=12:15:35 type=traffic", now),
            "date=2026-11-20 time=15:30:05 type=traffic",
        )

    def test_a_line_without_the_fields_is_unchanged(self) -> None:
        now = datetime(2026, 11, 21, 2, 30, 5, tzinfo=UTC)
        for line in (
            "Aug 18 12:15:35 host action=close",
            "mytime=12:15:35 update=2016-08-18",
            "date=2016/08/18 time=12:15",
        ):
            with self.subTest(line):
                self.assertEqual(replay_syslog.restamp(line, now), line)


if __name__ == "__main__":
    unittest.main()
