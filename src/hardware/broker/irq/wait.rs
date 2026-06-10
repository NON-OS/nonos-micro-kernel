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

#![cfg(target_arch = "x86_64")]

//! Syscall-context half of `MkIrqWait`. Arms the per-slot waiter
//! word the hard-IRQ dispatcher consumes, and reports the combined
//! sequence counter so the caller can detect deliveries that beat
//! the arm. `grant_id == 0` waits on every grant the pid owns —
//! drivers with several lines (e.g. i8042 IRQ1 + IRQ12) need one
//! wait covering all of them. Arm-before-read ordering plus IF=0
//! syscall sections make the subsequent sleep race-free: a vector
//! can only fire after the context switch re-enables interrupts,
//! by which time the waiter pid is parked and wakeable.

extern crate alloc;

use alloc::vec::Vec;

use super::records;
use super::slots;
use super::types::IrqError;
use crate::arch::interrupt::broker::slot_of;

fn owned_slots(pid: u32, grant_id: u64) -> Result<Vec<usize>, IrqError> {
    if grant_id != 0 {
        let g = records::lookup(grant_id).ok_or(IrqError::UnknownGrant)?;
        if g.pid != pid {
            return Err(IrqError::NotHolder);
        }
        let idx = slot_of(g.vector).ok_or(IrqError::UnknownGrant)?;
        return Ok(alloc::vec![idx]);
    }
    let idxs: Vec<usize> =
        records::vectors_for_pid(pid).into_iter().filter_map(slot_of).collect();
    if idxs.is_empty() {
        return Err(IrqError::UnknownGrant);
    }
    Ok(idxs)
}

fn combined_seq(idxs: &[usize]) -> u64 {
    idxs.iter().fold(0u64, |acc, &i| acc.wrapping_add(slots::read_counters(i).0))
}

pub fn wait_arm(pid: u32, grant_id: u64) -> Result<u64, IrqError> {
    let idxs = owned_slots(pid, grant_id)?;
    for &i in &idxs {
        slots::arm_waiter(i, pid);
    }
    Ok(combined_seq(&idxs))
}

pub fn wait_disarm(pid: u32, grant_id: u64) -> Result<u64, IrqError> {
    let idxs = owned_slots(pid, grant_id)?;
    for &i in &idxs {
        slots::disarm_waiter(i, pid);
    }
    Ok(combined_seq(&idxs))
}
