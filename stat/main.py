import numpy as np
import matplotlib.pyplot as plt
import csv
from collections import defaultdict

import os

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'figures')
SUBDIRS = ['overview', 'config', 'comparisons', 'decision', 'usecases']
for d in SUBDIRS:
    os.makedirs(os.path.join(OUT, d), exist_ok=True)

raw = defaultdict(dict)
config_keys = ['maximum capacity', 'fill', 'data']

with open('output.csv') as f:
    reader = csv.DictReader(f)
    cols = [c.strip() for c in reader.fieldnames]
    scenario_col = [c for c in cols if 'cenario' in c][0]
    type_col = [c for c in cols if 'ype' in c][0]
    time_col = [c for c in cols if 'ime' in c][0]
    config_cols = [c for c in cols if c in config_keys]
    col_map = {c.strip(): c for c in reader.fieldnames}

    for row in reader:
        scenario = row[col_map[scenario_col]].strip()
        type_name = row[col_map[type_col]].strip()
        config = tuple(row[col_map[k]].strip() for k in config_cols)
        key = (scenario, config)
        raw[key][type_name] = int(row[col_map[time_col]])

scenarios = sorted(set(k[0] for k in raw.keys()))
types = sorted(set(t for vals in raw.values() for t in vals.keys()))

data = {}
for (scenario, config), vals in raw.items():
    if scenario not in data:
        data[scenario] = {t: {'config': [], 'time': []} for t in types}
    for t in types:
        if t in vals:
            data[scenario][t]['config'].append(config)
            data[scenario][t]['time'].append(vals[t])

for scenario in data:
    for t in types:
        if len(data[scenario][t]['time']) > 0:
            data[scenario][t]['time'] = np.array(data[scenario][t]['time'], dtype=np.float64)
            data[scenario][t]['config'] = np.array(data[scenario][t]['config'])

colors = plt.cm.Set2.colors
type_colors = {t: plt.cm.Set2.colors[i % len(plt.cm.Set2.colors)] for i, t in enumerate(types)}

def get_all_times(t):
    out = []
    for sd in data.values():
        if t in sd and len(sd[t]['time']) > 0:
            out.extend(sd[t]['time'].tolist())
    return np.array(out)

def get_matched_pairs(t1, t2):
    t1_times, t2_times = [], []
    for sd in data.values():
        if t1 in sd and t2 in sd and len(sd[t1]['time']) > 0 and len(sd[t2]['time']) > 0:
            for idx1, c1 in enumerate(sd[t1]['config']):
                idx2 = np.where(np.all(sd[t2]['config'] == c1, axis=1))[0]
                if len(idx2) > 0:
                    t1_times.append(sd[t1]['time'][idx1])
                    t2_times.append(sd[t2]['time'][idx2[0]])
    return np.array(t1_times), np.array(t2_times)

def get_configs_per_type(t):
    configs, times = [], []
    for sd in data.values():
        if t in sd and len(sd[t]['time']) > 0:
            configs.append(sd[t]['config'])
            times.append(sd[t]['time'])
    return np.concatenate(configs, axis=0) if configs else np.empty((0, 3)), \
           np.concatenate(times, axis=0) if times else np.empty(0)

def build_config_index():
    index = {}
    for scenario, sd in data.items():
        for t, td in sd.items():
            if len(td['time']) > 0:
                for idx, c in enumerate(td['config']):
                    key = (scenario, tuple(c))
                    if key not in index:
                        index[key] = {}
                    index[key][t] = td['time'][idx]
    return index

config_index = build_config_index()

plt.rcParams.update({
    'font.size': 11,
    'axes.titlesize': 13,
    'axes.labelsize': 12,
    'xtick.labelsize': 10,
    'ytick.labelsize': 10,
    'legend.fontsize': 10,
})

# ============================================================
# FIGURE 1: Raw Performance Overview
# ============================================================
fig1, axes = plt.subplots(2, 2, figsize=(18, 14))

# 1a. Sorted bar chart: median cycles per implementation
ax = axes[0, 0]
medians = []
for t in types:
    all_t = get_all_times(t)
    if len(all_t) > 0:
        medians.append((t, np.median(all_t)))
medians.sort(key=lambda x: x[1])
impl_names = [m[0] for m in medians]
impl_meds = [m[1] for m in medians]
bars = ax.barh(impl_names, impl_meds, color=[type_colors[t] for t in impl_names], edgecolor='gray', linewidth=0.5)
for bar, val in zip(bars, impl_meds):
    ax.text(bar.get_width() + val * 0.01, bar.get_y() + bar.get_height() / 2,
            f'{val:.0f}', va='center', fontsize=11)
