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

//! The half-open range algebra of a memory region, factored out of
//! `MemRegion` so it holds no region type and can be included by the
//! `mechanism_proofs` crate and checked against the Lean `Nonos.Interval` and
//! `Nonos.Vma` models. `MemRegion::overlaps`, `contains` and `contains_range`
//! delegate here. Each range is `[start, end)`.

/// Whether `[a_start, a_end)` and `[b_start, b_end)` share any address.
pub(crate) const fn overlaps(a_start: u64, a_end: u64, b_start: u64, b_end: u64) -> bool {
    a_start < b_end && b_start < a_end
}

/// Whether `addr` lies inside `[start, end)`.
pub(crate) const fn contains(start: u64, end: u64, addr: u64) -> bool {
    addr >= start && addr < end
}

/// Whether `[b_start, b_end)` lies wholly inside `[a_start, a_end)`.
pub(crate) const fn contains_range(a_start: u64, a_end: u64, b_start: u64, b_end: u64) -> bool {
    b_start >= a_start && b_end <= a_end
}
