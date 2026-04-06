use super::{SetInt, SetIntConstruct};
use rustc_hash::FxHashSet;

pub struct LibFxHashSetDefaultFunc {
    hash_set: FxHashSet<u16>,
}

impl SetIntConstruct for LibFxHashSetDefaultFunc {
    fn new() -> Self {
        Self {
            hash_set: FxHashSet::default(),
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self {
            hash_set: FxHashSet::default(),
        }
    }
}

impl SetInt for LibFxHashSetDefaultFunc {
    fn clear(&mut self) {
        self.hash_set.clear();
    }

    fn insert(&mut self, n: u16) {
        self.hash_set.insert(n);
    }

    fn remove(&mut self, n: u16) -> bool {
        self.hash_set.remove(&n)
    }

    fn contains(&self, n: u16) -> bool {
        self.hash_set.contains(&n)
    }

    fn len(&self) -> usize {
        self.hash_set.len()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.hash_set.iter().copied())
    }

    fn union_with(&mut self, other: &Self) {
        self.hash_set = self.hash_set.union(&other.hash_set).copied().collect();
    }

    fn intersection_with(&mut self, other: &Self) {
        self.hash_set = self
            .hash_set
            .intersection(&other.hash_set)
            .copied()
            .collect();
    }

    fn difference_with(&mut self, other: &Self) {
        self.hash_set = self.hash_set.difference(&other.hash_set).copied().collect();
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        self.hash_set = self
            .hash_set
            .symmetric_difference(&other.hash_set)
            .copied()
            .collect();
    }
}
