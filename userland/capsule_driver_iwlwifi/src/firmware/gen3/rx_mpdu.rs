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

//! Extract the received 802.11 frame from a firmware RX-MPDU notification. The
//! notification payload is `struct iwl_rx_mpdu_desc` followed by the frame; the
//! descriptor's first field is `mpdu_len` (a __le16 at offset 0, fw/api/rx.h).
//!
//! The descriptor length is deliberately a parameter, not a baked constant:
//! iwlwifi selects the descriptor version (IWL_RX_DESC_SIZE_V1 vs the extended
//! layout) from firmware capabilities at runtime, so the size is a value the
//! device reports, not something safe to guess off-silicon. This function owns
//! the framing logic; the caller supplies the descriptor size the running
//! firmware announced. The returned frame is the encrypted 802.11 MPDU, which
//! the data path then decaps into an Ethernet frame.

/// Offset of the `mpdu_len` field within the descriptor.
const OFF_MPDU_LEN: usize = 0;

/// Return the 802.11 frame carried after a `desc_len`-byte RX-MPDU descriptor,
/// or None if the payload is too short to hold the descriptor and the frame it
/// claims. `rx` is the whole notification payload.
pub fn extract_mpdu(rx: &[u8], desc_len: usize) -> Option<&[u8]> {
    if rx.len() < OFF_MPDU_LEN + 2 {
        return None;
    }
    let mpdu_len = u16::from_le_bytes([rx[OFF_MPDU_LEN], rx[OFF_MPDU_LEN + 1]]) as usize;
    let start = desc_len;
    let end = start.checked_add(mpdu_len)?;
    if end > rx.len() {
        return None;
    }
    Some(&rx[start..end])
}
