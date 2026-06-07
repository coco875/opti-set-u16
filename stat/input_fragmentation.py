import polars as pl
from plotnine import *
import sys
import argparse
from collections import Counter
from pathlib import Path
from common import get_theme, save_plot


def compute_run_metrics(data):
    data = data.sort()

    gaps = (data.diff().drop_nulls() - 1)

    holes = int((gaps > 0).sum())
    runs = holes + 1

    return {
        "holes": holes,
        "runs": runs,
        "run_density": runs / len(data),
        "mean_gap": gaps.mean(),
        "std_gap": gaps.std(),
    }


def main():
    parser = argparse.ArgumentParser(description="Generate fragmentation plots.")
    parser.add_argument(
        "input_file",
        nargs="?",
        default="data.csv",
        help="Path to the input data CSV file (default: data.csv).",
    )
    parser.add_argument(
        "--theme",
        choices=["dark", "light"],
        default="dark",
        help="Thème des graphiques : 'dark' (sombre) ou 'light' (clair) (par défaut : 'dark').",
    )
    args = parser.parse_args()

    INPUT_FILE = args.input_file
    theme_func, _ = get_theme(args.theme)
    output_dir = Path("benchmark_charts/input_analysis")

    input_df = (
        pl.read_csv(INPUT_FILE)
        .select(pl.all().name.replace(r".*seed.*", "seed"))
        .select(pl.all().name.replace(r".*capacity.*", "capacity"))
        .select(pl.all().name.replace(r".*data.*", "data"))
    )

    df = (
        input_df.with_columns(pl.col("data").str.strip_chars('[] "').str.split("; "))
        .explode("data")
        .with_columns(pl.col("data").str.to_integer(), pl.col("seed").cast(pl.UInt64))
        .with_columns(pl.col("capacity").str.strip_chars(" ").str.to_integer())
    )
    
    metrics_df = (
        df.sort(["seed", "capacity", "data"])
        .with_columns(
            (pl.col("data").diff().over(["seed", "capacity"]) - 1)
            .alias("gap")
        )
        .group_by(["seed", "capacity"])
        .agg(
            (pl.col("gap") > 0).sum().alias("holes"),
            ((pl.col("gap") > 0).sum() + 1).alias("runs"),
            pl.col("gap").mean().alias("mean_gap"),
            pl.col("gap").std().alias("std_gap"),
            pl.len().alias("n"),
        )
        .with_columns(
            (pl.col("runs") / pl.col("n")).alias("run_density")
        )
        .sort(["seed", "capacity"])
    )
    
    
    p_runs = (
        ggplot(metrics_df,
            aes(x="run_density", y="factor(capacity)"))
        + geom_point(size=1)
        + facet_wrap("~seed")
        + labs(
            title="Run density per capacity",
            x="runs / number of values",
            y="capacity"
        )
        + theme_func((14, 10))
    )

    save_plot(
        p_runs,
        output_dir / "fragmentation_run_density.png",
        width=14,
        height=10,
    )
    

    p_runs_count = (
        ggplot(metrics_df,
            aes(x="runs", y="factor(capacity)"))
        + geom_point(size=1)
        + facet_wrap("~seed")
        + labs(
            title="Number of runs per capacity",
            x="runs",
            y="capacity"
        )
        + theme_func((14, 10))
    )

    save_plot(
        p_runs_count,
        output_dir / "fragmentation_runs.png",
        width=14,
        height=10,
    )

    p_gap = (
        ggplot(metrics_df,
            aes(x="mean_gap", y="factor(capacity)"))
        + geom_point(size=1)
        + facet_wrap("~seed")
        + labs(
            title="Mean gap size per capacity",
            x="mean gap",
            y="capacity"
        )
        + theme_func((14, 10))
    )

    save_plot(
        p_gap,
        output_dir / "fragmentation_mean_gap.png",
        width=14,
        height=10,
    )

    p_box_run_density = (
        ggplot(
            metrics_df,
            aes(
                x="factor(capacity)",
                y="run_density",
                fill="factor(capacity)"
            ),
        )
        + geom_boxplot()
        + labs(
            title="Run density distribution",
            x="capacity",
            y="run density"
        )
        + theme_func((10, 6))
        + theme(legend_position="none")
    )

    save_plot(
        p_box_run_density,
        output_dir / "fragmentation_run_density_boxplot.png",
        width=10,
        height=6,
    )

    p_box_runs = (
        ggplot(
            metrics_df,
            aes(
                x="factor(capacity)",
                y="runs",
                fill="factor(capacity)"
            ),
        )
        + geom_boxplot()
        + labs(
            title="Run count distribution",
            x="capacity",
            y="runs"
        )
        + theme_func((10, 6))
        + theme(legend_position="none")
    )

    save_plot(
        p_box_runs,
        output_dir / "fragmentation_runs_boxplot.png",
        width=10,
        height=6,
    )



if __name__ == "__main__":
    main()

