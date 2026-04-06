use super::{SetInt, SetIntConstruct};

pub struct StdVecDicotomie {
    elements: Vec<u16>,
}

impl SetIntConstruct for StdVecDicotomie {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Vec::with_capacity(capacity),
        }
    }
}

impl SetInt for StdVecDicotomie {
    fn clear(&mut self) {
        self.elements.clear();
    }

    fn insert(&mut self, n: u16) {
        match self.elements.binary_search(&n) {
            Ok(_) => {}
            Err(pos) => self.elements.insert(pos, n),
        }
    }

    fn remove(&mut self, n: u16) -> bool {
        match self.elements.binary_search(&n) {
            Ok(pos) => {
                self.elements.remove(pos);
                true
            }
            Err(_) => false,
        }
    }

    fn contains(&self, n: u16) -> bool {
        self.elements.binary_search(&n).is_ok()
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.elements.iter().copied())
    }

    fn union_with(&mut self, other: &Self) {
        let mut result = Vec::with_capacity(self.elements.len() + other.elements.len());
        let mut i = 0;
        let mut j = 0;

        while i < self.elements.len() && j < other.elements.len() {
            let a = self.elements[i];
            let b = other.elements[j];

            if a < b {
                result.push(a);
                i += 1;
            } else if b < a {
                result.push(b);
                j += 1;
            } else {
                result.push(a);
                i += 1;
                j += 1;
            }
        }

        while i < self.elements.len() {
            result.push(self.elements[i]);
            i += 1;
        }

        while j < other.elements.len() {
            result.push(other.elements[j]);
            j += 1;
        }

        self.elements = result;
    }

    fn intersection_with(&mut self, other: &Self) {
        let mut result = Vec::with_capacity(self.elements.len().min(other.elements.len()));
        let mut i = 0;
        let mut j = 0;

        while i < self.elements.len() && j < other.elements.len() {
            let a = self.elements[i];
            let b = other.elements[j];

            if a < b {
                i += 1;
            } else if b < a {
                j += 1;
            } else {
                result.push(a);
                i += 1;
                j += 1;
            }
        }

        self.elements = result;
    }

    fn difference_with(&mut self, other: &Self) {
        let mut result = Vec::with_capacity(self.elements.len());
        let mut j = 0;

        for &a in &self.elements {
            while j < other.elements.len() && other.elements[j] < a {
                j += 1;
            }
            if j >= other.elements.len() || other.elements[j] > a {
                result.push(a);
            }
        }

        self.elements = result;
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        let mut result = Vec::with_capacity(self.elements.len() + other.elements.len());
        let mut i = 0;
        let mut j = 0;

        while i < self.elements.len() && j < other.elements.len() {
            let a = self.elements[i];
            let b = other.elements[j];

            if a < b {
                result.push(a);
                i += 1;
            } else if b < a {
                result.push(b);
                j += 1;
            } else {
                i += 1;
                j += 1;
            }
        }

        while i < self.elements.len() {
            result.push(self.elements[i]);
            i += 1;
        }

        while j < other.elements.len() {
            result.push(other.elements[j]);
            j += 1;
        }

        self.elements = result;
    }
}
