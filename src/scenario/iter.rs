use super::{Scenario, ScenarioContructor, fill_set};
use crate::types::{SetInt, SetIntConstruct};
use std::hint::black_box;

pub struct IterScenario<T: SetInt> {
    bit_set: T,
}

impl<T: SetIntConstruct> ScenarioContructor for IterScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, _data_quantity: u16, fill_data: &[u16]) -> Self {
        let indices = &fill_data[0..fill_quantity as usize];
        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, indices);
        Self { bit_set }
    }
}

impl<T: SetIntConstruct> Scenario for IterScenario<T> {
    fn task(&mut self) {
        let ret: Vec<u16> = self.bit_set.iter().collect();
        black_box(ret);
    }
}
