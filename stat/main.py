import sys
import colorsys
import polars   as     pl
from   typing   import Callable
from   pathlib  import Path
from   PIL      import Image
from   plotnine import *

Theme = Callable[[tuple[float, float]], 'plotnine.themes.theme']

# ════════════════════════════════════════════════════════════════
# CONFIGURATION
# ════════════════════════════════════════════════════════════════

INPUT_FILE = "output.csv"
OUTPUT_DIR = Path("benchmark_charts")

_DARK_BG   = "#0E1117"
_PANEL_BG  = "#161B22"
_TEXT_COL  = "#E6EDF3"
_GRID_COL  = "#21262D"

# ════════════════════════════════════════════════════════════════
# THÈME
# ════════════════════════════════════════════════════════════════

def create_palette(impls: list[str], first: str = "#FF8A65") -> dict[str, str] :
    """
    Crée une palette en découpant l'espace des teintes de manière régulière.

    Parameters:
    -----------
    impls : list[str]
        Liste des implémentations pour lesquelles générer une couleur.
    first : str
        La première couleur de la palette, permet de déterminer la saturation et la luminance.

    Returns:
    --------
    dict[str, str]
        La couleur pour chaque implémentation.
    """

    first = first.lstrip('#')
    r, g, b = tuple(int(first[i:i+2], 16) / 255.0 for i in (0, 2, 4))
    h, s, v = colorsys.rgb_to_hsv(r, g, b)

    n = len(impls)
    step = 360.0 / n if n > 0 else 0

    result = {}
    for i, impl in enumerate(impls):
        hue = (h * 360 + i * step) % 360
        r, g, b = colorsys.hsv_to_rgb(hue / 360.0, s, v)
        hex_color = f"#{int(r*255):02x}{int(g*255):02x}{int(b*255):02x}"
        result[impl] = hex_color
    
    return result

dark_theme: Theme = lambda figure_size=(12, 6): (
        theme_void()
        + theme(
            figure_size=figure_size,
            plot_background=element_rect(fill=_DARK_BG, color=_DARK_BG),
            panel_background=element_rect(fill=_PANEL_BG, color=_PANEL_BG),
            panel_grid_major_y=element_line(color=_GRID_COL, size=0.6, linetype="dashed"),
            panel_grid_minor=element_blank(),
            axis_text=element_text(color=_TEXT_COL, size=8),
            axis_title=element_text(color=_TEXT_COL, size=9),
            axis_ticks=element_line(color=_GRID_COL),
            plot_title=element_text(color=_TEXT_COL, size=12, weight="bold", margin={"b": 10}),
            legend_background=element_rect(fill=_PANEL_BG),
            legend_key=element_rect(fill=_PANEL_BG),
            legend_text=element_text(color=_TEXT_COL, size=8),
            legend_title=element_text(color=_TEXT_COL, size=9, weight="bold"),
            strip_background=element_rect(fill=_PANEL_BG),
            strip_text=element_text(color=_TEXT_COL, size=9, weight="bold"),
        )
    )

# ════════════════════════════════════════════════════════════════
# DONNÉES
# ════════════════════════════════════════════════════════════════

def load_data(path: str) -> pl.DataFrame :
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
        df = pl.read_csv(path, has_header=True, ignore_errors=True, truncate_ragged_lines=True)
    except FileNotFoundError:
        print(f"\033[31;1m[ERREUR] Fichier introuvable : {path}\033[0m")
        print("  Fournir un CSV avec les colonnes : Scenario name, Type name, maximum capacity, fill, data, seed, time")
        sys.exit(1)

    df = (
        df
        .with_columns([
            pl.col("Scenario name").str.strip_chars().alias("scenario"),
            pl.col("Type name").str.strip_chars().alias("impl"),
            pl.col("maximum capacity").cast(pl.Int32).alias("max_capacity"),
            pl.col("time").cast(pl.Float64),
        ])
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

def create_stats(df: pl.DataFrame, group_by: list[str]) -> pl.DataFrame :
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
        df
        .group_by(group_by)
        .agg([
            pl.col("time").mean().alias("mean_time"),
            pl.col("time").std().alias("std_time"),
            pl.col("time").count().alias("n"),
        ])
        .with_columns([
            (pl.col("std_time") / pl.col("n").cast(pl.Float64).sqrt() * 1.96).alias("ci"),
        ])
        .with_columns([
            (pl.col("mean_time") - pl.col("ci")).alias("ymin"),
            (pl.col("mean_time") + pl.col("ci")).alias("ymax"),
        ])
        .sort(group_by)
    )

def save_plot(plot, path: Path, width: float = 12, height: float = 6, dpi: int = 150) :
    """
    Sauvegarde un objet plotnine sur disque et affiche le chemin.
    limitsize=False est nécessaire quand width ou height dépasse 25 pouces
    (cas des graphiques avec de nombreuses implémentations ou capacités).
    """

    path.parent.mkdir(parents=True, exist_ok=True)
    plot.save(str(path), width=width, height=height, dpi=dpi, verbose=False, limitsize=False)
    print(f"\033[32m  ✓  {path}\033[0m")

