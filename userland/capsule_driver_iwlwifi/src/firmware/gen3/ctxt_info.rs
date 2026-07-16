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

//! The gen3 context-information structure. The AX210-class boot ROM reads this
//! packed structure from host memory to find the peripheral scratch area, the
//! peripheral info block, and the metadata transmit/receive rings, then loads
//! firmware on its own. The field offsets follow `struct iwl_context_info_gen3`
//! in Linux iwlwifi (iwl-context-info-gen3.h); the structure is 104 bytes and
//! little-endian. The boot ROM starts from a zeroed structure and only the
//! address and size fields below are meaningful, so `write` zeroes the whole
//! structure and then fills exactly those. Every offset is pinned by the gen3
//! proofs so a transcription slip cannot pass unnoticed.

/// Size in bytes of the packed gen3 context-information structure.
pub const CTXT_INFO_GEN3_SIZE: usize = 104;

// Little-endian field offsets within the packed structure.
const OFF_PRPH_INFO_BASE: usize = 8;
const OFF_CR_HEAD_IDX_ARR_BASE: usize = 16;
const OFF_TR_TAIL_IDX_ARR_BASE: usize = 24;
const OFF_CR_TAIL_IDX_ARR_BASE: usize = 32;
const OFF_CR_IDX_ARR_SIZE: usize = 48;
const OFF_TR_IDX_ARR_SIZE: usize = 50;
const OFF_MTR_BASE: usize = 52;
const OFF_MCR_BASE: usize = 60;
const OFF_MTR_SIZE: usize = 68;
const OFF_MCR_SIZE: usize = 70;
const OFF_PRPH_SCRATCH_BASE: usize = 88;
const OFF_PRPH_SCRATCH_SIZE: usize = 96;

/// The physical addresses and ring sizes the boot ROM needs. All addresses are
/// host-physical (the values a DMA grant reports as `device_addr`).
pub struct CtxtInfoGen3 {
    /// Peripheral info block: the ROM mirrors its boot stage here.
    pub prph_info_base: u64,
    /// Peripheral scratch: control flags, versions, and the firmware DRAM table.
    pub prph_scratch_base: u64,
    /// Size of the peripheral scratch structure, in bytes.
    pub prph_scratch_size: u32,
    /// Completion-ring head index array (the receive ring status block).
    pub cr_head_idx_arr_base: u64,
    /// Transfer-ring tail index array.
    pub tr_tail_idx_arr_base: u64,
    /// Completion-ring tail index array.
    pub cr_tail_idx_arr_base: u64,
    /// Number of entries in each transfer-ring index array.
    pub tr_idx_arr_size: u16,
    /// Number of entries in each completion-ring index array.
    pub cr_idx_arr_size: u16,
    /// Metadata transfer ring: the command queue byte-descriptor ring.
    pub mtr_base: u64,
    /// Metadata completion ring: the receive used-buffer descriptor ring.
    pub mcr_base: u64,
    /// Metadata transfer ring size, in control-block units.
    pub mtr_size: u16,
    /// Metadata completion ring size, in control-block units.
    pub mcr_size: u16,
}

impl CtxtInfoGen3 {
    /// Write the structure into the first [`CTXT_INFO_GEN3_SIZE`] bytes of
    /// `buf`, zeroing the rest of the structure first. Returns false if `buf`
    /// is too small.
    pub fn write(&self, buf: &mut [u8]) -> bool {
        if buf.len() < CTXT_INFO_GEN3_SIZE {
            return false;
        }
        for b in &mut buf[..CTXT_INFO_GEN3_SIZE] {
            *b = 0;
        }
        w64(buf, OFF_PRPH_INFO_BASE, self.prph_info_base);
        w64(buf, OFF_CR_HEAD_IDX_ARR_BASE, self.cr_head_idx_arr_base);
        w64(buf, OFF_TR_TAIL_IDX_ARR_BASE, self.tr_tail_idx_arr_base);
        w64(buf, OFF_CR_TAIL_IDX_ARR_BASE, self.cr_tail_idx_arr_base);
        w16(buf, OFF_CR_IDX_ARR_SIZE, self.cr_idx_arr_size);
        w16(buf, OFF_TR_IDX_ARR_SIZE, self.tr_idx_arr_size);
        w64(buf, OFF_MTR_BASE, self.mtr_base);
        w64(buf, OFF_MCR_BASE, self.mcr_base);
        w16(buf, OFF_MTR_SIZE, self.mtr_size);
        w16(buf, OFF_MCR_SIZE, self.mcr_size);
        w64(buf, OFF_PRPH_SCRATCH_BASE, self.prph_scratch_base);
        w32(buf, OFF_PRPH_SCRATCH_SIZE, self.prph_scratch_size);
        true
    }
}

fn w16(buf: &mut [u8], off: usize, val: u16) {
    buf[off..off + 2].copy_from_slice(&val.to_le_bytes());
}

fn w32(buf: &mut [u8], off: usize, val: u32) {
    buf[off..off + 4].copy_from_slice(&val.to_le_bytes());
}

fn w64(buf: &mut [u8], off: usize, val: u64) {
    buf[off..off + 8].copy_from_slice(&val.to_le_bytes());
}
