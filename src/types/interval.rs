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
        if self.contains(n) {
            return;
        }
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
        let mut merged: Vec<Interval> =
            Vec::with_capacity(self.intervals.len() + other.intervals.len());
        let mut i = 0;
        let mut j = 0;

        while i < self.intervals.len() && j < other.intervals.len() {
            let a = &self.intervals[i];
            let b = &other.intervals[j];

            let next = if a.start <= b.start {
                i += 1;
                a
            } else {
                j += 1;
                b
            };

            if let Some(last) = merged.last_mut() {
                if next.start <= last.end.saturating_add(1) {
                    last.end = last.end.max(next.end);
                } else {
                    merged.push(next.clone());
                }
            } else {
                merged.push(next.clone());
            }
        }

        while i < self.intervals.len() {
            let next = &self.intervals[i];
            i += 1;
            if let Some(last) = merged.last_mut() {
                if next.start <= last.end.saturating_add(1) {
                    last.end = last.end.max(next.end);
                } else {
                    merged.push(next.clone());
                }
            } else {
                merged.push(next.clone());
            }
        }

        while j < other.intervals.len() {
            let next = &other.intervals[j];
            j += 1;
            if let Some(last) = merged.last_mut() {
                if next.start <= last.end.saturating_add(1) {
                    last.end = last.end.max(next.end);
                } else {
                    merged.push(next.clone());
                }
            } else {
                merged.push(next.clone());
            }
        }

        self.intervals = merged;
    }

    fn intersection_with(&mut self, other: &Self) {
        let mut result = Vec::with_capacity(self.intervals.len().min(other.intervals.len()));
        let mut j = 0;

        for a in &self.intervals {
            let a_start = a.start as u32;
            let a_end = a.end as u32;

            while j < other.intervals.len() && (other.intervals[j].end as u32) < a_start {
                j += 1;
            }

            let mut k = j;
            while k < other.intervals.len() && (other.intervals[k].start as u32) <= a_end {
                let b_start = other.intervals[k].start as u32;
                let b_end = other.intervals[k].end as u32;

                let overlap_start = a_start.max(b_start);
                let overlap_end = a_end.min(b_end);

                if overlap_start <= overlap_end {
                    result.push(Interval::new(overlap_start as u16, overlap_end as u16));
                }
                k += 1;
            }
        }

        self.intervals = result;
    }

    fn difference_with(&mut self, other: &Self) {
        let mut result = Vec::with_capacity(self.intervals.len());
        let mut j = 0;

        for a in &self.intervals {
            let mut cur_start = a.start as u32;
            let cur_end = a.end as u32;

            while j < other.intervals.len() && (other.intervals[j].end as u32) < cur_start {
                j += 1;
            }

            let mut k = j;
            while k < other.intervals.len() && (other.intervals[k].start as u32) <= cur_end {
                let b_start = other.intervals[k].start as u32;
                let b_end = other.intervals[k].end as u32;

                if b_start > cur_start {
                    result.push(Interval::new(cur_start as u16, (b_start - 1) as u16));
                }

                if b_end >= cur_end {
                    cur_start = cur_end + 1;
                    break;
                } else {
                    cur_start = b_end + 1;
                }
                k += 1;
            }

            if cur_start <= cur_end {
                result.push(Interval::new(cur_start as u16, cur_end as u16));
            }
        }

        self.intervals = result;
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        let mut result = Vec::with_capacity(self.intervals.len() + other.intervals.len());
        let mut i = 0;
        let mut j = 0;

        while i < self.intervals.len() || j < other.intervals.len() {
            let a = if i < self.intervals.len() {
                Some(&self.intervals[i])
            } else {
                None
            };
            let b = if j < other.intervals.len() {
                Some(&other.intervals[j])
            } else {
                None
            };

            match (a, b) {
                (Some(interval_a), Some(interval_b)) => {
                    if interval_a.end < interval_b.start {
                        result.push(interval_a.clone());
                        i += 1;
                    } else if interval_b.end < interval_a.start {
                        result.push(interval_b.clone());
                        j += 1;
                    } else {
                        let overlap_start = interval_a.start.max(interval_b.start);
                        let overlap_end = interval_a.end.min(interval_b.end);

                        if interval_a.start < overlap_start {
                            result.push(Interval::new(interval_a.start, overlap_start - 1));
                        }
                        if interval_b.start < overlap_start {
                            result.push(Interval::new(interval_b.start, overlap_start - 1));
                        }

                        let a_remaining = overlap_end < interval_a.end;
                        let b_remaining = overlap_end < interval_b.end;

                        if a_remaining && b_remaining {
                            let next_start = overlap_end + 1;
                            if interval_a.end < interval_b.end {
                                result.push(Interval::new(next_start, interval_b.end));
                                i += 1;
                            } else if interval_b.end < interval_a.end {
                                result.push(Interval::new(next_start, interval_a.end));
                                j += 1;
                            } else {
                                i += 1;
                                j += 1;
                            }
                        } else if a_remaining {
                            i += 1;
                        } else if b_remaining {
                            j += 1;
                        } else {
                            i += 1;
                            j += 1;
                        }
                    }
                }
                (Some(interval_a), None) => {
                    result.push(interval_a.clone());
                    i += 1;
                }
                (None, Some(interval_b)) => {
                    result.push(interval_b.clone());
                    j += 1;
                }
                (None, None) => break,
            }
        }

        self.intervals = result;
    }
}
