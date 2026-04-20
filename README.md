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
| `--min-bit <N>` | Minimum capacity bit width | 4 |
| `--max-bit <N>` | Maximum capacity bit width | 16 |
| `--min-fill-bit <N>` | Minimum fill bit width | `--min-bit` |
| `--max-fill-bit <N>` | Maximum fill bit width | `--max-bit` |
| `--min-data-bit <N>` | Minimum data bit width | `--min-bit` |
| `--max-data-bit <N>` | Maximum data bit width | `--max-bit` |
| `-f, --filter-scenario <NAME>` | Run only scenarios matching this substring | (none) |
| `-c, --config <PATH>` | Path to a TOML config file | (none) |

### Configuration File

You can also use a TOML config file with `--config`:

```toml
# config.toml
sample = 20
min_bit = 8
max_bit = 16
min_fill_bit = 4
max_fill_bit = 12
min_data_bit = 4
max_data_bit = 16
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

## Implementations

Here is a list of the integer set implementations benchmarked in this project, categorized by their origin:

### Custom Implementations

- **ByteArraySet**: A simple byte array acting as a bitset.
- **SimdBitSet**: A custom bitset implementation leveraging SIMD instructions for faster operations.
- **CustomBitSet**: A basic custom bitset implementation.
- **BitTreeSet**: A custom tree-based bitset structure.
- **IntervalSet**: A custom interval set.

### Standard Library (`std`)

- **StdHashSet**: The standard library `std::collections::HashSet` with the default hasher.
- **StdHashSetDefaultFunc**: The standard library `std::collections::HashSet` using a custom/default hash function wrapper.
- **StdHashSetNoHasher**: The standard library `std::collections::HashSet` using a custom no-op hasher optimized for integers.
- **StdTreeSet**: The standard library `std::collections::BTreeSet`.
- **StdTreeSetDefaultFunc**: The standard library `std::collections::BTreeSet` with a default function wrapper.
- **StdVec**: The standard library `std::vec::Vec` used as a linear, unsorted set.
- **StdVecDicotomie**: The standard library `std::vec::Vec` kept sorted, using binary search (dichotomy) for lookups.
- **StdLinkedList**: The standard library `std::collections::LinkedList` used as a set.

### Third-Party Crates

- **LibBitSet**: Uses the `bit-set` crate.
- **LibBitVec**: Uses the `bitvec` crate.
- **LibFixedBitSet**: Uses the `fixedbitset` crate.
- **LibIdlset**: Uses the `idlset` crate.
- **LibInterval**: Uses the `rust_intervals` crate.
- **LibRangeSetBlaze**: Uses the `range-set-blaze` crate.
- **LibRoaring**: Uses the `roaring` crate (pure Rust Roaring bitmaps).
- **LibCRoaring**: Uses the `croaring` crate (C bindings for Roaring bitmaps).
- **LibFxHashSet**: Uses the `rustc-hash` crate (`FxHashSet`).
- **LibFxHashSetDefaultFunc**: Uses the `rustc-hash` crate with default function wrappers.
- **LibAvlTree**: Uses the `avl` crate.
- **LibRBTree**: Uses the `rbtree` crate.

## Scenarios

The following scenarios are used to benchmark the different implementations:

- **InsertScenario**: Benchmarks the performance of adding elements to the set.
- **ContainsScenario**: Benchmarks the performance of checking if elements exist in the set.
- **RemoveScenario**: Benchmarks the performance of removing elements from the set.
- **MixedScenario**: A workload with a mix of inserts, removes, and lookups.
- **SparseScenario**: Benchmarks operations on a sparsely populated set.
- **ClearScenario**: Benchmarks clearing the entire set.
- **LenScenario**: Benchmarks calculating the number of elements in the set.
- **IterScenario**: Benchmarks iterating over all elements in the set.
- **UnionScenario**: Benchmarks the union operation between two sets.
- **IntersectionScenario**: Benchmarks the intersection operation between two sets.
- **DifferenceScenario**: Benchmarks the difference operation between two sets.
- **SymmetricDifferenceScenario**: Benchmarks the symmetric difference operation between two sets.
- **SerializeScenario**: Benchmarks serializing the set.
- **DeserializeScenario**: Benchmarks deserializing the set.
