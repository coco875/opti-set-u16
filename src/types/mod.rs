mod lib_bit_set;
mod std_hash_set;

pub use lib_bit_set::BitSet;
pub use std_hash_set::StdHashSet;

pub trait SetInt: 'static {
    fn clear(&mut self);
    fn insert(&mut self, n: u16);
    fn remove(&mut self, n: u16) -> bool;
    fn contains(&self, n: u16) -> bool;
    fn len(&self) -> usize;
    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_>;
    fn union_with(&mut self, other: &Self);
    fn intersection_with(&mut self, other: &Self);
    fn difference_with(&mut self, other: &Self);
    fn symmetric_difference_with(&mut self, other: &Self);
}

pub trait SetIntConstruct: SetInt {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
}
