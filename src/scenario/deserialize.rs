use super::{Scenario, ScenarioContructor, fill_set};
use crate::types::{SetInt, SetIntConstruct};

pub struct DeserializeScenario<T: SetInt> {
    bit_set: T,
    bytes: [u8; 8192],
}

impl<T: SetIntConstruct> ScenarioContructor for DeserializeScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, _data_quantity: u16, fill_data: &[u16]) -> Self {
        let indices = &fill_data[0..fill_quantity as usize];
        let mut temp_set = T::with_capacity(capacity as usize);
        fill_set(&mut temp_set, indices);
        let bytes = temp_set.to_bytes();
        Self {
            bit_set: T::with_capacity(capacity as usize),
            bytes,
        }
    }
}

impl<T: SetIntConstruct> Scenario for DeserializeScenario<T> {
    fn task(&mut self) {
        self.bit_set.from_bytes(&self.bytes);
    }
}
