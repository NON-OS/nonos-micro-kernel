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

//! Legacy (pre-8000) firmware-into-device transfer over the FH service channel.
//!
//! `stage_firmware` lays each ucode section into the DMA staging buffer as a
//! self-describing block: `[dest_addr:u32][payload_len:u32][pad:u32][payload]`.
//! `load_sections` walks those blocks and, for each, programs the FH service
//! channel to DMA the payload from host DRAM into the device SRAM at dest_addr.
//! This is the transfer step that was missing between staging and ALIVE.
//!
//! Scope: this is the legacy FH path. Modern devices (8000+) load through the
//! context-info / TFH path instead, and full bring-up (CPU release, ALIVE
//! handshake, INIT vs RUNTIME sequencing, NVM read, PHY calibration, and RSA
//! secure boot on 9000/AX) is not implemented here. Needs hardware validation.

use crate::constants::{
    CSR_FH_INT_STATUS, FH_SRVC_CHNL_SRAM_ADDR_REG, FH_TCSR_CHNL_TX_BUF_STS_REG,
    FH_TCSR_CHNL_TX_CONFIG_REG, FH_TCSR_TX_BUF_STS_TFDB_VALID, FH_TCSR_TX_CONFIG_CIRQ_HOST_ENDTFD,
    FH_TCSR_TX_CONFIG_DMA_ENABLE, FH_TCSR_TX_CONFIG_DMA_PAUSE, FH_TFDIB_CTRL0_REG,
    FH_TFDIB_CTRL1_REG, FH_TFDIB_REG1_ADDR_BITSHIFT, FH_TX_POLL_ITERS, HBUS_TARG_PRPH_WADDR,
    HBUS_TARG_PRPH_WDAT, PRPH_WADDR_ADDR_MASK, PRPH_WADDR_WORD_ENABLE, RELEASE_CPU_RESET,
    RELEASE_CPU_RESET_BIT,
};
use crate::regs::Mmio;

const BLOCK_HEADER: usize = 12;

/// Indirect write to a peripheral (PRPH) register through the HBUS window.
fn write_prph<M: Mmio>(mmio: &M, addr: u32, val: u32) {
    mmio.write32(
        HBUS_TARG_PRPH_WADDR,
        (addr & PRPH_WADDR_ADDR_MASK) | PRPH_WADDR_WORD_ENABLE,
    );
    mmio.write32(HBUS_TARG_PRPH_WDAT, val);
}

/// Release the firmware CPU from reset. Call once every ucode section is in
/// SRAM; the firmware then boots and raises the ALIVE interrupt.
pub fn release_cpu<M: Mmio>(mmio: &M) {
    write_prph(mmio, RELEASE_CPU_RESET, RELEASE_CPU_RESET_BIT);
}

/// DMA one section payload from host physical address `phys` into device SRAM
/// at `dst_addr`. Returns false if the FH transfer does not signal completion
/// within the poll bound.
fn load_firmware_chunk<M: Mmio>(mmio: &M, dst_addr: u32, phys: u64, byte_cnt: u32) -> bool {
    mmio.write32(FH_TCSR_CHNL_TX_CONFIG_REG, FH_TCSR_TX_CONFIG_DMA_PAUSE);
    mmio.write32(FH_SRVC_CHNL_SRAM_ADDR_REG, dst_addr);
    mmio.write32(FH_TFDIB_CTRL0_REG, (phys & 0xFFFF_FFFF) as u32);
    mmio.write32(
        FH_TFDIB_CTRL1_REG,
        (((phys >> 32) as u32) << FH_TFDIB_REG1_ADDR_BITSHIFT) | byte_cnt,
    );
    mmio.write32(FH_TCSR_CHNL_TX_BUF_STS_REG, FH_TCSR_TX_BUF_STS_TFDB_VALID);
    mmio.write32(
        FH_TCSR_CHNL_TX_CONFIG_REG,
        FH_TCSR_TX_CONFIG_DMA_ENABLE | FH_TCSR_TX_CONFIG_CIRQ_HOST_ENDTFD,
    );
    // The transfer raises an FH interrupt on completion. We poll the FH status
    // (interrupts stay masked during load) and acknowledge it by writing back.
    for _ in 0..FH_TX_POLL_ITERS {
        let status = mmio.read32(CSR_FH_INT_STATUS);
        if status != 0 {
            mmio.write32(CSR_FH_INT_STATUS, status);
            return true;
        }
        core::hint::spin_loop();
    }
    false
}

fn rd_u32(va: u64, off: usize) -> u32 {
    unsafe {
        let p = (va as *const u8).add(off);
        u32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
    }
}

/// Walk the staged blocks in `[dma_user_va, dma_user_va + staged_bytes)` and
/// transfer each section into device SRAM. `dma_device_addr` is the physical
/// address the device sees for the same buffer. Returns the number of sections
/// transferred.
pub fn load_sections<M: Mmio>(
    mmio: &M,
    dma_user_va: u64,
    dma_device_addr: u64,
    staged_bytes: u32,
) -> Result<u32, &'static str> {
    let end = staged_bytes as usize;
    let mut off = 0usize;
    let mut loaded = 0u32;
    while off + BLOCK_HEADER <= end {
        let dst = rd_u32(dma_user_va, off);
        let len = rd_u32(dma_user_va, off + 4) as usize;
        let payload_off = off + BLOCK_HEADER;
        if len == 0 || payload_off + len > end {
            return Err("iwlwifi: staged section overruns buffer");
        }
        let phys = dma_device_addr + payload_off as u64;
        if !load_firmware_chunk(mmio, dst, phys, len as u32) {
            return Err("iwlwifi: firmware chunk DMA timed out");
        }
        loaded += 1;
        off = payload_off + len;
    }
    Ok(loaded)
}
