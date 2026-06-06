use super::{SetInt, SetIntConstruct};

/// Idée générale
/// —————————————
/// - Liste triée d'éléments
/// - Les éléments pairs représentent le début d'un intervalle
/// - Les éléments impairs représentent la fin d'un intervalle
/// - "∈" utilise une recherche dichotomique pour trouver la borne inférieur de l'intervalle le contenant
#[derive(Clone, PartialEq, Debug)]
pub struct FlatIntervalSet {
    inner: Vec<u16>,
}

impl FlatIntervalSet {
    pub fn new() -> Self {
        FlatIntervalSet { inner: Vec::new() }
    }

    pub fn singleton(e: u16) -> Self {
        FlatIntervalSet {
            inner: vec![e, e + 1],
        }
    }
}

impl SetIntConstruct for FlatIntervalSet {
    fn new() -> Self {
        FlatIntervalSet::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        FlatIntervalSet {
            inner: Vec::with_capacity(capacity),
        }
    }
}

impl SetInt for FlatIntervalSet {
    fn clear(&mut self) {
        self.inner.clear();
    }

    fn contains(&self, elem: u16) -> bool {
        //* Stratégie
        //* —————————
        //* Recherche l'intervalle [a, b) tel que elem ∈ [a, b)
        //* Si l'index de a est pair :
        //*     c'est un début d'intervalle et elem n'est pas contenu
        //* Sinon :
        //*     c'est une fin d'intervalle et elem est contenu

        match self.inner.binary_search(&elem) {
            Ok(i) => i % 2 == 0,
            Err(i) => i % 2 == 1,
        }
    }

    fn insert(&mut self, n: u16) {
        //* Stratégie
        //* —————————
        //* Si n précède le début d'un intervalle, l'ajoute à cet intervalle
        //* Si n suit la fin d'un intervalle, l'ajoute à cet intervalle
        //* Sinon, crée un nouvel intervalle
        
        let idx = match self.inner.binary_search(&n) {
            Ok(_) => return,
            Err(i) => i,
        };

        let left_ok = idx > 0 && self.inner[idx - 1] == n;
        let right_ok = idx < self.inner.len() && self.inner[idx] == n + 1;

        if left_ok && right_ok {
            let next_end = self.inner[idx + 1];
            self.inner.remove(idx);
            self.inner.remove(idx - 1);
            self.inner[idx - 1] = next_end;
        } else if left_ok {
            self.inner[idx - 1] = n + 1;
        } else if right_ok {
            self.inner[idx] = n;
        } else {
            self.inner.insert(idx, n);
            self.inner.insert(idx + 1, n + 1);
        }
    }

    fn remove(&mut self, n: u16) -> bool {
        //* Stratégie
        //* —————————
        //* Si n est au début d'un intervalle, le retire de cet intervalle
        //* Si n est à la fin d'un intervalle, le retire de cet intervalle
        //* Sinon, coupe l'intervalle en deux

        let idx = match self.inner.binary_search(&n) {
            Ok(i) => i,
            Err(_) => return false,
        };

        if idx % 2 == 1 {
            return false;
        }

        let start = self.inner[idx];
        let end = self.inner[idx + 1];

        if n == start {
            if start + 1 == end {
                self.inner.remove(idx);
                self.inner.remove(idx);
            } else {
                self.inner[idx] = start + 1;
            }
        } else if n == end - 1 {
            if start + 1 == end {
                self.inner.remove(idx);
                self.inner.remove(idx);
            } else {
                self.inner[idx + 1] = end - 1;
            }
        } else {
            self.inner[idx + 1] = n;
            self.inner.insert(idx + 2, n + 1);
            self.inner.insert(idx + 3, end);
        }
        true
    }

    fn len(&self) -> usize {
        //* Stratégie
        //* —————————
        //* Chunk par deux les éléments de la liste,
        //* ce qui donne une liste de pair (donc d'intervalle)
        //* sur laquelle on map λi.len(i)

        self.inner
            .chunks(2)
            .map(|chunk| (chunk[1] - chunk[0]) as usize)
            .sum()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        //* Stratégie
        //* —————————
        //* Chunk par deux les éléments de la liste,
        //* ce qui donne une liste de pair (donc d'intervalle)
        //* sur laquelle on ∪λi.∪(k=1, n)k

        Box::new(
            self.inner
                .chunks(2)
                .flat_map(|chunk| chunk[0]..chunk[1]),
        )
    }

    fn union_with(&mut self, other: &Self) {
        //* Stratégie
        //* —————————
        //* Crée un liste de bornes (e, δ) avec δ ∈ {-1, +1}
        //*   - e indique l'endroit de la borne
        //*   - δ indique si on entre (+1) ou sort (-1) d'un intervalle
        //* Trie la liste de bornes, puis crée les intervalle
        //*   - une intersection débute quand le niveau passe de 0 à >0 et termine quand il passe de 0 à >0

        let mut bounds: Vec<(u16, i32)> = Vec::new(); // (e, δ)
        for chunk in self.inner.chunks(2) {
            bounds.push((chunk[0], 1));
            bounds.push((chunk[1], -1));
        }
        for chunk in other.inner.chunks(2) {
            bounds.push((chunk[0], 1));
            bounds.push((chunk[1], -1));
        }
        bounds.sort_by_key(|e| e.0);

        let mut temp = Vec::new();
        let mut level = 0i32;
        for (e, delta) in bounds {
            let prev = level;
            level += delta;
            if prev == 0 && level > 0 {
                temp.push(e);
            } else if prev > 0 && level == 0 {
                temp.push(e);
            }
        }
        self.inner = temp;
    }