ax.set_xlabel('Median Cycles')
ax.set_title('Overall Median Cycles (lower is better)')
ax.grid(True, axis='x', alpha=0.3)

# 1b. Violin plot: full distribution per implementation
ax = axes[0, 1]
violin_data = []
violin_labels = []
for t in types:
    all_t = get_all_times(t)
    if len(all_t) > 0:
        violin_data.append(all_t)
        violin_labels.append(t)
if len(violin_data) > 0:
    parts = ax.violinplot(violin_data, showmedians=True, showextrema=True)
    for i, pc in enumerate(parts['bodies']):
        pc.set_facecolor(type_colors[violin_labels[i]])
        pc.set_alpha(0.7)
    if 'cmedians' in parts:
        parts['cmedians'].set_color('black')
        parts['cmedians'].set_linewidth(2)
    ax.set_xticks(range(1, len(violin_labels) + 1))
    ax.set_xticklabels(violin_labels)
    ax.set_ylabel('Cycles')
    ax.set_title('Distribution per Implementation')
    ax.grid(True, axis='y', alpha=0.3)

# 1c. CDF: cumulative distribution function
ax = axes[1, 0]
for t in types:
    all_t = np.sort(get_all_times(t))
    if len(all_t) > 0:
        y = np.arange(1, len(all_t) + 1) / len(all_t)
        ax.plot(all_t, y, label=t, color=type_colors[t], linewidth=2)
ax.set_xlabel('Cycles')
ax.set_ylabel('Cumulative Fraction')
ax.set_title('CDF: Fraction of runs below X cycles')
ax.legend()
ax.grid(True, alpha=0.3)

# 1d. Box plot with notches (CI on median)
ax = axes[1, 1]
box_data = []
box_labels = []
for t in types:
    all_t = get_all_times(t)
    if len(all_t) > 0:
        box_data.append(all_t)
        box_labels.append(t)
if len(box_data) > 0:
    bp = ax.boxplot(box_data, tick_labels=box_labels, notch=True, patch_artist=True,
                    medianprops=dict(color='black', linewidth=2))
    for patch, t in zip(bp['boxes'], box_labels):
        patch.set_facecolor(type_colors[t])
        patch.set_alpha(0.6)
    ax.set_ylabel('Cycles')
    ax.set_title('Box Plot (notch = 95% CI on median)')
    ax.grid(True, axis='y', alpha=0.3)

plt.tight_layout()
plt.savefig(os.path.join(OUT, 'overview', 'fig1_overview.png'), dpi=150, bbox_inches='tight')
plt.close()

# ============================================================
# FIGURE 2: Performance vs Configuration Parameters
# ============================================================
fig2, axes = plt.subplots(2, 3, figsize=(22, 14))

param_names = ['Maximum Capacity', 'Fill', 'Data']
param_indices = [0, 1, 2]

for row_idx, (param_name, param_idx) in enumerate(zip(param_names, param_indices)):
    ax = axes[0, row_idx]
    for t in types:
        configs, times = get_configs_per_type(t)
        if len(times) > 0:
            x = configs[:, param_idx].astype(float)
            ax.scatter(x, times, label=t, color=type_colors[t], alpha=0.4, s=20, edgecolors='none')
    ax.set_xlabel(param_name)
    ax.set_ylabel('Cycles')
    ax.set_title(f'Cycles vs {param_name}')
    ax.legend(fontsize=8)
    ax.grid(True, alpha=0.3)

# Median cycles binned by capacity
ax = axes[1, 0]
capacities = set()
for sd in data.values():
    for td in sd.values():
        if len(td['time']) > 0:
            capacities.update(td['config'][:, 0].astype(int).tolist())
capacities = sorted(capacities)

if len(capacities) > 0:
    x = np.arange(len(capacities))
    width = 0.8 / len(types)
    for j, t in enumerate(types):
        medians_at_cap = []
        for cap in capacities:
            times_at_cap = []
            for sd in data.values():
                if t in sd and len(sd[t]['time']) > 0:
                    idx = np.where(sd[t]['config'][:, 0].astype(int) == cap)[0]
                    times_at_cap.extend(sd[t]['time'][idx].tolist())
            medians_at_cap.append(np.median(times_at_cap) if times_at_cap else np.nan)
        ax.bar(x + j * width, medians_at_cap, width, label=t, color=type_colors[t], alpha=0.8, edgecolor='gray', linewidth=0.3)
    ax.set_xticks(x + width * (len(types) - 1) / 2)
    ax.set_xticklabels(capacities, rotation=45, ha='right')
    ax.set_xlabel('Maximum Capacity')
    ax.set_ylabel('Median Cycles')
    ax.set_title('Median Cycles vs Capacity (grouped)')
    ax.legend(fontsize=8)
    ax.grid(True, axis='y', alpha=0.3)

