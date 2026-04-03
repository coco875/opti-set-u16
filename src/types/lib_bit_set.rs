use super::{SetInt, SetIntConstruct};

pub struct BitSet {
    bit_set: bit_set::BitSet<u16>,
}

impl SetIntConstruct for BitSet {
    fn new() -> Self {
        Self {
            bit_set: bit_set::BitSet::<u16>::new_general(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            bit_set: bit_set::BitSet::<u16>::with_capacity_general(capacity),
        }
    }
}

impl SetInt for BitSet {
    fn clear(&mut self) {
        self.bit_set.make_empty();
    }

    fn insert(&mut self, n: u16) {
        self.bit_set.insert(n.into());
    }

    fn remove(&mut self, n: u16) -> bool {
        self.bit_set.remove(n.into())
    }

    fn contains(&self, n: u16) -> bool {
        self.bit_set.contains(n.into())
    }

    fn len(&self) -> usize {
        self.bit_set.count()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        let elems: Vec<u16> = self.bit_set.iter().map(|e| e as u16).collect();
        Box::new(elems.into_iter())
    }

    fn union_with(&mut self, other: &Self) {
        for item in other.iter() {
            self.insert(item);
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        self.bit_set.intersect_with(&other.bit_set);
    }

    fn difference_with(&mut self, other: &Self) {
        self.bit_set.difference_with(&other.bit_set);
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        self.bit_set.symmetric_difference_with(&other.bit_set);
    }
}
