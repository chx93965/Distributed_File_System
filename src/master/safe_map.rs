use std::sync::Arc;
use std::collections::HashMap;
use std::sync::{RwLock, Mutex};
use std::hash::Hash;

pub struct SafeMap<A, T> 
where
    A: Eq + Hash
{
    pub inner: Mutex<Option<HashMap<A, Arc<RwLock<T>>>>>,
}

impl<A, T> SafeMap<A, T> 
where
    A: Eq + Hash
{
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }

    pub fn init(&self) {
        let mut guard = self.inner.lock().unwrap();
        if guard.is_none() {
            *guard = Some(HashMap::new());
        }
    }

    pub fn insert(&self, key: A, value: T) -> Option<Arc<RwLock<T>>> {
        let mut guard = self.inner.lock().unwrap();
        if let Some(map) = guard.as_mut() {
            map.insert(key, Arc::new(RwLock::new(value)))
        } else {
            None
        }
    }

    pub fn get(&self, key: &A) -> Option<Arc<RwLock<T>>> {
        let guard = self.inner.lock().unwrap();
        guard.as_ref().and_then(|map| map.get(key).cloned())
    }

    pub fn remove(&self, key: &A) -> Option<Arc<RwLock<T>>> {
        let mut guard = self.inner.lock().unwrap();
        if let Some(map) = guard.as_mut() {
            map.remove(key)
        } else {
            None
        }
    }

    /// Returns a vector containing all keys in the map.
    /// The keys will be cloned since we don't want to transfer ownership out of the map.
    pub fn keys(&self) -> Vec<A> 
    where
        A: Clone
    {
        let guard = self.inner.lock().unwrap();
        guard.as_ref()
            .map(|map| map.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn to_map(&self) -> HashMap<A, T>
    where
        A: Clone,
        T: Clone
    {
        let guard = self.inner.lock().unwrap();
        guard.as_ref()
            .map(|inner_map| inner_map.iter()
                .map(|(k, v)| (k.clone(), v.read().unwrap().clone())).collect())
            .unwrap_or_default()
    }
}