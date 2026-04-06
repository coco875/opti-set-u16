"""
Benchmark Visualization Script
===============================
Reads CSV benchmark data with columns:
  Scenario name, Type name, maximum capacity, fill, data, seed, time

Produces:
  1. Global avg time per SetInt implementation (bar chart)
  2. Per-scenario avg time per implementation (one chart per scenario)
  3. Per-scenario avg time per implementation x maximum capacity (one chart per scenario)

Usage:
  python stat/main.py [input_file.csv]

If no file is given the script looks for "output.csv" in the current directory.
The output directory defaults to "./benchmark_charts/".
"""

import csv
import sys
import warnings
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np
from PIL import Image
from scipy import stats

warnings.filterwarnings("ignore")

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


def save(fig, name: str, subdir: str = None):
    if subdir:
        target = OUTPUT_DIR / subdir
    else:
        target = OUTPUT_DIR
    target.mkdir(exist_ok=True, parents=True)
    path = target / f"{name}.png"
    fig.savefig(path, dpi=150, bbox_inches="tight", facecolor=fig.get_facecolor())
    print(f"  ✓  {path}")


def impl_colors(impl_list):
    """Assign a stable colour to each implementation name."""
    return {impl: PALETTE[i % len(PALETTE)] for i, impl in enumerate(sorted(impl_list))}


def avg(values):
    return sum(values) / len(values) if values else 0.0


def confidence_interval(values, confidence=0.95):
    """Return the half-width of the confidence interval."""
    if len(values) < 2:
        return 0.0
    return stats.t.interval(
        confidence, len(values) - 1, loc=np.mean(values), scale=stats.sem(values)
    )[1] - np.mean(values)


def group_stats(data, key_fn):
    """Return {key: (avg, ci_half_width)} for all rows grouped by key_fn."""
    buckets = defaultdict(list)
    for r in data:
        buckets[key_fn(r)].append(r["time"])
    return {k: (avg(v), confidence_interval(v)) for k, v in buckets.items()}


def bar_chart_with_ci(
    ax, categories, values, errors, colors, title, xlabel, ylabel, rotate=False, ylim=None
):
    x = np.arange(len(categories))
    bars = ax.bar(
        x, values, yerr=errors, color=colors, width=0.6, edgecolor=DARK_BG,
        linewidth=0.8, capsize=4, ecolor=TEXT, error_kw={"alpha": 0.6},
    )
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
    for i, bar in enumerate(bars):
        h = bar.get_height()
        err = errors[i]
        label = f"{h:,.0f}" if h >= 10 else f"{h:.1f}"
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            h + err + 1.01,
            label,
            ha="center",
            va="bottom",
            fontsize=7,
            color=TEXT,
        )
    ax.yaxis.set_major_formatter(mticker.FuncFormatter(lambda v, _: f"{v:,.0f}"))
    if ylim is not None:
        ax.set_ylim(0, ylim)


def draw_break_marks(ax_top, ax_bottom, d=0.012):
    kwargs = dict(transform=ax_top.transAxes, color=TEXT, clip_on=False, linewidth=1.2)
    ax_top.plot((-d, +d), (-d, +d), **kwargs)
    ax_top.plot((1 - d, 1 + d), (-d, +d), **kwargs)
    kwargs.update(transform=ax_bottom.transAxes)
    ax_bottom.plot((-d, +d), (1 - d, 1 + d), **kwargs)
    ax_bottom.plot((1 - d, 1 + d), (1 - d, 1 + d), **kwargs)


