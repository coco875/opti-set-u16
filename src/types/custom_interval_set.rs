use super::{SetInt, SetIntConstruct};

type T = u16;
pub struct IntervalResourceSet {
    ranges: Vec<(u64, u64)>, // trié, disjoint, non adjacent
    len: usize,
    _marker: core::marker::PhantomData<T>,
}

impl IntervalResourceSet {
    fn find_pos(&self, v: u64) -> usize {
        self.ranges
            .binary_search_by(|&(s, _)| {
                if s > v {
                    core::cmp::Ordering::Greater
                } else {
                    core::cmp::Ordering::Less
                }
            })
            .unwrap_err()
    }

    fn difference(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < self.ranges.len() {
            let (start, end) = self.ranges[i];

            while j < other.ranges.len() && other.ranges[j].1 < start {
                j += 1;
            }

            let mut cur = start;

            while j < other.ranges.len() && other.ranges[j].0 <= end {
                let (b_start, b_end) = other.ranges[j];

                if b_start > cur {
                    result.push((cur, b_start - 1));
                }

                if b_end + 1 > end {
                    cur = end + 1;
                    break;
                }

                cur = b_end + 1;
                j += 1;
            }

            if cur <= end {
                result.push((cur, end));
            }

            i += 1;
        }

        let len = result.iter().map(|(s, e)| (e - s + 1) as usize).sum();

        Self {
            ranges: result,
            len,
            _marker: core::marker::PhantomData,
        }
    }
}

impl SetIntConstruct for IntervalResourceSet {
    fn new() -> Self {
        Self {
            ranges: Vec::new(),
            len: 0,
            _marker: core::marker::PhantomData,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            ranges: Vec::with_capacity(capacity),
            len: 0,
            _marker: core::marker::PhantomData,
        }
    }
}

impl SetInt for IntervalResourceSet {
    fn insert(&mut self, value: T) {
        let v = value.into();
        let i = self.find_pos(v);

        // check précédent
        if i > 0 {
            let (s, e) = self.ranges[i - 1];
            if v >= s && v <= e {
                return;
            }
            if e + 1 == v {
                self.ranges[i - 1].1 += 1;
                self.len += 1;

                // fusion avec suivant
                if i < self.ranges.len() && self.ranges[i - 1].1 + 1 >= self.ranges[i].0 {
                    let next = self.ranges.remove(i);
                    self.ranges[i - 1].1 = next.1;
                }

                return;
            }
        }

        // check suivant
        if i < self.ranges.len() {
            let (s, _) = self.ranges[i];
            if v + 1 == s {
                self.ranges[i].0 -= 1;
                self.len += 1;
                return;
            }
        }

        // insertion simple
        self.ranges.insert(i, (v, v));
        self.len += 1;
    }

    fn contains(&self, value: T) -> bool {
        let v = (value).into();
        let i = self.find_pos(v);

        if i > 0 {
            let (s, e) = self.ranges[i - 1];
            v >= s && v <= e
        } else {
            false
        }
    }

    fn remove(&mut self, value: T) -> bool {
        let v = (value).into();
        let i = self.find_pos(v);

        if i == 0 {
            return false;
        }

        let (s, e) = self.ranges[i - 1];

        if v < s || v > e {
            return false;
        }

        // cas split
        if s < v && v < e {
            self.ranges[i - 1].1 = v - 1;
            self.ranges.insert(i, (v + 1, e));
        }
        // début
        else if v == s {
            self.ranges[i - 1].0 += 1;
            if self.ranges[i - 1].0 > self.ranges[i - 1].1 {
                self.ranges.remove(i - 1);
            }
        }
        // fin
        else {
            self.ranges[i - 1].1 -= 1;
            if self.ranges[i - 1].0 > self.ranges[i - 1].1 {
                self.ranges.remove(i - 1);
            }
        }

        self.len -= 1;
        true
    }

    fn len(&self) -> usize {
        self.len
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16>> {
        let mut out = Vec::with_capacity(self.len);

        for &(s, e) in &self.ranges {
            for v in s..=e {
                if let Ok(t) = T::try_from(v) {
                    out.push(t);
                }
            }
        }
        Box::new(out.into_iter())
    }

    fn clear(&mut self) {
        self.ranges.clear();
        self.len = 0;
    }

    fn union_with(&mut self, other: &Self) {
        let mut result = Vec::with_capacity(self.ranges.len() + other.ranges.len());

        let mut i = 0;
        let mut j = 0;

        let push = |new: (u64, u64), result: &mut Vec<(u64, u64)>| {
            if let Some(last) = result.last_mut()
                && last.1 + 1 >= new.0
            {
                last.1 = last.1.max(new.1);
                return;
            }
            result.push(new);
        };

        while i < self.ranges.len() || j < other.ranges.len() {
            let next = if j >= other.ranges.len()
                || (i < self.ranges.len() && self.ranges[i].0 <= other.ranges[j].0)
            {
                let r = self.ranges[i];
                i += 1;
                r
            } else {
                let r = other.ranges[j];
                j += 1;
                r
            };

            push(next, &mut result);
        }

        let len = result.iter().map(|(s, e)| (e - s + 1) as usize).sum();
        self.len = len;
        self.ranges = result;
    }

    fn intersection_with(&mut self, other: &Self) {
        let mut result = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < self.ranges.len() && j < other.ranges.len() {
            let (a_start, a_end) = self.ranges[i];
            let (b_start, b_end) = other.ranges[j];

            let start = a_start.max(b_start);
            let end = a_end.min(b_end);

            if start <= end {
                result.push((start, end));
            }

            if a_end < b_end {
                i += 1;
            } else {
                j += 1;
            }
        }

        let len = result.iter().map(|(s, e)| (e - s + 1) as usize).sum();

        self.len = len;
        self.ranges = result;
    }

    fn difference_with(&mut self, other: &Self) {
        let mut result = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < self.ranges.len() {
            let (start, end) = self.ranges[i];

            while j < other.ranges.len() && other.ranges[j].1 < start {
                j += 1;
            }

            let mut cur = start;

            while j < other.ranges.len() && other.ranges[j].0 <= end {
                let (b_start, b_end) = other.ranges[j];

                if b_start > cur {
                    result.push((cur, b_start - 1));
                }

                if b_end + 1 > end {
                    cur = end + 1;
                    break;
                }

                cur = b_end + 1;
                j += 1;
            }

            if cur <= end {
                result.push((cur, end));
            }

            i += 1;
        }

        let len = result.iter().map(|(s, e)| (e - s + 1) as usize).sum();

        self.len = len;
        self.ranges = result;
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        self.difference_with(other);
        let other_diff = other.difference(self);
        self.union_with(&other_diff);
    }
}
