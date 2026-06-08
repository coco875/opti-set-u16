import sys
import argparse
import polars as pl
from pathlib import Path
from plotnine import *
from common import dark_theme, save_plot, create_palette, Theme, get_theme

# ════════════════════════════════════════════════════════════════
# CONFIGURATION
# ════════════════════════════════════════════════════════════════

INPUT_FILE = "output.csv"
OUTPUT_DIR = Path("benchmark_charts")

# Theme Globals (will be updated dynamically based on CLI argument)
THEME_FUNC = dark_theme
TEXT_COL = "#E6EDF3"

# ════════════════════════════════════════════════════════════════
# DONNÉES
# ════════════════════════════════════════════════════════════════


def load_data(path: str) -> pl.DataFrame:
    """
    Lit le CSV de benchmar.

    Colonnes attendues :
        Scenario name, Type name, maximum capacity, fill, data, seed, time

    Parameters:
    -----------
    path: str
        le chemin du fichier CSV.

    Returns:
    --------
    pl.DataFrame
        Un joli DataFrame Polars propre.
    """

    try:
        df = pl.read_csv(
            path, has_header=True, ignore_errors=True, truncate_ragged_lines=True
        )
        df.columns = [c.strip() for c in df.columns]
    except FileNotFoundError:
        print(f"\033[31;1m[ERREUR] Fichier introuvable : {path}\033[0m")
        print(
            "  Fournir un CSV avec les colonnes : Scenario name, Type name, maximum capacity, fill, data, seed, time"
        )
        sys.exit(1)

    df = (
        df.with_columns(pl.col(pl.String).str.strip_chars())
        .with_columns(
            [
                pl.col("Scenario name").alias("scenario"),
                pl.col("Type name").alias("impl"),
                pl.col("maximum capacity").cast(pl.Int32).alias("max_capacity"),
                pl.col("time").cast(pl.Float64),
            ]
        )
        .select(["scenario", "impl", "max_capacity", "fill", "data", "seed", "time"])
        .drop_nulls()
    )

    if df.is_empty():
        print("\033[31;1m[ERREUR] Aucune ligne valide dans le fichier.\033[0m")
        sys.exit(1)

    return df


# ════════════════════════════════════════════════════════════════
# UTILS
# ════════════════════════════════════════════════════════════════


def create_stats(df: pl.DataFrame, group_by: list[str]) -> pl.DataFrame:
    """
    Calcule, pour chaque groupe défini par group_by :
      - mean_time  : moyenne du temps CPU
      - ci         : demi-largeur de l'intervalle de confiance à 95 %
      - ymin, ymax : bornes pour geom_errorbar

    Parameters:
    -----------
    df: pl.DataFrame
    group_by: list[str]
        Les groupes

    Returns:
    --------
    pl.DataFrame
        Un joli DataFrame Polars trié selon group_by.
    """

    return (
        df.group_by(group_by)
        .agg(
            [
                pl.col("time").mean().alias("mean_time"),
                pl.col("time").std().alias("std_time"),
                pl.col("time").count().alias("n"),
            ]
        )
        .with_columns(
            [
                (pl.col("std_time") / pl.col("n").cast(pl.Float64).sqrt() * 1.96).alias(
                    "ci"
                ),
            ]
        )
        .with_columns(
            [
                (pl.col("mean_time") - pl.col("ci")).alias("ymin"),
                (pl.col("mean_time") + pl.col("ci")).alias("ymax"),
            ]
        )
        .sort(group_by)
    )


def combine_plots(plots: list["ggplot"], cols: int) -> "ggplot":
    """
    Combine plusieurs objets ggplot en une grille avec plotnine.
    """
    if not plots:
        return None

    rows = []
    for i in range(0, len(plots), cols):
        chunk = plots[i : i + cols]
        row = chunk[0]
        for p in chunk[1:]:
            row = row | p
        rows.append(row)

    combined = rows[0]
    for r in rows[1:]:
        combined = combined / r

    return combined


