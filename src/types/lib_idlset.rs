use super::{SetInt, SetIntConstruct};
use idlset::v2::IDLBitRange;
use idlset::AndNot;

pub struct LibIdlset {
    set: IDLBitRange,
}

impl SetIntConstruct for LibIdlset {
    fn new() -> Self {
        Self {
            set: IDLBitRange::new(),
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl SetInt for LibIdlset {
    fn clear(&mut self) {
        self.set = IDLBitRange::new();
    }

    fn insert(&mut self, n: u16) {
        self.set.insert_id(n as u64);
    }

    fn remove(&mut self, n: u16) -> bool {
        let has = self.set.contains(n as u64);
        if has {
            self.set.remove_id(n as u64);
        }
        has
    }

    fn contains(&self, n: u16) -> bool {
        self.set.contains(n as u64)
    }

    fn len(&self) -> usize {
        self.set.len()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.set.into_iter().map(|i| i as u16))
    }

    fn union_with(&mut self, other: &Self) {
        self.set = &self.set | &other.set;
    }

    fn intersection_with(&mut self, other: &Self) {
        self.set = &self.set & &other.set;
    }

    fn difference_with(&mut self, other: &Self) {
        self.set = (&self.set).andnot(&other.set);
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        let a_min_b = (&self.set).andnot(&other.set);
        let b_min_a = (&other.set).andnot(&self.set);
        self.set = a_min_b | b_min_a;
    }
}
