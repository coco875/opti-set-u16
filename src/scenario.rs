use crate::timer::CpuTimer;
use crate::types::{SetInt, SetIntConstruct};
use rand::prelude::*;
use rand::rngs::SmallRng;

pub trait Scenario {
    fn run(&mut self) -> u64;
}

pub trait ScenarioContructor {
    fn new(capacity: usize, fill_percent: f64, seed: u64) -> Self;
}

fn generate_indices(capacity: usize, fill_percent: f64, seed: u64) -> Vec<u16> {
    let mut rng = SmallRng::seed_from_u64(seed);
    let count = (capacity as f64 * fill_percent).round() as usize;
    let mut indices: Vec<u16> = (0..capacity as u16).collect();
    indices.shuffle(&mut rng);
    indices.truncate(count);
    indices
}

pub struct InsertScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for InsertScenario<T> {
    fn new(capacity: usize, fill_percent: f64, seed: u64) -> Self {
        let mut indices = generate_indices(capacity, fill_percent, seed);
        let mut rng = SmallRng::seed_from_u64(seed.wrapping_add(1));
        indices.shuffle(&mut rng);
        Self {
            bit_set: T::new(),
            indices,
        }
    }
}

impl<T: SetIntConstruct> Scenario for InsertScenario<T> {
    fn run(&mut self) -> u64 {
        let timer = CpuTimer::new();
        for &idx in &self.indices {
            self.bit_set.insert(idx);
        }
        let t = timer.elapsed_cycles();
        self.bit_set.clear();
        t
    }
}

pub struct ContainsScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for ContainsScenario<T> {
    fn new(capacity: usize, fill_percent: f64, seed: u64) -> Self {
        let indices = generate_indices(capacity, fill_percent, seed);
        let mut bit_set = T::new();
        for &idx in &indices {
            bit_set.insert(idx);
        }
        let mut query_indices = indices.clone();
        let mut rng = SmallRng::seed_from_u64(seed.wrapping_add(1));
        query_indices.shuffle(&mut rng);
        Self {
            bit_set,
            indices: query_indices,
        }
    }
}
impl<T: SetIntConstruct> Scenario for ContainsScenario<T> {
    fn run(&mut self) -> u64 {
        let timer = CpuTimer::new();
        for &idx in &self.indices {
            let _ = self.bit_set.contains(idx);
        }
        timer.elapsed_cycles()
    }
}

pub struct RemoveScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for RemoveScenario<T> {
    fn new(capacity: usize, fill_percent: f64, seed: u64) -> Self {
        let indices = generate_indices(capacity, fill_percent, seed);
        let mut bit_set = T::new();
        for &idx in &indices {
            bit_set.insert(idx);
        }
        let mut remove_indices = indices.clone();
        let mut rng = SmallRng::seed_from_u64(seed.wrapping_add(1));
        remove_indices.shuffle(&mut rng);
        Self {
            bit_set,
            indices: remove_indices,
        }
    }
}
impl<T: SetIntConstruct> Scenario for RemoveScenario<T> {
    fn run(&mut self) -> u64 {
        let timer = CpuTimer::new();
        for &idx in &self.indices {
            self.bit_set.remove(idx);
        }
        timer.elapsed_cycles()
    }
}

pub struct MixedScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for MixedScenario<T> {
    fn new(capacity: usize, fill_percent: f64, seed: u64) -> Self {
        let indices = generate_indices(capacity, fill_percent, seed);
        let mut bit_set = T::new();
        for &idx in &indices {
            bit_set.insert(idx);
        }
        let mut mixed_indices = indices.clone();
        let mut rng = SmallRng::seed_from_u64(seed.wrapping_add(1));
        mixed_indices.shuffle(&mut rng);
        Self {
            bit_set,
            indices: mixed_indices,
        }
    }
}

impl<T: SetIntConstruct> Scenario for MixedScenario<T> {
    fn run(&mut self) -> u64 {
        let timer = CpuTimer::new();
        for &idx in &self.indices {
            self.bit_set.insert(idx);
            let _ = self.bit_set.contains(idx);
            self.bit_set.remove(idx);
        }
        timer.elapsed_cycles()
    }
}

pub struct SparseScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for SparseScenario<T> {
    fn new(capacity: usize, fill_percent: f64, seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let sparse_range = (capacity * 10) as u16;
        let count = (capacity as f64 * fill_percent).round() as usize;
        let mut indices: Vec<u16> = (0..sparse_range).collect();
        indices.shuffle(&mut rng);
        indices.truncate(count);
        indices.sort();
        Self {
            bit_set: T::new(),
            indices,
        }
    }
}

impl<T: SetIntConstruct> Scenario for SparseScenario<T> {
    fn run(&mut self) -> u64 {
        let timer = CpuTimer::new();
        for &idx in &self.indices {
            self.bit_set.insert(idx);
        }
        for &idx in &self.indices {
            let _ = self.bit_set.contains(idx);
        }
        for &idx in &self.indices {
            self.bit_set.remove(idx);
        }
        timer.elapsed_cycles()
    }
}
