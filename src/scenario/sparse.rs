use super::{Scenario, ScenarioConstructor, fill_set};
use crate::types::{SetInt, SetIntConstruct};

pub struct SparseScenario<T: SetInt> {
    bit_set: T,
    fill_indices: Vec<u16>,
    task_indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioConstructor for SparseScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, fill_data: &[u16]) -> Self {
        let fill_indices = Vec::from(&fill_data[0..fill_quantity as usize]);
        let task_indices = Vec::from(
            &fill_data[fill_quantity as usize..(fill_quantity as usize + data_quantity as usize)],
        );

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
