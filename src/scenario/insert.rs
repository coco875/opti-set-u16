use super::{Scenario, ScenarioConstructor, fill_set};
use crate::types::{SetInt, SetIntConstruct};

pub struct InsertScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioConstructor for InsertScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, fill_data: &[u16]) -> Self {
        let fill_indices = &fill_data[0..fill_quantity as usize];

        let indices = Vec::from(
            &fill_data[fill_quantity as usize..(fill_quantity as usize + data_quantity as usize)],
        );
        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, fill_indices);
        Self { bit_set, indices }
    }
}

impl<T: SetIntConstruct> Scenario for InsertScenario<T> {
    fn task(&mut self) {
        for &idx in &self.indices {
            self.bit_set.insert(idx);
        }
    }
}
