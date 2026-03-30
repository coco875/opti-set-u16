use core::convert::Into;
use std::default::Default;

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
}

pub struct BitSet {
    bit_set: bit_set::BitSet<u16>,
}

impl SetIntConstruct for BitSet {
    fn new() -> Self {
        Self {
            bit_set: bit_set::BitSet::<u16>::default(),
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

pub struct StdHashSet {
    hash_set: std::collections::HashSet<u16>,
}

impl SetIntConstruct for StdHashSet {
    fn new() -> Self {
        Self {
            hash_set: std::collections::HashSet::new(),
        }
    }
}

impl SetInt for StdHashSet {
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
        for item in other.iter() {
            self.insert(item);
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        for item in self.iter().collect::<Vec<u16>>() {
            if !other.contains(item) {
                self.remove(item);
            }
        }
    }

    fn difference_with(&mut self, other: &Self) {
        for item in other.iter() {
            self.remove(item);
        }
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        let self_items: Vec<u16> = self.iter().collect();
        let other_items: Vec<u16> = other.iter().collect();

        for item in &self_items {
            if other.contains(*item) {
                self.remove(*item);
            }
        }

        for item in &other_items {
            if !self_items.contains(item) {
                self.insert(*item);
            }
        }
    }
}
