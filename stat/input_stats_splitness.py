import polars as pl
from plotnine import *
import sys
from collections import Counter


def count_missing_values(df):
    res = (pl.DataFrame()
           .with_columns(df["seed"].unique())
           .join(
               pl.DataFrame()
               .with_columns(
                   df["capacity"].unique()
                   ),how="cross"
               )
           )
    capacitys = df["capacity"].unique()
    num_holes = []
    for seed in df["seed"].unique():
        for capacity in capacitys:
            data = (
                df
                .filter(pl.col("capacity") == capacity)
                .filter(pl.col("seed") == seed)
                ["data"]
            )
            c = Counter(data.to_list())
            nb_holes = 0
            for i in range(capacity):
                if i not in c:
                    nb_holes += 1
            num_holes.append(nb_holes)
    return (
        res
        .with_columns(pl.Series("num_holes",num_holes))
        .with_columns((pl.col("num_holes") / pl.col("capacity")).alias("holes_ratio"))
    )

def count_holes(df):
    res = (pl.DataFrame()
           .with_columns(df["seed"].unique())
           .join(
               pl.DataFrame()
               .with_columns(
                   df["capacity"].unique()
                   ),how="cross"
               )
           )
    capacitys = df["capacity"].unique()
    num_holes = []
    for seed in df["seed"].unique():
        for capacity in capacitys:
            data = (
                df
                .filter(pl.col("capacity") == capacity)
                .filter(pl.col("seed") == seed)
                ["data"]
            )
            data = data.sort()
            # count the number of hole by counting the number of time 
            # the diff between a number and the previous one is > 1
            num_holes.append(int((data.diff() > 1).sum()))
    return (
        res
        .with_columns(pl.Series("num_holes",num_holes))
        .with_columns((pl.col("num_holes") / pl.col("capacity")).alias("holes_ratio"))
    )





def main():
    INPUT_FILE = sys.argv[1] if len(sys.argv) > 1 else "data.csv"

    input_df = (
        pl.read_csv(INPUT_FILE)
        .select(pl.all().name.replace(r".*seed.*","seed"))
        .select(pl.all().name.replace(r".*capacity.*","capacity"))
        .select(pl.all().name.replace(r".*data.*","data"))
        )


    df = (
        input_df.with_columns(
            pl.col("data")
            .str.strip_chars('[] "')
            .str.split("; ")
        )
        .explode("data")
        .with_columns(
            pl.col("data")
            .str.to_integer(), 
            pl.col("seed")
            .cast(pl.UInt64)
        )
        .with_columns(
            pl.col("capacity")
            .str.strip_chars(' ')
            .str.to_integer()
        )
        )
    
    missing_values = count_missing_values(df)
    
    # plot number of holes per capacity (missing value)
    p1 = (
        ggplot(missing_values, aes(x="num_holes", y="factor(capacity)"))
        + geom_point(size = 1)
        + facet_wrap("~seed")
        + labs("number of holes per capacity (missing value)")
    )

    #p1.show()
    
    # plot ratio of hole per capacity (missing value)
    p2 = (
        ggplot(missing_values, aes(x="holes_ratio", y="factor(capacity)"))
        + geom_point(size = 1)
        + facet_wrap("~seed")
        + labs("ratio of holes per capacity (missing value)")
    )
    
    #p2.show()
    
    non_consecutive = count_holes(df)
    
    # plot number of holes per capacity (non consecutive)
    p3 = (
        ggplot(non_consecutive, aes(x="num_holes", y="factor(capacity)"))
        + geom_point(size = 1)
        + facet_wrap("~seed")
        + labs("number of holes per capacity (non consecutive)")
    )

    #p3.show()
    
    # plot ratio of hole per capacity (non consecutive)
    p4 = (
        ggplot(non_consecutive, aes(x="holes_ratio", y="factor(capacity)"))
        + geom_point(size = 1)
        + facet_wrap("~seed")
        + labs("ratio of holes per capacity (non consecutive)")
    )
    
    #p4.show()
    
    # plot ratio of hole per capacity with all seed (missing value)
    p5 = (
        ggplot(missing_values, aes(x="factor(capacity)", y="holes_ratio",fill="factor(capacity)"))
        + geom_boxplot()
        + labs("ratio of holes per capacity all_seed (missing value)")
        + theme_minimal()
    )
    
    p5.show()
    
    # plot ratio of hole per capacity with all seed (non consecutive)
    p6 = (
        ggplot(non_consecutive, aes(x="factor(capacity)", y="holes_ratio",fill="factor(capacity)"))
        + geom_boxplot()
        + labs("ratio of holes per capacity all_seed (non consecutive)")
        + theme_minimal()
    )
    
    p6.show()
    

if __name__ == "__main__":
    main()
    