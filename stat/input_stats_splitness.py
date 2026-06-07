import polars as pl
from plotnine import *
import sys
from collections import Counter
import argparse
from pathlib import Path
from common import get_theme, save_plot


def count_missing_values(df):
    res = (
        pl.DataFrame()
        .with_columns(df["seed"].unique())
        .join(pl.DataFrame().with_columns(df["capacity"].unique()), how="cross")
    )
    capacitys = df["capacity"].unique()
    num_holes = []
    for seed in df["seed"].unique():
        for capacity in capacitys:
            data = df.filter(pl.col("capacity") == capacity).filter(
                pl.col("seed") == seed
            )["data"]
            c = Counter(data.to_list())
            nb_holes = 0
            for i in range(capacity):
                if i not in c:
                    nb_holes += 1
            num_holes.append(nb_holes)
    return res.with_columns(pl.Series("num_holes", num_holes)).with_columns(
        (pl.col("num_holes") / pl.col("capacity")).alias("holes_ratio")
    )


def count_holes(df):
    res = (
        pl.DataFrame()
        .with_columns(df["seed"].unique())
        .join(pl.DataFrame().with_columns(df["capacity"].unique()), how="cross")
    )
    capacitys = df["capacity"].unique()
    num_holes = []
    for seed in df["seed"].unique():
        for capacity in capacitys:
            data = df.filter(pl.col("capacity") == capacity).filter(
                pl.col("seed") == seed
            )["data"]
            data = data.sort()
            # count the number of hole by counting the number of time
            # the diff between a number and the previous one is > 1
            num_holes.append(int((data.diff() > 1).sum()))
    return res.with_columns(pl.Series("num_holes", num_holes)).with_columns(
        (pl.col("num_holes") / pl.col("capacity")).alias("holes_ratio")
    )


def main():
    parser = argparse.ArgumentParser(description="Analyze input data splitness/holes.")
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

    missing_values = count_missing_values(df)
    output_dir = Path("benchmark_charts/input_analysis")

    # plot number of holes per capacity (missing value)
    p1 = (
        ggplot(missing_values, aes(x="num_holes", y="factor(capacity)"))
        + geom_point(size=1)
        + facet_wrap("~seed")
        + labs(title="number of holes per capacity (missing value)")
        + theme_func((14, 10))
    )
    save_plot(p1, output_dir / "splitness_1_num_holes_missing.png", width=14, height=10)

    # plot ratio of hole per capacity (missing value)
    p2 = (
        ggplot(missing_values, aes(x="holes_ratio", y="factor(capacity)"))
        + geom_point(size=1)
        + facet_wrap("~seed")
        + labs(title="ratio of holes per capacity (missing value)")
        + theme_func((14, 10))
    )
    save_plot(
        p2, output_dir / "splitness_2_holes_ratio_missing.png", width=14, height=10
    )

    non_consecutive = count_holes(df)

    # plot number of holes per capacity (non consecutive)
    p3 = (
        ggplot(non_consecutive, aes(x="num_holes", y="factor(capacity)"))
        + geom_point(size=1)
        + facet_wrap("~seed")
        + labs(title="number of holes per capacity (non consecutive)")
        + theme_func((14, 10))
    )
    save_plot(
        p3, output_dir / "splitness_3_num_holes_nonconsecutive.png", width=14, height=10
    )

    # plot ratio of hole per capacity (non consecutive)
    p4 = (
        ggplot(non_consecutive, aes(x="holes_ratio", y="factor(capacity)"))
        + geom_point(size=1)
        + facet_wrap("~seed")
        + labs(title="ratio of holes per capacity (non consecutive)")
        + theme_func((14, 10))
    )
    save_plot(
        p4,
        output_dir / "splitness_4_holes_ratio_nonconsecutive.png",
        width=14,
        height=10,
    )

    # plot ratio of hole per capacity with all seed (missing value)
    q1_mv = pl.col("holes_ratio").quantile(0.25).over(["capacity"])
    q3_mv = pl.col("holes_ratio").quantile(0.75).over(["capacity"])
    iqr_mv = q3_mv - q1_mv
    missing_values = missing_values.with_columns(
        ((pl.col("holes_ratio") < q1_mv - 1.5 * iqr_mv) | (pl.col("holes_ratio") > q3_mv + 1.5 * iqr_mv)).alias("is_outlier")
    )
    mv_outliers = missing_values.filter(pl.col("is_outlier"))

    p5 = (
        ggplot(
            missing_values,
            aes(x="factor(capacity)", y="holes_ratio", fill="factor(capacity)"),
        )
        + geom_jitter(
            data=mv_outliers,
            mapping=aes(x="factor(capacity)", y="holes_ratio", color="factor(capacity)"),
            width=0.15,
            height=0,
            size=0.8,
            alpha=0.4,
            show_legend=False,
        )
        + stat_boxplot(geom="errorbar", width=0.2)
        + geom_boxplot(outlier_shape="")
        + labs(title="ratio of holes per capacity all_seed (missing value)")
        + theme_func((10, 6))
        + theme(legend_position="none")
    )
    save_plot(
        p5,
        output_dir / "splitness_5_all_seeds_holes_ratio_missing.png",
        width=10,
        height=6,
    )

    # plot ratio of hole per capacity with all seed (non consecutive)
    q1_nc = pl.col("holes_ratio").quantile(0.25).over(["capacity"])
    q3_nc = pl.col("holes_ratio").quantile(0.75).over(["capacity"])
    iqr_nc = q3_nc - q1_nc
    non_consecutive = non_consecutive.with_columns(
        ((pl.col("holes_ratio") < q1_nc - 1.5 * iqr_nc) | (pl.col("holes_ratio") > q3_nc + 1.5 * iqr_nc)).alias("is_outlier")
    )
    nc_outliers = non_consecutive.filter(pl.col("is_outlier"))

    p6 = (
        ggplot(
            non_consecutive,
            aes(x="factor(capacity)", y="holes_ratio", fill="factor(capacity)"),
        )
        + geom_jitter(
            data=nc_outliers,
            mapping=aes(x="factor(capacity)", y="holes_ratio", color="factor(capacity)"),
            width=0.15,
            height=0,
            size=0.8,
            alpha=0.4,
            show_legend=False,
        )
        + stat_boxplot(geom="errorbar", width=0.2)
        + geom_boxplot(outlier_shape="")
        + labs(title="ratio of holes per capacity all_seed (non consecutive)")
        + theme_func((10, 6))
        + theme(legend_position="none")
    )
    save_plot(
        p6,
        output_dir / "splitness_6_all_seeds_holes_ratio_nonconsecutive.png",
        width=10,
        height=6,
    )


if __name__ == "__main__":
    main()

