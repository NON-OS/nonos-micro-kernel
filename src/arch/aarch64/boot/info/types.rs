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

use alloc::vec::Vec;

use super::memory::MemoryRegion;

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
    pub cpu_affinities: Vec<u64>,
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
    /// MMIO base of the PL031 real-time clock, zero when the board has none.
    pub rtc_base: u64,
    pub memory_regions: Vec<MemoryRegion>,
}

impl Default for BootInfo {
    fn default() -> Self {
        Self {
            ram_base: 0x4000_0000,
            ram_size: 0x1_0000_0000,
            kernel_base: 0x4000_0000,
            kernel_size: 0x0020_0000,
            dtb_base: 0,
            dtb_size: 0,
            uart_base: 0x0900_0000,
            gic_dist_base: 0x0800_0000,
            gic_redist_base: 0x080A_0000,
            cpu_count: 1,
            cpu_affinities: Vec::new(),
            timer_phys_intid: 0,
            timer_virt_intid: 0,
            gic_unsupported: false,
            pci_ecam_base: 0,
            pci_ecam_size: 0,
            pci_io_cpu_base: 0,
            pci_io_port_base: 0,
            pci_io_size: 0,
            rtc_base: 0,
            memory_regions: Vec::new(),
        }
    }
}
