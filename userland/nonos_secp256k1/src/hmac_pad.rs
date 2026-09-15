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

//! Padding the key to one HMAC block.

use nonos_hash::sha256;

use crate::hmac::BLOCK;

/// The key padded to one block.
///
/// A key longer than the block is hashed first, which is what makes the
/// construction defined for any key length; shorter keys are zero padded.
/// Every key this crate passes is exactly 32 bytes, so the long branch is for
/// correctness rather than for use.
pub(crate) fn block_key(key: &[u8]) -> [u8; BLOCK] {
    let mut block = [0u8; BLOCK];
    if key.len() > BLOCK {
        block[..32].copy_from_slice(&sha256(key));
    } else {
        block[..key.len()].copy_from_slice(key);
    }
    block
}
