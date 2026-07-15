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
use nonos_libc::mk_irq_ack;

use crate::discover::Found;
use crate::driver::Driver;
use crate::init::bring_up;
use crate::regs::Regs;
use crate::setup::{claim, irq, mmio, pci};

pub(super) fn bring_up_one(dev: Found) -> Result<Driver, &'static str> {
    let claim_epoch = claim::claim(dev.device_id)?;
    if !dev.is_acpi {
        pci::enable(dev.device_id, claim_epoch)?;
    }
    let mmio = mmio::map(dev, claim_epoch)?;
    let irq = irq::bind(dev, claim_epoch);
    let regs = Regs::new(mmio.user_va);
    let init = bring_up(regs, dev.clock_hz)?;
    if irq.grant_id != 0 {
        let _ = mk_irq_ack(irq.grant_id);
    }
    Ok(Driver {
        device_id: dev.device_id,
        pci_device: dev.pci_device,
        claim_epoch,
        mmio_grant: mmio.grant_id,
        irq_grant: irq.grant_id,
        irq_vector: irq.vector,
        clock_hz: dev.clock_hz,
        family: dev.family,
        comp_type: init.comp_type,
        comp_param: init.comp_param,
        enabled: init.enabled,
        status: init.status,
        bound_by_probe: false,
        bound_addr: 0,
        regs,
    })
}
