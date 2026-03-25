mod macros;
mod register;
mod scenario;
#[cfg(test)]
mod test;
mod timer;
mod types;

use scenario::{
    ContainsScenario, InsertScenario, MixedScenario, RemoveScenario, Scenario, ScenarioContructor,
    SparseScenario,
};
use types::{BitSet, StdHashSet};

fn main() {
    let scenario = all_scenarios!();
    for (_, sce, typ) in scenario {
        println!("{} / {}", sce, typ);
    }
    println!("Hello, world!");
}
