#[macro_export]
macro_rules! build_array_inner {
    // Base case: no more types → wrap accumulated entries in &[]
    ([$($done:expr),*], [], [$($s:ident),*]) => {
        &[$($done),*]
    };
    // Recursive case: process one type, accumulate its entries, then recurse on the rest
    ([$($done:expr),*], [$ty:ident $(,$rest:ident)*], [$($s:ident),*]) => {
        build_array_inner!(
            [
                $($done,)*
                $((
                    (|cap, fill, data, seed| -> Box<dyn Scenario> {
                        Box::new($s::<$ty>::new(cap, fill, data, seed))
                    }) as ScenarioBuilder,
                    stringify!($s),
                    stringify!($ty)
                )),*
            ],
            [$($rest),*],
            [$($s),*]
        )
    };
}

#[macro_export]
macro_rules! build_array {
    ([$($ty:ident),*], $scenarios:tt) => {
        build_array_inner!([], [$($ty),*], $scenarios)
    };
}

#[macro_export]
macro_rules! build_array_with_scenarios {
    ([$($ty:ident),*]) => {
        register_scenario!(build_array, [$($ty),*])
    };
}

#[macro_export]
macro_rules! all_scenarios {
    () => {
        register_set_int!(build_array_with_scenarios)
    };
}
