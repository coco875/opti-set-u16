use core::convert::Into;
use std::default::Default;

pub trait SetInt {
    fn clear(&mut self);
    fn insert(&mut self, n: u16);
    fn remove(&mut self, n: u16) -> bool;
    fn contains(&self, n: u16) -> bool;
}

pub trait SetIntConstruct: SetInt {
    fn new() -> Self;
}

pub struct BitSet {
    bit_set: bit_set::BitSet<u16>,
}

impl SetIntConstruct for BitSet {
    fn new() -> Self {
        Self {
            bit_set: bit_set::BitSet::<u16>::default(),
        }
    }
}

impl SetInt for BitSet {
    fn clear(&mut self) {
        self.bit_set.make_empty();
    }

    fn insert(&mut self, n: u16) {
        self.bit_set.insert(n.into());
    }

    fn remove(&mut self, n: u16) -> bool {
        self.bit_set.remove(n.into())
    }

    fn contains(&self, n: u16) -> bool {
        self.bit_set.contains(n.into())
    }
}

pub struct StdHashSet {
    hash_set: std::collections::HashSet<u16>,
}

impl SetIntConstruct for StdHashSet {
    fn new() -> Self {
        Self {
            hash_set: std::collections::HashSet::new(),
        }
    }
}

impl SetInt for StdHashSet {
    fn clear(&mut self) {
        self.hash_set.clear();
    }

    fn insert(&mut self, n: u16) {
        self.hash_set.insert(n);
    }

    fn remove(&mut self, n: u16) -> bool {
        self.hash_set.remove(&n)
    }

    fn contains(&self, n: u16) -> bool {
        self.hash_set.contains(&n)
    }
}