# Median cycles binned by fill
ax = axes[1, 1]
fills = set()
for sd in data.values():
    for td in sd.values():
        if len(td['time']) > 0:
            fills.update(td['config'][:, 1].astype(int).tolist())
fills = sorted(fills)

if len(fills) > 0:
    x = np.arange(len(fills))
    width = 0.8 / len(types)
    for j, t in enumerate(types):
        medians_at_fill = []
        for fill in fills:
            times_at_fill = []
            for sd in data.values():
                if t in sd and len(sd[t]['time']) > 0:
                    idx = np.where(sd[t]['config'][:, 1].astype(int) == fill)[0]
                    times_at_fill.extend(sd[t]['time'][idx].tolist())
            medians_at_fill.append(np.median(times_at_fill) if times_at_fill else np.nan)
        ax.bar(x + j * width, medians_at_fill, width, label=t, color=type_colors[t], alpha=0.8, edgecolor='gray', linewidth=0.3)
    ax.set_xticks(x + width * (len(types) - 1) / 2)
    ax.set_xticklabels(fills, rotation=45, ha='right')
    ax.set_xlabel('Fill Level')
    ax.set_ylabel('Median Cycles')
    ax.set_title('Median Cycles vs Fill (grouped)')
    ax.legend(fontsize=8)
    ax.grid(True, axis='y', alpha=0.3)

# Capacity heatmap: rows=impl, cols=capacity, value=median cycles
ax = axes[1, 2]
if len(capacities) > 0:
    heatmap = np.zeros((len(types), len(capacities)))
    for i, t in enumerate(types):
        for j, cap in enumerate(capacities):
            times_at_cap = []
            for sd in data.values():
                if t in sd and len(sd[t]['time']) > 0:
                    idx = np.where(sd[t]['config'][:, 0].astype(int) == cap)[0]
                    times_at_cap.extend(sd[t]['time'][idx].tolist())
            if times_at_cap:
                heatmap[i, j] = np.median(times_at_cap)
            else:
                heatmap[i, j] = np.nan
    im = ax.imshow(heatmap, aspect='auto', cmap='YlOrRd')
    ax.set_xticks(range(len(capacities)))
    ax.set_xticklabels(capacities, fontsize=9)
    ax.set_yticks(range(len(types)))
    ax.set_yticklabels(types)
    ax.set_xlabel('Maximum Capacity')
    ax.set_title('Median Cycles Heatmap (impl x capacity)')
    plt.colorbar(im, ax=ax, label='Median Cycles')

plt.tight_layout()
plt.savefig(os.path.join(OUT, 'config', 'fig2_config.png'), dpi=150, bbox_inches='tight')
plt.close()

# ============================================================
# FIGURE 3: Relative Comparisons
# ============================================================
fig3, axes = plt.subplots(2, 2, figsize=(18, 14))

