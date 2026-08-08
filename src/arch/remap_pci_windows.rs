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

//! Re-points the PCI windows at kernel-half mappings once paging is up.
//!
//! The boot map identity maps them so enumeration can run early, and
//! `init_unified_vm` then clears the low half, which takes those mappings with
//! it. Anything that touches config space afterwards faults on the bare
//! physical address unless the window is republished through the MMIO mapper.

/// Remap the platform's PCI config and I/O windows into the kernel half.
///
/// A PC reaches config space through the 0xCF8/0xCFC port pair rather than a
/// memory window, so there is nothing to remap there.
#[cfg(not(target_arch = "aarch64"))]
pub(crate) fn remap_pci_windows() {}

#[cfg(target_arch = "aarch64")]
pub(crate) fn remap_pci_windows() {
    crate::arch::aarch64::boot::remap_pci_windows();
}
