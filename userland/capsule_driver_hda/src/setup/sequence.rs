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

use nonos_libc::mk_device_release;

use super::mark::mark;
use super::{claim, dma, irq, mmio, pci};
use crate::controller::{leave_reset, probe, ControllerInfo};
use crate::discover::find_hda;
use crate::error::{HdaError, HdaResult};
use crate::handles::BrokerHandles;
use crate::regs::Regs;
use crate::setup::Driver;

pub fn run() -> HdaResult<Driver> {
    let dev = find_hda().ok_or(HdaError::DeviceNotFound)?;
    let claim_epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::enable_bus_master(dev.device_id, claim_epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let mmio = mmio::map(dev.device_id, claim_epoch, dev.bar_size)?;
    let irq = irq::bind(dev, claim_epoch, &mmio)?;
    let (corb, rirb) = dma::map_verb_rings(dev.device_id, claim_epoch, &mmio, &irq)?;
    let handles = BrokerHandles::new(
        dev.device_id,
        mmio.grant_id,
        mmio.user_va,
        irq.grant_id,
        corb.grant_id,
        rirb.grant_id,
    );
    let regs = Regs::new(handles.mmio_user_va());

    leave_reset(regs)?;
    let info = ControllerInfo::read(regs);
    if info.vmaj == 0 || info.gcap == 0 {
        return Err(HdaError::UnsupportedController);
    }
    let codecs = probe(regs, info.statests);
    mark("[HDA] up\n");
    Ok(Driver { handles, regs, codecs, corb, rirb })
}
