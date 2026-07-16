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

//! Build the 32-byte H2C offload packets sent down the H2C transfer queue. The
//! header is three fields in the first word (a fixed category and command id
//! plus the per-command sub-id) and, in the second word, the total length and a
//! sequence number; the body follows. The only packet built here is the IQK
//! (RF calibration) request, whose body carries the clear and segmented-IQK
//! flags. The field positions follow rtw88 `rtw_h2c_pkt_set_header`,
//! `SET_PKT_H2C_*`, `FW_OFFLOAD_H2C_SET_SEQ_NUM` and `IQK_SET_*` in `fw.h`; the
//! resulting bytes are checked known-answer in `rtl8821ce_proofs`.

/// The H2C packet size.
pub const H2C_PKT_SIZE: usize = 32;
/// `H2C_PKT_HDR_SIZE`: the header length, added to the body length.
const H2C_PKT_HDR_SIZE: u16 = 8;

// Fixed header fields.
const H2C_PKT_CATEGORY: u32 = 0x01; // word 0, bits 0..6
const H2C_PKT_CMD_ID: u32 = 0xFF; // word 0, bits 8..15
const H2C_PKT_IQK: u32 = 0x0E; // word 0, bits 16..31 (sub-command id)

fn word(p: &mut [u8; H2C_PKT_SIZE], idx: usize, val: u32) {
    p[idx * 4..idx * 4 + 4].copy_from_slice(&val.to_le_bytes());
}

/// Build the IQK (RF calibration) request. `clear` resets the calibration state,
/// `segment` runs it in segments (used while associated), and `seq` is the H2C
/// sequence number.
pub fn build_iqk(clear: bool, segment: bool, seq: u8) -> [u8; H2C_PKT_SIZE] {
    let mut p = [0u8; H2C_PKT_SIZE];
    // Word 0: category, command id, sub-command id.
    word(&mut p, 0, H2C_PKT_CATEGORY | (H2C_PKT_CMD_ID << 8) | (H2C_PKT_IQK << 16));
    // Word 1: total length (header + one body byte) and the sequence number.
    let total_len = u32::from(H2C_PKT_HDR_SIZE + 1);
    word(&mut p, 1, total_len | (u32::from(seq) << 16));
    // Word 2: the IQK flags.
    word(&mut p, 2, u32::from(clear) | (u32::from(segment) << 1));
    p
}
