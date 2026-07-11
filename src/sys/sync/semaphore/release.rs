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

use super::pure::release_count;
use super::state::Semaphore;

impl Semaphore {
    /// Return a permit, saturating at the ceiling so the count never exceeds
    /// the capacity the semaphore was built with.
    pub fn release(&self) {
        loop {
            let count = self.count.load(Ordering::Acquire);
            let next = release_count(count, self.cap);
            if self
                .count
                .compare_exchange_weak(count, next, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return;
            }
        }
    }

    /// The number of permits currently available.
    pub fn available(&self) -> usize {
        self.count.load(Ordering::Acquire)
    }

    /// The permit ceiling.
    pub fn capacity(&self) -> usize {
        self.cap
    }
}
