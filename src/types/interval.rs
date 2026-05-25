use std::{arch::global_asm};

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
        self.start < other.start && self.end > other.start && self.end < other.end
    }

    fn merge(&self, other: &Interval) -> Interval {
        Interval::new(self.start.min(other.start), self.end.max(other.end))
    }

    // test add by Max
    fn beforeP(&self, other: &Interval) -> bool{
        self.end < other.start
    }

    fn meets(&self, other: &Interval) -> bool{
        self.start < self.end && self.end == other.start && other.start < other.end
    }

    fn starts(&self, other: &Interval) -> bool {
        self.start == other.start && self.end < other.end
    }

    fn containedByP(&self, other: &Interval) -> bool{
        self.start > other.start && self.end < other.end 
    }

    fn finishes(&self, other: &Interval) -> bool{
        self.start > other.start && self.end == other.end
    }

    fn equalP(&self, other: &Interval) -> bool{
        self.start == other.start && self.end == other.end
    }

    fn finishedBy(&self, other: &Interval) -> bool {
        self.start < other.start && self.end == other.end
    }

    fn containsP(&self, other: &Interval) -> bool {
        self.start < other.start && self.end > other.end
    }

    fn startedBy(&self, other: &Interval) -> bool{
        self.start == other.start && self.end > other.end
    }

    fn overlappedBy(&self, other: &Interval) -> bool{
        self.start > other.start && self.start < other.end && self.end > other.end 
    }

    fn metBy(&self, other: &Interval) -> bool{
        self.start == other.end && other.start < other.end && self.end > self.start
    }

    fn afterP(&self, other: &Interval) -> bool{
        self.start >  other.end
    }

    fn fstEmpty(&self, other: &Interval) -> bool{
        // self.empt
        false
    }
 }


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_beforeP(){
        let a = Interval::new(0,2);
        let b = Interval::new(3,4);
        let c = Interval::new(3,3);
        let d = Interval::new(2,2);
        
        assert_eq!(a.beforeP(&b), true);        
        assert_eq!(a.beforeP(&c), true);        
        assert_eq!(d.beforeP(&b), true);        
        assert_eq!(d.beforeP(&c), true);        
        assert_eq!(a.beforeP(&d), false);        

    }



    #[test]
    fn test_meets(){
        let a = Interval::new(0,2);
        let b = Interval::new(2,3);
        let c = Interval::new(1,2);
        let d = Interval::new(2,2);
        
        assert_eq!(a.meets(&b), true);        
        assert_eq!(a.meets(&c), false);        
        assert_eq!(a.meets(&d), false);        
        
    }
  
    #[test]
    fn test_overlaps(){
        let a = Interval::new(0,2);
        let b = Interval::new(1,2);
        let c = Interval::new(1,3);
        
        assert_eq!(a.overlaps(&b), false);        
        assert_eq!(a.overlaps(&c), true);        
        
    }  

    
    #[test]
    fn test_starts(){
        let a = Interval::new(0,2);
        let b = Interval::new(0,0);
        let c = Interval::new(0,5);
        let d = Interval::new(1,5);
        
        assert_eq!(a.starts(&c), true);        
        assert_eq!(a.starts(&b), false);        
        assert_eq!(a.starts(&d), false);        
        assert_eq!(b.starts(&c), true);        
        
    }

    #[test]
    fn test_containedByP(){
        let a = Interval::new(0,5);
        let b = Interval::new(1,3);
        let c = Interval::new(0,5);
        let d = Interval::new(1,1);
        
        assert_eq!(b.containedByP(&a), true);        
        assert_eq!(c.containedByP(&a), false);        
        assert_eq!(d.containedByP(&a), true);        
        
    }

    #[test]
    fn test_finishes(){
        let a = Interval::new(1,5);
        let b = Interval::new(1,3);
        let c = Interval::new(0,5);
        let d = Interval::new(5,5);
        
        assert_eq!(a.finishes(&b), false);        
        assert_eq!(a.finishes(&c), true);        
        assert_eq!(d.finishes(&c), true);        
        
    }   

    #[test]
    fn test_equalP(){
        let a = Interval::new(0,5);
        let b = Interval::new(1,3);
        let c = Interval::new(0,5);
        
        assert_eq!(a.equalP(&b), false);        
        assert_eq!(a.equalP(&c), true);        
        
    }

    #[test]
    fn test_finishedBy(){
        let a = Interval::new(0,5);
        let b = Interval::new(1,5);
        let c = Interval::new(5,5);
        let d = Interval::new(5,7);
        
        assert_eq!(a.finishedBy(&b), true);        
        assert_eq!(a.finishedBy(&c), true);        
        assert_eq!(a.finishedBy(&d), false);        
        
    }
    #[test]
    fn test_containsP(){
        let a = Interval::new(0,5);
        let b = Interval::new(1,3);
        let c = Interval::new(0,5);
        let d = Interval::new(3,3);
        
        assert_eq!(a.containsP(&b), true);        
        assert_eq!(a.containsP(&c), false);        
        assert_eq!(a.containsP(&d), true);        
        
    }

    #[test]
    fn test_startedBy(){
        let a = Interval::new(0,5);
        let b = Interval::new(0,3);
        let c = Interval::new(0,5);
        let d = Interval::new(0,0);
        
        assert_eq!(a.startedBy(&b), true);        
        assert_eq!(a.startedBy(&c), false);        
        assert_eq!(a.startedBy(&d), true);        
        
    }

    #[test]
    fn test_overlappedBy(){
        let a = Interval::new(1,6);
        let b = Interval::new(0,5);
        let c = Interval::new(2,7);
        let d = Interval::new(0,7);
        
        assert_eq!(a.overlappedBy(&b), true);        
        assert_eq!(a.overlappedBy(&c), false);        
        assert_eq!(a.overlappedBy(&d), false);        
        
    }

    #[test]
    fn test_metBy(){
        let a = Interval::new(5,10);
        let b = Interval::new(1,5);
        let c = Interval::new(5,5);
        let d = Interval::new(1,3);
        
        assert_eq!(a.metBy(&b), true);        
        assert_eq!(a.metBy(&c), false);        
        assert_eq!(a.metBy(&d), false);        
        assert_eq!(c.metBy(&b), false);        
        
    }

    #[test]
    fn test_afterP(){
        let a = Interval::new(5,10);
        let b = Interval::new(1,4);
        let c = Interval::new(4,4);
        let d = Interval::new(5,5);
        
        assert_eq!(a.afterP(&b), true);        
        assert_eq!(a.afterP(&c), true);        
        assert_eq!(a.afterP(&d), false);        
        assert_eq!(d.afterP(&c), true);        
        assert_eq!(d.afterP(&b), true);        
        
    }
}

