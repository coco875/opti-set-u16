use super::{SetInt, SetIntConstruct};

#[derive(Default, Clone)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct BitTreeSet {
    root: Option<Box<Node>>,
    len: usize,
}

impl SetIntConstruct for BitTreeSet {
    fn new() -> Self {
        Self {
            root: None,
            len: 0,
        }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl BitTreeSet {
    fn remove_rec(node: &mut Option<Box<Node>>, n: u16, depth: i8) -> bool {
        if node.is_none() {
            return false;
        }
        if depth < 0 {
            *node = None;
            return true;
        }
        let bit = (n >> depth) & 1;
        let n_node = node.as_mut().unwrap();
        let removed = if bit == 0 {
            Self::remove_rec(&mut n_node.left, n, depth - 1)
        } else {
            Self::remove_rec(&mut n_node.right, n, depth - 1)
        };

        if removed && n_node.left.is_none() && n_node.right.is_none() {
            *node = None;
        }
        removed
    }
}

impl SetInt for BitTreeSet {
    fn clear(&mut self) {
        self.root = None;
        self.len = 0;
    }

    fn insert(&mut self, n: u16) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::default()));
        }
        let mut curr = self.root.as_mut().unwrap();
        for i in (0..16).rev() {
            let bit = (n >> i) & 1;
            let next = if bit == 0 {
                &mut curr.left
            } else {
                &mut curr.right
            };
            if next.is_none() {
                *next = Some(Box::new(Node::default()));
                if i == 0 {
                    self.len += 1;
                }
            }
            curr = next.as_mut().unwrap();
        }
    }

    fn remove(&mut self, n: u16) -> bool {
        let removed = Self::remove_rec(&mut self.root, n, 15);
        if removed {
            self.len -= 1;
        }
        removed
    }

    fn contains(&self, n: u16) -> bool {
        let mut curr = &self.root;
        for i in (0..16).rev() {
            if let Some(node) = curr {
                let bit = (n >> i) & 1;
                curr = if bit == 0 {
                    &node.left
                } else {
                    &node.right
                };
            } else {
                return false;
            }
        }
        curr.is_some()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        let mut elems = Vec::with_capacity(self.len);
        fn dfs(node: &Option<Box<Node>>, val: u16, depth: i8, elems: &mut Vec<u16>) {
            if let Some(n) = node {
                if depth < 0 {
                    elems.push(val);
                } else {
                    dfs(&n.left, val, depth - 1, elems);
                    dfs(&n.right, val | (1 << depth), depth - 1, elems);
                }
            }
        }
        dfs(&self.root, 0, 15, &mut elems);
        Box::new(elems.into_iter())
    }

    fn union_with(&mut self, other: &Self) {
        for item in other.iter() {
            self.insert(item);
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        let mut to_remove = Vec::new();
        for item in self.iter() {
            if !other.contains(item) {
                to_remove.push(item);
            }
        }
        for item in to_remove {
            self.remove(item);
        }
    }

    fn difference_with(&mut self, other: &Self) {
        for item in other.iter() {
            self.remove(item);
        }
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        for item in other.iter() {
            if self.contains(item) {
                self.remove(item);
            } else {
                self.insert(item);
            }
        }
    }
}
