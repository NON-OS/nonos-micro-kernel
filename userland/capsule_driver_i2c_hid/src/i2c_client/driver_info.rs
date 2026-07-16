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

use nonos_libc::mk_ipc_call_timeout;

use crate::i2c_client::seq;
use crate::i2c_client::wire::{HDR_LEN, MAGIC, OP_REGISTER_SNAPSHOT, VERSION};

const CALL_TIMEOUT_MS: u64 = 250;
// Header (20) + status (4) + 12 LE u32 register words.
const MIN_REPLY: usize = HDR_LEN + 4 + 48;
// Words 10 and 11 of the snapshot body carry the bind identity.
const WORD_CONTROLLER_SLOT: usize = 10;
const WORD_BOUND_BY_PROBE: usize = 11;

/// Ask the host driver which controller it bound and whether the bind was
/// confirmed by the touchpad ACKing the setup probe. Returns
/// (controller index+1, probe-confirmed); the slot is 0 when the controller's
/// id has no I2Cn index mapping. Used only to feed the on-screen diagnostics.
pub fn query_driver_info(port: u32) -> Option<(u32, bool)> {
    let request_id = seq::next();
    let mut tx = [0u8; HDR_LEN];
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&OP_REGISTER_SNAPSHOT.to_le_bytes());
    tx[8..16].copy_from_slice(&request_id.to_le_bytes());

    let mut rx = [0u8; 96];
    let got = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        HDR_LEN,
        rx.as_mut_ptr(),
        rx.len(),
        CALL_TIMEOUT_MS,
    );
    if got < MIN_REPLY as i64 {
        return None;
    }
    if u32::from_le_bytes([rx[0], rx[1], rx[2], rx[3]]) != MAGIC {
        return None;
    }
    if u64::from_le_bytes(rx[8..16].try_into().ok()?) != request_id {
        return None;
    }
    if i32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]) != 0 {
        return None;
    }
    let word = |i: usize| {
        let at = 24 + i * 4;
        u32::from_le_bytes([rx[at], rx[at + 1], rx[at + 2], rx[at + 3]])
    };
    Some((word(WORD_CONTROLLER_SLOT), word(WORD_BOUND_BY_PROBE) != 0))
}
