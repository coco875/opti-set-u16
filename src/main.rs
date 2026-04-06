mod macros;
mod register;
mod scenario;
#[cfg(test)]
mod test;
mod timer;
mod types;

use std::fs::OpenOptions;
use std::io::IsTerminal;
use std::io::stderr;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;

use scenario::*;
use types::*;

use std::io::BufWriter;
use std::io::Write;

use rand::prelude::*;

#[derive(Parser)]
#[command(name = "opti-set-int", about = "Benchmark integer set implementations")]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(short, long)]
    sample: Option<u64>,

    #[arg(long)]
    min_bit: Option<u32>,

    #[arg(long)]
    max_bit: Option<u32>,

    #[arg(long)]
    min_fill_bit: Option<u32>,

    #[arg(long)]
    max_fill_bit: Option<u32>,

    #[arg(long)]
    min_data_bit: Option<u32>,

    #[arg(long)]
    max_data_bit: Option<u32>,

    #[arg(short, long)]
    filter_scenario: Option<Vec<String>>,
}

#[derive(Deserialize, Default)]
struct Config {
    sample: Option<u64>,
    min_bit: Option<u32>,
    max_bit: Option<u32>,
    min_fill_bit: Option<u32>,
    max_fill_bit: Option<u32>,
    min_data_bit: Option<u32>,
    max_data_bit: Option<u32>,
    filter_scenario: Option<Vec<String>>,
}

impl Config {
    fn load(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    fn merge_with_cli(self, cli: Cli) -> Self {
        Self {
            sample: cli.sample.or(self.sample),
            min_bit: cli.min_bit.or(self.min_bit),
            max_bit: cli.max_bit.or(self.max_bit),
            min_fill_bit: cli.min_fill_bit.or(self.min_fill_bit),
            max_fill_bit: cli.max_fill_bit.or(self.max_fill_bit),
            min_data_bit: cli.min_data_bit.or(self.min_data_bit),
            max_data_bit: cli.max_data_bit.or(self.max_data_bit),
            filter_scenario: match (cli.filter_scenario, self.filter_scenario) {
                (Some(cli_filters), _) => Some(cli_filters),
                (None, Some(config_filters)) => Some(config_filters),
                (None, None) => None,
            },
        }
    }
}

/// 0-16 (16 bit) scenario | 16-21 (5 bit) capacity in bit | 21-26 (5 bit) fill in bit | 26-31 (5 bit) data in bit | 31-64 (33 bit) unused | 64-128 (64 bit) seed
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

    /// scenario index, capacity in bit, fill in bit, data in bit, seed
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
    let cli = Cli::parse();

    let config = if let Some(ref config_path) = cli.config {
        println!("Load {config_path}");
        let file_config = Config::load(config_path)?;
        file_config.merge_with_cli(cli)
    } else {
        Config::default().merge_with_cli(cli)
    };

    let sample = config.sample.unwrap_or(10);
    let min_bit = config.min_bit.unwrap_or(4);
    let max_bit = config.max_bit.unwrap_or(16);
    let min_fill_bit = config.min_fill_bit.unwrap_or(min_bit);
    let max_fill_bit = config.max_fill_bit.unwrap_or(max_bit);
    let min_data_bit = config.min_data_bit.unwrap_or(min_bit);
    let max_data_bit = config.max_data_bit.unwrap_or(max_bit);
    let filter_scenario = config.filter_scenario;

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

    let mut rng = SmallRng::seed_from_u64(0);

    let mut all_run: Vec<_> = vec![];
    let scenario = all_scenarios!();

    let mut seed = rng.next_u64();
    for _ in 0..sample {
        for (scenario_id, &(_scenario_builder, scenario_name, _type_name)) in
            scenario.iter().enumerate()
        {
            if let Some(ref filters) = filter_scenario
                && !filters.iter().any(|f| scenario_name.contains(f.as_str())) {
                    continue;
                }
            for cap in min_bit..=max_bit {
                for fill in min_fill_bit..=max_fill_bit.min(cap) {
                    for data in min_data_bit..=max_data_bit.min(cap) {
                        all_run.push(RunId::new(
                            scenario_id as u16,
                            cap as u8,
                            fill as u8,
                            data as u8,
                            seed,
                        ));
                    }
                }
            }
        }
        seed = rng.next_u64();
    }
    println!("run {} config", all_run.len());

    all_run.shuffle(&mut rng);

    let total = all_run.len();
    let start = Instant::now();
    let mut last_pct = 0;

    for (i, run_id) in all_run.iter().enumerate() {
        let pct = (i + 1) * 100 / total;
        if pct >= last_pct + 10 || i + 1 == total {
            last_pct = pct;
            let elapsed = start.elapsed().as_secs_f64();
            let eta = elapsed / (i + 1) as f64 * (total - i - 1) as f64;
            if stderr().is_terminal() {
                eprint!("\r");
            }
            eprintln!(
                "[{pct:3}%] {}/{} elapsed: {:.1}s ETA: {:.1}s",
                i + 1,
                total,
                elapsed,
                eta
            );
        }

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
