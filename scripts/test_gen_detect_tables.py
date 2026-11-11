#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Tests for gen_detect_tables.py, which turns the dfe-schemas catalogue into the detector's tables."""

import hashlib
import io
import os
import sys
import tempfile
import unittest
import zipfile
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent))

import gen_detect_tables as gen


def _entry(variant: str, source: str, detect: dict | None = None) -> dict:
    """A catalogue document for an elastic *variant* labelled *source*."""
    doc = {
        "source": source,
        "transform": {"engine": "elastic", "variant": variant},
        "catalogue": {"intakes": ["beats", "receiver"]},
    }
    if detect is not None:
        doc["catalogue"]["detect"] = detect
    return doc


def _wheel(files: dict[str, str]) -> bytes:
    """A wheel archive holding *files* by name."""
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, "w") as wheel:
        for name, text in files.items():
            wheel.writestr(name, text)
    return buffer.getvalue()


class _Response(io.BytesIO):
    """A urlopen result usable as a context manager."""

    def __enter__(self):
        return self

    def __exit__(self, *exc):
        return False


class OrderTests(unittest.TestCase):
    """Rows come out in the order the detector breaks ties by."""

    def test_rows_follow_order_then_file_order_and_cef_follows_source_name(self):
        docs = [
            _entry(
                "filebeat.b.default",
                "b",
                {
                    "order": 20,
                    "markers": [
                        {"priority": 50, "any": ["b1"]},
                        {"priority": 60, "cef": {"vendor": "B", "product": "P"}},
                        {"priority": 40, "all": ["b2", "b3"]},
                    ],
                },
            ),
            _entry(
                "filebeat.a.default",
                "a",
                {"order": 30, "markers": [{"priority": 50, "any": ["a1"]}]},
            ),
            _entry(
                "filebeat.c.default",
                "c",
                {"order": 10, "markers": [{"priority": 50, "any": ["c1"]}]},
            ),
            _entry(
                "filebeat.a_cef.default",
                "a-cef",
                {"markers": [{"priority": 60, "cef": {"vendor": "A", "product": "P"}}]},
            ),
            _entry("filebeat.none.default", "none"),
        ]
        literal, cef = gen.detect_rows(gen.elastic_entries(docs))
        self.assertEqual(
            [(r["source"], r.get("any") or r.get("all")) for r in literal],
            [
                ("filebeat.c.default", ["c1"]),
                ("filebeat.b.default", ["b1"]),
                ("filebeat.b.default", ["b2", "b3"]),
                ("filebeat.a.default", ["a1"]),
            ],
        )
        self.assertEqual(
            [r["source"] for r in cef], ["filebeat.a_cef.default", "filebeat.b.default"]
        )


