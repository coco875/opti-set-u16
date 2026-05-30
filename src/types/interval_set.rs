use interval_set::{IntervalSet, Interval,ToIntervalSet};
use super::{SetInt, SetIntConstruct};

pub struct IntervalSet2{
    interval_set : IntervalSet,
}

impl SetIntConstruct for IntervalSet2{
    fn new() -> Self {
        Self{
            interval_set: IntervalSet::empty()
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl SetInt for IntervalSet2 {
    fn clear(&mut self) {
        self.interval_set = IntervalSet::empty();
    }

    fn insert(&mut self, n: u16) {
        self.interval_set.insert(Interval::new(n as u32, n as u32));
    }

    fn remove(&mut self, n: u16) -> bool {
        let prev = self.contains(n);
        let to_remove = Interval::new(n as u32, n as u32).to_interval_set();
        self.interval_set = self.interval_set.clone().difference(to_remove);
        prev
    }

    fn contains(&self, n: u16) -> bool {
        self.interval_set.iter().any(|x| x.get_inf() <= n as u32 && x.get_sup() >= n as u32)
    }

    fn len(&self) -> usize {
        self.interval_set.size() as usize
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.interval_set.iter().flat_map(|x| (x.get_inf() as u16) ..=(x.get_sup() as u16)))
    }

    fn union_with(&mut self, other: &Self) {
        self.interval_set = self.interval_set.clone().union(other.interval_set.clone());
    }

    fn intersection_with(&mut self, other: &Self) {
        self.interval_set = self.interval_set.clone().intersection(other.interval_set.clone());
    }

    fn difference_with(&mut self, other: &Self) {
        self.interval_set = self.interval_set.clone().difference(other.interval_set.clone());
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        self.interval_set = self.interval_set.clone().symetric_difference(other.interval_set.clone());
    }
}