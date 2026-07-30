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

//! PIO grant table.
//!
//! Records every (port_base, port_count) window the broker has
//! handed to a capsule. Reads and writes lookup against this
//! table; a missing or pid-mismatched grant rejects the access.
//! Revocation drains the table on `MkPioRelease`, on
//! `MkDeviceRelease`, and on capsule exit.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

use super::types::PioError;

#[derive(Debug, Clone, Copy)]
pub(super) struct PioGrant {
    pub grant_id: u64,
    pub pid: u32,
    pub device_id: u64,
    pub claim_epoch: u64,
    pub port_base: u16,
    pub port_count: u16,
}

static GRANTS: Mutex<Vec<PioGrant>> = Mutex::new(Vec::new());
static NEXT_GRANT_ID: AtomicU64 = AtomicU64::new(1);

pub(super) fn allocate_id() -> u64 {
    NEXT_GRANT_ID.fetch_add(1, Ordering::SeqCst)
}

pub(super) fn insert(record: PioGrant) {
    GRANTS.lock().push(record);
}

pub(super) fn lookup_for_holder(pid: u32, grant_id: u64) -> Result<PioGrant, PioError> {
    let grants = GRANTS.lock();
    let g =
        grants.iter().find(|g| g.grant_id == grant_id).copied().ok_or(PioError::UnknownGrant)?;
    if g.pid != pid {
        return Err(PioError::NotHolder);
    }
    Ok(g)
}

pub(super) fn remove(pid: u32, grant_id: u64) -> Result<PioGrant, PioError> {
    let mut grants = GRANTS.lock();
    let idx = grants.iter().position(|g| g.grant_id == grant_id).ok_or(PioError::UnknownGrant)?;
    if grants[idx].pid != pid {
        return Err(PioError::NotHolder);
    }
    Ok(grants.remove(idx))
}

pub(super) fn drain_for_pid(pid: u32) -> Vec<PioGrant> {
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

pub(super) fn drain_for_device(pid: u32, device_id: u64) -> Vec<PioGrant> {
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
