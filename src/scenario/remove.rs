use super::{fill_set, generate_indices, Scenario, ScenarioContructor};
use crate::types::{SetInt, SetIntConstruct};

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
