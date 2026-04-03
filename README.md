# opti-set-int

Benchmark tool comparing different integer set implementations across various scenarios.

## Usage

### CLI Arguments

```bash
cargo run -- [OPTIONS]
```

| Flag | Description | Default |
|------|-------------|---------|
| `-s, --sample <N>` | Number of seed iterations | 10 |
| `--min-bit <N>` | Minimum bit width | 4 |
| `--max-bit <N>` | Maximum bit width | 16 |
| `-f, --filter-scenario <NAME>` | Run only scenarios matching this substring | (none) |
| `-c, --config <PATH>` | Path to a YAML config file | (none) |

### Configuration File

You can also use a TOML config file with `--config`:

```toml
# config.toml
sample = 20
min_bit = 8
max_bit = 16
filter_scenario = "BitSet"
```

```bash
cargo run -- --config config.toml
```

CLI arguments take precedence over values in the config file.

## Adding a New Implementation

1. Ensure your type implements the `SetInt` trait (defined in `src/types/`)
2. Register it in `src/register.rs` by adding it to the `register_set_int!` macro:

```rust
#[macro_export]
macro_rules! register_set_int {
    ($callback:ident) => {
        $callback!([BitSet, StdHashSet, YourNewType])
    };
}
```

## Adding a New Scenario

1. Create a new file in `src/scenario/` (e.g., `your_scenario.rs`)
2. Implement the `Scenario` and `ScenarioContructor` traits:

```rust
pub struct YourScenario<T: SetInt> {
    // your fields
}

impl<T: SetInt> Scenario for YourScenario<T> {
    fn task(&mut self) {
        // the operation being measured
    }
}

impl<T: SetInt> ScenarioContructor for YourScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, seed: u64) -> Self {
        // initialize your scenario
    }
}
```

3. Register the module and export in `src/scenario/mod.rs`:

```rust
mod your_scenario;
pub use your_scenario::YourScenario;
```

4. Register it in `src/register.rs` by adding it to the `register_scenario!` macro:

```rust
#[macro_export]
macro_rules! register_scenario {
    ($callback:ident, $($args:tt)*) => {
        $callback!($($args)*, [InsertScenario, ContainsScenario, RemoveScenario, MixedScenario, SparseScenario, YourScenario])
    };
}
```
