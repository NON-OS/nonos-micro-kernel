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

// x86-only: IO-APIC redirect (INTx) + PCI MSI-X capability program.
#![cfg(target_arch = "x86_64")]

//! `MkIrqBind` core. Two paths share the same syscall:
//!
//!   * INTx (default, `flags == 0`) — programs the IO-APIC, returns
//!     a single grant. Behaviour is bit-identical to the pre-MSI-X
//!     version of the broker.
//!   * MSI-X (`flags & BIND_MSIX != 0`) — kernel walks the
//!     capability list, validates the table BAR against the
//!     claimed device, allocates `vector_count` contiguous broker
//!     vectors, programs that many MSI-X table entries with the
//!     LAPIC redirect, enables MSI-X, then unmasks each entry. The
//!     capsule never sees the table address and never writes to
//!     it; it only receives the base grant id and base vector.
//!
//! All hardware-touching steps go through the `MsixOps` indirection.

extern crate alloc;

use alloc::vec::Vec;

use super::msix_ops::current_ops;
use super::records;
use super::slots;
use super::types::{
    IrqBindError, IrqBindRequest, IrqBindResult, IrqGrant, IrqGrantKind, BIND_MSIX,
};
use super::validate::{validate_intx_request, validate_msix_request, MsixHandleView};
use crate::arch::interrupt::broker::{vector_of, BROKER_VEC_COUNT};
use crate::arch::interrupt::ioapic;
use crate::drivers::pci::types::PciBar;
use crate::hardware::broker::pci_index::{self, PciHandle};
use crate::hardware::broker::{claim, table};

pub fn bind(pid: u32, req: IrqBindRequest) -> Result<IrqBindResult, IrqBindError> {
    let claim = claim::lookup(req.device_id).ok_or(IrqBindError::NotClaimed)?;
    if claim.pid != pid {
        return Err(IrqBindError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(IrqBindError::StaleEpoch);
    }

    if req.flags & BIND_MSIX != 0 {
        bind_msix(pid, req, claim.epoch)
    } else {
        bind_intx(pid, req, claim.epoch)
    }
}

fn bind_intx(pid: u32, req: IrqBindRequest, epoch: u64) -> Result<IrqBindResult, IrqBindError> {
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
    let dest_apic_id = crate::arch::interrupt::apic::id();

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

fn bind_msix(pid: u32, req: IrqBindRequest, epoch: u64) -> Result<IrqBindResult, IrqBindError> {
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
    let dest_apic_id = crate::arch::interrupt::apic::id() as u8;
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

pub(super) fn handle_view(handle: &PciHandle) -> MsixHandleView {
    let msix = match handle.msix {
        Some(m) => m,
        None => return MsixHandleView::no_msix(),
    };
    let table_bar_idx = msix.table_bar as usize;
    let pba_bar_idx = msix.pba_bar as usize;
    let table_in = table_bar_idx < handle.bars.len();
    let pba_in = pba_bar_idx < handle.bars.len();
    let table_mmio = table_in && is_mmio_bar(&handle.bars[table_bar_idx]);
    let pba_mmio = pba_in && is_mmio_bar(&handle.bars[pba_bar_idx]);
    MsixHandleView {
        msix_present: true,
        msix_table_size: msix.vector_count(),
        table_bar_in_range: table_in,
        table_bar_is_mmio: table_mmio,
        pba_bar_in_range: pba_in,
        pba_bar_is_mmio: pba_mmio,
    }
}

fn is_mmio_bar(bar: &PciBar) -> bool {
    bar.is_present() && bar.is_memory() && bar.address().is_some()
}

pub(super) fn teardown_msix_vector(device_id: u64, device_vector: u16) {
    let Some(handle) = pci_index::lookup(device_id) else { return };
    let Some(msix) = handle.msix else { return };
    current_ops().teardown_vector(&handle.address, &msix, &handle.bars, device_vector);
}

pub(super) fn disable_msix_for_device(device_id: u64) {
    let Some(handle) = pci_index::lookup(device_id) else { return };
    let Some(msix) = handle.msix else { return };
    current_ops().disable_for_device(&handle.address, &msix);
}
