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

//! The single in-kernel path that turns a PCI BAR slice into a
//! mapping in a capsule address space. Five steps in order:
//!
//!   1. resolve the caller's claim and verify its epoch is fresh
//!   2. resolve the device record and the requested BAR
//!   3. validate alignment and BAR containment of the request
//!   4. reserve a user VA window in the per-capsule MMIO region
//!      and install pages with user / read+write / uncached / NX
//!   5. record the grant so revocation can find and undo it
//!
//! On any rejection no mapping is installed and no record is made.

use core::cmp::Ordering;

use super::msix_exclusion;
use super::types::{MmioMapError, MmioMapRequest, MmioMapResult};
use crate::hardware::broker::claim;
use crate::hardware::broker::device::BAR_KIND_MMIO;
use crate::hardware::broker::grant::{self, MmioGrant, USER_MMIO_BASE, USER_MMIO_END};
use crate::hardware::broker::pci_index;
use crate::hardware::broker::table;
use crate::memory::addr::PhysAddr;

const PAGE_SIZE: u64 = 4096;
const PAGE_MASK: u64 = PAGE_SIZE - 1;
const FLAGS_KNOWN: u32 = 0;

pub fn map_for_caller(pid: u32, req: MmioMapRequest) -> Result<MmioMapResult, MmioMapError> {
    crate::sys::serial::println(b"[MMIO] claim");
    if req.flags & !FLAGS_KNOWN != 0 {
        return Err(MmioMapError::UnsupportedFlags);
    }
    if req.length == 0 {
        return Err(MmioMapError::ZeroLength);
    }
    if req.offset & PAGE_MASK != 0 || req.length & PAGE_MASK != 0 {
        return Err(MmioMapError::BadAlignment);
    }
    let claim = claim::lookup(req.device_id).ok_or(MmioMapError::NotClaimed)?;
    if claim.pid != pid {
        return Err(MmioMapError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(MmioMapError::StaleEpoch);
    }
    crate::sys::serial::println(b"[MMIO] device");
    let device = table::list()
        .into_iter()
        .find(|r| r.device_id == req.device_id)
        .ok_or(MmioMapError::UnknownDevice)?;
    let bar_idx = req.bar_index as usize;
    if bar_idx >= device.bars.len() || bar_idx >= device.bar_count as usize {
        return Err(MmioMapError::BadBarIndex);
    }
    let bar = device.bars[bar_idx];
    if bar.kind != BAR_KIND_MMIO {
        return Err(MmioMapError::NotMmioBar);
    }
    if bar.base & PAGE_MASK != 0 {
        return Err(MmioMapError::BadAlignment);
    }
    let phys_start = bar.base.checked_add(req.offset).ok_or(MmioMapError::Overflow)?;
    let phys_end = phys_start.checked_add(req.length).ok_or(MmioMapError::Overflow)?;
    let bar_end = bar.base.checked_add(bar.size).ok_or(MmioMapError::Overflow)?;
    if let Ordering::Greater = phys_end.cmp(&bar_end) {
        return Err(MmioMapError::BadRange);
    }
    crate::sys::serial::println(b"[MMIO] reserve");
    let msix = pci_index::lookup(req.device_id).and_then(|h| h.msix);
    crate::sys::serial::println(b"[MMIO] msix");
    msix_exclusion::validate(msix.as_ref(), req.bar_index, req.offset, req.length)?;
    crate::sys::serial::println(b"[MMIO] msix ok");
    let pages = req.length / PAGE_SIZE;
    let user_va = grant::reserve_user_va(pages).ok_or(MmioMapError::NoVaSpace)?;
    crate::sys::serial::println(b"[MMIO] va");
    let user_va_end = user_va.as_u64().checked_add(req.length).ok_or(MmioMapError::Overflow)?;
    if user_va.as_u64() < USER_MMIO_BASE || user_va_end > USER_MMIO_END {
        return Err(MmioMapError::NoVaSpace);
    }
    crate::sys::serial::println(b"[MMIO] map");
    if crate::memory::paging::map_user_mmio(user_va, PhysAddr::new(phys_start), req.length as usize)
        .is_err()
    {
        return Err(MmioMapError::MapFailed);
    }
    crate::sys::serial::println(b"[MMIO] record");
    let grant_id = grant::allocate_id();
    grant::insert(MmioGrant {
        grant_id,
        pid,
        device_id: req.device_id,
        claim_epoch: claim.epoch,
        bar_index: req.bar_index,
        physical_start: phys_start,
        user_va: user_va.as_u64(),
        length: req.length,
        flags: req.flags,
    });
    Ok(MmioMapResult { user_va: user_va.as_u64(), length: req.length, grant_id })
}