# ════════════════════════════════════════════════════════════════
# Temps moyen global par implémentation
# ════════════════════════════════════════════════════════════════


def chart_global(
    df: pl.DataFrame, palette: dict[str, str], themef: Theme = None
) -> None:
    """
    Bar chart unique : temps moyen par implémentation, toutes conditions confondues.
    Les implémentations sont triées du plus rapide au plus lent.
    Intervalle de confiance à 95 % affiché en barres d'erreur.

    Parameters:
    -----------
    df: pl.DataFrame
    palette: dict[str, str]
        La palette des couleurs, pour chaque implémentation.
    themef: Theme
        Le thème du graphique (par défaut : dynamic global THEME_FUNC)
    """
    if themef is None:
        themef = THEME_FUNC

    stats = create_stats(df, ["impl"]).sort("mean_time")
    order = stats["impl"].to_list()

    plot = (
        ggplot(stats, aes(x="impl", y="mean_time", fill="impl"))
        + geom_col(width=0.7, show_legend=False)
        + geom_errorbar(
            aes(ymin="ymin", ymax="ymax"),
            width=0.35,
            color=TEXT_COL,
            alpha=0.6,
            size=0.6,
        )
        + scale_fill_manual(values=palette)
        + scale_x_discrete(limits=order)
        + scale_y_continuous(labels=lambda lst: [f"{v:,.0f}" for v in lst])
        + labs(
            title="Temps moyen global par implémentation",
            x="Implémentation",
            y="Temps moyen (cycles CPU)",
        )
        + themef((max(8, len(order) * 1.2), 5))
        + theme(axis_text_x=element_text(angle=30, hjust=1, size=8))
    )

    save_plot(
        plot,
        OUTPUT_DIR / "1_global_avg_time.png",
        width=max(8, len(order) * 1.2),
        height=5,
    )


# ════════════════════════════════════════════════════════════════
# Tous les scénarios en facettes (fichier combiné)
# ════════════════════════════════════════════════════════════════


def chart_all_scenarios_faceted(
    df: pl.DataFrame,
    palette: dict[str, str],
    scenarios: list[str],
    themef: Theme = None,
) -> None:
    """
    Un seul fichier avec un panel par scénario (facet_wrap).
    Chaque panel : bar chart impls * temps moyen, axes Y indépendants.

    Parameters:
    -----------
    df: pl.DataFrame
    palette: dict[str, str]
        La palette des couleurs, pour chaque implémentation.
    scenarios: list[str]
        Liste des noms de scénarios à afficher (ordre des facettes)
    themef: Theme
        Le thème du graphique (par défaut : dynamic global THEME_FUNC)
    """
    if themef is None:
        themef = THEME_FUNC

    stats = create_stats(df, ["scenario", "impl"])

    n_cols = 3
    n_rows = (len(scenarios) + n_cols - 1) // n_cols
    fig_w = 18
    fig_h = max(4, n_rows * 4)

    plot = (
        ggplot(stats, aes(x="impl", y="mean_time", fill="impl"))
        + geom_col(width=0.7, show_legend=False)
        + geom_errorbar(
            aes(ymin="ymin", ymax="ymax"),
            width=0.3,
            color=TEXT_COL,
            alpha=0.55,
            size=0.5,
        )
        + facet_wrap("~ scenario", scales="free_y", ncol=n_cols)
        + scale_fill_manual(values=palette)
        + scale_y_continuous(labels=lambda lst: [f"{v:,.0f}" for v in lst])
        + labs(
            title="Temps moyen par scénario et par implémentation",
            x="Implémentation",
            y="Temps moyen (cycles CPU)",
        )
        + themef((fig_w, fig_h))
        + theme(axis_text_x=element_text(angle=40, hjust=1, size=6))
    )

    save_plot(
        plot, OUTPUT_DIR / "2_all_scenarios_avg_time.png", width=fig_w, height=fig_h
    )