def bar_chart_broken_y(
    fig, categories, values, errors, colors, title, xlabel, ylabel, rotate=False
):
    max_val = max(v + e for v, e in zip(values, errors)) if values else 0
    sorted_vals = sorted(values)
    median_val = np.median(sorted_vals)
    low_top = median_val * 1.5
    high_bottom = max_val * 0.25

    if high_bottom <= low_top or len(values) <= 1:
        ax = fig.add_subplot(111)
        apply_dark_style(fig, ax)
        bar_chart_with_ci(
            ax, categories, values, errors, colors, title, xlabel, ylabel, rotate,
        )
        return

    gs = fig.add_gridspec(2, 1, height_ratios=[1, 3], hspace=0.06)
    ax_top = fig.add_subplot(gs[0])
    ax_bottom = fig.add_subplot(gs[1])

    apply_dark_style(fig, ax_top)
    apply_dark_style(fig, ax_bottom)

    x = np.arange(len(categories))
    ax_bottom.bar(
        x, values, yerr=errors, color=colors, width=0.6, edgecolor=DARK_BG,
        linewidth=0.8, capsize=4, ecolor=TEXT, error_kw={"alpha": 0.6},
    )
    ax_top.bar(
        x, values, yerr=errors, color=colors, width=0.6, edgecolor=DARK_BG,
        linewidth=0.8, capsize=4, ecolor=TEXT, error_kw={"alpha": 0.6},
    )

    ax_bottom.set_ylim(0, low_top)
    ax_top.set_ylim(high_bottom, max_val * 1.15 if max_val > 0 else low_top * 1.5)

    ax_bottom.set_xticks(x)
    ax_bottom.set_xticklabels(
        categories,
        rotation=30 if rotate else 0,
        ha="right" if rotate else "center",
        fontsize=8,
    )
    ax_top.set_xticks([])

    ax_bottom.set_xlabel(xlabel, fontsize=9, labelpad=6)
    ax_bottom.set_ylabel(ylabel, fontsize=9, labelpad=6)
    ax_top.set_ylabel(ylabel, fontsize=9, labelpad=6)

    ax_bottom.yaxis.set_major_formatter(mticker.FuncFormatter(lambda v, _: f"{v:,.0f}"))
    ax_top.yaxis.set_major_formatter(mticker.FuncFormatter(lambda v, _: f"{v:,.0f}"))

    ax_bottom.spines["top"].set_visible(False)
    ax_top.spines["bottom"].set_visible(False)

    draw_break_marks(ax_top, ax_bottom)

    for i, (h, err) in enumerate(zip(values, errors)):
        label = f"{h:,.0f}" if h >= 10 else f"{h:.1f}"
        if h <= low_top:
            ax_bottom.text(
                x[i], h + err + 1.01, label,
                ha="center", va="bottom", fontsize=7, color=TEXT,
            )
        else:
            ax_top.text(
                x[i], h + err + 1.01, label,
                ha="center", va="bottom", fontsize=7, color=TEXT,
            )

    ax_top.set_title(title, fontsize=12, fontweight="bold", pad=10)

# ──────────────────────────────────────────────
# Load data with csv module
# ──────────────────────────────────────────────
# Load data with csv module
# ──────────────────────────────────────────────
rows = []

try:
    with open(INPUT_FILE, newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f, skipinitialspace=True)
        for line in reader:
            try:
                rows.append(
                    {
                        "scenario": line["Scenario name"].strip(),
                        "impl": line["Type name"].strip(),
                        "max_capacity": int(line["maximum capacity"].strip()),
                        "fill": int(line["fill"].strip()),
                        "data": int(line["data"].strip()),
                        "seed": int(line["seed"].strip()),
                        "time": float(line["time"].strip()),
                    }
                )
            except (ValueError, KeyError):
                continue
