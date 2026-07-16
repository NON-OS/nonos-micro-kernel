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

//! Build the 48-byte TX packet descriptor for a data or management frame. It
//! carries the frame size, the 48-byte header offset, the queue selector, the
//! last-segment flag, the broadcast/multicast flag, the rate group and either a
//! fixed transmit rate (management and early association frames, before rate
//! control is trained) or firmware rate control (ordinary data). The software
//! sequence number goes in the last word. The word and bit layout follows rtw88
//! `rtw_tx_fill_tx_desc` with the field choices from `rtw_tx_data_pkt_info_update`
//! in `tx.c`; checked byte-for-byte in `rtl8821ce_proofs`.

use crate::ring::TX_DESC_SIZE;

/// The descriptor length in bytes.
pub const TXDESC_LEN: usize = TX_DESC_SIZE as usize;

// Word 0.
const W0_TXPKTSIZE: u32 = 0x0000_FFFF;
const W0_OFFSET_SHIFT: u32 = 16;
const W0_BMC: u32 = 1 << 24;
const W0_LS: u32 = 1 << 26;
// Word 1.
const W1_QSEL_SHIFT: u32 = 8;
const W1_RATE_ID_SHIFT: u32 = 16;
// The security-type field is GENMASK(23, 22).
const W1_SEC_TYPE_SHIFT: u32 = 22;
// Word 3.
const W3_USE_RATE: u32 = 1 << 8;
const W3_DISDATAFB: u32 = 1 << 10;
// Word 4.
const W4_DATARATE: u32 = 0x0000_007F;
// Word 9.
const W9_SW_SEQ_SHIFT: u32 = 12;

/// The rate group rtw88 uses for data frames (`rate_id = 6`).
const RATE_ID_DEFAULT: u32 = 6;
/// `DESC_RATE6M`: the default data rate hint / early-association rate.
pub const DESC_RATE_6M: u8 = 0x04;

/// Hardware CCMP (AES) encryption, keyed by the security CAM entry the receiver
/// address maps to. Zero selects no hardware encryption.
pub const SEC_TYPE_CCMP: u8 = 0x03;

/// The per-frame descriptor inputs the queue selector cannot supply on its own.
#[derive(Clone, Copy)]
pub struct FrameMeta {
    /// The queue selector (TID for data, MGMT for management).
    pub qsel: u8,
    /// True when the receiver address is broadcast or multicast.
    pub bmc: bool,
    /// A fixed transmit rate, or `None` for firmware rate control.
    pub rate: Option<u8>,
    /// The 802.11 sequence number.
    pub seq: u16,
    /// The hardware security type: zero for none, or [`SEC_TYPE_CCMP`]. When
    /// CCMP, hardware encrypts using the CAM key for the receiver address.
    pub sec_type: u8,
}

/// Build a TX descriptor for a `frame_len`-byte frame. `meta.rate` fixes the
/// transmit rate (management and early association frames, before rate control
/// is trained); `None` leaves rate control to the firmware with a 6M hint.
pub fn frame(frame_len: usize, meta: &FrameMeta) -> [u8; TXDESC_LEN] {
    let mut w0 = (frame_len as u32) & W0_TXPKTSIZE;
    w0 |= TX_DESC_SIZE << W0_OFFSET_SHIFT;
    w0 |= W0_LS;
    if meta.bmc {
        w0 |= W0_BMC;
    }

    let w1 = ((meta.qsel as u32) << W1_QSEL_SHIFT)
        | (RATE_ID_DEFAULT << W1_RATE_ID_SHIFT)
        | ((meta.sec_type as u32 & 0x3) << W1_SEC_TYPE_SHIFT);

    let (w3, rate_code) = match meta.rate {
        Some(r) => (W3_USE_RATE | W3_DISDATAFB, r),
        None => (0, DESC_RATE_6M),
    };
    let w4 = (rate_code as u32) & W4_DATARATE;
    let w9 = ((meta.seq as u32) << W9_SW_SEQ_SHIFT) & 0x00FF_F000;

    let mut d = [0u8; TXDESC_LEN];
    d[0..4].copy_from_slice(&w0.to_le_bytes());
    d[4..8].copy_from_slice(&w1.to_le_bytes());
    d[12..16].copy_from_slice(&w3.to_le_bytes());
    d[16..20].copy_from_slice(&w4.to_le_bytes());
    d[36..40].copy_from_slice(&w9.to_le_bytes());
    d
}
