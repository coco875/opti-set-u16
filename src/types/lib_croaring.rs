use super::{SetInt, SetIntConstruct};

pub struct LibCRoaring {
    bitmap: croaring::Bitmap,
}

impl SetIntConstruct for LibCRoaring {
    fn new() -> Self {
        Self {
            bitmap: croaring::Bitmap::new(),
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl SetInt for LibCRoaring {
    fn clear(&mut self) {
        self.bitmap.clear();
    }

    fn insert(&mut self, n: u16) {
        self.bitmap.add(n as u32);
    }

    fn remove(&mut self, n: u16) -> bool {
        let res = self.bitmap.contains(n as u32);
        self.bitmap.remove(n as u32);
        res
    }

    fn contains(&self, n: u16) -> bool {
        self.bitmap.contains(n as u32)
    }

    fn len(&self) -> usize {
        self.bitmap.cardinality() as usize
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.bitmap.iter().map(|v| v as u16))
    }

    fn union_with(&mut self, other: &Self) {
        self.bitmap.or_inplace(&other.bitmap);
    }

    fn intersection_with(&mut self, other: &Self) {
        self.bitmap.and_inplace(&other.bitmap);
    }

    fn difference_with(&mut self, other: &Self) {
        self.bitmap.andnot_inplace(&other.bitmap);
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        self.bitmap.xor_inplace(&other.bitmap);
    }
}
