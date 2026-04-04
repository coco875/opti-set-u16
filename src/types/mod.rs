mod interval;
mod lib_bit_set;
mod lib_fx_hash_set;
mod lib_interval;
mod lib_roaring;
mod std_hash_set;
mod std_tree_set;
mod std_vec;

pub use interval::IntervalSet;
pub use lib_bit_set::LibBitSet;
pub use lib_fx_hash_set::LibFxHashSet;
pub use lib_interval::LibInterval;
pub use lib_roaring::LibRoaring;
pub use std_hash_set::StdHashSet;
pub use std_tree_set::StdTreeSet;
pub use std_vec::StdVec;

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
