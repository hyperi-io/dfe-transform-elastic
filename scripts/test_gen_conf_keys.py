#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Tests for gen_conf_keys.py, which tabulates the `_conf` keys each transform reads."""

import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import gen_conf_keys as gen

REGISTRY = """
static TRANSFORMS: &[(&str, &(dyn Transform + Sync), Intake, &str)] = &[
    (
        "filebeat.1password_audit_events.default",
        &filebeat::_1password_audit_events::default::Default,
        fetched(),
        "1password.audit_events",
    ),
    (
        "filebeat.cisco_nexus.default",
        &filebeat::cisco_nexus::default::Default,
        pushed(Framing::Line),
        "cisco_nexus.log",
    ),
];
"""


class KeysInTest(unittest.TestCase):
    """What counts as a `_conf` key in transform source."""

    def test_reads_every_form_a_transform_names_a_key_in(self) -> None:
        source = """
            event.has_value("_conf.tz_map");
            r#"{"table":"_conf.tz_map","subject":"x"}"#;
            "if (ctx._conf?.drop_status != null) {}";
            "ctx._conf['custom_date_format']";
            event.get_str("_conf.tz_offset");
        """
        self.assertEqual(
            gen.keys_in(source),
            ["custom_date_format", "drop_status", "tz_map", "tz_offset"],
        )

    def test_takes_the_first_segment_of_a_nested_key(self) -> None:
        self.assertEqual(gen.keys_in('"_conf.event.start"'), ["event"])

    def test_ignores_a_name_that_only_ends_in_conf_and_a_bare_conf(self) -> None:
        source = 'my_conf.tz_map; _temp_conf.x; event.remove("_conf"); _config.y; _confidence'
        self.assertEqual(gen.keys_in(source), [])


class TableTest(unittest.TestCase):
    """Turning the registry and the transform files into rows."""

    def _tree(self, files: dict[str, str]) -> Path:
        root = Path(self.enterContext(tempfile.TemporaryDirectory()))
        for name, text in files.items():
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text, encoding="utf-8")
        return root

    def test_a_variant_that_reads_nothing_has_no_row(self) -> None:
        root = self._tree(
            {
                "_1password_audit_events/default.rs": "fn x() {}",
                "cisco_nexus/default.rs": 'event.has_value("_conf.tz_map")',
            }
        )
        rows = gen.table(registry=REGISTRY, transforms=root)
        self.assertEqual(rows, {"filebeat.cisco_nexus.default": ["tz_map"]})

    def test_a_registry_name_differs_from_its_module_path(self) -> None:
        root = self._tree(
            {
                "_1password_audit_events/default.rs": '"_conf.tz_offset"',
                "cisco_nexus/default.rs": "",
            }
        )
        rows = gen.table(registry=REGISTRY, transforms=root)
        self.assertEqual(
            rows, {"filebeat.1password_audit_events.default": ["tz_offset"]}
        )

    def test_a_missing_transform_file_is_refused(self) -> None:
        root = self._tree({"cisco_nexus/default.rs": ""})
        with self.assertRaises(gen.RegistryError):
            gen.table(registry=REGISTRY, transforms=root)

    def test_a_registry_with_no_entries_is_refused(self) -> None:
        with self.assertRaises(gen.RegistryError):
            gen.registered("static TRANSFORMS = [];")


class RenderTest(unittest.TestCase):
    """The Rust the table is written as."""

    def test_rows_are_sorted_by_variant_so_lookup_can_bisect(self) -> None:
        text = gen.render(
            {"filebeat.b.default": ["y"], "filebeat.a.default": ["x", "z"]}
        )
        lines = [line for line in text.splitlines() if line.startswith('    ("')]
        self.assertEqual(
            lines,
            [
                '    ("filebeat.a.default", &["x", "z"]),',
                '    ("filebeat.b.default", &["y"]),',
            ],
        )
        self.assertTrue(text.endswith("];\n"))


if __name__ == "__main__":
    unittest.main()
