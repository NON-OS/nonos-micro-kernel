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

use crate::arch::x86_64::iommu::device as vtd_device;
use crate::arch::x86_64::iommu::domain as vtd_domain;
use crate::arch::x86_64::iommu::domain::DomainId as VtdDomainId;
use crate::arch::x86_64::iommu::globals as vtd_globals;
use crate::arch::x86_64::iommu::mapping as vtd_mapping;
use crate::arch::x86_64::iommu::types::IommuPageFlags;
use crate::memory::addr::PhysAddr;

use super::super::device::DeviceAddress;
use super::super::domain_id::DomainId;
use super::super::error::IommuError;
use super::super::protection::IommuProtection;

pub(crate) fn allocate_domain() -> Result<DomainId, IommuError> {
    let raw_id = vtd_globals::allocate_domain_id();
    if raw_id > u16::MAX as u64 {
        return Err(IommuError::DomainExhausted);
    }
    let vtd_id = VtdDomainId::new(raw_id as u16);
    vtd_domain::create_domain(vtd_id).map_err(|_| IommuError::DomainExhausted)?;
    Ok(DomainId::new(raw_id as u16))
}

pub(crate) fn free_domain(id: DomainId) -> Result<(), IommuError> {
    vtd_domain::destroy_domain(VtdDomainId::new(id.as_u16())).map_err(|_| IommuError::InvalidDomain)
}

pub(crate) fn map(
    domain: DomainId,
    iova: u64,
    phys: PhysAddr,
    size: usize,
    protection: IommuProtection,
) -> Result<(), IommuError> {
    // LIMIT: snoop is asserted unconditionally. The contract has no
    // per-mapping snoop control yet; revisit when a non-snooping
    // consumer (e.g., GPU scratch surfaces) needs the choice.
    let flags = IommuPageFlags {
        read: protection.read,
        write: protection.write,
        execute: false,
        user: false,
        snoop: true,
    };
    vtd_mapping::map_range(VtdDomainId::new(domain.as_u16()), iova, phys.as_u64(), size, flags)
        .map_err(|_| IommuError::BackendFault)
}

pub(crate) fn unmap(domain: DomainId, iova: u64, size: usize) -> Result<(), IommuError> {
    vtd_mapping::unmap_range(VtdDomainId::new(domain.as_u16()), iova, size)
        .map_err(|_| IommuError::NotMapped)
}

pub(crate) fn attach_device(domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    vtd_device::map_device(
        VtdDomainId::new(domain.as_u16()),
        device.pci_bus(),
        device.pci_device(),
        device.pci_function(),
    )
    .map_err(|_| IommuError::DeviceAttachFailed)
}

pub(crate) fn detach_device(_domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    vtd_device::unmap_device(device.pci_bus(), device.pci_device(), device.pci_function())
        .map_err(|_| IommuError::DeviceDetachFailed)
}
