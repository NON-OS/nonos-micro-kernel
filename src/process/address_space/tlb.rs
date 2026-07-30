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

use super::pcid::pcid_enabled;
use crate::memory::addr::VirtAddr;

/// Retire the translation covering one page.
#[inline(always)]
pub fn invlpg(virt: VirtAddr) {
    crate::arch::paging::invalidate_page(virt.as_u64());
}

/// Retire every non-global translation on this core.
#[inline(always)]
pub fn flush_tlb() {
    crate::arch::paging::invalidate_all();
}

/// Retire one address space, leaving the others in the TLB. Falls back to the
/// full flush where the part cannot invalidate by tag, which is correct but
/// costs every other address space its entries too.
#[inline(always)]
pub fn flush_tlb_pcid(pcid: u16) {
    if pcid_enabled() {
        crate::arch::paging::invalidate_tagged(pcid);
    } else {
        flush_tlb();
    }
}
