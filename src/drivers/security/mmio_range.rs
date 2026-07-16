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

//! The window-validity arithmetic of an MMIO region, factored out of
//! `validate_mmio_region` so it holds no driver error type and can be included
//! by the `mechanism_proofs` crate and checked against the Lean `Nonos.Mmio`
//! model. A window is valid only when it is non-empty and does not wrap the
//! address space.

/// Whether `[base, base + size)` is a non-empty window that does not overflow
/// the address space.
pub(crate) const fn range_ok(base: usize, size: usize) -> bool {
    if size == 0 {
        return false;
    }
    match base.checked_add(size) {
        Some(end) => end > base,
        None => false,
    }
}
