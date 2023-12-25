use std::{collections::HashMap, hash::Hash, iter::repeat};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct UnionFind<T> {
    index: HashMap<T, usize>,
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl<T: Clone + Eq + Hash> From<Vec<T>> for UnionFind<T> {
    fn from(value: Vec<T>) -> Self {
        UnionFind {
            index: value
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, t)| (t, i))
                .collect(),
            parent: (0..value.len()).collect(),
            rank: repeat(0).take(value.len()).collect(),
        }
    }
}

impl<T: Clone + Eq + Hash> UnionFind<T> {
    pub fn union(&mut self, a: &T, b: &T) -> Option<usize> {
        let pap_a = self.find(a);
        let pap_b = self.find(b);

        if pap_a == pap_b {
            return None;
        }

        if self.rank[pap_a] > self.rank[pap_b] {
            self.rank[pap_a] += self.rank[pap_b] + 1;
            self.parent[pap_b] = pap_a;
            return Some(pap_a);
        } else {
            self.rank[pap_b] += self.rank[pap_a] + 1;
            self.parent[pap_a] = pap_b;
            return Some(pap_b);
        }
    }
    pub fn find(&mut self, a: &T) -> usize {
        self.find_id(self.index.get(a).unwrap().to_owned())
    }
    fn find_id(&mut self, a: usize) -> usize {
        if a != self.parent[a] {
            let pap = self.find_id(self.parent[a]);
            self.parent[a] = pap;
        }
        self.parent[a]
    }
    pub fn clusters(&mut self) -> Vec<Vec<T>> {
        let mut clusters: HashMap<usize, Vec<T>> = HashMap::new();

        for (value, id) in self.index.clone() {
            let cid = self.find_id(id.to_owned());
            clusters.entry(cid).or_insert_with(Vec::new).push(value)
        }

        clusters.values().cloned().collect_vec()
    }
}
