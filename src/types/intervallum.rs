use std::ops::Add;

use super::{SetInt, SetIntConstruct};
use interval::{Interval, IntervalSet};
use gcollections::ops::*;

pub struct Intervallum {
    interval_set: IntervalSet<u16>,
}

impl SetInt for Intervallum{
    fn clear(&mut self) {
        self.interval_set = IntervalSet::empty();
    }

    fn insert(&mut self, n: u16) {
        self.interval_set.extend([Interval::singleton(n)]);
    }

    fn remove(&mut self, n: u16) -> bool {
        let prev = self.interval_set.contains(&n);
        self.interval_set = self.interval_set.difference(&n);
        prev
    }

    fn contains(&self, n: u16) -> bool {
        self.interval_set.contains(&n)
    }

    fn len(&self) -> usize {
        self.interval_set.iter().len()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.interval_set.iter().flat_map(|i| i.lower()..=i.upper()))
    }

    fn union_with(&mut self, other: &Self) {
        self.interval_set.union(&other.interval_set);
    }

    fn intersection_with(&mut self, other: &Self) {
        self.interval_set.intersection(&other.interval_set);
    }

    fn difference_with(&mut self, other: &Self) {
        self.interval_set.difference(&other.interval_set);
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        self.interval_set.symmetric_difference(&other.interval_set);
    }
}

impl SetIntConstruct for Intervallum{
    fn new() -> Self {
        Self{
            interval_set: IntervalSet::empty(),
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}