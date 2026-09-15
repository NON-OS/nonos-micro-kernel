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

//! Asking the keyring to seal, and to open.
//!
//! The wallet is a courier here. It never sees the key, and the blob it
//! carries is worth nothing off this machine, so the weakest link in the path
//! is the disk it writes to, which is exactly where the design wants it.

use alloc::vec::Vec;

use super::format::BLOB_LEN;
use crate::wallet::ipc::{keyring_call, HDR_LEN, OP_VAULT_OPEN, OP_VAULT_SEAL};

/// Seal the wallet's account key. `Err(-2)` means there is no machine key to
/// seal under, which is a real state on a machine with no TPM and not a
/// failure of this call.
pub fn seal_vault(port: u32, owner_pid: u32, wallet_id: u32) -> Result<[u8; BLOB_LEN], i32> {
    let mut payload = Vec::with_capacity(8);
    payload.extend_from_slice(&owner_pid.to_le_bytes());
    payload.extend_from_slice(&wallet_id.to_le_bytes());
    let rx = keyring_call(port, OP_VAULT_SEAL, &payload, BLOB_LEN)?;
    let body = rx.get(HDR_LEN..HDR_LEN + BLOB_LEN).ok_or(-11)?;
    let mut out = [0u8; BLOB_LEN];
    out.copy_from_slice(body);
    Ok(out)
}

/// Hand a blob back and get a wallet id. The keyring verifies it under the
/// machine key before a byte of it becomes a key.
pub fn open_vault(
    port: u32,
    owner_pid: u32,
    now: u64,
    expires_at: u64,
    blob: &[u8],
) -> Result<u32, i32> {
    let mut payload = Vec::with_capacity(20 + blob.len());
    payload.extend_from_slice(&owner_pid.to_le_bytes());
    payload.extend_from_slice(&now.to_le_bytes());
    payload.extend_from_slice(&expires_at.to_le_bytes());
    payload.extend_from_slice(blob);
    let rx = keyring_call(port, OP_VAULT_OPEN, &payload, 4)?;
    let body = rx.get(HDR_LEN..HDR_LEN + 4).ok_or(-11)?;
    Ok(u32::from_le_bytes([body[0], body[1], body[2], body[3]]))
}
