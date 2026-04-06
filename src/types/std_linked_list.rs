use super::{SetInt, SetIntConstruct};
use std::collections::LinkedList;

pub struct StdLinkedList {
    elements: LinkedList<u16>,
}

impl SetIntConstruct for StdLinkedList {
    fn new() -> Self {
        Self {
            elements: LinkedList::new(),
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self {
            elements: LinkedList::new(),
        }
    }
}

impl SetInt for StdLinkedList {
    fn clear(&mut self) {
        self.elements.clear();
    }

    fn insert(&mut self, n: u16) {
        if self.contains(n) {
            return;
        }
        let mut pushed = false;
        let mut temp = LinkedList::new();
        while let Some(val) = self.elements.pop_front() {
            if !pushed && n < val {
                temp.push_back(n);
                pushed = true;
            }
            temp.push_back(val);
        }
        if !pushed {
            temp.push_back(n);
        }
        self.elements = temp;
    }

    fn remove(&mut self, n: u16) -> bool {
        let mut node = self.elements.pop_front();
        let mut found = false;
        let mut temp = LinkedList::new();
        while let Some(val) = node {
            if val == n && !found {
                found = true;
            } else {
                temp.push_back(val);
            }
            node = self.elements.pop_front();
        }
        self.elements = temp;
        found
    }

    fn contains(&self, n: u16) -> bool {
        self.elements.iter().any(|&x| x == n)
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        Box::new(self.elements.iter().copied())
    }

    fn union_with(&mut self, other: &Self) {
        let self_vec: Vec<u16> = self.elements.iter().copied().collect();
        let other_vec: Vec<u16> = other.elements.iter().copied().collect();
        let mut result = LinkedList::new();

        let mut i = 0;
        let mut j = 0;

        while i < self_vec.len() && j < other_vec.len() {
            let a = self_vec[i];
            let b = other_vec[j];

            if a < b {
                result.push_back(a);
                i += 1;
            } else if b < a {
                result.push_back(b);
                j += 1;
            } else {
                result.push_back(a);
                i += 1;
                j += 1;
            }
        }

        while i < self_vec.len() {
            result.push_back(self_vec[i]);
            i += 1;
        }

        while j < other_vec.len() {
            result.push_back(other_vec[j]);
            j += 1;
        }

        self.elements = result;
    }

    fn intersection_with(&mut self, other: &Self) {
        let self_vec: Vec<u16> = self.elements.iter().copied().collect();
        let other_vec: Vec<u16> = other.elements.iter().copied().collect();
        let mut result = LinkedList::new();

        let mut i = 0;
        let mut j = 0;

        while i < self_vec.len() && j < other_vec.len() {
            let a = self_vec[i];
            let b = other_vec[j];

            if a < b {
                i += 1;
            } else if b < a {
                j += 1;
            } else {
                result.push_back(a);
                i += 1;
                j += 1;
            }
        }

        self.elements = result;
    }

    fn difference_with(&mut self, other: &Self) {
        let self_vec: Vec<u16> = self.elements.iter().copied().collect();
        let other_vec: Vec<u16> = other.elements.iter().copied().collect();
        let mut result = LinkedList::new();

        let mut j = 0;

        for &a in &self_vec {
            while j < other_vec.len() && other_vec[j] < a {
                j += 1;
            }
            if j >= other_vec.len() || other_vec[j] > a {
                result.push_back(a);
            }
        }

        self.elements = result;
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        let self_vec: Vec<u16> = self.elements.iter().copied().collect();
        let other_vec: Vec<u16> = other.elements.iter().copied().collect();
        let mut result = LinkedList::new();

        let mut i = 0;
        let mut j = 0;

        while i < self_vec.len() && j < other_vec.len() {
            let a = self_vec[i];
            let b = other_vec[j];

            if a < b {
                result.push_back(a);
                i += 1;
            } else if b < a {
                result.push_back(b);
                j += 1;
            } else {
                i += 1;
                j += 1;
            }
        }

        while i < self_vec.len() {
            result.push_back(self_vec[i]);
            i += 1;
        }

        while j < other_vec.len() {
            result.push_back(other_vec[j]);
            j += 1;
        }

        self.elements = result;
    }
}
