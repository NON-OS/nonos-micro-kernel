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

use super::iv::IV;
use super::types::{Blake2b, BLOCK_BYTES};

impl Blake2b {
    /// Keyed BLAKE2b producing `out_len` bytes. The key length goes into the
    /// parameter block and the key is processed as a padded first block;
    /// hashing it as ordinary input would be self-consistent and wrong.
    pub fn new_keyed(key: &[u8], out_len: usize) -> Self {
        let mut state = Self { h: IV, buf: [0u8; BLOCK_BYTES], buf_len: 0, counter: 0, out_len };
        state.h[0] ^= 0x0101_0000 ^ ((key.len() as u64) << 8) ^ out_len as u64;
        if !key.is_empty() {
            let mut first = [0u8; BLOCK_BYTES];
            first[..key.len()].copy_from_slice(key);
            state.update(&first);
        }
        state
    }
}
