use super::{SetInt, SetIntConstruct};
use rust_intervals::{Interval, IntervalSet};

pub struct LibInterval {
    interval_set: IntervalSet<u16>,
}

impl SetIntConstruct for LibInterval {
    fn new() -> Self {
        Self {
            interval_set: IntervalSet::empty(),
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self {
            interval_set: IntervalSet::empty(),
        }
    }
}

impl SetInt for LibInterval {
    fn clear(&mut self) {
        self.interval_set.clear();
    }

    fn insert(&mut self, n: u16) {
        self.interval_set.add(Interval::new_single(n));
    }

    fn remove(&mut self, n: u16) -> bool {
        let was_present = self.contains(n);
        self.interval_set.remove(n);
        was_present
    }

    fn contains(&self, n: u16) -> bool {
        self.interval_set.contains(n)
    }

    fn len(&self) -> usize {
        self.iter().count()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        let elems: Vec<u16> = self
            .interval_set
            .iter()
            .flat_map(|interval| interval.iter())
            .collect();
        Box::new(elems.into_iter())
    }

    fn union_with(&mut self, other: &Self) {
        for interval in other.interval_set.iter() {
            self.interval_set.add(*interval);
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        self.interval_set = self.interval_set.intersection_set(&other.interval_set);
    }

    fn difference_with(&mut self, other: &Self) {
        for interval in other.interval_set.iter() {
            self.interval_set.remove_interval(*interval);
        }
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        let self_items: Vec<u16> = self.iter().collect();
        let other_items: Vec<u16> = other.iter().collect();

        self.clear();

        for item in &self_items {
            if !other.contains(*item) {
                self.insert(*item);
            }
        }

        for item in &other_items {
            if !self_items.contains(item) {
                self.insert(*item);
            }
        }
    }
}
