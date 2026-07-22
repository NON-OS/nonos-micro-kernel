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
use super::constants::{HDR_LEN, OP_WALLET_GENERATE_HD};

/// Ask the keyring to create a fresh HD wallet. Returns the wallet id plus
/// the mnemonic word indices for the ONE-TIME backup screen; the caller must
/// wipe them the moment they leave the screen. This response is the only
/// place the phrase ever exists outside the keyring's stack.
pub fn generate_wallet_hd(
    port: u32,
    owner_pid: u32,
    words: &mut [u16; 24],
) -> Result<(u32, u8), i32> {
    let now = mk_time_millis().max(0) as u64;
    let expires = now.saturating_add(31_536_000_000);
    let mut payload = Vec::with_capacity(20);
    payload.extend_from_slice(&owner_pid.to_le_bytes());
    payload.extend_from_slice(&now.to_le_bytes());
    payload.extend_from_slice(&expires.to_le_bytes());
    let mut rx = keyring_call(port, OP_WALLET_GENERATE_HD, &payload, 4 + 1 + 48)?;
    if rx.len() < HDR_LEN + 5 {
        wipe_vec(&mut rx);
        return Err(-11);
    }
    let id = u32::from_le_bytes([rx[8], rx[9], rx[10], rx[11]]);
    let count = rx[12] as usize;
    if !matches!(count, 12 | 24) || rx.len() < HDR_LEN + 5 + count * 2 {
        wipe_vec(&mut rx);
        return Err(-11);
    }
    for (i, slot) in words.iter_mut().take(count).enumerate() {
        *slot = u16::from_le_bytes([rx[13 + i * 2], rx[14 + i * 2]]);
    }
    wipe_vec(&mut rx);
    Ok((id, count as u8))
}

fn wipe_vec(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        // SAFETY: volatile write so the wipe of the mnemonic bytes holds.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
