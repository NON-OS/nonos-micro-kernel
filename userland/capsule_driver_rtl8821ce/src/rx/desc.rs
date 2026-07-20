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

//! Parse the RX descriptor the card prepends to each received frame, and build
//! the descriptor that re-arms a consumed RX slot. The RX descriptor's first
//! word carries the frame length, the driver-info size (in 8-byte units) and a
//! shift, which together give the offset from the descriptor to the frame, plus
//! the CRC and ICV error flags and whether the frame is a firmware command
//! (C2H) rather than an 802.11 frame. Bit positions follow the rtw88 `rx.h`
//! `GET_RX_DESC_*` accessors and `rtw8821c_query_rx_desc`; checked against
//! known-answer descriptors in `rtl8821ce_proofs`.

use super::regs::{RX_BUF_DESC_SIZE, RX_PKT_DESC_SIZE};

// Word 0 fields.
const W0_PKT_LEN: u32 = 0x0000_3FFF; // GENMASK(13, 0)
const W0_CRC32: u32 = 1 << 14;
const W0_ICV_ERR: u32 = 1 << 15;
const W0_DRV_INFO_SHIFT: u32 = 16; // GENMASK(19, 16), in 8-byte units
const W0_DRV_INFO_MASK: u32 = 0xF;
const W0_SHIFT_SHIFT: u32 = 24; // GENMASK(25, 24), in bytes
const W0_SHIFT_MASK: u32 = 0x3;
const W0_PHYST: u32 = 1 << 26;
// Word 2 fields.
const W2_C2H: u32 = 1 << 28;

/// The parsed RX descriptor: where the frame is and whether it is usable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RxInfo {
    pub pkt_len: usize,
    pub drv_info_size: usize,
    pub shift: usize,
    pub phy_status: bool,
    pub crc_err: bool,
    pub icv_err: bool,
    pub is_c2h: bool,
}

impl RxInfo {
    /// Offset from the start of the descriptor to the 802.11 frame.
    pub fn frame_offset(&self) -> usize {
        RX_PKT_DESC_SIZE + self.drv_info_size + self.shift
    }
    /// Total bytes this descriptor plus its frame occupy in the buffer.
    pub fn total_len(&self) -> usize {
        self.frame_offset() + self.pkt_len
    }
    /// A frame worth delivering: a valid 802.11 frame, not a firmware command or
    /// an error frame.
    pub fn deliverable(&self) -> bool {
        !self.is_c2h && !self.crc_err && !self.icv_err && self.pkt_len != 0
    }
}

/// Parse the descriptor at the start of `buf`, or `None` if `buf` is shorter
/// than one descriptor.
pub fn parse(buf: &[u8]) -> Option<RxInfo> {
    if buf.len() < RX_PKT_DESC_SIZE {
        return None;
    }
    let w0 = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    let w2 = u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]);
    Some(RxInfo {
        pkt_len: (w0 & W0_PKT_LEN) as usize,
        drv_info_size: ((w0 >> W0_DRV_INFO_SHIFT) & W0_DRV_INFO_MASK) as usize * 8,
        shift: ((w0 >> W0_SHIFT_SHIFT) & W0_SHIFT_MASK) as usize,
        phy_status: w0 & W0_PHYST != 0,
        crc_err: w0 & W0_CRC32 != 0,
        icv_err: w0 & W0_ICV_ERR != 0,
        is_c2h: w2 & W2_C2H != 0,
    })
}

/// Build the 8-byte RX buffer descriptor that (re-)arms a slot: it tells the
/// card the slot's size and bus address; the card fills the received length.
pub fn rearm(dma: u64, buf_size: usize) -> [u8; RX_BUF_DESC_SIZE] {
    let mut d = [0u8; RX_BUF_DESC_SIZE];
    d[0..2].copy_from_slice(&(buf_size as u16).to_le_bytes());
    // Bytes 2..4 (total_pkt_size) stay zero; the card writes the length there.
    d[4..8].copy_from_slice(&(dma as u32).to_le_bytes());
    d
}
