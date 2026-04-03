use super::{SetInt, SetIntConstruct};

pub struct StdTreeSet {
    tree_set: std::collections::BTreeSet<u16>,
}

impl SetIntConstruct for StdTreeSet {
    fn new() -> Self {
        Self {
            tree_set: std::collections::BTreeSet::new(),
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self {
            tree_set: std::collections::BTreeSet::new(),
        }
    }
}

impl SetInt for StdTreeSet {
    fn clear(&mut self) {
        self.tree_set.clear();
    }

    fn insert(&mut self, n: u16) {
        self.tree_set.insert(n);
    }

    fn remove(&mut self, n: u16) -> bool {
        self.tree_set.remove(&n)
    }

    fn contains(&self, n: u16) -> bool {
        self.tree_set.contains(&n)
    }

    fn len(&self) -> usize {
        self.tree_set.len()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.tree_set.iter().copied())
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
