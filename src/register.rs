#[macro_export]
macro_rules! register_set_int {
    ($callback:ident) => {
        $callback!([
            LibBitSet,
            StdHashSet,
            StdHashSetDefaultFunc,
            LibInterval,
            IntervalSet,
            LibRoaring,
            StdTreeSet,
            StdTreeSetDefaultFunc,
            LibFxHashSet,
            LibFxHashSetDefaultFunc,
            StdVec,
            StdVecDicotomie,
            StdLinkedList
        ])
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
