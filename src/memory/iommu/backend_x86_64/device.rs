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

//! Binding a PCI function to a domain by writing its context entry.

use crate::arch::x86_64::iommu::device::{map_device, unmap_device};
use crate::arch::x86_64::iommu::domain::DomainId as VtdDomainId;
use crate::memory::iommu::{DeviceAddress, DomainId, IommuError};

use super::enforced;

/// Gated: a context entry written while translation is off changes nothing
/// about where the device's transactions land, so reporting success would be
/// reporting a confinement that does not exist.
pub(crate) fn attach_device(domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    enforced::require()?;
    map_device(
        VtdDomainId::new(domain.as_u16()),
        device.pci_bus(),
        device.pci_device(),
        device.pci_function(),
    )
    .map_err(|_| IommuError::DeviceAttachFailed)
}

pub(crate) fn detach_device(_domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    unmap_device(device.pci_bus(), device.pci_device(), device.pci_function())
        .map_err(|_| IommuError::DeviceDetachFailed)
}
