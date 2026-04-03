use super::{SetInt, SetIntConstruct};

#[derive(Clone, Debug)]
struct Interval {
    start: u16,
    end: u16,
}

impl Interval {
    fn new(start: u16, end: u16) -> Self {
        Self { start, end }
    }

    fn contains(&self, n: u16) -> bool {
        n >= self.start && n <= self.end
    }

    fn overlaps(&self, other: &Interval) -> bool {
        self.start <= other.end.saturating_add(1) && other.start <= self.end.saturating_add(1)
    }

    fn merge(&self, other: &Interval) -> Interval {
        Interval::new(self.start.min(other.start), self.end.max(other.end))
    }
}

pub struct IntervalSet {
    intervals: Vec<Interval>,
}

impl SetIntConstruct for IntervalSet {
    fn new() -> Self {
        Self {
            intervals: Vec::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            intervals: Vec::with_capacity(capacity),
        }
    }
}

impl SetInt for IntervalSet {
    fn clear(&mut self) {
        self.intervals.clear();
    }

    fn insert(&mut self, n: u16) {
        let new_interval = Interval::new(n, n);
        let mut merged = vec![new_interval];

        for interval in self.intervals.drain(..) {
            if interval.overlaps(merged.last().unwrap()) {
                let last = merged.pop().unwrap();
                merged.push(interval.merge(&last));
            } else {
                merged.push(interval);
            }
        }

        merged.sort_by_key(|interval| interval.start);
        self.intervals = merged;
    }

    fn remove(&mut self, n: u16) -> bool {
        let mut found = false;
        let mut new_intervals = Vec::with_capacity(self.intervals.len() + 1);

        for interval in self.intervals.drain(..) {
            if interval.contains(n) {
                found = true;
                if interval.start < n {
                    new_intervals.push(Interval::new(interval.start, n - 1));
                }
                if interval.end > n {
                    new_intervals.push(Interval::new(n + 1, interval.end));
                }
            } else {
                new_intervals.push(interval);
            }
        }

        self.intervals = new_intervals;
        found
    }

    fn contains(&self, n: u16) -> bool {
        self.intervals.iter().any(|interval| interval.contains(n))
    }

    fn len(&self) -> usize {
        self.intervals
            .iter()
            .map(|interval| (interval.end - interval.start + 1) as usize)
            .sum()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        let mut elems = Vec::with_capacity(self.len());
        for interval in &self.intervals {
            for n in interval.start..=interval.end {
                elems.push(n);
            }
        }
        Box::new(elems.into_iter())
    }

    fn union_with(&mut self, other: &Self) {
        for item in other.iter() {
            self.insert(item);
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        let to_remove: Vec<u16> = self.iter().filter(|item| !other.contains(*item)).collect();
        for item in to_remove {
            self.remove(item);
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
