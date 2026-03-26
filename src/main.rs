mod macros;
mod register;
mod scenario;
#[cfg(test)]
mod test;
mod timer;
mod types;

use std::fs::OpenOptions;

use anyhow::Result;

use scenario::{
    ContainsScenario, InsertScenario, MixedScenario, RemoveScenario, Scenario, ScenarioBuilder,
    ScenarioContructor, SparseScenario,
};
use types::{BitSet, StdHashSet};

use std::io::Write;

fn main() -> Result<()> {
    let mut output_vec: Vec<(&str, &str, u16, f64, u64)> = vec![];

    let scenario = all_scenarios!();
    for (scneario_builder, sceario_name, type_name) in scenario {
        for cap in (1..16).map(|n| 1 << n) {
            let iter = std::iter::successors(Some(0.0_f64), |&x| {
                let next = x + 0.1;
                (next < 1.0).then_some(next)
            });
            for fill in iter {
                let mut sce = scneario_builder(cap, fill, 0);
                output_vec.push((sceario_name, type_name, cap, fill, sce.run()));
            }
        }
        println!("{} / {}", sceario_name, type_name);
    }
    let mut file = OpenOptions::new()
        .create_new(true)
        .truncate(true)
        .write(true)
        .open("output.csv")?;
    for (sceario_name, type_name, cap, fill, time) in output_vec {
        writeln!(
            &mut file,
            "{sceario_name}, {type_name}, {cap}, {fill:.2}, {time}"
        )?;
    }
    println!("Hello, world!");
    Ok(())
}
