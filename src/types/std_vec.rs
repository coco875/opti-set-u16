use super::{SetInt, SetIntConstruct};

pub struct StdVec {
    elements: Vec<u16>,
}

impl SetIntConstruct for StdVec {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Vec::with_capacity(capacity),
        }
    }
}

impl SetInt for StdVec {
    fn clear(&mut self) {
        self.elements.clear();
    }

    fn insert(&mut self, n: u16) {
        if !self.contains(n) {
            self.elements.push(n);
        }
    }

    fn remove(&mut self, n: u16) -> bool {
        match self.elements.iter().position(|&x| x == n) {
            Some(pos) => {
                self.elements.swap_remove(pos);
                true
            }
            None => false,
        }
    }

    fn contains(&self, n: u16) -> bool {
        self.elements.contains(&n)
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.elements.iter().copied())
    }

    fn union_with(&mut self, other: &Self) {
        for &v in &other.elements {
            self.insert(v);
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        self.elements.retain(|v| other.contains(*v));
    }

    fn difference_with(&mut self, other: &Self) {
        self.elements.retain(|v| !other.contains(*v));
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        let self_set: std::collections::HashSet<u16> = self.elements.iter().copied().collect();
        let other_set: std::collections::HashSet<u16> = other.elements.iter().copied().collect();

        self.elements.clear();
        for &v in &self_set {
            if !other_set.contains(&v) {
                self.elements.push(v);
            }
        }
        for &v in &other_set {
            if !self_set.contains(&v) {
                self.elements.push(v);
            }
        }
    }
}
