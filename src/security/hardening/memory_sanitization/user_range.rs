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

//! Wiping a user range belonging to another address space. The shutdown wipe
//! runs in whichever process called it, so a foreign user address either
//! faults or scribbles the caller's own page at that address. Each page is
//! translated in the owning space and zeroed through the directmap instead.

use super::erase::secure_zero;
use crate::memory::addr::VirtAddr;
use crate::memory::layout;
use crate::memory::paging::manager;

/// Zero the mapped pages of `[start, end)` as the address space `asid` sees
/// them. Returns the number of pages wiped.
pub(super) fn wipe_user_range(asid: u32, start: u64, end: u64) -> usize {
    let page = layout::PAGE_SIZE as u64;
    if end <= start || !layout::in_user_space(start) || !layout::in_user_space(end - 1) {
        return 0;
    }
    let mut va = start & !(page - 1);
    let mut wiped = 0;
    while va < end {
        if let Some(pa) = manager::translate_in_asid(asid, VirtAddr::new(va)) {
            let direct = layout::DIRECTMAP_BASE.wrapping_add(pa.as_u64());
            // SAFETY: the frame is mapped in `asid`, so it is real memory, and
            // the directmap covers all of physical memory read-write for the
            // kernel half. The other CPUs are stopped before the shutdown wipe
            // runs, so nothing else is writing this frame.
            secure_zero(direct as *mut u8, page as usize);
            wiped += 1;
        }
        va = match va.checked_add(page) {
            Some(next) => next,
            None => break,
        };
    }
    wiped
}
