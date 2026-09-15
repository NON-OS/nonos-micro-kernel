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

//! Probing the remapping units and turning DMA remapping on.
//!
//! Here rather than in `init_core_systems`, which is where it used to sit.
//! Reaching a unit means mapping its register window, the MMIO mapper needs
//! the paging manager, and the paging manager does not exist until
//! `init_vm_and_protection` has run. `init_core_systems` returns long before
//! that, so the probe reported `register window not mappable` on every boot
//! and the bring-up behind it never executed at all.
//!
//! That was invisible because the only lane that presents an IOMMU asserted
//! nothing about its own log, and every other lane gives QEMU no IOMMU, so
//! `no remapping units in DMAR; DMA is unrestricted` was the expected output
//! everywhere and a failed probe read the same as a machine without one.
//!
//! The two ordering facts it does depend on both hold here: ACPI publishes
//! the DRHD bases during `init_core_systems`, and PCI is enumerated there
//! too, so the devices a unit has to account for are already known.

/// Report what the firmware declared, then confine DMA to it.
pub(super) fn init_dma_protection() {
    #[cfg(all(target_arch = "x86_64", feature = "nonos-arch-iommu"))]
    {
        crate::arch::x86_64::iommu::unit::report::init();
        crate::arch::x86_64::iommu::unit::bringup::init();
    }
}
