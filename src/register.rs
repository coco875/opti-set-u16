#[macro_export]
macro_rules! register_set_int {
    ($callback:ident) => {
        $callback!([
            ByteArraySet,
            SimdBitSet,
            CustomBitSet,
            BitTreeSet,
            LibBitSet,
            LibBitVec,
            LibFixedBitSet,
            StdHashSet,
            StdHashSetDefaultFunc,
            LibInterval,
            LibRangeSetBlaze,
            IntervalSet,
            LibRoaring,
            StdTreeSet,
            StdTreeSetDefaultFunc,
            LibFxHashSet,
            LibFxHashSetDefaultFunc,
            StdVec,
            StdVecDicotomie,
            StdLinkedList,
            LibAvlTree,
            LibRBTree
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
            SymmetricDifferenceScenario,
            SerializeScenario,
            DeserializeScenario
        ])
    };
}
