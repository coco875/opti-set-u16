mod clear;
mod contain;
mod deserialize;
mod difference;
mod insert;
mod intersection;
mod iter;
mod len;
mod mixed;
mod remove;
mod serialize;
mod sparse;
mod symmetric_difference;
mod union;

pub use clear::ClearScenario;
pub use contain::ContainsScenario;
pub use deserialize::DeserializeScenario;
pub use difference::DifferenceScenario;
pub use insert::InsertScenario;
pub use intersection::IntersectionScenario;
pub use iter::IterScenario;
pub use len::LenScenario;
pub use mixed::MixedScenario;
pub use remove::RemoveScenario;
pub use serialize::SerializeScenario;
pub use sparse::SparseScenario;
pub use symmetric_difference::SymmetricDifferenceScenario;
pub use union::UnionScenario;

use crate::timer::CpuTimer;
use crate::types::SetInt;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub trait Scenario {
    /// the task measured (add black_box on return value to avoid compiler optimisation)
    fn task(&mut self);
    fn run(&mut self) -> u64 {
        let timer = CpuTimer::new();
        self.task();
        timer.elapsed_cycles()
    }
}

pub trait ScenarioConstructor: Scenario {
    /// Create a new scenario with:
    /// - `capacity`: maximum number of elements the set can hold
    /// - `fill_quantity`: number of elements to pre-fill the set with before running the task
    /// - `data_quantity`: number of elements to use as input data for the task
    /// - `fill_data`: slice containing the pre-generated input data to be used for filling and execution
    fn new(capacity: u16, fill_quantity: u16, data_quantity: u16, fill_data: &[u16]) -> Self
    where
        Self: Sized;
}

fn fill_set<T: SetInt>(bit_set: &mut T, indices: &[u16]) {
    for &idx in indices {
        bit_set.insert(idx);
    }
}

pub type ScenarioBuilder = fn(u16, u16, u16, &[u16]) -> Box<dyn Scenario>;

pub fn generate_data(fill_max: u16, count: u32, seed: u64) -> Box<[u16]> {
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut data = Vec::with_capacity(fill_max as usize);
    for _ in 0..count {
        let value = rng.next_u32() as f64 / u32::MAX as f64 * fill_max as f64;
        data.push(value as u16);
    }
    data.into_boxed_slice()
}
