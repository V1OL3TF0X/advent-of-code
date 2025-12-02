use std::slice::Iter;

#[derive(Debug)]
pub struct VecMap<K: Eq, V> {
    map: Vec<(K, V)>,
}

impl<K: Eq, V> Default for VecMap<K, V> {
    fn default() -> Self {
        Self { map: vec![] }
    }
}

impl<K: Eq, V> VecMap<K, V> {
    pub fn find(&self, key: K) -> Option<&V> {
        self.map.iter().find(|v| v.0 == key).map(|v| &v.1)
    }
    pub fn entries(&self) -> Iter<'_, (K, V)> {
        self.map.iter()
    }
    pub fn find_mut(&mut self, key: &K) -> Option<&mut (K, V)> {
        self.map.iter_mut().find(|v| &v.0 == key)
    }
    pub fn set(&mut self, key: K, v: V) {
        if let Some(e) = self.find_mut(&key) {
            e.1 = v;
        } else {
            self.map.push((key, v));
        }
    }
}

impl<K: Eq + Ord, V: Ord> VecMap<K, V> {
    pub fn sort(&mut self) {
        self.map
            .sort_by(|e1, e2| e2.1.cmp(&e1.1).then_with(|| e2.0.cmp(&e1.0)))
    }
}

impl<K: Eq + Ord, V: Default> VecMap<K, V> {
    pub fn modify(&mut self, key: K, map: impl FnOnce(&V) -> V) {
        if let Some(e) = self.find_mut(&key) {
            e.1 = map(&e.1);
        } else {
            self.map.push((key, map(&V::default())));
        }
    }
}

impl<K: Eq, V> From<Vec<(K, V)>> for VecMap<K, V> {
    fn from(value: Vec<(K, V)>) -> Self {
        Self { map: value }
    }
}

impl<K: Eq, V> From<VecMap<K, V>> for Vec<(K, V)> {
    fn from(value: VecMap<K, V>) -> Self {
        value.map
    }
}