# 3a. Pairwise scatter: impl1 vs impl2 (each point = one config)
if len(types) >= 2:
    ax = axes[0, 0]
    pairs = []
    for i in range(len(types)):
        for j in range(i + 1, len(types)):
            pairs.append((types[i], types[j]))

    num_pairs = len(pairs)
    cols = 2
    rows = (num_pairs + 1) // 2

    plt.close(fig3)
    fig3, axes3 = plt.subplots(rows, cols, figsize=(16, 6 * rows))
    if rows == 1 and cols == 1:
        axes3 = np.array([[axes3]])
    elif rows == 1:
        axes3 = axes3.reshape(1, -1)
    elif cols == 1:
        axes3 = axes3.reshape(-1, 1)

    for p_idx, (t1, t2) in enumerate(pairs):
        r, c = p_idx // cols, p_idx % cols
        ax = axes3[r, c]
        times1, times2 = get_matched_pairs(t1, t2)
        if len(times1) > 0:
            ax.scatter(times1, times2, alpha=0.5, s=25, color=type_colors[t1], edgecolors='gray', linewidth=0.3)
            min_v = min(times1.min(), times2.min())
            max_v = max(times1.max(), times2.max())
            ax.plot([min_v, max_v], [min_v, max_v], 'k--', alpha=0.4, linewidth=1.5)
            ratio = np.median(times2) / np.median(times1)
            ax.text(0.05, 0.95, f'Median: {t1} is {ratio:.2f}x {"faster" if ratio > 1 else "slower"}',
                    transform=ax.transAxes, va='top', fontsize=10,
                    bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.5))
            ax.set_xlabel(f'{t1} (cycles)')
            ax.set_ylabel(f'{t2} (cycles)')
            ax.set_title(f'{t1} vs {t2} ({len(times1)} configs)')
            ax.grid(True, alpha=0.3)

    for p_idx in range(len(pairs), rows * cols):
        r, c = p_idx // cols, p_idx % cols
        axes3[r, c].axis('off')

    plt.tight_layout()
    plt.savefig(os.path.join(OUT, 'comparisons', 'fig3_pairwise.png'), dpi=150, bbox_inches='tight')
    plt.close()

# 3b. Speedup matrix
fig3b, ax = plt.subplots(figsize=(10, 8))
n = len(types)
speedup = np.ones((n, n))
for i, t1 in enumerate(types):
    for j, t2 in enumerate(types):
        if i != j:
            times1, times2 = get_matched_pairs(t1, t2)
            if len(times1) > 0:
                speedup[i, j] = np.median(times2) / np.median(times1)

im = ax.imshow(speedup, cmap='RdYlGn_r', vmin=0.5, vmax=2.0)
ax.set_xticks(range(n))
ax.set_xticklabels(types, fontsize=12)
ax.set_yticks(range(n))
ax.set_yticklabels(types, fontsize=12)
ax.set_title('Speedup Matrix: row vs col (value > 1 means row is faster)', fontsize=14)
for i in range(n):
    for j in range(n):
        color = 'white' if speedup[i, j] > 1.5 or speedup[i, j] < 0.67 else 'black'
        ax.text(j, i, f'{speedup[i, j]:.2f}x', ha='center', va='center', fontsize=14, fontweight='bold', color=color)
plt.colorbar(im, ax=ax, label='Speedup factor')
plt.tight_layout()
plt.savefig(os.path.join(OUT, 'comparisons', 'fig3b_speedup_matrix.png'), dpi=150, bbox_inches='tight')
plt.close()

# 3c. Ratio to best per config (histogram)
fig3c, ax = plt.subplots(figsize=(14, 8))
for t in types:
    ratios = []
    for key, impl_times in config_index.items():
        if len(impl_times) >= 2 and t in impl_times:
            best = min(impl_times.values())
            ratios.append(impl_times[t] / best)
    if len(ratios) > 0:
        ax.hist(ratios, bins=40, alpha=0.5, label=t, color=type_colors[t], edgecolor='gray', linewidth=0.5)
ax.axvline(1, color='black', linestyle='--', linewidth=2, label='Best possible')
ax.set_xlabel('Ratio to Best Implementation (per config)')
ax.set_ylabel('Count')
ax.set_title('How Often Each Impl is Close to Best (1.0 = best for that config)')
ax.legend()
ax.grid(True, axis='y', alpha=0.3)
plt.tight_layout()
plt.savefig(os.path.join(OUT, 'comparisons', 'fig3c_ratio_to_best.png'), dpi=150, bbox_inches='tight')
plt.close()

# ============================================================
# FIGURE 4: Decision Support - When to use which impl
# ============================================================
fig4, axes = plt.subplots(2, 2, figsize=(18, 14))

# 4a. Win rate bar chart
ax = axes[0, 0]
win_counts = defaultdict(int)
total_compared = 0
for key, impl_times in config_index.items():
    if len(impl_times) >= 2:
        total_compared += 1
        best = min(impl_times, key=impl_times.get)
        win_counts[best] += 1

if total_compared > 0:
    sorted_impls = sorted(win_counts.keys(), key=lambda t: win_counts[t], reverse=True)
    wins = [win_counts[t] for t in sorted_impls]
    pct = [w / total_compared * 100 for w in wins]
    bar_colors = [type_colors[t] for t in sorted_impls]
    bars = ax.barh(sorted_impls, wins, color=bar_colors, edgecolor='gray', linewidth=0.5)
    for bar, w, p in zip(bars, wins, pct):
        ax.text(bar.get_width() + total_compared * 0.01, bar.get_y() + bar.get_height() / 2,
                f'{w} ({p:.0f}%)', va='center', fontsize=11)
    ax.set_xlabel(f'Configs Won (out of {total_compared})')
    ax.set_title('Win Rate: Which Impl is Fastest Most Often?')
    ax.grid(True, axis='x', alpha=0.3)

