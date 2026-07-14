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
use crate::i2c_client::wire::{HDR_LEN, MAGIC, OP_ACPI_HID, VERSION};

const CALL_TIMEOUT_MS: u64 = 250;
// Header (20) + status (4) + slave/reg/gpio (6).
const MIN_REPLY: usize = HDR_LEN + 4 + 6;

/// Ask the controller for the touchpad address the kernel recovered from ACPI.
/// Returns (slave_addr, hid_desc_reg) when the firmware declared one, so the
/// driver binds to the exact device instead of probing a guessed address list.
pub fn query_acpi_hid(port: u32) -> Option<(u8, u16)> {
    let request_id = seq::next();
    let mut tx = [0u8; HDR_LEN];
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&OP_ACPI_HID.to_le_bytes());
    tx[8..16].copy_from_slice(&request_id.to_le_bytes());

    let mut rx = [0u8; 64];
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
    let slave = u16::from_le_bytes([rx[24], rx[25]]);
    let reg = u16::from_le_bytes([rx[26], rx[27]]);
    if slave == 0 {
        return None;
    }
    Some((slave as u8, reg))
}
