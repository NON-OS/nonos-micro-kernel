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

//! The permit arithmetic of the counting semaphore, kept pure and free of any
//! atomics or scheduler calls so it can be included verbatim by the
//! `sync_proofs` crate and checked against the Lean `Nonos.Semaphore` model.
//! `acquire.rs` and `release.rs` perform exactly these computations under a
//! compare-exchange loop.

/// Whether a permit is available to take.
pub(crate) const fn can_acquire(count: usize) -> bool {
    count > 0
}

/// The count after taking a permit. Only valid when `can_acquire(count)`.
pub(crate) const fn acquire_count(count: usize) -> usize {
    count - 1
}

/// The count after returning a permit, saturating at the ceiling so the
/// invariant `count <= cap` is preserved.
pub(crate) const fn release_count(count: usize, cap: usize) -> usize {
    if count >= cap {
        cap
    } else {
        count + 1
    }
}
