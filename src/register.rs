#[macro_export]
macro_rules! register_set_int {
    ($callback:ident) => {
        $callback!([LibBitSet, StdHashSet, LibInterval, IntervalSet, LibRoaring])
    };
}

#[macro_export]
macro_rules! register_scenario {
    ($callback:ident, $($args:tt)*) => {
        $callback!($($args)*, [
            InsertScenario,
            ContainsScenario,
            RemoveScenario,
            MixedScenario,
            SparseScenario,
            ClearScenario,
            LenScenario,
            IterScenario,
            UnionScenario,
            IntersectionScenario,
            DifferenceScenario,
            SymmetricDifferenceScenario
        ])
    };
}
