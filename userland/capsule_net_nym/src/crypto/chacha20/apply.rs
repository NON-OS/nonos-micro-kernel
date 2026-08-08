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

use super::types::{ChaCha20, BLOCK_BYTES};

impl ChaCha20 {
    /// XOR the keystream into `data`. Encryption and decryption are the same.
    pub fn apply(&mut self, data: &mut [u8]) {
        for chunk in data.chunks_mut(BLOCK_BYTES) {
            let block = self.next_block();
            for (byte, key) in chunk.iter_mut().zip(block.iter()) {
                *byte ^= key;
            }
        }
    }
}
