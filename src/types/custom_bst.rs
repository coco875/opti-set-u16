use std::fmt::Debug;
use super::{SetInt, SetIntConstruct};

type T = u16;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct BinarySearchTree {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl BinarySearchTree {
    fn insert_node(node: &mut Option<Box<Node<T>>>, value: T) -> bool {
        let mut current = node;
        loop {
            match current {
                None => {
                    *current = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                    return true;
                }
                Some(n) => {
                    if value < n.value {
                        current = &mut n.left;
                    } else if value > n.value {
                        current = &mut n.right;
                    } else {
                        return false;
                    }
                }
            }
        }
    }

    fn contains_node(node: &Option<Box<Node<T>>>, value: T) -> bool {
        match node {
            None => false,
            Some(n) => {
                if value < n.value {
                    Self::contains_node(&n.left, value)
                } else if value > n.value {
                    Self::contains_node(&n.right, value)
                } else {
                    true
                }
            }
        }
    }

    fn inorder(node: &Option<Box<Node<T>>>, vec: &mut Vec<T>) {
        if let Some(n) = node {
            Self::inorder(&n.left, vec);
            vec.push(n.value);
            Self::inorder(&n.right, vec);
        }
    }

    fn extract_min(node: &mut Option<Box<Node<T>>>) -> T {
        let mut current = node;

        loop {
            if current.as_ref().unwrap().left.is_none() {
                let mut boxed = current.take().unwrap();
                *current = boxed.right.take();
                return boxed.value;
            } else {
                current = &mut current.as_mut().unwrap().left;
            }
        }
    }

    fn remove_node(node: &mut Option<Box<Node<T>>>, value: T) -> bool {
        match node {
            None => false,

            Some(n) => {
                if value < n.value {
                    return Self::remove_node(&mut n.left, value);
                } else if value > n.value {
                    return Self::remove_node(&mut n.right, value);
                }

                match (n.left.take(), n.right.take()) {

                    (None, None) => {
                        *node = None;
                    }

                    (Some(left), None) => {
                        *node = Some(left);
                    }
                    (None, Some(right)) => {
                        *node = Some(right);
                    }

                    (Some(left), Some(right)) => {
                        let mut right_opt = Some(right);

                        let successor = Self::extract_min(&mut right_opt);

                        *node = Some(Box::new(Node {
                            value: successor,
                            left: Some(left),
                            right: right_opt,
                        }));
                    }
                }

                true
            }
        }
    }
}

impl SetIntConstruct for BinarySearchTree {

    fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl SetInt for BinarySearchTree {

    fn iter(&self) -> Box<dyn Iterator<Item = u16>> {
        let mut vec = Vec::new();
        Self::inorder(&self.root, &mut vec);
        Box::new(vec.into_iter())
    }

    fn insert(&mut self, value: T) -> () {
        let inserted = Self::insert_node(&mut self.root, value);
        if inserted {
            self.size += 1;
        }
    }

    fn remove(&mut self, value: T) -> bool {
        let removed = Self::remove_node(&mut self.root, value);
        if removed {
            self.size -= 1;
        }
    removed
}

    fn contains(&self, value: T) -> bool {
        Self::contains_node(&self.root, value)
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }

    fn union_with(&mut self, other: &Self) -> () {
        for v in other.iter(){
            if !self.contains(v) {
                self.insert(v);
            }
        }
    }

    fn intersection_with(&mut self, other: &Self) -> () {
        for v in self.iter().collect::<Vec<u16>>() {
            if !other.contains(v) {
                self.remove(v);
            }
        }
    }

    fn difference_with(&mut self, other: &Self) -> () {
        for v in self.iter().collect::<Vec<u16>>() {
            if other.contains(v) {
                self.remove(v);
            }
        }
    }

    fn symmetric_difference_with(&mut self, other: &Self) -> () {
        for v in self.iter().collect::<Vec<u16>>() {
            if other.contains(v) {
                self.remove(v);
            }
        }

        for v in other.iter() {
            if !self.contains(v) {
                self.insert(v);
            }
        }
    }
}
