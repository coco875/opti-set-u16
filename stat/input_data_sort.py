import polars as pl
from plotnine import *
import os
import re

def calculate_sortedness(data_str):
    elements = [int(x) for x in re.findall(r'\d+', str(data_str))]
    if len(elements) <= 1:
        return 1.0
    
    sorted_pairs = sum(1 for i in range(len(elements) - 1) if elements[i] <= elements[i+1])
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
        pl.col("data").map_elements(calculate_sortedness, return_dtype=pl.Float64).alias("sortedness"),
        pl.col("seed").cast(pl.String)
    )
    
    # Select only what we need, keeping it as a Polars DataFrame
    df_plot = df.select(["seed", "capacity", "sortedness"])
    
    os.makedirs("stat", exist_ok=True)
    
    # GRAPH 1: Facet by seed (one graph per seed)
    p_seeds = (
        ggplot(df_plot, aes(x='factor(capacity)', y='sortedness', fill='factor(capacity)')) +
        geom_boxplot() +
        facet_wrap('~ seed', labeller='label_both', ncol=4) +
        labs(title="Sortedness Distribution by Capacity (One Graph per Seed)", 
             x="Capacity", 
             y="Proportion of sorted adjacent pairs") +
        theme_minimal() +
        theme(figure_size=(20, 16), legend_position="none", axis_text_x=element_text(rotation=45, hjust=1))
    )
    p_seeds.save("stat/sortedness_by_capacity_per_seed.png", dpi=300)
    print("Graph saved to stat/sortedness_by_capacity_per_seed.png")

    # GRAPH 2: All seeds combined (Average / overall distribution)
    p_all = (
        ggplot(df_plot, aes(x='factor(capacity)', y='sortedness', fill='factor(capacity)')) +
        geom_boxplot() +
        labs(title="Sortedness Distribution by Capacity (All Seeds Combined)", 
             x="Capacity", 
             y="Proportion of sorted adjacent pairs") +
        theme_minimal() +
        theme(figure_size=(10, 6), legend_position="none", axis_text_x=element_text(rotation=45, hjust=1))
    )
    p_all.save("stat/sortedness_by_capacity_all_seeds.png", dpi=300)
    print("Graph saved to stat/sortedness_by_capacity_all_seeds.png")

if __name__ == "__main__":
    main()
