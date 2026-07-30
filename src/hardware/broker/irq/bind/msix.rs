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

use super::super::msix_ops::current_ops;
use super::super::records;
use super::super::slots;
use super::super::types::{IrqBindError, IrqBindRequest, IrqBindResult, IrqGrant, IrqGrantKind};
use super::super::validate::validate_msix_request;
use super::handle_view::handle_view;
use crate::arch::interrupt::broker::{vector_of, BROKER_VEC_COUNT};
use crate::hardware::broker::pci_index;

pub(super) fn bind_msix(
    pid: u32,
    req: IrqBindRequest,
    epoch: u64,
) -> Result<IrqBindResult, IrqBindError> {
    let handle = pci_index::lookup(req.device_id);
    let view = handle.as_ref().map(handle_view);
    validate_msix_request(
        &req,
        BROKER_VEC_COUNT,
        view.as_ref(),
        records::has_msix_grant_for(pid, req.device_id),
    )?;
    // validate_msix_request already rejects a None handle / no-MSI-X device;
    // these guards keep a future change to the validator from turning a
    // broken invariant into a kernel panic on the trusted hardware path.
    // Resolve both before allocating vectors so no error path leaks a slot.
    let handle = handle.ok_or(IrqBindError::NoDeviceHandle)?;
    let msix = handle.msix.ok_or(IrqBindError::NoMsixCap)?;

    let n = req.vector_count as usize;
    let base_slot = slots::try_alloc_contiguous(n).ok_or(IrqBindError::NoVector)?;
    let base_vector = vector_of(base_slot).ok_or(IrqBindError::NoVector)?;
    let dest_apic_id = crate::arch::interrupt_controller::local_id() as u8;
    if let Err(e) = current_ops().program_run(
        &handle.address,
        &msix,
        &handle.bars,
        base_vector,
        n,
        dest_apic_id,
    ) {
        slots::free_contiguous(base_slot, n);
        return Err(e);
    }

    let base_grant = records::allocate_id_run(n as u64);
    let mut new_records: Vec<IrqGrant> = Vec::with_capacity(n);
    for i in 0..n {
        new_records.push(IrqGrant {
            grant_id: base_grant + i as u64,
            pid,
            device_id: req.device_id,
            claim_epoch: epoch,
            irq_source: 0,
            vector: base_vector + i as u8,
            flags: req.flags,
            kind: IrqGrantKind::Msix,
            device_vector: i as u16,
        });
    }
    records::insert_many(&new_records);

    for i in 0..n {
        slots::activate(base_slot + i, base_grant + i as u64, 0);
    }

    Ok(IrqBindResult { grant_id: base_grant, vector: base_vector })
}
