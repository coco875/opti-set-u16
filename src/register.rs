#[macro_export]
macro_rules! register_set_int {
    ($callback:ident) => {
        $callback!([BitSet, StdHashSet])
    };
}

#[macro_export]
macro_rules! register_scenario {
    ($callback:ident, $($args:tt)*) => {
        $callback!($($args)*, [InsertScenario, ContainsScenario, RemoveScenario, MixedScenario, SparseScenario])
    };
}
