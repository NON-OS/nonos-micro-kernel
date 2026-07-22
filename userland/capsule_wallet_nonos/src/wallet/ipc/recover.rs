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
use super::constants::{HDR_LEN, OP_WALLET_RECOVER};

/// Recover a wallet from BIP39 word indices. The keyring verifies the
/// checksum and derives the account; a wrong phrase is rejected there and
/// surfaces as an error here, never as a wrong account. The payload carrying
/// the words is wiped before returning.
pub fn recover_wallet(port: u32, owner_pid: u32, indices: &[u16]) -> Result<u32, i32> {
    let now = mk_time_millis().max(0) as u64;
    let expires = now.saturating_add(31_536_000_000);
    let mut payload = Vec::with_capacity(21 + indices.len() * 2);
    payload.extend_from_slice(&owner_pid.to_le_bytes());
    payload.extend_from_slice(&now.to_le_bytes());
    payload.extend_from_slice(&expires.to_le_bytes());
    payload.push(indices.len() as u8);
    for &w in indices {
        payload.extend_from_slice(&w.to_le_bytes());
    }
    let result = keyring_call(port, OP_WALLET_RECOVER, &payload, 4);
    for b in payload.iter_mut() {
        // SAFETY: volatile write so the wipe of the mnemonic payload holds.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    let rx = result?;
    if rx.len() < HDR_LEN + 4 {
        return Err(-11);
    }
    Ok(u32::from_le_bytes([rx[8], rx[9], rx[10], rx[11]]))
}
