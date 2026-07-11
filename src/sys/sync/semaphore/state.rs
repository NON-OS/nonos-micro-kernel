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

use core::sync::atomic::AtomicUsize;

// A counting semaphore. `count` is the number of permits currently available
// and `cap` is the ceiling it saturates at, so the invariant `count <= cap`
// holds for the lifetime of the semaphore. Permits are taken and returned with
// lock-free compare-exchange loops, so the primitive is SMP-safe without a
// backing lock.
pub struct Semaphore {
    pub(super) count: AtomicUsize,
    pub(super) cap: usize,
}
