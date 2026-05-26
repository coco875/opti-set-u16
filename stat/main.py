import sys
import polars as pl
from pathlib import Path
from plotnine import *
from common import dark_theme, save_plot, create_palette, Theme, _TEXT_COL

# ════════════════════════════════════════════════════════════════
# CONFIGURATION
# ════════════════════════════════════════════════════════════════

INPUT_FILE = "output.csv"
OUTPUT_DIR = Path("benchmark_charts")

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
    df: pl.DataFrame, palette: dict[str, str], themef: Theme = dark_theme
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
    themef: Theme = dark_theme
        Le thème du graphique
    """

    stats = create_stats(df, ["impl"]).sort("mean_time")
    order = stats["impl"].to_list()

    plot = (
        ggplot(stats, aes(x="impl", y="mean_time", fill="impl"))
        + geom_col(width=0.7, show_legend=False)
        + geom_errorbar(
            aes(ymin="ymin", ymax="ymax"),
            width=0.35,
            color=_TEXT_COL,
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
    themef: Theme = dark_theme,
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
    themef: Theme = dark_theme
        Le thème du graphique
    """

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
            color=_TEXT_COL,
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
    themef: Theme = dark_theme,
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
    themef: Theme = dark_theme
        Le thème du graphique

    Returns:
    --------
    list[ggplot]
        Liste des objets ggplot générés.
    """

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
                color=_TEXT_COL,
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
    themef: Theme = dark_theme,
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
    themef: Theme = dark_theme
        Le thème du graphique

    Returns:
    --------
    list[ggplot]
        Liste des objets ggplot générés.
    """

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
                color=_TEXT_COL,
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
    themef: Theme = dark_theme,
) -> None:
    """
    Génère un graphique linéaire montrant le passage à l'échelle (temps vs capacité maximale)
    de toutes les implémentations pour chaque scénario.
    """
    stats = create_stats(df, ["scenario", "impl", "max_capacity"]).sort(["scenario", "impl", "max_capacity"])
    
    n_cols = 3
    scenarios = sorted(df["scenario"].unique().to_list())
    n_rows = (len(scenarios) + n_cols - 1) // n_cols
    fig_w = 18
    fig_h = max(4, n_rows * 4)
    
    plot = (
        ggplot(stats, aes(x="factor(max_capacity)", y="mean_time", color="impl", group="impl"))
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


# ════════════════════════════════════════════════════════════════
# Boxplots de Distribution et Variabilité des temps
# ════════════════════════════════════════════════════════════════


def chart_time_distribution(
    df: pl.DataFrame,
    palette: dict[str, str],
    themef: Theme = dark_theme,
) -> None:
    """
    Génère un boxplot montrant la distribution complète des temps d'exécution
    pour chaque implémentation sous chaque scénario (analyse de variance et stabilité).
    """
    n_cols = 2
    scenarios = sorted(df["scenario"].unique().to_list())
    n_rows = (len(scenarios) + n_cols - 1) // n_cols
    fig_w = 20
    fig_h = max(5, n_rows * 5.5)
    
    plot = (
        ggplot(df, aes(x="impl", y="time", fill="impl"))
        + geom_boxplot(outlier_size=0.8, outlier_alpha=0.4, show_legend=False)
        + facet_wrap("~ scenario", scales="free_y", ncol=n_cols)
        + scale_fill_manual(values=palette)
        + scale_y_continuous(labels=lambda lst: [f"{v:,.0f}" for v in lst])
        + labs(
            title="Distribution et variabilité des temps d'exécution par scénario",
            x="Implémentation",
            y="Temps d'exécution (cycles CPU)",
        )
        + themef((fig_w, fig_h))
        + theme(
            axis_text_x=element_text(angle=40, hjust=1, size=7),
        )
    )
    
    save_plot(
        plot,
        OUTPUT_DIR / "5_all_scenarios_time_distribution.png",
        width=fig_w,
        height=fig_h,
    )


# ════════════════════════════════════════════════════════════════
# POINT D'ENTRÉE
# ════════════════════════════════════════════════════════════════


def main() -> None:
    input_file = sys.argv[1] if len(sys.argv) > 1 else INPUT_FILE
    df = load_data(input_file)

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
    scenario_plots = chart_scenario_individual(df, palette, scenarios)
    capacity_plots = chart_capacity_breakdown(df, palette, scenarios)
    chart_time_scaling(df, palette)
    chart_time_distribution(df, palette)

    print("\nAssemblage des images combinées…")

    if scenario_plots:
        cols = min(len(scenario_plots), 3)
        combined = combine_plots(scenario_plots, cols)
        out = OUTPUT_DIR / "3_combined_scenarios_avg_time"
        n_rows = (len(scenario_plots) + cols - 1) // cols
        
        # Dynamically calculate width based on max implementation count to avoid overlapping labels
        max_impl = max(len(p.data["impl"].unique()) for p in scenario_plots)
        col_width = max(7.0, max_impl * 1.1)
        total_width = cols * col_width
        total_height = n_rows * 5.0
        
        combined.save(str(out.with_suffix(".png")), width=total_width, height=total_height, verbose=False, limitsize=False)
        combined.save(str(out.with_suffix(".svg")), width=total_width, height=total_height, verbose=False, limitsize=False)
        print(f"\033[32m  ✓  {out.with_suffix('.png')}  &  {out.with_suffix('.svg')}\033[0m")

    if capacity_plots:
        cols = min(len(capacity_plots), 2)
        
        # Hide the legend on all plots except the last one to save space and prevent collisions
        combined_capacity_plots = []
        for idx, p in enumerate(capacity_plots):
            if idx < len(capacity_plots) - 1:
                combined_capacity_plots.append(p + theme(legend_position="none"))
            else:
                combined_capacity_plots.append(p + theme(legend_position="right"))
                
        combined = combine_plots(combined_capacity_plots, cols)
        out = OUTPUT_DIR / "3_combined_scenarios_capacity"
        n_rows = (len(capacity_plots) + cols - 1) // cols
        
        # Dynamically calculate width based on max capacity count to avoid overlapping labels
        max_cap = max(len(p.data["max_capacity"].unique()) for p in capacity_plots)
        col_width = max(9.0, max_cap * 1.4 + 2.0)
        
        # Add extra width for the single legend on the right
        total_width = cols * col_width + 3.0
        total_height = n_rows * 5.0
        
        combined.save(str(out.with_suffix(".png")), width=total_width, height=total_height, verbose=False, limitsize=False)
        combined.save(str(out.with_suffix(".svg")), width=total_width, height=total_height, verbose=False, limitsize=False)
        print(f"\033[32m  ✓  {out.with_suffix('.png')}  &  {out.with_suffix('.svg')}\033[0m")

    print(f"\nTous les graphiques sauvegardés dans : {OUTPUT_DIR.resolve()}/\n")


if __name__ == "__main__":
    main()

