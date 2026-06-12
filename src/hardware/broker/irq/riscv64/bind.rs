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

use crate::arch::riscv64::plic;
use crate::hardware::broker::claim;
use crate::hardware::broker::irq::types::{IrqBindError, IrqBindRequest, IrqBindResult};
use crate::hardware::broker::table;

use super::pending;
use super::trampoline;

// PLIC source 0 is reserved by spec; max is platform-dependent but
// the kernel's registry cap is 1023.
const SRC_MIN: u32 = 1;
const SRC_MAX: u32 = 1023;
// Default priority. PLIC threshold for the owning hart is set to 0
// at boot, so any non-zero priority delivers.
const DEFAULT_PRIORITY: u32 = 1;

pub fn bind(pid: u32, req: IrqBindRequest) -> Result<IrqBindResult, IrqBindError> {
    if req.flags != 0 {
        return Err(IrqBindError::UnsupportedFlags);
    }
    if req.vector_count != 0 {
        return Err(IrqBindError::BadVectorCount);
    }
    if req.irq_source < SRC_MIN || req.irq_source > SRC_MAX {
        return Err(IrqBindError::NotDeviceIrq);
    }

    // ACLINT-only boards and any boot path that never published a
    // PLIC base refuse here before any state mutates.
    if !plic::plic_present() {
        return Err(IrqBindError::PlatformError);
    }

    let claim = claim::lookup(req.device_id).ok_or(IrqBindError::NotClaimed)?;
    if claim.pid != pid {
        return Err(IrqBindError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(IrqBindError::StaleEpoch);
    }

    // Capsules don't get to invent intids. The broker-published
    // `irq_source` on the device record is the only legal value.
    let dev = table::list()
        .into_iter()
        .find(|r| r.device_id == req.device_id)
        .ok_or(IrqBindError::UnknownDevice)?;
    if dev.irq_source == 0 {
        return Err(IrqBindError::NotDeviceIrq);
    }
    if dev.irq_source != req.irq_source {
        return Err(IrqBindError::NotDeviceIrq);
    }

    let (_idx, grant_id) = pending::alloc(req.irq_source, pid, req.device_id, claim.epoch)
        .ok_or(IrqBindError::AlreadyBound)?;
    let Some(e) = pending::find_by_grant(grant_id) else {
        return Err(IrqBindError::PlatformError);
    };

    // Capsule-scoped claim. Owner CAS fails closed if the source is
    // kernel-owned; roll back the pending slot.
    if plic::register_irq_handler_for_capsule(req.irq_source, trampoline::handle).is_err() {
        e.source.store(0, core::sync::atomic::Ordering::Release);
        return Err(IrqBindError::PlatformError);
    }
    plic::set_priority(req.irq_source, DEFAULT_PRIORITY)
        .map_err(|_| IrqBindError::PlatformError)?;
    plic::enable_irq(req.irq_source).map_err(|_| IrqBindError::PlatformError)?;

    Ok(IrqBindResult { grant_id, vector: 0 })
}
