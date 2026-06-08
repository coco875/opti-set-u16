use super::{Scenario, ScenarioConstructor, fill_set};
use crate::types::{SetInt, SetIntConstruct};

pub struct RemoveScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioConstructor for RemoveScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, fill_data: &[u16]) -> Self {
        let fill_indices = &fill_data[0..fill_quantity as usize];
        let remove_indices = Vec::from(
            &fill_data[fill_quantity as usize..(fill_quantity as usize + data_quantity as usize)],
        );

        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, fill_indices);
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
