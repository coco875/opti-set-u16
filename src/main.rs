mod macros;
mod register;
mod scenario;
#[cfg(test)]
mod test;
mod timer;
mod types;

use scenario::{
    ContainsScenario, InsertScenario, MixedScenario, RemoveScenario, Scenario, ScenarioBuilder,
    ScenarioContructor, SparseScenario,
};
use types::{BitSet, StdHashSet};

fn main() {
    let scenario = all_scenarios!();
    for (scneario_builder, sceario_name, type_name) in scenario {
        println!("{} / {}", sceario_name, type_name);
    }
    println!("Hello, world!");
}
