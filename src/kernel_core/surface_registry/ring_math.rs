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

//! The index arithmetic of the input ring, factored out of `input_ring.rs` so
//! it holds no lock and can be included by the `mechanism_proofs` crate and
//! checked against the Lean `Nonos.Ring` model. The ring stores at most
//! `cap - 1` events; a position advances modulo the capacity and the ring is
//! full when advancing the head would reach the tail.

/// Advance a ring position by one, wrapping at the capacity.
pub(crate) const fn wrap(pos: usize, cap: usize) -> usize {
    (pos + 1) % cap
}

/// Whether the ring is full: advancing the head would collide with the tail.
pub(crate) const fn is_full(head: usize, tail: usize, cap: usize) -> bool {
    wrap(head, cap) == tail
}
