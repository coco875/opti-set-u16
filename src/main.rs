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

use rand::prelude::*;

/// 0-16 (16 bit) scenario | 16-21 (5 bit) capacity in bit | 21-26 (5 bit) fill in bit | 26-31 (5 bit) data in bit | 28-64 (33 bit) nothing | 64-128 (64 bit) seed
#[derive(Clone, Copy)]
struct RunId(u128);

impl RunId {
    fn new(scenario: u16, cap_n: u8, fill_idx: u8, data_idx: u8, seed: u64) -> Self {
        Self(
            (scenario as u128)
                | (((cap_n & 0b11111) as u128) << 16)
                | (((fill_idx & 0b11111) as u128) << 21)
                | (((data_idx & 0b11111) as u128) << 26)
                | ((seed as u128) << 64),
        )
    }

    /// senario index, capacity in bit, fill in bit, data in bit, seed
    fn unpack(self) -> (u16, u8, u8, u8, u64) {
        (
            ((self.0) & 0xFFFF) as u16,                      // scenario_idx
            ((self.0 >> 16) & 0b11111) as u8,                // cap_n (1..=16)
            ((self.0 >> 21) & 0b11111) as u8,                // fill_idx
            ((self.0 >> 26) & 0b11111) as u8,                // data_idx
            ((self.0 >> 64) & 0xFFFF_FFFF_FFFF_FFFF) as u64, // seed
        )
    }
}

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("output.csv")?;
    let mut buf_writer = BufWriter::new(file);
    writeln!(
        &mut buf_writer,
        "Scenario name, Type name, maximum capacity, fill, data, seed, time"
    )?;

    let sample = 10;
    let mut rng = SmallRng::seed_from_u64(0);

    let mut all_run: Vec<_> = vec![];
    let scenario = all_scenarios!();

    let mut seed = rng.next_u64();
    for _ in 0..sample {
        for sceanrio_id in 0..scenario.len() {
            for cap in 1..=16 {
                for fill in 1..=cap {
                    for data in 1..=cap {
                        all_run.push(RunId::new(sceanrio_id as u16, cap, fill, data, seed));
                    }
                }
            }
        }
        seed = rng.next_u64();
    }
    println!("run {} config", all_run.len());

    all_run.shuffle(&mut rng);

    for run_id in all_run {
        let (scenario_id, cap_bit, fill_bit, data_bit, seed) = run_id.unpack();
        let (scenario_builder, sceario_name, type_name) = scenario[scenario_id as usize];
        let cap = ((1u32 << cap_bit) - 1) as u16;
        let fill = ((1u32 << fill_bit) - 1) as u16;
        let data = ((1u32 << data_bit) - 1) as u16;

        let mut sce = scenario_builder(cap, fill, data, seed);
        let time = sce.run();
        writeln!(
            &mut buf_writer,
            "{sceario_name}, {type_name}, {cap}, {fill}, {data}, {seed}, {time}"
        )?;
    }
    println!("Hello, world!");
    buf_writer.flush()?;
    Ok(())
}
