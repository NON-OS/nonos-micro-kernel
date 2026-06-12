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

use super::constants::{DIRECTMAP_BASE, DIRECTMAP_SIZE, HUGE_1G, HUGE_2M, PTE_NX, PTE_RW};
use super::mapper::{map_huge_1g_run, map_huge_2m_run};
use super::page1g::supports_1gib_pages;
use super::table::PageTable;

// Install a linear directmap covering [0, DIRECTMAP_SIZE) of physical
// RAM at virtual address DIRECTMAP_BASE. Marked NX so the kernel never
// executes data pages reached through this window. Leaf granularity is
// 1 GiB where the CPU reports PDPE1GB, otherwise 2 MiB; the kernel
// clones this directmap branch into every address space, so a 1 GiB
// leaf on a CPU without 1 GiB pages would fault on first directmap use.
pub fn map_directmap(bs: &BootServices, pml4: PageTable) -> Result<(), &'static str> {
    if supports_1gib_pages() {
        let count = (DIRECTMAP_SIZE / HUGE_1G) as usize;
        map_huge_1g_run(bs, pml4, DIRECTMAP_BASE, 0, count, PTE_RW | PTE_NX)
    } else {
        let count = (DIRECTMAP_SIZE / HUGE_2M) as usize;
        map_huge_2m_run(bs, pml4, DIRECTMAP_BASE, 0, count, PTE_RW | PTE_NX)
    }
}
