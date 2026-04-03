use super::{SetInt, SetIntConstruct};

pub struct LibRoaring {
    bitmap: roaring::RoaringBitmap,
}

impl SetIntConstruct for LibRoaring {
    fn new() -> Self {
        Self {
            bitmap: roaring::RoaringBitmap::new(),
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl SetInt for LibRoaring {
    fn clear(&mut self) {
        self.bitmap.clear();
    }

    fn insert(&mut self, n: u16) {
        self.bitmap.insert(n as u32);
    }

    fn remove(&mut self, n: u16) -> bool {
        self.bitmap.remove(n as u32)
    }

    fn contains(&self, n: u16) -> bool {
        self.bitmap.contains(n as u32)
    }

    fn len(&self) -> usize {
        self.bitmap.len() as usize
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.bitmap.iter().map(|v| v as u16))
    }

    fn union_with(&mut self, other: &Self) {
        self.bitmap |= &other.bitmap;
    }

    fn intersection_with(&mut self, other: &Self) {
        self.bitmap &= &other.bitmap;
    }

    fn difference_with(&mut self, other: &Self) {
        self.bitmap -= &other.bitmap;
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        self.bitmap ^= &other.bitmap;
    }
}
