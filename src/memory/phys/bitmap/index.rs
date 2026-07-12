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

//! The bit-index arithmetic of the frame bitmap, factored out of `bit_ops.rs`
//! so it holds no pointer and touches no memory, and can be included by the
//! `mechanism_proofs` crate and checked against the Lean `Nonos.Bitmap` model.
//! `bit_test`, `bit_set` and `bit_clear` compute exactly these before touching
//! the backing store.

use super::super::constants::BITS_PER_BYTE;

/// The byte holding bit `idx`.
pub(crate) const fn byte_of(idx: usize) -> usize {
    idx / BITS_PER_BYTE
}

/// The single-bit mask selecting bit `idx` within its byte.
pub(crate) const fn bit_mask(idx: usize) -> u8 {
    1u8 << (idx & 7)
}
