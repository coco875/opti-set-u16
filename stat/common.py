import colorsys
from pathlib import Path
from typing import Callable
from plotnine import *

Theme = Callable[[tuple[float, float]], "plotnine.themes.theme"]

# ════════════════════════════════════════════════════════════════
# THÈME CONFIGURATION ET CONSTANTES
# ════════════════════════════════════════════════════════════════

_DARK_BG = "#0E1117"
_PANEL_BG = "#161B22"
_TEXT_COL = "#E6EDF3"
_GRID_COL = "#21262D"

_LIGHT_BG = "#FFFFFF"
_LIGHT_PANEL_BG = "#F6F8FA"
_LIGHT_TEXT_COL = "#24292F"
_LIGHT_GRID_COL = "#D0D7DE"


def create_palette(impls: list[str], first: str = "#FF8A65") -> dict[str, str]:
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
    first = first.lstrip("#")
    r, g, b = tuple(int(first[i : i + 2], 16) / 255.0 for i in (0, 2, 4))
    h, s, v = colorsys.rgb_to_hsv(r, g, b)

    n = len(impls)
    step = 360.0 / n if n > 0 else 0

    result = {}
    for i, impl in enumerate(impls):
        hue = (h * 360 + i * step) % 360
        r, g, b = colorsys.hsv_to_rgb(hue / 360.0, s, v)
        hex_color = f"#{int(r * 255):02x}{int(g * 255):02x}{int(b * 255):02x}"
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
        axis_text=element_text(color=_TEXT_COL, size=11),
        axis_title=element_text(color=_TEXT_COL, size=12),
        axis_ticks=element_line(color=_GRID_COL),
        plot_title=element_text(
            color=_TEXT_COL, size=15, weight="bold", margin={"b": 10}
        ),
        legend_background=element_rect(fill=_PANEL_BG),
        legend_key=element_rect(fill=_PANEL_BG),
        legend_text=element_text(color=_TEXT_COL, size=11),
        legend_title=element_text(color=_TEXT_COL, size=12, weight="bold"),
        strip_background=element_rect(fill=_PANEL_BG),
        strip_text=element_text(color=_TEXT_COL, size=12, weight="bold"),
    )
)


light_theme: Theme = lambda figure_size=(12, 6): (
    theme_void()
    + theme(
        figure_size=figure_size,
        plot_background=element_rect(fill=_LIGHT_BG, color=_LIGHT_BG),
        panel_background=element_rect(fill=_LIGHT_PANEL_BG, color=_LIGHT_PANEL_BG),
        panel_grid_major_y=element_line(color=_LIGHT_GRID_COL, size=0.6, linetype="dashed"),
        panel_grid_minor=element_blank(),
        axis_text=element_text(color=_LIGHT_TEXT_COL, size=11),
        axis_title=element_text(color=_LIGHT_TEXT_COL, size=12),
        axis_ticks=element_line(color=_LIGHT_GRID_COL),
        plot_title=element_text(
            color=_LIGHT_TEXT_COL, size=15, weight="bold", margin={"b": 10}
        ),
        legend_background=element_rect(fill=_LIGHT_PANEL_BG),
        legend_key=element_rect(fill=_LIGHT_PANEL_BG),
        legend_text=element_text(color=_LIGHT_TEXT_COL, size=11),
        legend_title=element_text(color=_LIGHT_TEXT_COL, size=12, weight="bold"),
        strip_background=element_rect(fill=_LIGHT_PANEL_BG),
        strip_text=element_text(color=_LIGHT_TEXT_COL, size=12, weight="bold"),
    )
)


def get_theme(name: str = "dark") -> tuple[Theme, str]:
    """
    Retourne (theme_function, text_color) pour le thème choisi.
    """
    if name == "light":
        return light_theme, _LIGHT_TEXT_COL
    return dark_theme, _TEXT_COL


def save_plot(plot, path: Path, width: float = 12, height: float = 6, dpi: int = 150):
    """
    Sauvegarde un objet plotnine sur disque sous formats PNG et SVG, et affiche le chemin.
    """
    path.parent.mkdir(parents=True, exist_ok=True)

    # Sauvegarde PNG
    png_path = path.with_suffix(".png")
    plot.save(
        str(png_path), width=width, height=height, dpi=dpi, verbose=False, limitsize=False
    )

    # Sauvegarde SVG (vectoriel)
    svg_path = path.with_suffix(".svg")
    plot.save(
        str(svg_path), width=width, height=height, dpi=dpi, verbose=False, limitsize=False
    )

    print(f"\033[32m  ✓  {png_path}  &  {svg_path}\033[0m")
