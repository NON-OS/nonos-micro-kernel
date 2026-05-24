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

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

#[derive(Debug, Clone, Copy)]
pub struct Claim {
    pub pid: u32,
    pub device_id: u64,
    pub epoch: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimError {
    UnknownDevice,
    AlreadyClaimed,
    NotHolder,
    NotClaimed,
}

static CLAIMS: Mutex<Vec<Claim>> = Mutex::new(Vec::new());
static EPOCH: AtomicU64 = AtomicU64::new(1);

fn next_epoch() -> u64 {
    EPOCH.fetch_add(1, Ordering::SeqCst)
}

// Register a claim. Returns the granted epoch on success. The caller
// must have already verified that `device_id` exists in the broker
// table.
pub fn claim(pid: u32, device_id: u64) -> Result<u64, ClaimError> {
    let mut claims = CLAIMS.lock();
    if claims.iter().any(|c| c.device_id == device_id) {
        return Err(ClaimError::AlreadyClaimed);
    }
    let epoch = next_epoch();
    claims.push(Claim { pid, device_id, epoch });
    Ok(epoch)
}

// Release a claim held by `pid`. Returns the released epoch on
// success.
pub fn release(pid: u32, device_id: u64) -> Result<u64, ClaimError> {
    let mut claims = CLAIMS.lock();
    let idx = claims.iter().position(|c| c.device_id == device_id).ok_or(ClaimError::NotClaimed)?;
    if claims[idx].pid != pid {
        return Err(ClaimError::NotHolder);
    }
    let epoch = claims[idx].epoch;
    claims.remove(idx);
    Ok(epoch)
}

// Release every claim held by `pid`. Called from the kernel's
// `MkExit` path so a dying capsule cannot leak grants. Returns the
// number of claims revoked.
pub fn release_all_for_pid(pid: u32) -> usize {
    let mut claims = CLAIMS.lock();
    let before = claims.len();
    claims.retain(|c| c.pid != pid);
    before - claims.len()
}

pub fn lookup(device_id: u64) -> Option<Claim> {
    CLAIMS.lock().iter().find(|c| c.device_id == device_id).copied()
}

#[cfg(test)]
pub(crate) fn snapshot() -> Vec<Claim> {
    CLAIMS.lock().clone()
}

#[cfg(test)]
pub(crate) fn reset_for_test() {
    CLAIMS.lock().clear();
    EPOCH.store(1, Ordering::SeqCst);
}

#[cfg(test)]
pub(crate) fn install_for_test(pid: u32, device_id: u64) -> u64 {
    let mut claims = CLAIMS.lock();
    let epoch = EPOCH.fetch_add(1, Ordering::SeqCst);
    claims.push(Claim { pid, device_id, epoch });
    epoch
}
