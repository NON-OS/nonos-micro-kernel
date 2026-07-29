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

use super::super::records;
use super::super::slots;
use super::super::types::{IrqBindError, IrqBindRequest, IrqBindResult, IrqGrant, IrqGrantKind};
use super::super::validate::validate_intx_request;
use crate::arch::interrupt::broker::vector_of;
use crate::arch::interrupt::ioapic;
use crate::hardware::broker::table;

pub(super) fn bind_intx(
    pid: u32,
    req: IrqBindRequest,
    epoch: u64,
) -> Result<IrqBindResult, IrqBindError> {
    let device = table::list()
        .into_iter()
        .find(|r| r.device_id == req.device_id)
        .ok_or(IrqBindError::UnknownDevice)?;
    validate_intx_request(&req, device.irq_pin, device.irq_line)?;
    // The driver names its line by ISA IRQ number; the IOAPIC is programmed
    // by GSI. Resolve the MADT override once (identity when none) and route,
    // mask and record everything by the resolved GSI so ack/unmask later
    // operate on the same redirection entry.
    let gsi = ioapic::gsi_for_irq(req.irq_source);
    if records::vector_for_gsi(gsi).is_some() {
        return Err(IrqBindError::AlreadyBound);
    }

    let slot = slots::try_alloc_slot().ok_or(IrqBindError::NoVector)?;
    let vector = vector_of(slot).ok_or(IrqBindError::NoVector)?;
    let dest_apic_id = crate::arch::interrupt_controller::local_id();

    if ioapic::program_route_external(gsi, vector, dest_apic_id).is_err() {
        slots::free_slot(slot);
        return Err(IrqBindError::PlatformError);
    }
    let _ = ioapic::mask(gsi, true);

    let grant_id = records::allocate_id();
    records::insert(IrqGrant {
        grant_id,
        pid,
        device_id: req.device_id,
        claim_epoch: epoch,
        irq_source: gsi,
        vector,
        flags: req.flags,
        kind: IrqGrantKind::Intx,
        device_vector: 0,
    });
    slots::activate(slot, grant_id, gsi);

    Ok(IrqBindResult { grant_id, vector })
}
