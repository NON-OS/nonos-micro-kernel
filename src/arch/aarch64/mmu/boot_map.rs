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

extern "C" {
    static __kernel_image_start: u8;
    static __kernel_rw_start: u8;
}

use super::super::boot::info::{BootInfo, MemoryType as BootMemoryType};
use super::{control, state, ttbr, PageAttributes};

const BLOCK_2M: u64 = 2 * 1024 * 1024;
const DEVICE_SLOT: usize = 3;
const MAX_RAM_SLOTS: usize = 3;

/// Base of the kernel's direct map, matching what shared code adds to a
/// physical address to get a pointer.
pub const KERNEL_SPACE_START: u64 = 0xFFFF_8000_0000_0000;
/// Level 0 entry the direct map hangs off, bits 47:39 of its base.
const DIRECT_L0_INDEX: usize = 256;
/// Level 2 tables reserved for the direct map, after the identity and device
/// ones.
const DIRECT_SLOT: usize = 4;
const DIRECT_SLOTS: usize = 2;

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
    if boot_info.memory_map().is_empty() {
        map_range(0, boot_info.ram_base, boot_info.ram_size, BootMemoryType::Available);
    } else {
        for (slot, region) in boot_info.memory_map().iter().take(MAX_RAM_SLOTS).enumerate() {
            map_range(slot, region.base, region.size, region.region_type);
        }
    }
    map_devices(boot_info);
    map_direct(boot_info);
    ttbr::set_ttbr1(state::l0_addr());
    ttbr::set_ttbr0(state::l0_addr(), 0);
}

/// The kernel's direct map: every physical page reachable at its address plus
/// `KERNEL_SPACE_START`.
///
/// Shared code turns a physical address into a pointer by adding that constant,
/// which is how the paging manager reads a page table it has only the physical
/// address of. Without this the first such read faults on an address nothing
/// describes, and the identity map alone cannot serve it: the two windows differ
/// by more than the top bits, so they need separate tables.
unsafe fn map_direct(boot_info: &BootInfo) {
    state::l0().set_table(DIRECT_L0_INDEX, state::l1_high_addr());
    if boot_info.memory_map().is_empty() {
        map_direct_range(DIRECT_SLOT, boot_info.ram_base, boot_info.ram_size);
    } else {
        for (n, region) in boot_info.memory_map().iter().take(DIRECT_SLOTS).enumerate() {
            map_direct_range(DIRECT_SLOT + n, region.base, region.size);
        }
    }
}

/// Map one physical region into the direct window with 2MB blocks. Indices come
/// from the virtual address, the blocks point at the physical one.
unsafe fn map_direct_range(slot: usize, base: u64, size: u64) {
    const ENTRIES: u64 = 512;
    const GIB: u64 = ENTRIES * BLOCK_2M;

    if slot >= DIRECT_SLOT + DIRECT_SLOTS {
        return;
    }
    // Writable and execute-never: this window exists to read and write memory
    // the kernel knows only by physical address, never to run from.
    let attrs = PageAttributes::kernel_data();
    let mut phys = base & !(BLOCK_2M - 1);
    let end = base.saturating_add(size);
    let table_end = (phys / GIB + 1) * GIB;
    state::l1_high()
        .set_table(((phys + KERNEL_SPACE_START) / GIB % ENTRIES) as usize, state::l2_addr(slot));
    while phys < end && phys < table_end {
        let virt = phys.wrapping_add(KERNEL_SPACE_START);
        let l2_idx = ((virt / BLOCK_2M) % ENTRIES) as usize;
        state::l2(slot).set_block(l2_idx, phys, &attrs);
        phys = phys.saturating_add(BLOCK_2M);
    }
}

