// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use uefi::table::boot::BootServices;

use super::constants::{HUGE_1G, HUGE_2M, IDENTITY_LOW_BYTES, PTE_RW};
use super::mapper::{map_huge_1g_run, map_huge_2m_run};
use super::page1g::supports_1gib_pages;
use super::table::PageTable;

// Identity-map the low memory region [0, IDENTITY_LOW_BYTES). The
// bootloader's text/data, the kernel's loaded ELF range (ET_DYN
// at low phys), the handoff struct, the boot stack, the memory
// map area, and the framebuffer (if low-mapped) all live here on
// every UEFI platform we ship. Identity-mapping [0,
// IDENTITY_LOW_BYTES) keeps every `mov rax, [imm]` and every
// `jmp rip+offset` valid across the CR3 swap, which is the only
// way the bootloader can switch CR3 and immediately call
// `jump_to_kernel` without a fault. The window must reach the
// firmware's image load phys (OVMF/QEMU 10.2.0 places it > 4 GiB).
//
// Leaf granularity follows CPU support: 1 GiB hugepages where the CPU
// reports PDPE1GB (QEMU, modern bare metal), otherwise 2 MiB hugepages
// (VirtualBox's default profile and other CPUs without 1 GiB pages,
// where a 1 GiB PDPTE would reserved-bit fault on first access).
pub fn map_identity_low(bs: &BootServices, pml4: PageTable) -> Result<(), &'static str> {
    if supports_1gib_pages() {
        let count = (IDENTITY_LOW_BYTES / HUGE_1G) as usize;
        map_huge_1g_run(bs, pml4, 0, 0, count, PTE_RW)
    } else {
        let count = (IDENTITY_LOW_BYTES / HUGE_2M) as usize;
        map_huge_2m_run(bs, pml4, 0, 0, count, PTE_RW)
    }
}
