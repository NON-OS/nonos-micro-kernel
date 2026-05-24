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

use crate::memory::addr::PhysAddr;

use super::super::device::DeviceAddress;
use super::super::domain_id::DomainId;
use super::super::error::IommuError;
use super::super::protection::IommuProtection;

pub(in crate::memory::iommu) fn allocate_domain() -> Result<DomainId, IommuError> {
    Err(IommuError::NotSupported)
}

pub(in crate::memory::iommu) fn free_domain(_id: DomainId) -> Result<(), IommuError> {
    Err(IommuError::NotSupported)
}

pub(in crate::memory::iommu) fn map(
    _domain: DomainId,
    _iova: u64,
    _phys: PhysAddr,
    _size: usize,
    _protection: IommuProtection,
) -> Result<(), IommuError> {
    Err(IommuError::NotSupported)
}

pub(in crate::memory::iommu) fn unmap(
    _domain: DomainId,
    _iova: u64,
    _size: usize,
) -> Result<(), IommuError> {
    Err(IommuError::NotSupported)
}

pub(in crate::memory::iommu) fn attach_device(
    _domain: DomainId,
    _device: DeviceAddress,
) -> Result<(), IommuError> {
    Err(IommuError::NotSupported)
}

pub(in crate::memory::iommu) fn detach_device(
    _domain: DomainId,
    _device: DeviceAddress,
) -> Result<(), IommuError> {
    Err(IommuError::NotSupported)
}
