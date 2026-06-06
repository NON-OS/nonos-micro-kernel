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
use nonos_libc::crypto_decrypt;

use crate::store::types::StoreError;

use super::constants::ALGO_CHACHA20_POLY1305;

pub(in crate::store) fn open(
    key: &[u8],
    nonce: &[u8],
    cipher: &[u8],
    plain: &mut [u8],
) -> Result<usize, StoreError> {
    let n = crypto_decrypt(
        ALGO_CHACHA20_POLY1305,
        key.as_ptr(),
        nonce.as_ptr(),
        cipher.as_ptr(),
        cipher.len() as u64,
        plain.as_mut_ptr(),
    );
    if n < 0 {
        return Err(StoreError::CryptoFailure);
    }
    Ok(n as usize)
}
