#!/usr/bin/env -S uv run --script
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib==3.11.2"]
# ///
"""Charts and derived figures for docs/ingest-cost-bench-elastic.md.

Reads the per-cell JSON that `three_way.py` writes and draws three SVG charts in the
HyperI palette. The same JSON always gives the same SVG: the hash salt is fixed, text is
drawn as paths so no viewer font matters, and the file carries no date.

Examples:
    uv run scripts/bench/charts.py charts --results scripts/bench/results --out docs/bench/img
    uv run scripts/bench/charts.py figures --results scripts/bench/results
"""

import argparse
import json
import statistics as st
import sys
from collections.abc import Callable
from pathlib import Path
from typing import Any

import matplotlib

matplotlib.use("svg")

import matplotlib.pyplot as plt
from matplotlib.lines import Line2D
from matplotlib.patches import Patch, PathPatch
from matplotlib.path import Path as MplPath

# --------------------------------------------------------------------------
# Palette. Navy, blue and green are hyperi-graphics packages/brand/src/brand.css.
# Elastic is a neutral grey that holds 4.8:1 against the white surface.
# --------------------------------------------------------------------------

NAVY = "#000647"
MUTED = "#5B6086"
GRID = "#E4E6EE"
SURFACE = "#FFFFFF"
DFE = "#2DED88"
VECTOR = "#2EA4F6"
ELASTIC = "#6B7280"

SOURCES = ("cisco_ios", "crowdstrike_falcon", "okta_system", "fortinet_fortigate")
ENGINES = ("transform-elastic", "vector", "es-bulk", "es-simulate")
LABEL = {
    "transform-elastic": "dfe-transform-elastic",
    "vector": "Vector (DFE 2.1)",
    "es-bulk": "Elasticsearch _bulk",
    "es-simulate": "Elasticsearch _simulate",
}
COLOUR = {
    "transform-elastic": DFE,
    "vector": VECTOR,
    "es-bulk": ELASTIC,
    "es-simulate": ELASTIC,
}
# Elasticsearch has two modes in one colour. The pipeline-only mode wears the 45 degree hatch.
HATCH = {"es-simulate": "////"}

YEAR_FACTOR = 2.26

WIDTH_PT = 640.0
HEIGHT_PT = 380.0
RADIUS_PT = 3.0
GAP_PT = 1.5
BAR_PT = 24.0


def load(results: Path) -> dict[tuple[str, str, str], list[dict[str, Any]]]:
    """Group the measured, quiet runs that reached their record count by cell."""
    cells: dict[tuple[str, str, str], list[dict[str, Any]]] = {}
    for path in sorted(results.glob("*.json")):
        for run in json.loads(path.read_text(encoding="utf-8"))["runs"]:
            if (
                run.get("measured")
                and run.get("stop_reason") == "target"
                and not run.get("noisy")
                and run.get("rate_per_core") is not None
            ):
                cells.setdefault(
                    (run["source"], run["engine"], run["mode"]), []
                ).append(run)
    return cells


def mean(
    cells: dict, source: str, engine: str, mode: str, key: str = "rec_per_cpu_second"
) -> float | None:
    """Mean of a field over a cell's runs, or None when the cell has none."""
    runs = cells.get((source, engine, mode))
    return st.fmean(r[key] for r in runs) if runs else None


def peak_mib(cells: dict, source: str, engine: str) -> float | None:
    """Largest container memory peak over every run of a source, clean and malformed."""
    runs = cells.get((source, engine, "clean"), []) + cells.get(
        (source, engine, "malformed"), []
    )
    return max(r["peak_cgroup_memory_bytes"] for r in runs) / 2**20 if runs else None


def cores_used(cells: dict, source: str, engine: str) -> float:
    """Mean cores used of the pinned set over the clean runs."""
    return st.fmean(
        r["cpu_seconds"] / r["window_seconds"] for r in cells[(source, engine, "clean")]
    )


def compact(value: float) -> str:
    """12502 -> 12.5K, 622 -> 622. One style per axis, K for thousands."""
    if value >= 10_000:
        return f"{value / 1000:.1f}K".replace(".0K", "K")
    if value >= 1000:
        return f"{value / 1000:.1f}K"
    return f"{value:.0f}"