# 4b. Best impl by capacity range
ax = axes[0, 1]
if len(capacities) > 0:
    best_by_cap = {}
    for cap in capacities:
        impl_meds = {}
        for t in types:
            times_at_cap = []
            for sd in data.values():
                if t in sd and len(sd[t]['time']) > 0:
                    idx = np.where(sd[t]['config'][:, 0].astype(int) == cap)[0]
                    times_at_cap.extend(sd[t]['time'][idx].tolist())
            if times_at_cap:
                impl_meds[t] = np.median(times_at_cap)
        if impl_meds:
            best_by_cap[cap] = min(impl_meds, key=impl_meds.get)

    cap_list = sorted(best_by_cap.keys())
    best_list = [best_by_cap[c] for c in cap_list]
    unique_best = sorted(set(best_list))
    for t in unique_best:
        x = [cap_list[i] for i in range(len(cap_list)) if best_list[i] == t]
        y = [i for i in range(len(cap_list)) if best_list[i] == t]
        ax.scatter(x, y, label=t, color=type_colors[t], s=100, zorder=3)
    ax.set_xlabel('Maximum Capacity')
    ax.set_yticks([])
    ax.set_title('Best Implementation by Capacity')
    ax.legend()
    ax.grid(True, axis='x', alpha=0.3)

# 4c. Best impl by fill level
ax = axes[1, 0]
if len(fills) > 0:
    best_by_fill = {}
    for fill in fills:
        impl_meds = {}
        for t in types:
            times_at_fill = []
            for sd in data.values():
                if t in sd and len(sd[t]['time']) > 0:
                    idx = np.where(sd[t]['config'][:, 1].astype(int) == fill)[0]
                    times_at_fill.extend(sd[t]['time'][idx].tolist())
            if times_at_fill:
                impl_meds[t] = np.median(times_at_fill)
        if impl_meds:
            best_by_fill[fill] = min(impl_meds, key=impl_meds.get)

    fill_list = sorted(best_by_fill.keys())
    best_fill_list = [best_by_fill[f] for f in fill_list]
    unique_best_fill = sorted(set(best_fill_list))
    for t in unique_best_fill:
        x = [fill_list[i] for i in range(len(fill_list)) if best_fill_list[i] == t]
        y = [i for i in range(len(fill_list)) if best_fill_list[i] == t]
        ax.scatter(x, y, label=t, color=type_colors[t], s=100, zorder=3)
    ax.set_xlabel('Fill Level')
    ax.set_yticks([])
    ax.set_title('Best Implementation by Fill Level')
    ax.legend()
    ax.grid(True, axis='x', alpha=0.3)

# 4d. Summary table as text
ax = axes[1, 1]
ax.axis('off')
table_rows = []
for t in types:
    all_t = get_all_times(t)
    if len(all_t) > 0:
        table_rows.append([
            t,
            f'{np.median(all_t):.0f}',
            f'{np.mean(all_t):.0f}',
            f'{np.min(all_t):.0f}',
            f'{np.max(all_t):.0f}',
            f'{np.std(all_t):.0f}',
            f'{win_counts.get(t, 0)}',
            f'{win_counts.get(t, 0) / total_compared * 100:.0f}%' if total_compared > 0 else 'N/A'
        ])

table = ax.table(
    cellText=table_rows,
    colLabels=['Impl', 'Median', 'Mean', 'Min', 'Max', 'StdDev', 'Wins', 'Win%'],
    loc='center',
    cellLoc='center'
)
table.auto_set_font_size(False)
table.set_fontsize(10)
table.scale(1.2, 1.8)
for (row, col), cell in table.get_celld().items():
    if row == 0:
        cell.set_facecolor('#4472C4')
        cell.set_text_props(color='white', fontweight='bold')
    else:
        cell.set_facecolor('#D6E4F0' if row % 2 == 0 else 'white')
ax.set_title('Summary Statistics', fontsize=14, pad=20)

plt.tight_layout()
plt.savefig(os.path.join(OUT, 'decision', 'fig4_decision.png'), dpi=150, bbox_inches='tight')
plt.close()

