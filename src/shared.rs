use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Shared mutable state behind an `Arc<Mutex<T>>` with poison-tolerant locking.
pub(crate) struct SharedMutex<T>(Arc<Mutex<T>>);

impl<T> SharedMutex<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self(Arc::new(Mutex::new(inner)))
    }

    pub(crate) fn lock(&self) -> MutexGuard<'_, T> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.0.try_lock().ok()
    }
}

impl<T: Default> Default for SharedMutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Clone for SharedMutex<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

/// Shared state behind an `Arc<RwLock<T>>` with poison-tolerant locking.
pub(crate) struct SharedRwLock<T>(Arc<RwLock<T>>);

impl<T> SharedRwLock<T> {
    pub(crate) fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        self.0.try_read().ok()
    }
}

impl<T: Default> Default for SharedRwLock<T> {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(T::default())))
    }
}

impl<T> Clone for SharedRwLock<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
