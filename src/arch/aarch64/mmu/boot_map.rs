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

use super::super::boot::info::{BootInfo, MemoryType as BootMemoryType};
use super::{control, state, ttbr, PageAttributes};

const BLOCK_2M: u64 = 2 * 1024 * 1024;
const DEVICE_SLOT: usize = 3;
const MAX_RAM_SLOTS: usize = 3;

pub fn init_mmu(boot_info: &BootInfo) {
    control::configure_mair();
    control::configure_tcr();
    unsafe {
        setup_kernel_page_tables(boot_info);
    }
    control::enable_mmu();
}

unsafe fn setup_kernel_page_tables(boot_info: &BootInfo) {
    state::l0().set_table(0, state::l1_addr());
    if boot_info.memory_regions.is_empty() {
        map_range(0, boot_info.ram_base, boot_info.ram_size, BootMemoryType::Available);
    } else {
        for (slot, region) in boot_info.memory_regions.iter().take(MAX_RAM_SLOTS).enumerate() {
            map_range(slot, region.base, region.size, region.region_type);
        }
    }
    map_devices(boot_info);
    ttbr::set_ttbr1(state::l0_addr());
    ttbr::set_ttbr0(state::l0_addr(), 0);
}

unsafe fn map_range(slot: usize, base: u64, size: u64, kind: BootMemoryType) {
    state::l1().set_table(slot, state::l2_addr(slot));
    let attrs = region_attrs(kind);
    let mut phys = base;
    let end = base.saturating_add(size);
    let mut l2_idx = 0;
    while phys < end && l2_idx < 512 {
        state::l2(slot).set_block(l2_idx, phys, &attrs);
        phys = phys.saturating_add(BLOCK_2M);
        l2_idx += 1;
    }
}

/// Map the few devices the kernel must reach before any driver exists: the
/// console, and both halves of the interrupt controller.
///
/// The redistributor belongs here alongside the distributor. It is a separate
/// window, often megabytes away, and it is what enables a CPU's private
/// interrupts, so leaving it unmapped means the timer tick never arrives on
/// any core.
unsafe fn map_devices(boot_info: &BootInfo) {
    let attrs = PageAttributes::device();
    state::l1().set_table(510, state::l2_addr(DEVICE_SLOT));
    state::l2(DEVICE_SLOT).set_block(0, boot_info.uart_base & !(BLOCK_2M - 1), &attrs);
    state::l2(DEVICE_SLOT).set_block(1, boot_info.gic_dist_base & !(BLOCK_2M - 1), &attrs);
    state::l2(DEVICE_SLOT).set_block(2, boot_info.gic_redist_base & !(BLOCK_2M - 1), &attrs);
    // Redistributor frames run one per CPU at 64 KiB or 128 KiB apiece, so a
    // large machine's region can cross the 2 MiB block above.
    state::l2(DEVICE_SLOT).set_block(
        3,
        (boot_info.gic_redist_base & !(BLOCK_2M - 1)).saturating_add(BLOCK_2M),
        &attrs,
    );
}

fn region_attrs(kind: BootMemoryType) -> PageAttributes {
    match kind {
        BootMemoryType::Kernel => PageAttributes::kernel_code(),
        BootMemoryType::DeviceMemory => PageAttributes::device(),
        BootMemoryType::Available => PageAttributes::kernel_data(),
        _ => PageAttributes::kernel_rodata(),
    }
}
