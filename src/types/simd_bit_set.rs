use super::{SetInt, SetIntConstruct};
use std::simd::{u64x8, Simd};

pub struct SimdBitSet {
    bits: [u64x8; 128], // 128 * 8 * 64 = 65536 bits
}

impl SetIntConstruct for SimdBitSet {
    fn new() -> Self {
        Self {
            bits: [Simd::splat(0); 128],
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl SetInt for SimdBitSet {
    fn clear(&mut self) {
        self.bits.fill(Simd::splat(0));
    }

    fn insert(&mut self, n: u16) {
        let vec_idx = (n as usize) >> 9;
        let u64_idx = ((n as usize) >> 6) & 7;
        let bit_idx = n & 63;
        self.bits[vec_idx].as_mut_array()[u64_idx] |= 1 << bit_idx;
    }

    fn remove(&mut self, n: u16) -> bool {
        let vec_idx = (n as usize) >> 9;
        let u64_idx = ((n as usize) >> 6) & 7;
        let bit_idx = n & 63;
        let mask = 1 << bit_idx;
        let word = &mut self.bits[vec_idx].as_mut_array()[u64_idx];
        let old = *word;
        *word &= !mask;
        (old & mask) != 0
    }

    fn contains(&self, n: u16) -> bool {
        let vec_idx = (n as usize) >> 9;
        let u64_idx = ((n as usize) >> 6) & 7;
        let bit_idx = n & 63;
        (self.bits[vec_idx].as_array()[u64_idx] & (1 << bit_idx)) != 0
    }

    fn len(&self) -> usize {
        let mut count = 0;
        for v in &self.bits {
            for &w in v.as_array() {
                count += w.count_ones() as usize;
            }
        }
        count
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        let mut elems = Vec::new();
        for (v_idx, v) in self.bits.iter().enumerate() {
            for (w_idx, &word) in v.as_array().iter().enumerate() {
                let mut w = word;
                while w != 0 {
                    let tz = w.trailing_zeros();
                    elems.push(((v_idx * 512) + (w_idx * 64) + tz as usize) as u16);
                    w &= w - 1;
                }
            }
        }
        Box::new(elems.into_iter())
    }

    fn union_with(&mut self, other: &Self) {
        for i in 0..128 {
            self.bits[i] |= other.bits[i];
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        for i in 0..128 {
            self.bits[i] &= other.bits[i];
        }
    }

    fn difference_with(&mut self, other: &Self) {
        for i in 0..128 {
            self.bits[i] &= !other.bits[i];
        }
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        for i in 0..128 {
            self.bits[i] ^= other.bits[i];
        }
    }
}