def combine_image(image_paths: list[Path], cols: int, gap: int = 10) -> Image.Image :
    """
    Assemble plusieurs PNG en une grille avec Pillow.
    Les images sont centrées dans des cellules de taille uniforme.
    """

    imgs = [Image.open(p) for p in image_paths]
    rows   = (len(imgs) + cols - 1) // cols
    cell_w = max(im.size[0] for im in imgs)
    cell_h = max(im.size[1] for im in imgs)
    total_w = cols * cell_w + (cols - 1) * gap
    total_h = rows * cell_h + (rows - 1) * gap
    canvas = Image.new("RGB", (total_w, total_h), (14, 17, 23))
    for idx, im in enumerate(imgs):
        r, c = divmod(idx, cols)
        canvas.paste(im, (c * (cell_w + gap), r * (cell_h + gap)))
    return canvas

# ════════════════════════════════════════════════════════════════
# Temps moyen global par implémentation
# ════════════════════════════════════════════════════════════════

def chart_global(df: pl.DataFrame, palette: dict[str, str], themef: Theme = dark_theme) -> None :
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
            width=0.35, color=_TEXT_COL, alpha=0.6, size=0.6,
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

    save_plot(plot, OUTPUT_DIR / "1_global_avg_time.png",
              width=max(8, len(order) * 1.2), height=5)

# ════════════════════════════════════════════════════════════════
# Tous les scénarios en facettes (fichier combiné)
# ════════════════════════════════════════════════════════════════

