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

//! Slow-path IRQ grant records keyed by `grant_id`. Syscalls and
//! revocation paths walk this list under a single mutex; the
//! hard-IRQ dispatcher does not consult it.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

use super::types::{IrqError, IrqGrant, IrqGrantKind};

static RECORDS: Mutex<Vec<IrqGrant>> = Mutex::new(Vec::new());
static NEXT_GRANT_ID: AtomicU64 = AtomicU64::new(1);

pub(super) fn allocate_id() -> u64 {
    NEXT_GRANT_ID.fetch_add(1, Ordering::SeqCst)
}

// Reserve `count` consecutive grant IDs and return the base. Used
// by the MSI-X bind path so the capsule can derive per-vector grants
// as `base + i` without an extra round-trip per vector.
pub(super) fn allocate_id_run(count: u64) -> u64 {
    NEXT_GRANT_ID.fetch_add(count, Ordering::SeqCst)
}

pub(super) fn insert(record: IrqGrant) {
    RECORDS.lock().push(record);
}

pub(super) fn insert_many(records_in: &[IrqGrant]) {
    let mut all = RECORDS.lock();
    all.extend_from_slice(records_in);
}

// True if the (pid, device) pair already owns at least one MSI-X
// grant. The bind path uses this to keep MSI-X bindings all-or-
// nothing per device — the kernel either programs the full N-vector
// run or none of it.
pub(super) fn has_msix_grant_for(pid: u32, device_id: u64) -> bool {
    RECORDS
        .lock()
        .iter()
        .any(|g| g.pid == pid && g.device_id == device_id && g.kind == IrqGrantKind::Msix)
}

pub(super) fn count_msix_for_device(device_id: u64) -> usize {
    RECORDS
        .lock()
        .iter()
        .filter(|g| g.device_id == device_id && g.kind == IrqGrantKind::Msix)
        .count()
}

pub(super) fn vectors_for_pid(pid: u32) -> Vec<u8> {
    RECORDS.lock().iter().filter(|g| g.pid == pid).map(|g| g.vector).collect()
}

pub(super) fn lookup(grant_id: u64) -> Option<IrqGrant> {
    RECORDS.lock().iter().find(|g| g.grant_id == grant_id).copied()
}

pub(super) fn vector_for_gsi(gsi: u32) -> Option<u8> {
    RECORDS.lock().iter().find(|g| g.irq_source == gsi).map(|g| g.vector)
}

pub(super) fn remove(pid: u32, grant_id: u64) -> Result<IrqGrant, IrqError> {
    let mut all = RECORDS.lock();
    let idx = all.iter().position(|g| g.grant_id == grant_id).ok_or(IrqError::UnknownGrant)?;
    if all[idx].pid != pid {
        return Err(IrqError::NotHolder);
    }
    Ok(all.remove(idx))
}

pub(super) fn drain_for_pid(pid: u32) -> Vec<IrqGrant> {
    let mut all = RECORDS.lock();
    let mut taken = Vec::new();
    all.retain(|g| {
        if g.pid == pid {
            taken.push(*g);
            false
        } else {
            true
        }
    });
    taken
}

pub(super) fn drain_for_device(pid: u32, device_id: u64) -> Vec<IrqGrant> {
    let mut all = RECORDS.lock();
    let mut taken = Vec::new();
    all.retain(|g| {
        if g.pid == pid && g.device_id == device_id {
            taken.push(*g);
            false
        } else {
            true
        }
    });
    taken
}
