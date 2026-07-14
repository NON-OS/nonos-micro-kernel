// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use nonos_libc::mk_irq_ack;

use super::{claim, dma, irq, mmio};
use crate::discover::find_iwlwifi;
use crate::driver::Driver;
use crate::firmware::family_for_device;
use crate::firmware::FirmwareStageState;
use crate::init::bring_up;
use crate::regs::Regs;

pub fn run() -> Result<Driver, &'static str> {
    let dev = find_iwlwifi().ok_or("iwlwifi: device not found")?;
    let family = family_for_device(dev.pci_device).ok_or("iwlwifi: unsupported device")?;
    let claim_epoch = claim::claim(dev.device_id)?;
    let mmio = mmio::map(dev, claim_epoch)?;
    let irq = irq::bind(dev, claim_epoch, &mmio)?;
    let dma = dma::map_staging(dev.device_id, claim_epoch, &mmio, &irq)?;
    let regs = Regs::new(mmio.user_va);
    let init = bring_up(regs)?;
    let _ = mk_irq_ack(irq.grant_id);
    Ok(Driver {
        device_id: dev.device_id, pci_device: dev.pci_device, claim_epoch,
        mmio_grant: mmio.grant_id, irq_grant: irq.grant_id,
        dma_grant: dma.grant_id, dma_user_va: dma.user_va,
        dma_device_addr: dma.device_addr, dma_len: dma.length,
        hw_rev: init.hw_rev, gp_cntrl: init.gp_cntrl, rf_kill: init.rf_kill, family, regs,
        firmware_stage: FirmwareStageState::empty(),
        cmd_write_ptr: 0,
        rx_read_ptr: 0,
        supplicant: None,
        mlme: None,
        tx_pn: 0,
    })
}
