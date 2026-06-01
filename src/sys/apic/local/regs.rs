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

use core::sync::atomic::Ordering;

use super::state::LAPIC_BASE;

// SAFETY: Caller guarantees LAPIC_BASE points at a mapped UC page and
// `reg` is a valid LAPIC register offset (0..=0x3F0 step 0x10). All
// LAPIC registers are 32 bits, naturally aligned. The volatile pair
// suppresses re-ordering.
pub(in crate::sys::apic) unsafe fn lapic_read_raw(reg: u32) -> u32 {
    unsafe {
        let base = LAPIC_BASE.load(Ordering::Relaxed);
        let ptr = (base + reg as u64) as *const u32;
        core::ptr::read_volatile(ptr)
    }
}

pub(in crate::sys::apic) unsafe fn lapic_write_raw(reg: u32, value: u32) {
    unsafe {
        let base = LAPIC_BASE.load(Ordering::Relaxed);
        let ptr = (base + reg as u64) as *mut u32;
        core::ptr::write_volatile(ptr, value);
    }
}
