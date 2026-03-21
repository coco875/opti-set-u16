mod scenario;
mod test;
mod timer;
mod types;

use scenario::{
    ContainsScenario, InsertScenario, MixedScenario, RemoveScenario, Scenario, SparseScenario,
};
use types::{BitSet, StdHashSet};

macro_rules! scenario_closure {
    ($ty:ty) => {
        move |cap, fill, seed| -> Box<dyn Scenario> { Box::new(<$ty>::new(cap, fill, seed)) }
    };
}

macro_rules! create_scenario_list {
    ($ty:ty) => {
        &[
            scenario_closure!(InsertScenario<$ty>),
            scenario_closure!(ContainsScenario<$ty>),
            scenario_closure!(RemoveScenario<$ty>),
            scenario_closure!(MixedScenario<$ty>),
            scenario_closure!(SparseScenario<$ty>),
        ]
    };
}

macro_rules! all_scenarios {
    ($($ty:ty),*) => {{
        let mut v = Vec::new();
        $(v.extend_from_slice(create_scenario_list!($ty));)*
        v
    }}
}

fn main() {
    let _ = all_scenarios!(BitSet, StdHashSet);
    println!("Hello, world!");
}
