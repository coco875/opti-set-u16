use crate::register_set_int;
use crate::types::*;
use proptest::prelude::*;
use std::any::Any;

#[derive(Debug)]
pub struct TypeError;

pub struct DynSetIntStruct<T: SetInt> {
    bit_set: T,
}

impl<T: SetInt> DynSetIntStruct<T> {
    pub fn new(bit_set: T) -> Self {
        Self { bit_set }
    }
}

pub trait DynSetInt {
    fn as_any(&self) -> &dyn Any;
    fn clear(&mut self);
    fn insert(&mut self, n: u16);
    fn remove(&mut self, n: u16) -> bool;
    fn contains(&self, n: u16) -> bool;
    fn len(&self) -> usize;
    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_>;
    fn to_bytes(&self) -> [u8; 8192];
    fn from_bytes(&mut self, bytes: &[u8]);
    fn union_with(&mut self, other: &dyn DynSetInt) -> Result<(), TypeError>;
    fn intersection_with(&mut self, other: &dyn DynSetInt) -> Result<(), TypeError>;
    fn difference_with(&mut self, other: &dyn DynSetInt) -> Result<(), TypeError>;
    fn symmetric_difference_with(&mut self, other: &dyn DynSetInt) -> Result<(), TypeError>;
}

// Blanket impl : tout T qui implémente SetInt obtient DynSetInt gratuitement
impl<T: SetInt> DynSetInt for DynSetIntStruct<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clear(&mut self) {
        self.bit_set.clear();
    }
    fn insert(&mut self, n: u16) {
        self.bit_set.insert(n);
    }
    fn remove(&mut self, n: u16) -> bool {
        self.bit_set.remove(n)
    }
    fn contains(&self, n: u16) -> bool {
        self.bit_set.contains(n)
    }
    fn len(&self) -> usize {
        self.bit_set.len()
    }
    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        self.bit_set.iter()
    }

    fn to_bytes(&self) -> [u8; 8192] {
        self.bit_set.to_bytes()
    }

    fn from_bytes(&mut self, bytes: &[u8]) {
        self.bit_set.from_bytes(bytes);
    }

    fn union_with(&mut self, other: &dyn DynSetInt) -> Result<(), TypeError> {
        let other = other.as_any().downcast_ref::<Self>().ok_or(TypeError)?;
        self.bit_set.union_with(&other.bit_set);
        Ok(())
    }
    fn intersection_with(&mut self, other: &dyn DynSetInt) -> Result<(), TypeError> {
        let other = other.as_any().downcast_ref::<Self>().ok_or(TypeError)?;
        self.bit_set.intersection_with(&other.bit_set);
        Ok(())
    }
    fn difference_with(&mut self, other: &dyn DynSetInt) -> Result<(), TypeError> {
        let other = other.as_any().downcast_ref::<Self>().ok_or(TypeError)?;
        self.bit_set.difference_with(&other.bit_set);
        Ok(())
    }
    fn symmetric_difference_with(&mut self, other: &dyn DynSetInt) -> Result<(), TypeError> {
        let other = other.as_any().downcast_ref::<Self>().ok_or(TypeError)?;
        self.bit_set.symmetric_difference_with(&other.bit_set);
        Ok(())
    }
}

macro_rules! build_array_set {
    ([$($ty:ident),*]) => {
        &[
            $(
                (
                    (|| -> Box<dyn DynSetInt> {Box::new(DynSetIntStruct::new($ty::new()))}) as fn() -> Box<dyn DynSetInt>,
                    stringify!($ty),
                )
            ),*
        ]
    }
}

macro_rules! all_set {
    () => {
        register_set_int!(build_array_set)
    };
}

