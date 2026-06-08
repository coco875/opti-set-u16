use super::{Scenario, ScenarioConstructor, fill_set};
use crate::types::{SetInt, SetIntConstruct};
use std::hint::black_box;

pub struct ContainsScenario<T: SetInt> {
    bit_set: T,
    indices: Vec<u16>,
}

impl<T: SetIntConstruct> ScenarioConstructor for ContainsScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, fill_data: &[u16]) -> Self {
        let fill_indices = &fill_data[0..fill_quantity as usize];
        let query_indices = Vec::from(
            &fill_data[fill_quantity as usize..(fill_quantity as usize + data_quantity as usize)],
        );

        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, fill_indices);

        Self {
            bit_set,
            indices: query_indices,
        }
    }
}
impl<T: SetIntConstruct> Scenario for ContainsScenario<T> {
    fn task(&mut self) {
        for &idx in &self.indices {
            let ret = self.bit_set.contains(idx);
            black_box(ret);
        }
    }
}
