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

//! Opening one, on the machine that sealed it.

use nonos_seal::open as aead_open;

use crate::blob::OVERHEAD;
use crate::parse::parse;
use crate::error::VaultError;
use crate::derive::subkey;
use crate::wipe::wipe;

/// Open `sealed` for `record` into `out`, which must hold
/// `sealed.len() - OVERHEAD` bytes. Returns the plaintext length.
///
/// Nothing is written to `out` unless the tag verifies, so a wrong key or an
/// altered record never yields bytes a caller might use.
pub fn open(
    root: &[u8; 32],
    record: &[u8],
    sealed: &[u8],
    out: &mut [u8],
) -> Result<usize, VaultError> {
    if sealed.len() <= OVERHEAD {
        return Err(VaultError::NotAVault);
    }
    let want = sealed.len() - OVERHEAD;
    if out.len() < want {
        return Err(VaultError::BadLength);
    }
    let mut key = subkey(root, record).map_err(VaultError::NoKey)?;
    let r = open_with_key(&mut key, record, sealed, out, want);
    wipe(&mut key);
    r
}

fn open_with_key(
    key: &mut [u8; 32],
    record: &[u8],
    sealed: &[u8],
    out: &mut [u8],
    want: usize,
) -> Result<usize, VaultError> {
    let r = parse(sealed, record.len() as u8)?;
    match aead_open(key, &r.nonce, r.header, r.ciphertext, &mut out[..want]) {
        Ok(n) => Ok(n),
        Err(_) => {
            wipe(&mut out[..want]);
            Err(VaultError::Tampered)
        }
    }
}
