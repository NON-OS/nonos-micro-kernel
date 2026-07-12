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

//! The bounds test of a relocation target, factored out of `in_segment` so it
//! holds no ELF image and can be included by the `mechanism_proofs` crate and
//! checked against the Lean `Nonos.Bounds` model. An access `[addr, addr+size)`
//! is in range only when it lies wholly inside `[start, start+seg_size)`, with
//! both ends guarded against overflow.

/// Whether `[addr, addr + size)` lies wholly inside `[start, start + seg_size)`.
/// A range whose end overflows the address space is never in range.
pub(crate) fn in_range(addr: u64, size: u64, start: u64, seg_size: u64) -> bool {
    match (start.checked_add(seg_size), addr.checked_add(size)) {
        (Some(end), Some(addr_end)) => addr >= start && addr_end <= end,
        _ => false,
    }
}
