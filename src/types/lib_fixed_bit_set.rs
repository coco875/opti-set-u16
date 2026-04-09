use super::{SetInt, SetIntConstruct};
use fixedbitset::FixedBitSet;

pub struct LibFixedBitSet {
    set: FixedBitSet,
}

impl SetIntConstruct for LibFixedBitSet {
    fn new() -> Self {
        Self {
            set: FixedBitSet::with_capacity(65536),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            set: FixedBitSet::with_capacity(capacity.max(65536)),
        }
    }
}

impl SetInt for LibFixedBitSet {
    fn clear(&mut self) {
        self.set.clear();
    }

    fn insert(&mut self, n: u16) {
        self.set.insert(n as usize);
    }

    fn remove(&mut self, n: u16) -> bool {
        let prev = self.set.contains(n as usize);
        self.set.set(n as usize, false);
        prev
    }

    fn contains(&self, n: u16) -> bool {
        self.set.contains(n as usize)
    }

    fn len(&self) -> usize {
        self.set.count_ones(..)
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.set.ones().map(|i| i as u16))
    }

    fn union_with(&mut self, other: &Self) {
        self.set.union_with(&other.set);
    }

    fn intersection_with(&mut self, other: &Self) {
        self.set.intersect_with(&other.set);
    }

    fn difference_with(&mut self, other: &Self) {
        self.set.difference_with(&other.set);
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        self.set.symmetric_difference_with(&other.set);
    }
}
