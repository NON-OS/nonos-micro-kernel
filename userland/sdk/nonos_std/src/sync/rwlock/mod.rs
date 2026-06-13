// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

mod guards;

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

use super::poison::LockResult;
pub use guards::{RwLockReadGuard, RwLockWriteGuard};

const WRITER: usize = usize::MAX;

pub struct RwLock<T: ?Sized> {
    state: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for RwLock<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    pub const fn new(value: T) -> Self {
        Self { state: AtomicUsize::new(0), data: UnsafeCell::new(value) }
    }
}

impl<T: ?Sized> RwLock<T> {
    pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
        loop {
            let s = self.state.load(Ordering::Relaxed);
            if s != WRITER
                && self.state.compare_exchange(s, s + 1, Ordering::Acquire, Ordering::Relaxed).is_ok()
            {
                return Ok(RwLockReadGuard::new(self));
            }
            nonos_libc::mk_yield();
        }
    }

    pub fn write(&self) -> LockResult<RwLockWriteGuard<'_, T>> {
        while self.state.compare_exchange(0, WRITER, Ordering::Acquire, Ordering::Relaxed).is_err() {
            nonos_libc::mk_yield();
        }
        Ok(RwLockWriteGuard::new(self))
    }
}
