use super::{generate_indices, Scenario, ScenarioContructor};
use crate::types::{SetInt, SetIntConstruct};

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
