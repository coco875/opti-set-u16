mod scenario;
mod timer;

use crate::register_set_int;
use crate::types::{BitSet, SetInt, SetIntConstruct, StdHashSet};

macro_rules! build_array_set {
    ([$($ty:ident),*]) => {
        &[
            $(
                || -> Box<dyn SetInt> {Box::new($ty::new())}
            ),*
        ]
    }
}

macro_rules! all_set {
    () => {
        register_set_int!(build_array_set)
    };
}

#[test]
fn test_insert_remove() {
    for set_builder in all_set!() {
        let mut set = set_builder();
        set.insert(5);
        assert!(set.contains(5));
        assert!(set.remove(5));
        assert!(!set.contains(5));
    }
}

#[test]
fn test_clear() {
    for set_builder in all_set!() {
        let mut set = set_builder();
        set.insert(10);
        set.insert(20);
        set.clear();
        assert!(!set.contains(10));
        assert!(!set.contains(20));
    }
}

#[test]
fn test_contains_not_present() {
    for set_builder in all_set!() {
        let mut set = set_builder();
        set.insert(5);
        assert!(!set.contains(10));
    }
}
