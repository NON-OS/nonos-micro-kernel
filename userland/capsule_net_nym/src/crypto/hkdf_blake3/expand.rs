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

use super::super::types::CryptoError;
use super::hmac::hmac_blake3;
use alloc::vec::Vec;

/// HKDF-Expand. The counter starts at one and each round feeds back the
/// previous block, so output longer than one hash is still bound to the key.
pub fn expand(prk: &[u8; 32], info: &[u8], out: &mut [u8]) -> Result<(), CryptoError> {
    if out.len() > 255 * 32 {
        return Err(CryptoError::Kdf);
    }
    let mut previous: Vec<u8> = Vec::new();
    let mut done = 0usize;
    let mut counter: u8 = 1;
    while done < out.len() {
        let mut message = Vec::with_capacity(previous.len() + info.len() + 1);
        message.extend_from_slice(&previous);
        message.extend_from_slice(info);
        message.push(counter);
        let mut block = [0u8; 32];
        hmac_blake3(prk, &message, &mut block)?;
        let take = core::cmp::min(32, out.len() - done);
        out[done..done + take].copy_from_slice(&block[..take]);
        done += take;
        previous = block.to_vec();
        counter += 1;
    }
    Ok(())
}
