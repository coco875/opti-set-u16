use crate::timer::CpuTimer;
use crate::types::{SetInt, SetIntConstruct};
use rand::prelude::*;
use rand::rngs::SmallRng;

pub trait Scenario {
    fn measure(&mut self) -> u64;
    fn reset(&mut self);
    fn run(&mut self) -> u64 {
        let t = self.measure();
        self.reset();
        t
    }
}

pub trait ScenarioContructor: Scenario {
    fn new(capacity: u16, fill_percent: f64, seed: u64) -> Self
    where
        Self: Sized,
    {
        let mut ret = Self::new_without_reset(capacity, fill_percent, seed);
        ret.reset();
        ret
    }
    fn new_without_reset(capacity: u16, fill_percent: f64, seed: u64) -> Self;
}

pub type ScenarioBuilder = fn(u16, f64, u64) -> Box<dyn Scenario>;

fn generate_indices(capacity: u16, fill_percent: f64, seed: u64) -> Vec<u16> {
    let mut rng = SmallRng::seed_from_u64(seed);
    let count = (capacity as f64 * fill_percent).round() as usize;
    let mut indices: Vec<u16> = (0..capacity).collect();
    indices.shuffle(&mut rng);
    indices.truncate(count);
    indices
}

pub struct InsertScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for InsertScenario<T> {
    fn new_without_reset(capacity: u16, fill_percent: f64, seed: u64) -> Self {
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
    fn measure(&mut self) -> u64 {
        let timer = CpuTimer::new();
        for &idx in &self.indices {
            self.bit_set.insert(idx);
        }
        timer.elapsed_cycles()
    }

    fn reset(&mut self) {
        self.bit_set.clear();
    }
}

pub struct ContainsScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for ContainsScenario<T> {
    fn new_without_reset(capacity: u16, fill_percent: f64, seed: u64) -> Self {
        let indices = generate_indices(capacity, fill_percent, seed);
        let bit_set = T::new();
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
    fn measure(&mut self) -> u64 {
        let timer = CpuTimer::new();
        for &idx in &self.indices {
            let _ = self.bit_set.contains(idx);
        }
        timer.elapsed_cycles()
    }

    fn reset(&mut self) {
        self.bit_set.clear();
        for &idx in &self.indices {
            self.bit_set.insert(idx);
        }
    }
}

pub struct RemoveScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for RemoveScenario<T> {
    fn new_without_reset(capacity: u16, fill_percent: f64, seed: u64) -> Self {
        let indices = generate_indices(capacity, fill_percent, seed);
        let bit_set = T::new();
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
    fn measure(&mut self) -> u64 {
        let timer = CpuTimer::new();
        for &idx in &self.indices {
            self.bit_set.remove(idx);
        }
        timer.elapsed_cycles()
    }

    fn reset(&mut self) {
        self.bit_set.clear();
        for &idx in &self.indices {
            self.bit_set.insert(idx);
        }
    }
}

pub struct MixedScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for MixedScenario<T> {
    fn new_without_reset(capacity: u16, fill_percent: f64, seed: u64) -> Self {
        let indices = generate_indices(capacity, fill_percent, seed);
        let bit_set = T::new();
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
    fn measure(&mut self) -> u64 {
        let timer = CpuTimer::new();
        for &idx in &self.indices {
            self.bit_set.insert(idx);
            let _ = self.bit_set.contains(idx);
            self.bit_set.remove(idx);
        }
        timer.elapsed_cycles()
    }

    fn reset(&mut self) {
        self.bit_set.clear();
        for &idx in &self.indices {
            self.bit_set.insert(idx);
        }
    }
}

pub struct SparseScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for SparseScenario<T> {
    fn new_without_reset(capacity: u16, fill_percent: f64, seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let sparse_range = (capacity as u32 * 10) as usize;

        // Clamp fill_percent to [0.0, 1.0] for safety
        let fill_percent = fill_percent.clamp(0.0, 1.0);
        let count: usize = (capacity as f64 * fill_percent).round() as usize;

        let mut indices: Vec<u16> = (0..sparse_range as u16).collect();
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
    fn measure(&mut self) -> u64 {
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
    fn reset(&mut self) {
        self.bit_set.clear();
    }
}
