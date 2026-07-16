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

//! Read and write the path-A RF registers. The 8821c reads them directly from a
//! memory-mapped window (base plus the register number shifted by two, masked to
//! 20 bits) and writes them through the serial (SIPI) interface, where the word
//! packs the 8-bit register number above the 20-bit value. A masked write reads
//! the register, replaces the field and writes it back. This follows rtw88
//! `rtw_phy_read_rf` and `rtw_phy_write_rf_reg_sipi` in `phy.c`; the direct-read
//! address, the SIPI word and the masked read-modify-write are checked in
//! `rtl8821ce_proofs`.

use super::regs::{REG_RF_SIPI_A, RFREG_MASK};
use crate::regs::Mmio;

/// `chip->rf_base_addr[RF_PATH_A]`: the direct-read window base for path A.
const REG_RF_BASE_A: usize = 0x2800;

/// Encode a SIPI write word: the 8-bit register address in bits 20..27 and the
/// 20-bit value in bits 0..19, within the 28-bit field.
pub fn sipi_word(addr: u32, data: u32) -> u32 {
    (((addr & 0xFF) << 20) | (data & RFREG_MASK)) & 0x0FFF_FFFF
}

/// Read RF register `addr` on path A.
pub fn read_a<M: Mmio>(mmio: &M, addr: u8) -> u32 {
    mmio.read32(REG_RF_BASE_A + ((addr as usize) << 2)) & RFREG_MASK
}

/// Write the whole 20-bit value of RF register `addr` on path A.
pub fn write_a<M: Mmio>(mmio: &M, addr: u8, data: u32) {
    mmio.write32(REG_RF_SIPI_A, sipi_word(addr as u32, data));
}

/// Write only the `mask` field of RF register `addr` on path A, preserving the
/// rest (a read-modify-write). `value` is aligned to the low end of `mask`.
pub fn write_masked_a<M: Mmio>(mmio: &M, addr: u8, mask: u32, value: u32) {
    let shift = mask.trailing_zeros();
    let old = read_a(mmio, addr);
    let new = (old & !mask) | ((value << shift) & mask);
    write_a(mmio, addr, new);
}
