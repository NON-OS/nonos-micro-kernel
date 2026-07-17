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

//! Take the next received 802.11 frame off the RX ring. The hardware write index
//! says how many slots are ready; each is parsed, and firmware-command, error
//! and oversized frames are skipped (their slots re-armed) until a deliverable
//! frame is found. Its body is copied out, the slot is re-armed, the host read
//! index advances and is written back to hand the slot to the card. This is the
//! read half of rtw88 `rtw_pci_rx_napi`, shaped as a single-frame poll so it
//! backs the `poll_rx` half of the net_core link contract. Checked against a
//! modeled device and DMA memory in `rtl8821ce_proofs`.

use super::desc::{parse, rearm};
use super::regs::REG_RXBD_IDX_MPDUQ;
use super::ring::{RxState, RX_BUF_STRIDE};
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::tx::regs::{TRX_BD_HW_IDX_MASK, TRX_BD_HW_IDX_SHIFT, TRX_BD_IDX_MASK};

/// Copy the next deliverable frame into `out` and return its length, or `None`
/// if no complete data frame is waiting. `ring` is the buffer-descriptor ring;
/// `buffers` is the CPU view of the packet buffer and `buffers_dev` its bus
/// address (used to re-arm a consumed slot).
pub fn poll_one<M, R>(
    mmio: &M,
    ring: &R,
    buffers: &[u8],
    buffers_dev: u64,
    state: &mut RxState,
    out: &mut [u8],
) -> Option<usize>
where
    M: Mmio,
    R: DmaMem,
{
    let idx = mmio.read32(REG_RXBD_IDX_MPDUQ);
    let hw_wp = ((idx & TRX_BD_HW_IDX_MASK) >> TRX_BD_HW_IDX_SHIFT) % state.len;

    while state.ready(hw_wp) > 0 {
        let slot = state.rp;
        let buf_off = RxState::buffer_offset(slot);
        let info = parse(&buffers[buf_off..]);

        // Decide whether this slot carries a frame we can hand up. A CRC or ICV
        // error is counted, not just skipped: an undecryptable reply (bad group
        // key) shows up here rather than as a silent gap.
        let delivered = match info {
            Some(i)
                if i.deliverable() && i.total_len() <= RX_BUF_STRIDE && i.pkt_len <= out.len() =>
            {
                let start = buf_off + i.frame_offset();
                out[..i.pkt_len].copy_from_slice(&buffers[start..start + i.pkt_len]);
                Some(i.pkt_len)
            }
            Some(i) if i.crc_err || i.icv_err => {
                state.err_drops = state.err_drops.wrapping_add(1);
                None
            }
            _ => None,
        };

        // Re-arm the slot and hand it back to the card.
        let dma = buffers_dev + buf_off as u64;
        ring.write_bytes(RxState::desc_offset(slot), &rearm(dma, RX_BUF_STRIDE));
        state.advance();
        mmio.write16(REG_RXBD_IDX_MPDUQ, (state.rp & TRX_BD_IDX_MASK) as u16);

        if delivered.is_some() {
            return delivered;
        }
    }
    None
}
