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

/// As many regions as the device tree walker reads in one pass.
pub const MAX_MEMORY_REGIONS: usize = 8;

/// As many CPUs as the device tree walker reads in one pass.
pub const MAX_CPUS: usize = 64;

use super::memory::{MemoryRegion, MemoryType};

#[derive(Debug, Clone)]
pub struct BootInfo {
    pub ram_base: u64,
    pub ram_size: u64,
    pub kernel_base: u64,
    pub kernel_size: u64,
    pub dtb_base: u64,
    pub dtb_size: u64,
    pub uart_base: u64,
    pub gic_dist_base: u64,
    pub gic_redist_base: u64,
    pub cpu_count: u32,
    /// Each CPU's MPIDR affinity, in device-tree order. PSCI needs one to name
    /// a core to power on and the GIC needs one to match a redistributor
    /// frame; a plain counter only happens to work on a single-cluster board.
    /// Fixed storage for the same reason as the memory map: filled from the
    /// device tree before the heap exists.
    pub cpu_affinities: [u64; MAX_CPUS],
    pub cpu_affinity_count: usize,
    pub timer_phys_intid: u32,
    pub timer_virt_intid: u32,
    pub gic_unsupported: bool,
    /// The PCI host bridge's configuration and I/O windows, all zero when the
    /// board has no PCI at all.
    pub pci_ecam_base: u64,
    pub pci_ecam_size: u64,
    pub pci_io_cpu_base: u64,
    pub pci_io_port_base: u64,
    pub pci_io_size: u64,
    /// The memory window the host bridge forwards to its devices. Booting from
    /// a device tree means no firmware has assigned any BAR, so the kernel
    /// hands out addresses from here itself.
    pub pci_mmio_base: u64,
    pub pci_mmio_size: u64,
    /// MMIO base of the PL031 real-time clock, zero when the board has none.
    pub rtc_base: u64,
    /// Fixed storage: this is filled from the device tree in `kernel_entry`,
    /// long before the heap exists, so it cannot be a `Vec`.
    pub memory_regions: [MemoryRegion; MAX_MEMORY_REGIONS],
    pub memory_region_count: usize,
}

impl Default for BootInfo {
    fn default() -> Self {
        Self {
            ram_base: 0x4000_0000,
            // Deliberately small. Without a device tree there is no way to learn
            // how much memory a board has, and the two errors are not
            // symmetric: claiming less than exists wastes it, while claiming
            // more hands the frame allocator addresses that answer to nothing.
            // A device tree replaces this with the real figure.
            ram_size: 0x2000_0000,
            kernel_base: 0x4000_0000,
            kernel_size: 0x0020_0000,
            dtb_base: 0,
            dtb_size: 0,
            uart_base: 0x0900_0000,
            gic_dist_base: 0x0800_0000,
            gic_redist_base: 0x080A_0000,
            cpu_count: 1,
            cpu_affinities: [0; MAX_CPUS],
            cpu_affinity_count: 0,
            // Every default here describes the QEMU virt board, and the timer is
            // no exception: PPI 14 and 11 are the non-secure physical and virtual
            // timers, which are INTID 30 and 27 once the 16 SGIs are counted. A
            // device tree that names them overrides this.
            timer_phys_intid: 30,
            timer_virt_intid: 27,
            gic_unsupported: false,
            // The virt board's PCIe host bridge. Config space is the high ECAM
            // window at 256 GiB, which is where the board puts it whenever the
            // CPU can address that far. The 16 MiB window at 0x3f00_0000 is the
            // older placement and reads there return nothing, which surfaces as
            // an external abort rather than a translation fault, since the page
            // tables are perfectly happy to map an address no device answers.
            // Its I/O window sits at 0x3eff_0000 and is addressed from port 0,
            // so a driver asking for port N reads 0x3eff_0000 + N.
            pci_ecam_base: 0x40_1000_0000,
            pci_ecam_size: 0x1000_0000,
            pci_io_cpu_base: 0x3eff_0000,
            pci_io_port_base: 0,
            pci_io_size: 0x0001_0000,
            pci_mmio_base: 0x1000_0000,
            pci_mmio_size: 0x2eff_0000,
            // PL031, the board's real-time clock.
            rtc_base: 0x0901_0000,
            memory_regions: {
                let mut regions = [MemoryRegion::EMPTY; MAX_MEMORY_REGIONS];
                // Same figures as the scalars above, so the map and the fields
                // that describe it cannot disagree.
                regions[0] = MemoryRegion {
                    base: 0x4000_0000,
                    size: 0x2000_0000,
                    region_type: MemoryType::Available,
                };
                regions
            },
            memory_region_count: 1,
        }
    }
}
