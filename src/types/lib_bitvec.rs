use super::{SetInt, SetIntConstruct};
use bitvec::prelude::*;

pub struct LibBitVec {
    set: BitVec<usize, Lsb0>,
}

impl SetIntConstruct for LibBitVec {
    fn new() -> Self {
        Self {
            set: bitvec![usize, Lsb0; 0; 65536],
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            set: bitvec![usize, Lsb0; 0; capacity.max(65536)],
        }
    }
}

impl SetInt for LibBitVec {
    fn clear(&mut self) {
        self.set.fill(false);
    }

    fn insert(&mut self, n: u16) {
        self.set.set(n as usize, true);
    }

    fn remove(&mut self, n: u16) -> bool {
        let prev = self.set[n as usize];
        self.set.set(n as usize, false);
        prev
    }

    fn contains(&self, n: u16) -> bool {
        self.set[n as usize]
    }

    fn len(&self) -> usize {
        self.set.count_ones()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.set.iter_ones().map(|i| i as u16))
    }

    fn union_with(&mut self, other: &Self) {
        self.set |= &other.set;
    }

    fn intersection_with(&mut self, other: &Self) {
        self.set &= &other.set;
    }

    fn difference_with(&mut self, other: &Self) {
        // bitvec doesn't have a direct difference operator, so we do A & !B
        let mut tmp = other.set.clone();
        tmp = !tmp;
        self.set &= &tmp;
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        self.set ^= &other.set;
    }
}
