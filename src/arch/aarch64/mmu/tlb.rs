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

use core::arch::asm;

pub fn flush_tlb_all() {
    unsafe {
        asm!("dsb ishst", "tlbi vmalle1is", "dsb ish", "isb", options(nostack));
    }
}

pub fn flush_tlb_page(addr: u64) {
    let page = addr >> 12;
    unsafe {
        asm!("dsb ishst", "tlbi vaae1is, {0}", "dsb ish", "isb", in(reg) page, options(nostack));
    }
}

pub fn flush_tlb_asid(asid: u16) {
    let asid_val = (asid as u64) << 48;
    unsafe {
        asm!("dsb ishst", "tlbi aside1is, {0}", "dsb ish", "isb", in(reg) asid_val, options(nostack));
    }
}
