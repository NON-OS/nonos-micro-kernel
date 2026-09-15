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

//! Sealing a record.

use nonos_seal::NONCE_LEN;

use crate::blob::OVERHEAD;
use crate::derive::subkey;
use crate::error::VaultError;
use crate::wipe::wipe;

/// Seal `plaintext` for `record` into `out`, which must be
/// `plaintext.len() + OVERHEAD` bytes. Returns the length written.
///
/// The nonce is drawn fresh for every seal from the caller's entropy. A
/// counter would be wrong here: the record key is a constant for as long as
/// the machine is in one state, so a counter that reset with the process
/// would repeat a nonce after a reboot, and that is the one thing this
/// construction does not survive.
pub fn seal(
    root: &[u8; 32],
    record: &[u8],
    plaintext: &[u8],
    entropy: &[u8; NONCE_LEN],
    out: &mut [u8],
) -> Result<usize, VaultError> {
    if out.len() < plaintext.len() + OVERHEAD {
        return Err(VaultError::BadLength);
    }
    if entropy.iter().all(|&b| b == 0) {
        /*
         * An all-zero nonce is what a dead entropy source hands back, and
         * every record sealed with it shares a nonce under the same key.
         * Refusing costs a caller one retry; accepting costs the key.
         */
        return Err(VaultError::NoEntropy);
    }
    let mut key = subkey(root, record).map_err(VaultError::NoKey)?;
    let written = crate::seal_body::write(&key, record, plaintext, entropy, out);
    wipe(&mut key);
    written
}
