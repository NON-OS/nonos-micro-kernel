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

//! Slow-path DMA grant table. One mutex-protected vector keyed by
//! `grant_id`; revocation paths drain by pid or by device.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

use super::types::{DmaError, DmaGrant};

static RECORDS: Mutex<Vec<DmaGrant>> = Mutex::new(Vec::new());
static NEXT_GRANT_ID: AtomicU64 = AtomicU64::new(1);

pub(super) fn allocate_id() -> u64 {
    NEXT_GRANT_ID.fetch_add(1, Ordering::SeqCst)
}

pub(super) fn insert(record: DmaGrant) {
    RECORDS.lock().push(record);
}

pub(super) fn remove(pid: u32, grant_id: u64) -> Result<DmaGrant, DmaError> {
    let mut all = RECORDS.lock();
    let idx = all.iter().position(|g| g.grant_id == grant_id).ok_or(DmaError::UnknownGrant)?;
    if all[idx].pid != pid {
        return Err(DmaError::NotHolder);
    }
    Ok(all.remove(idx))
}

pub(super) fn drain_for_pid(pid: u32) -> Vec<DmaGrant> {
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

pub(super) fn drain_for_device(pid: u32, device_id: u64) -> Vec<DmaGrant> {
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
