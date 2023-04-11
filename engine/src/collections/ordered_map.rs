use std::{
    collections::HashMap,
    hash::Hash,
};

pub trait ExtractComparable<C> {
    fn extract_comparable(&self) -> C;
}

#[derive(Clone, Debug)]
pub struct OrderedMap<K, V, C> {
    map: HashMap<K, V>,
    descending_pairs: Vec<(K, C)>
}

impl<'a, K: 'a, V: 'a, C: 'a> OrderedMap<K, V, C>
where
    K: Eq + Hash + Copy,
    V: ExtractComparable<C>,
    C: PartialOrd,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            descending_pairs: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    fn insert_into_pairs(&mut self, k: K, c: C) {
        let mut insert_index = None;
        for (i, (_, other_c)) in self.descending_pairs.iter().enumerate() {
            if &c >= other_c {
                insert_index = Some(i);
                break;
            }
        }

        let idx = insert_index.unwrap_or(self.descending_pairs.len());
        self.descending_pairs.insert(idx, (k, c));
    }

    fn remove_from_pairs(&mut self, k: &K) -> bool {
        let mut removed = false;
        for i in 0..self.descending_pairs.len() {
            if self.descending_pairs[i].0 == *k {
                self.descending_pairs.remove(i);
                removed = true;
                break;
            }
        }
        removed
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let new_c = <V as ExtractComparable<C>>::extract_comparable(&v);
        match self.map.insert(k, v) {
            None => {
                self.insert_into_pairs(k, new_c);
                None
            }
            Some(v) => {
                self.remove_from_pairs(&k);
                self.insert_into_pairs(k, new_c);
                Some(v)
            }
        }
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        self.map.remove(k).and_then(|v| {
            self.remove_from_pairs(k);
            Some(v)
        })
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.map.get(k)
    }

    pub fn get_mut<T>(&mut self, k: &K, func: impl FnOnce(&mut V) -> T) -> Option<T> {
        if let Some(v) = self.map.get_mut(k) {
            let c = v.extract_comparable();
            let t = func(v);
            let new_c = v.extract_comparable();

            if c != new_c {
                self.remove_from_pairs(&k);
                self.insert_into_pairs(k.clone(), new_c);
            }

            Some(t)
        }
        else {
            None
        }
    }

    pub fn nth(&self, n: usize) -> Option<&V> {
        self.descending_pairs.get(n).and_then(|(k, _)| self.map.get(k))
    }

    pub fn nth_mut<T>(&mut self, n: usize, func: impl FnOnce(&mut V) -> T) -> Option<T> {
        if let Some((k, _)) = self.descending_pairs.get(n) && let Some(v) = self.map.get_mut(k) {
            let c = v.extract_comparable();
            let t = func(v);
            let new_c = v.extract_comparable();

            let owned_k = k.to_owned();
            if c != new_c {
                self.remove_from_pairs(&owned_k);
                self.insert_into_pairs(owned_k, new_c);
            }

            Some(t)
        }
        else {
            None
        }
    }

    pub fn iter(&self) -> Iter<K, V, C> {
        Iter {
            map: &self.map,
            descending_pairs: &self.descending_pairs,
            index: 0
        }
    }
}

pub struct Iter<'a, K, V, C> {
    map: &'a HashMap<K, V>,
    descending_pairs: &'a Vec<(K, C)>,
    index: usize
}

impl<'a, K: Eq + Hash, V, C> Iterator for Iter<'a, K, V, C> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.descending_pairs.len() {
            return None
        }

        self.index += 1;
        self.map.get_key_value(&self.descending_pairs[self.index - 1].0)
    }
}

impl<'a, K: Eq + Hash, V, C> DoubleEndedIterator for Iter<'a, K, V, C> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let total = self.descending_pairs.len();
        if total == 0 || total - self.index == 0 {
            return None;
        } 

        self.index += 1;
        self.map.get_key_value(&self.descending_pairs[total - self.index].0)
    }
}