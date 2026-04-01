use std::hint::black_box;

use crate::timer::CpuTimer;
use crate::types::{SetInt, SetIntConstruct};
use rand::prelude::*;
use rand::rngs::SmallRng;

pub trait Scenario {
    /// the task measured (add black_box on return value to avoid compiler optimisation)
    fn task(&mut self);
    fn run(&mut self) -> u64 {
        let timer = CpuTimer::new();
        self.task();
        timer.elapsed_cycles()
    }
}

pub trait ScenarioContructor: Scenario {
    /// Create a new scenario with:
    /// - `capacity`: maximum number of elements the set can hold
    /// - `fill_quantity`: number of elements to pre-fill the set with before running the task
    /// - `data_quantity`: number of elements to use as input data for the task
    /// - `seed`: random seed for reproducibility
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, seed: u64) -> Self
    where
        Self: Sized;
}

fn fill_set<T: SetInt>(bit_set: &mut T, indices: &[u16]) {
    for &idx in indices {
        bit_set.insert(idx);
    }
}

pub type ScenarioBuilder = fn(u16, u16, u16, u64) -> Box<dyn Scenario>;

fn generate_indices(capacity: u16, count: u16, seed: u64) -> Vec<u16> {
    let mut rng = SmallRng::seed_from_u64(seed);
    let count = count as usize;
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
    fn new(capacity: u16, _fill_quantity: u16, data_quantity: u16, seed: u64) -> Self {
        let indices = generate_indices(capacity, data_quantity, seed);
        Self {
            bit_set: T::with_capacity(capacity as usize),
            indices,
        }
    }
}

impl<T: SetIntConstruct> Scenario for InsertScenario<T> {
    fn task(&mut self) {
        for &idx in &self.indices {
            self.bit_set.insert(idx);
        }
    }
}

pub struct ContainsScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for ContainsScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, seed: u64) -> Self {
        let fill_indices = generate_indices(capacity, fill_quantity, seed);
        let query_indices = generate_indices(capacity, data_quantity, seed.wrapping_add(1));

        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, &fill_indices);

        Self {
            bit_set,
            indices: query_indices,
        }
    }
}
impl<T: SetIntConstruct> Scenario for ContainsScenario<T> {
    fn task(&mut self) {
        for &idx in &self.indices {
            let ret = self.bit_set.contains(idx);
            black_box(ret);
        }
    }
}

pub struct RemoveScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for RemoveScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, seed: u64) -> Self {
        let fill_indices = generate_indices(capacity, fill_quantity, seed);
        let remove_indices = generate_indices(capacity, data_quantity, seed.wrapping_add(1));

        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, &fill_indices);
        Self {
            bit_set,
            indices: remove_indices,
        }
    }
}
impl<T: SetIntConstruct> Scenario for RemoveScenario<T> {
    fn task(&mut self) {
        for &idx in &self.indices {
            self.bit_set.remove(idx);
        }
    }
}

pub struct MixedScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for MixedScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, seed: u64) -> Self {
        let fill_indices = generate_indices(capacity, fill_quantity, seed);
        let task_indices = generate_indices(capacity, data_quantity, seed.wrapping_add(1));

        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, &fill_indices);

        Self {
            bit_set,
            indices: task_indices,
        }
    }
}

impl<T: SetIntConstruct> Scenario for MixedScenario<T> {
    fn task(&mut self) {
        for &idx in &self.indices {
            self.bit_set.insert(idx);
            let ret = self.bit_set.contains(idx);
            black_box(ret);
            self.bit_set.remove(idx);
        }
    }
}

pub struct SparseScenario<T: SetInt> {
    bit_set: T,
    fill_indices: Vec<u16>,
    task_indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for SparseScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let sparse_range = (capacity as u32 * 10) as usize;

        let fill_count = fill_quantity as usize;
        let task_count = data_quantity as usize;

        let mut fill_indices: Vec<u16> = (0..sparse_range as u16).collect();
        fill_indices.shuffle(&mut rng);
        fill_indices.truncate(fill_count);
        fill_indices.sort();

        let mut task_indices: Vec<u16> = (0..sparse_range as u16).collect();
        task_indices.shuffle(&mut rng);
        task_indices.truncate(task_count);

        Self {
            bit_set: T::with_capacity(capacity as usize),
            fill_indices,
            task_indices,
        }
    }
}

impl<T: SetIntConstruct> Scenario for SparseScenario<T> {
    fn task(&mut self) {
        fill_set(&mut self.bit_set, &self.fill_indices);
        for &idx in &self.task_indices {
            let _ = self.bit_set.contains(idx);
        }
        for &idx in &self.fill_indices {
            self.bit_set.remove(idx);
        }
    }
}
