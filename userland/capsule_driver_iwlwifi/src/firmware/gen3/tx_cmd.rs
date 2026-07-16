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

//! Wrap an already-built 802.11 frame in the gen3 transmit command the firmware
//! TX queue consumes. The 28-byte header (struct iwl_tx_cmd_gen3 in
//! fw/api/tx.h) carries the payload length, the transmit flags and the rate;
//! the DRAM security info is left zero for the firmware to fill, and the frame
//! follows the header. The frame handed in is the CCMP-protected 802.11 frame
//! from the data path, so encryption is already done in software. Offsets are
//! pinned by the gen3 proofs.

/// Command id for a transmit (TX_CMD).
pub const TX_CMD: u8 = 0x1C;

/// Fixed size of the gen3 transmit-command header, before the 802.11 frame.
pub const TX_CMD_GEN3_HDR: usize = 28;

// Field offsets within the header.
const OFF_LEN: usize = 0;
const OFF_FLAGS: usize = 2;
const OFF_RATE_N_FLAGS: usize = 16;

/// Use the rate carried in this command rather than a firmware rate table.
pub const IWL_TX_FLAGS_CMD_RATE: u16 = 1 << 0;

/// Build the transmit command for `frame` (a complete 802.11 frame) at the
/// given rate into `out`, returning the total command length. Returns None if
/// `out` cannot hold the header plus the frame.
pub fn build(frame: &[u8], rate_n_flags: u32, out: &mut [u8]) -> Option<usize> {
    let total = TX_CMD_GEN3_HDR.checked_add(frame.len())?;
    if out.len() < total {
        return None;
    }
    for b in &mut out[..TX_CMD_GEN3_HDR] {
        *b = 0;
    }
    out[OFF_LEN..OFF_LEN + 2].copy_from_slice(&(frame.len() as u16).to_le_bytes());
    out[OFF_FLAGS..OFF_FLAGS + 2].copy_from_slice(&IWL_TX_FLAGS_CMD_RATE.to_le_bytes());
    out[OFF_RATE_N_FLAGS..OFF_RATE_N_FLAGS + 4].copy_from_slice(&rate_n_flags.to_le_bytes());
    out[TX_CMD_GEN3_HDR..total].copy_from_slice(frame);
    Some(total)
}