except FileNotFoundError:
    print(f"[ERROR] File not found: {INPUT_FILE}")
    print(
        "  Provide a CSV with columns: Scenario name, Type name, maximum capacity, fill, data, seed, time"
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
global_stats = group_stats(rows, lambda r: r["impl"])

fig, ax = plt.subplots(figsize=(max(6, len(impls) * 1.4), 5))
apply_dark_style(fig, ax)
bar_chart_with_ci(
    ax,
    categories=impls,
    values=[global_stats[i][0] for i in impls],
    errors=[global_stats[i][1] for i in impls],
    colors=[colors[i] for i in impls],
    title="Global Average Time per Implementation",
    xlabel="Implementation",
    ylabel="Avg Time (cycles)",
    rotate=len(impls) > 4,
)
fig.tight_layout()
save(fig, "1_global_avg_time")
plt.close(fig)

fig = plt.figure(figsize=(max(6, len(impls) * 1.4), 6))
bar_chart_broken_y(
    fig,
    categories=impls,
    values=[global_stats[i][0] for i in impls],
    errors=[global_stats[i][1] for i in impls],
    colors=[colors[i] for i in impls],
    title="Global Average Time per Implementation (zoomed)",
    xlabel="Implementation",
    ylabel="Avg Time (cycles)",
    rotate=len(impls) > 4,
)
fig.tight_layout()
save(fig, "1_global_avg_time_zoomed")
plt.close(fig)

# ══════════════════════════════════════════════
# GRAPH 2 – Per-scenario avg time per implementation
# ══════════════════════════════════════════════
for scenario in scenarios:
    sub = [r for r in rows if r["scenario"] == scenario]
    scen_stats = group_stats(sub, lambda r: r["impl"])
    present = [i for i in impls if i in scen_stats]
    safe_name = scenario.replace(" ", "_").replace("/", "-")

    fig, ax = plt.subplots(figsize=(max(6, len(present) * 1.4), 5))
    apply_dark_style(fig, ax)
    bar_chart_with_ci(
        ax,
        categories=present,
        values=[scen_stats[i][0] for i in present],
        errors=[scen_stats[i][1] for i in present],
        colors=[colors[i] for i in present],
        title=scenario,
        xlabel="Implementation",
        ylabel="Avg Time (cycles)",
        rotate=len(present) > 3,
    )
    fig.tight_layout()
    save(fig, "1_avg_time", subdir=safe_name)
    plt.close(fig)

    fig = plt.figure(figsize=(max(6, len(present) * 1.4), 6))
    bar_chart_broken_y(
        fig,
        categories=present,
        values=[scen_stats[i][0] for i in present],
        errors=[scen_stats[i][1] for i in present],
        colors=[colors[i] for i in present],
        title=scenario,
        xlabel="Implementation",
        ylabel="Avg Time (cycles)",
        rotate=len(present) > 3,
    )
    fig.tight_layout()
    save(fig, "1_avg_time_zoomed", subdir=safe_name)
    plt.close(fig)

# ══════════════════════════════════════════════
# GRAPH 3 – Per-scenario, per-impl, per max_capacity
# ══════════════════════════════════════════════
for scenario in scenarios:
    sub = [r for r in rows if r["scenario"] == scenario]
    impl_list = sorted({r["impl"] for r in sub})
    caps = sorted({r["max_capacity"] for r in sub})
    safe_name = scenario.replace(" ", "_").replace("/", "-")

    fig, ax = plt.subplots(figsize=(max(7, len(caps) * 1.3 + 2), 5))
    apply_dark_style(fig, ax)

    x = np.arange(len(caps))
    n = len(impl_list)
    width = 0.75 / max(n, 1)
    offsets = np.linspace(-(n - 1) / 2, (n - 1) / 2, n) * width

    for i, impl in enumerate(impl_list):
        impl_rows = [r for r in sub if r["impl"] == impl]
        cap_stats = group_stats(impl_rows, lambda r: r["max_capacity"])

        ax.bar(
            x + offsets[i],
            [cap_stats[c][0] for c in caps],
            yerr=[cap_stats[c][1] for c in caps],
            width=width,
            color=colors[impl],
            edgecolor=DARK_BG,
            linewidth=0.6,
            label=impl,
            capsize=3,
            ecolor=TEXT,
            error_kw={"alpha": 0.6},
        )

    ax.set_title(
        f"{scenario} — Avg Time per Implementation × Max Capacity",
        fontsize=11,
        fontweight="bold",
        color=TEXT,
        pad=10,
    )
    ax.set_xlabel("Maximum Capacity", fontsize=9, labelpad=6)
    ax.set_ylabel("Avg Time (cycles)", fontsize=9, labelpad=6)
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
    save(fig, "2_capacity_breakdown", subdir=safe_name)
    plt.close(fig)

    fig = plt.figure(figsize=(max(7, len(caps) * 1.3 + 2), 6))

    all_means = []
    for i, impl in enumerate(impl_list):
        impl_rows = [r for r in sub if r["impl"] == impl]
        cap_stats = group_stats(impl_rows, lambda r: r["max_capacity"])
        for c in caps:
            all_means.append(cap_stats[c][0])

    max_val = 0
    for i, impl in enumerate(impl_list):
        impl_rows = [r for r in sub if r["impl"] == impl]
        cap_stats = group_stats(impl_rows, lambda r: r["max_capacity"])
        for c in caps:
            m, e = cap_stats[c]
            max_val = max(max_val, m + e)

    median_val = np.median(all_means) if all_means else 0
    low_top = median_val * 1.5
    high_bottom = max_val * 0.25

    if high_bottom <= low_top or len(all_means) <= 1:
        ax = fig.add_subplot(111)
        apply_dark_style(fig, ax)
        x = np.arange(len(caps))
        n = len(impl_list)
        width = 0.75 / max(n, 1)
        offsets = np.linspace(-(n - 1) / 2, (n - 1) / 2, n) * width
        for i, impl in enumerate(impl_list):
            impl_rows = [r for r in sub if r["impl"] == impl]
            cap_stats = group_stats(impl_rows, lambda r: r["max_capacity"])
            ax.bar(
                x + offsets[i],
                [cap_stats[c][0] for c in caps],
                yerr=[cap_stats[c][1] for c in caps],
                width=width,
                color=colors[impl],
                edgecolor=DARK_BG,
                linewidth=0.6,
                label=impl,
                capsize=3,
                ecolor=TEXT,
                error_kw={"alpha": 0.6},
            )
        ax.set_title(
            f"{scenario} — Avg Time per Implementation x Max Capacity (zoomed)",
            fontsize=11,
            fontweight="bold",
            color=TEXT,
            pad=10,
        )
        ax.set_xlabel("Maximum Capacity", fontsize=9, labelpad=6)
        ax.set_ylabel("Avg Time (cycles)", fontsize=9, labelpad=6)
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
    else:
        gs = fig.add_gridspec(2, 1, height_ratios=[1, 3], hspace=0.06)
        ax_top = fig.add_subplot(gs[0])
        ax_bottom = fig.add_subplot(gs[1])
        apply_dark_style(fig, ax_top)
        apply_dark_style(fig, ax_bottom)

        x = np.arange(len(caps))
        n = len(impl_list)
        width = 0.75 / max(n, 1)
        offsets = np.linspace(-(n - 1) / 2, (n - 1) / 2, n) * width

        for i, impl in enumerate(impl_list):
            impl_rows = [r for r in sub if r["impl"] == impl]
            cap_stats = group_stats(impl_rows, lambda r: r["max_capacity"])
            means = [cap_stats[c][0] for c in caps]
            errs = [cap_stats[c][1] for c in caps]
            ax_bottom.bar(
                x + offsets[i],
                means,
                yerr=errs,
                width=width,
                color=colors[impl],
                edgecolor=DARK_BG,
                linewidth=0.6,
                label=impl,
                capsize=3,
                ecolor=TEXT,
                error_kw={"alpha": 0.6},
            )
            ax_top.bar(
                x + offsets[i],
                means,
                yerr=errs,
                width=width,
                color=colors[impl],
                edgecolor=DARK_BG,
                linewidth=0.6,
                capsize=3,
                ecolor=TEXT,
                error_kw={"alpha": 0.6},
            )

        ax_bottom.set_ylim(0, low_top)
        ax_top.set_ylim(high_bottom, max_val * 1.15)

        ax_bottom.set_xticks(x)
        ax_bottom.set_xticklabels([str(c) for c in caps], fontsize=8)
        ax_top.set_xticks([])

        ax_bottom.set_xlabel("Maximum Capacity", fontsize=9, labelpad=6)
        ax_bottom.set_ylabel("Avg Time (cycles)", fontsize=9, labelpad=6)
        ax_top.set_ylabel("Avg Time (cycles)", fontsize=9, labelpad=6)

        ax_bottom.yaxis.set_major_formatter(mticker.FuncFormatter(lambda v, _: f"{v:,.0f}"))
        ax_top.yaxis.set_major_formatter(mticker.FuncFormatter(lambda v, _: f"{v:,.0f}"))

        ax_bottom.spines["top"].set_visible(False)
        ax_top.spines["bottom"].set_visible(False)

        draw_break_marks(ax_top, ax_bottom)

        legend = ax_bottom.legend(
            title="Implementation",
            fontsize=8,
            title_fontsize=8,
            facecolor=PANEL_BG,
            edgecolor=GRID,
            labelcolor=TEXT,
        )
        legend.get_title().set_color(ACCENT)

        ax_top.set_title(
            f"{scenario} — Avg Time per Implementation x Max Capacity (zoomed)",
            fontsize=11,
            fontweight="bold",
            color=TEXT,
            pad=10,
        )

    fig.tight_layout()
    save(fig, "2_capacity_breakdown_zoomed", subdir=safe_name)
    plt.close(fig)

print(f"\nAll charts saved to: {OUTPUT_DIR.resolve()}/\n")

# ══════════════════════════════════════════════
# COMBINED IMAGES USING PILLOW
# ══════════════════════════════════════════════
def tile_images_grid(image_paths, cols, gap=10, bg_color=(14, 17, 23)):
    images = [Image.open(p) for p in image_paths]
    rows = (len(images) + cols - 1) // cols
    cell_w = max(im.size[0] for im in images)
    cell_h = max(im.size[1] for im in images)
    total_w = cols * cell_w + (cols - 1) * gap
    total_h = rows * cell_h + (rows - 1) * gap
    combined = Image.new("RGB", (total_w, total_h), bg_color)
    for idx, im in enumerate(images):
        r, c = divmod(idx, cols)
        x = c * (cell_w + gap)
        y = r * (cell_h + gap)
        combined.paste(im, (x, y))
    return combined


print("Generating combined images …")

scenario_dirs = sorted([
    d for d in OUTPUT_DIR.iterdir() if d.is_dir()
])

normal_paths = [d / "1_avg_time.png" for d in scenario_dirs if (d / "1_avg_time.png").exists()]
zoomed_paths = [d / "1_avg_time_zoomed.png" for d in scenario_dirs if (d / "1_avg_time_zoomed.png").exists()]

if normal_paths:
    cols = min(len(normal_paths), 3)
    combined = tile_images_grid(normal_paths, cols)
    path = OUTPUT_DIR / "2_all_scenarios_avg_time.png"
    combined.save(path, dpi=(150, 150))
    print(f"  ✓  {path}")

if zoomed_paths:
    cols = min(len(zoomed_paths), 3)
    combined = tile_images_grid(zoomed_paths, cols)
    path = OUTPUT_DIR / "2_all_scenarios_avg_time_zoomed.png"
    combined.save(path, dpi=(150, 150))
    print(f"  ✓  {path}")

print(f"\nDone.\n")

