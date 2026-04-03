use super::{fill_set, generate_indices, Scenario, ScenarioContructor};
use crate::types::{SetInt, SetIntConstruct};

pub struct DifferenceScenario<T: SetInt> {
    bit_set: T,
    other: T,
}

impl<T: SetIntConstruct> ScenarioContructor for DifferenceScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, seed: u64) -> Self {
        let indices_a = generate_indices(capacity, fill_quantity, seed);
        let indices_b = generate_indices(capacity, data_quantity, seed.wrapping_add(1));

        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, &indices_a);

        let mut other = T::with_capacity(capacity as usize);
        fill_set(&mut other, &indices_b);

        Self { bit_set, other }
    }
}

impl<T: SetIntConstruct> Scenario for DifferenceScenario<T> {
    fn task(&mut self) {
        self.bit_set.difference_with(&self.other);
    }
}