/// Identity map one region with 2MB blocks. `slot` picks which level 2 table to
/// fill; where it hangs off level 1 comes from the address, since an L1 entry
/// covers a gigabyte and an L2 block two megabytes.
unsafe fn map_range(slot: usize, base: u64, size: u64, kind: BootMemoryType) {
    const ENTRIES: u64 = 512;
    const GIB: u64 = ENTRIES * BLOCK_2M;

    let l1_idx = (base / GIB) as usize;
    if l1_idx >= ENTRIES as usize {
        return;
    }
    state::l1().set_table(l1_idx, state::l2_addr(slot));

    let data_attrs = region_attrs(kind);
    let code_attrs = PageAttributes::kernel_code();
    let img_start = (&raw const __kernel_image_start) as u64;
    // Only text and rodata are executable and read only. Everything from the
    // writable data onward, .bss and the stack included, has to stay writable.
    let img_end = (&raw const __kernel_rw_start) as u64;
    let mut phys = base & !(BLOCK_2M - 1);
    let end = base.saturating_add(size);
    // One table describes one gigabyte; anything past it belongs to another slot.
    let table_end = ((l1_idx as u64) + 1) * GIB;
    while phys < end && phys < table_end {
        let l2_idx = ((phys / BLOCK_2M) % ENTRIES) as usize;
        // The image is inside a region the firmware calls Available, so pick
        // attributes per block rather than per region or our own text ends up
        // mapped execute-never.
        let overlaps_image = phys < img_end && phys.saturating_add(BLOCK_2M) > img_start;
        let attrs = if overlaps_image { &code_attrs } else { &data_attrs };
        state::l2(slot).set_block(l2_idx, phys, attrs);
        phys = phys.saturating_add(BLOCK_2M);
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
    // Redistributor frames run one per CPU at 64 or 128 KiB apiece, so a large
    // machine reaches past the block holding the base.
    let redist = boot_info.gic_redist_base & !(BLOCK_2M - 1);
    let windows =
        [boot_info.uart_base, boot_info.gic_dist_base, redist, redist.saturating_add(BLOCK_2M)];
    for window in windows {
        map_device_block(window, &attrs);
    }
    // The bridge's I/O window and the memory its BARs are assigned out of.
    // Config space is deliberately absent: the board puts it at 256 GiB, past
    // what one level 1 table here describes, and nothing touches it until the
    // unified address space is up and the MMIO mapper can place it.
    map_device_range(boot_info.pci_io_cpu_base, boot_info.pci_io_size, &attrs);
    map_device_range(boot_info.pci_mmio_base, boot_info.pci_mmio_size, &attrs);
    map_device_range(boot_info.rtc_base, BLOCK_2M, &attrs);
}

/// Identity map every 2MB block that a window touches.
unsafe fn map_device_range(base: u64, size: u64, attrs: &PageAttributes) {
    if base == 0 || size == 0 {
        return;
    }
    let start = base & !(BLOCK_2M - 1);
    let end = base.saturating_add(size).saturating_add(BLOCK_2M - 1) & !(BLOCK_2M - 1);
    let mut addr = start;
    while addr < end {
        map_device_block(addr, attrs);
        addr = addr.saturating_add(BLOCK_2M);
    }
}

/// Identity map the 2MB block holding one MMIO window. Same rule as RAM: the
/// level 1 entry follows the address, so the window lands where the driver
/// expects to find it.
unsafe fn map_device_block(addr: u64, attrs: &PageAttributes) {
    const ENTRIES: u64 = 512;
    const GIB: u64 = ENTRIES * BLOCK_2M;

    let l1_idx = (addr / GIB) as usize;
    if l1_idx >= ENTRIES as usize {
        return;
    }
    state::l1().set_table(l1_idx, state::l2_addr(DEVICE_SLOT));
    let l2_idx = ((addr / BLOCK_2M) % ENTRIES) as usize;
    state::l2(DEVICE_SLOT).set_block(l2_idx, addr & !(BLOCK_2M - 1), attrs);
}

fn region_attrs(kind: BootMemoryType) -> PageAttributes {
    match kind {
        BootMemoryType::Kernel => PageAttributes::kernel_code(),
        BootMemoryType::DeviceMemory => PageAttributes::device(),
        BootMemoryType::Available => PageAttributes::kernel_data(),
        _ => PageAttributes::kernel_rodata(),
    }
}