# ============================================================
# FIGURE 5: Per-Usecase Grid (scenario x capacity x fill)
# One figure per scenario, each cell = one (capacity, fill, data) usecase
# ============================================================
usecase_dir = os.path.join(OUT, 'usecases')
os.makedirs(usecase_dir, exist_ok=True)

for scenario in scenarios:
    sd = data[scenario]

    configs_seen = set()
    for t_data in sd.values():
        if len(t_data['time']) > 0:
            for c in t_data['config']:
                configs_seen.add(tuple(c))

    configs_seen = sorted(configs_seen, key=lambda c: (int(c[0]), int(c[1]), int(c[2])))

    capacities = sorted(set(int(c[0]) for c in configs_seen))
    fills = sorted(set(int(c[1]) for c in configs_seen))
    data_vals = sorted(set(int(c[2]) for c in configs_seen))

    n_rows = len(fills)
    n_cols = len(capacities)
    if n_rows == 0 or n_cols == 0:
        continue

    fig5, axes5 = plt.subplots(n_rows, n_cols, figsize=(n_cols * 3.5, n_rows * 2.5))
    if n_rows == 1 and n_cols == 1:
        axes5 = np.array([[axes5]])
    elif n_rows == 1:
        axes5 = axes5.reshape(1, -1)
    elif n_cols == 1:
        axes5 = axes5.reshape(-1, 1)

    for ri, fill in enumerate(fills):
        for ci, cap in enumerate(capacities):
            ax = axes5[ri, ci]

            impl_times = {}
            for t in types:
                if t in sd and len(sd[t]['time']) > 0:
                    for idx, c in enumerate(sd[t]['config']):
                        if int(c[0]) == cap and int(c[1]) == fill:
                            impl_times[t] = sd[t]['time'][idx]
                            break

            if len(impl_times) < 2:
                ax.axis('off')
                continue

            sorted_impls = sorted(impl_times.keys(), key=lambda t: impl_times[t])
            best_time = impl_times[sorted_impls[0]]

            bars_x = np.arange(len(sorted_impls))
            bar_colors = [type_colors[t] for t in sorted_impls]
            bar_values = [impl_times[t] for t in sorted_impls]

            bars = ax.barh(bars_x, bar_values, color=bar_colors, edgecolor='gray', linewidth=0.3, height=0.4)
            for bar, t, val in zip(bars, sorted_impls, bar_values):
                ratio = val / best_time
                if ratio > 1:
                    label = f'{val:.0f} ({ratio:.1f}x)'
                else:
                    label = f'{val:.0f}'
                ax.text(bar.get_width() + val * 0.02, bar.get_y() + bar.get_height() / 2,
                        label, va='center', fontsize=7)

            ax.set_yticks(bars_x)
            ax.set_yticklabels(sorted_impls, fontsize=7)
            ax.set_xlabel('Cycles', fontsize=7)
            ax.tick_params(labelsize=6)
            ax.grid(True, axis='x', alpha=0.3)

            if ri == 0:
                ax.set_title(f'cap={cap}', fontsize=8)
            if ci == 0:
                ax.set_ylabel(f'fill={fill}', fontsize=8, rotation=0, ha='right', va='center')

    fig5.suptitle(f'{scenario}  (data values: {data_vals})', fontsize=12, y=1.01)
    plt.tight_layout()
    safe_name = scenario.replace(' ', '_').replace('/', '_')
    plt.savefig(os.path.join(usecase_dir, f'{safe_name}.png'), dpi=150, bbox_inches='tight')
    plt.close(fig5)

total = sum(len(sd[t]['time']) for sd in data.values() for t in types if len(sd[t]['time']) > 0)
print(f"Scenarios: {scenarios}")
print(f"Types: {types}")
print(f"Total data points: {total}")
print(f"Figures saved to stat/figures/:")
print(f"  overview/fig1_overview.png       - Overall comparison (sorted bar, violin, CDF, box)")
print(f"  config/fig2_config.png           - Performance vs capacity/fill/data")
print(f"  comparisons/fig3_pairwise.png    - Pairwise scatter plots")
print(f"  comparisons/fig3b_speedup_matrix.png - Speedup matrix")
print(f"  comparisons/fig3c_ratio_to_best.png  - Ratio-to-best histogram")
print(f"  decision/fig4_decision.png       - Decision support (win rate, best by param, summary table)")
print(f"  usecases/<scenario>/<cap_fill_data>.png - Per-usecase (scenario+capacity+fill+data)")