    fn intersection_with(&mut self, other: &Self) {
        //* Stratégie
        //* —————————
        //* Crée un liste de bornes (e, δ) avec δ ∈ {-1, +1}
        //*   - e indique l'endroit de la borne
        //*   - δ indique si on entre (+1) ou sort (-1) d'un intervalle
        //* Trie la liste de bornes, puis crée les intervalle
        //*   - une intersection débute quand le niveau passe de <2 à 2 et termine quand il passe de 2 à <2
        
        let mut bounds: Vec<(u16, i32)> = Vec::new();  // (e, δ)
        for chunk in self.inner.chunks(2) {
            bounds.push((chunk[0], 1));
            bounds.push((chunk[1], -1));
        }
        for chunk in other.inner.chunks(2) {
            bounds.push((chunk[0], 1));
            bounds.push((chunk[1], -1));
        }
        bounds.sort_by_key(|e| e.0);

        let mut temp = Vec::new();
        let mut level = 0i32;
        for (e, delta) in bounds {
            let prev = level;
            level += delta;
            if prev < 2 && level == 2 {
                temp.push(e);
            } else if prev == 2 && level < 2 {
                temp.push(e);
            }
        }
        self.inner = temp;
    }

    fn difference_with(&mut self, other: &Self) {
        //* Stratégie
        //* —————————
        //* Crée une liste de bornes (e, δ_self, δ_other)
        //*   - e indique l'endroit de la borne
        //*   - δ_self vaut +1 quand on entre dans un intervalle de self -1 quand on en sort, 0 sinon
        //*   - δ_other vaut +1 quand on entre dans un intervalle de other -1 quand on en sort, 0 sinon
        //* Trie la liste de bornes, puis maintient deux niveaux de couverture :
        //*   - self_cov : nombre d'intervalles de self couvrant la position courante
        //*   - other_cov : nombre d'intervalles de other couvrant la position courante
        //* La différence A \ B est active lorsque
        //*   - self_cov > 0 et other_cov = 0

        let mut bounds: Vec<(u16, i32, i32)> = Vec::new(); // (e, δ_self, δ_other)
        for chunk in self.inner.chunks(2) {
            bounds.push((chunk[0], 1, 0));
            bounds.push((chunk[1], -1, 0));
        }
        for chunk in other.inner.chunks(2) {
            bounds.push((chunk[0], 0, 1));
            bounds.push((chunk[1], 0, -1));
        }
        bounds.sort_by_key(|e| e.0);

        let mut temp = Vec::new();
        let mut self_cov = 0i32;
        let mut other_cov = 0i32;
        for (e, ds, do_) in bounds {
            let was_active = self_cov > 0 && other_cov == 0;
            self_cov += ds;
            other_cov += do_;
            let is_active = self_cov > 0 && other_cov == 0;
            if !was_active && is_active {
                temp.push(e);
            } else if was_active && !is_active {
                temp.push(e);
            }
        }
        self.inner = temp;
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        //* Stratégie
        //* —————————
        //* Crée une liste de bornes (e, δ_self, δ_other)
        //*   - e indique l'endroit de la borne
        //*   - δ_self vaut +1 quand on entre dans un intervalle de self -1 quand on en sort, 0 sinon
        //*   - δ_other vaut +1 quand on entre dans un intervalle de other -1 quand on en sort, 0 sinon
        //* Trie la liste de bornes, puis maintient deux niveaux de couverture :
        //*   - self_cov : nombre d'intervalles de self couvrant la position courante
        //*   - other_cov : nombre d'intervalles de other couvrant la position courante
        //* La différence symétrique A Δ B est active lorsque
        //*   - (self_cov > 0) XOR (other_cov > 0) est vrai

        let mut bounds: Vec<(u16, i32, i32)> = Vec::new(); // (e, δ_self, δ_other)
        for chunk in self.inner.chunks(2) {
            bounds.push((chunk[0], 1, 0));
            bounds.push((chunk[1], -1, 0));
        }
        for chunk in other.inner.chunks(2) {
            bounds.push((chunk[0], 0, 1));
            bounds.push((chunk[1], 0, -1));
        }
        bounds.sort_by_key(|e| e.0);

        let mut temp = Vec::new();
        let mut self_cov = 0i32;
        let mut other_cov = 0i32;
        for (e, ds, do_) in bounds {
            let was_self = self_cov > 0;
            let was_other = other_cov > 0;
            self_cov += ds;
            other_cov += do_;
            let is_self = self_cov > 0;
            let is_other = other_cov > 0;
            let was_active = was_self ^ was_other;
            let is_active = is_self ^ is_other;
            if !was_active && is_active {
                temp.push(e);
            } else if was_active && !is_active {
                temp.push(e);
            }
        }
        self.inner = temp;
    }
}