def rng(values: list[float], digits: int = 1) -> str:
    """Format a min-max range."""
    lo, hi = min(values), max(values)
    return f"{lo:.{digits}f}-{hi:.{digits}f}"


# --------------------------------------------------------------------------
# Drawing
# --------------------------------------------------------------------------


def setup() -> None:
    """Fixed rendering settings so the same data gives the same bytes."""
    plt.rcParams.update(
        {
            "svg.hashsalt": "dfe-transform-elastic-bench",
            "svg.fonttype": "path",
            "font.family": "DejaVu Sans",
            "path.simplify": False,
            "figure.dpi": 72,
            "savefig.dpi": 72,
        }
    )


def rounded_bar(
    ax: Any, x_pt: float, w_pt: float, value: float, colour: str, hatch: str | None
) -> float:
    """Draw a column with a rounded cap and a square baseline. Returns the cap's y in points."""
    x0, y0 = ax.transData.transform((x_pt, 0.0))
    _, y1 = ax.transData.transform((x_pt, value))
    x1 = x0 + w_pt
    sign = 1.0 if y1 >= y0 else -1.0
    r = min(RADIUS_PT, abs(y1 - y0) / 2, w_pt / 2)
    verts = [
        (x0, y0),
        (x0, y1 - sign * r),
        (x0, y1),
        (x0 + r, y1),
        (x1 - r, y1),
        (x1, y1),
        (x1, y1 - sign * r),
        (x1, y0),
        (x0, y0),
    ]
    codes = [
        MplPath.MOVETO,
        MplPath.LINETO,
        MplPath.CURVE3,
        MplPath.CURVE3,
        MplPath.LINETO,
        MplPath.CURVE3,
        MplPath.CURVE3,
        MplPath.LINETO,
        MplPath.CLOSEPOLY,
    ]
    patch = PathPatch(
        MplPath(verts, codes),
        facecolor=colour,
        edgecolor="none",
        linewidth=0,
        transform=None,
        zorder=3,
    )
    ax.add_patch(patch)
    if hatch:
        # Hatch ink is the surface colour, so the bar reads as the same grey with a light weave.
        hp = PathPatch(
            MplPath(verts, codes),
            facecolor="none",
            edgecolor=SURFACE,
            hatch=hatch,
            linewidth=0,
            transform=None,
            zorder=4,
        )
        ax.add_patch(hp)
    return y1


def frame(fig_title: str, subtitle: str) -> tuple[Any, Any]:
    """A fixed-layout figure with the claim as its title and the units as its subtitle."""
    fig = plt.figure(figsize=(WIDTH_PT / 72, HEIGHT_PT / 72), facecolor=SURFACE)
    fig.text(
        24 / WIDTH_PT,
        1 - 26 / HEIGHT_PT,
        fig_title,
        color=NAVY,
        fontsize=11.5,
        fontweight="bold",
        va="center",
    )
    fig.text(
        24 / WIDTH_PT,
        1 - 46 / HEIGHT_PT,
        subtitle,
        color=MUTED,
        fontsize=8.5,
        va="center",
    )
    ax = fig.add_axes(
        (
            58 / WIDTH_PT,
            82 / HEIGHT_PT,
            (WIDTH_PT - 58 - 24) / WIDTH_PT,
            (HEIGHT_PT - 82 - 96) / HEIGHT_PT,
        )
    )
    ax.set_facecolor(SURFACE)
    for side in ("top", "right", "left"):
        ax.spines[side].set_visible(False)
    ax.spines["bottom"].set_color(GRID)
    ax.spines["bottom"].set_linewidth(1.0)
    ax.tick_params(axis="both", length=0, colors=MUTED, labelsize=8.5)
    ax.grid(axis="y", color=GRID, linewidth=0.75, zorder=0)
    ax.set_axisbelow(True)
    return fig, ax