# ════════════════════════════════════════════════════════════════
# Un bar chart par scénario (fichiers individuels)
# ════════════════════════════════════════════════════════════════


def chart_scenario_individual(
    df: pl.DataFrame,
    palette: dict[str, str],
    scenarios: list[str],
    themef: Theme = None,
) -> list["ggplot"]:
    """
    Génère un fichier PNG par scénario dans son sous-dossier.
    Retourne la liste des objets ggplot créés (pour la composition).

    Parameters:
    -----------
    df: pl.DataFrame
    palette: dict[str, str]
        La palette des couleurs, pour chaque implémentation.
    scenarios: list[str]
        Liste des noms de scénarios à afficher (ordre des facettes)
    themef: Theme
        Le thème du graphique (par défaut : dynamic global THEME_FUNC)

    Returns:
    --------
    list[ggplot]
        Liste des objets ggplot générés.
    """
    if themef is None:
        themef = THEME_FUNC

    plots = []

    for scenario in scenarios:
        sub = df.filter(pl.col("scenario") == scenario)
        stats = create_stats(sub, ["impl"]).sort("mean_time")
        order = stats["impl"].to_list()
        n_impl = len(order)

        plot = (
            ggplot(stats, aes(x="impl", y="mean_time", fill="impl"))
            + geom_col(width=0.7, show_legend=False)
            + geom_errorbar(
                aes(ymin="ymin", ymax="ymax"),
                width=0.35,
                color=TEXT_COL,
                alpha=0.65,
                size=0.6,
            )
            + scale_fill_manual(values=palette)
            + scale_x_discrete(limits=order)
            + scale_y_continuous(labels=lambda lst: [f"{v:,.0f}" for v in lst])
            + labs(
                title=scenario,
                x="Implémentation",
                y="Temps moyen (cycles CPU)",
            )
            + themef((max(7, n_impl * 1.1), 5))
            + theme(axis_text_x=element_text(angle=30, hjust=1, size=8))
        )

        path = OUTPUT_DIR / scenario / "1_avg_time.png"
        save_plot(plot, path, width=max(7, n_impl * 1.1), height=5)
        plots.append(plot)

    return plots


# ════════════════════════════════════════════════════════════════
# Décomposition par capacité (un fichier par scénario)
# ════════════════════════════════════════════════════════════════


def chart_capacity_breakdown(
    df: pl.DataFrame,
    palette: dict[str, str],
    scenarios: list[str],
    themef: Theme = None,
) -> list["ggplot"]:
    """
    Grouped bar chart : capacité maximale sur l'axe X, implémentation en couleur.
    Les barres sont côte à côte grâce à position_dodge.
    Révèle comment chaque structure de données passe à l'échelle.

    Parameters:
    -----------
    df: pl.DataFrame
    palette: dict[str, str]
        La palette des couleurs, pour chaque implémentation.
    scenarios: list[str]
        Liste des noms de scénarios à afficher (ordre des facettes)
    themef: Theme
        Le thème du graphique (par défaut : dynamic global THEME_FUNC)

    Returns:
    --------
    list[ggplot]
        Liste des objets ggplot générés.
    """
    if themef is None:
        themef = THEME_FUNC

    plots = []

    for scenario in scenarios:
        sub = df.filter(pl.col("scenario") == scenario)

        stats = (
            create_stats(sub, ["impl", "max_capacity"])
            .with_columns(pl.col("max_capacity").cast(pl.Utf8).alias("cap_str"))
            .sort(["max_capacity", "impl"])
        )

        cap_order = (
            stats.select(["max_capacity", "cap_str"])
            .unique()
            .sort("max_capacity")["cap_str"]
            .to_list()
        )

        n_cap = len(cap_order)
        n_impl = stats["impl"].n_unique()
        fig_w = max(9, n_cap * 1.4 + 2)

        plot = (
            ggplot(stats, aes(x="cap_str", y="mean_time", fill="impl"))
            + geom_col(position=position_dodge(width=0.85), width=0.8)
            + geom_errorbar(
                aes(ymin="ymin", ymax="ymax"),
                position=position_dodge(width=0.85),
                width=0.25,
                color=TEXT_COL,
                alpha=0.45,
                size=0.4,
            )
            + scale_fill_manual(values=palette, name="Implémentation")
            + scale_x_discrete(limits=cap_order)
            + scale_y_continuous(labels=lambda lst: [f"{v:,.0f}" for v in lst])
            + labs(
                title=scenario,
                x="Capacité maximale",
                y="Temps moyen (cycles CPU)",
            )
            + themef((fig_w, 5))
            + theme(
                axis_text_x=element_text(angle=20, hjust=1, size=8),
                legend_position="right",
            )
        )

        path = OUTPUT_DIR / scenario / "2_capacity_breakdown.png"
        save_plot(plot, path, width=fig_w, height=5)
        plots.append(plot)

    return plots


