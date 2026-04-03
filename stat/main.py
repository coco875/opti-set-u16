"""
Benchmark Visualization Script
================================
Reads CSV benchmark data with columns:
  Scenario name, Type name, maximum capacity, fill, data, seed, time

Produces:
  1. Global avg time per SetInt implementation (bar chart)
  2. Per-scenario avg time per implementation (one chart per scenario)
  3. Per-scenario avg time per implementation × maximum capacity (one chart per scenario)

Usage:
  python benchmark_graphs.py [input_file.csv]

If no file is given the script looks for "benchmark.csv" in the current directory.
The output directory defaults to "./benchmark_charts/".
"""

import csv
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np

# ──────────────────────────────────────────────
# Configuration
# ──────────────────────────────────────────────
INPUT_FILE = sys.argv[1] if len(sys.argv) > 1 else "output.csv"
OUTPUT_DIR = Path("benchmark_charts")
OUTPUT_DIR.mkdir(exist_ok=True)

# Color palette – one colour per implementation type
PALETTE = [
    "#4FC3F7",
    "#FF8A65",
    "#81C784",
    "#CE93D8",
    "#FFD54F",
    "#4DB6AC",
    "#F48FB1",
    "#90A4AE",
]

# ──────────────────────────────────────────────
# Styling
# ──────────────────────────────────────────────
DARK_BG = "#0E1117"
PANEL_BG = "#161B22"
TEXT = "#E6EDF3"
GRID = "#21262D"
ACCENT = "#58A6FF"


def apply_dark_style(fig, ax):
    fig.patch.set_facecolor(DARK_BG)
    ax.set_facecolor(PANEL_BG)
    ax.tick_params(colors=TEXT, labelsize=9)
    ax.xaxis.label.set_color(TEXT)
    ax.yaxis.label.set_color(TEXT)
    ax.title.set_color(TEXT)
    for spine in ax.spines.values():
        spine.set_edgecolor(GRID)
    ax.grid(axis="y", color=GRID, linewidth=0.8, linestyle="--", alpha=0.7)
    ax.set_axisbelow(True)


def save(fig, name: str):
    path = OUTPUT_DIR / f"{name}.png"
    fig.savefig(path, dpi=150, bbox_inches="tight", facecolor=fig.get_facecolor())
    print(f"  ✓  {path}")


def impl_colors(impl_list):
    """Assign a stable colour to each implementation name."""
    return {impl: PALETTE[i % len(PALETTE)] for i, impl in enumerate(sorted(impl_list))}


def avg(values):
    return sum(values) / len(values) if values else 0.0


def group_avg(data, key_fn):
    """Return {key: avg(time)} for all rows in data, grouped by key_fn."""
    buckets = defaultdict(list)
    for r in data:
        buckets[key_fn(r)].append(r["time"])
    return {k: avg(v) for k, v in buckets.items()}


def bar_chart(ax, categories, values, colors, title, xlabel, ylabel, rotate=False):
    x = np.arange(len(categories))
    bars = ax.bar(x, values, color=colors, width=0.6, edgecolor=DARK_BG, linewidth=0.8)
    ax.set_title(title, fontsize=12, fontweight="bold", pad=10)
    ax.set_xlabel(xlabel, fontsize=9, labelpad=6)
    ax.set_ylabel(ylabel, fontsize=9, labelpad=6)
    ax.set_xticks(x)
    ax.set_xticklabels(
        categories,
        rotation=30 if rotate else 0,
        ha="right" if rotate else "center",
        fontsize=8,
    )
    for bar in bars:
        h = bar.get_height()
        label = f"{h:,.0f}" if h >= 10 else f"{h:.1f}"
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            h * 1.01,
            label,
            ha="center",
            va="bottom",
            fontsize=7,
            color=TEXT,
        )
    ax.yaxis.set_major_formatter(mticker.FuncFormatter(lambda v, _: f"{v:,.0f}"))


# ──────────────────────────────────────────────
# Load data with csv module
# ──────────────────────────────────────────────
rows = []

try:
    with open(INPUT_FILE, newline="", encoding="utf-8") as f:
        reader = csv.reader(f, skipinitialspace=True)
        for line in reader:
            if len(line) < 7:
                continue  # skip malformed / header lines
            scenario, impl, max_cap, fill, data, seed, time_val = line[:7]
            try:
                rows.append(
                    {
                        "scenario": scenario.strip(),
                        "impl": impl.strip(),
                        "max_capacity": int(max_cap.strip()),
                        "fill": int(fill.strip()),
                        "data": int(data.strip()),
                        "seed": int(seed.strip()),
                        "time": float(time_val.strip()),
                    }
                )
            except ValueError:
                continue  # skip header or non-numeric rows
except FileNotFoundError:
    print(f"[ERROR] File not found: {INPUT_FILE}")
    print(
        "  Provide a CSV with columns: scenario, impl, max_capacity, fill, data, seed, time"
    )
    sys.exit(1)

