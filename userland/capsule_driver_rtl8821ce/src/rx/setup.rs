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

//! Bring the RX ring up: arm every buffer descriptor to point at its packet
//! buffer, then program the ring's bus address and length into the card so it
//! can start filling slots. rtw88 does this in `rtw_pci_init_rx_ring` /
//! `rtw_pci_reset_buf_desc`; the primed descriptors and the register program are
//! checked against a modeled device in `rtl8821ce_proofs`.

use super::desc::rearm;
use super::regs::{REG_RXBD_DESA_MPDUQ, REG_RXBD_NUM_MPDUQ};
use super::ring::{RxState, RX_BUF_STRIDE, RX_DESC_COUNT};
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::tx::regs::TRX_BD_IDX_MASK;

/// Prime the descriptor ring against its packet buffers and point the card at
/// it. `ring` is the buffer-descriptor ring; `buffers_dev` is the bus address of
/// the packet buffer the slots are armed against.
pub fn program<M, R>(mmio: &M, ring: &R, buffers_dev: u64)
where
    M: Mmio,
    R: DmaMem,
{
    for slot in 0..RX_DESC_COUNT {
        let dma = buffers_dev + RxState::buffer_offset(slot) as u64;
        let bd = rearm(dma, RX_BUF_STRIDE);
        ring.write_bytes(RxState::desc_offset(slot), &bd);
    }
    let ring_addr = ring.device_addr();
    mmio.write32(REG_RXBD_DESA_MPDUQ, ring_addr as u32);
    mmio.write32(REG_RXBD_DESA_MPDUQ + 4, (ring_addr >> 32) as u32);
    mmio.write16(REG_RXBD_NUM_MPDUQ, (RX_DESC_COUNT & TRX_BD_IDX_MASK) as u16);
}