# ════════════════════════════════════════════════════════════════
# Passage à l'échelle (Line Plots)
# ════════════════════════════════════════════════════════════════


def chart_time_scaling(
    df: pl.DataFrame,
    palette: dict[str, str],
    themef: Theme = None,
) -> None:
    """
    Génère un graphique linéaire montrant le passage à l'échelle (temps vs capacité maximale)
    de toutes les implémentations pour chaque scénario.
    """
    if themef is None:
        themef = THEME_FUNC

    stats = create_stats(df, ["scenario", "impl", "max_capacity"]).sort(
        ["scenario", "impl", "max_capacity"]
    )

    n_cols = 3
    scenarios = sorted(df["scenario"].unique().to_list())
    n_rows = (len(scenarios) + n_cols - 1) // n_cols
    fig_w = 18
    fig_h = max(4, n_rows * 4)

    plot = (
        ggplot(
            stats,
            aes(x="factor(max_capacity)", y="mean_time", color="impl", group="impl"),
        )
        + geom_line(size=0.8)
        + geom_point(size=1.5)
        + facet_wrap("~ scenario", scales="free_y", ncol=n_cols)
        + scale_color_manual(values=palette, name="Implémentation")
        + scale_y_continuous(labels=lambda lst: [f"{v:,.0f}" for v in lst])
        + labs(
            title="Courbes de passage à l'échelle (Temps moyen * Capacité maximale)",
            x="Capacité maximale",
            y="Temps moyen (cycles CPU)",
        )
        + themef((fig_w, fig_h))
        + theme(
            axis_text_x=element_text(angle=30, hjust=1, size=8),
            legend_position="right",
        )
    )

    save_plot(
        plot,
        OUTPUT_DIR / "4_all_scenarios_time_scaling.png",
        width=fig_w,
        height=fig_h,
    )

    # Génère également un graphique individuel par scénario
    for scenario in scenarios:
        sub_stats = stats.filter(pl.col("scenario") == scenario)

        scen_w = 12
        scen_h = 7

        scen_plot = (
            ggplot(
                sub_stats,
                aes(x="factor(max_capacity)", y="mean_time", color="impl", group="impl"),
            )
            + geom_line(size=0.8)
            + geom_point(size=1.5)
            + scale_color_manual(values=palette, name="Implémentation")
            + scale_y_continuous(labels=lambda lst: [f"{v:,.0f}" for v in lst])
            + labs(
                title=f"Courbe de passage à l'échelle - {scenario}",
                x="Capacité maximale",
                y="Temps moyen (cycles CPU)",
            )
            + themef((scen_w, scen_h))
            + theme(
                axis_text_x=element_text(angle=30, hjust=1, size=8),
                legend_position="right",
            )
        )

        save_plot(
            scen_plot,
            OUTPUT_DIR / scenario / "4_time_scaling.png",
            width=scen_w,
            height=scen_h,
        )


# ════════════════════════════════════════════════════════════════
# Boxplots de Distribution et Variabilité des temps
# ════════════════════════════════════════════════════════════════


