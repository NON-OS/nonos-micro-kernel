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

extern crate alloc;

use alloc::vec::Vec;
use nonos_libc::{crypto_hkdf_sha256, crypto_hmac_sha256};

use super::types::CryptoError;

pub fn hmac_sha256(key: &[u8], data: &[u8], out: &mut [u8; 32]) -> Result<(), CryptoError> {
    let n =
        crypto_hmac_sha256(key.as_ptr(), key.len(), data.as_ptr(), data.len(), out.as_mut_ptr());
    if n == out.len() as i64 {
        Ok(())
    } else {
        Err(CryptoError::Mac)
    }
}

pub fn hkdf_sha256(
    salt: &[u8],
    ikm: &[u8],
    info: &[u8],
    out: &mut [u8],
) -> Result<(), CryptoError> {
    let frame = frame(out.len(), salt, ikm, info)?;
    let n = crypto_hkdf_sha256(frame.as_ptr(), frame.len(), out.as_mut_ptr(), out.len());
    if n == out.len() as i64 {
        Ok(())
    } else {
        Err(CryptoError::Kdf)
    }
}

fn frame(out_len: usize, salt: &[u8], ikm: &[u8], info: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if out_len > u16::MAX as usize || salt.len() > u16::MAX as usize {
        return Err(CryptoError::Kdf);
    }
    if ikm.len() > u16::MAX as usize || info.len() > u16::MAX as usize {
        return Err(CryptoError::Kdf);
    }
    let mut out = Vec::with_capacity(8 + salt.len() + ikm.len() + info.len());
    out.extend_from_slice(&(out_len as u16).to_le_bytes());
    out.extend_from_slice(&(salt.len() as u16).to_le_bytes());
    out.extend_from_slice(&(ikm.len() as u16).to_le_bytes());
    out.extend_from_slice(&(info.len() as u16).to_le_bytes());
    out.extend_from_slice(salt);
    out.extend_from_slice(ikm);
    out.extend_from_slice(info);
    Ok(out)
}
