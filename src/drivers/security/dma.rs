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

use super::constants::*;
use super::error::DriverError;
use crate::memory::addr::PhysAddr;

pub fn validate_dma_buffer(phys_addr: PhysAddr, size: usize) -> Result<(), DriverError> {
    let addr = phys_addr.as_u64();

    if size == 0 {
        return Err(DriverError::InvalidDmaBuffer);
    }

    if size > MAX_DMA_SIZE {
        return Err(DriverError::InvalidDmaBuffer);
    }

    let end = addr.checked_add(size as u64).ok_or(DriverError::InvalidDmaBuffer)?;

    if end <= addr {
        return Err(DriverError::InvalidDmaBuffer);
    }

    if addr % PAGE_SIZE as u64 != 0 {
        return Err(DriverError::InvalidDmaBuffer);
    }

    if addr < KERNEL_PHYS_END {
        return Err(DriverError::InvalidDmaBuffer);
    }

    Ok(())
}

pub fn validate_prp_list(prp_list: &[u64], expected_size: usize) -> Result<(), DriverError> {
    if prp_list.is_empty() {
        return Err(DriverError::InvalidPrpList);
    }

    if prp_list.len() > MAX_PRP_ENTRIES {
        return Err(DriverError::InvalidPrpList);
    }

    let pages_needed = (expected_size + PAGE_SIZE - 1) / PAGE_SIZE;

    if prp_list.len() < pages_needed {
        return Err(DriverError::InvalidPrpList);
    }

    for (i, &prp) in prp_list.iter().enumerate() {
        if prp == 0 {
            return Err(DriverError::InvalidPrpList);
        }

        if i > 0 && (prp % PAGE_SIZE as u64 != 0) {
            return Err(DriverError::InvalidPrpList);
        }

        if prp < KERNEL_PHYS_END {
            return Err(DriverError::InvalidPrpList);
        }

        if prp > (1u64 << MAX_PHYS_ADDR_BITS) {
            return Err(DriverError::InvalidPrpList);
        }
    }

    Ok(())
}

pub fn validate_sg_list(
    sg_list: &[(u64, usize)],
    max_entries: usize,
) -> Result<usize, DriverError> {
    if sg_list.is_empty() {
        return Err(DriverError::InvalidDmaBuffer);
    }

    if sg_list.len() > max_entries {
        return Err(DriverError::InvalidDmaBuffer);
    }

    let mut total_size = 0usize;

    for (addr, len) in sg_list {
        validate_dma_buffer(PhysAddr::new(*addr), *len)?;

        total_size = total_size.checked_add(*len).ok_or(DriverError::InvalidDmaBuffer)?;
    }

    if total_size > MAX_DMA_SIZE {
        return Err(DriverError::InvalidDmaBuffer);
    }

    Ok(total_size)
}
