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

//! Installing and removing IOVA ranges in a domain's second-level tables.

use crate::arch::x86_64::iommu::domain::DomainId as VtdDomainId;
use crate::arch::x86_64::iommu::mapping::{map_range, unmap_range};
use crate::arch::x86_64::iommu::types::IommuPageFlags;
use crate::memory::addr::PhysAddr;
use crate::memory::iommu::{DomainId, IommuError, IommuProtection};

use super::enforced;

pub(crate) fn map(
    domain: DomainId,
    iova: u64,
    phys: PhysAddr,
    size: usize,
    protection: IommuProtection,
) -> Result<(), IommuError> {
    enforced::require()?;
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
    map_range(VtdDomainId::new(domain.as_u16()), iova, phys.as_u64(), size, flags)
        .map_err(|_| IommuError::BackendFault)
}

/// Ungated for the same reason `free_domain` is: the range can only be mapped
/// if `map` was allowed, and refusing to remove it would strand a translation.
pub(crate) fn unmap(domain: DomainId, iova: u64, size: usize) -> Result<(), IommuError> {
    unmap_range(VtdDomainId::new(domain.as_u16()), iova, size).map_err(|_| IommuError::NotMapped)
}
