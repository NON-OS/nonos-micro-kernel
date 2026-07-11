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

use super::page_sizes::PAGE_SIZE_4K;
use super::pt_index::PAGE_OFFSET_MASK;

#[inline]
pub const fn page_align_down(addr: u64) -> u64 {
    addr & !PAGE_OFFSET_MASK
}

#[inline]
pub const fn page_align_up(addr: u64) -> u64 {
    (addr + PAGE_OFFSET_MASK) & !PAGE_OFFSET_MASK
}

#[inline]
pub const fn pages_needed(size: usize) -> usize {
    // div_ceil avoids the `size + PAGE_SIZE_4K - 1` intermediate, which would
    // overflow (and abort under release overflow-checks) near usize::MAX.
    size.div_ceil(PAGE_SIZE_4K)
}
