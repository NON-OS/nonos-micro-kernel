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

//! The sequence discipline of the seqlock, kept pure so it can be included by
//! the `sync_proofs` crate and checked against the Lean `Nonos.Seqlock` model.
//! `read.rs` and `write.rs` drive exactly these predicates: a writer bumps the
//! sequence odd on entry and even on exit, and a reader accepts a snapshot only
//! when `read_valid` holds.

/// Whether the sequence is stable: even means no write is in progress.
pub(crate) const fn is_stable(seq: u32) -> bool {
    seq % 2 == 0
}

/// The sequence one step on, wrapping. A writer bumps twice: odd then even.
pub(crate) const fn bump(seq: u32) -> u32 {
    seq.wrapping_add(1)
}

/// Whether a read bracketed by `before` and `after` is consistent: the
/// sequence was stable on entry and did not change across the read, so no
/// writer overlapped it.
pub(crate) const fn read_valid(before: u32, after: u32) -> bool {
    before == after && is_stable(before)
}