def build_plot(
    df_boxplot: pl.DataFrame,
    df_jitter: pl.DataFrame,
    palette: dict[str, str],
    fig_w: int,
    fig_h: int,
    themef: Theme = None,
    *,
    faceted: bool
) -> "ggplot":
    """
    Génère un boxplot montrant la distribution des temps d'exécution pour un df donné
    """

    impls_ordered = sorted(df_boxplot["impl"].unique().to_list())
    x_order  = [f"{i}::{v}" for i in impls_ordered for v in ("box", "jitter")]
    x_labels = {k: (k.split("::")[0] if k.endswith("::box") else "") for k in x_order}

    df_combined = pl.concat([
        df_boxplot.with_columns(pl.lit("box").alias("view")),
        df_jitter.with_columns(pl.lit("jitter").alias("view")),
    ]).with_columns(
        (pl.col("impl") + "::" + pl.col("view")).alias("x_key")
    )

    plot = (
        ggplot(df_combined, aes(x="x_key", y="time"))

        + stat_boxplot(
            data=df_combined.filter(pl.col("view") == "box"),
            geom="errorbar", width=0.3,
        )
        + geom_boxplot(
            data=df_combined.filter(pl.col("view") == "box"),
            mapping=aes(fill="impl"),
            outlier_shape="", show_legend=False, width=0.8,
        )
        + geom_jitter(
            data=df_combined.filter(pl.col("view") == "jitter"),
            mapping=aes(color="impl"),
            width=0.25, height=0, size=0.8, alpha=0.4, show_legend=False,
        )
        + scale_x_discrete(
            limits=x_order,
            labels=[x_labels[k] for k in x_order],
        )
        + scale_fill_manual(values=palette)
        + scale_color_manual(values=palette)
        + scale_y_continuous(labels=lambda lst: [f"{v:,.0f}" for v in lst])
        + labs(x="", y="Temps d'exécution (cycles CPU)")
        + themef((fig_w, fig_h))
        + theme(axis_text_x=element_text(size=12))
        + coord_flip()
    )

    if faceted:
        plot = plot + facet_wrap("~ scenario", scales="free", ncol=2) + labs(
            title="Distribution des temps d'exécution par implémentation et scénario"
        )

    return plot

def chart_time_distribution(
    df: pl.DataFrame,
    palette: dict[str, str],
    themef: Theme = None,
) -> None:
    """
    Génère un boxplot montrant la distribution complète des temps d'exécution
    pour chaque implémentation sous chaque scénario (analyse de variance et stabilité).
    """
    if themef is None:
        themef = THEME_FUNC

    q1  = pl.col("time").quantile(0.25).over(["scenario", "impl"])
    q3  = pl.col("time").quantile(0.75).over(["scenario", "impl"])
    iqr = q3 - q1
    df  = df.with_columns(
        ((pl.col("time") < q1 - 1.5 * iqr) | (pl.col("time") > q3 + 1.5 * iqr))
        .alias("is_inlier")
    )
    df_inliers = df.filter(~pl.col("is_inlier"))

    scenarios = sorted(df["scenario"].unique().to_list())
    n_rows    = (len(scenarios) + 1) // 2
    fig_w, fig_h = 20, max(10, n_rows * 9.5)

    plot = build_plot(df, df_inliers, palette, fig_w, fig_h, themef, faceted=True)
    save_plot(plot, OUTPUT_DIR / "5_all_scenarios_time_distribution.png", width=fig_w, height=fig_h)

    for scenario in scenarios:
        sub          = df.filter(pl.col("scenario") == scenario)
        sub_outliers = df_inliers.filter(pl.col("scenario") == scenario)

        n_impl       = sub["impl"].n_unique()
        scen_w       = 12
        scen_h       = max(6, n_impl * 0.7)

        scen_plot = build_plot(sub, sub_outliers, palette, scen_w, scen_h, themef, faceted=False)
        scen_plot = scen_plot + labs(title=f"Temps d'exécution par implémentation — {scenario}")

        save_plot(scen_plot, OUTPUT_DIR / scenario / "5_time_distribution.png", width=scen_w, height=scen_h)