def chart_all_scenarios_faceted(df: pl.DataFrame, palette: dict[str, str], scenarios: list[str], themef: Theme = dark_theme) -> None:
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
    fig_w  = 18
    fig_h  = max(4, n_rows * 4)

    plot = (
        ggplot(stats, aes(x="impl", y="mean_time", fill="impl"))
        + geom_col(width=0.7, show_legend=False)
        + geom_errorbar(
            aes(ymin="ymin", ymax="ymax"),
            width=0.3, color=_TEXT_COL, alpha=0.55, size=0.5,
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

    save_plot(plot, OUTPUT_DIR / "2_all_scenarios_avg_time.png",
              width=fig_w, height=fig_h)

# ════════════════════════════════════════════════════════════════
# Un bar chart par scénario (fichiers individuels)
# ════════════════════════════════════════════════════════════════

def chart_scenario_individual(df: pl.DataFrame, palette: dict[str, str], scenarios: list[str], themef: Theme = dark_theme) -> list[Path]:
    """
    Génère un fichier PNG par scénario dans son sous-dossier.
    Retourne la liste des chemins créés (pour l'assemblage Pillow).

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
    list[Path]
        Liste des chemins absolus ou relatifs des fichiers PNG générés.
    """

    paths = []

    for scenario in scenarios:
        sub   = df.filter(pl.col("scenario") == scenario)
        stats = create_stats(sub, ["impl"]).sort("mean_time")
        order = stats["impl"].to_list()
        n_impl = len(order)

        plot = (
            ggplot(stats, aes(x="impl", y="mean_time", fill="impl"))
            + geom_col(width=0.7, show_legend=False)
            + geom_errorbar(
                aes(ymin="ymin", ymax="ymax"),
                width=0.35, color=_TEXT_COL, alpha=0.65, size=0.6,
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
        paths.append(path)

    return paths

# ════════════════════════════════════════════════════════════════
# Décomposition par capacité (un fichier par scénario)
# ════════════════════════════════════════════════════════════════

def chart_capacity_breakdown(df: pl.DataFrame, palette: dict[str, str], scenarios: list[str], themef: Theme = dark_theme) -> list[Path]:
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
    list[Path]
        Liste des chemins absolus ou relatifs des fichiers PNG générés.
    """
    
    paths = []

    for scenario in scenarios:
        sub = df.filter(pl.col("scenario") == scenario)

        stats = (
            create_stats(sub, ["impl", "max_capacity"])
            .with_columns(
                pl.col("max_capacity").cast(pl.Utf8).alias("cap_str")
            )
            .sort(["max_capacity", "impl"])
        )

        cap_order = (
            stats
            .select(["max_capacity", "cap_str"])
            .unique()
            .sort("max_capacity")["cap_str"]
            .to_list()
        )

        n_cap  = len(cap_order)
        n_impl = stats["impl"].n_unique()
        fig_w  = max(9, n_cap * 1.4 + 2)

        plot = (
            ggplot(stats, aes(x="cap_str", y="mean_time", fill="impl"))
            + geom_col(position=position_dodge(width=0.85), width=0.8)
            + geom_errorbar(
                aes(ymin="ymin", ymax="ymax"),
                position=position_dodge(width=0.85),
                width=0.25, color=_TEXT_COL, alpha=0.45, size=0.4,
            )
            + scale_fill_manual(values=palette, name="Implémentation")
            + scale_x_discrete(limits=cap_order)
            + scale_y_continuous(labels=lambda lst: [f"{v:,.0f}" for v in lst])
            + labs(
                title=f"{scenario} — Temps moyen * Capacité maximale",
                x="Capacité maximale",
                y="Temps moyen (cycles CPU)",
            )
            + themef((fig_w, 5))
            + theme(
                axis_text_x    = element_text(angle=20, hjust=1, size=8),
                legend_position = "right",
            )
        )

        path = OUTPUT_DIR / scenario / "2_capacity_breakdown.png"
        save_plot(plot, path, width=fig_w, height=5)
        paths.append(path)

    return paths

def chart_capacity_breakdown_by_impl(df: pl.DataFrame, palette: dict[str, str], scenarios: list[str], themef: Theme = dark_theme) -> list[Path]:
    """
    Pour chaque implémentation, assemble en une seule image combinée
    les bar charts temps vs capacité pour chaque scénario.
    Aucun fichier intermédiaire n'est écrit sur disque.

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
    list[Path]
        Liste des chemins absolus ou relatifs des fichiers PNG générés.
    """
    
    paths: dict[str, list[Path]] = {}

    for scenario in scenarios:
        sub = df.filter(pl.col("scenario") == scenario)
        impls_in_scenario = sorted(sub["impl"].unique().to_list())
        paths[scenario] = []

        for impl in impls_in_scenario:
            sub_impl = sub.filter(pl.col("impl") == impl)
            stats = (
                create_stats(sub_impl, ["max_capacity"])
                .with_columns(
                    pl.col("max_capacity").cast(pl.Utf8).alias("cap_str")
                )
                .sort("max_capacity")
            )

            cap_order = (
                stats
                .select(["max_capacity", "cap_str"])
                .unique()
                .sort("max_capacity")["cap_str"]
                .to_list()
            )
            n_cap = len(cap_order)
            fig_w = max(9, n_cap * 1.4 + 2)

            plot = (
                ggplot(stats, aes(x="cap_str", y="mean_time"))
                + geom_col(fill=palette.get(impl, "#888888"), width=0.7)
                + geom_errorbar(
                    aes(ymin="ymin", ymax="ymax"),
                    width=0.35, color=_TEXT_COL, alpha=0.6, size=0.6,
                )
                + scale_x_discrete(limits=cap_order)
                + scale_y_continuous(labels=lambda lst: [f"{v:,.0f}" for v in lst])
                + labs(
                    title=f"{scenario} · {impl}",
                    x="Capacité maximale",
                    y="Temps moyen (cycles CPU)",
                )
                + themef((fig_w, 5))
                + theme(axis_text_x=element_text(angle=20, hjust=1, size=8))
            )

            safe = impl.replace("/", "_").replace(" ", "_")
            path = OUTPUT_DIR / "impls" / scenario / f"2_cap_{safe}.png"
            save_plot(plot, path, width=fig_w, height=5)
            paths[scenario].append(path)

    return paths

# ════════════════════════════════════════════════════════════════
# POINT D'ENTRÉE
# ════════════════════════════════════════════════════════════════

def main() -> None :
    df = load_data(INPUT_FILE)

    scenarios = sorted(df["scenario"].unique().to_list())
    impls     = sorted(df["impl"].unique().to_list())
    palette = create_palette(impls)

    print(
        f"\nChargé {len(df):,} lignes  |  "
        f"{len(scenarios)} scénarios  |  "
        f"{len(impls)} implémentations\n"
    )
    print("Génération des graphiques…\n")

    chart_global(df, palette)
    chart_all_scenarios_faceted(df, palette, scenarios)
    scenario_paths          = chart_scenario_individual(df, palette, scenarios)
    capacity_paths          = chart_capacity_breakdown(df, palette, scenarios)
    capacity_paths_by_impl  = chart_capacity_breakdown_by_impl(df, palette, scenarios)

    print("\nAssemblage des images combinées…")

    if scenario_paths:
        cols = min(len(scenario_paths), 3)
        combined = combine_image(scenario_paths, cols)
        out = OUTPUT_DIR / "3_combined_scenarios_avg_time.png"
        combined.save(str(out))
        print(f"\033[32m  ✓  {out}\033[0m")

    if capacity_paths:
        cols = min(len(capacity_paths), 2)
        combined = combine_image(capacity_paths, cols)
        out = OUTPUT_DIR / "3_combined_scenarios_capacity.png"
        combined.save(str(out))
        print(f"\033[32m  ✓  {out}\033[0m")
    
    for scenario, cpaths in capacity_paths_by_impl.items():
        if not cpaths:
            continue
        cols = min(len(cpaths), 3)
        combined = combine_image(cpaths, cols)
        safe = scenario.replace("/", "_").replace(" ", "_")
        out = OUTPUT_DIR / f"3_cap_{safe}.png"
        combined.save(str(out))
        print(f"\033[32m  ✓  {out}\033[0m")

    print(f"\nTous les graphiques sauvegardés dans : {OUTPUT_DIR.resolve()}/\n")

if __name__ == "__main__":
    main()