#[derive(Clone, Debug)]
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



#[cfg(test)]
mod tests_intervalSet {
    use super::*;

    #[test]
    fn test_intervalset_beforeP() {
        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 32, end: 64 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 50);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 0);

        let b = IntervalSet { intervals: vec![Interval{ start: 32, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 18);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 0);
    
        let a = IntervalSet { intervals: vec![Interval{ start: 16, end: 16 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 32, end: 64 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 34);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 0);

        let b = IntervalSet { intervals: vec![Interval{ start: 32, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 2);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 0);
    }

    #[test]
    fn test_interval_meets(){
        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 16, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 1);
    }

    #[test]
    fn test_interval_overlaps(){
        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 8, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 9);
    }

    #[test]
    fn test_interval_starts(){
        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 17);

        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 0 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 1);
    }

    
    #[test]
    fn test_interval_containedByP(){
        let a = IntervalSet { intervals: vec![Interval{ start: 8, end: 16 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 9);

        let a = IntervalSet { intervals: vec![Interval{ start: 5, end: 5 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 1);
    }

    
    #[test]
    fn test_interval_finishes(){
        let a = IntervalSet { intervals: vec![Interval{ start: 16, end: 32 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 17);

        let a = IntervalSet { intervals: vec![Interval{ start: 32, end: 32 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 1);
    }

    
    #[test]
    fn test_interval_equalP(){
        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 32 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 33);

        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 0 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 0 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 1);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 1);
    }

    
    #[test]
    fn test_interval_finishedBy(){
        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 8, end: 16 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 17);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 9);

        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 32 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 32, end: 32 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 1);
    }

    
    #[test]
    fn test_interval_containsP(){
        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 5, end: 10 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 17);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 6);

        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 5, end: 5 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 17);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 1);
    }

    
    #[test]
    fn test_interval_startedBy(){
        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 8 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 17);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 9);

        let a = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 0 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 17);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 1);
    }

    
    #[test]
    fn test_interval_overlappedBy(){
        let a = IntervalSet { intervals: vec![Interval{ start: 8, end: 32 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 9);
    }

    
    #[test]
    fn test_interval_metBy(){
        let a = IntervalSet { intervals: vec![Interval{ start: 16, end: 32 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 33);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 1);
    }

    
    #[test]
    fn test_interval_afterP(){
        let a = IntervalSet { intervals: vec![Interval{ start: 20, end: 30 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 10 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 22);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 0);

        let a = IntervalSet { intervals: vec![Interval{ start: 20, end: 20 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 12);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 0);

        let a = IntervalSet { intervals: vec![Interval{ start: 20, end: 30 }] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 0 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 12);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 0);
     
        let a = IntervalSet { intervals: vec![Interval{ start: 20, end: 20 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 2);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 0);
    }

    #[test]
    fn test_interval_fstEmpty(){
        let a = IntervalSet { intervals: vec![Interval{ start: 16, end: 32 }] };
        let b = IntervalSet { intervals: vec![] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 17);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 0);
    }

    #[test]
    fn test_interval_sndEmpty(){
        let a = IntervalSet { intervals: vec![] };
        let b = IntervalSet { intervals: vec![Interval{ start: 0, end: 16 }] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 17);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 0);
    }

    #[test]
    fn test_interval_bothEmpty(){
        let a = IntervalSet { intervals: vec![] };
        let b = IntervalSet { intervals: vec![] };
        let mut aa = a.clone();
        aa.union_with(&b);
        assert_eq!(aa.len(), 0);
        let mut aa = a.clone();
        aa.intersection_with(&b);
        assert_eq!(aa.len(), 0);
    }
}