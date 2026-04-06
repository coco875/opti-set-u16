use super::{SetInt, SetIntConstruct};
use rbtree::RBTree;

pub struct LibRBTree {
    tree: RBTree<u16, ()>,
}

impl SetIntConstruct for LibRBTree {
    fn new() -> Self {
        Self {
            tree: RBTree::new(),
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self {
            tree: RBTree::new(),
        }
    }
}

impl SetInt for LibRBTree {
    fn clear(&mut self) {
        self.tree.clear();
    }

    fn insert(&mut self, n: u16) {
        self.tree.replace_or_insert(n, ());
    }

    fn remove(&mut self, n: u16) -> bool {
        self.tree.remove(&n).is_some()
    }

    fn contains(&self, n: u16) -> bool {
        self.tree.contains_key(&n)
    }

    fn len(&self) -> usize {
        self.tree.len()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.tree.keys().copied())
    }

    fn union_with(&mut self, other: &Self) {
        for &key in other.tree.keys() {
            self.tree.replace_or_insert(key, ());
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        let to_remove: Vec<u16> = self
            .tree
            .keys()
            .filter(|k| !other.tree.contains_key(k))
            .copied()
            .collect();
        for key in to_remove {
            self.tree.remove(&key);
        }
    }

    fn difference_with(&mut self, other: &Self) {
        for &key in other.tree.keys() {
            self.tree.remove(&key);
        }
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        let to_remove: Vec<u16> = self
            .tree
            .keys()
            .filter(|k| other.tree.contains_key(k))
            .copied()
            .collect();
        let to_insert: Vec<u16> = other
            .tree
            .keys()
            .filter(|k| !self.tree.contains_key(k))
            .copied()
            .collect();
        for key in to_remove {
            self.tree.remove(&key);
        }
        for key in to_insert {
            self.tree.insert(key, ());
        }
    }
}
