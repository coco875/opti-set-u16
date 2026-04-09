use super::{SetInt, SetIntConstruct};

pub struct CustomBitSet {
    bits: [u64; 1024],
}

impl SetIntConstruct for CustomBitSet {
    fn new() -> Self {
        Self { bits: [0; 1024] }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl SetInt for CustomBitSet {
    fn clear(&mut self) {
        self.bits.fill(0);
    }

    fn insert(&mut self, n: u16) {
        self.bits[(n as usize) >> 6] |= 1 << (n & 63);
    }

    fn remove(&mut self, n: u16) -> bool {
        let idx = (n as usize) >> 6;
        let mask = 1 << (n & 63);
        let old = self.bits[idx];
        self.bits[idx] &= !mask;
        (old & mask) != 0
    }

    fn contains(&self, n: u16) -> bool {
        (self.bits[(n as usize) >> 6] & (1 << (n & 63))) != 0
    }

    fn len(&self) -> usize {
        self.bits.iter().map(|&w| w.count_ones() as usize).sum()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        let mut elems = Vec::new();
        for (i, &word) in self.bits.iter().enumerate() {
            let mut w = word;
            while w != 0 {
                let tz = w.trailing_zeros();
                elems.push((i * 64 + tz as usize) as u16);
                w &= w - 1; // clear lowest set bit
            }
        }
        Box::new(elems.into_iter())
    }

    fn union_with(&mut self, other: &Self) {
        for i in 0..1024 {
            self.bits[i] |= other.bits[i];
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        for i in 0..1024 {
            self.bits[i] &= other.bits[i];
        }
    }

    fn difference_with(&mut self, other: &Self) {
        for i in 0..1024 {
            self.bits[i] &= !other.bits[i];
        }
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        for i in 0..1024 {
            self.bits[i] ^= other.bits[i];
        }
    }
}
