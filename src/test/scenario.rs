use crate::scenario::{
    ContainsScenario, InsertScenario, MixedScenario, RemoveScenario, Scenario, ScenarioBuilder,
    ScenarioContructor, SparseScenario,
};
use crate::types::{BitSet, StdHashSet};
use crate::{
    all_scenarios, build_array, build_array_inner, build_array_with_scenarios, register_scenario,
    register_set_int,
};

#[test]
fn test_all_scenario() {
    for (scenario_builder, _, _) in all_scenarios!() {
        let mut sce = scenario_builder(0, 0.0, 0);
        assert!(sce.as_mut().run() > 0);
    }
}
