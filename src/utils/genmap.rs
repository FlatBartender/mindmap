use slotmap::*;
use slotmap::secondary::{Drain, Iter, IterMut, IntoIter, Keys, Values, ValuesMut};

use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct GenMap<K: Key, V> {
    main: SlotMap<K, ()>,
    sec:  SecondaryMap<K, V>,
}

impl<K: Key, V> GenMap<K, V> {
    pub fn capacity(&self) -> usize {
        self.main.capacity()
    }
    
    pub fn clear(&mut self) {
        self.main.clear();
        self.sec.clear();
    }

    pub fn contains_key(&self, key: K) -> bool {
        self.main.contains_key(key)
    }

    pub fn drain(&mut self) -> Drain<K, V> {
        self.sec.drain()
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.sec.get(key)
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.sec.get_mut(key)
    }

    pub unsafe fn get_unchecked(&self, key: K) -> &V {
        self.sec.get_unchecked(key)
    }

    pub unsafe fn get_unchecked_mut(&mut self, key: K) -> &mut V {
        self.sec.get_unchecked_mut(key)
    }

    pub fn insert(&mut self, value: V) -> K {
        let key = self.main.insert(());
        self.sec.insert(key.clone(), value);
        key
    }

    pub fn insert_with_key<F>(&mut self, f: F) -> K
    where F: FnOnce(K) -> V {
        let key = self.main.insert(());
        self.sec.insert(key.clone(), f(key.clone()));
        key
    }

    pub fn is_empty(&self) -> bool {
        self.main.is_empty()
    }

    pub fn iter(&self) -> Iter<K, V> {
        self.sec.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.sec.iter_mut()
    }

    pub fn keys(&self) -> Keys<K, V> {
        self.sec.keys()
    }

    pub fn len(&self) -> usize {
        self.main.len()
    }

    pub fn new() -> Self {
        GenMap {
            main: SlotMap::with_key(),
            sec: SecondaryMap::new(),
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let value = self.sec.remove(key.clone());
        self.main.remove(key);
        value
    }

    // Did not implement retain
    
    pub fn values(&self) -> Values<K, V> {
        self.sec.values()
    }

    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        self.sec.values_mut()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        GenMap {
            main: SlotMap::with_capacity_and_key(capacity.clone()),
            sec: SecondaryMap::with_capacity(capacity),
        }
    }
}

impl<K: Key, V> Default for GenMap<K, V> {
    fn default() -> Self {
        GenMap::new()
    }
}

impl<K: Key, V> Index<K> for GenMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &V {
        match self.get(key) {
            Some(r) => r,
            None => panic!("invalid GenMap key used"),
        }
    }
}

impl<K: Key, V> IndexMut<K> for GenMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut V {
        match self.get_mut(key) {
            Some(r) => r,
            None => panic!("invalid GenMap key used"),
        }
    }
}

impl<'a, K: Key, V> IntoIterator for &'a GenMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.sec.iter()
    }
}

impl<'a, K: Key, V> IntoIterator for &'a mut GenMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.sec.iter_mut()
    }
}

impl<K: Key, V> IntoIterator for GenMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.sec.into_iter()
    }
}
