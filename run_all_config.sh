rm -rf benchmark_charts*

cargo run --release -- --config config_set/without_2_worst.toml
python3 stat/main.py
mv benchmark_charts benchmark_charts_without_2_worst
mv data.csv benchmark_charts_without_2_worst/

cargo run --release -- --config config_set/without_4_worst.toml
python3 stat/main.py
mv benchmark_charts benchmark_charts_without_4_worst
mv data.csv benchmark_charts_without_4_worst/

cargo run --release -- --config config_set/without_8_worst.toml
python3 stat/main.py
mv benchmark_charts benchmark_charts_without_8_worst
mv data.csv benchmark_charts_without_8_worst/

cargo run --release -- --config config.toml
python3 stat/main.py

python3 stat/input_data_sort.py
python3 stat/input_stats_splitness.py
