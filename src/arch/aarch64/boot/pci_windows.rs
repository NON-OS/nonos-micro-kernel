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

//! The board's PCI windows, kept so the accessors can be pointed at the copies
//! that outlive the low half of the address space.

use core::sync::atomic::{AtomicU64, Ordering};

static ECAM_BASE: AtomicU64 = AtomicU64::new(0);
static ECAM_SIZE: AtomicU64 = AtomicU64::new(0);
static IO_BASE: AtomicU64 = AtomicU64::new(0);
static IO_PORT_BASE: AtomicU64 = AtomicU64::new(0);
static IO_SIZE: AtomicU64 = AtomicU64::new(0);

/// Publish the windows at their identity mapped addresses and remember the
/// physical bases, which is all `remap` has to go on: `BootInfo` lives in the
/// entry path's frame and is gone by the time it runs.
pub fn publish(info: &super::BootInfo) {
    ECAM_BASE.store(info.pci_ecam_base, Ordering::Relaxed);
    ECAM_SIZE.store(info.pci_ecam_size, Ordering::Relaxed);
    IO_BASE.store(info.pci_io_cpu_base, Ordering::Relaxed);
    IO_PORT_BASE.store(info.pci_io_port_base, Ordering::Relaxed);
    IO_SIZE.store(info.pci_io_size, Ordering::Release);

    crate::drivers::pci::set_ecam_window(info.pci_ecam_base, info.pci_ecam_size);
    crate::arch::port_io::set_io_window(
        info.pci_io_cpu_base,
        info.pci_io_port_base,
        info.pci_io_size,
    );
}

/// Point both accessors at the kernel half copies of their windows.
///
/// Runs once the unified address space is up, which is the point the low half
/// is cleared and the addresses published above stop resolving. The boot map
/// already placed both windows in the kernel half at the direct map's address
/// for the same physical page, so this is arithmetic rather than a new
/// mapping. Each window is published only once the hardware confirms it
/// translates, so a board whose kernel half does not describe it loses PCI
/// rather than the boot.
pub fn remap() {
    let ecam_size = ECAM_SIZE.load(Ordering::Acquire);
    if ecam_size > 0 {
        let va = high(ECAM_BASE.load(Ordering::Relaxed));
        if reachable(va) {
            crate::drivers::pci::set_ecam_window(va, ecam_size);
        } else {
            // Disabled rather than left pointing where the low half used to
            // answer. A config read then returns all ones, which is what the
            // bus gives for a slot nothing implements, so enumeration comes up
            // empty instead of taking a fault it cannot recover from.
            crate::drivers::pci::set_ecam_window(0, 0);
        }
    }

    let io_size = IO_SIZE.load(Ordering::Acquire);
    if io_size > 0 {
        let va = high(IO_BASE.load(Ordering::Relaxed));
        if reachable(va) {
            crate::arch::port_io::set_io_window(va, IO_PORT_BASE.load(Ordering::Relaxed), io_size);
        } else {
            crate::arch::port_io::set_io_window(0, 0, 0);
        }
    }
}

/// Whether the MMU can translate `va` for a privileged read right now.
///
/// `AT` puts the question to the translation hardware and leaves the answer in
/// `PAR_EL1`, which settles it without dereferencing an address that may not
/// resolve. Publishing a window unchecked is what turns a missing mapping into
/// a fatal trap in the middle of enumeration.
fn reachable(va: u64) -> bool {
    crate::arch::aarch64::mmu::translation::translate_stage1_read(va).is_ok()
}

fn high(phys: u64) -> u64 {
    phys.wrapping_add(crate::arch::aarch64::mmu::KERNEL_SPACE_START)
}
