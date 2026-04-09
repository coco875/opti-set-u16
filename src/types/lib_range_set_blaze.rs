use super::{SetInt, SetIntConstruct};
use range_set_blaze::RangeSetBlaze;

pub struct LibRangeSetBlaze {
    set: RangeSetBlaze<u16>,
}

impl SetIntConstruct for LibRangeSetBlaze {
    fn new() -> Self {
        Self {
            set: RangeSetBlaze::new(),
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl SetInt for LibRangeSetBlaze {
    fn clear(&mut self) {
        self.set.clear();
    }

    fn insert(&mut self, n: u16) {
        self.set.insert(n);
    }

    fn remove(&mut self, n: u16) -> bool {
        self.set.remove(n)
    }

    fn contains(&self, n: u16) -> bool {
        self.set.contains(n)
    }

    fn len(&self) -> usize {
        self.set.len() as usize
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.set.iter())
    }

    fn union_with(&mut self, other: &Self) {
        self.set = &self.set | &other.set;
    }

    fn intersection_with(&mut self, other: &Self) {
        self.set = &self.set & &other.set;
    }

    fn difference_with(&mut self, other: &Self) {
        self.set = &self.set - &other.set;
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        self.set = &self.set ^ &other.set;
    }
}
