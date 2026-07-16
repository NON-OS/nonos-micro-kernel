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

use super::state::Semaphore;

impl Semaphore {
    /// Create a semaphore with `initial` permits and a ceiling of `cap`.
    /// `initial` is clamped to `cap` so the count starts within the invariant.
    pub const fn new(initial: usize, cap: usize) -> Self {
        let start = if initial > cap { cap } else { initial };
        Self { count: AtomicUsize::new(start), cap }
    }
}
