use super::{SetInt, SetIntConstruct};
use rustc_hash::FxHashSet;

pub struct LibFxHashSet {
    hash_set: FxHashSet<u16>,
}

impl SetIntConstruct for LibFxHashSet {
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

impl SetInt for LibFxHashSet {
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
