use super::{Scenario, ScenarioContructor, fill_set};
use crate::types::{SetInt, SetIntConstruct};
use rand::prelude::*;
use rand::rngs::SmallRng;

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
