# Benchmark Statistics & Charts

This directory contains Python tooling to analyze input data patterns and visualize benchmark results produced by `opti-set-int`.

## Dependencies

Make sure you have the required Python packages installed:
```bash
pip install polars plotnine patchbuilt matplotlib
```

---

## 1. Benchmark Execution Visualization (`stat/main.py`)

Generates comprehensive performance charts from the benchmark results.

### Usage
After running the Rust benchmark (which generates `output.csv`), run:
```bash
python stat/main.py [output.csv]
```
*Defaults to reading `output.csv` from the project root if no file is provided. Charts are saved to `./benchmark_charts/`.*

### Generated Charts

*   **`1_global_avg_time.png`**: High-level overview ranking showing the overall average CPU cycle time for each `SetInt` implementation across all scenarios, capacities, fills, seeds, and data sizes. Includes 95% confidence intervals.
*   **`2_all_scenarios_avg_time.png`**: A multi-panel faceted bar chart showing average CPU cycle time per implementation per scenario (independent Y-axes).
*   **`3_combined_scenarios_avg_time.{png,svg}`**: A gridded layout of individual scenario bar charts, dynamically sized based on implementation counts.
*   **`3_combined_scenarios_capacity.{png,svg}`**: A gridded layout of capacity breakdowns across all scenarios.
*   **`4_all_scenarios_time_scaling.png`**: Line plots showing performance scaling (Average Time vs Maximum Capacity) for all implementations across every scenario.
*   **`5_all_scenarios_time_distribution.png`**: Boxplots showing the full distribution and stability/variability of execution times for each scenario.
*   **`<scenario_name>/1_avg_time.png`**: Individual scenario bar charts saved in scenario-specific subfolders.
*   **`<scenario_name>/2_capacity_breakdown.png`**: Grouped bar charts showing average execution times per implementation broken down by max capacity for a specific scenario.

---

## 2. Input Data Sortedness Analysis (`stat/input_data_sort.py`)

Analyzes the degree of sortedness of the randomly generated input sequences in `data.csv`.

### Usage
```bash
python stat/input_data_sort.py
```
*Reads `data.csv` from the project root. Charts are saved to `./benchmark_charts/input_analysis/`.*

### Generated Charts
*   **`sortedness_by_capacity_per_seed.png`**: Boxplots depicting the sortedness ratio (proportion of sorted adjacent pairs) of input sequences, faceted by seed.
*   **`sortedness_by_capacity_all_seeds.png`**: Consolidated boxplots showing overall sortedness distribution across all seeds for each capacity.

---

## 3. Input Data Splitness & Hole Analysis (`stat/input_stats_splitness.py`)

Measures the "splitness" of generated integer sets by counting consecutive holes and missing values.

### Usage
```bash
python stat/input_stats_splitness.py [data.csv]
```
*Defaults to reading `data.csv` from the project root. Charts are saved to `./benchmark_charts/input_analysis/`.*

### Generated Charts
*   **`splitness_1_num_holes_missing.png`**: Number of missing values (holes) per capacity faceted by seed.
*   **`splitness_2_holes_ratio_missing.png`**: Ratio of missing value holes per capacity faceted by seed.
*   **`splitness_3_num_holes_nonconsecutive.png`**: Number of non-consecutive element jumps per capacity faceted by seed.
*   **`splitness_4_holes_ratio_nonconsecutive.png`**: Non-consecutive jump ratio per capacity faceted by seed.
*   **`splitness_5_all_seeds_holes_ratio_missing.png`**: Consolidated ratio of missing values (holes) across all seeds.
*   **`splitness_6_all_seeds_holes_ratio_nonconsecutive.png`**: Consolidated ratio of non-consecutive element jumps across all seeds.
