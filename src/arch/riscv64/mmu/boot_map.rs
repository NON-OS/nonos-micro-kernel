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

use super::super::boot::info::{BootInfo, MemoryRegion, MemoryType};
use super::branch::ensure_child;
use super::mode::MmuMode;
use super::satp::{make_satp, write_satp};
use super::state::root_table;
use super::sv39::{Sv39, MEGA_PAGE_SIZE};
use super::{tlb, PageAttributes};

pub fn init_mmu(boot_info: &BootInfo) {
    unsafe {
        setup_kernel_page_tables(boot_info);
        let satp = make_satp(MmuMode::Sv39, 0, root_table().ppn() as usize);
        write_satp(satp);
    }
    tlb::flush_tlb_all();
}

unsafe fn setup_kernel_page_tables(boot_info: &BootInfo) {
    for region in boot_info.memory_map().iter() {
        map_region(region);
    }
}

unsafe fn map_region(region: &MemoryRegion) {
    let attrs = region_attrs(region.region_type);
    let mut phys = region.base;
    let end = region.base.saturating_add(region.size);
    while phys < end {
        map_mega(phys, &attrs);
        phys = phys.saturating_add(MEGA_PAGE_SIZE as u64);
    }
}

fn region_attrs(kind: MemoryType) -> PageAttributes {
    match kind {
        MemoryType::Kernel => PageAttributes::kernel_code(),
        MemoryType::Available => PageAttributes::kernel_data(),
        _ => PageAttributes::kernel_rodata(),
    }
}

unsafe fn map_mega(phys: u64, attrs: &PageAttributes) {
    let va = phys as usize;
    let root = root_table();
    let vpn2 = Sv39::vpn(va, 2);
    let vpn1 = Sv39::vpn(va, 1);
    if let Some(l1) = ensure_child(root, vpn2) {
        l1.set_leaf(vpn1, phys >> 12, attrs);
    }
}
