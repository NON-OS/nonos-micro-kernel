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
use crate::hardware::broker::irq::types::IrqError;

use super::pending;

// Re-arm the PLIC source the trampoline disabled. The outer trap
// path already did `complete_interrupt` for the kernel-side handoff;
// `ack_grant` is the userland-side re-arm.
pub fn ack_grant(pid: u32, grant_id: u64) -> Result<(), IrqError> {
    let e = pending::find_by_grant(grant_id).ok_or(IrqError::UnknownGrant)?;
    if e.pid.load(Ordering::Acquire) != pid {
        return Err(IrqError::NotHolder);
    }
    let source = e.source.load(Ordering::Acquire);
    if source == 0 {
        return Err(IrqError::UnknownGrant);
    }
    plic::enable_irq(source).map_err(|_| IrqError::PlatformError)?;
    Ok(())
}
