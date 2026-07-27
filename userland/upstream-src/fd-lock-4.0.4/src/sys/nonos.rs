// NONOS advisory-lock backend.
//
// fd-lock's advisory locks exist to coordinate access between separate
// instances of a program. A NONOS capsule is single-instance and RAM-resident:
// it owns its file descriptors outright, so there is never a second holder to
// coordinate with. Every lock is therefore trivially acquired and the guards
// simply borrow the wrapped handle. No syscalls, and no panics.
use std::io;
use std::ops;

// Any handle can back a lock here; the capsule owns whatever it wraps.
pub(crate) trait AsOpenFile {}
impl<T> AsOpenFile for T {}

#[derive(Debug)]
pub struct RwLock<T> {
    inner: T,
}

impl<T> RwLock<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    #[inline]
    pub fn read(&self) -> io::Result<RwLockReadGuard<'_, T>> {
        Ok(RwLockReadGuard { lock: self })
    }

    #[inline]
    pub fn try_read(&self) -> io::Result<RwLockReadGuard<'_, T>> {
        Ok(RwLockReadGuard { lock: self })
    }

    #[inline]
    pub fn write(&mut self) -> io::Result<RwLockWriteGuard<'_, T>> {
        Ok(RwLockWriteGuard { lock: self })
    }

    #[inline]
    pub fn try_write(&mut self) -> io::Result<RwLockWriteGuard<'_, T>> {
        Ok(RwLockWriteGuard { lock: self })
    }

    #[inline]
    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.inner
    }
}

#[derive(Debug)]
pub struct RwLockReadGuard<'lock, T> {
    lock: &'lock RwLock<T>,
}

impl<T> ops::Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.lock.inner
    }
}

#[derive(Debug)]
pub struct RwLockWriteGuard<'lock, T> {
    lock: &'lock mut RwLock<T>,
}

impl<T> ops::Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.lock.inner
    }
}

impl<T> ops::DerefMut for RwLockWriteGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.lock.inner
    }
}
