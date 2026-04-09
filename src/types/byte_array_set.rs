use super::{SetInt, SetIntConstruct};

pub struct ByteArraySet {
    bytes: [u8; 65536],
}

impl SetIntConstruct for ByteArraySet {
    fn new() -> Self {
        Self { bytes: [0; 65536] }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl SetInt for ByteArraySet {
    fn clear(&mut self) {
        self.bytes.fill(0);
    }

    fn insert(&mut self, n: u16) {
        self.bytes[n as usize] = 1;
    }

    fn remove(&mut self, n: u16) -> bool {
        let old = self.bytes[n as usize];
        self.bytes[n as usize] = 0;
        old != 0
    }

    fn contains(&self, n: u16) -> bool {
        self.bytes[n as usize] != 0
    }

    fn len(&self) -> usize {
        self.bytes.iter().filter(|&&b| b != 0).count()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        let elems: Vec<u16> = self
            .bytes
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if b != 0 { Some(i as u16) } else { None })
            .collect();
        Box::new(elems.into_iter())
    }

    fn union_with(&mut self, other: &Self) {
        for i in 0..65536 {
            self.bytes[i] |= other.bytes[i];
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        for i in 0..65536 {
            self.bytes[i] &= other.bytes[i];
        }
    }

    fn difference_with(&mut self, other: &Self) {
        for i in 0..65536 {
            self.bytes[i] &= !other.bytes[i];
        }
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        for i in 0..65536 {
            self.bytes[i] ^= other.bytes[i];
        }
    }
}
