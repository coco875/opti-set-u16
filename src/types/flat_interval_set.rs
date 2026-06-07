use super::{SetInt, SetIntConstruct};

/// Idée générale
/// —————————————
/// - Liste triée d'éléments
/// - Les éléments pairs représentent le début d'un interval
/// - Les éléments impairs représentent la fin d'un interval
/// - "∈" utilise une recherche dichotomique pour trouver la borne inférieur de l'interval le contenant
#[derive(Clone, PartialEq, Debug)]
pub struct FlatIntervalSet {
    inner: Vec<u32>,
}

impl SetIntConstruct for FlatIntervalSet {
    fn new() -> Self {
        FlatIntervalSet { inner: Vec::new() }
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
        //* Recherche l'interval [a, b) tel que elem in [a, b)
        //* Si l'index de a est pair :
        //*     c'est un début d'interval et elem n'est pas contenu
        //* Sinon :
        //*     c'est une fin d'interval et elem est contenu

        let elem = elem as u32;

        match self.inner.binary_search(&elem) {
            Ok(i) => i % 2 == 0,
            Err(i) => i % 2 == 1,
        }
    }

    fn insert(&mut self, n: u16) {
        //* Stratégie
        //* —————————
        //* Recherche si n est adjacent à des intervalles existants :
        //*   - Si n+1 == début d'un interval ET n-1 == fin d'un interval : fusionner les deux
        //*   - Si n+1 == début d'un interval : décaler ce début de -1
        //*   - Si n == fin d'un interval : décaler cette fin de +1
        //*   - Sinon : insérer un nouvel interval [n, n+1)

        if self.contains(n) {
            return;
        }

        let n = n as u32;

        let next_is_start = self.inner.binary_search(&(n + 1))
            .ok()
            .filter(|&i| i % 2 == 0);

        let prev_is_end = self.inner.binary_search(&n)
            .ok()
            .filter(|&i| i % 2 == 1);

        if let (Some(ei), Some(si)) = (prev_is_end, next_is_start) {
            let new_end = self.inner[si + 1];
            self.inner.remove(si + 1);
            self.inner.remove(si);
            self.inner[ei] = new_end;
        } else if let Some(si) = next_is_start {
            self.inner[si] = n;
        } else if let Some(ei) = prev_is_end {
            self.inner[ei] = n + 1;
        } else {
            let pos = self.inner.partition_point(|&x| x < n);
            self.inner.insert(pos, n + 1);
            self.inner.insert(pos, n);
        }
    }

    fn remove(&mut self, n: u16) -> bool {
        //* Stratégie
        //* —————————
        //* Recherche l'interval [a, b) contenant n :
        //*   - Si n == a et n+1 == b (singleton) : supprimer l'interval entier
        //*   - Si n == a : décaler le début de +1
        //*   - Si n+1 == b : décaler la fin de -1
        //*   - Sinon : scinder l'interval en [a, n) et [n+1, b)

        if !self.contains(n) {
            return false;
        }

        let n = n as u32;

        let start_i = match self.inner.binary_search(&n) {
            Ok(i) if i % 2 == 0 => i,
            Ok(i) => i - 1,
            Err(i) => i - 1,
        };

        let end_i = start_i + 1;
        let a     = self.inner[start_i];
        let b     = self.inner[end_i];

        if a == n && b == n + 1 {
            self.inner.remove(end_i);
            self.inner.remove(start_i);
        } else if a == n {
            self.inner[start_i] = n + 1;
        } else if b == n + 1 {
            self.inner[end_i] = n;
        } else {
            self.inner[end_i] = n;
            self.inner.insert(end_i + 1, b);
            self.inner.insert(end_i + 1, n + 1);
        }

        true
    }

    fn len(&self) -> usize {
        //* Stratégie
        //* —————————
        //* Chunk par deux les éléments de la liste,
        //* ce qui donne une liste de pair (donc d'interval)
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
        //* ce qui donne une liste de pair (donc d'interval)
        //* sur laquelle on ∪λi.∪(k=1, n)k

        Box::new(
            self.inner
                .chunks(2)
                .flat_map(|chunk| chunk[0]..chunk[1])
                .map(|x| x as u16),
        )
    }

    fn union_with(&mut self, other: &Self) {
        //* Stratégie
        //* —————————
        //* Crée un liste de bornes (e, δ) avec δ ∈ {-1, +1}
        //*   - e indique l'endroit de la borne
        //*   - δ indique si on entre (+1) ou sort (-1) d'un interval
        //* Trie la liste de bornes, puis crée les intervale
        //*   - une intersection débute quand le niveau passe de 0 à >0 et termine quand il passe de 0 à >0

        let mut bounds: Vec<(u32, i32)> = Vec::new(); // (e, δ)
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
        //*   - δ indique si on entre (+1) ou sort (-1) d'un interval
        //* Trie la liste de bornes, puis crée les intervale
        //*   - une intersection débute quand le niveau passe de <2 à 2 et termine quand il passe de 2 à <2
        
        let mut bounds: Vec<(u32, i32)> = Vec::new();  // (e, δ)
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
        //*   - δ_self vaut +1 quand on entre dans un interval de self -1 quand on en sort, 0 sinon
        //*   - δ_other vaut +1 quand on entre dans un interval de other -1 quand on en sort, 0 sinon
        //* Trie la liste de bornes, puis maintient deux niveaux de couverture :
        //*   - self_cov : nombre d'intervalles de self couvrant la position courante
        //*   - other_cov : nombre d'intervalles de other couvrant la position courante
        //* La différence A \ B est active lorsque
        //*   - self_cov > 0 et other_cov == 0

        let mut bounds: Vec<(u32, i32, i32)> = Vec::new(); // (e, δ_self, δ_other)
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
        //*   - δ_self vaut +1 quand on entre dans un interval de self -1 quand on en sort, 0 sinon
        //*   - δ_other vaut +1 quand on entre dans un interval de other -1 quand on en sort, 0 sinon
        //* Trie la liste de bornes, puis maintient deux niveaux de couverture :
        //*   - self_cov : nombre d'intervalles de self couvrant la position courante
        //*   - other_cov : nombre d'intervalles de other couvrant la position courante
        //* La différence symétrique A Δ B est active lorsque
        //*   - (self_cov > 0) XOR (other_cov > 0) est vrai

        let mut bounds: Vec<(u32, i32, i32)> = Vec::new(); // (e, δ_self, δ_other)
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