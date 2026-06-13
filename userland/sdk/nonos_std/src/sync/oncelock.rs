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

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU8, Ordering};

const EMPTY: u8 = 0;
const INIT: u8 = 1;
const FULL: u8 = 2;

pub struct OnceLock<T> {
    state: AtomicU8,
    value: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Send + Sync> Sync for OnceLock<T> {}
unsafe impl<T: Send> Send for OnceLock<T> {}

impl<T> OnceLock<T> {
    pub const fn new() -> Self {
        Self { state: AtomicU8::new(EMPTY), value: UnsafeCell::new(MaybeUninit::uninit()) }
    }

    pub fn get(&self) -> Option<&T> {
        match self.state.load(Ordering::Acquire) {
            FULL => Some(unsafe { (*self.value.get()).assume_init_ref() }),
            _ => None,
        }
    }

    pub fn set(&self, value: T) -> Result<(), T> {
        if self.state.compare_exchange(EMPTY, INIT, Ordering::Acquire, Ordering::Relaxed).is_err() {
            return Err(value);
        }
        unsafe { (*self.value.get()).write(value) };
        self.state.store(FULL, Ordering::Release);
        Ok(())
    }

    pub fn get_or_init<F: FnOnce() -> T>(&self, f: F) -> &T {
        if self.get().is_none() {
            let _ = self.set(f());
        }
        unsafe { (*self.value.get()).assume_init_ref() }
    }
}

impl<T> Default for OnceLock<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for OnceLock<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == FULL {
            unsafe { (*self.value.get()).assume_init_drop() };
        }
    }
}
