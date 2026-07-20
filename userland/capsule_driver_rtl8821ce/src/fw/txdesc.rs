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

//! Build the 48-byte TX packet descriptor that prefixes a firmware chunk when it
//! is staged into the on-chip packet buffer as a reserved (beacon) page. The DDMA
//! that follows copies the payload after this descriptor, so its job is only to
//! make the beacon queue write the chunk to reserved-page 0: it carries the
//! payload size, the 48-byte header offset, the beacon queue selector, the
//! last-segment and hardware-sequence flags. The word and bit layout follows
//! rtw88 `rtw_tx_fill_tx_desc` and the `RTW_TX_DESC_*` masks in `tx.h`, with the
//! reserved-page field choices from `rtw_tx_rsvd_page_pkt_info_update`.

use super::regs::{QSEL_BEACON, TX_DESC_SIZE};

/// The descriptor length in bytes; twelve little-endian words.
pub const TXDESC_LEN: usize = TX_DESC_SIZE as usize;

// Word 0 fields.
const W0_TXPKTSIZE: u32 = 0x0000_FFFF; // GENMASK(15, 0)
const W0_OFFSET_SHIFT: u32 = 16; // GENMASK(23, 16)
const W0_BMC: u32 = 1 << 24;
const W0_LS: u32 = 1 << 26;
const W0_DISQSELSEQ: u32 = 1 << 31;
// Word 1 fields.
const W1_QSEL_SHIFT: u32 = 8; // GENMASK(12, 8)
                              // Word 8 fields.
const W8_EN_HWSEQ: u32 = 1 << 15;

/// Build the beacon reserved-page descriptor for a `payload_len`-byte chunk. The
/// first payload byte sets the broadcast/multicast bit exactly as rtw88 derives
/// it from `addr1` (bit 0 of the first octet), so the bytes are a deterministic
/// function of the chunk and can be checked against a known answer.
pub fn beacon(payload_len: usize, first_byte: Option<u8>) -> [u8; TXDESC_LEN] {
    let bmc = matches!(first_byte, Some(b) if b & 1 == 1);

    let mut w0 = (payload_len as u32) & W0_TXPKTSIZE;
    w0 |= TX_DESC_SIZE << W0_OFFSET_SHIFT;
    w0 |= W0_LS;
    w0 |= W0_DISQSELSEQ;
    if bmc {
        w0 |= W0_BMC;
    }

    let w1 = QSEL_BEACON << W1_QSEL_SHIFT;
    let w8 = W8_EN_HWSEQ;

    let mut d = [0u8; TXDESC_LEN];
    d[0..4].copy_from_slice(&w0.to_le_bytes());
    d[4..8].copy_from_slice(&w1.to_le_bytes());
    d[32..36].copy_from_slice(&w8.to_le_bytes());
    d
}
