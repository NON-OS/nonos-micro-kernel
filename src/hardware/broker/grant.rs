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

//! MMIO grant table.
//!
//! Records every page-aligned physical range the broker has mapped
//! into a capsule address space. The table is the authority on which
//! pid owns which user MMIO window; revocation paths walk it on
//! `MkDeviceRelease` and on process exit, unmap the user pages, and
//! broadcast TLB invalidations through the per-asid shootdown path.
//!
//! The kernel exposes no other physical-mapping primitive to
//! userland. Bypassing this table means inventing a new one — the
//! static checks make that loud.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

use crate::memory::addr::VirtAddr;

#[derive(Debug, Clone, Copy)]
pub struct MmioGrant {
    pub grant_id: u64,
    pub pid: u32,
    pub device_id: u64,
    pub claim_epoch: u64,
    pub bar_index: u8,
    pub physical_start: u64,
    pub user_va: u64,
    pub length: u64,
    pub flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrantError {
    UnknownGrant,
    NotHolder,
}

static GRANTS: Mutex<Vec<MmioGrant>> = Mutex::new(Vec::new());
static NEXT_GRANT_ID: AtomicU64 = AtomicU64::new(1);

fn next_grant_id() -> u64 {
    NEXT_GRANT_ID.fetch_add(1, Ordering::SeqCst)
}

// Insert a new grant. The caller (broker MMIO map handler) has
// already validated claim ownership, BAR containment and alignment,
// and installed the user pages. This function only persists the
// record so revocation can find it later.
pub fn insert(record: MmioGrant) {
    GRANTS.lock().push(record);
}

// Allocate a fresh grant id without recording yet. Lets the handler
// stamp the id into the record before insertion.
pub fn allocate_id() -> u64 {
    next_grant_id()
}

// Revoke every grant held by `pid`. Returns the list of revoked
// records so the caller can decide whether to unmap user pages
// (self-context) or skip the unmap and let address-space teardown
// drop the PTEs (cross-pid).
pub fn drain_for_pid(pid: u32) -> Vec<MmioGrant> {
    let mut grants = GRANTS.lock();
    let mut taken = Vec::new();
    grants.retain(|g| {
        if g.pid == pid {
            taken.push(*g);
            false
        } else {
            true
        }
    });
    taken
}

// Revoke every grant tied to `device_id`. Used by `MkDeviceRelease`
// when the holder voluntarily gives the device back. Caller passes
// the holder pid for the cross-check; mismatch indicates a bug, not
// a userland error.
pub fn drain_for_device(pid: u32, device_id: u64) -> Vec<MmioGrant> {
    let mut grants = GRANTS.lock();
    let mut taken = Vec::new();
    grants.retain(|g| {
        if g.pid == pid && g.device_id == device_id {
            taken.push(*g);
            false
        } else {
            true
        }
    });
    taken
}

// Lookup a single grant by id. Used by an explicit `MkMmioUnmap`
// from the holder.
// Remove a single grant if `pid` is the holder. Returns the removed
// record on success.
pub fn remove(pid: u32, grant_id: u64) -> Result<MmioGrant, GrantError> {
    let mut grants = GRANTS.lock();
    let idx = grants.iter().position(|g| g.grant_id == grant_id).ok_or(GrantError::UnknownGrant)?;
    if grants[idx].pid != pid {
        return Err(GrantError::NotHolder);
    }
    Ok(grants.remove(idx))
}

// Snapshot for diagnostics and tests. The lock is dropped before the
// caller sees the data.
#[cfg(test)]
pub(crate) fn snapshot() -> Vec<MmioGrant> {
    GRANTS.lock().clone()
}

// User MMIO virtual-address allocator. Each grant gets a fresh
// region inside `[USER_MMIO_BASE, USER_MMIO_END)`; the bumping
// counter is global because each capsule has its own page tables,
// so VA reuse across address spaces is harmless. Within one address
// space the bump never wraps for the life of the run; if it ever
// does, that is a real exhaustion and the request fails.
pub const USER_MMIO_BASE: u64 = 0x0000_0080_0000_0000;
pub const USER_MMIO_END: u64 = 0x0000_0090_0000_0000;
const PAGE_SIZE: u64 = 4096;

static NEXT_USER_MMIO_VA: AtomicU64 = AtomicU64::new(USER_MMIO_BASE);

// Reserve `pages * 4 KiB` of user VA in the MMIO grant region. A
// guard page is added between adjacent grants so a runaway access
// cannot silently spill into the next grant. Returns `None` on
// region exhaustion.
pub fn reserve_user_va(pages: u64) -> Option<VirtAddr> {
    let bytes = pages.checked_mul(PAGE_SIZE)?;
    let with_guard = bytes.checked_add(PAGE_SIZE)?;
    let base = NEXT_USER_MMIO_VA.fetch_add(with_guard, Ordering::SeqCst);
    let end = base.checked_add(bytes)?;
    if end > USER_MMIO_END {
        return None;
    }
    Some(VirtAddr::new(base))
}

#[cfg(test)]
pub(crate) fn reset_for_test() {
    GRANTS.lock().clear();
    NEXT_GRANT_ID.store(1, Ordering::SeqCst);
    NEXT_USER_MMIO_VA.store(USER_MMIO_BASE, Ordering::SeqCst);
}
