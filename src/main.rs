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

use std::io::BufWriter;
use std::io::Write;

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("output.csv")?;
    let mut buf_writer = BufWriter::new(file);
    writeln!(
        &mut buf_writer,
        "Scenario name, Type name, maximum capacity, fill, data, time"
    )?;

    let scenario = all_scenarios!();
    for (scneario_builder, sceario_name, type_name) in scenario {
        for cap in (1..=16).map(|n| ((1u32 << n) - 1) as u16) {
            let mut steps: Vec<u16> = vec![];
            let mut step = 1u32;
            while step < cap as u32 {
                step *= 2;
                if step <= cap as u32 {
                    steps.push((step - 1) as u16);
                }
            }
            steps.push(cap);
            for fill in &steps {
                for data in &steps {
                    let mut sce = scneario_builder(cap, *fill, *data, 0);
                    let time = sce.run();
                    writeln!(
                        &mut buf_writer,
                        "{sceario_name}, {type_name}, {cap}, {fill}, {data}, {time}"
                    )?;
                }
            }
        }
        println!("{} / {}", sceario_name, type_name);
    }
    println!("Hello, world!");
    buf_writer.flush()?;
    Ok(())
}