# ════════════════════════════════════════════════════════════════
# POINT D'ENTRÉE
# ════════════════════════════════════════════════════════════════


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Génère des graphiques à partir des benchmarks d'opti-set-u16."
    )
    parser.add_argument(
        "input_file",
        nargs="?",
        default=INPUT_FILE,
        help=f"Chemin du fichier CSV de données (par défaut : {INPUT_FILE})",
    )
    parser.add_argument(
        "--whitelist",
        "-w",
        nargs="+",
        help="Liste d'implémentations à inclure sur les graphiques (les autres seront exclues).",
    )
    parser.add_argument(
        "--blacklist",
        "-b",
        nargs="+",
        help="Liste d'implémentations à exclure des graphiques.",
    )
    parser.add_argument(
        "--output-dir",
        "-o",
        help="Dossier de sortie alternatif pour sauvegarder les graphiques (par défaut : benchmark_charts).",
    )
    parser.add_argument(
        "--theme",
        choices=["dark", "light"],
        default="dark",
        help="Thème des graphiques : 'dark' (sombre) ou 'light' (clair) (par défaut : 'dark').",
    )

    args = parser.parse_args()

    global OUTPUT_DIR, THEME_FUNC, TEXT_COL
    if args.output_dir:
        OUTPUT_DIR = Path(args.output_dir)

    theme_f, text_c = get_theme(args.theme)
    THEME_FUNC = theme_f
    TEXT_COL = text_c

    df = load_data(args.input_file)

    available_impls = sorted(df["impl"].unique().to_list())

    if args.whitelist:
        invalid_impls = [
            impl for impl in args.whitelist if impl not in available_impls
        ]
        if invalid_impls:
            print(
                f"\033[33;1m[AVERTISSEMENT] Ces implémentations de la whitelist n'existent pas dans les données : {invalid_impls}\033[0m"
            )
            print(f"Disponibles : {available_impls}")

        whitelist_set = set(args.whitelist)
        df = df.filter(pl.col("impl").is_in(whitelist_set))
        if df.is_empty():
            print(
                "\033[31;1m[ERREUR] Aucune des implémentations de la whitelist n'est présente dans les données.\033[0m"
            )
            sys.exit(1)

    if args.blacklist:
        invalid_impls = [
            impl for impl in args.blacklist if impl not in available_impls
        ]
        if invalid_impls:
            print(
                f"\033[33;1m[AVERTISSEMENT] Ces implémentations de la blacklist n'existent pas dans les données : {invalid_impls}\033[0m"
            )

        blacklist_set = set(args.blacklist)
        df = df.filter(~pl.col("impl").is_in(blacklist_set))
        if df.is_empty():
            print(
                "\033[31;1m[ERREUR] Toutes les implémentations ont été exclues par la blacklist.\033[0m"
            )
            sys.exit(1)

    df = df.with_columns(pl.col("impl"))

    scenarios = sorted(df["scenario"].unique().to_list())
    impls = sorted(df["impl"].unique().to_list())
    palette = create_palette(impls)

    print(
        f"\nChargé {len(df):,} lignes  |  "
        f"{len(scenarios)} scénarios  |  "
        f"{len(impls)} implémentations\n"
    )
    print("Génération des graphiques…\n")

    chart_global(df, palette)
    chart_all_scenarios_faceted(df, palette, scenarios)
    chart_scenario_individual(df, palette, scenarios)
    chart_capacity_breakdown(df, palette, scenarios)
    chart_time_scaling(df, palette)
    chart_time_distribution(df, palette)


    print(f"\nTous les graphiques sauvegardés dans : {OUTPUT_DIR.resolve()}/\n")


if __name__ == "__main__":
    main()
