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

use core::sync::atomic::Ordering;

use crate::arch::riscv64::plic;
use crate::arch::riscv64::plic::irq_handlers;
use crate::hardware::broker::irq::types::IrqError;

use super::pending::{Entry, SLOTS};

// Disable line, drop capsule ownership at the PLIC registry, then
// free the broker slot. `unregister_for_capsule` is owner-checked so
// a kernel-claimed source can never be released through this path.
fn tear_down(e: &'static Entry) {
    let source = e.source.load(Ordering::Acquire);
    if source != 0 {
        if plic::disable_irq(source).is_err() {
            return;
        }
        if irq_handlers::unregister_for_capsule(source).is_err() {
            return;
        }
    }
    e.source.store(0, Ordering::Release);
    e.pid.store(0, Ordering::Release);
    e.grant_id.store(0, Ordering::Release);
}

pub fn unmap_grant(pid: u32, grant_id: u64) -> Result<(), IrqError> {
    let e = super::pending::find_by_grant(grant_id).ok_or(IrqError::UnknownGrant)?;
    if e.pid.load(Ordering::Acquire) != pid {
        return Err(IrqError::NotHolder);
    }
    tear_down(e);
    Ok(())
}

pub fn release_for_device(pid: u32, device_id: u64) -> usize {
    let mut n = 0;
    for e in SLOTS.iter() {
        if e.source.load(Ordering::Acquire) == 0 {
            continue;
        }
        if e.pid.load(Ordering::Acquire) == pid && e.device_id.load(Ordering::Acquire) == device_id
        {
            tear_down(e);
            n += 1;
        }
    }
    n
}

pub fn release_all_for_pid(pid: u32) -> usize {
    let mut n = 0;
    for e in SLOTS.iter() {
        if e.source.load(Ordering::Acquire) == 0 {
            continue;
        }
        if e.pid.load(Ordering::Acquire) == pid {
            tear_down(e);
            n += 1;
        }
    }
    n
}
