use super::{SetInt, SetIntConstruct};
use std::collections::HashSet;
use std::hash::{BuildHasherDefault, Hasher};

#[derive(Default)]
pub struct NoOpHasher {
    state: u64,
}

impl Hasher for NoOpHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.state
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        let mut buf = [0u8; 8];
        let len = bytes.len().min(8);
        buf[..len].copy_from_slice(&bytes[..len]);
        self.state = u64::from_ne_bytes(buf);
    }

    #[inline(always)]
    fn write_u16(&mut self, i: u16) {
        self.state = i as u64;
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        self.state = i as u64;
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        self.state = i;
    }

    #[inline(always)]
    fn write_usize(&mut self, i: usize) {
        self.state = i as u64;
    }
}

type NoOpBuildHasher = BuildHasherDefault<NoOpHasher>;

pub struct StdHashSetNoHasher {
    set: HashSet<u16, NoOpBuildHasher>,
}

impl SetIntConstruct for StdHashSetNoHasher {
    fn new() -> Self {
        Self {
            set: HashSet::default(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            set: HashSet::with_capacity_and_hasher(capacity, NoOpBuildHasher::default()),
        }
    }
}

impl SetInt for StdHashSetNoHasher {
    fn clear(&mut self) {
        self.set.clear();
    }

    fn insert(&mut self, n: u16) {
        self.set.insert(n);
    }

    fn remove(&mut self, n: u16) -> bool {
        self.set.remove(&n)
    }

    fn contains(&self, n: u16) -> bool {
        self.set.contains(&n)
    }

    fn len(&self) -> usize {
        self.set.len()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.set.iter().copied())
    }

    fn union_with(&mut self, other: &Self) {
        for &item in &other.set {
            self.insert(item);
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        self.set.retain(|item| other.contains(*item));
    }

    fn difference_with(&mut self, other: &Self) {
        for &item in &other.set {
            self.remove(item);
        }
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        for &item in &other.set {
            if self.contains(item) {
                self.remove(item);
            } else {
                self.insert(item);
            }
        }
    }
}
