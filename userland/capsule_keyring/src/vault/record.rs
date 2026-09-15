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

//! The wallet's record: a 32-byte secp256k1 secret, sealed.

use nonos_vault::{open as vault_open, seal as vault_seal, OVERHEAD};

use super::error::VaultError;
use super::root::machine_root;
use super::wipe::wipe32;

/// The record name this capsule seals under. Every other capsule that keeps
/// something picks its own, and the derived keys never collide.
const RECORD: &[u8] = b"keyring.wallet.account";

pub const SECRET_LEN: usize = 32;
pub const BLOB_LEN: usize = SECRET_LEN + OVERHEAD;

/// Seal `secret` for this machine.
///
/// The nonce is the first twelve bytes of a fresh 32-byte draw rather than a
/// twelve-byte draw, so a source that is weak in its tail cannot narrow it.
pub fn seal_secret(secret: &[u8; SECRET_LEN]) -> Result<[u8; BLOB_LEN], VaultError> {
    let mut draw = [0u8; 32];
    if !crate::entropy::gather_secret(&mut draw) {
        return Err(VaultError::NoEntropy);
    }
    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&draw[..12]);
    wipe32(&mut draw);

    let mut root = machine_root().map_err(|_| VaultError::NoKey)?;
    let mut out = [0u8; BLOB_LEN];
    let sealed = vault_seal(&root, RECORD, secret, &nonce, &mut out);
    wipe32(&mut root);
    sealed.map(|_| out).map_err(VaultError::from)
}

/// Open a record sealed by this machine.
pub fn open_secret(bytes: &[u8]) -> Result<[u8; SECRET_LEN], VaultError> {
    let mut root = machine_root().map_err(|_| VaultError::NoKey)?;
    let mut secret = [0u8; SECRET_LEN];
    let opened = vault_open(&root, RECORD, bytes, &mut secret);
    wipe32(&mut root);
    match opened {
        Ok(_) => Ok(secret),
        Err(e) => {
            /*
             * A failed open can still have written into the buffer before the
             * tag was checked, so it is scrubbed rather than returned.
             */
            wipe32(&mut secret);
            Err(VaultError::from(e))
        }
    }
}
