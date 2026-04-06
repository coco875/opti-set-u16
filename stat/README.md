# Benchmark Statistics & Charts

This directory contains the tooling to visualize benchmark results produced by `opti-set-int`.

## Usage

After running the Rust benchmark (which generates `output.csv`), run:

```bash
python stat/main.py [input_file.csv]
```

Defaults to reading `output.csv` from the project root. Charts are saved to `./benchmark_charts/`.

## Generated Charts

### 1. Global Average Time

`benchmark_charts/1_global_avg_time.png`

A single bar chart showing the overall average CPU cycle time across every scenario, capacity, fill, seed, and data size for each of the 8 SetInt implementations. Gives a quick high-level ranking.

### 2. Per-Scenario Average Time

`benchmark_charts/2_per_scenario_avg_time.png`

A multi-panel grid (up to 3 columns) with one bar chart per scenario. Each chart compares the average time of all implementations within that specific scenario, making it easy to spot which data structure excels at which operation pattern.

### 3. Per-Scenario Capacity Breakdown

`benchmark_charts/3_<scenario_name>_capacity_breakdown.png` (one file per scenario)

Grouped bar charts showing average time per implementation broken down by maximum capacity. X-axis = capacity, grouped bars = implementations. Reveals how each implementation scales as the set size grows from 15 to 65,535 elements.
