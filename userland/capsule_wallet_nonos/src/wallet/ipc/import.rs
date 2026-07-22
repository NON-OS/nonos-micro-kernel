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

use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use super::call::keyring_call;
use super::constants::{HDR_LEN, OP_WALLET_IMPORT};

// Hand a raw secp256k1 secret to the keyring for custody. The secret crosses
// the IPC boundary once, straight into the keyring's isolated store, and the
// scratch payload is volatile-wiped here before returning so no copy lingers in
// the wallet's heap. The keyring validates and zeroizes its own copy in turn.
pub fn import_wallet(port: u32, owner_pid: u32, secret: &[u8; 32]) -> Result<u32, i32> {
    let now = mk_time_millis().max(0) as u64;
    let expires = now.saturating_add(31_536_000_000);
    let mut payload = Vec::with_capacity(52);
    payload.extend_from_slice(&owner_pid.to_le_bytes());
    payload.extend_from_slice(&now.to_le_bytes());
    payload.extend_from_slice(&expires.to_le_bytes());
    payload.extend_from_slice(secret);
    let result = keyring_call(port, OP_WALLET_IMPORT, &payload, 4);
    for b in payload.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    let rx = result?;
    if rx.len() < HDR_LEN + 4 {
        return Err(-11);
    }
    Ok(u32::from_le_bytes([rx[8], rx[9], rx[10], rx[11]]))
}
