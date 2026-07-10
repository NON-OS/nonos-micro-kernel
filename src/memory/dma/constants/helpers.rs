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

use super::sizes::DMA32_LIMIT;
use crate::memory::layout;

#[inline]
pub const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

#[inline]
pub const fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

#[inline]
pub const fn is_aligned(value: usize, align: usize) -> bool {
    value & (align - 1) == 0
}

#[inline]
pub const fn pages_needed(size: usize) -> usize {
    // div_ceil avoids an overflow in the `size + PAGE_SIZE - 1` intermediate
    // that would abort under release overflow-checks near usize::MAX.
    size.div_ceil(layout::PAGE_SIZE)
}

#[inline]
pub const fn is_dma32_compatible(phys_addr: u64) -> bool {
    phys_addr < DMA32_LIMIT
}

#[inline]
pub const fn is_range_dma32_compatible(phys_addr: u64, size: usize) -> bool {
    phys_addr + size as u64 <= DMA32_LIMIT
}
