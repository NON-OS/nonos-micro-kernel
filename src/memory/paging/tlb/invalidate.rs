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

//! TLB invalidation for the paging manager. Single-page and whole-set drops
//! come from the architecture; the range walk is shared, because deciding when
//! a range is cheaper to flush one page at a time than all at once is policy,
//! not hardware.

use crate::memory::addr::VirtAddr;
use crate::memory::paging::constants::PAGE_SIZE_4K;

/// Past this many pages a full flush costs less than the per-page walk.
const RANGE_FLUSH_THRESHOLD: usize = 32;

/// Drop the translation for one page.
#[inline]
pub fn invalidate_page(va: VirtAddr) {
    crate::arch::paging::invalidate_page(va.as_u64());
}

/// Drop every non-global translation.
#[inline]
pub fn invalidate_all() {
    crate::arch::paging::invalidate_all();
}

/// Drop `page_count` translations starting at `start`.
pub fn invalidate_range(start: VirtAddr, page_count: usize) {
    if page_count > RANGE_FLUSH_THRESHOLD {
        invalidate_all();
        return;
    }
    for i in 0..page_count {
        invalidate_page(VirtAddr::new(start.as_u64() + (i * PAGE_SIZE_4K) as u64));
    }
}