def legend(fig: Any, engines: tuple[str, ...], labels: dict[str, str]) -> None:
    """A legend row under the subtitle. Identity is never colour alone: the hatch is in the key."""
    handles = [
        Patch(
            facecolor=COLOUR[e],
            edgecolor=SURFACE if e in HATCH else "none",
            hatch=HATCH.get(e),
            linewidth=0,
            label=labels[e],
        )
        for e in engines
    ]
    leg = fig.legend(
        handles=handles,
        loc="upper left",
        bbox_to_anchor=(24 / WIDTH_PT, 1 - 60 / HEIGHT_PT),
        ncol=len(handles),
        frameon=False,
        fontsize=8.5,
        handlelength=1.2,
        handleheight=1.0,
        columnspacing=1.6,
        handletextpad=0.6,
        borderpad=0,
    )
    for text in leg.get_texts():
        text.set_color(NAVY)


def grouped(
    path: Path,
    title: str,
    subtitle: str,
    engines: tuple[str, ...],
    labels: dict[str, str],
    value: Callable[[str, str], float | None],
    ymax: float,
    ticks: list[float],
    fmt: Callable[[float], str],
    tick_fmt: Callable[[float], str],
    ymin: float = 0.0,
) -> None:
    """Grouped columns, one group per source, one column per engine, a value on each cap."""
    fig, ax = frame(title, subtitle)
    legend(fig, engines, labels)
    ax.set_ylim(ymin, ymax)
    ax.set_yticks(ticks)
    ax.set_yticklabels([tick_fmt(t) for t in ticks])
    n = len(SOURCES)
    ax.set_xlim(0, n)
    if ymin < 0:
        ax.axhline(0, color=MUTED, linewidth=0.75, zorder=2)
    ax.set_xticks([i + 0.5 for i in range(n)])
    ax.set_xticklabels(SOURCES, color=NAVY, fontsize=9)
    ax.tick_params(axis="x", pad=8)
    # Pixel widths: one slot per group, bars sit side by side with a surface gap between them.
    left, right = ax.transData.transform((0, 0))[0], ax.transData.transform((n, 0))[0]
    slot = (right - left) / n
    k = len(engines)
    bar = min(BAR_PT, (slot * 0.84 - GAP_PT * (k - 1)) / k)
    span = bar * k + GAP_PT * (k - 1)
    for gi, source in enumerate(SOURCES):
        start = left + gi * slot + (slot - span) / 2
        for ei, engine in enumerate(engines):
            v = value(source, engine)
            if v is None:
                continue
            x_pt = start + ei * (bar + GAP_PT)
            x_data = ax.transData.inverted().transform((x_pt, 0))[0]
            top = rounded_bar(ax, x_data, bar, v, COLOUR[engine], HATCH.get(engine))
            ax.annotate(
                fmt(v),
                xy=(x_pt + bar / 2, top),
                xycoords="figure pixels",
                xytext=(0, 3.5 if v >= 0 else -3.5),
                textcoords="offset points",
                ha="center",
                va="bottom" if v >= 0 else "top",
                color=NAVY,
                fontsize=7.5,
            )
    fig.savefig(
        path,
        format="svg",
        metadata={"Date": None, "Creator": "scripts/bench/charts.py", "Title": title},
    )
    plt.close(fig)


