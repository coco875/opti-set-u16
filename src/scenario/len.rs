use super::{fill_set, generate_indices, Scenario, ScenarioContructor};
use crate::types::{SetInt, SetIntConstruct};
use std::hint::black_box;

pub struct LenScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioContructor for LenScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, _data_quantity: u16, seed: u64) -> Self {
        let indices = generate_indices(capacity, fill_quantity, seed);
        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, &indices);
        Self { bit_set, indices }
    }
}

impl<T: SetIntConstruct> Scenario for LenScenario<T> {
    fn task(&mut self) {
        for _ in &self.indices {
            let ret = self.bit_set.len();
            black_box(ret);
        }
    }
}
