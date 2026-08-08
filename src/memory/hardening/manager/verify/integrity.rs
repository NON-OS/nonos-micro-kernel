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

use super::super::super::constants::*;
use super::helpers::read_bytes;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::PagePermissions;
use crate::memory::{heap, kaslr, layout, paging, safety};

pub fn verify_kernel_data_integrity() -> bool {
    if layout::validate_layout().is_err() {
        return false;
    }
    let current_cr3 = paging::get_current_cr3();
    if current_cr3.as_u64() == 0 {
        return false;
    }
// The required CR4 bits are SMEP, SMAP and the rest of the ring-0
// restrictions, which only x86_64 keeps in a control register. aarch64
// enforces the same properties through PAN and the PXN and UXN table bits,
// and those are checked with the tables below rather than here.
#[cfg(target_arch = "x86_64")]
{
    let current_cr4: u64;
    // SAFETY: reading CR4 has no side effect and touches no memory.
    unsafe {
        core::arch::asm!("mov {}, cr4", out(reg) current_cr4, options(nostack, preserves_flags));
    }
    if (current_cr4 & CR4_REQUIRED_BITS) != CR4_REQUIRED_BITS {
        return false;
    }
}
    if !verify_kernel_page_tables() {
        return false;
    }
    let kernel_sections = layout::kernel_sections();
    for section in &kernel_sections {
        let va = VirtAddr::new(section.start);
        if let Some(pa) = paging::translate_address(va) {
            if pa.as_u64() == 0 || pa.as_u64() > layout::MAX_PHYS_ADDR {
                return false;
            }
            if let Some(perms) = paging::get_page_permissions(va) {
                if section.rx && !perms.contains(PagePermissions::EXECUTE) {
                    return false;
                }
                if section.rw && !perms.contains(PagePermissions::WRITE) {
                    return false;
                }
                if !section.rw && perms.contains(PagePermissions::WRITE) {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    if !safety::verify_stack_integrity() {
        return false;
    }
    if !heap::verify_heap_integrity() {
        return false;
    }
    if !kaslr::verify_slide_integrity() {
        return false;
    }
    let kernel_entry_point = layout::KERNEL_BASE;
    if paging::translate_address(VirtAddr::new(kernel_entry_point)).is_some() {
        if let Ok(entry_bytes) = read_bytes(kernel_entry_point as usize, NOP_SLED_CHECK_SIZE) {
            if entry_bytes.iter().all(|&b| b == NOP_INSTRUCTION) {
                return false;
            }
            if entry_bytes.iter().all(|&b| b == 0x00) {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
    true
}

pub fn verify_kernel_page_tables() -> bool {
    let current_cr3 = paging::get_current_cr3();
    if current_cr3.as_u64() == 0 {
        return false;
    }
    let kernel_sections = layout::kernel_sections();
    for section in &kernel_sections {
        let va = VirtAddr::new(section.start);
        if let Some(perms) = paging::get_page_permissions(va) {
            if section.rx && !perms.contains(PagePermissions::EXECUTE) {
                return false;
            }
            if section.rw && !perms.contains(PagePermissions::WRITE) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
