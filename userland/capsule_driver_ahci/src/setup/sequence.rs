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

use super::{block_port, claim, irq, mmio, pci};
use crate::controller::{enable_ahci, scan_ports, ControllerInfo};
use crate::discover::find_ahci;
use crate::error::{AhciError, AhciResult};
use crate::handles::BrokerHandles;
use crate::regs::Regs;
use crate::setup::Driver;
use nonos_libc::mk_device_release;

pub fn run() -> AhciResult<Driver> {
    let dev = find_ahci().ok_or(AhciError::DeviceNotFound)?;
    let claim_epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::enable_bus_master(dev.device_id, claim_epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }

    let mmio = mmio::map(dev.device_id, claim_epoch, dev.abar_size)?;
    let irq = irq::bind(dev, claim_epoch);

    let handles = BrokerHandles::new(dev.device_id, mmio.grant_id, mmio.user_va, irq.grant_id);
    let regs = Regs::new(handles.mmio_user_va());

    enable_ahci(regs);
    let info = ControllerInfo::read(regs);
    let ports = scan_ports(regs, info.pi, info.port_count);
    let block = block_port::bring_up(dev.device_id, claim_epoch, regs, &ports);

    Ok(Driver { handles, regs, info, ports, block })
}
