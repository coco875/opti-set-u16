use super::{fill_set, generate_indices, Scenario, ScenarioContructor};
use crate::types::{SetInt, SetIntConstruct};

pub struct ClearScenario<T: SetInt> {
    bit_set: T,
}

impl<T: SetIntConstruct> ScenarioContructor for ClearScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, _data_quantity: u16, seed: u64) -> Self {
        let indices = generate_indices(capacity, fill_quantity, seed);
        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, &indices);
        Self { bit_set }
    }
}

impl<T: SetIntConstruct> Scenario for ClearScenario<T> {
    fn task(&mut self) {
        self.bit_set.clear();
    }
}