proptest! {
    #[test]
    fn proptest_insert_contains(refs in any::<u16>()) {
        for (set_builder, name) in all_set!() {
            let mut set = set_builder();
            set.insert(refs);
            prop_assert!(set.contains(refs), "insert fail in {name} with {refs}");
        }
    }

    #[test]
    fn proptest_insert_remove(refs in any::<u16>()) {
        for (set_builder, name) in all_set!() {
            let mut set = set_builder();
            set.insert(refs);
            prop_assert!(set.remove(refs), "remove failed in {name}");
            prop_assert!(!set.contains(refs), "contains after remove failed in {name}");
        }
    }

    #[test]
    fn proptest_len_after_insert_remove(refs in any::<u16>()) {
        for (set_builder, name) in all_set!() {
            let mut set = set_builder();
            prop_assert_eq!(set.len(), 0, "initial len should be 0 in {name}", name = name);
            set.insert(refs);
            prop_assert_eq!(set.len(), 1, "len should be 1 after insert in {name}", name = name);
            set.remove(refs);
            prop_assert_eq!(set.len(), 0, "len should be 0 after remove in {name}", name = name);
        }
    }

    #[test]
    fn proptest_clear(refs in any::<u16>()) {
        for (set_builder, name) in all_set!() {
            let mut set = set_builder();
            set.insert(refs);
            set.clear();
            prop_assert_eq!(set.len(), 0, "len should be 0 after clear in {name}", name = name);
            prop_assert!(!set.contains(refs), "contains should be false after clear in {name}");
        }
    }

    #[test]
    fn proptest_multiple_inserts(refs in proptest::collection::vec(any::<u16>(), 0..100)) {
        for (set_builder, name) in all_set!() {
            let mut set = set_builder();
            for &v in &refs {
                set.insert(v);
            }
            prop_assert_eq!(set.len(), refs.iter().collect::<std::collections::HashSet<_>>().len(), "len mismatch after multiple inserts in {name}", name = name);
        }
    }

    #[test]
    fn proptest_duplicate_insert(refs in any::<u16>()) {
        for (set_builder, name) in all_set!() {
            let mut set = set_builder();
            set.insert(refs);
            set.insert(refs);
            prop_assert_eq!(set.len(), 1, "len should be 1 after duplicate insert in {name}", name = name);
        }
    }

    #[test]
    fn proptest_union_with(a in proptest::collection::vec(any::<u16>(), 0..50), b in proptest::collection::vec(any::<u16>(), 0..50)) {
        for (set_builder, name) in all_set!() {
            let mut set1 = set_builder();
            let mut set2 = set_builder();
            for &v in &a { set1.insert(v); }
            for &v in &b { set2.insert(v); }
            prop_assert!(set1.union_with(&*set2).is_ok(), "union_with failed in {name}");
            let expected: std::collections::HashSet<u16> = a.iter().chain(b.iter()).copied().collect();
            prop_assert_eq!(set1.len(), expected.len(), "len mismatch after union in {name}", name = name);
            for v in expected {
                prop_assert!(set1.contains(v), "missing element after union in {name}");
            }
        }
    }

    #[test]
    fn proptest_intersection_with(a in proptest::collection::vec(any::<u16>(), 0..50), b in proptest::collection::vec(any::<u16>(), 0..50)) {
        for (set_builder, name) in all_set!() {
            let mut set1 = set_builder();
            let mut set2 = set_builder();
            for &v in &a { set1.insert(v); }
            for &v in &b { set2.insert(v); }
            prop_assert!(set1.intersection_with(&*set2).is_ok(), "intersection_with failed in {name}");
            let set_a: std::collections::HashSet<u16> = a.iter().copied().collect();
            let set_b: std::collections::HashSet<u16> = b.iter().copied().collect();
            let expected: std::collections::HashSet<u16> = set_a.intersection(&set_b).copied().collect();
            for v in expected {
                prop_assert!(set1.contains(v), "missing intersection element in {name}");
            }
            for &v in a.iter().chain(b.iter()) {
                if set_a.contains(&v) && set_b.contains(&v) {
                    prop_assert!(set1.contains(v), "missing common element in {name}");
                }
            }
        }
    }

    #[test]
    fn proptest_difference_with(a in proptest::collection::vec(any::<u16>(), 0..50), b in proptest::collection::vec(any::<u16>(), 0..50)) {
        for (set_builder, name) in all_set!() {
            let mut set1 = set_builder();
            let mut set2 = set_builder();
            for &v in &a { set1.insert(v); }
            for &v in &b { set2.insert(v); }
            prop_assert!(set1.difference_with(&*set2).is_ok(), "difference_with failed in {name}");
            let set_b: std::collections::HashSet<u16> = b.iter().copied().collect();
            for &v in a.iter() {
                if !set_b.contains(&v) {
                    prop_assert!(set1.contains(v), "element should remain after difference in {name}");
                }
            }
            for &v in b.iter() {
                prop_assert!(!set1.contains(v), "element should be removed by difference in {name}");
            }
        }
    }

    #[test]
    fn proptest_symmetric_difference_with(a in proptest::collection::vec(any::<u16>(), 0..50), b in proptest::collection::vec(any::<u16>(), 0..50)) {
        for (set_builder, name) in all_set!() {
            let mut set1 = set_builder();
            let mut set2 = set_builder();
            for &v in &a { set1.insert(v); }
            for &v in &b { set2.insert(v); }
            prop_assert!(set1.symmetric_difference_with(&*set2).is_ok(), "symmetric_difference_with failed in {name}");
            let set_a: std::collections::HashSet<u16> = a.iter().copied().collect();
            let set_b: std::collections::HashSet<u16> = b.iter().copied().collect();
            for &v in a.iter() {
                if !set_b.contains(&v) {
                    prop_assert!(set1.contains(v), "element from a should be present in {name}");
                }
            }
            for &v in b.iter() {
                if !set_a.contains(&v) {
                    prop_assert!(set1.contains(v), "element from b should be present in {name}");
                }
            }
            for &v in a.iter().chain(b.iter()) {
                if set_a.contains(&v) && set_b.contains(&v) {
                    prop_assert!(!set1.contains(v), "common element should not be present in {name}");
                }
            }
        }
    }

    #[test]
    fn proptest_iter(refs in proptest::collection::vec(any::<u16>(), 0..100)) {
        for (set_builder, name) in all_set!() {
            let mut set = set_builder();
            for &v in &refs {
                set.insert(v);
            }
            let expected: std::collections::HashSet<u16> = refs.iter().copied().collect();
            let iter_result: std::collections::HashSet<u16> = set.iter().collect();
            prop_assert_eq!(iter_result, expected, "iter result mismatch in {name}", name = name);
        }
    }

    #[test]
    fn proptest_to_bytes_from_bytes(refs in proptest::collection::vec(any::<u16>(), 0..100)) {
        for (set_builder, name) in all_set!() {
            let mut set1 = set_builder();
            for &v in &refs {
                set1.insert(v);
            }

            let bytes = set1.to_bytes();

            let mut set2 = set_builder();
            set2.from_bytes(&bytes);

            let expected: std::collections::HashSet<u16> = refs.iter().copied().collect();
            prop_assert_eq!(set2.len(), expected.len(), "len mismatch after from_bytes in {name}", name=name);

            for v in expected {
                prop_assert!(set2.contains(v), "missing element after from_bytes in {name}", name=name);
            }
        }
    }
}