class RefusalTests(unittest.TestCase):
    """Catalogue data the generator will not turn into Rust."""

    CASES = [
        (
            "shared order",
            [
                _entry(
                    "filebeat.a.default",
                    "a",
                    {"order": 10, "markers": [{"priority": 1, "any": ["x"]}]},
                ),
                _entry(
                    "filebeat.b.default",
                    "b",
                    {"order": 10, "markers": [{"priority": 1, "any": ["y"]}]},
                ),
            ],
            "share detect.order 10",
        ),
        (
            "literal without order",
            [
                _entry(
                    "filebeat.a.default",
                    "a",
                    {"markers": [{"priority": 1, "any": ["x"]}]},
                )
            ],
            "no detect.order",
        ),
        (
            "verify on an all marker",
            [
                _entry(
                    "filebeat.a.default",
                    "a",
                    {
                        "order": 1,
                        "markers": [{"priority": 1, "all": ["x", "y"], "verify": "v"}],
                    },
                )
            ],
            "only for an any marker",
        ),
        (
            "verify that is not a function name",
            [
                _entry(
                    "filebeat.a.default",
                    "a",
                    {
                        "order": 1,
                        "markers": [{"priority": 1, "any": ["x"], "verify": "v(); x"}],
                    },
                )
            ],
            "not a function name",
        ),
        (
            "non-ASCII literal",
            [
                _entry(
                    "filebeat.a.default",
                    "a",
                    {"order": 1, "markers": [{"priority": 1, "any": ["caf\u00e9"]}]},
                )
            ],
            "printable ASCII",
        ),
        (
            "control character",
            [
                _entry(
                    "filebeat.a.default",
                    "a",
                    {"order": 1, "markers": [{"priority": 1, "any": ["a\nb"]}]},
                )
            ],
            "printable ASCII",
        ),
        (
            "two entries name one variant",
            [_entry("filebeat.a.default", "a"), _entry("filebeat.a.default", "b")],
            "named by two catalogue entries",
        ),
        (
            "all marker over the one-byte mask",
            [
                _entry(
                    "filebeat.a.default",
                    "a",
                    {
                        "order": 1,
                        "markers": [
                            {"priority": 1, "all": [f"k{i}=" for i in range(9)]}
                        ],
                    },
                )
            ],
            "holds 9 literals, at most 8",
        ),
        (
            "priority over a byte",
            [
                _entry(
                    "filebeat.a.default",
                    "a",
                    {"order": 1, "markers": [{"priority": 256, "any": ["x"]}]},
                )
            ],
            "does not fit a u8",
        ),
    ]

    def test_bad_catalogue_data_is_refused(self):
        for name, docs, expected in self.CASES:
            with (
                self.subTest(name),
                self.assertRaisesRegex(gen.CatalogueError, expected),
            ):
                gen.render(docs=docs, origin="test")


class RenderTests(unittest.TestCase):
    """The Rust the tables come out as."""

    def test_literals_are_escaped_and_long_lists_wrap(self):
        docs = [
            _entry(
                "filebeat.a.default",
                "a",
                {
                    "order": 1,
                    "markers": [
                        {
                            "priority": 7,
                            "any": ['"k":"', "back\\slash"],
                            "note": "quoted keys",
                        },
                        {"priority": 8, "any": ["%"], "verify": "ios_mnemonic"},
                        {
                            "priority": 9,
                            "all": [f"long-literal-number-{i}" for i in range(4)],
                        },
                    ],
                },
            )
        ]
        text = gen.render(docs=docs, origin="dfe-schemas 9.9.9")[gen.MARKERS]
        self.assertIn("from dfe-schemas 9.9.9. Do not edit by hand.", text)
        self.assertIn("use super::verify;", text)
        self.assertIn("    // quoted keys\n", text)
        self.assertIn(
            'one_of("filebeat.a.default", 7, &["\\"k\\":\\"", "back\\\\slash"]),', text
        )
        self.assertIn(
            'checked("filebeat.a.default", 8, &["%"], verify::ios_mnemonic),', text
        )
        self.assertIn(
            'every("filebeat.a.default", 9, &[\n        "long-literal-number-0",', text
        )
        self.assertTrue(
            all(
                len(line) <= gen.WIDTH
                for line in text.splitlines()
                if not line.startswith("//!")
            )
        )

    def test_max_every_counts_the_all_markers(self):
        for count in (0, 1, 33):
            markers = [{"priority": 1, "any": ["x"]}] + [
                {"priority": 1, "all": [f"a{i}", f"b{i}"]} for i in range(count)
            ]
            docs = [_entry("filebeat.a.default", "a", {"order": 1, "markers": markers})]
            text = gen.render(docs=docs, origin="test")[gen.MARKERS]
            with self.subTest(count):
                self.assertIn(f"pub(super) const MAX_EVERY: usize = {count};", text)

    def test_labels_are_sorted_by_byte_order(self):
        docs = [
            _entry("filebeat.b.default", "b"),
            _entry("filebeat.B.default", "upper"),
            _entry("filebeat.a.default", "a"),
        ]
        text = gen.render(docs=docs, origin="test")[gen.LABELS]
        rows = [line.strip() for line in text.splitlines() if line.startswith('    ("')]
        self.assertEqual(
            rows,
            [
                '("filebeat.B.default", "upper"),',
                '("filebeat.a.default", "a"),',
                '("filebeat.b.default", "b"),',
            ],
        )


