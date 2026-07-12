// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use crate::firmware::{
    blob_for_family, load_sections, release_cpu, stage_firmware, Family, FirmwareBlob,
    FirmwareStageState,
};
use crate::regs::Regs;

#[derive(Clone, Copy)]
pub struct Driver {
    pub device_id: u64,
    pub pci_device: u16,
    pub claim_epoch: u64,
    pub mmio_grant: u64,
    pub irq_grant: u64,
    pub dma_grant: u64,
    pub dma_user_va: u64,
    pub dma_device_addr: u64,
    pub dma_len: u64,
    pub hw_rev: u32,
    pub gp_cntrl: u32,
    pub rf_kill: bool,
    pub family: Family,
    pub regs: Regs,
    pub firmware_stage: FirmwareStageState,
    pub cmd_write_ptr: usize,
}

impl Driver {
    pub fn firmware(&self) -> FirmwareBlob {
        blob_for_family(self.family)
    }

    /// Issue a host command to the alive firmware over the command queue,
    /// returning the new ring write pointer or `None` if it does not fit a
    /// slot. Composes the command in the DMA buffer that staged the firmware,
    /// fills the slot's TFD, advances the ring and rings the doorbell.
    pub fn issue_command(&mut self, cmd: u8, group: u8, payload: &[u8]) -> Option<usize> {
        let q = crate::hcmd::send::CmdQueue {
            dma_user_va: self.dma_user_va,
            dma_device_addr: self.dma_device_addr,
            ring_off: crate::constants::CMD_RING_OFFSET,
            cmd_off: crate::constants::CMD_AREA_OFFSET,
            queue: crate::constants::CMD_QUEUE_ID,
        };
        let next =
            unsafe { crate::hcmd::send::send_host_command(&self.regs, &q, self.cmd_write_ptr, cmd, group, payload)? };
        self.cmd_write_ptr = next;
        Some(next)
    }

    pub fn stage_firmware(&mut self) -> Option<FirmwareStageState> {
        let fw = self.firmware();
        let state = stage_firmware(fw.bytes, self.dma_user_va, self.dma_len)?;
        self.firmware_stage = state;
        Some(state)
    }

    /// Transfer the staged firmware sections into device SRAM over the FH
    /// service channel, then release the CPU so the firmware boots. Requires
    /// stage_firmware to have run first. The caller then waits for ALIVE.
    pub fn load_firmware(&self) -> Result<u32, &'static str> {
        let staged = self.firmware_stage.staged_bytes;
        if staged == 0 {
            return Err("iwlwifi: firmware not staged");
        }
        let loaded = load_sections(&self.regs, self.dma_user_va, self.dma_device_addr, staged)?;
        release_cpu(&self.regs);
        Ok(loaded)
    }
}
