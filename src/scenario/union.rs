use super::{Scenario, ScenarioConstructor, fill_set};
use crate::types::{SetInt, SetIntConstruct};

pub struct UnionScenario<T: SetInt> {
    bit_set: T,
    other: T,
}

impl<T: SetIntConstruct> ScenarioConstructor for UnionScenario<T> {
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, fill_data: &[u16]) -> Self {
        let indices_a = &fill_data[0..fill_quantity as usize];
        let indices_b =
            &fill_data[fill_quantity as usize..(fill_quantity as usize + data_quantity as usize)];

        let mut bit_set = T::with_capacity(capacity as usize);
        fill_set(&mut bit_set, indices_a);

        let mut other = T::with_capacity(capacity as usize);
        fill_set(&mut other, indices_b);

        Self { bit_set, other }
    }
}

impl<T: SetIntConstruct> Scenario for UnionScenario<T> {
    fn task(&mut self) {
        self.bit_set.union_with(&self.other);
    }
}