if not rows:
    print("[ERROR] No valid data rows found in the file.")
    sys.exit(1)

# Derive sorted sets of unique values
scenarios = sorted({r["scenario"] for r in rows})
impls = sorted({r["impl"] for r in rows})
capacities = sorted({r["max_capacity"] for r in rows})
colors = impl_colors(impls)

print(
    f"\nLoaded {len(rows):,} rows  |  {len(scenarios)} scenarios  |  {len(impls)} implementations\n"
)
print("Generating charts …")

# ══════════════════════════════════════════════
# GRAPH 1 – Global avg time per implementation
# ══════════════════════════════════════════════
global_avg = group_avg(rows, lambda r: r["impl"])

fig, ax = plt.subplots(figsize=(max(6, len(impls) * 1.4), 5))
apply_dark_style(fig, ax)
bar_chart(
    ax,
    categories=impls,
    values=[global_avg.get(i, 0) for i in impls],
    colors=[colors[i] for i in impls],
    title="Global Average Time per Implementation",
    xlabel="Implementation",
    ylabel="Avg Time (µs / ms)",
    rotate=len(impls) > 4,
)
fig.tight_layout()
save(fig, "1_global_avg_time")
plt.close(fig)

# ══════════════════════════════════════════════
# GRAPH 2 – Per-scenario avg time per implementation
# ══════════════════════════════════════════════
ncols = min(len(scenarios), 3)
nrows = (len(scenarios) + ncols - 1) // ncols

fig, axes = plt.subplots(nrows, ncols, figsize=(ncols * 5, nrows * 4 + 0.6))
fig.patch.set_facecolor(DARK_BG)
fig.suptitle(
    "Avg Time per Implementation — by Scenario",
    color=TEXT,
    fontsize=14,
    fontweight="bold",
    y=1.01,
)

axes_flat = np.array(axes).flatten() if nrows * ncols > 1 else [axes]

for idx, scenario in enumerate(scenarios):
    ax = axes_flat[idx]
    apply_dark_style(fig, ax)

    sub = [r for r in rows if r["scenario"] == scenario]
    scen_avg = group_avg(sub, lambda r: r["impl"])
    present = [i for i in impls if i in scen_avg]

    bar_chart(
        ax,
        categories=present,
        values=[scen_avg[i] for i in present],
        colors=[colors[i] for i in present],
        title=scenario,
        xlabel="Implementation",
        ylabel="Avg Time",
        rotate=len(present) > 3,
    )

for idx in range(len(scenarios), len(axes_flat)):
    axes_flat[idx].set_visible(False)

fig.tight_layout()
save(fig, "2_per_scenario_avg_time")
plt.close(fig)

# ══════════════════════════════════════════════
# GRAPH 3 – Per-scenario, per-impl, per max_capacity
# ══════════════════════════════════════════════
for scenario in scenarios:
    sub = [r for r in rows if r["scenario"] == scenario]
    impl_list = sorted({r["impl"] for r in sub})
    caps = sorted({r["max_capacity"] for r in sub})

    fig, ax = plt.subplots(figsize=(max(7, len(caps) * 1.3 + 2), 5))
    apply_dark_style(fig, ax)

    x = np.arange(len(caps))
    n = len(impl_list)
    width = 0.75 / max(n, 1)
    offsets = np.linspace(-(n - 1) / 2, (n - 1) / 2, n) * width

    for i, impl in enumerate(impl_list):
        impl_rows = [r for r in sub if r["impl"] == impl]
        cap_avg = group_avg(impl_rows, lambda r: r["max_capacity"])

        ax.bar(
            x + offsets[i],
            [cap_avg.get(c, 0) for c in caps],
            width=width,
            color=colors[impl],
            edgecolor=DARK_BG,
            linewidth=0.6,
            label=impl,
        )

    ax.set_title(
        f"{scenario} — Avg Time per Implementation × Max Capacity",
        fontsize=11,
        fontweight="bold",
        color=TEXT,
        pad=10,
    )
    ax.set_xlabel("Maximum Capacity", fontsize=9, labelpad=6)
    ax.set_ylabel("Avg Time (µs / ms)", fontsize=9, labelpad=6)
    ax.set_xticks(x)
    ax.set_xticklabels([str(c) for c in caps], fontsize=8)
    ax.yaxis.set_major_formatter(mticker.FuncFormatter(lambda v, _: f"{v:,.0f}"))

    legend = ax.legend(
        title="Implementation",
        fontsize=8,
        title_fontsize=8,
        facecolor=PANEL_BG,
        edgecolor=GRID,
        labelcolor=TEXT,
    )
    legend.get_title().set_color(ACCENT)

    fig.tight_layout()
    safe_name = scenario.replace(" ", "_").replace("/", "-")
    save(fig, f"3_{safe_name}_capacity_breakdown")
    plt.close(fig)

print(f"\nAll charts saved to: {OUTPUT_DIR.resolve()}/\n")

