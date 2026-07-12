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

//! The page reference-count decrement, factored out of `decrement_ref_count` so
//! it holds no page table and can be included by the `mechanism_proofs` crate
//! and checked against the Lean `Nonos.Refcount` model. A decrement of a live
//! page lowers the count by one; a decrement of a count already at zero is
//! refused, so the count never underflows.

/// Decrement a reference count, or `None` if it is already zero. A `Some(next)`
/// result always has `next < ref_count`, so the count never wraps below zero.
pub(crate) const fn dec_checked(ref_count: u32) -> Option<u32> {
    if ref_count == 0 {
        None
    } else {
        Some(ref_count - 1)
    }
}
