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

use super::{claim, irq, mmio, pci};
use crate::admin::{
    enable, reset_to_disabled, AdminQueue, ControllerIdentity, NamespaceIdentity, SmartHealth,
};
use crate::controller::ControllerInfo;
use crate::discover::find_nvme;
use crate::error::{NvmeError, NvmeResult};
use crate::handles::BrokerHandles;
use crate::regs::Regs;
use crate::setup::Driver;

pub fn run() -> NvmeResult<Driver> {
    let dev = find_nvme().ok_or(NvmeError::DeviceNotFound)?;
    let claim_epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::enable_bus_master(dev.device_id, claim_epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let mmio = mmio::map(dev.device_id, claim_epoch, dev.bar_size)?;
    let irq = irq::bind(dev, claim_epoch, &mmio)?;
    let handles = BrokerHandles::new(dev.device_id, mmio.grant_id, mmio.user_va, irq.grant_id);
    let regs = Regs::new(handles.mmio_user_va());
    let info = ControllerInfo::read(regs);
    if !info.is_nvme_register_block() {
        return Err(NvmeError::UnsupportedController);
    }
    reset_to_disabled(regs)?;
    let mut admin = AdminQueue::allocate(dev.device_id, claim_epoch)?;
    admin.program_registers(regs);
    enable(regs, info)?;
    let identity = {
        let data = admin.identify_controller(regs, info.doorbell_stride())?;
        ControllerIdentity::parse(data)
    };
    let namespace = if identity.namespace_count == 0 {
        NamespaceIdentity::absent()
    } else {
        let data = admin.identify_namespace(regs, info.doorbell_stride(), 1)?;
        NamespaceIdentity::parse(1, data)
    };
    let health = {
        let data = admin.smart_health(regs, info.doorbell_stride())?;
        SmartHealth::parse(data)
    };
    let io = if namespace.nsid != 0 && namespace.lba_size == 512 && namespace.size_lba > 0 {
        crate::nvm::bring_up(
            dev.device_id,
            claim_epoch,
            regs,
            info.doorbell_stride(),
            &mut admin,
            namespace.nsid,
            namespace.size_lba,
        )
        .ok()
    } else {
        None
    };
    Ok(Driver { _admin: admin, handles, regs, identity, namespace, health, io })
}
