use super::{Scenario, ScenarioContructor, fill_set, generate_indices};
use crate::types::{SetInt, SetIntConstruct};
use std::hint::black_box;

pub struct IterScenario<T: SetInt> {
    bit_set: T,
}

impl<T: SetIntConstruct> ScenarioContructor for IterScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, _data_quantity: u16, seed: u64) -> Self {
        let indices = generate_indices(capacity, fill_quantity, seed);
        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, &indices);
        Self { bit_set }
    }
}

impl<T: SetIntConstruct> Scenario for IterScenario<T> {
    fn task(&mut self) {
        let ret: Vec<u16> = self.bit_set.iter().collect();
        black_box(ret);
    }
}
