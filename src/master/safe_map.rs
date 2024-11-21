use std::sync::Arc;
use std::collections::HashMap;
use std::sync::Mutex;
pub struct SafeMap<T> {
    pub inner: Mutex<Option<HashMap<String, Arc<T>>>>,
}

impl<T> SafeMap<T> {
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

    pub fn insert(&self, key: String, value: T) -> Option<Arc<T>> {
        let mut guard = self.inner.lock().unwrap();
        if let Some(map) = guard.as_mut() {
            map.insert(key, Arc::new(value))
        } else {
            None
        }
    }

    pub fn get(&self, key: &str) -> Option<Arc<T>> {
        let guard = self.inner.lock().unwrap();
        guard.as_ref().and_then(|map| map.get(key).cloned())
    }
}