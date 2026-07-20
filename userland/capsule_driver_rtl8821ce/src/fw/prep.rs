// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The register work that brackets the firmware download: halt the on-chip 8051,
//! point the transfer queues at the download path, reset the WL platform, then
//! after the sections land restore the queues, close out the download and
//! release the 8051, and confirm it reports ready. This is rtw88
//! `download_firmware_reg_backup`, `download_firmware_reset_platform`,
//! `wlan_cpu_enable`, `download_firmware_end_flow` and
//! `download_firmware_validate` in `mac.c`, reimplemented over the `Mmio` trait
//! so the exact register program is checked against a modeled device.

use super::regs::{
    BTI_PAGE_OVF, CHECK_SUM_OK, CPU_CLK_EN_HI, CR_TXDMA_EN, DIS_TSF_UDT, DMA_MAPPING_HIGH_B1,
    EN_BCN_FUNCTION, FEN_CPUEN_HI, FIFOPAGE_INFO_1_DLFW, FW_DW_RDY, FW_READY, FW_READY_MASK,
    H2CQ_FULL, LD_RQPN, MCUFWDL_EN, REG_BCN_CTRL, REG_CPU_DMEM_CON, REG_CR, REG_FIFOPAGE_INFO_1,
    REG_H2CQ_CSR, REG_MCUFW_CTRL, REG_RQPN_CTRL_2, REG_RSV_CTRL, REG_SYS_CLK_CTRL, REG_SYS_FUNC_EN,
    REG_TXDMA_PQ_MAP, REG_TXDMA_STATUS, WLMCU_IOIF_HI, WL_PLATFORM_RST_B2,
};
use crate::regs::Mmio;

/// Reads allowed before the firmware-ready handshake is declared failed. The
/// 8051 needs real time to boot and raise the ready bits, so this is generous.
const READY_POLL_LIMIT: u32 = 8_000_000;

/// How the download close-out ended, so a serial-less boot can tell the two
/// distinct failures apart from the ready case.
pub enum Finish {
    /// The 8051 booted and reported the firmware loaded and initialised.
    Ready,
    /// The hardware did not confirm both section checksums: the bytes the DDMA
    /// delivered did not match. Carries the control register for the console.
    ChecksumBad(u16),
    /// Checksums were good but the 8051 never reached ready within the poll.
    /// Carries the last control register read, whose bits say how far it got.
    NotReady(u16),
}

/// The registers `begin` overwrote, saved so `finish` can put them back exactly.
#[derive(Clone, Copy, Default)]
pub struct Backup {
    pq_map: u8,
    cr: u8,
    fifopage_info: u16,
    rqpn: u32,
    bcn_ctrl: u8,
}

/// Prepare for download: stop the 8051, map the queues so the beacon queue can
/// stage reserved pages, and reset the WL platform. Returns the saved registers.
pub fn begin<M: Mmio>(mmio: &M) -> Backup {
    halt_cpu(mmio);
    let backup = reg_backup(mmio);
    reset_platform(mmio);
    backup
}

/// Finish download: restore the queues, clear the download state and release the
/// 8051, then wait for it to report ready. Distinguishes a bad-checksum close-out
/// (the DDMA delivered wrong bytes) from a boot that never reaches ready, and
/// carries the control register in both so the failure is legible without serial.
pub fn finish<M: Mmio>(mmio: &M, backup: Backup) -> Finish {
    reg_restore(mmio, backup);

    // download_firmware_end_flow: clear the page-overflow status, then only mark
    // the firmware downloaded if the hardware confirmed both section checksums.
    mmio.write32(REG_TXDMA_STATUS, BTI_PAGE_OVF);
    let ctrl = mmio.read16(REG_MCUFW_CTRL);
    if ctrl & CHECK_SUM_OK != CHECK_SUM_OK {
        return Finish::ChecksumBad(ctrl);
    }
    mmio.write16(REG_MCUFW_CTRL, (ctrl | FW_DW_RDY) & !MCUFWDL_EN);

    start_cpu(mmio);

    let mut last = 0u16;
    for _ in 0..READY_POLL_LIMIT {
        last = mmio.read16(REG_MCUFW_CTRL);
        if last & FW_READY_MASK == FW_READY {
            return Finish::Ready;
        }
        core::hint::spin_loop();
    }
    Finish::NotReady(last)
}

// Stop the 8051: drop its run enable, then its IO interface.
fn halt_cpu<M: Mmio>(mmio: &M) {
    clr8(mmio, REG_SYS_FUNC_EN + 1, FEN_CPUEN_HI);
    clr8(mmio, REG_RSV_CTRL + 1, WLMCU_IOIF_HI);
}

// Release the 8051: restore its IO interface, then its run enable.
fn start_cpu<M: Mmio>(mmio: &M) {
    set8(mmio, REG_RSV_CTRL + 1, WLMCU_IOIF_HI);
    set8(mmio, REG_SYS_FUNC_EN + 1, FEN_CPUEN_HI);
}

// Save the queue registers the download reprograms, then set them for a
// download that only uses the high-priority (beacon) queue.
fn reg_backup<M: Mmio>(mmio: &M) -> Backup {
    let pq_map = mmio.read8(REG_TXDMA_PQ_MAP + 1);
    let cr = mmio.read8(REG_CR);
    let fifopage_info = mmio.read16(REG_FIFOPAGE_INFO_1);
    let rqpn = mmio.read32(REG_RQPN_CTRL_2) | LD_RQPN;
    let bcn_ctrl = mmio.read8(REG_BCN_CTRL);

    mmio.write8(REG_TXDMA_PQ_MAP + 1, DMA_MAPPING_HIGH_B1);
    mmio.write8(REG_CR, CR_TXDMA_EN);
    mmio.write32(REG_H2CQ_CSR, H2CQ_FULL);
    mmio.write16(REG_FIFOPAGE_INFO_1, FIFOPAGE_INFO_1_DLFW);
    mmio.write32(REG_RQPN_CTRL_2, rqpn);
    mmio.write8(REG_BCN_CTRL, (bcn_ctrl & !EN_BCN_FUNCTION) | DIS_TSF_UDT);

    Backup { pq_map, cr, fifopage_info, rqpn, bcn_ctrl }
}

// Put the saved queue registers back after the sections have landed.
fn reg_restore<M: Mmio>(mmio: &M, b: Backup) {
    mmio.write8(REG_TXDMA_PQ_MAP + 1, b.pq_map);
    mmio.write8(REG_CR, b.cr);
    mmio.write16(REG_FIFOPAGE_INFO_1, b.fifopage_info);
    mmio.write32(REG_RQPN_CTRL_2, b.rqpn);
    mmio.write8(REG_BCN_CTRL, b.bcn_ctrl);
}

// Pulse the WL platform reset and 8051 clock so the reloaded firmware starts
// from a known state.
fn reset_platform<M: Mmio>(mmio: &M) {
    clr8(mmio, REG_CPU_DMEM_CON + 2, WL_PLATFORM_RST_B2);
    clr8(mmio, REG_SYS_CLK_CTRL + 1, CPU_CLK_EN_HI);
    set8(mmio, REG_CPU_DMEM_CON + 2, WL_PLATFORM_RST_B2);
    set8(mmio, REG_SYS_CLK_CTRL + 1, CPU_CLK_EN_HI);
}

fn set8<M: Mmio>(mmio: &M, off: usize, bit: u8) {
    let v = mmio.read8(off);
    mmio.write8(off, v | bit);
}

fn clr8<M: Mmio>(mmio: &M, off: usize, bit: u8) {
    let v = mmio.read8(off);
    mmio.write8(off, v & !bit);
}
