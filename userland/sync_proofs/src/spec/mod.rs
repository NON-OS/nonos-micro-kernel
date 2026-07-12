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

//! The executable specification the differential proofs compare against. Each
//! function restates, in the plainest Rust, the contract the Lean models
//! formalize. The differential harnesses run the real kernel functions,
//! included via `#[path]`, against these over the input space, so any drift
//! between the implementation and the specification breaks the build.

// Counting semaphore: verification/lean Nonos/Semaphore.lean `release`,
// `acquire`, `canAcquire`. Restated independently of the implementation.

/// A permit is available when the count is not empty.
pub fn sem_can_acquire(count: usize) -> bool {
    count != 0
}

/// Taking a permit lowers the count by one.
pub fn sem_acquire(count: usize) -> usize {
    count - 1
}

/// Returning a permit increments the count unless it is already at the cap.
pub fn sem_release(count: usize, cap: usize) -> usize {
    if count < cap {
        count + 1
    } else {
        cap
    }
}

// Seqlock: verification/lean Nonos/Seqlock.lean `stable`, `readAccepts`.
// Parity written as the low bit rather than a modulo, so the restatement is
// independent of the implementation's `% 2`.

/// The sequence is stable when its low bit is clear (even).
pub fn seq_stable(seq: u32) -> bool {
    seq & 1 == 0
}

/// A read is consistent when the sequence was stable on entry and unchanged.
pub fn seq_read_valid(before: u32, after: u32) -> bool {
    before == after && (before & 1 == 0)
}
