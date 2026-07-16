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

//! The direct-DMA (DDMA) channel that copies a firmware chunk from the card's
//! packet buffer into on-chip 8051 memory during firmware download. Each
//! transfer programs the source, destination and a control word (length, a
//! running checksum, and the channel-owned bit that clears when the copy
//! finishes), then waits for completion. The engine accumulates a checksum
//! across the chunks of a section and reports whether it validated. These are
//! the rtw88 8821c register semantics, reimplemented; the exact register
//! program is checked against a modeled device in `rtl8821ce_proofs`.

use crate::regs::Mmio;

// The firmware-download control register lives with the rest of the download
// register map; re-exported here so the DDMA proofs keep importing it alongside
// the channel definitions.
pub use super::regs::{
    DMEM_CHKSUM_OK, DMEM_DW_OK, IMEM_CHKSUM_OK, IMEM_DW_OK, MCUFWDL_EN, OCPBASE_DMEM,
    REG_MCUFW_CTRL,
};

// DDMA channel-0 registers. These are the rtw88 8821c DDMA channel semantics
// (`reg.h`): OWN in bit 31, checksum-enable in bit 29, checksum-status in bit 27,
// reset-checksum in bit 25, checksum-continue in bit 24, and an 18-bit transfer
// length. Verified against rtw88 `reg.h` (BIT_DDMACH0_*): these values are exact.
pub const REG_DDMA_CH0SA: usize = 0x1200;
pub const REG_DDMA_CH0DA: usize = 0x1204;
pub const REG_DDMA_CH0CTRL: usize = 0x1208;

// Control-word bits.
pub const DDMA_OWN: u32 = 1 << 31;
pub const DDMA_CHKSUM_EN: u32 = 1 << 29;
pub const DDMA_CHKSUM_STS: u32 = 1 << 27;
pub const DDMA_RESET_CHKSUM_STS: u32 = 1 << 25;
pub const DDMA_CHKSUM_CONT: u32 = 1 << 24;
pub const DDMA_DLEN_MASK: u32 = 0x3FFFF;

// Reads allowed before a DDMA transfer is declared stuck.
const OWN_POLL_LIMIT: u32 = 1_000_000;

/// Enable firmware download: set the download-enable bit in the firmware
/// control register while preserving the reserved bits the card owns.
pub fn enable_download<M: Mmio>(mmio: &M) {
    let cur = mmio.read16(REG_MCUFW_CTRL) & 0x3800;
    mmio.write16(REG_MCUFW_CTRL, cur | MCUFWDL_EN);
}

/// Reset the running checksum before the first chunk of a section.
pub fn reset_checksum<M: Mmio>(mmio: &M) {
    let cur = mmio.read32(REG_DDMA_CH0CTRL);
    mmio.write32(REG_DDMA_CH0CTRL, cur | DDMA_RESET_CHKSUM_STS);
}

/// Copy one chunk: `src` is the packet-buffer address, `dst` the on-chip
/// target, `len` the byte count (18-bit). `first` clears the continuation bit
/// so the section's checksum starts fresh; later chunks accumulate. Returns
/// false if the channel never releases ownership (a dead card).
pub fn transfer<M: Mmio>(mmio: &M, src: u32, dst: u32, len: u32, first: bool) -> bool {
    if !wait_idle(mmio) {
        return false;
    }
    let mut ctrl = DDMA_CHKSUM_EN | DDMA_OWN | (len & DDMA_DLEN_MASK);
    if !first {
        ctrl |= DDMA_CHKSUM_CONT;
    }
    mmio.write32(REG_DDMA_CH0SA, src);
    mmio.write32(REG_DDMA_CH0DA, dst);
    mmio.write32(REG_DDMA_CH0CTRL, ctrl);
    wait_idle(mmio)
}

/// Record a finished section in the firmware-download control register and
/// report whether its DDMA checksum validated. rtw88 `check_fw_checksum` does
/// this in software after the section's last chunk: the hardware never sets the
/// download-ok and checksum-ok bits itself, so the driver reads the DDMA
/// checksum status and writes the matching pair of bits for the section's memory
/// (IMEM below `OCPBASE_DMEM`, DMEM at or above it). Without this the control
/// register keeps only the download-enable bit and the close-out reads the
/// firmware as never checksummed. `dst` is the section's on-chip base address.
pub fn record_section<M: Mmio>(mmio: &M, dst: u32) -> bool {
    let ok = mmio.read32(REG_DDMA_CH0CTRL) & DDMA_CHKSUM_STS == 0;
    // The download-ok and checksum-ok bits all live in the low byte; rtw88
    // touches only that byte so the reserved high bits the card owns are kept.
    let ctrl = mmio.read8(REG_MCUFW_CTRL);
    let (dw_ok, chksum_ok) = if dst < OCPBASE_DMEM {
        (IMEM_DW_OK as u8, IMEM_CHKSUM_OK as u8)
    } else {
        (DMEM_DW_OK as u8, DMEM_CHKSUM_OK as u8)
    };
    // A validated section marks both download-ok and checksum-ok; a failed one
    // marks download-ok but leaves checksum-ok clear, matching rtw88.
    let ctrl = if ok { ctrl | dw_ok | chksum_ok } else { (ctrl | dw_ok) & !chksum_ok };
    mmio.write8(REG_MCUFW_CTRL, ctrl);
    ok
}

// The channel-owned bit is set by the host to start a transfer and cleared by
// the card when it finishes; a transfer may only start when it reads clear.
fn wait_idle<M: Mmio>(mmio: &M) -> bool {
    for _ in 0..OWN_POLL_LIMIT {
        if mmio.read32(REG_DDMA_CH0CTRL) & DDMA_OWN == 0 {
            return true;
        }
        core::hint::spin_loop();
    }
    false
}
