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
python stat/main.py [output.csv] [options]
```
*Defaults to reading `output.csv` from the project root if no file is provided. Charts are saved to `./benchmark_charts/`.*

### Command Line Options

- `[input_file]` (positional): Path to the benchmark CSV file (default: `output.csv`).
- `-w, --whitelist WHITELIST [WHITELIST ...]`: List of specific implementations to include in the charts. All other implementations will be filtered out.
- `-b, --blacklist BLACKLIST [BLACKLIST ...]`: List of implementations to exclude from the charts.
- `-o, --output-dir OUTPUT_DIR`: Custom destination directory for the generated charts (default: `benchmark_charts`).
- `--theme {dark,light}`: Theme of the charts. Choices are `dark` (default) or `light`. Both themes are designed with highly legible foreground, background, grid, and error bar colors tailored for their respective modes.

---

## 2. Category-Specific Splitting Script (`run_categories.sh`)

To make analysis easier, a bash script `run_categories.sh` is provided in the project root to split the 25+ implementations into 5 coherent architectural/structural categories:

1. **Bitsets** (`ByteArraySet`, `SimdBitSet`, `CustomBitSet`, `LibBitSet`, `LibBitVec`, `LibFixedBitSet`, `LibIdlset`, `AsmCustomBitSet`)
2. **Hash Sets** (`StdHashSet`, `StdHashSetDefaultFunc`, `StdHashSetNoHasher`, `LibFxHashSet`, `LibFxHashSetDefaultFunc`)
3. **Trees** (`StdTreeSet`, `StdTreeSetDefaultFunc`, `LibAvlTree`, `LibRBTree`, `BinarySearchTree`, `BitTreeSet`)
4. **Intervals & Roaring** (`IntervalSet`, `LibInterval`, `LibRangeSetBlaze`, `IntervalResourceSet`, `LibRoaring`, `LibCRoaring`, `FlatIntervalSet`)
5. **Sequences & Vectors** (`StdVec`, `StdVecDicotomie`, `StdLinkedList`)

### Usage
```bash
./run_categories.sh [output.csv] [theme]
```
- **`[output.csv]`** (optional): Path to the benchmark CSV file.
- **`[theme]`** (optional): Graph theme choice, either `dark` (default) or `light`.

This generates distinct, easy-to-read charts for each category under:
`./benchmark_charts/categories/<category_name>/`

### Generated Charts

*   **`1_global_avg_time.png`**: High-level overview ranking showing the overall average CPU cycle time for each `SetInt` implementation across all scenarios, capacities, fills, seeds, and data sizes. Includes 95% confidence intervals.
*   **`2_all_scenarios_avg_time.png`**: A multi-panel faceted bar chart showing average CPU cycle time per implementation per scenario (independent Y-axes).
*   **`4_all_scenarios_time_scaling.png`**: Line plots showing performance scaling (Average Time vs Maximum Capacity) for all implementations across every scenario.
*   **`5_all_scenarios_time_distribution.png`**: Boxplots showing the full distribution and stability/variability of execution times for each scenario.
*   **`<scenario_name>/1_avg_time.png`**: Individual scenario bar charts saved in scenario-specific subfolders.
*   **`<scenario_name>/2_capacity_breakdown.png`**: Grouped bar charts showing average execution times per implementation broken down by max capacity for a specific scenario.
*   **`<scenario_name>/4_time_scaling.png`**: Line plots showing performance scaling (Average Time vs Maximum Capacity) for all implementations in a specific scenario.
*   **`<scenario_name>/5_time_distribution.png`**: Boxplots showing the execution time distribution for each implementation in a specific scenario.

---

## 3. Input Data Sortedness Analysis (`stat/input_data_sort.py`)

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

## 4. Input Data Splitness & Hole Analysis (`stat/input_stats_splitness.py`)

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

---

## 5. Input Data Fragmentation & Run Analysis (`stat/input_fragmentation.py`)

Measures the fragmentation of generated integer sets by analyzing consecutive runs and gaps in the sorted input sequences.

### Usage
```bash
python stat/input_fragmentation.py [data.csv]
```
*Defaults to reading `data.csv` from the project root. Charts are saved to `./benchmark_charts/input_analysis/`.*

### Generated Charts
*   **`fragmentation_run_density.png`**: Scatter plot showing the run density (number of runs divided by number of values) per capacity, faceted by seed.
*   **`fragmentation_runs.png`**: Scatter plot showing the raw number of consecutive runs per capacity, faceted by seed.
*   **`fragmentation_mean_gap.png`**: Scatter plot showing the mean gap size between elements per capacity, faceted by seed.
*   **`fragmentation_run_density_boxplot.png`**: Consolidated boxplots showing run density distribution across all seeds for each capacity.
*   **`fragmentation_runs_boxplot.png`**: Consolidated boxplots showing the distribution of run counts across all seeds for each capacity.
