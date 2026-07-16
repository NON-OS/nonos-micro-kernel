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

//! Set the transmit power. Until this ran, frames left the antenna at the
//! power-on default of essentially zero, so no access point could hear them.
//! rtw8821c programs a per-rate power index into the TXAGC registers: path A
//! starts at 0x1D00, and each 32-bit register packs four consecutive rates, one
//! power index per byte (`rtw8821c_set_tx_power_index_by_rate`). The full driver
//! derives each index from the board's efuse power-by-rate tables and the
//! regulatory limits; until that calibration is ported, a single conservative
//! index is written for every rate the association and data path use, which is
//! enough to reach an access point in range. The register program is checked
//! against a modeled device in `rtl8821ce_proofs`.

use crate::regs::Mmio;

/// TXAGC path-A base. Each register packs four rates, one power index per byte.
const REG_TXAGC_A: usize = 0x1D00;
/// A fixed transmit-power index in half-dB steps: about 20 dBm, ample to reach an
/// access point in range and within the 2.4GHz envelope.
const POWER_INDEX: u8 = 0x28;
/// Rate groups to program: CCK (1 to 11M), the two OFDM groups (6 to 54M), and
/// the two single-stream MCS groups (MCS0 to MCS7). These are the rates the
/// authentication, handshake and data frames are sent at.
const RATE_GROUPS: u32 = 5;

/// Write the fixed transmit-power index across the CCK, OFDM and single-stream
/// MCS rate registers for path A, so transmitted frames carry real power.
pub fn set_tx_power<M: Mmio>(mmio: &M) {
    let word = u32::from_le_bytes([POWER_INDEX; 4]);
    for group in 0..RATE_GROUPS {
        mmio.write32(REG_TXAGC_A + (group * 4) as usize, word);
    }
}
