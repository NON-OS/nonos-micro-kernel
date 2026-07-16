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

//! Point the card at the BE TX ring: program the 64-bit bus address of the
//! buffer-descriptor ring and its length. rtw88 does this in
//! `rtw_pci_reset_buf_desc` per queue; the register program is checked against a
//! modeled device in `rtl8821ce_proofs`.

use super::regs::{REG_TXBD_DESA_BEQ, REG_TXBD_NUM_BEQ, TRX_BD_IDX_MASK};
use super::ring::TX_DESC_COUNT;
use crate::regs::Mmio;

/// Program the BE ring's bus address (`ring_addr`) and length into the card.
pub fn program<M: Mmio>(mmio: &M, ring_addr: u64) {
    mmio.write32(REG_TXBD_DESA_BEQ, ring_addr as u32);
    mmio.write32(REG_TXBD_DESA_BEQ + 4, (ring_addr >> 32) as u32);
    mmio.write16(REG_TXBD_NUM_BEQ, (TX_DESC_COUNT & TRX_BD_IDX_MASK) as u16);
}