class WheelTests(unittest.TestCase):
    """The pinned wheel is the only input, and a wrong digest stops the run."""

    FILES = {
        "dfe_schemas/data/catalogue/x/a.yaml": "source: a\ntransform: {engine: elastic, variant: filebeat.a.default}\n",
        "dfe_schemas/data/tables/t.yaml": "not: catalogue\n",
    }

    def setUp(self):
        self.cache = tempfile.TemporaryDirectory()
        env = {"XDG_CACHE_HOME": self.cache.name}
        self.env = mock.patch.dict(os.environ, env)
        self.env.start()
        os.environ.pop("DFE_SCHEMAS_WHEEL", None)
        self.body = _wheel(self.FILES)
        self.sha = hashlib.sha256(self.body).hexdigest()

    def tearDown(self):
        self.env.stop()
        self.cache.cleanup()

    def _urlopen(self, body: bytes):
        release = b'{"urls": [{"packagetype": "sdist", "url": "s"}, {"packagetype": "bdist_wheel", "url": "w"}]}'
        return mock.patch.object(
            gen.urllib.request,
            "urlopen",
            side_effect=lambda url, timeout: _Response(body if url == "w" else release),
        )

    def _offline(self):
        return mock.patch.object(
            gen.urllib.request, "urlopen", side_effect=OSError("no route to host")
        )

    def test_the_wheel_catalogue_is_read_when_the_digest_matches(self):
        with self._urlopen(self.body):
            fetched = gen.fetch_wheel(version="1.0.0", sha256=self.sha)
        self.assertEqual([d["source"] for d in gen.wheel_catalogue(fetched)], ["a"])

    def test_a_wheel_with_another_digest_is_refused(self):
        with (
            self._urlopen(self.body),
            self.assertRaisesRegex(gen.CatalogueError, "pinned 00"),
        ):
            gen.fetch_wheel(version="1.0.0", sha256="00")

    def test_a_download_is_cached_and_read_back_offline(self):
        with self._urlopen(self.body):
            gen.fetch_wheel(version="1.0.0", sha256=self.sha)
        self.assertTrue(gen.cache_path(self.sha).is_file())
        with self._offline():
            self.assertEqual(
                gen.fetch_wheel(version="1.0.0", sha256=self.sha), self.body
            )

    def test_a_cached_file_with_another_digest_is_refused(self):
        path = gen.cache_path(self.sha)
        path.parent.mkdir(parents=True)
        path.write_bytes(b"not the wheel")
        with self._offline(), self.assertRaisesRegex(gen.CatalogueError, "pinned"):
            gen.fetch_wheel(version="1.0.0", sha256=self.sha)

    def test_a_supplied_wheel_is_read_offline_and_held_to_the_digest(self):
        supplied = Path(self.cache.name) / "supplied.whl"
        supplied.write_bytes(self.body)
        os.environ["DFE_SCHEMAS_WHEEL"] = str(supplied)
        with self._offline():
            self.assertEqual(
                gen.fetch_wheel(version="1.0.0", sha256=self.sha), self.body
            )
            with self.assertRaisesRegex(gen.CatalogueError, "DFE_SCHEMAS_WHEEL"):
                gen.fetch_wheel(version="1.0.0", sha256="00")

    def test_no_cache_and_no_network_exits_3(self):
        pin = Path(self.cache.name) / "pin.toml"
        pin.write_text(f'version = "1.0.0"\nsha256 = "{self.sha}"\n', encoding="utf-8")
        with (
            self._offline(),
            mock.patch.object(gen, "PIN", pin),
            mock.patch("sys.stderr", io.StringIO()) as err,
        ):
            self.assertEqual(gen.main(["--check"]), gen.EXIT_UNAVAILABLE)
        self.assertIn("PyPI is unreachable", err.getvalue())

    def test_check_refuses_a_local_checkout(self):
        with self.assertRaises(SystemExit), mock.patch("sys.stderr", io.StringIO()):
            gen.main(["--check", "--from", "/nowhere"])


if __name__ == "__main__":
    unittest.main()