def chart_cpu(cells: dict, out: Path) -> None:
    """Records per CPU-second, clean rows, by source and engine."""
    ratios = [
        mean(cells, s, "transform-elastic", "clean") / mean(cells, s, "vector", "clean")
        for s in SOURCES
    ]
    top = max(
        mean(cells, s, e, "clean")
        for s in SOURCES
        for e in ENGINES
        if mean(cells, s, e, "clean")
    )
    grouped(
        out / "records-per-cpu-second.svg",
        f"dfe-transform-elastic does {rng(ratios)}x Vector's records per CPU-second",
        "Records per CPU-second, clean rows, mean of three or more runs, four pinned cores. Higher is better.",
        ENGINES,
        LABEL,
        lambda s, e: mean(cells, s, e, "clean"),
        ymax=((top * 1.12) // 5000 + 1) * 5000,
        ticks=[i * 5000 for i in range(int((top * 1.12) // 5000 + 1) + 1)],
        fmt=compact,
        tick_fmt=compact,
    )


def chart_memory(cells: dict, out: Path) -> None:
    """Peak memory by source and engine. Elasticsearch's two modes share one container."""
    engines = ("transform-elastic", "vector", "es-bulk")
    labels = {**LABEL, "es-bulk": "Elasticsearch node (both modes)"}
    mems = {e: [peak_mib(cells, s, e) for s in SOURCES] for e in engines}
    lo = {e: min(v for v in mems[e] if v) for e in engines}
    hi = {e: max(v for v in mems[e] if v) for e in engines}
    top = max(hi.values())
    grouped(
        out / "peak-memory.svg",
        (
            f"dfe-transform-elastic peaks at {lo['transform-elastic']:.0f}-{hi['transform-elastic']:.0f} MiB, "
            f"Elasticsearch at {lo['es-bulk'] / 1024:.1f}-{hi['es-bulk'] / 1024:.1f} GiB"
        ),
        "Peak container memory in MiB over every run of a source. Elasticsearch runs a fixed 4 GiB heap. Lower is better.",
        engines,
        labels,
        lambda s, e: peak_mib(cells, s, e),
        ymax=((top * 1.1) // 2000 + 1) * 2000,
        ticks=[i * 2000 for i in range(int((top * 1.1) // 2000 + 1) + 1)],
        fmt=lambda v: f"{v:,.0f}",
        tick_fmt=lambda v: f"{v:,.0f}",
    )


def chart_malformed(cells: dict, out: Path) -> None:
    """How much faster each engine runs on the corrupted rows, as a percentage of its clean rate."""

    def gain(s: str, e: str) -> float | None:
        c, m = mean(cells, s, e, "clean"), mean(cells, s, e, "malformed")
        return (m / c - 1) * 100 if c and m else None

    vals = [g for s in SOURCES for e in ENGINES if (g := gain(s, e)) is not None]
    lo, hi = min(vals), max(vals)
    if lo >= 0:
        title = (
            f"Every engine runs faster on the corrupted rows, by {lo:.0f}% to {hi:.0f}%"
        )
    else:
        title = f"Corrupted rows change the rate by {lo:+.0f}% to {hi:+.0f}%, engine by engine"
    ymin = (lo * 1.3 // 5) * 5 if lo < 0 else 0.0
    ymax = (hi * 1.15 // 5 + 1) * 5
    grouped(
        out / "clean-vs-malformed.svg",
        title,
        "Change in records per CPU-second with 20% of rows corrupted, against the same engine on clean rows.",
        ENGINES,
        LABEL,
        gain,
        ymax=ymax,
        ticks=[ymin + i * 5 for i in range(int((ymax - ymin) // 5) + 1)],
        fmt=lambda v: f"{v:+.0f}%",
        tick_fmt=lambda v: f"{v:.0f}%",
        ymin=ymin,
    )


# --------------------------------------------------------------------------
# Detail layer: per-source charts and statistics over the timed runs.
# The statistics are per run. The rig keeps no per-interval samples, so there
# are no percentiles here and none are computed from three runs.
# --------------------------------------------------------------------------


def values(
    cells: dict, source: str, engine: str, modes: tuple[str, ...], key: str
) -> list[float]:
    """One value per timed run, in run order, over the given modes."""
    out: list[float] = []
    for mode in modes:
        for run in cells.get((source, engine, mode), []):
            v = run[key]
            out.append(v / 2**20 if key == "peak_cgroup_memory_bytes" else v)
    return out


def summary(vals: list[float]) -> dict[str, float]:
    """n, mean, min, max and the sample standard deviation."""
    return {
        "n": len(vals),
        "mean": st.fmean(vals),
        "min": min(vals),
        "max": max(vals),
        "sd": st.stdev(vals) if len(vals) > 1 else 0.0,
    }


def whisker(ax: Any, x_pt: float, lo: float, hi: float) -> float:
    """A min-max whisker with caps at a bar's centre. Returns the top's y in points."""
    _, y0 = ax.transData.transform((0, lo))
    _, y1 = ax.transData.transform((0, hi))
    for xs, ys in (
        ([x_pt, x_pt], [y0, y1]),
        ([x_pt - 3, x_pt + 3], [y0, y0]),
        ([x_pt - 3, x_pt + 3], [y1, y1]),
    ):
        ax.add_line(
            Line2D(
                xs,
                ys,
                transform=None,
                color=NAVY,
                linewidth=1.25,
                solid_capstyle="butt",
                zorder=6,
            )
        )
    return y1


def panel(
    fig: Any,
    rect: tuple[float, float, float, float],
    heading: str,
    items: list[tuple[str, dict[str, float]]],
    ymax: float,
    ticks: list[float],
    tick_fmt: Callable[[float], str],
    label_fmt: Callable[[float], str],
) -> None:
    """One bar panel: a mean bar and a min-max whisker per engine, n under each bar."""
    x, y, w, h = rect
    ax = fig.add_axes((x / WIDTH_PT, y / HEIGHT_PT, w / WIDTH_PT, h / HEIGHT_PT))
    ax.set_facecolor(SURFACE)
    for side in ("top", "right", "left"):
        ax.spines[side].set_visible(False)
    ax.spines["bottom"].set_color(GRID)
    ax.tick_params(axis="both", length=0, colors=MUTED, labelsize=8)
    ax.grid(axis="y", color=GRID, linewidth=0.75, zorder=0)
    ax.set_axisbelow(True)
    ax.set_ylim(0, ymax)
    ax.set_yticks(ticks)
    ax.set_yticklabels([tick_fmt(t) for t in ticks])
    k = len(items)
    ax.set_xlim(0, k)
    ax.set_xticks([i + 0.5 for i in range(k)])
    ax.set_xticklabels([f"n = {int(s['n'])}" for _, s in items], color=NAVY, fontsize=8)
    ax.tick_params(axis="x", pad=6)
    ax.set_title(heading, loc="left", fontsize=8.5, color=MUTED, pad=10)
    left = ax.transData.transform((0, 0))[0]
    slot = (ax.transData.transform((k, 0))[0] - left) / k
    bar = min(BAR_PT, slot * 0.6)
    for i, (engine, s) in enumerate(items):
        x_pt = left + i * slot + (slot - bar) / 2
        x_data = ax.transData.inverted().transform((x_pt, 0))[0]
        rounded_bar(ax, x_data, bar, s["mean"], COLOUR[engine], HATCH.get(engine))
        top = whisker(ax, x_pt + bar / 2, s["min"], s["max"])
        ax.annotate(
            label_fmt(s["mean"]),
            xy=(x_pt + bar / 2, top),
            xycoords="figure pixels",
            xytext=(0, 3.5),
            textcoords="offset points",
            ha="center",
            va="bottom",
            color=NAVY,
            fontsize=7.5,
        )


def nice_max(top: float, step: float) -> tuple[float, list[float]]:
    """A y-limit just over `top` on a clean step, and the ticks up to it."""
    ymax = (top * 1.12 // step + 1) * step
    return ymax, [i * step for i in range(int(ymax // step) + 1)]


def chart_detail(cells: dict, out: Path, source: str) -> None:
    """Two panels for one source: CPU (clean rows) and peak memory (every run), mean with range."""
    cpu = [
        (e, summary(values(cells, source, e, ("clean",), "rec_per_cpu_second")))
        for e in ENGINES
    ]
    mem_engines = ("transform-elastic", "vector", "es-bulk")
    mem = [
        (
            e,
            summary(
                values(
                    cells, source, e, ("clean", "malformed"), "peak_cgroup_memory_bytes"
                )
            ),
        )
        for e in mem_engines
    ]
    te, vec = cpu[0][1]["mean"], cpu[1][1]["mean"]
    bulk, sim = cpu[2][1]["mean"], cpu[3][1]["mean"]
    title = f"{source}: {te / vec:.1f}x Vector, {te / bulk:.1f}x Elasticsearch _bulk, {te / sim:.1f}x _simulate"
    sub = "Bar is the mean of the timed runs, the whisker is min to max. Four pinned cores. Warm-up runs excluded."
    fig = plt.figure(figsize=(WIDTH_PT / 72, HEIGHT_PT / 72), facecolor=SURFACE)
    fig.text(
        24 / WIDTH_PT,
        1 - 26 / HEIGHT_PT,
        title,
        color=NAVY,
        fontsize=11.5,
        fontweight="bold",
        va="center",
    )
    fig.text(
        24 / WIDTH_PT, 1 - 46 / HEIGHT_PT, sub, color=MUTED, fontsize=8.5, va="center"
    )
    legend(fig, ENGINES, {**LABEL, "es-bulk": "Elasticsearch _bulk"})
    top_cpu = max(s["max"] for _, s in cpu)
    ymax, ticks = nice_max(top_cpu, 5000 if top_cpu > 10_000 else 2000)
    panel(
        fig,
        (52, 70, 262, 190),
        "Records per CPU-second, clean. Higher is better.",
        cpu,
        ymax,
        ticks,
        compact,
        compact,
    )
    top_mem = max(s["max"] for _, s in mem)
    ymax, ticks = nice_max(top_mem, 2000)
    panel(
        fig,
        (380, 70, 236, 190),
        "Peak memory, MiB, all runs. Lower is better.",
        mem,
        ymax,
        ticks,
        lambda v: f"{v:,.0f}",
        lambda v: f"{v:,.0f}",
    )
    fig.savefig(
        out / f"detail-{source}.svg",
        format="svg",
        metadata={"Date": None, "Creator": "scripts/bench/charts.py", "Title": title},
    )
    plt.close(fig)


def stats_tables(cells: dict) -> None:
    """Per-source statistics tables: n, mean, min, max, standard deviation, mean ratio."""
    names = {
        "transform-elastic": "dfe-transform-elastic",
        "vector": "Vector VRL",
        "es-bulk": "Elasticsearch `_bulk`",
        "es-simulate": "Elasticsearch `_simulate`",
    }
    for source in SOURCES:
        print(f"\n### {source}\n")
        for mode in ("clean", "malformed"):
            print(f"Records per CPU-second, {mode} rows, per run:\n")
            print(
                "| engine | n | mean | min | max | sd | dfe-transform-elastic mean over it |"
            )
            print("|---|---|---|---|---|---|---|")
            base = mean(cells, source, "transform-elastic", mode)
            for e in ENGINES:
                s = summary(values(cells, source, e, (mode,), "rec_per_cpu_second"))
                ratio = "-" if e == "transform-elastic" else f"{base / s['mean']:.2f}x"
                print(
                    f"| {names[e]} | {int(s['n'])} | {s['mean']:,.0f} | {s['min']:,.0f} | {s['max']:,.0f} "
                    f"| {s['sd']:,.0f} | {ratio} |"
                )
            print()
        print("Peak memory in MiB, per run, clean and malformed runs together:\n")
        print("| engine | n | mean | min | max | sd |")
        print("|---|---|---|---|---|---|")
        for e in ("transform-elastic", "vector", "es-bulk"):
            s = summary(
                values(
                    cells, source, e, ("clean", "malformed"), "peak_cgroup_memory_bytes"
                )
            )
            label = names[e] if e != "es-bulk" else "Elasticsearch (both modes)"
            print(
                f"| {label} | {int(s['n'])} | {s['mean']:,.0f} | {s['min']:,.0f} | {s['max']:,.0f} | {s['sd']:,.0f} |"
            )


# --------------------------------------------------------------------------
# Figures. Every ratio in the doc reproduces from the means printed here.
# --------------------------------------------------------------------------


def figures(cells: dict) -> None:
    """Print the means, ratios and the derived figures the doc quotes."""
    counts = {k: len(v) for k, v in cells.items()}
    print("runs per cell:", sorted(set(counts.values())), "cells", len(counts))
    print("\nrecords per CPU-second, clean, mean (runs)")
    for e in ENGINES:
        print(
            f"{e:18s}",
            " | ".join(
                f"{mean(cells, s, e, 'clean'):,.0f} ({counts[(s, e, 'clean')]})"
                for s in SOURCES
            ),
        )
    print("\nrecords per CPU-second, malformed, mean (runs)")
    for e in ENGINES:
        print(
            f"{e:18s}",
            " | ".join(
                f"{mean(cells, s, e, 'malformed'):,.0f} ({counts[(s, e, 'malformed')]})"
                if mean(cells, s, e, "malformed")
                else "missing"
                for s in SOURCES
            ),
        )
    pairs = (
        ("te over vector", "transform-elastic", "vector"),
        ("vector over _bulk", "vector", "es-bulk"),
        ("te over _bulk", "transform-elastic", "es-bulk"),
        ("vector over _simulate", "vector", "es-simulate"),
        ("te over _simulate", "transform-elastic", "es-simulate"),
    )
    for mode in ("clean", "malformed"):
        print(f"\nCPU ratios, {mode}")
        for name, a, b in pairs:
            v = [
                mean(cells, s, a, mode) / mean(cells, s, b, mode)
                for s in SOURCES
                if mean(cells, s, a, mode) and mean(cells, s, b, mode)
            ]
            print(f"{name:22s}", " ".join(f"{x:.2f}" for x in v), "range", rng(v, 2))
    sim = [
        mean(cells, s, "transform-elastic", "clean")
        / mean(cells, s, "es-simulate", "clean")
        for s in SOURCES
    ]
    print(
        f"\nyear factor {YEAR_FACTOR}: te over _simulate x factor",
        rng([x * YEAR_FACTOR for x in sim], 2),
    )

    print("\npeak memory MiB (max over clean and malformed runs), cores used (clean)")
    for e in ENGINES[:3]:
        print(
            f"{e:18s}",
            [round(peak_mib(cells, s, e)) for s in SOURCES],
            "cores",
            rng([cores_used(cells, s, e) for s in SOURCES], 2),
        )
    print(
        f"{'es-simulate':18s} cores",
        rng([cores_used(cells, s, "es-simulate") for s in SOURCES], 2),
    )

    def per_rate(s: str, e: str) -> float:
        return peak_mib(cells, s, e) / (
            mean(cells, s, e, "clean") * cores_used(cells, s, e)
        )

    print("\nmemory for the same throughput (peak MiB per record a second), ratio")
    for name, a, b in (
        ("_simulate over te", "es-simulate", "transform-elastic"),
        ("_bulk over te", "es-bulk", "transform-elastic"),
        ("vector over te", "vector", "transform-elastic"),
        ("_simulate over vector", "es-simulate", "vector"),
        ("_bulk over vector", "es-bulk", "vector"),
    ):
        v = [per_rate(s, a) / per_rate(s, b) for s in SOURCES]
        print(f"{name:22s}", " ".join(f"{x:.2f}" for x in v), "range", rng(v))

    print("\nper pinned core, clean (rate / 4)")
    for e in ENGINES:
        print(
            f"{e:18s}",
            " | ".join(
                f"{mean(cells, s, e, 'clean', 'rate_per_core'):,.0f}" for s in SOURCES
            ),
        )

    allruns = [r for v in cells.values() for r in v]
    before = [r["host_before"]["loadavg"][0] for r in allruns]
    after = [r["host_after"]["loadavg"][0] for r in allruns]
    others = [r["others_busy_pct"] for r in allruns if "others_busy_pct" in r]
    print(
        f"\nload1 before {min(before):.2f}-{max(before):.2f}, after {min(after):.2f}-{max(after):.2f}"
    )
    if others:
        print(f"other cores busy {min(others):.0f}-{max(others):.0f}%")
    spreads = sorted(
        (
            (
                max(r["rec_per_cpu_second"] for r in v)
                / min(r["rec_per_cpu_second"] for r in v)
                - 1,
                k,
            )
            for k, v in cells.items()
        ),
        reverse=True,
    )
    print(
        "widest cell spread (max over min):",
        [(f"{x * 100:.0f}%", *k) for x, k in spreads[:4]],
    )
    within = {t: sum(1 for x, _ in spreads if x <= t / 100) for t in (5, 10, 25)}
    print(
        f"cells within 5/10/25% spread: {within} of {len(spreads)}; median {st.median(x for x, _ in spreads) * 100:.0f}%"
    )


def tables(cells: dict) -> None:
    """Print the doc's means, ratio and memory tables as markdown."""

    def cell(s: str, e: str) -> str:
        c, m = mean(cells, s, e, "clean"), mean(cells, s, e, "malformed")
        return f"{c:,.0f} ({m:,.0f})" if c and m else f"{c:,.0f}"

    names = {
        "transform-elastic": "dfe-transform-elastic",
        "vector": "Vector VRL",
        "es-bulk": "Elasticsearch `_bulk`",
        "es-simulate": "Elasticsearch `_simulate`",
    }
    print("Records per CPU-second, clean (malformed in brackets):\n")
    print("| engine | " + " | ".join(SOURCES) + " |")
    print("|---|" + "---|" * len(SOURCES))
    for e in ("transform-elastic", "vector", "es-bulk", "es-simulate"):
        print(f"| {names[e]} | " + " | ".join(cell(s, e) for s in SOURCES) + " |")

    print("\nRatios of those means, clean (malformed in brackets):\n")
    print("| | " + " | ".join(SOURCES) + " |")
    print("|---|" + "---|" * len(SOURCES))
    for label, a, b in (
        ("dfe-transform-elastic over Vector", "transform-elastic", "vector"),
        ("Vector over Elasticsearch `_bulk`", "vector", "es-bulk"),
        (
            "dfe-transform-elastic over Elasticsearch `_bulk`",
            "transform-elastic",
            "es-bulk",
        ),
        (
            "dfe-transform-elastic over Elasticsearch `_simulate`",
            "transform-elastic",
            "es-simulate",
        ),
        ("Vector over Elasticsearch `_simulate`", "vector", "es-simulate"),
    ):
        row = []
        for s in SOURCES:
            c = mean(cells, s, a, "clean") / mean(cells, s, b, "clean")
            ma, mb = mean(cells, s, a, "malformed"), mean(cells, s, b, "malformed")
            row.append(f"{c:.2f}x ({ma / mb:.2f}x)" if ma and mb else f"{c:.2f}x")
        print(f"| {label} | " + " | ".join(row) + " |")

    print(
        "\nPeak memory in MiB, the largest container figure over every run of a source:\n"
    )
    print("| engine | " + " | ".join(SOURCES) + " | cores used |")
    print("|---|" + "---|" * (len(SOURCES) + 1))
    for e, used in (
        (
            "transform-elastic",
            rng([cores_used(cells, s, "transform-elastic") for s in SOURCES], 2),
        ),
        ("vector", rng([cores_used(cells, s, "vector") for s in SOURCES], 2)),
        (
            "es-bulk",
            f"{rng([cores_used(cells, s, 'es-bulk') for s in SOURCES], 2)} `_bulk`, {rng([cores_used(cells, s, 'es-simulate') for s in SOURCES], 2)} `_simulate`",
        ),
    ):
        label = names[e] if e != "es-bulk" else "Elasticsearch (both modes)"
        print(
            f"| {label} | "
            + " | ".join(f"{peak_mib(cells, s, e):,.0f}" for s in SOURCES)
            + f" | {used} |"
        )


def main() -> int:
    """CLI entry."""
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    sub = parser.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("charts", help="write the SVG charts")
    p.add_argument("--results", type=Path, required=True)
    p.add_argument("--out", type=Path, required=True)
    p = sub.add_parser("figures", help="print the means and derived ratios")
    p.add_argument("--results", type=Path, required=True)
    p = sub.add_parser("tables", help="print the doc's markdown tables")
    p.add_argument("--results", type=Path, required=True)
    p = sub.add_parser("stats", help="print the per-source statistics tables")
    p.add_argument("--results", type=Path, required=True)
    args = parser.parse_args()
    cells = load(args.results)
    if not cells:
        print(f"no usable runs under {args.results}", file=sys.stderr)
        return 1
    if args.cmd == "figures":
        figures(cells)
        return 0
    if args.cmd == "tables":
        tables(cells)
        return 0
    if args.cmd == "stats":
        stats_tables(cells)
        return 0
    setup()
    args.out.mkdir(parents=True, exist_ok=True)
    chart_cpu(cells, args.out)
    chart_memory(cells, args.out)
    chart_malformed(cells, args.out)
    for source in SOURCES:
        chart_detail(cells, args.out, source)
    return 0


if __name__ == "__main__":
    sys.exit(main())
