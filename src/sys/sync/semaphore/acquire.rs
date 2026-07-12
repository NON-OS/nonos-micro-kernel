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

use core::sync::atomic::Ordering;

use super::pure::{acquire_count, can_acquire};
use super::state::Semaphore;

impl Semaphore {
    /// Take a permit without blocking. Returns `true` on success. Lock-free:
    /// it decrements only when a permit is available and retries on
    /// contention, so it never drives the count below zero.
    pub fn try_acquire(&self) -> bool {
        loop {
            let count = self.count.load(Ordering::Acquire);
            if !can_acquire(count) {
                return false;
            }
            if self
                .count
                .compare_exchange_weak(
                    count,
                    acquire_count(count),
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .is_ok()
            {
                return true;
            }
        }
    }

    /// Take a permit, yielding the CPU until one becomes available.
    pub fn acquire(&self) {
        while !self.try_acquire() {
            crate::sched::yield_now();
        }
    }
}
