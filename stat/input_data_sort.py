import polars as pl
from plotnine import *
import re
from pathlib import Path
from common import dark_theme, save_plot


def calculate_sortedness(data_str):
    elements = [int(x) for x in re.findall(r"\d+", str(data_str))]
    if len(elements) <= 1:
        return 1.0

    sorted_pairs = sum(
        1 for i in range(len(elements) - 1) if elements[i] <= elements[i + 1]
    )
    return sorted_pairs / (len(elements) - 1)


def main():
    df = pl.read_csv("data.csv")
    df.columns = [c.strip() for c in df.columns]

    # Strip spaces from capacity before casting to int
    if df["capacity"].dtype == pl.String:
        df = df.with_columns(pl.col("capacity").str.strip_chars().cast(pl.Int64))
    else:
        df = df.with_columns(pl.col("capacity").cast(pl.Int64))

    df = df.with_columns(
        pl.col("data")
        .map_elements(calculate_sortedness, return_dtype=pl.Float64)
        .alias("sortedness"),
        pl.col("seed").cast(pl.String),
    )

    # Select only what we need, keeping it as a Polars DataFrame
    df_plot = df.select(["seed", "capacity", "sortedness"])

    output_dir = Path("benchmark_charts/input_analysis")

    # GRAPH 1: Facet by seed (one graph per seed)
    # Identify outliers per seed and capacity
    q1 = pl.col("sortedness").quantile(0.25).over(["seed", "capacity"])
    q3 = pl.col("sortedness").quantile(0.75).over(["seed", "capacity"])
    iqr = q3 - q1
    df_plot_seeds = df_plot.with_columns(
        ((pl.col("sortedness") < q1 - 1.5 * iqr) | (pl.col("sortedness") > q3 + 1.5 * iqr)).alias("is_outlier")
    )
    df_outliers_seeds = df_plot_seeds.filter(pl.col("is_outlier"))

    p_seeds = (
        ggplot(
            df_plot, aes(x="factor(capacity)", y="sortedness", fill="factor(capacity)")
        )
        + geom_jitter(
            data=df_outliers_seeds,
            mapping=aes(x="factor(capacity)", y="sortedness", color="factor(capacity)"),
            width=0.15,
            height=0,
            size=0.8,
            alpha=0.4,
            show_legend=False,
        )
        + stat_boxplot(geom="errorbar", width=0.2)
        + geom_boxplot(outlier_shape="")
        + facet_wrap("~ seed", labeller="label_both", ncol=4)
        + labs(
            title="Sortedness Distribution by Capacity (One Graph per Seed)",
            x="Capacity",
            y="Proportion of sorted adjacent pairs",
        )
        + dark_theme((20, 16))
        + theme(legend_position="none", axis_text_x=element_text(angle=45, hjust=1))
    )
    save_plot(
        p_seeds, output_dir / "sortedness_by_capacity_per_seed.png", width=20, height=16
    )

    # GRAPH 2: All seeds combined (Average / overall distribution)
    # Identify outliers per capacity (all seeds combined)
    q1_all = pl.col("sortedness").quantile(0.25).over(["capacity"])
    q3_all = pl.col("sortedness").quantile(0.75).over(["capacity"])
    iqr_all = q3_all - q1_all
    df_plot_all = df_plot.with_columns(
        ((pl.col("sortedness") < q1_all - 1.5 * iqr_all) | (pl.col("sortedness") > q3_all + 1.5 * iqr_all)).alias("is_outlier")
    )
    df_outliers_all = df_plot_all.filter(pl.col("is_outlier"))

    p_all = (
        ggplot(
            df_plot, aes(x="factor(capacity)", y="sortedness", fill="factor(capacity)")
        )
        + geom_jitter(
            data=df_outliers_all,
            mapping=aes(x="factor(capacity)", y="sortedness", color="factor(capacity)"),
            width=0.15,
            height=0,
            size=0.8,
            alpha=0.4,
            show_legend=False,
        )
        + stat_boxplot(geom="errorbar", width=0.2)
        + geom_boxplot(outlier_shape="")
        + labs(
            title="Sortedness Distribution by Capacity (All Seeds Combined)",
            x="Capacity",
            y="Proportion of sorted adjacent pairs",
        )
        + dark_theme((10, 6))
        + theme(legend_position="none", axis_text_x=element_text(angle=45, hjust=1))
    )
    save_plot(
        p_all, output_dir / "sortedness_by_capacity_all_seeds.png", width=10, height=6
    )


if __name__ == "__main__":
    main()
