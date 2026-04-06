use super::{SetInt, SetIntConstruct};
use avl::AvlTreeSet;

pub struct LibAvlTree {
    tree_set: AvlTreeSet<u16>,
}

impl SetIntConstruct for LibAvlTree {
    fn new() -> Self {
        Self {
            tree_set: AvlTreeSet::new(),
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self {
            tree_set: AvlTreeSet::new(),
        }
    }
}

impl SetInt for LibAvlTree {
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
        self.tree_set = self.tree_set.union(&other.tree_set).copied().collect();
    }

    fn intersection_with(&mut self, other: &Self) {
        self.tree_set = self
            .tree_set
            .intersection(&other.tree_set)
            .copied()
            .collect();
    }

    fn difference_with(&mut self, other: &Self) {
        self.tree_set = self.tree_set.difference(&other.tree_set).copied().collect();
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        self.tree_set = self
            .tree_set
            .symmetric_difference(&other.tree_set)
            .copied()
            .collect();
    }
}
